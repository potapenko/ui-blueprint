//! Narrow owned-process boundary for the platform implementation. No child PID,
//! argv or environment is taken from canonical UI payloads. Implementations must
//! be allocation-free in steady-state and report only fixed error categories.
use crate::HostError;
use std::{
    ffi::CStr,
    os::{fd::BorrowedFd, unix::ffi::OsStrExt},
    path::Path,
};

pub const CHILD_INPUT_FD: i32 = 3;
pub const CHILD_OUTPUT_FD: i32 = 4;
pub const CHILD_FATAL_FD: i32 = 5;
pub const MAX_PATH_BYTES: usize = 4096;
pub const MAX_POLL_FDS: usize = crate::limits::MAX_WORKERS * (1 + crate::limits::HELPERS) * 3;

/// Trusted executable location, owned in a fixed root. No inherited environment,
/// shell, extra arguments or unbounded OsString collection is requested.
pub struct SpawnSpec {
    path: [u8; MAX_PATH_BYTES],
    length: usize,
}
impl SpawnSpec {
    pub fn new(path: &Path) -> Result<Self, HostError> {
        let bytes = path.as_os_str().as_bytes();
        if !path.is_absolute()
            || bytes.is_empty()
            || bytes.len() >= MAX_PATH_BYTES
            || bytes.contains(&0)
        {
            return Err(HostError::InvalidInput);
        }
        let mut result = Self {
            path: [0; MAX_PATH_BYTES],
            length: bytes.len(),
        };
        result.path[..bytes.len()].copy_from_slice(bytes);
        Ok(result)
    }
    pub fn executable(&self) -> &CStr {
        // Constructor proved a single final NUL and no interior NUL.
        CStr::from_bytes_with_nul(&self.path[..=self.length]).expect("bounded SpawnSpec invariant")
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Exited { code: i32 },
    Signaled { signal: i32 },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Transfer {
    Bytes(usize),
    WouldBlock,
    Closed,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChildSetup {
    pub main_stack_bytes: usize,
    pub watchdog_stack_bytes: usize,
}

/// Each value must come from a real child created by this platform owner.
/// Termination targets only that unreaped child; no PID supplied by caller/UI.
/// All parent descriptors are nonblocking/CLOEXEC. Spawn maps child protocol to
/// 3/4/5; 0/1/2 go to /dev/null. No other session descriptor is inherited.
/// Drop must not report successful cleanup before confirmed reap.
pub trait OwnedProcess {
    fn write_input(&mut self, bytes: &[u8]) -> Result<Transfer, HostError>;
    fn read_output(&mut self, bytes: &mut [u8]) -> Result<Transfer, HostError>;
    fn read_fatal(&mut self, bytes: &mut [u8]) -> Result<Transfer, HostError>;
    fn close_input(&mut self);
    fn terminate(&mut self) -> Result<(), HostError>;
    fn try_reap(&mut self) -> Result<ProcessState, HostError>;
    fn input_fd(&self) -> Option<BorrowedFd<'_>>;
    fn output_fd(&self) -> BorrowedFd<'_>;
    fn fatal_fd(&self) -> BorrowedFd<'_>;
}
/// Fixed interests supplied by the supervisor; no owned Vec/string/graph here.
pub struct PollInterest<'a> {
    pub fd: BorrowedFd<'a>,
    pub read: bool,
    pub write: bool,
    pub ready_read: bool,
    pub ready_write: bool,
    pub closed: bool,
}
pub trait ProcessPlatform {
    type Child: OwnedProcess;
    /// Implement exact-path posix_spawn with bounded argv/env/file actions;
    /// returned PID/FDs are owned and tied to this spawn until wait/reap.
    fn spawn(&mut self, spec: &SpawnSpec) -> Result<Self::Child, HostError>;
    /// At most MAX_POLL_FDS; parent supplies a bounded relative wait, not a new
    /// clock domain. EINTR returns without hidden retry/deadline extension.
    fn poll(&mut self, interests: &mut [PollInterest<'_>], wait_ms: u32) -> Result<(), HostError>;
}

/// Platform child setup must set RLIMIT_CORE=0 and request/verify the main stack
/// before reading untrusted input. The worker separately verifies its fixed-size
/// watchdog stack through the same platform owner. No GlobalAlloc is installed
/// in the host library; allocator fatal callbacks use its executable-private path.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StackProof {
    pub main_stack_bytes: usize,
    pub watchdog_stack_bytes: usize,
    pub core_dumps_disabled: bool,
}

/// Executable-side platform hooks. The implementation belongs to the same narrow
/// OS owner, and MUST perform no Rust heap allocation, formatting, panic or unwind.
/// Core invokes setup before untrusted input and invokes current_stack_bytes from
/// the actual watchdog thread; no placeholder stack-proof values are acceptable.
pub trait WorkerPlatform {
    fn setup_main(main_stack_bytes: usize) -> Result<usize, HostError>;
    /// Compute a supported creation request with checked validation/arithmetic,
    /// without raising the caller's ceiling. This is NOT actual extent proof:
    /// caller MUST check current_stack_bytes on the created watchdog and refuse
    /// untrusted work unless that actual extent is within the original ceiling.
    fn watchdog_stack_request(ceiling: usize) -> Result<usize, HostError>;
    fn current_stack_bytes() -> Result<usize, HostError>;
    /// Best-effort one fixed write to the configured nonblocking private status FD,
    /// then _exit. fd<0 skips the write (bootstrap status unavailable); the parent
    /// must classify that absence as generic failure, not guess quota exhaustion.
    fn fatal_exit(fd: i32, record: &[u8; crate::CONTROL_BYTES], exit_code: i32) -> !;
}
