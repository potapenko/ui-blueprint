//! Reusable D02 test support, not a product transport or lifecycle implementation.
use std::{
    io::{self, BufRead, BufReader, Read},
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};
use uiblueprint_plugin_api::{ClockReading, Completion, Error, Limits, ObservationSession, Ticket};
use uiblueprint_schema::{SchemaVersion, model::*};

pub struct Harness {
    session: ObservationSession,
    ticket: Ticket,
    context: Context,
    epoch: Instant,
    clock_domain: Id,
    deadline_ticks: u64,
}

impl Harness {
    pub fn new(
        descriptor: SessionDescriptor,
        request: Request,
        limits: Limits,
    ) -> Result<Self, Error> {
        let epoch = Instant::now();
        let clock_domain = request.clock_domain.clone();
        let context = request.context.clone();
        let duration = request.limits.deadline_ms;
        let mut session = ObservationSession::attach(descriptor, clock_domain.clone(), limits)?;
        let bytes = serde_json::to_vec(&Document {
            schema_version: SchemaVersion::CURRENT,
            artifact: Artifact::Request(Box::new(request)),
        })
        .map_err(|_| Error::InvalidFrame)?;
        let now = ClockReading {
            domain: clock_domain.clone(),
            milliseconds: millis(epoch.elapsed())?,
        };
        let ticket = session.begin(&bytes, &now)?;
        let deadline_ticks = now
            .milliseconds
            .checked_add(duration)
            .ok_or(Error::ResourceLimit)?;
        Ok(Self {
            session,
            ticket,
            context,
            epoch,
            clock_domain,
            deadline_ticks,
        })
    }
    pub fn ticket(&self) -> &Ticket {
        &self.ticket
    }
    pub fn target(&self) -> &Identity {
        &self.context.target
    }
    pub fn clock_domain(&self) -> &Id {
        &self.clock_domain
    }
    pub fn elapsed_ms(&self) -> Result<u64, Error> {
        millis(self.epoch.elapsed())
    }
    fn now(&self) -> Result<ClockReading, Error> {
        Ok(ClockReading {
            domain: self.clock_domain.clone(),
            milliseconds: self.elapsed_ms()?,
        })
    }
    /// Remaining parent IO wait budget, derived from the same reading supplied to
    /// begin. The core API remains authoritative about expiration/late responses.
    pub fn remaining(&self) -> Result<Duration, Error> {
        Ok(Duration::from_millis(
            self.deadline_ticks.saturating_sub(self.elapsed_ms()?),
        ))
    }
    pub fn receive(&mut self, frame: &[u8]) -> Result<(), Error> {
        self.session.receive(&self.ticket, frame, &self.now()?)
    }
    pub fn complete(&mut self) -> Result<Completion, Error> {
        self.session.complete(&self.ticket, &self.now()?)
    }
    pub fn cancel(&mut self) -> Result<Completion, Error> {
        self.session.cancel(&self.ticket)
    }
    pub fn expire(&mut self) -> Result<Completion, Error> {
        self.session.expire(&self.ticket, &self.now()?)
    }
    pub fn detach(&mut self) -> Vec<Completion> {
        self.session.detach()
    }
    /// Canonical retained results for equality checks and D05 samples. No new DTO
    /// or file loading: these are the actual channel values returned by the core.
    pub fn retained_documents(&self, completion: Completion) -> Vec<Document> {
        completion
            .channels
            .into_iter()
            .map(|(channel, result)| Document {
                schema_version: SchemaVersion::CURRENT,
                artifact: Artifact::ChannelResponse(Box::new(ChannelResponse {
                    request_id: completion.ticket.request_id.clone(),
                    session_id: completion.ticket.session_id.clone(),
                    dispatch_sequence: completion.ticket.sequence,
                    target: self.context.target.clone(),
                    channel,
                    result,
                })),
            })
            .collect()
    }
}

fn millis(duration: Duration) -> Result<u64, Error> {
    u64::try_from(duration.as_millis()).map_err(|_| Error::InvalidClock)
}

#[derive(Debug, PartialEq, Eq)]
pub enum FrameEvent {
    Frame(Vec<u8>),
    Eof,
    Oversize,
    Io,
    FrameLimitReached,
}

/// A line is one canonical JSON document, including its newline in the byte cap.
/// EOF may terminate a final nonempty frame. Oversize terminates the reader rather
/// than draining an unbounded stream. No JSON commands are interpreted here.
pub fn read_frame<R: BufRead>(reader: &mut R, limit: usize) -> FrameEvent {
    let Some(bound) = limit.checked_add(1).and_then(|n| u64::try_from(n).ok()) else {
        return FrameEvent::Oversize;
    };
    let mut bytes = Vec::new();
    match reader.take(bound).read_until(b'\n', &mut bytes) {
        Ok(0) => FrameEvent::Eof,
        Ok(_) if bytes.len() > limit => FrameEvent::Oversize,
        Ok(_) => FrameEvent::Frame(bytes),
        Err(_) => FrameEvent::Io,
    }
}

/// One bounded reader for an owned stream. Native callers close/reap their owned
/// producer before joining its handle. A reader cannot interrupt an arbitrary OS
/// read by itself; the executable wrapper ends its own process on terminal exit.
pub fn pump_frames<R: Read + Send + 'static>(
    reader: R,
    limit: usize,
    max_frames: usize,
) -> (Receiver<FrameEvent>, thread::JoinHandle<()>) {
    let (tx, rx) = mpsc::sync_channel(1);
    let handle = thread::spawn(move || {
        let mut reader = BufReader::new(reader);
        for _ in 0..max_frames {
            let event = read_frame(&mut reader, limit);
            let terminal = !matches!(event, FrameEvent::Frame(_));
            if tx.send(event).is_err() || terminal {
                return;
            }
        }
        let _ = tx.send(FrameEvent::FrameLimitReached);
    });
    (rx, handle)
}

pub fn read_document(path: &std::path::Path, limit: usize) -> io::Result<Document> {
    let bound = limit
        .checked_add(1)
        .and_then(|n| u64::try_from(n).ok())
        .ok_or_else(|| io::Error::other("invalid limit"))?;
    let mut bytes = Vec::new();
    std::fs::File::open(path)?
        .take(bound)
        .read_to_end(&mut bytes)?;
    Document::from_json(&bytes, limit).map_err(|_| io::Error::other("invalid canonical document"))
}
