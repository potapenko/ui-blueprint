//! Explicit single-step caller. Canonical result and source bodies stay opaque.
use crate::{Failure, arguments::ActionArguments};
use std::io::Write;

#[cfg(not(all(target_os = "macos", feature = "web")))]
pub(crate) fn execute(args: ActionArguments, _: &mut impl Write) -> Result<u8, Failure> {
    let _ = (
        args.command,
        args.connection,
        args.source,
        args.request,
        args.worker,
        args.max_input,
        args.max_output,
        args.json,
    );
    Err(Failure::unsupported("unsupported_command"))
}

#[cfg(all(target_os = "macos", feature = "web"))]
pub(crate) use supported::execute;

#[cfg(all(target_os = "macos", feature = "web"))]
mod supported {
    use super::*;
    use crate::{arguments::ActionCommand, connection::*};
    use std::time::Duration;
    use uiblueprint_host::{
        HostError, OperationClass,
        authority::TargetLease,
        domain::HostDomain,
        host_types::{EffectReceipt, HostCompletion, HostEvent, OutputRequest, Terminal},
        process::DarwinPlatform,
        publication::ActionPublicationStatus as Status,
        supervisor::RuntimeHost,
        worker_tape,
    };
    use uiblueprint_schema::{SchemaVersion, model::*};

    pub(crate) fn execute(args: ActionArguments, output: &mut impl Write) -> Result<u8, Failure> {
        let mut remaining = args.max_input;
        let connection = load(&args.connection, &mut remaining)?;
        let Provider::Web { setup, .. } = &connection.provider else {
            return Err(Failure::unsupported("unsupported_command"));
        };
        let request_bytes = crate::input::read(&args.request, &mut remaining)?;
        let mut request = Document::from_json(&request_bytes, args.max_input)
            .map_err(|_| Failure::invalid("invalid_request"))?;
        drop(request_bytes);
        let Artifact::Request(r) = &request.artifact else {
            return Err(Failure::invalid("invalid_request"));
        };
        let action = match (&r.operation, args.command) {
            (Operation::Prepare { action }, ActionCommand::Prepare)
            | (Operation::Act { action }, ActionCommand::Execute) => action,
            _ => return Err(Failure::invalid("invalid_request")),
        };
        if !matches!(action.intent, Intent::SetChecked { .. })
            || action.modality != InputModality::Setter
        {
            return Err(Failure::unsupported("unsupported_command"));
        }
        let session = &connection.session;
        if session.target != connection.target
            || r.context.target != connection.target
            || r.context.session_id != session.session_id
            || r.context.plugin != session.plugin
            || !session.allowed_scopes.contains(&r.context.scope_id)
            || r.context
                .surfaces
                .iter()
                .any(|s| !session.surfaces.contains(s))
            || !session.surfaces.contains(&setup.surface)
        {
            return Err(Failure::invalid("connection_request_mismatch"));
        }
        // Read bounded raw bytes only. Envelope extraction, graph validation and
        // exact action/source agreement happen in the admitted guarded worker.
        let source = crate::input::read(&args.source, &mut remaining)?;
        let deadline_ms = r.limits.deadline_ms;
        let request_output = r.limits.max_output_bytes as usize;
        let limits = connection.host_limits.limits()?;
        let attach_end = until(connection.attach_deadline_ms)?;
        until(deadline_ms)?;
        let total = args
            .max_output
            .checked_sub(1)
            .filter(|n| *n > 0)
            .ok_or(Failure::invalid("output_limit"))?
            .min(limits.request_output_bytes)
            .min(request_output);
        let requested = OutputRequest {
            channels: 1,
            frame_bytes: limits.output_bytes.min(total),
            total_bytes: total,
            input_format: 1,
            retained_partition: 0,
        };
        let setup = crate::output::trusted_input(setup, limits.input_bytes)?;
        let descriptor = Document {
            schema_version: SchemaVersion::CURRENT,
            artifact: Artifact::Session(Box::new(connection.session)),
        };
        let descriptor = encoded(&descriptor, limits.input_bytes)?;
        Document::from_json(&descriptor, limits.input_bytes)
            .map_err(|_| Failure::invalid("invalid_connection"))?;
        let domain = HostDomain::new::<DarwinPlatform>(limits).map_err(host_error)?;
        let mut host = RuntimeHost::new(&domain, executable(&args.worker)?, DarwinPlatform)
            .map_err(host_error)?;
        let result = (|| -> Result<HostCompletion<'_>, Failure> {
            let target =
                TargetLease::authorized(&connection.target, args.command == ActionCommand::Execute)
                    .map_err(host_error)?;
            let parts = [descriptor.as_slice(), setup.as_slice()];
            let mut input = host
                .reserve_attach_input(target, tape_len(&parts)?)
                .map_err(host_error)?;
            worker_tape::encode(&parts, input.bytes_mut()).map_err(host_error)?;
            let attached = host.attach_web(input, attach_end).map_err(host_error)?;
            let event_end = attach_end
                .checked_add(Duration::from_millis(limits.cleanup_ms))
                .ok_or(Failure::invalid("invalid_deadline"))?;
            let clock = match next(&mut host, event_end)? {
                HostEvent::Attached { session, clock } if session == attached => clock,
                _ => return Err(Failure::io()),
            };
            let Artifact::Request(r) = &mut request.artifact else {
                unreachable!("Request validated above")
            };
            r.clock_domain = Id(clock.as_str().into());
            let request = encoded(&request, limits.input_bytes)?;
            let parts = [source.as_slice(), request.as_slice()];
            let mut input = host
                .reserve_input(attached, tape_len(&parts)?)
                .map_err(host_error)?;
            worker_tape::encode(&parts, input.bytes_mut()).map_err(host_error)?;
            let end = until(deadline_ms)?;
            host.submit(
                attached,
                match args.command {
                    ActionCommand::Prepare => OperationClass::Prepare,
                    ActionCommand::Execute => OperationClass::Mutation,
                },
                input,
                requested,
                end,
            )
            .map_err(host_error)?;
            let event_end = end
                .checked_add(Duration::from_millis(limits.cleanup_ms))
                .ok_or(Failure::invalid("invalid_deadline"))?;
            match next(&mut host, event_end)? {
                HostEvent::Complete(c) => Ok(c),
                _ => Err(Failure::io()),
            }
        })();
        let mut terminal_during_shutdown = None;
        let cleanup = shutdown(&mut host, limits.cleanup_ms, &mut terminal_during_shutdown);
        let (completion, failure) = match result {
            Ok(c) => (Some(c), None),
            Err(error) => (terminal_during_shutdown, Some(error)),
        };
        if let Some(c) = &completion {
            if args.json {
                crate::output::observation_lines(c, args.max_output, output)?;
            } else {
                output
                    .write_all(&crate::output::action_compact(
                        args.command,
                        c,
                        cleanup.is_ok(),
                        args.max_output,
                    )?)
                    .map_err(|_| Failure::io())?;
            }
        }
        cleanup?;
        if let Some(error) = failure {
            return Err(error);
        }
        let c = completion.as_ref().ok_or(Failure::io())?;
        completion_exit(
            args.command,
            c.terminal,
            c.effect,
            c.action_status(),
            c.missing(),
        )
    }

    fn completion_exit(
        command: ActionCommand,
        terminal: Terminal,
        effect: EffectReceipt,
        status: Option<Status>,
        missing: u8,
    ) -> Result<u8, Failure> {
        match terminal {
            Terminal::Failed(
                HostError::Io
                | HostError::InvalidControl
                | HostError::InvalidState
                | HostError::CleanupPending
                | HostError::AllocationFailure
                | HostError::SystemAllocationFailure,
            ) => return Err(Failure::io()),
            Terminal::Failed(
                HostError::InvalidInput
                | HostError::InvalidLimits
                | HostError::ResourceLimit
                | HostError::Overflow,
            ) if effect == EffectReceipt::NotDispatched => {
                return Err(Failure::invalid("invalid_input"));
            }
            Terminal::Failed(HostError::WorkerFailed) if effect == EffectReceipt::NotDispatched => {
                return Err(Failure::io());
            }
            Terminal::Cancelled | Terminal::TimedOut => return Ok(4),
            _ => (),
        }
        if missing != 0 {
            return Ok(4);
        }
        match (command, terminal, effect, status) {
            (
                ActionCommand::Prepare,
                Terminal::Completed,
                EffectReceipt::NotDispatched,
                Some(Status::Prepared),
            ) => Ok(0),
            (
                ActionCommand::Execute,
                Terminal::Completed,
                EffectReceipt::Confirmed { .. },
                Some(Status::VerifiedSuccess),
            ) => Ok(0),
            (ActionCommand::Execute, _, _, Some(Status::VerifiedMismatch)) => Ok(3),
            _ => Ok(4),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn semantic_status_never_substitutes_for_ack_terminal_delivery_or_cleanup() {
            let confirmed = EffectReceipt::Confirmed { nonce: 7 };
            let possible = EffectReceipt::Possible { nonce: 7 };
            for (command, terminal, effect, status, missing, expected) in [
                (
                    ActionCommand::Prepare,
                    Terminal::Completed,
                    EffectReceipt::NotDispatched,
                    Some(Status::Prepared),
                    0,
                    0,
                ),
                (
                    ActionCommand::Prepare,
                    Terminal::Completed,
                    EffectReceipt::NotDispatched,
                    None,
                    0,
                    4,
                ),
                (
                    ActionCommand::Execute,
                    Terminal::Completed,
                    confirmed,
                    Some(Status::VerifiedSuccess),
                    0,
                    0,
                ),
                (
                    ActionCommand::Execute,
                    Terminal::Completed,
                    confirmed,
                    Some(Status::VerifiedMismatch),
                    0,
                    3,
                ),
                (
                    ActionCommand::Execute,
                    Terminal::Completed,
                    confirmed,
                    Some(Status::Uncertain),
                    0,
                    4,
                ),
                (
                    ActionCommand::Execute,
                    Terminal::Completed,
                    confirmed,
                    None,
                    0,
                    4,
                ),
                (
                    ActionCommand::Execute,
                    Terminal::Completed,
                    possible,
                    Some(Status::VerifiedSuccess),
                    0,
                    4,
                ),
                (
                    ActionCommand::Execute,
                    Terminal::Completed,
                    confirmed,
                    Some(Status::VerifiedSuccess),
                    1,
                    4,
                ),
                (
                    ActionCommand::Execute,
                    Terminal::Failed(HostError::WorkerFailed),
                    possible,
                    Some(Status::VerifiedSuccess),
                    0,
                    4,
                ),
                (
                    ActionCommand::Execute,
                    Terminal::Cancelled,
                    confirmed,
                    Some(Status::VerifiedSuccess),
                    0,
                    4,
                ),
                (
                    ActionCommand::Execute,
                    Terminal::TimedOut,
                    confirmed,
                    Some(Status::VerifiedSuccess),
                    0,
                    4,
                ),
                (
                    ActionCommand::Execute,
                    Terminal::Failed(HostError::InvalidInput),
                    possible,
                    None,
                    0,
                    4,
                ),
                (
                    ActionCommand::Execute,
                    Terminal::Failed(HostError::CleanupPending),
                    confirmed,
                    Some(Status::VerifiedSuccess),
                    0,
                    1,
                ),
                (
                    ActionCommand::Execute,
                    Terminal::Failed(HostError::InvalidControl),
                    possible,
                    None,
                    1,
                    1,
                ),
                (
                    ActionCommand::Execute,
                    Terminal::Failed(HostError::InvalidInput),
                    EffectReceipt::NotDispatched,
                    None,
                    1,
                    2,
                ),
            ] {
                let actual = completion_exit(command, terminal, effect, status, missing);
                assert_eq!(
                    actual.unwrap_or_else(|e| e.exit),
                    expected,
                    "{terminal:?} {effect:?} {status:?}"
                );
            }
        }
    }
}
