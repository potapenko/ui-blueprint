//! Explicit operator-capability caller; canonical observation bodies stay opaque.
use crate::{Failure, arguments::ObserveArguments};
use std::io::Write;

#[cfg(not(all(target_os = "macos", any(feature = "macos", feature = "web"))))]
pub(crate) fn execute(args: ObserveArguments, _: &mut impl Write) -> Result<u8, Failure> {
    let _ = (
        args.connection,
        args.request,
        args.worker,
        args.max_input,
        args.max_output,
    );
    Err(Failure::unsupported(
        "unsupported_observe_platform_or_feature",
    ))
}

#[cfg(all(target_os = "macos", any(feature = "macos", feature = "web")))]
pub(crate) use supported::execute;

#[cfg(all(target_os = "macos", any(feature = "macos", feature = "web")))]
mod supported {
    use super::*;
    use serde::{Deserialize, Deserializer};
    use std::{
        fs,
        path::Path,
        thread,
        time::{Duration, Instant},
    };
    #[cfg(feature = "web")]
    use uiblueprint_host::worker_tape;
    use uiblueprint_host::{
        HostError, HostLimits,
        authority::TargetLease,
        domain::HostDomain,
        host_types::{HostCompletion, HostEvent, OutputRequest, Terminal},
        native_binding::NativeHelperBinding,
        process::DarwinPlatform,
        process_api::SpawnSpec,
        supervisor::RuntimeHost,
    };
    use uiblueprint_schema::{SchemaVersion, model::*};

    // Explicit trusted configuration only. This duplicates no canonical graph;
    // HostLimits itself intentionally has no public serialized configuration.
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Profile {
        workers: usize,
        worker_bytes: usize,
        publication_reserve: usize,
        bootstrap_bytes: usize,
        parent_bytes: usize,
        input_bytes: usize,
        ingress_bytes: usize,
        output_bytes: usize,
        request_output_bytes: usize,
        completion_groups: usize,
        control_bytes: usize,
        cleanup_ms: u64,
        retained_domain_bytes: usize,
        retained_per_worker: usize,
        main_stack_bytes: usize,
        watchdog_stack_bytes: usize,
    }
    impl Profile {
        fn limits(self) -> Result<HostLimits, Failure> {
            HostLimits {
                workers: self.workers,
                worker_bytes: self.worker_bytes,
                publication_reserve: self.publication_reserve,
                bootstrap_bytes: self.bootstrap_bytes,
                parent_bytes: self.parent_bytes,
                input_bytes: self.input_bytes,
                ingress_bytes: self.ingress_bytes,
                output_bytes: self.output_bytes,
                request_output_bytes: self.request_output_bytes,
                completion_groups: self.completion_groups,
                control_bytes: self.control_bytes,
                cleanup_ms: self.cleanup_ms,
                retained_domain_bytes: self.retained_domain_bytes,
                retained_per_worker: self.retained_per_worker,
                main_stack_bytes: self.main_stack_bytes,
                watchdog_stack_bytes: self.watchdog_stack_bytes,
            }
            .validate()
            .map_err(host_error)
        }
    }
    #[derive(Deserialize)]
    #[serde(tag = "backend", rename_all = "snake_case", deny_unknown_fields)]
    enum Provider {
        NativeFixture {
            helper_executable: std::path::PathBuf,
            configuration: String,
            channels: u8,
        },
        #[cfg(feature = "web")]
        Web {
            #[serde(deserialize_with = "object")]
            setup: Box<uiblueprint_host::web_config::WebSetup>,
            #[serde(deserialize_with = "object")]
            selection: uiblueprint_host::web_config::WebSelection,
        },
        #[cfg(not(feature = "web"))]
        Web {
            setup: serde::de::IgnoredAny,
            selection: serde::de::IgnoredAny,
        },
        #[serde(other)]
        Unsupported,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Connection {
        connection_version: String,
        target: Identity,
        session: SessionDescriptor,
        #[serde(deserialize_with = "object")]
        host_limits: Profile,
        attach_deadline_ms: u64,
        provider: Provider,
    }
    // serde derives can accept sequence-form structs; local connection records
    // must remain objects just like the existing canonical records they embed.
    fn object<'de, D: Deserializer<'de>, T: Deserialize<'de>>(d: D) -> Result<T, D::Error> {
        struct Map<T>(std::marker::PhantomData<T>);
        impl<'de, T: Deserialize<'de>> serde::de::Visitor<'de> for Map<T> {
            type Value = T;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(self, access: A) -> Result<T, A::Error> {
                T::deserialize(serde::de::value::MapAccessDeserializer::new(access))
            }
        }
        d.deserialize_map(Map(std::marker::PhantomData))
    }
    fn host_error(error: HostError) -> Failure {
        use HostError::*;
        match error {
            InvalidLimits | ResourceLimit | Overflow | InvalidInput => {
                Failure::invalid("observe_invalid_or_limit")
            }
            PermissionDenied | ResyncRequired | StaleOperation | Busy | DeadlineExpired => {
                Failure {
                    code: "observe_unavailable",
                    exit: 4,
                }
            }
            AllocationFailure
            | SystemAllocationFailure
            | Io
            | WorkerFailed
            | InvalidState
            | InvalidControl
            | CleanupPending => Failure {
                code: "observe_worker_or_cleanup_failure",
                exit: 1,
            },
        }
    }
    fn until(ms: u64) -> Result<Instant, Failure> {
        if ms == 0 {
            return Err(Failure::invalid("invalid_deadline"));
        }
        Instant::now()
            .checked_add(Duration::from_millis(ms))
            .ok_or(Failure::invalid("invalid_deadline"))
    }
    fn executable(path: &Path) -> Result<SpawnSpec, Failure> {
        let spec = SpawnSpec::new(path).map_err(host_error)?;
        if !fs::metadata(path).map_err(|_| Failure::io())?.is_file() {
            return Err(Failure::io());
        }
        Ok(spec)
    }
    fn next<'a>(
        host: &mut RuntimeHost<'a, DarwinPlatform>,
        end: Instant,
    ) -> Result<HostEvent<'a>, Failure> {
        loop {
            match host.next_event().map_err(host_error)? {
                HostEvent::Pending
                | HostEvent::HelperClosed { .. }
                | HostEvent::HelperCleanupPending { .. } => (),
                event => return Ok(event),
            }
            if Instant::now() >= end {
                return Err(host_error(HostError::DeadlineExpired));
            }
            thread::sleep(Duration::from_millis(1));
        }
    }
    fn shutdown<'a>(
        host: &mut RuntimeHost<'a, DarwinPlatform>,
        ms: u64,
        completion: &mut Option<HostCompletion<'a>>,
    ) -> Result<(), Failure> {
        let end = until(ms)?;
        loop {
            match host.shutdown().map_err(host_error)? {
                HostEvent::ShutdownComplete => return Ok(()),
                HostEvent::Complete(c) => *completion = Some(c),
                HostEvent::CleanupPending { .. } | HostEvent::HelperCleanupPending { .. } => {
                    return Err(host_error(HostError::CleanupPending));
                }
                _ => (),
            }
            if Instant::now() >= end {
                return Err(host_error(HostError::CleanupPending));
            }
            thread::sleep(Duration::from_millis(1));
        }
    }
    fn encoded(document: &Document, limit: usize) -> Result<Vec<u8>, Failure> {
        // Uses the existing CLI bounded writer, not an unbounded payload clone.
        crate::output::trusted_input(document, limit)
    }
    #[cfg(feature = "web")]
    fn tape_len(parts: &[&[u8]]) -> Result<usize, Failure> {
        parts
            .iter()
            .try_fold(8 + worker_tape::SEGMENTS * 8, |n, part| {
                n.checked_add(part.len())
            })
            .ok_or(Failure::invalid("input_limit"))
    }
    pub(crate) fn execute(args: ObserveArguments, output: &mut impl Write) -> Result<u8, Failure> {
        let mut remaining = args.max_input;
        let bytes = crate::input::read(&args.connection, &mut remaining)?;
        if bytes.iter().find(|c| !c.is_ascii_whitespace()) != Some(&b'{') {
            return Err(Failure::invalid("invalid_connection"));
        }
        let connection: Connection =
            serde_json::from_slice(&bytes).map_err(|_| Failure::invalid("invalid_connection"))?;
        drop(bytes);
        if connection.connection_version != "1.0.0" {
            return Err(Failure::invalid("invalid_connection_version"));
        }
        let request_bytes = crate::input::read(&args.request, &mut remaining)?;
        let mut request = Document::from_json(&request_bytes, args.max_input)
            .map_err(|_| Failure::invalid("invalid_request"))?;
        drop(request_bytes);
        let Artifact::Request(r) = &request.artifact else {
            return Err(Failure::invalid("invalid_request"));
        };
        let Operation::Observe { channels } = &r.operation else {
            return Err(Failure::invalid("invalid_request"));
        };
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
        {
            return Err(Failure::invalid("connection_request_mismatch"));
        }
        let mask: u8 = channels.iter().fold(0, |bits, channel| {
            bits | match channel {
                Channel::ExternalSemantics => 1,
                Channel::RenderedCapture => 2,
                Channel::OptInLayoutProbe => 4,
            }
        });
        let deadline_ms = r.limits.deadline_ms;
        let request_output = r.limits.max_output_bytes as usize;
        let limits = connection.host_limits.limits()?;
        let attach_end = until(connection.attach_deadline_ms)?;
        until(deadline_ms)?;
        let total = args
            .max_output
            .checked_sub(mask.count_ones() as usize)
            .filter(|n| *n > 0)
            .ok_or(Failure::invalid("output_limit"))?
            .min(limits.request_output_bytes)
            .min(request_output);
        let requested = OutputRequest {
            channels: mask,
            frame_bytes: limits.output_bytes.min(total),
            total_bytes: total,
            input_format: 0,
            retained_partition: 0,
        };
        let worker = executable(&args.worker)?;
        let native = match &connection.provider {
            Provider::NativeFixture {
                helper_executable,
                configuration,
                channels,
            } => {
                if !cfg!(feature = "macos") {
                    return Err(Failure::unsupported("unsupported_observe_backend"));
                }
                if mask & !channels != 0 {
                    return Err(Failure::invalid("connection_request_mismatch"));
                }
                if configuration
                    .len()
                    .saturating_add(uiblueprint_host::CONTROL_BYTES)
                    > limits.control_bytes
                {
                    return Err(Failure::invalid("input_limit"));
                }
                Some(
                    NativeHelperBinding::authorized(
                        executable(helper_executable)?,
                        *channels,
                        configuration.as_bytes(),
                    )
                    .map_err(host_error)?,
                )
            }
            #[cfg(feature = "web")]
            Provider::Web { setup, .. } => {
                if !session.surfaces.contains(&setup.surface) {
                    return Err(Failure::invalid("connection_request_mismatch"));
                }
                None
            }
            #[cfg(not(feature = "web"))]
            Provider::Web { setup, selection } => {
                let _ = (setup, selection);
                return Err(Failure::unsupported("unsupported_observe_backend"));
            }
            Provider::Unsupported => {
                return Err(Failure::unsupported("unsupported_observe_backend"));
            }
        };
        let descriptor = Document {
            schema_version: SchemaVersion::CURRENT,
            artifact: Artifact::Session(Box::new(connection.session)),
        };
        let descriptor = encoded(&descriptor, limits.input_bytes)?;
        // Validate the trusted descriptor before dispatch; output bodies stay opaque.
        Document::from_json(&descriptor, limits.input_bytes)
            .map_err(|_| Failure::invalid("invalid_connection"))?;
        #[cfg(feature = "web")]
        let web_bytes = match &connection.provider {
            Provider::Web { setup, selection } => Some((
                crate::output::trusted_input(setup, limits.input_bytes)?,
                crate::output::trusted_input(selection, limits.input_bytes)?,
            )),
            _ => None,
        };
        let domain = HostDomain::new::<DarwinPlatform>(limits).map_err(host_error)?;
        let mut host = RuntimeHost::new(&domain, worker, DarwinPlatform).map_err(host_error)?;
        let result = (|| -> Result<HostCompletion<'_>, Failure> {
            let target = TargetLease::authorized(&connection.target, false).map_err(host_error)?;
            #[cfg(feature = "web")]
            let attached = if let Some((setup, _)) = &web_bytes {
                let parts = [&descriptor[..], &setup[..]];
                let mut input = host
                    .reserve_attach_input(target, tape_len(&parts)?)
                    .map_err(host_error)?;
                worker_tape::encode(&parts, input.bytes_mut()).map_err(host_error)?;
                host.attach_web(input, attach_end).map_err(host_error)?
            } else {
                let mut input = host
                    .reserve_attach_input(target, descriptor.len())
                    .map_err(host_error)?;
                input.bytes_mut().copy_from_slice(&descriptor);
                host.attach(input, attach_end).map_err(host_error)?
            };
            #[cfg(not(feature = "web"))]
            let attached = {
                let mut input = host
                    .reserve_attach_input(target, descriptor.len())
                    .map_err(host_error)?;
                input.bytes_mut().copy_from_slice(&descriptor);
                host.attach(input, attach_end).map_err(host_error)?
            };
            let event_end = attach_end
                .checked_add(Duration::from_millis(limits.cleanup_ms))
                .ok_or(Failure::invalid("invalid_deadline"))?;
            let clock = match next(&mut host, event_end)? {
                HostEvent::Attached { session, clock } if session == attached => clock,
                HostEvent::Complete(c) => {
                    return Err(completion_error(&c).unwrap_or(host_error(HostError::WorkerFailed)));
                }
                _ => return Err(host_error(HostError::WorkerFailed)),
            };
            if let Some(binding) = native {
                host.configure_native_helpers(attached, binding)
                    .map_err(host_error)?;
            }
            let Artifact::Request(r) = &mut request.artifact else {
                unreachable!("Request validated above")
            };
            r.clock_domain = Id(clock.as_str().into());
            let request = encoded(&request, limits.input_bytes)?;
            let end = until(deadline_ms)?;
            #[cfg(feature = "web")]
            if let Some((_, selection)) = &web_bytes {
                let parts = [&request[..], &selection[..]];
                let mut input = host
                    .reserve_input(attached, tape_len(&parts)?)
                    .map_err(host_error)?;
                worker_tape::encode(&parts, input.bytes_mut()).map_err(host_error)?;
                host.submit_web_observe(attached, input, requested, end)
                    .map_err(host_error)?;
            } else {
                let mut input = host
                    .reserve_input(attached, request.len())
                    .map_err(host_error)?;
                input.bytes_mut().copy_from_slice(&request);
                host.submit_native_observe(attached, input, requested, end)
                    .map_err(host_error)?;
            }
            #[cfg(not(feature = "web"))]
            {
                let mut input = host
                    .reserve_input(attached, request.len())
                    .map_err(host_error)?;
                input.bytes_mut().copy_from_slice(&request);
                host.submit_native_observe(attached, input, requested, end)
                    .map_err(host_error)?;
            }
            let event_end = end
                .checked_add(Duration::from_millis(limits.cleanup_ms))
                .ok_or(Failure::invalid("invalid_deadline"))?;
            match next(&mut host, event_end)? {
                HostEvent::Complete(c) => Ok(c),
                _ => Err(host_error(HostError::WorkerFailed)),
            }
        })();
        let mut terminal_during_shutdown = None;
        let cleanup = shutdown(&mut host, limits.cleanup_ms, &mut terminal_during_shutdown);
        let (completion, failure) = match result {
            Ok(c) => (Some(c), None),
            Err(error) => (terminal_during_shutdown, Some(error)),
        };
        if let Some(c) = &completion {
            crate::output::observation_lines(c, args.max_output, output)?;
        }
        cleanup?;
        if let Some(error) = failure.or_else(|| completion.as_ref().and_then(completion_error)) {
            return Err(error);
        }
        Ok(0)
    }
    fn completion_error(c: &HostCompletion<'_>) -> Option<Failure> {
        match c.terminal {
            Terminal::Completed if c.missing() == 0 && c.incomplete_channels() == 0 => None,
            Terminal::Completed | Terminal::Cancelled | Terminal::TimedOut => Some(Failure {
                code: "observe_incomplete",
                exit: 4,
            }),
            Terminal::Failed(error) => Some(host_error(error)),
        }
    }
}
