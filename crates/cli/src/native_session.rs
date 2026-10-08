//! One process, one attached fixture session, explicit bounded command records.
use crate::{Failure, connection::*};
use serde::Deserialize;
use std::{
    ffi::OsString,
    io::{Read, Write},
    os::fd::AsFd,
    path::PathBuf,
    time::{Duration, Instant},
};
use uiblueprint_host::{
    OperationClass,
    authority::TargetLease,
    domain::HostDomain,
    host_types::{EffectReceipt, HostEvent, OutputRequest, Terminal},
    native_binding::NativeHelperBinding,
    process::{DarwinPlatform, input_ready},
    publication::ActionPublicationStatus,
    supervisor::RuntimeHost,
    worker_tape,
};
use uiblueprint_schema::{SchemaVersion, model::*};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Command {
    request: PathBuf,
    source: Option<PathBuf>,
    expectation: Option<PathBuf>,
}
struct Arguments {
    connection: PathBuf,
    worker: PathBuf,
    duration: u64,
    input: usize,
    output: usize,
}
fn arguments(mut args: impl Iterator<Item = OsString>) -> Result<Arguments, Failure> {
    let (mut connection, mut worker, mut duration, mut input, mut output) =
        (None, None, None, None, None);
    while let Some(flag) = args.next() {
        let value = args.next().ok_or(Failure::invalid("invalid_arguments"))?;
        let number = || {
            value
                .to_str()
                .and_then(|s| s.parse::<u64>().ok())
                .filter(|n| *n > 0)
                .ok_or(Failure::invalid("invalid_arguments"))
        };
        match flag.to_str() {
            Some("--connection") if connection.is_none() => connection = Some(PathBuf::from(value)),
            Some("--worker") if worker.is_none() => worker = Some(PathBuf::from(value)),
            Some("--duration-ms") if duration.is_none() => duration = Some(number()?),
            Some("--max-input-bytes") if input.is_none() => {
                input =
                    Some(usize::try_from(number()?).map_err(|_| Failure::invalid("input_limit"))?)
            }
            Some("--max-output-bytes") if output.is_none() => {
                output =
                    Some(usize::try_from(number()?).map_err(|_| Failure::invalid("output_limit"))?)
            }
            _ => return Err(Failure::invalid("invalid_arguments")),
        }
    }
    let invalid = Failure::invalid("invalid_arguments");
    let result = Arguments {
        connection: connection.ok_or(invalid)?,
        worker: worker.ok_or(invalid)?,
        duration: duration.ok_or(invalid)?,
        input: input.ok_or(invalid)?,
        output: output.ok_or(invalid)?,
    };
    if result.duration > 300000 || result.input > 2 * 1024 * 1024 || result.output > 2 * 1024 * 1024
    {
        return Err(invalid);
    }
    Ok(result)
}
pub(crate) fn execute(
    args: impl Iterator<Item = OsString>,
    output: &mut impl Write,
) -> Result<u8, Failure> {
    let args = arguments(args)?;
    let mut connection_budget = args.input;
    let connection = load(&args.connection, &mut connection_budget)?;
    let Provider::NativeFixture {
        helper_executable,
        configuration,
        channels: 1,
    } = &connection.provider
    else {
        return Err(Failure::unsupported("native_fixture_required"));
    };
    let binding = NativeHelperBinding::form_session(
        executable(helper_executable)?,
        configuration.as_bytes(),
        args.duration,
    )
    .map_err(host_error)?;
    let session_end = until(args.duration)?;
    let limits = connection.host_limits.limits()?;
    let descriptor = Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Session(Box::new(connection.session.clone())),
    };
    let descriptor = encoded(&descriptor, limits.input_bytes)?;
    Document::from_json(&descriptor, limits.input_bytes)
        .map_err(|_| Failure::invalid("invalid_connection"))?;
    let target = TargetLease::authorized(&connection.target, true).map_err(host_error)?;
    let domain = HostDomain::new::<DarwinPlatform>(limits).map_err(host_error)?;
    let mut host =
        RuntimeHost::new(&domain, executable(&args.worker)?, DarwinPlatform).map_err(host_error)?;
    let result = (|| {
        let mut input = host
            .reserve_attach_input(target, descriptor.len())
            .map_err(host_error)?;
        input.bytes_mut().copy_from_slice(&descriptor);
        let attach_end = until(connection.attach_deadline_ms)?.min(session_end);
        let attached = host.attach(input, attach_end).map_err(host_error)?;
        let clock = match next(
            &mut host,
            attach_end + Duration::from_millis(limits.cleanup_ms),
        )? {
            HostEvent::Attached { session, clock } if session == attached => clock,
            _ => return Err(Failure::io()),
        };
        host.configure_native_helpers(attached, binding)
            .map_err(host_error)?;
        let mut stdin = std::fs::File::open("/dev/stdin").map_err(|_| Failure::io())?;
        let mut line = Vec::with_capacity(4096);
        loop {
            if Instant::now() >= session_end {
                return Ok(4);
            }
            // Poll registered owners even while the caller is idle; no UI reads.
            match host.next_event().map_err(host_error)? {
                HostEvent::Pending => (),
                HostEvent::HelperClosed { .. } => return Ok(4),
                _ => return Err(Failure::io()),
            }
            if !input_ready(stdin.as_fd(), 10).map_err(host_error)? {
                continue;
            }
            let mut byte = [0];
            if stdin.read(&mut byte).map_err(|_| Failure::io())? == 0 {
                return if line.is_empty() {
                    Ok(0)
                } else {
                    Err(Failure::invalid("incomplete_command"))
                };
            }
            if byte[0] != b'\n' {
                if line.len() == 4096 {
                    return Err(Failure::invalid("command_limit"));
                }
                line.push(byte[0]);
                continue;
            }
            let command: Command =
                serde_json::from_slice(&line).map_err(|_| Failure::invalid("invalid_command"))?;
            line.clear();
            let mut remaining = args.input;
            let raw = crate::input::read(&command.request, &mut remaining)?;
            let mut request = Document::from_json(&raw, args.input)
                .map_err(|_| Failure::invalid("invalid_request"))?;
            let Artifact::Request(r) = &mut request.artifact else {
                return Err(Failure::invalid("invalid_request"));
            };
            if r.context.target != connection.target
                || r.context.session_id != connection.session.session_id
                || r.context.plugin != connection.session.plugin
                || !connection
                    .session
                    .allowed_scopes
                    .contains(&r.context.scope_id)
                || r.context.surfaces != connection.session.surfaces
            {
                return Err(Failure::invalid("connection_request_mismatch"));
            }
            let class = match &r.operation {
                Operation::Observe { channels } if channels == &[Channel::ExternalSemantics] => {
                    OperationClass::Observe
                }
                Operation::Prepare { .. } => OperationClass::Prepare,
                Operation::Act { .. } => OperationClass::Mutation,
                _ => return Err(Failure::unsupported("unsupported_operation")),
            };
            let is_action = class != OperationClass::Observe;
            if is_action != (command.source.is_some() && command.expectation.is_some())
                || (!is_action && (command.source.is_some() || command.expectation.is_some()))
            {
                return Err(Failure::invalid("action_files_required"));
            }
            let source = command
                .source
                .as_ref()
                .map(|p| crate::input::read(p, &mut remaining))
                .transpose()?;
            let expected = command
                .expectation
                .as_ref()
                .map(|p| crate::input::read(p, &mut remaining))
                .transpose()?;
            r.clock_domain = Id(clock.as_str().into());
            let end = until(r.limits.deadline_ms)?.min(session_end);
            let cap = args
                .output
                .checked_sub(1)
                .ok_or(Failure::invalid("output_limit"))?
                .min(r.limits.max_output_bytes as usize)
                .min(limits.output_bytes);
            let request = encoded(&request, limits.input_bytes)?;
            let parts = if is_action {
                vec![
                    source.as_ref().unwrap().as_slice(),
                    request.as_slice(),
                    expected.as_ref().unwrap().as_slice(),
                ]
            } else {
                vec![request.as_slice()]
            };
            let size = if is_action {
                tape_len(&parts)?
            } else {
                request.len()
            };
            let mut input = host.reserve_input(attached, size).map_err(host_error)?;
            if is_action {
                worker_tape::encode(&parts, input.bytes_mut()).map_err(host_error)?;
            } else {
                input.bytes_mut().copy_from_slice(&request);
            }
            let out = OutputRequest {
                channels: 1,
                frame_bytes: cap,
                total_bytes: cap,
                input_format: u8::from(is_action),
                retained_partition: 0,
            };
            if is_action {
                host.submit(attached, class, input, out, end)
                    .map_err(host_error)?;
            } else {
                host.submit_native_observe(attached, input, out, end)
                    .map_err(host_error)?;
            }
            let completion = loop {
                match next(&mut host, end + Duration::from_millis(limits.cleanup_ms))? {
                    HostEvent::Complete(c) => break c,
                    HostEvent::HelperClosed { .. } => continue,
                    _ => return Err(Failure::io()),
                }
            };
            crate::output::observation_lines(&completion, args.output, output)?;
            output.flush().map_err(|_| Failure::io())?;
            if let Terminal::Failed(error) = completion.terminal {
                if class == OperationClass::Mutation && completion.effect_unknown() {
                    return Ok(4);
                }
                return Err(host_error(error));
            }
            if completion.terminal != Terminal::Completed || completion.missing() != 0 {
                return Ok(4);
            }
            if class == OperationClass::Mutation
                && completion.action_status() == Some(ActionPublicationStatus::VerifiedMismatch)
            {
                return Ok(3);
            }
            if class == OperationClass::Mutation
                && (completion.effect == EffectReceipt::NotDispatched
                    || completion.action_status() != Some(ActionPublicationStatus::VerifiedSuccess))
            {
                return Ok(4);
            }
            if class == OperationClass::Prepare
                && completion.action_status() != Some(ActionPublicationStatus::Prepared)
            {
                return Ok(4);
            }
            // Partial AX coverage is usable observation, not a claim of full scope.
        }
    })();
    shutdown(&mut host, limits.cleanup_ms, &mut None)?;
    result
}
