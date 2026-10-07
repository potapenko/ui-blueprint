//! Terminal publication-only probe; pre-existing DTOs are System-owned and never
//! dropped in the intercepted window. Output backing/ballast are explicitly
//! charged by the real guard. Not whole-worker or OS/RSS accounting.
#![cfg(target_os = "macos")]
// Include exact private production owners rather than a substitute encoder/guard.
#[allow(dead_code)]
#[path = "../../src/quota_allocator.rs"]
mod quota_allocator;
#[allow(dead_code)]
#[path = "../../src/worker_io.rs"]
mod worker_io;
#[allow(dead_code)]
#[path = "../../src/worker_main.rs"]
mod worker_main;
#[allow(dead_code)]
#[path = "../../src/worker_native.rs"]
mod worker_native;
#[allow(dead_code)]
#[path = "../../src/worker_ops.rs"]
mod worker_ops;
#[cfg(feature = "web")]
#[allow(dead_code)]
#[path = "../../src/worker_web.rs"]
mod worker_web;
use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};
use uiblueprint_host::worker_tape;
use uiblueprint_host::{process::DarwinPlatform, process_api::WorkerPlatform, *};
use uiblueprint_schema::{SchemaVersion, analysis::AnalysisDocument, model::*};
static ACTIVE: AtomicBool = AtomicBool::new(false);
static CALLS: AtomicUsize = AtomicUsize::new(0);
static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
struct Window;
const GUARD: quota_allocator::GuardedAllocator = quota_allocator::GuardedAllocator;
fn charged(n: usize) {
    CALLS.fetch_add(1, Ordering::Relaxed);
    let live = LIVE.fetch_add(n, Ordering::Relaxed) + n;
    PEAK.fetch_max(live, Ordering::Relaxed);
}
fn released(n: usize) {
    if LIVE.fetch_sub(n, Ordering::Relaxed) < n {
        quota_allocator::fatal(quota_allocator::FatalReason::Invariant, 0);
    }
}
// SAFETY: this isolated single-thread terminal window routes only its new scratch
// allocations through the real guard; old DTO/backing ownership is never mixed.
// Exact pointer/layout contracts are forwarded, with no callback allocation/unwind.
unsafe impl GlobalAlloc for Window {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        if ACTIVE.load(Ordering::Relaxed) {
            charged(l.size());
            unsafe { GUARD.alloc(l) }
        } else {
            unsafe { System.alloc(l) }
        }
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        if ACTIVE.load(Ordering::Relaxed) {
            charged(l.size());
            unsafe { GUARD.alloc_zeroed(l) }
        } else {
            unsafe { System.alloc_zeroed(l) }
        }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        if ACTIVE.load(Ordering::Relaxed) {
            charged(n);
            let value = unsafe { GUARD.realloc(p, l, n) };
            released(l.size());
            value
        } else {
            unsafe { System.realloc(p, l, n) }
        }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        if ACTIVE.load(Ordering::Relaxed) {
            unsafe { GUARD.dealloc(p, l) };
            released(l.size());
        } else {
            unsafe { System.dealloc(p, l) }
        }
    }
}
#[global_allocator]
static ALLOCATOR: Window = Window;
const BUFFER: usize = 512 * 1024;
const BALLAST: usize = 4096;
fn core() -> Document {
    Document::from_json(
        include_bytes!("../../../../fixtures/golden/ENV-SNAPSHOT-VALID.json"),
        65536,
    )
    .unwrap()
}
fn main() {
    DarwinPlatform::setup_main(8 * 1_048_576).unwrap();
    std::panic::set_hook(Box::new(|_| {
        quota_allocator::fatal(quota_allocator::FatalReason::Panic, 0)
    }));
    let mut io = worker_io::WorkerIo::inherited().unwrap();
    let operation = io.control().unwrap();
    let mode = operation.value;
    assert!(mode <= 6);
    let mut snapshot = core();
    if mode == 1 {
        let Artifact::Snapshot(s) = &mut snapshot.artifact else {
            unreachable!()
        };
        let mut changed = false;
        for node in &mut s.nodes {
            for property in &mut node.properties {
                if let Property::Requested {
                    state:
                        Availability::Known {
                            value: Value::Text(text),
                        },
                    ..
                } = property
                    && !changed
                {
                    *text = "\0\n\"\\é".repeat(10_000);
                    changed = true;
                }
            }
        }
        assert!(changed);
        snapshot.validate().unwrap();
    }
    let measurement = AnalysisDocument::from_json(
        include_bytes!("../../../../fixtures/analysis/measurement-gap.json"),
        65536,
    )
    .unwrap();
    let check = AnalysisDocument::from_json(
        include_bytes!("../../../../fixtures/analysis/check-converted.json"),
        65536,
    )
    .unwrap();
    let Artifact::Snapshot(s) = &snapshot.artifact else {
        unreachable!()
    };
    let failed = Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::ChannelResponse(Box::new(ChannelResponse {
            request_id: Id("publication".into()),
            session_id: s.context.session_id.clone(),
            dispatch_sequence: 1,
            target: s.context.target.clone(),
            channel: Channel::RenderedCapture,
            result: ChannelResult::Failed(Issue {
                code: ErrorCode::PermissionRequired,
                scope_id: s.context.scope_id.clone(),
                failed_step: Some(Id("capture".into())),
                recovery_class: Id("explicit_permission".into()),
            }),
        })),
    };
    failed.validate().unwrap();
    quota_allocator::configure(
        operation.correlation.session_epoch,
        BUFFER + BALLAST,
        BUFFER + BALLAST + 1_048_576,
        1_048_576,
        5,
    );
    quota_allocator::operation(
        operation.correlation.operation,
        operation.class,
        quota_allocator::Phase::Encode,
    );
    let output_layout = Layout::from_size_align(BUFFER, 1).unwrap();
    let ballast_layout = Layout::from_size_align(BALLAST, 16).unwrap();
    // SAFETY: valid nonzero layouts, exclusively owned until the final explicit
    // deallocation. The output is initialized; no borrowed slice survives use.
    let output = unsafe { GUARD.alloc_zeroed(output_layout) };
    let ballast = unsafe { GUARD.alloc(ballast_layout) };
    for i in 0..BALLAST {
        unsafe {
            ballast.add(i).write_volatile(0xa5);
            assert_eq!(ballast.add(i).read_volatile(), 0xa5);
        }
    }
    let limit = if mode == 5 { 16 } else { BUFFER };
    let bytes = unsafe { std::slice::from_raw_parts_mut(output, limit) };
    let mut fixed = worker_main::FixedOutput { bytes, used: 0 };
    ACTIVE.store(true, Ordering::Release);
    let reserve = quota_allocator::PublicationGuard::enter(quota_allocator::Phase::Encode);
    let encoded = match mode {
        2 => serde_json::to_writer(&mut fixed, &measurement),
        3 => serde_json::to_writer(&mut fixed, &check),
        4 => serde_json::to_writer(&mut fixed, &failed),
        _ => serde_json::to_writer(&mut fixed, &snapshot),
    };
    let writer_failed = match encoded {
        Ok(()) => false,
        Err(error) => {
            assert!(error.is_io());
            drop(error);
            true
        }
    };
    let mut io_failed = false;
    if !writer_failed {
        io_failed =
            worker_main::publish(&mut io, operation, 0, &fixed.bytes[..fixed.used], false).is_err();
    }
    let used = fixed.used;
    drop(reserve); // Must return to the already-full ordinary level.
    ACTIVE.store(false, Ordering::Release);
    assert_eq!(LIVE.load(Ordering::Relaxed), 0);
    assert_eq!(writer_failed, mode == 5);
    assert_eq!(io_failed, mode == 6);
    io.write_control(Control {
        kind: ControlKind::Terminal,
        class: operation.class,
        slot: 0,
        flags: u8::from(writer_failed) | u8::from(io_failed) << 1,
        correlation: operation.correlation,
        length: used as u64,
        value: CALLS.load(Ordering::Relaxed) as u64,
        auxiliary: PEAK.load(Ordering::Relaxed) as u64,
    })
    .unwrap();
    // SAFETY: same live uniquely owned allocations and original exact layouts;
    // no output access follows. Pre-window System DTOs are never dropped here.
    unsafe {
        GUARD.dealloc(ballast, ballast_layout);
        GUARD.dealloc(output, output_layout);
    }
    quota_allocator::fatal(quota_allocator::FatalReason::ParentGone, mode); // Explicit reporting sentinel, not liveness proof.
}
