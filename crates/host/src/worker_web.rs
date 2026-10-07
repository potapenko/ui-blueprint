//! Executable-private Web producer. This module is called only under the real
//! worker allocator/watchdog; the parent remains an opaque bounded byte owner.
use crate::{
    quota_allocator as guard,
    worker_io::WorkerIo,
    worker_main::{self, FixedOutput},
    worker_ops::CanonicalSession,
};
use std::{sync::OnceLock, time::Instant};
use uiblueprint_host::{
    Control, HostError, HostLimits, OperationClass,
    authority::TargetLease,
    diagnostic::{DiagnosticCause as Cause, DiagnosticRecord, DiagnosticStage as Stage},
    web_config::{WebSelection, WebSetup},
    worker_tape::Tape,
};
use uiblueprint_schema::model::{Artifact, Channel, Document, Id};
use uiblueprint_web::{cdp, collector, transport};

// Worker diagnostic output is fixed private control/status, never raw library logs.
// A foreign logger is refused, not replaced; max_level remains untouched.
struct WorkerLog;
impl log::Log for WorkerLog {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        false
    }
    fn log(&self, _: &log::Record<'_>) {}
    fn flush(&self) {}
}
static LOG: WorkerLog = WorkerLog;
static LOG_BOUNDARY: OnceLock<Result<(), HostError>> = OnceLock::new();
fn logging() -> Result<(), HostError> {
    *LOG_BOUNDARY
        .get_or_init(|| transport::install_log_boundary(&LOG).map_err(|_| HostError::InvalidState))
}

pub(super) struct WebSession {
    collector: collector::Collector,
    limits: HostLimits,
}
impl WebSession {
    /// Existing read-only Prepare Tape. Input capability claims are unresolved;
    /// only the actual provider's independently acquired facts create ActionCase.
    pub(super) fn prepare_action(
        &mut self,
        session: &mut CanonicalSession<'_>,
        io: &mut WorkerIo,
        publication: &mut [u8],
        control: Control,
        input: &[u8],
    ) -> Result<u64, HostError> {
        let result = (|| {
            if control.class != OperationClass::Prepare || control.flags & 8 == 0 {
                return Err(HostError::InvalidControl);
            }
            let (snapshot, request, clock) = match session.prepare_input(input) {
                Ok(input) => input,
                Err(error) => {
                    return crate::worker_action::publish_refusal(
                        io,
                        publication,
                        control,
                        Id("prepare-input".into()),
                        error,
                        publication.len(),
                    );
                }
            };
            let limit = request.limits.max_output_bytes as usize;
            let deadline = worker_main::admit_action(io, control, request.limits.deadline_ms)?;
            let mut timing =
                crate::worker_effect::WorkerActionControl::new(clock, worker_main::clock_origin());
            use uiblueprint_plugin_api::actions::ActionControl;
            let now = timing.now();
            let remaining = u64::try_from(
                deadline
                    .saturating_duration_since(Instant::now())
                    .as_millis(),
            )
            .map_err(|_| HostError::Overflow)?;
            if remaining == 0 {
                return Err(HostError::DeadlineExpired);
            }
            let provider =
                collector::CheckboxProvider::new(&mut self.collector, request.limits.clone());
            match provider.prepare_exact(&snapshot, &request, &now, remaining) {
                Ok(case) => {
                    crate::worker_action::publish_prepared(io, publication, control, case, limit)
                }
                Err(issue) => crate::worker_action::publish_refusal(
                    io,
                    publication,
                    control,
                    issue.scope_id,
                    action_issue_error(issue.code),
                    limit,
                ),
            }
        })();
        if self.collector.pending_invalidation() {
            session.invalidate_retained_session()?;
            self.collector.acknowledge_invalidation();
        }
        result
    }
    pub(super) fn act(
        &mut self,
        session: &mut CanonicalSession<'_>,
        io: &mut WorkerIo,
        publication: &mut [u8],
        control: Control,
        input: &[u8],
    ) -> Result<u64, HostError> {
        let result = (|| {
            if control.class != OperationClass::Mutation || control.flags & 8 == 0 {
                return Err(HostError::InvalidControl);
            }
            let (case, limits, target, clock) = match session.action_input(input) {
                Ok(input) => input,
                Err(error) => {
                    return crate::worker_action::publish_refusal(
                        io,
                        publication,
                        control,
                        Id("action-input".into()),
                        error,
                        publication.len(),
                    );
                }
            };
            let deadline = worker_main::admit_action(io, control, limits.deadline_ms)?;
            let mut provider =
                collector::CheckboxProvider::new(&mut self.collector, limits.clone());
            crate::worker_action::ActionOperation {
                io,
                publication,
                control,
                target,
                clock,
                origin: worker_main::clock_origin(),
                deadline,
            }
            .execute(case, limits, &mut provider)
        })();
        if self.collector.pending_invalidation() {
            session.invalidate_retained_session()?;
            self.collector.acknowledge_invalidation();
        }
        result
    }
    /// setup is immutable TRUSTED attachment configuration, never an observation
    /// body/UI-provided endpoint. Core owns transport/selection of that authority.
    pub(super) fn attach(
        raw_descriptor: &[u8],
        setup: WebSetup,
        target: TargetLease,
        clock: Id,
        origin: Instant,
        limits: HostLimits,
        deadline: Instant,
    ) -> Result<Self, HostError> {
        guard::phase(guard::Phase::Decode);
        let doc = Document::from_json(raw_descriptor, limits.input_bytes)
            .map_err(|_| HostError::InvalidInput)?;
        let Artifact::Session(descriptor) = doc.artifact else {
            return Err(HostError::InvalidInput);
        };
        if !target.matches(&descriptor.target)
            || !target.permits(OperationClass::Observe)
            || !descriptor.surfaces.contains(&setup.surface)
        {
            return Err(HostError::PermissionDenied);
        }
        // Refuse an incompatible acquisition configuration even before WS handshake.
        if setup.transport.frame_bytes > setup.collector.max_reply_bytes
            || setup.transport.message_bytes > setup.collector.max_reply_bytes
        {
            return Err(HostError::InvalidLimits);
        }
        logging()?;
        guard::phase(guard::Phase::Admission);
        let transport = transport::Transport::connect(
            &setup.endpoint,
            setup.transport.limits(),
            transport::OperationLimits {
                deadline,
                max_read_bytes: setup.collector.io_read_bytes,
                max_write_bytes: setup.collector.io_write_bytes,
                max_work: setup.collector.io_work,
            },
        )
        .map_err(transport_error)?;
        let client = cdp::Client::new(
            transport,
            cdp::Binding {
                target: descriptor.target.clone(),
                cdp_session_id: setup.cdp_session_id.clone(),
            },
            setup.cdp.limits(),
        )
        .map_err(cdp_error)?;
        let collector = collector::Collector::attach(
            client,
            collector::Binding {
                session_id: descriptor.session_id,
                target: descriptor.target,
                surface: setup.surface,
                cdp_session_id: setup.cdp_session_id,
                clock: collector::Clock {
                    domain: clock,
                    origin,
                },
                allowed_scopes: descriptor.allowed_scopes,
                plugin: descriptor.plugin,
            },
            setup.collector.limits(),
            deadline,
        )
        .map_err(collector_error)?;
        Ok(Self { collector, limits })
    }
    /// Tape contains exactly an existing canonical Request and WebSelection CONFIG,
    /// never preassembled channel responses. Ticket and ACK are the real Core owners.
    pub(super) fn observe(
        &mut self,
        session: &mut CanonicalSession<'_>,
        io: &mut WorkerIo,
        publication: &mut [u8],
        control: Control,
        input: &[u8],
        now: fn() -> u64,
    ) -> Result<(), HostError> {
        // Keep the ObservationRun borrow inside this operation. Its success or
        // failure cleanup ends before the retained cache is invalidated below.
        let result = (|| {
            if control.class != OperationClass::Observe {
                return Err(host_diagnostic(
                    io,
                    Stage::Decode,
                    HostError::PermissionDenied,
                ));
            }
            if input.len() > self.limits.input_bytes {
                return Err(host_diagnostic(
                    io,
                    Stage::Capacity,
                    HostError::ResourceLimit,
                ));
            }
            guard::phase(guard::Phase::Decode);
            let tape = Tape::decode(input).map_err(|e| host_diagnostic(io, Stage::Decode, e))?;
            if tape.count() != 2 {
                return Err(host_diagnostic(io, Stage::Decode, HostError::InvalidInput));
            }
            let selection = WebSelection::decode(
                tape.get(1)
                    .map_err(|e| host_diagnostic(io, Stage::Decode, e))?,
                self.limits.input_bytes,
            )
            .map_err(|e| host_diagnostic(io, Stage::Decode, e))?;
            let channels = control.flags & 7;
            let (request, mut run) = session
                .begin_observation(
                    tape.get(0)
                        .map_err(|e| host_diagnostic(io, Stage::Decode, e))?,
                    now,
                    channels,
                )
                .map_err(|e| host_diagnostic(io, Stage::Begin, e))?;
            let sequence = run.ticket.sequence;
            // No collection before both real begin and the authoritative parent permit.
            let deadline = worker_main::admit_observation(
                io,
                control,
                sequence,
                request.limits.deadline_ms,
                channels,
            )
            .map_err(|e| host_diagnostic(io, Stage::Permit, e))?;
            let frame_cap = (control.auxiliary as u32) as usize;
            let total_cap = usize::try_from(control.auxiliary >> 32)
                .map_err(|_| host_diagnostic(io, Stage::Capacity, HostError::Overflow))?;
            if frame_cap == 0
                || frame_cap > self.limits.output_bytes
                || frame_cap > publication.len()
                || total_cap == 0
                || total_cap > self.limits.request_output_bytes
            {
                return Err(host_diagnostic(
                    io,
                    Stage::Capacity,
                    HostError::ResourceLimit,
                ));
            }
            let mut total = 0usize;
            let mut callback_error = None;
            let mut callback = |document: Document| {
                let mut stage = Stage::Encode;
                let result = (|| -> Result<(), HostError> {
                    let Artifact::ChannelResponse(response) = &document.artifact else {
                        return Err(HostError::InvalidInput);
                    };
                    let channel = response.channel;
                    let slot = match channel {
                        Channel::ExternalSemantics => 0,
                        Channel::RenderedCapture => 1,
                        Channel::OptInLayoutProbe => 2,
                    };
                    if channels & (1 << slot) == 0 {
                        return Err(HostError::InvalidControl);
                    }
                    let mut encoded = FixedOutput {
                        bytes: &mut publication[..frame_cap],
                        used: 0,
                    };
                    stage = Stage::Encode;
                    {
                        let _reserve = guard::PublicationGuard::enter(guard::Phase::Validate);
                        let result = serde_json::to_writer(&mut encoded, &document);
                        result.map_err(|_| HostError::ResourceLimit)?;
                    }
                    // Avoid retaining this original graph alongside receive's canonical decode.
                    drop(document);
                    let new_total = total.checked_add(encoded.used).ok_or(HostError::Overflow)?;
                    if new_total > total_cap {
                        return Err(HostError::ResourceLimit);
                    }
                    stage = Stage::Receive;
                    let incomplete =
                        run.receive_channel(&encoded.bytes[..encoded.used], channel)?;
                    stage = Stage::Publish;
                    {
                        let _reserve = guard::PublicationGuard::enter(guard::Phase::Validate);
                        worker_main::publish(
                            io,
                            control,
                            slot,
                            &encoded.bytes[..encoded.used],
                            u8::from(incomplete),
                        )?;
                    }
                    total = new_total;
                    Ok(())
                })();
                match result {
                    Ok(()) => collector::Publication::Acknowledged,
                    Err(error) => {
                        callback_error = Some((stage, error));
                        collector::Publication::Stop
                    }
                }
            };
            guard::phase(guard::Phase::Admission);
            let collected = match selection {
                WebSelection::Rooted {
                    root,
                    max_visited_nodes,
                } => {
                    let scope = collector::RootedScope {
                        scope_id: request.context.scope_id.clone(),
                        root: collector::RootSeed {
                            session_id: root.session_id,
                            target: root.target,
                            surface: root.surface,
                            document_backend_id: root.document_backend_id,
                            backend_node_id: root.backend_node_id,
                            sensitivity: root.sensitivity,
                        },
                        max_visited_nodes,
                    };
                    self.collector
                        .observe_rooted(&request, &scope, sequence, deadline, &mut callback)
                        .map(|_| ())
                }
                WebSelection::Initial {
                    ids,
                    max_visited_nodes,
                } => {
                    let scope = collector::InitialScope {
                        scope_id: request.context.scope_id.clone(),
                        ids: ids
                            .into_iter()
                            .map(|v| collector::DomId {
                                id: v.id,
                                sensitivity: v.sensitivity,
                            })
                            .collect(),
                        max_visited_nodes,
                    };
                    // Ref metadata is already represented by the canonical Snapshot/Observation.
                    // Do not add a convenience response or an uncharged retained-ref cache.
                    self.collector
                        .observe_initial(&request, &scope, sequence, deadline, &mut callback)
                        .map(|_| ())
                }
                WebSelection::References { nodes } => {
                    let scope = collector::Scope {
                        scope_id: request.context.scope_id.clone(),
                        nodes: nodes
                            .into_iter()
                            .map(|v| collector::NodeRef {
                                reference: v.reference,
                                sensitivity: v.sensitivity,
                            })
                            .collect(),
                    };
                    self.collector
                        .observe(&request, &scope, sequence, deadline, &mut callback)
                        .map(|_| ())
                }
            };
            if let Some((stage, error)) = callback_error {
                return Err(host_diagnostic(io, stage, error));
            }
            if let Err(error) = collected {
                io.set_diagnostic(collector_diagnostic(error));
                return Err(collector_error(error));
            }
            run.finish()
                .map_err(|e| host_diagnostic(io, Stage::Finish, e))
        })();
        if self.collector.pending_invalidation() {
            session
                .invalidate_retained_session()
                .map_err(|error| host_diagnostic(io, Stage::Finish, error))?;
            self.collector.acknowledge_invalidation();
        }
        result
    }
}
fn action_issue_error(code: uiblueprint_schema::model::ErrorCode) -> HostError {
    use uiblueprint_schema::model::ErrorCode::*;
    match code {
        PermissionRequired => HostError::PermissionDenied,
        StaleTarget | TargetUnresolved | AmbiguousTarget | ResyncRequired => {
            HostError::ResyncRequired
        }
        Timeout => HostError::DeadlineExpired,
        _ => HostError::InvalidInput,
    }
}
fn collector_error(error: collector::Failure) -> HostError {
    use collector::{ErrorKind as E, SelectionStatus as S};
    match error.kind {
        E::Limit => HostError::ResourceLimit,
        E::Timeout => HostError::DeadlineExpired,
        E::StaleTarget | E::ResyncRequired => HostError::ResyncRequired,
        E::Selection {
            status: S::Incomplete,
            ..
        } => HostError::ResourceLimit,
        E::Selection {
            status: S::TimedOut,
            ..
        } => HostError::DeadlineExpired,
        E::Selection {
            status: S::Missing | S::Ambiguous,
            ..
        } => HostError::ResyncRequired,
        E::Cdp(kind) => cdp_error(cdp::Failure {
            kind,
            wire_id: None,
            send_progress: error.send_progress,
        }),
        E::CleanupUnconfirmed => HostError::CleanupPending,
        E::Protocol(_) => HostError::WorkerFailed,
        _ => HostError::InvalidInput,
    }
}
fn cdp_error(error: cdp::Failure) -> HostError {
    match error.kind {
        cdp::ErrorKind::Budget | cdp::ErrorKind::CounterExhausted => HostError::ResourceLimit,
        cdp::ErrorKind::Transport(kind) => transport_error(transport::Failure {
            kind,
            send_progress: error.send_progress,
        }),
        cdp::ErrorKind::Cancelled | cdp::ErrorKind::Detached => HostError::StaleOperation,
        cdp::ErrorKind::WrongSession | cdp::ErrorKind::UnexpectedReply => HostError::ResyncRequired,
        _ => HostError::InvalidInput,
    }
}
fn transport_error(error: transport::Failure) -> HostError {
    match error.kind {
        transport::ErrorKind::Timeout => HostError::DeadlineExpired,
        transport::ErrorKind::ByteLimit
        | transport::ErrorKind::WorkLimit
        | transport::ErrorKind::Capacity => HostError::ResourceLimit,
        transport::ErrorKind::Cancelled | transport::ErrorKind::Closed => HostError::StaleOperation,
        transport::ErrorKind::EndpointRejected | transport::ErrorKind::LoggingBoundary => {
            HostError::PermissionDenied
        }
        _ => HostError::Io,
    }
}

fn host_diagnostic(io: &mut WorkerIo, stage: Stage, error: HostError) -> HostError {
    io.set_diagnostic(DiagnosticRecord::host(stage, error));
    error
}
fn collector_diagnostic(error: collector::Failure) -> DiagnosticRecord {
    use collector::{ErrorKind as E, SelectionStatus as S};
    let mut record = DiagnosticRecord {
        stage: Stage::Collect,
        cause: Cause::CollectorInvalidInput,
        remote_cleanup: match error.remote_cleanup {
            collector::RemoteCleanup::NotRequired => 0,
            collector::RemoteCleanup::Released => 1,
            collector::RemoteCleanup::Unconfirmed => 2,
        },
        send_progress: match error.send_progress {
            transport::SendProgress::NotQueued => 0,
            transport::SendProgress::Queued => 1,
            transport::SendProgress::PossiblyWritten => 2,
            transport::SendProgress::Flushed => 3,
        },
        code: 0,
        count: 0,
    };
    record.cause = match error.kind {
        E::InvalidInput => Cause::CollectorInvalidInput,
        E::Limit => Cause::CollectorLimit,
        E::StaleTarget => Cause::CollectorStaleTarget,
        E::ResyncRequired => Cause::CollectorResyncRequired,
        E::Malformed => {
            record.code = error.malformed_site.map_or(0, |site| i32::from(site as u8));
            Cause::CollectorMalformed
        }
        E::Protocol(code) => {
            record.code = code;
            Cause::CollectorProtocol
        }
        E::Timeout => Cause::CollectorTimeout,
        E::PublicationStopped => Cause::CollectorPublicationStopped,
        E::CleanupUnconfirmed => Cause::CollectorCleanupUnconfirmed,
        E::Selection {
            status,
            visited_nodes,
        } => {
            record.count = visited_nodes;
            match status {
                S::Missing => Cause::SelectionMissing,
                S::Ambiguous => Cause::SelectionAmbiguous,
                S::Incomplete => Cause::SelectionIncomplete,
                S::Unsupported => Cause::SelectionUnsupported,
                S::TimedOut => Cause::SelectionTimedOut,
            }
        }
        E::Cdp(kind) => match kind {
            cdp::ErrorKind::InvalidLimits => Cause::CdpInvalidLimits,
            cdp::ErrorKind::InvalidBinding => Cause::CdpInvalidBinding,
            cdp::ErrorKind::Encoding => Cause::CdpEncoding,
            cdp::ErrorKind::Envelope => Cause::CdpEnvelope,
            cdp::ErrorKind::Budget => Cause::CdpBudget,
            cdp::ErrorKind::Detached => Cause::CdpDetached,
            cdp::ErrorKind::Busy => Cause::CdpBusy,
            cdp::ErrorKind::CounterExhausted => Cause::CdpCounterExhausted,
            cdp::ErrorKind::WrongSession => Cause::CdpWrongSession,
            cdp::ErrorKind::UnexpectedReply => Cause::CdpUnexpectedReply,
            cdp::ErrorKind::UncorrelatedError(code) => {
                record.code = code;
                Cause::CdpUncorrelatedError
            }
            cdp::ErrorKind::Cancelled => Cause::CdpCancelled,
            cdp::ErrorKind::Transport(kind) => match kind {
                transport::ErrorKind::InvalidLimits => Cause::TransportInvalidLimits,
                transport::ErrorKind::EndpointRejected => Cause::TransportEndpointRejected,
                transport::ErrorKind::LoggingBoundary => Cause::TransportLoggingBoundary,
                transport::ErrorKind::Connect => Cause::TransportConnect,
                transport::ErrorKind::Handshake => Cause::TransportHandshake,
                transport::ErrorKind::Extensions => Cause::TransportExtensions,
                transport::ErrorKind::Timeout => Cause::TransportTimeout,
                transport::ErrorKind::Cancelled => Cause::TransportCancelled,
                transport::ErrorKind::ByteLimit => Cause::TransportByteLimit,
                transport::ErrorKind::WorkLimit => Cause::TransportWorkLimit,
                transport::ErrorKind::Io => Cause::TransportIo,
                transport::ErrorKind::Protocol => Cause::TransportProtocol,
                transport::ErrorKind::Capacity => Cause::TransportCapacity,
                transport::ErrorKind::Binary => Cause::TransportBinary,
                transport::ErrorKind::Closed => Cause::TransportClosed,
                transport::ErrorKind::PendingWrite => Cause::TransportPendingWrite,
                transport::ErrorKind::NoPendingWrite => Cause::TransportNoPendingWrite,
            },
        },
    };
    record
}
