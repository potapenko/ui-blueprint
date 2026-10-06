//! Dedicated single-process diagnostic allocator. Never linked into library/CLI
//! production targets. std capacities cannot expose parser/BTree working storage.
use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
static INVALID: AtomicBool = AtomicBool::new(false);
pub struct CountingSystem;

fn add(bytes: usize) {
    match LIVE.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_add(bytes)) {
        Ok(before) => {
            PEAK.fetch_max(before + bytes, Ordering::SeqCst);
        }
        Err(_) => INVALID.store(true, Ordering::SeqCst),
    }
}
fn sub(bytes: usize) {
    if LIVE
        .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(bytes))
        .is_err()
    {
        INVALID.store(true, Ordering::SeqCst);
    }
}

// SAFETY: every pointer/layout is forwarded unchanged to System, with no pointer
// dereference or ownership substitution. Successful realloc changes accounting
// only after System returns; failed allocation/realloc leaves ownership unchanged.
// Counters are allocation-free atomics, never formatting/locking/allocating from
// allocator callbacks. All allocations and deallocations use this same System.
unsafe impl GlobalAlloc for CountingSystem {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: GlobalAlloc's caller supplies a valid nonzero layout.
        let p = unsafe { System.alloc(layout) };
        if !p.is_null() {
            add(layout.size());
        }
        p
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: forwarding the valid layout preserves zero-initialization.
        let p = unsafe { System.alloc_zeroed(layout) };
        if !p.is_null() {
            add(layout.size());
        }
        p
    }
    unsafe fn dealloc(&self, p: *mut u8, layout: Layout) {
        // SAFETY: caller supplies a live allocation and its original layout.
        unsafe { System.dealloc(p, layout) };
        sub(layout.size());
    }
    unsafe fn realloc(&self, p: *mut u8, old: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: System receives the original pointer/layout and valid new size.
        let result = unsafe { System.realloc(p, old, new_size) };
        if !result.is_null() {
            if new_size >= old.size() {
                add(new_size - old.size());
            } else {
                sub(old.size() - new_size);
            }
        }
        result
    }
}

pub struct Mark {
    baseline: usize,
}
pub struct Stats {
    pub retained_delta: i128,
    pub peak_above_baseline: usize,
    pub baseline: usize,
}
/// Measurements are sequential in one dedicated example process. Resetting peak
/// while unrelated work runs would make attribution invalid; callers must not do it.
pub fn begin() -> Mark {
    let baseline = LIVE.load(Ordering::SeqCst);
    PEAK.store(baseline, Ordering::SeqCst);
    Mark { baseline }
}
pub fn finish(mark: Mark) -> Result<Stats, &'static str> {
    if INVALID.load(Ordering::SeqCst) {
        return Err("allocator accounting overflow/underflow");
    }
    Ok(Stats {
        retained_delta: LIVE.load(Ordering::SeqCst) as i128 - mark.baseline as i128,
        peak_above_baseline: PEAK.load(Ordering::SeqCst).saturating_sub(mark.baseline),
        baseline: mark.baseline,
    })
}
pub fn live() -> usize {
    LIVE.load(Ordering::SeqCst)
}
