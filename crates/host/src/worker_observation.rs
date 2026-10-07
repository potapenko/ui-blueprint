//! One real admitted observation. Drop cancels unfinished work; publication stays
//! a separate existing parent ACK boundary owned by the worker's IO composition.
use super::*;
use uiblueprint_plugin_api::{Error as ObservationError, Ticket};
pub(crate) struct ObservationRun<'a> {
    session: &'a mut ObservationSession,
    clock: Id,
    now: fn() -> u64,
    max_frame: usize,
    pub ticket: Ticket,
    done: bool,
}
fn error(value: ObservationError) -> HostError {
    match value {
        ObservationError::ResourceLimit => HostError::ResourceLimit,
        ObservationError::DeadlineExpired => HostError::DeadlineExpired,
        ObservationError::OutsideSession => HostError::PermissionDenied,
        _ => HostError::InvalidInput,
    }
}
impl ObservationRun<'_> {
    pub fn receive_channel(&mut self, bytes: &[u8], expected: Channel) -> Result<(), HostError> {
        guard::phase(guard::Phase::Validate);
        let doc =
            Document::from_json(bytes, self.max_frame).map_err(|_| HostError::InvalidInput)?;
        let Artifact::ChannelResponse(response) = doc.artifact else {
            return Err(HostError::InvalidInput);
        };
        if response.channel != expected {
            return Err(HostError::InvalidControl);
        }
        drop(response);
        self.session
            .receive(
                &self.ticket,
                bytes,
                &ClockReading {
                    domain: self.clock.clone(),
                    milliseconds: (self.now)(),
                },
            )
            .map_err(error)
    }
    pub fn finish(mut self) -> Result<(), HostError> {
        self.session
            .complete(
                &self.ticket,
                &ClockReading {
                    domain: self.clock.clone(),
                    milliseconds: (self.now)(),
                },
            )
            .map_err(error)?;
        self.done = true;
        Ok(())
    }
}
impl Drop for ObservationRun<'_> {
    fn drop(&mut self) {
        if !self.done {
            let _ = self.session.cancel(&self.ticket);
        }
    }
}
impl CanonicalSession<'_> {
    pub(crate) fn begin_observation(
        &mut self,
        input: &[u8],
        now: fn() -> u64,
        requested: u8,
    ) -> Result<(Box<Request>, super::ObservationRun<'_>), HostError> {
        guard::phase(guard::Phase::Decode);
        let doc = Document::from_json(input, self.limits.input_bytes)
            .map_err(|_| HostError::InvalidInput)?;
        let Artifact::Request(request) = doc.artifact else {
            return Err(HostError::InvalidInput);
        };
        let Operation::Observe { channels } = &request.operation else {
            return Err(HostError::PermissionDenied);
        };
        let mask = channels.iter().fold(0u8, |mask, c| {
            mask | match c {
                Channel::ExternalSemantics => 1,
                Channel::RenderedCapture => 2,
                Channel::OptInLayoutProbe => 4,
            }
        });
        if mask != requested {
            return Err(HostError::InvalidInput);
        }
        let clock = self.clock.clone();
        let ticket = self
            .observations
            .begin(
                input,
                &ClockReading {
                    domain: clock.clone(),
                    milliseconds: now(),
                },
            )
            .map_err(error)?;
        Ok((
            request,
            ObservationRun {
                session: &mut self.observations,
                clock,
                now,
                max_frame: self.limits.output_bytes,
                ticket,
                done: false,
            },
        ))
    }
    pub(crate) fn observe_native(
        &mut self,
        input: &[u8],
        now: fn() -> u64,
        requested: u8,
        exchange: &mut crate::worker_native::NativeExchange<'_>,
    ) -> Result<(), HostError> {
        let (request, mut operation) = self.begin_observation(input, now, requested)?;
        exchange.begin(
            operation.ticket.sequence,
            request.limits.deadline_ms,
            requested,
        )?;
        let mut failure = None;
        for (slot, channel) in [
            Channel::ExternalSemantics,
            Channel::RenderedCapture,
            Channel::OptInLayoutProbe,
        ]
        .into_iter()
        .enumerate()
        {
            if requested & (1 << slot) == 0 {
                continue;
            }
            let length = match exchange.collect(slot as u8) {
                Ok(n) => n,
                Err(
                    e @ (HostError::DeadlineExpired | HostError::CleanupPending | HostError::Io),
                ) => return Err(e),
                Err(e) => {
                    failure.get_or_insert(e);
                    continue;
                }
            };
            operation.receive_channel(exchange.bytes(length), channel)?;
            // Failed canonical responses are also complete, validated channel
            // records. Their Issue is preserved instead of empty success.
            exchange.publish(slot as u8, length)?;
        }
        if let Some(error) = failure {
            return Err(error);
        }
        operation.finish()
    }
}
