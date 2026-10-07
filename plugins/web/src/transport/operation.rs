use super::{CloseStatus, ErrorKind, Event, Failure, SendProgress, SendStatus, Transport};
use tungstenite::Message;

pub struct Operation<'a> {
    transport: &'a mut Transport,
    pending: bool,
    control_pending: bool,
    peer_closed: bool,
    progress: SendProgress,
    write_start: usize,
}
impl<'a> Operation<'a> {
    pub(super) fn new(transport: &'a mut Transport) -> Self {
        Self {
            transport,
            pending: false,
            control_pending: false,
            peer_closed: false,
            progress: SendProgress::NotQueued,
            write_start: 0,
        }
    }
    fn preflight(&mut self) -> Result<(), Failure> {
        let result = self
            .transport
            .ws
            .as_mut()
            .ok_or(Failure::plain(ErrorKind::Closed))?
            .get_mut()
            .step();
        result.map_err(|kind| self.fail(kind))
    }
    fn fail(&mut self, kind: ErrorKind) -> Failure {
        self.update_progress();
        let error = Failure {
            kind,
            send_progress: self.progress,
        };
        self.transport.abort();
        error
    }
    fn update_progress(&mut self) {
        if self.pending
            && self
                .transport
                .ws
                .as_ref()
                .is_some_and(|w| w.get_ref().written() > self.write_start)
        {
            self.progress = SendProgress::PossiblyWritten;
        }
    }
    fn classify(&mut self, error: tungstenite::Error) -> Failure {
        let guard_kind = self
            .transport
            .ws
            .as_mut()
            .and_then(|w| w.get_mut().check().err());
        let kind = guard_kind.unwrap_or(match error {
            tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed => {
                ErrorKind::Closed
            }
            tungstenite::Error::Capacity(_) | tungstenite::Error::WriteBufferFull(_) => {
                ErrorKind::Capacity
            }
            tungstenite::Error::Io(_) => ErrorKind::Io,
            _ => ErrorKind::Protocol,
        });
        self.fail(kind)
    }
    fn would_block(&mut self, error: &tungstenite::Error) -> bool {
        matches!(error,tungstenite::Error::Io(e) if matches!(e.kind(),std::io::ErrorKind::WouldBlock|std::io::ErrorKind::TimedOut))
            && self
                .transport
                .ws
                .as_mut()
                .is_some_and(|w| w.get_mut().check().is_ok())
    }
    /// Flushed only means bytes accepted by the socket. Never CDP/application success.
    pub fn send_text(&mut self, text: &str) -> Result<SendStatus, Failure> {
        if self.pending || self.control_pending {
            return Err(Failure {
                kind: ErrorKind::PendingWrite,
                send_progress: self.progress,
            });
        }
        self.progress = SendProgress::NotQueued;
        self.preflight()?;
        if text.len() > self.transport.limits.outbound_bytes {
            return Err(Failure::plain(ErrorKind::Capacity));
        }
        let Some(ws) = self.transport.ws.as_mut() else {
            return Err(Failure::plain(ErrorKind::Closed));
        };
        if !ws.can_write() {
            return Err(self.fail(ErrorKind::Closed));
        }
        self.write_start = ws.get_ref().written();
        self.pending = true;
        self.progress = SendProgress::Queued;
        let result = ws.write(Message::text(text));
        match result {
            Ok(()) => self.resume_send(),
            Err(tungstenite::Error::WriteBufferFull(_)) => {
                self.pending = false;
                self.progress = SendProgress::NotQueued;
                Err(Failure::plain(ErrorKind::Capacity))
            }
            Err(error) if self.would_block(&error) => {
                self.update_progress();
                Ok(SendStatus::Pending)
            }
            Err(error) => Err(self.classify(error)),
        }
    }
    /// Resumes the original queued bytes under the same operation budget/deadline.
    pub fn resume_send(&mut self) -> Result<SendStatus, Failure> {
        self.preflight()?;
        if !self.pending {
            return Err(Failure::plain(ErrorKind::NoPendingWrite));
        }
        let result = self
            .transport
            .ws
            .as_mut()
            .ok_or(Failure::plain(ErrorKind::Closed))?
            .flush();
        match result {
            Ok(()) => {
                self.pending = false;
                self.progress = SendProgress::Flushed;
                Ok(SendStatus::Flushed)
            }
            Err(error) if self.would_block(&error) => {
                self.update_progress();
                Ok(SendStatus::Pending)
            }
            Err(error) => Err(self.classify(error)),
        }
    }
    pub fn receive(&mut self) -> Result<Event, Failure> {
        self.preflight()?;
        if self.pending {
            return Err(Failure {
                kind: ErrorKind::PendingWrite,
                send_progress: self.progress,
            });
        }
        if self.control_pending && !self.flush_control()? {
            return Ok(Event::WouldBlock);
        }
        if self.peer_closed {
            return Err(self.fail(ErrorKind::Closed));
        }
        let result = self
            .transport
            .ws
            .as_mut()
            .ok_or(Failure::plain(ErrorKind::Closed))?
            .read();
        let message = match result {
            Ok(message) => message,
            Err(error) if self.would_block(&error) => return Ok(Event::WouldBlock),
            Err(error) => return Err(self.classify(error)),
        };
        // Also cap payload delivered from codec read-ahead charged to an earlier operation.
        let charge = self
            .transport
            .ws
            .as_mut()
            .ok_or(Failure::plain(ErrorKind::Closed))?
            .get_mut()
            .deliver(message.len());
        if let Err(kind) = charge {
            return Err(self.fail(kind));
        }
        let event = match message {
            Message::Text(text) => Event::Text(text.to_string()),
            Message::Ping(_) => {
                self.flush_control()?;
                Event::Ping
            }
            Message::Pong(_) => Event::Pong,
            Message::Close(_) => {
                self.peer_closed = true;
                if self.flush_control()? {
                    self.transport.abort();
                }
                return Ok(Event::PeerClose);
            }
            Message::Binary(_) | Message::Frame(_) => return Err(self.fail(ErrorKind::Binary)),
        };
        let checked = self
            .transport
            .ws
            .as_mut()
            .ok_or(Failure::plain(ErrorKind::Closed))?
            .get_mut()
            .check();
        if let Err(kind) = checked {
            return Err(self.fail(kind));
        }
        Ok(event)
    }
    fn flush_control(&mut self) -> Result<bool, Failure> {
        self.preflight()?;
        let result = self
            .transport
            .ws
            .as_mut()
            .ok_or(Failure::plain(ErrorKind::Closed))?
            .flush();
        match result {
            Ok(()) => {
                self.control_pending = false;
                Ok(true)
            }
            Err(error) if self.would_block(&error) => {
                self.control_pending = true;
                Ok(false)
            }
            Err(error) => Err(self.classify(error)),
        }
    }
    /// Graceful close until the existing operation deadline, then owned shutdown.
    /// Pending application data is discarded rather than flushed after cancellation.
    pub fn close(&mut self) -> Result<CloseStatus, Failure> {
        self.preflight()?;
        if self.pending {
            self.transport.abort();
            return Ok(CloseStatus::ForcedShutdown);
        }
        let result = self
            .transport
            .ws
            .as_mut()
            .ok_or(Failure::plain(ErrorKind::Closed))?
            .close(None);
        if let Err(error) = result {
            return Err(self.classify(error));
        }
        loop {
            self.preflight()?;
            let result = self
                .transport
                .ws
                .as_mut()
                .ok_or(Failure::plain(ErrorKind::Closed))?
                .read();
            match result {
                Err(tungstenite::Error::ConnectionClosed) => {
                    self.transport.abort();
                    return Ok(CloseStatus::HandshakeComplete);
                }
                Ok(_) => (),
                Err(error) => return Err(self.classify(error)),
            }
        }
    }
}
impl Drop for Operation<'_> {
    fn drop(&mut self) {
        if self.pending || self.control_pending || self.peer_closed {
            self.transport.abort();
        }
    }
}
