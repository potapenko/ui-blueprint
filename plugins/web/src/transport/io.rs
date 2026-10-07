use super::{ErrorKind, OperationLimits};
use std::{
    io::{self, Read, Write},
    net::{Shutdown, TcpStream},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

pub(super) trait SocketIo: Read + Write + Send {
    fn timeouts(&self, remaining: Duration) -> io::Result<()>;
    fn shutdown(&self);
}
impl SocketIo for TcpStream {
    fn timeouts(&self, remaining: Duration) -> io::Result<()> {
        self.set_read_timeout(Some(remaining))?;
        self.set_write_timeout(Some(remaining))
    }
    fn shutdown(&self) {
        let _ = self.shutdown(Shutdown::Both);
    }
}
struct CancelState {
    cancelled: AtomicBool,
    // One shutdown-only duplicate. Taking it closes the FD even if handles survive.
    socket: Mutex<Option<TcpStream>>,
}
#[derive(Clone)]
pub struct Cancellation {
    state: Arc<CancelState>,
}
impl std::fmt::Debug for Cancellation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Cancellation")
    }
}
impl Cancellation {
    pub(super) fn new(socket: Option<TcpStream>) -> Self {
        Self {
            state: Arc::new(CancelState {
                cancelled: AtomicBool::new(false),
                socket: Mutex::new(socket),
            }),
        }
    }
    pub(super) fn attach(&self, socket: TcpStream) -> Result<(), ErrorKind> {
        let mut slot = self.state.socket.lock().unwrap_or_else(|p| p.into_inner());
        if self.is_cancelled() {
            let _ = socket.shutdown(Shutdown::Both);
            return Err(ErrorKind::Cancelled);
        }
        *slot = Some(socket);
        Ok(())
    }
    /// Permanently cancels this owned connection and wakes blocked socket IO.
    /// Already delivered bytes are not rolled back.
    pub fn cancel(&self) {
        self.state.cancelled.store(true, Ordering::Release);
        let mut slot = self
            .state
            .socket
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(socket) = slot.take() {
            let _ = socket.shutdown(Shutdown::Both);
        }
    }
    pub fn is_cancelled(&self) -> bool {
        self.state.cancelled.load(Ordering::Acquire)
    }
}

pub(super) struct Guarded {
    stream: Box<dyn SocketIo>,
    pub cancel: Cancellation,
    limits: OperationLimits,
    read: usize,
    written: usize,
    delivered: usize,
    work: usize,
    pub failure: Option<ErrorKind>,
}
impl Guarded {
    pub fn new(stream: Box<dyn SocketIo>, cancel: Cancellation, limits: OperationLimits) -> Self {
        Self {
            stream,
            cancel,
            limits,
            read: 0,
            written: 0,
            delivered: 0,
            work: 0,
            failure: None,
        }
    }
    pub fn reset(&mut self, limits: OperationLimits) {
        self.limits = limits;
        self.read = 0;
        self.written = 0;
        self.delivered = 0;
        self.work = 0;
        self.failure = None;
    }
    pub fn written(&self) -> usize {
        self.written
    }
    pub fn check(&mut self) -> Result<Duration, ErrorKind> {
        let failure = if self.cancel.is_cancelled() {
            Some(ErrorKind::Cancelled)
        } else if Instant::now() >= self.limits.deadline {
            Some(ErrorKind::Timeout)
        } else {
            self.failure
        };
        if let Some(kind) = failure {
            self.failure = Some(kind);
            return Err(kind);
        }
        Ok(self
            .limits
            .deadline
            .saturating_duration_since(Instant::now()))
    }
    pub fn step(&mut self) -> Result<(), ErrorKind> {
        self.check()?;
        if self.work >= self.limits.max_work {
            self.failure = Some(ErrorKind::WorkLimit);
            return Err(ErrorKind::WorkLimit);
        }
        self.work += 1;
        Ok(())
    }
    pub fn deliver(&mut self, bytes: usize) -> Result<(), ErrorKind> {
        self.check()?;
        if bytes > self.limits.max_read_bytes.saturating_sub(self.delivered) {
            self.failure = Some(ErrorKind::ByteLimit);
            return Err(ErrorKind::ByteLimit);
        }
        self.delivered += bytes;
        Ok(())
    }
    fn before_io(&mut self, remaining_bytes: usize) -> io::Result<()> {
        self.step().map_err(io_error)?;
        if remaining_bytes == 0 {
            self.failure = Some(ErrorKind::ByteLimit);
            return Err(io_error(ErrorKind::ByteLimit));
        }
        let remaining = self.check().map_err(io_error)?;
        if remaining.is_zero() {
            self.failure = Some(ErrorKind::Timeout);
            return Err(io_error(ErrorKind::Timeout));
        }
        self.stream.timeouts(remaining)
    }
    pub fn shutdown(&self) {
        self.cancel.cancel();
        self.stream.shutdown();
    }
}
#[derive(Debug)]
struct GuardFailure(ErrorKind);
impl std::fmt::Display for GuardFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("transport operation refused")
    }
}
impl std::error::Error for GuardFailure {}
pub(super) fn failure_kind(error: &io::Error) -> Option<ErrorKind> {
    error.get_ref()?.downcast_ref::<GuardFailure>().map(|e| e.0)
}
fn io_error(kind: ErrorKind) -> io::Error {
    io::Error::new(
        match kind {
            ErrorKind::Timeout => io::ErrorKind::TimedOut,
            ErrorKind::Cancelled => io::ErrorKind::Interrupted,
            _ => io::ErrorKind::Other,
        },
        GuardFailure(kind),
    )
}
impl Read for Guarded {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }
        let left = self.limits.max_read_bytes.saturating_sub(self.read);
        self.before_io(left)?;
        let size = buffer.len().min(left);
        let result = self.stream.read(&mut buffer[..size]);
        if result.is_err() {
            self.check().map_err(io_error)?;
        }
        let n = result?;
        self.read += n;
        self.check().map_err(io_error)?;
        Ok(n)
    }
}
impl Write for Guarded {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }
        let left = self.limits.max_write_bytes.saturating_sub(self.written);
        self.before_io(left)?;
        let result = self.stream.write(&buffer[..buffer.len().min(left)]);
        if result.is_err() {
            self.check().map_err(io_error)?;
        }
        let n = result?;
        if n == 0 {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "transport write made no progress",
            ));
        }
        self.written += n;
        self.check().map_err(io_error)?;
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.step().map_err(io_error)?;
        let remaining = self.check().map_err(io_error)?;
        if remaining.is_zero() {
            self.failure = Some(ErrorKind::Timeout);
            return Err(io_error(ErrorKind::Timeout));
        }
        self.stream.timeouts(remaining)?;
        self.stream.flush()?;
        self.check().map_err(io_error)?;
        Ok(())
    }
}
impl Drop for Guarded {
    fn drop(&mut self) {
        self.shutdown();
    }
}
