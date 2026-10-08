//! Worker-only adoption of the inherited protocol endpoints. No spawning/reaping
//! implementation is duplicated here; those remain Native's process owner.
#![allow(unsafe_code)]
use std::{
    fs::File,
    io::{Read, Write},
    os::fd::FromRawFd,
};
use uiblueprint_host::{
    CONTROL_BYTES, Control, HostError,
    process_api::{CHILD_FATAL_FD, CHILD_INPUT_FD, CHILD_OUTPUT_FD},
};
pub struct WorkerIo {
    input: File,
    output: File,
    diagnostic: Option<uiblueprint_host::diagnostic::DiagnosticRecord>,
}
impl WorkerIo {
    pub fn inherited() -> Result<Self, HostError> {
        for fd in [CHILD_INPUT_FD, CHILD_OUTPUT_FD, CHILD_FATAL_FD] {
            // SAFETY: fcntl with F_GETFD has no additional pointer argument.
            if unsafe { libc::fcntl(fd, libc::F_GETFD) } < 0 {
                return Err(HostError::Io);
            }
        }
        // SAFETY: private worker entry adopts descriptors 3/4 exactly once after
        // proving they exist. The registered spawn contract transfers their sole
        // child-process ownership; watchdog only borrows input until joined.
        Ok(Self {
            input: unsafe { File::from_raw_fd(CHILD_INPUT_FD) },
            output: unsafe { File::from_raw_fd(CHILD_OUTPUT_FD) },
            diagnostic: None,
        })
    }
    /// Operation-thread-only inline metadata, with no allocation or logging.
    #[cfg(feature = "web")]
    pub(super) fn set_diagnostic(
        &mut self,
        record: uiblueprint_host::diagnostic::DiagnosticRecord,
    ) {
        self.diagnostic = Some(record);
    }
    pub(super) fn take_diagnostic(
        &mut self,
    ) -> Option<uiblueprint_host::diagnostic::DiagnosticRecord> {
        self.diagnostic.take()
    }
    pub fn control(&self) -> Result<Control, HostError> {
        let mut bytes = [0; CONTROL_BYTES];
        (&self.input)
            .read_exact(&mut bytes)
            .map_err(|_| HostError::Io)?;
        Control::decode(&bytes)
    }
    pub fn read(&self, bytes: &mut [u8]) -> Result<(), HostError> {
        (&self.input).read_exact(bytes).map_err(|_| HostError::Io)
    }
    pub fn write_control(&self, control: Control) -> Result<(), HostError> {
        (&self.output)
            .write_all(&control.encode())
            .map_err(|_| HostError::Io)
    }
    pub fn write(&self, bytes: &[u8]) -> Result<(), HostError> {
        (&self.output).write_all(bytes).map_err(|_| HostError::Io)
    }
}

/// Allocation-free liveness wait. Input File remains owned by the operation
/// thread until the watchdog has joined; this function never consumes its bytes.
pub fn parent_alive(wait_ms: i32) -> bool {
    let mut fd = libc::pollfd {
        fd: CHILD_INPUT_FD,
        // Darwin does not report socket EOF/HUP for an empty interest mask.
        // Request readability so peer closure is reported; polling never reads
        // or consumes the operation thread's protocol bytes.
        events: libc::POLLIN,
        revents: 0,
    };
    // SAFETY: one initialized pollfd, inherited live input FD; wait is bounded.
    let result = unsafe { libc::poll(&mut fd, 1, wait_ms) };
    if result < 0 {
        // SAFETY: __error returns this thread's errno storage. Interrupted waits
        // defer to the next bounded watchdog iteration without extending deadline.
        return unsafe { *libc::__error() } == libc::EINTR;
    }
    fd.revents & (libc::POLLHUP | libc::POLLERR | libc::POLLNVAL) == 0
}
