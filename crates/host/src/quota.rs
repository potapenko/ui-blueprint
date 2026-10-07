//! Allocation-free checked accounting used by the executable's GlobalAlloc.
//! This module does not install an allocator or claim System/RSS accounting.
use std::sync::atomic::{AtomicUsize, Ordering};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChargeError {
    Limit,
    Overflow,
    Underflow,
}
pub struct QuotaCounter {
    live: AtomicUsize,
    peak: AtomicUsize,
}
impl QuotaCounter {
    pub const fn new() -> Self {
        Self {
            live: AtomicUsize::new(0),
            peak: AtomicUsize::new(0),
        }
    }
    pub fn charge(&self, bytes: usize, limit: usize) -> Result<(), ChargeError> {
        let mut current = self.live.load(Ordering::Acquire);
        loop {
            let next = current.checked_add(bytes).ok_or(ChargeError::Overflow)?;
            if next > limit {
                return Err(ChargeError::Limit);
            }
            match self.live.compare_exchange_weak(
                current,
                next,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    self.peak.fetch_max(next, Ordering::Relaxed);
                    return Ok(());
                }
                Err(actual) => current = actual,
            }
        }
    }
    pub fn release(&self, bytes: usize) -> Result<(), ChargeError> {
        self.live
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
                current.checked_sub(bytes)
            })
            .map(|_| ())
            .map_err(|_| ChargeError::Underflow)
    }
    pub fn live(&self) -> usize {
        self.live.load(Ordering::Acquire)
    }
    pub fn peak(&self) -> usize {
        self.peak.load(Ordering::Relaxed)
    }
}
impl Default for QuotaCounter {
    fn default() -> Self {
        Self::new()
    }
}
