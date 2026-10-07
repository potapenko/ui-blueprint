//! Bounded session-host owners. Worker/process enforcement is layered onto these
//! fixed limits, leases and control records; buffers alone are not a memory guard.
#![deny(unsafe_code)]

pub mod buffers;
pub mod limits;
#[cfg(unix)]
pub mod process_api;
pub mod protocol;

pub use buffers::{BufferClass, ByteLease, ParentBuffers, PoolUsage};
pub use limits::HostLimits;
pub use protocol::{CONTROL_BYTES, Control, ControlKind, Correlation, OperationClass};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostError {
    InvalidLimits,
    ResourceLimit,
    AllocationFailure,
    Overflow,
    Busy,
    InvalidInput,
    InvalidState,
    InvalidControl,
    StaleOperation,
    DeadlineExpired,
    PermissionDenied,
    Io,
    WorkerFailed,
    SystemAllocationFailure,
    CleanupPending,
    ResyncRequired,
}
impl std::fmt::Display for HostError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl std::error::Error for HostError {}

pub(crate) fn add(a: usize, b: usize) -> Result<usize, HostError> {
    a.checked_add(b).ok_or(HostError::Overflow)
}
pub(crate) fn mul(a: usize, b: usize) -> Result<usize, HostError> {
    a.checked_mul(b).ok_or(HostError::Overflow)
}
