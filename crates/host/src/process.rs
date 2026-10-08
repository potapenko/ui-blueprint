//! Public Darwin process ownership; no UI, SDK collector, shell or ambient env.
//! Core must retain/quarantine this owner until try_reap confirms exit. Drop is
//! best-effort only and never grants permission to release host reservations.
#![allow(unsafe_code)]

mod reaping;
mod spawn;
mod worker;

use crate::{HostError, process_api::*};
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd};

/// Stateless OS boundary. Root, child and poll backing are fixed-size owners.
pub struct DarwinPlatform;

#[derive(Clone, Copy)]
enum Ownership {
    Child(libc::pid_t),
    Reaped(ProcessState),
    Lost,
}

/// The PID originates only from this module's successful posix_spawn. No other
/// component may wait/reap these children or change the supported SIGCHLD policy
/// during their lifetime (Darwin has no atomic pidfd kill). Core/embedding owns
/// that stable process-wide invariant; local point checks detect violations.
pub struct DarwinChild {
    input: Option<OwnedFd>,
    output: OwnedFd,
    fatal: OwnedFd,
    ownership: Ownership,
}

fn errno() -> i32 {
    // SAFETY: __error returns this thread's valid errno storage.
    unsafe { *libc::__error() }
}
fn os_error(code: i32) -> HostError {
    match code {
        libc::EACCES | libc::EPERM => HostError::PermissionDenied,
        libc::ENOMEM => HostError::SystemAllocationFailure,
        libc::EMFILE | libc::ENFILE | libc::EAGAIN => HostError::ResourceLimit,
        _ => HostError::Io,
    }
}
fn transfer(result: isize, reading: bool) -> Result<Transfer, HostError> {
    if result > 0 {
        return Ok(Transfer::Bytes(result as usize));
    }
    if result == 0 {
        return Ok(if reading {
            Transfer::Closed
        } else {
            Transfer::Bytes(0)
        });
    }
    match errno() {
        libc::EAGAIN | libc::EINTR => Ok(Transfer::WouldBlock),
        libc::EPIPE | libc::ECONNRESET | libc::ENOTCONN => Ok(Transfer::Closed),
        error => Err(os_error(error)),
    }
}
fn read(fd: BorrowedFd<'_>, bytes: &mut [u8]) -> Result<Transfer, HostError> {
    if bytes.is_empty() {
        return Ok(Transfer::Bytes(0));
    }
    // SAFETY: fd is borrowed from an owned open nonblocking socket; bytes is a
    // writable slice valid for exactly len bytes throughout this syscall.
    transfer(
        unsafe { libc::read(fd.as_raw_fd(), bytes.as_mut_ptr().cast(), bytes.len()) },
        true,
    )
}

impl ProcessPlatform for DarwinPlatform {
    type Child = DarwinChild;
    fn validate_parent_reaping() -> Result<(), HostError> {
        reaping::supported()
    }
    fn spawn(&mut self, spec: &SpawnSpec) -> Result<DarwinChild, HostError> {
        spawn::spawn(spec)
    }
    fn poll(&mut self, interests: &mut [PollInterest<'_>], wait_ms: u32) -> Result<(), HostError> {
        if interests.len() > MAX_POLL_FDS || wait_ms > i32::MAX as u32 {
            return Err(HostError::InvalidLimits);
        }
        let mut fds = [libc::pollfd {
            fd: -1,
            events: 0,
            revents: 0,
        }; MAX_POLL_FDS];
        for (interest, fd) in interests.iter_mut().zip(&mut fds) {
            interest.ready_read = false;
            interest.ready_write = false;
            interest.closed = false;
            fd.fd = interest.fd.as_raw_fd();
            if interest.read {
                fd.events |= libc::POLLIN;
            }
            if interest.write {
                fd.events |= libc::POLLOUT;
            }
        }
        // SAFETY: initialized fixed array has at least interests.len entries;
        // every fd's borrow outlives the call. wait_ms conversion was checked.
        let result = unsafe {
            libc::poll(
                fds.as_mut_ptr(),
                interests.len() as libc::nfds_t,
                wait_ms as i32,
            )
        };
        if result < 0 {
            // The caller recomputes its deadline; never renew a wait after EINTR.
            return if errno() == libc::EINTR {
                Ok(())
            } else {
                Err(os_error(errno()))
            };
        }
        for (interest, fd) in interests.iter_mut().zip(fds) {
            interest.closed = fd.revents & (libc::POLLHUP | libc::POLLERR | libc::POLLNVAL) != 0;
            interest.ready_read = interest.read && fd.revents & (libc::POLLIN | libc::POLLHUP) != 0;
            interest.ready_write =
                interest.write && !interest.closed && fd.revents & libc::POLLOUT != 0;
        }
        Ok(())
    }
}

impl OwnedProcess for DarwinChild {
    fn write_input(&mut self, bytes: &[u8]) -> Result<Transfer, HostError> {
        let Some(input) = &self.input else {
            return Ok(Transfer::Closed);
        };
        if bytes.is_empty() {
            return Ok(Transfer::Bytes(0));
        }
        // SAFETY: live owned socket and immutable slice. Per-send NOSIGNAL avoids
        // changing the supervisor's global SIGPIPE disposition; DONTWAIT bounds IO.
        transfer(
            unsafe {
                libc::send(
                    input.as_raw_fd(),
                    bytes.as_ptr().cast(),
                    bytes.len(),
                    libc::MSG_NOSIGNAL | libc::MSG_DONTWAIT,
                )
            },
            false,
        )
    }
    fn read_output(&mut self, bytes: &mut [u8]) -> Result<Transfer, HostError> {
        read(self.output.as_fd(), bytes)
    }
    fn read_fatal(&mut self, bytes: &mut [u8]) -> Result<Transfer, HostError> {
        read(self.fatal.as_fd(), bytes)
    }
    fn close_input(&mut self) {
        self.input.take();
    }
    fn input_fd(&self) -> Option<BorrowedFd<'_>> {
        self.input.as_ref().map(AsFd::as_fd)
    }
    fn output_fd(&self) -> BorrowedFd<'_> {
        self.output.as_fd()
    }
    fn fatal_fd(&self) -> BorrowedFd<'_> {
        self.fatal.as_fd()
    }
    fn try_reap(&mut self) -> Result<ProcessState, HostError> {
        let pid = match self.ownership {
            Ownership::Reaped(state) => return Ok(state),
            Ownership::Lost => return Err(HostError::CleanupPending),
            Ownership::Child(pid) => pid,
        };
        if reaping::supported().is_err() {
            self.ownership = Ownership::Lost;
            return Err(HostError::CleanupPending);
        }
        let mut status = 0;
        // SAFETY: pid is the exclusive unreaped child returned by our spawn;
        // status points to initialized writable storage, WNOHANG never blocks.
        let result = unsafe { libc::waitpid(pid, &mut status, libc::WNOHANG) };
        if result == 0 {
            return Ok(ProcessState::Running);
        }
        if result < 0 {
            let error = errno();
            if error == libc::EINTR {
                return Ok(ProcessState::Running);
            }
            if error == libc::ECHILD {
                self.ownership = Ownership::Lost;
            }
            return Err(HostError::CleanupPending);
        }
        let state = if libc::WIFEXITED(status) {
            ProcessState::Exited {
                code: libc::WEXITSTATUS(status),
            }
        } else if libc::WIFSIGNALED(status) {
            ProcessState::Signaled {
                signal: libc::WTERMSIG(status),
            }
        } else {
            return Err(HostError::CleanupPending);
        };
        self.ownership = Ownership::Reaped(state);
        Ok(state)
    }
    fn terminate(&mut self) -> Result<(), HostError> {
        if self.try_reap()? != ProcessState::Running {
            return Ok(());
        }
        let Ownership::Child(pid) = self.ownership else {
            return Err(HostError::CleanupPending);
        };
        // Recheck after wait before using the PID. A detected policy loss is
        // irreversible even if another component subsequently restores SIG_DFL.
        if reaping::supported().is_err() {
            self.ownership = Ownership::Lost;
            return Err(HostError::CleanupPending);
        }
        // SAFETY: Core/embedding maintains default/no-auto-reap SIGCHLD and
        // exclusive wait ownership throughout this child's lifetime. Under that
        // invariant, natural exit leaves a zombie reserving this PID until our
        // wait. &mut excludes another owned reap. Checks above fail closed on
        // detected drift; arbitrary unsynchronized native mutation is unsupported.
        if unsafe { libc::kill(pid, libc::SIGKILL) } == 0 {
            return Ok(());
        }
        if errno() == libc::ESRCH && self.try_reap()? != ProcessState::Running {
            return Ok(());
        }
        Err(HostError::CleanupPending)
    }
}

impl Drop for DarwinChild {
    fn drop(&mut self) {
        // Not an acknowledgement: Core must keep its grant/quarantine until an
        // explicit successful try_reap. No blocking destructor or hidden reaper.
        if matches!(self.ownership, Ownership::Child(_)) {
            let _ = self.terminate();
            let _ = self.try_reap();
        }
    }
}

/// Bounded readiness for a caller-owned input descriptor. Never consumes bytes.
/// The caller must be its sole reader until the following bounded read completes.
pub fn input_ready(input: BorrowedFd<'_>, timeout_ms: u32) -> Result<bool, HostError> {
    let mut fd = libc::pollfd {
        fd: input.as_raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    };
    // SAFETY: one initialized pollfd with a descriptor borrowed for this call;
    // poll cannot retain it and the timeout is representable and bounded.
    let result = unsafe { libc::poll(&mut fd, 1, timeout_ms.min(1000) as i32) };
    if result < 0 {
        return if errno() == libc::EINTR {
            Ok(false)
        } else {
            Err(HostError::Io)
        };
    }
    Ok(result > 0)
}
