//! Executable-private allocator. Only session-worker installs this GlobalAlloc.
//! Unsafe forwarding retains the caller's exact pointer/layout contract; no
//! allocation, formatting, locks, panic or unwind occurs in failure handling.
#![allow(unsafe_code)]
use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicBool, AtomicI32, AtomicU8, AtomicU64, AtomicUsize, Ordering},
};
use uiblueprint_host::process::DarwinPlatform;
use uiblueprint_host::process_api::WorkerPlatform;
use uiblueprint_host::{
    Control, ControlKind, Correlation, OperationClass,
    limits::MIB,
    quota::{ChargeError, QuotaCounter},
};

static COUNTER: QuotaCounter = QuotaCounter::new();
static ORDINARY: AtomicUsize = AtomicUsize::new(MIB);
static TOTAL: AtomicUsize = AtomicUsize::new(MIB);
static PUBLISHER: AtomicUsize = AtomicUsize::new(0);
static CONFIGURED: AtomicBool = AtomicBool::new(false);
static FATAL_FD: AtomicI32 = AtomicI32::new(-1);
static EPOCH: AtomicU64 = AtomicU64::new(0);
static OPERATION: AtomicU64 = AtomicU64::new(0);
static CLASS: AtomicU8 = AtomicU8::new(OperationClass::Attach as u8);
static PHASE: AtomicU8 = AtomicU8::new(Phase::Bootstrap as u8);
#[derive(Clone, Copy)]
#[repr(u64)]
pub enum FatalReason {
    Quota = 1,
    System = 2,
    Invariant = 3,
    Panic = 4,
    ParentGone = 5,
    Deadline = 6,
    Configuration = 7,
}
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Phase {
    Bootstrap = 0,
    Decode = 1,
    Validate = 2,
    Replay = 3,
    Encode = 4,
    Admission = 5,
    Publication = 6,
}

pub struct GuardedAllocator;
fn current_thread() -> usize {
    // SAFETY: pthread_self has no pointer inputs, allocation or ownership change.
    unsafe { libc::pthread_self() as usize }
}
fn reserve(bytes: usize) {
    let limit = if PUBLISHER.load(Ordering::Acquire) == current_thread() {
        TOTAL.load(Ordering::Acquire)
    } else {
        ORDINARY.load(Ordering::Acquire)
    };
    if let Err(error) = COUNTER.charge(bytes, limit) {
        fatal(
            match error {
                ChargeError::Limit => FatalReason::Quota,
                _ => FatalReason::Invariant,
            },
            bytes as u64,
        );
    }
}
fn release(bytes: usize) {
    if COUNTER.release(bytes).is_err() {
        fatal(FatalReason::Invariant, bytes as u64);
    }
}
// Exact System forwarding is fixed by the shipping GlobalAlloc methods below.
unsafe fn system_alloc(layout: Layout) -> *mut u8 {
    // SAFETY: forwarded caller layout satisfies GlobalAlloc::alloc requirements.
    unsafe { System.alloc(layout) }
}
unsafe fn system_alloc_zeroed(layout: Layout) -> *mut u8 {
    // SAFETY: same valid layout, with zeroing supplied by System.
    unsafe { System.alloc_zeroed(layout) }
}
unsafe fn system_realloc(pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
    // SAFETY: original live pointer/layout and valid new size are forwarded.
    unsafe { System.realloc(pointer, layout, new_size) }
}
/// Shared allocation/zeroed charging and null handling, private to the executable.
/// The disposable probe may supply a bounded null-returning forwarder; shipping
/// callers always select one of the two exact System functions above.
///
/// # Safety
/// Layout is valid and nonzero. The forwarder must obey the corresponding
/// GlobalAlloc contract (including zeroing when required), return null or valid
/// storage of this exact layout, and never unwind, log, lock or recursively call
/// the global allocator. A nonnull result is owned by the caller exactly once.
pub(super) unsafe fn allocate_with(
    layout: Layout,
    forward: unsafe fn(Layout) -> *mut u8,
) -> *mut u8 {
    reserve(layout.size());
    // SAFETY: callback contract and original valid layout are the caller's
    // obligations. Charging occurs before this single forwarding operation.
    let pointer = unsafe { forward(layout) };
    if pointer.is_null() {
        release(layout.size());
        fatal(FatalReason::System, layout.size() as u64);
    }
    pointer
}
/// Private realloc forwarding with full-new overlap accounting.
///
/// # Safety
/// Pointer is the live uniquely owned allocation of exactly layout, already
/// charged by this guard; new_size is nonzero and valid for that alignment.
/// Forwarder obeys GlobalAlloc::realloc: null preserves the old allocation;
/// success transfers ownership to a valid new layout and preserves required
/// bytes. It must not unwind, recursively allocate or add logging/locks.
pub(super) unsafe fn reallocate_with(
    pointer: *mut u8,
    layout: Layout,
    new_size: usize,
    forward: unsafe fn(*mut u8, Layout, usize) -> *mut u8,
) -> *mut u8 {
    reserve(new_size);
    // SAFETY: caller establishes both layout and forwarding contracts. Both old
    // and FULL new requested layouts remain charged during the backend call.
    let changed = unsafe { forward(pointer, layout, new_size) };
    if changed.is_null() {
        release(new_size);
        // Old storage remains owned/charged until process termination; neither
        // deallocation nor a fabricated successful pointer is performed here.
        fatal(FatalReason::System, new_size as u64);
    }
    release(layout.size());
    changed
}
// SAFETY: each operation receives valid GlobalAlloc inputs from Rust. The exact
// pointer/alignment/layout goes unchanged to System. Failed reservations never
// reach System; counters are atomic and no callback unwinds or allocates.
unsafe impl GlobalAlloc for GuardedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: valid caller layout and fixed non-recursive System forwarder.
        unsafe { allocate_with(layout, system_alloc) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: valid caller layout and fixed zeroing System forwarder.
        unsafe { allocate_with(layout, system_alloc_zeroed) }
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: caller owns this allocation and provides its original layout.
        unsafe { System.dealloc(pointer, layout) };
        release(layout.size());
    }
    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: valid caller pointer/layout/new size; shipping always uses
        // the exact System implementation, never a runtime-selected callback.
        unsafe { reallocate_with(pointer, layout, new_size, system_realloc) }
    }
}
pub fn configure(epoch: u64, ordinary: usize, total: usize, bootstrap: usize, fatal_fd: i32) {
    if epoch == 0
        || ordinary == 0
        || ordinary > 63 * MIB
        || total <= ordinary
        || total > 64 * MIB
        || total - ordinary > MIB
        || bootstrap == 0
        || bootstrap > MIB
        || COUNTER.peak() > bootstrap
        || COUNTER.live() > ordinary
        || CONFIGURED.swap(true, Ordering::AcqRel)
    {
        fatal(FatalReason::Configuration, 0);
    }
    EPOCH.store(epoch, Ordering::Release);
    FATAL_FD.store(fatal_fd, Ordering::Release);
    TOTAL.store(total, Ordering::Release);
    ORDINARY.store(ordinary, Ordering::Release);
}
pub fn operation(sequence: u64, class: OperationClass, phase: Phase) {
    CLASS.store(class as u8, Ordering::Relaxed);
    PHASE.store(phase as u8, Ordering::Relaxed);
    OPERATION.store(sequence, Ordering::Release);
}
pub fn phase(phase: Phase) {
    PHASE.store(phase as u8, Ordering::Release);
}
pub fn fatal(reason: FatalReason, requested: u64) -> ! {
    let operation = OPERATION.load(Ordering::Acquire);
    let class =
        OperationClass::try_from(CLASS.load(Ordering::Relaxed)).unwrap_or(OperationClass::Attach);
    let record = Control {
        kind: ControlKind::Fatal,
        class,
        slot: 0,
        flags: PHASE.load(Ordering::Relaxed),
        correlation: Correlation {
            session_epoch: EPOCH.load(Ordering::Acquire),
            operation,
        },
        length: requested,
        value: reason as u64,
        auxiliary: COUNTER.live() as u64,
    }
    .encode();
    DarwinPlatform::fatal_exit(
        FATAL_FD.load(Ordering::Acquire),
        &record,
        100 + reason as i32,
    )
}
/// Only the operation thread may spend publication reserve. The watchdog's
/// accidental allocations cannot borrow that allowance while another thread emits.
pub struct PublicationGuard {
    previous: Phase,
}
impl PublicationGuard {
    pub fn enter(previous: Phase) -> Self {
        if PUBLISHER
            .compare_exchange(0, current_thread(), Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            fatal(FatalReason::Invariant, 0);
        }
        phase(Phase::Publication);
        Self { previous }
    }
}
impl Drop for PublicationGuard {
    fn drop(&mut self) {
        if COUNTER.live() > ORDINARY.load(Ordering::Acquire) {
            fatal(FatalReason::Invariant, 0);
        }
        phase(self.previous);
        PUBLISHER.store(0, Ordering::Release);
    }
}
