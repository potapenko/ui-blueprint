//! Disposable method-level probe of the actual production allocator source.
//! It intentionally does NOT install a global allocator: only explicit valid
//! GlobalAlloc calls are charged, so their exact live layouts are observable.
//! The real session-worker separately proves installed/global coverage.
#![cfg(target_os = "macos")]
#[allow(dead_code)] // Include the complete unchanged owner, not a substitute guard.
#[path = "../../src/quota_allocator.rs"]
mod guard;

use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicBool, Ordering},
};
use uiblueprint_host::{OperationClass, process::DarwinPlatform, process_api::WorkerPlatform};

const ALLOCATOR: guard::GuardedAllocator = guard::GuardedAllocator;
const CAP: usize = 4096;
static INJECTED: AtomicBool = AtomicBool::new(false);

fn injected_null(size: usize) -> bool {
    // SAFETY: fd4 is the disposable probe's inherited owned output lane; one
    // fixed stack byte is written without formatting, allocation or recursion.
    let marker = [0xa1_u8];
    if unsafe { libc::write(4, marker.as_ptr().cast(), marker.len()) } != 1 {
        DarwinPlatform::fatal_exit(-1, &[0; 64], 82);
    }
    size == 64 && !INJECTED.swap(true, Ordering::SeqCst)
}
unsafe fn fail_one_alloc(layout: Layout) -> *mut u8 {
    if injected_null(layout.size()) {
        std::ptr::null_mut()
    } else {
        // SAFETY: caller supplies valid nonzero layout; fallback is unchanged System.
        unsafe { System.alloc(layout) }
    }
}
unsafe fn fail_one_zeroed(layout: Layout) -> *mut u8 {
    if injected_null(layout.size()) {
        std::ptr::null_mut()
    } else {
        // SAFETY: same layout contract, including zeroing on nonnull success.
        unsafe { System.alloc_zeroed(layout) }
    }
}
unsafe fn fail_one_realloc(pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
    // SAFETY: the caller owns this exact initialized old layout. Inspecting it
    // before returning null does not free, resize or transfer the old allocation.
    for offset in 0..layout.size() {
        if unsafe { pointer.add(offset).read_volatile() } != 0xa5 {
            DarwinPlatform::fatal_exit(-1, &[0; 64], 83);
        }
    }
    if injected_null(new_size) {
        std::ptr::null_mut()
    } else {
        // SAFETY: original live pointer/layout and valid new size; System owns
        // the same backend. Null preserves old ownership under its contract.
        unsafe { System.realloc(pointer, layout, new_size) }
    }
}

fn mode() -> u8 {
    let mut mode = [0_u8];
    // SAFETY: fd3 belongs to this disposable process; the destination is one live
    // initialized byte and is not accessed by any other thread.
    if unsafe { libc::read(3, mode.as_mut_ptr().cast(), 1) } != 1 {
        DarwinPlatform::fatal_exit(-1, &[0; 64], 80);
    }
    mode[0]
}
fn allocate(size: usize, align: usize, zeroed: bool) -> *mut u8 {
    let layout = Layout::from_size_align(size, align).expect("small valid layout");
    assert_ne!(size, 0);
    // SAFETY: valid nonzero layouts, allocated through this same GuardedAllocator.
    // Every successful pointer is consumed or released exactly once below.
    unsafe {
        if zeroed {
            ALLOCATOR.alloc_zeroed(layout)
        } else {
            ALLOCATOR.alloc(layout)
        }
    }
}
fn release(pointer: *mut u8, size: usize, align: usize) {
    // SAFETY: every caller supplies the live pointer from allocate/realloc and
    // its exact current layout; no pointer is used after this call.
    unsafe { ALLOCATOR.dealloc(pointer, Layout::from_size_align(size, align).unwrap()) }
}
fn observe(pointer: *mut u8, size: usize, initial_zero: bool) {
    // SAFETY: all offsets are in the caller-owned initialized allocation. Volatile
    // access makes this actual storage proof observable, not an elided allocation.
    for index in 0..size {
        unsafe {
            if initial_zero {
                assert_eq!(pointer.add(index).read_volatile(), 0);
            }
            pointer.add(index).write_volatile(0xa5);
            assert_eq!(pointer.add(index).read_volatile(), 0xa5);
        }
    }
}
fn successful_allocations() {
    let p = allocate(256, 64, false);
    assert_eq!(p as usize % 64, 0);
    observe(p, 256, false);
    // SAFETY: p is live with exactly this layout; new_size is valid with align64.
    // Success replaces the pointer, so the original is never used/freed again.
    let p = unsafe { ALLOCATOR.realloc(p, Layout::from_size_align(256, 64).unwrap(), 1024) };
    for i in 0..256 {
        // SAFETY: realloc preserves the first min(old,new) initialized bytes.
        assert_eq!(unsafe { p.add(i).read_volatile() }, 0xa5);
    }
    observe(p, 1024, false);
    release(p, 1024, 64);
    let zeroed = allocate(128, 32, true);
    observe(zeroed, 128, true);
    release(zeroed, 128, 32);
    let exact = allocate(CAP, 16, false);
    observe(exact, CAP, false);
    release(exact, CAP, 16);
}
fn concurrent_allocations() {
    std::thread::scope(|scope| {
        for _ in 0..2 {
            scope.spawn(|| {
                for _ in 0..8 {
                    let p = allocate(256, 16, true);
                    observe(p, 256, true);
                    release(p, 256, 16);
                }
            });
        }
    });
}
fn main() {
    DarwinPlatform::setup_main(8 * 1_048_576).expect("owned stack/core setup");
    let command = mode();
    guard::configure(
        77,
        CAP,
        2 * CAP,
        1_048_576,
        if command == b'M' { -1 } else { 5 },
    );
    guard::operation(41, OperationClass::Validate, guard::Phase::Decode);
    match command {
        b'0' => (),
        b'A' => successful_allocations(),
        b'Q' => concurrent_allocations(),
        b'O' | b'M' => {
            allocate(CAP + 1, 16, false);
            unreachable!("quota must terminate before System");
        }
        b'R' => {
            let p = allocate(3072, 16, false);
            observe(p, 3072, false);
            // SAFETY: both layouts are valid; even this shrinking realloc needs
            // full new2048+old3072 reservation and must refuse before System.
            unsafe {
                ALLOCATOR.realloc(p, Layout::from_size_align(3072, 16).unwrap(), 2048);
            }
            unreachable!("realloc overlap must be charged")
        }
        b'P' => {
            let publication = guard::PublicationGuard::enter(guard::Phase::Encode);
            let p = allocate(CAP + 1, 16, true);
            observe(p, CAP + 1, true);
            release(p, CAP + 1, 16);
            drop(publication);
        }
        b'I' => {
            let publication = guard::PublicationGuard::enter(guard::Phase::Encode);
            let p = allocate(CAP + 1, 16, false);
            observe(p, CAP + 1, false);
            drop(publication); // Deliberately live ownership; fatal invariant, no unwind.
            unreachable!("publication reserve cannot escape the phase")
        }
        b'N' => {
            let _first = guard::PublicationGuard::enter(guard::Phase::Encode);
            let _second = guard::PublicationGuard::enter(guard::Phase::Encode);
            unreachable!("nested publisher must refuse")
        }
        b'T' => {
            let _publication = guard::PublicationGuard::enter(guard::Phase::Encode);
            std::thread::spawn(|| {
                allocate(CAP + 1, 16, false);
            })
            .join()
            .unwrap();
            unreachable!("another thread cannot use publication reserve")
        }
        b'C' => guard::configure(77, CAP, 2 * CAP, 1_048_576, 5),
        b'S' => guard::fatal(guard::FatalReason::System, 64), // Encoding only, not System-null proof.
        b'F' | b'Z' => {
            let old = allocate(32, 16, false);
            observe(old, 32, false);
            let forward = if command == b'F' {
                fail_one_alloc
            } else {
                fail_one_zeroed
            };
            // SAFETY: valid64-byte layout; callback returns null exactly once,
            // otherwise System with the required semantics, without unwind or
            // guard recursion. Old32 stays owned until the deliberate fatal exit.
            unsafe {
                guard::allocate_with(Layout::from_size_align(64, 16).unwrap(), forward);
            }
            unreachable!("injected null must terminate after releasing only its charge")
        }
        b'B' => {
            // SAFETY: valid bounded layout/callback. Precharge must refuse before
            // calling the forwarder, so its fixed marker must never be written.
            unsafe {
                guard::allocate_with(
                    Layout::from_size_align(CAP + 1, 16).unwrap(),
                    fail_one_alloc,
                );
            }
            unreachable!("quota must precede the forwarding callback")
        }
        b'G' | b'H' => {
            let (old_size, new_size) = if command == b'G' {
                (32, 64)
            } else {
                (3072, 2048)
            };
            let old = allocate(old_size, 16, false);
            observe(old, old_size, false);
            // SAFETY: owned initialized old layout, valid bounded new size and
            // a non-unwinding callback that preserves old storage on injected null.
            unsafe {
                guard::reallocate_with(
                    old,
                    Layout::from_size_align(old_size, 16).unwrap(),
                    new_size,
                    fail_one_realloc,
                );
            }
            unreachable!("injected null or old+new precharge must terminate")
        }
        _ => DarwinPlatform::fatal_exit(-1, &[0; 64], 81),
    }
    // A deliberately invoked existing fatal hook reports its private live counter.
    // This is a probe sentinel, NOT a claim that the parent actually disappeared.
    guard::fatal(guard::FatalReason::ParentGone, command as u64)
}
