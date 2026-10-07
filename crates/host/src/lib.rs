//! Bounded session-host owners. Worker/process enforcement is layered onto these
//! fixed limits, leases and control records; buffers alone are not a memory guard.
#![deny(unsafe_code)]

pub mod authority;
pub mod buffers;
#[doc(hidden)]
pub mod diagnostic;
#[cfg(unix)]
pub mod domain;
#[cfg(unix)]
mod effects;
#[cfg(unix)]
pub mod helpers;
#[cfg(unix)]
pub mod host_types;
pub mod limits;
#[cfg(unix)]
pub mod native_binding;

#[cfg(target_os = "macos")]
pub mod process;
#[cfg(unix)]
pub mod process_api;
pub mod protocol;
pub mod publication;
pub mod quota;
#[cfg(unix)]
mod reaping;
#[cfg(unix)]
pub mod supervisor;
#[cfg(feature = "web")]
pub mod web_config;
pub mod worker_config;
pub mod worker_tape;

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
    /// Valid typed action refused by current provider facts, not malformed input.
    ActionRefused,
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
