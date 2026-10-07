//! Minimal managed-parent lifetime ownership. It never installs or resets a signal
//! handler. Foreign sigaction/wait callers must honor the documented embedding
//! contract; arbitrary same-process native interference is not sandboxed.
use crate::HostError;
use std::{
    cell::Cell,
    sync::atomic::{AtomicBool, Ordering},
};
static PARENT_OWNED: AtomicBool = AtomicBool::new(false);

pub(crate) struct ParentReapingLease {
    validate: fn() -> Result<(), HostError>,
    poisoned: Cell<bool>,
}
impl ParentReapingLease {
    pub(crate) fn acquire(validate: fn() -> Result<(), HostError>) -> Result<Self, HostError> {
        if PARENT_OWNED
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return Err(HostError::Busy);
        }
        let lease = Self {
            validate,
            poisoned: Cell::new(false),
        };
        validate()?; // Failure drops the unique claim before any child exists.
        Ok(lease)
    }
    pub(crate) fn check(&self) -> Result<(), HostError> {
        if self.poisoned.get() {
            return Err(HostError::CleanupPending);
        }
        if (self.validate)().is_err() {
            self.poisoned.set(true);
            return Err(HostError::CleanupPending);
        }
        Ok(())
    }
    pub(crate) fn poisoned(&self) -> bool {
        self.poisoned.get()
    }
}
impl Drop for ParentReapingLease {
    fn drop(&mut self) {
        PARENT_OWNED.store(false, Ordering::Release);
    }
}
