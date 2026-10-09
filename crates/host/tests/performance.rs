//! Q02 opt-in timing consumer. Uses the shipping guarded host, never a second collector.
#![cfg(all(target_os = "macos", feature = "web"))]
use serde::Deserialize;
use std::{
    fs,
    io::{self, BufRead, Read, Write},
    path::Path,
    thread,
    time::{Duration, Instant},
};
use uiblueprint_host::{
    HostError, HostLimits,
    authority::TargetLease,
    domain::HostDomain,
    host_types::{HostEvent, OutputRequest},
    native_binding::NativeHelperBinding,
    process::DarwinPlatform,
    process_api::SpawnSpec,
    supervisor::RuntimeHost,
    web_config::{WebSelection, WebSetup},
    worker_tape,
};
use uiblueprint_schema::model::{Artifact, Document, Id};

#[derive(Deserialize)]
#[serde(tag = "backend", rename_all = "snake_case", deny_unknown_fields)]
enum Provider {
    Web {
        setup: Box<WebSetup>,
    },
    Native {
        helper: String,
        configuration: String,
        channels: u8,
    },
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    descriptor: Document,
    provider: Provider,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Call {
    request: Document,
    selection: Option<WebSelection>,
    native_configuration: Option<String>,
}

fn limits() -> HostLimits {
    const MIB: usize = 1048576;
    HostLimits {
        workers: 2,
        worker_bytes: 64 * MIB,
        publication_reserve: MIB,
        bootstrap_bytes: MIB,
        parent_bytes: 32 * MIB,
        input_bytes: 2 * MIB,
        ingress_bytes: 512 * 1024,
        output_bytes: 512 * 1024,
        request_output_bytes: 2 * MIB,
        completion_groups: 2,
        control_bytes: 4096,
        cleanup_ms: 1000,
        retained_domain_bytes: 64 * MIB,
        retained_per_worker: 15 * MIB,
        main_stack_bytes: 8 * MIB,
        watchdog_stack_bytes: MIB,
    }
}

fn emit(value: serde_json::Value) {
    let mut stdout = io::stdout().lock();
    writeln!(stdout, "@Q02 {}", value).expect("owned harness stdout");
    stdout.flush().expect("harness flush");
}
fn usage(domain: &HostDomain) -> serde_json::Value {
    let value = domain.usage();
    serde_json::json!({"parent_owned_bytes":value.parent_owned_bytes,
        "retained_reserved_bytes":value.retained_reserved_bytes,"reserved_sessions":value.reserved_sessions,
        "completion_groups":value.completion_groups,"abandoned":value.abandoned,"reaping_poisoned":value.reaping_poisoned})
}
fn next<'a>(
    host: &mut RuntimeHost<'a, DarwinPlatform>,
    end: Instant,
) -> Result<HostEvent<'a>, HostError> {
    loop {
        if Instant::now() >= end {
            return Err(HostError::DeadlineExpired);
        }
        match host.next_event()? {
            HostEvent::Pending | HostEvent::HelperClosed { .. } => {
                thread::sleep(Duration::from_micros(100))
            }
            event => return Ok(event),
        }
    }
}
fn tape(parts: &[&[u8]]) -> Vec<u8> {
    let mut data =
        vec![0; 8 + worker_tape::SEGMENTS * 8 + parts.iter().map(|p| p.len()).sum::<usize>()];
    worker_tape::encode(parts, &mut data).expect("bounded test Tape");
    data
}
fn cleanup(host: &mut RuntimeHost<'_, DarwinPlatform>) -> bool {
    let end = Instant::now() + Duration::from_secs(3);
    while Instant::now() < end {
        match host.shutdown() {
            Ok(HostEvent::ShutdownComplete) => return true,
            Ok(_) => thread::sleep(Duration::from_micros(100)),
            Err(_) => return false,
        }
    }
    false
}

#[test]
#[ignore = "Q02 requires explicit functional pin, idle resource release and owned fixture"]
fn requested_series() {
    assert_eq!(std::env::var("UIB_Q02_ALLOW").as_deref(), Ok("1"));
    let started = Instant::now();
    let config_path = std::env::var("UIB_Q02_CONFIG").expect("explicit config");
    let mut bytes = Vec::new();
    fs::File::open(config_path)
        .expect("owned config")
        .take(65537)
        .read_to_end(&mut bytes)
        .expect("bounded config read");
    assert!(bytes.len() <= 65536);
    let config: Config = serde_json::from_slice(&bytes).expect("strict config");
    config.descriptor.validate().expect("canonical descriptor");
    let Artifact::Session(descriptor) = &config.descriptor.artifact else {
        panic!("session required")
    };
    let target = TargetLease::authorized(&descriptor.target, false).expect("read-only own fixture");
    let domain = HostDomain::new::<DarwinPlatform>(limits()).expect("unchanged D05 profile");
    let mut host = RuntimeHost::new(
        &domain,
        SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).expect("saved worker"),
        DarwinPlatform,
    )
    .expect("guarded host");
    let outcome = (|| -> Result<(), HostError> {
        let descriptor =
            serde_json::to_vec(&config.descriptor).map_err(|_| HostError::InvalidInput)?;
        let attach_bytes = match &config.provider {
            Provider::Web { setup } => tape(&[
                &descriptor,
                &serde_json::to_vec(setup).map_err(|_| HostError::InvalidInput)?,
            ]),
            Provider::Native { .. } => descriptor,
        };
        let mut input = host.reserve_attach_input(target, attach_bytes.len())?;
        input.bytes_mut().copy_from_slice(&attach_bytes);
        let end = Instant::now() + Duration::from_secs(2);
        let session = match &config.provider {
            Provider::Web { .. } => host.attach_web(input, end)?,
            Provider::Native { .. } => host.attach(input, end)?,
        };
        let HostEvent::Attached { clock, .. } = next(&mut host, end)? else {
            return Err(HostError::WorkerFailed);
        };
        let clock = clock.as_str().to_owned();
        if let Provider::Native {
            helper,
            configuration,
            channels,
        } = &config.provider
        {
            host.configure_native_helpers(
                session,
                NativeHelperBinding::form_session(
                    SpawnSpec::new(Path::new(helper))?,
                    configuration.as_bytes(),
                    180_000,
                )?,
            )?;
            if channels & 2 != 0 {
                host.configure_native_helpers(
                    session,
                    NativeHelperBinding::authorized(
                        SpawnSpec::new(Path::new(helper))?,
                        2,
                        configuration.as_bytes(),
                    )?,
                )?;
            }
        }
        emit(
            serde_json::json!({"kind":"attached","setup_ms":started.elapsed().as_secs_f64()*1000.0}),
        );
        let stdin = io::stdin();
        let mut input = stdin.lock();
        let mut number = 0;
        loop {
            // Finite caller commands; waiting here cannot collect UI. Bounded before JSON parse.
            let mut line = String::new();
            let count = input
                .by_ref()
                .take(65537)
                .read_line(&mut line)
                .map_err(|_| HostError::Io)?;
            if count == 0 {
                break;
            }
            if count > 65536 || !line.ends_with('\n') {
                return Err(HostError::InvalidInput);
            }
            number += 1;
            if number > 121 {
                return Err(HostError::InvalidInput);
            }
            let begin = Instant::now();
            let mut call: Call =
                serde_json::from_str(&line).map_err(|_| HostError::InvalidInput)?;
            if let Some(configuration) = &call.native_configuration {
                let Provider::Native {
                    helper, channels, ..
                } = &config.provider
                else {
                    return Err(HostError::InvalidInput);
                };
                let end = begin + Duration::from_secs(1);
                loop {
                    let binding = NativeHelperBinding::authorized(
                        SpawnSpec::new(Path::new(helper))?,
                        if channels & 2 != 0 { 2 } else { *channels },
                        configuration.as_bytes(),
                    )?;
                    match host.configure_native_helpers(session, binding) {
                        Ok(()) => break,
                        Err(HostError::Busy) if Instant::now() < end => {
                            // Drive only already-owned helper cleanup, never UI acquisition.
                            match host.next_event()? {
                                HostEvent::Pending | HostEvent::HelperClosed { .. } => {
                                    thread::sleep(Duration::from_micros(100))
                                }
                                _ => return Err(HostError::InvalidState),
                            }
                        }
                        Err(error) => return Err(error),
                    }
                }
            }
            let Artifact::Request(request) = &mut call.request.artifact else {
                return Err(HostError::InvalidInput);
            };
            request.clock_domain = Id(clock.clone());
            let deadline_ms = request.limits.deadline_ms;
            let frame = usize::try_from(request.limits.max_output_bytes)
                .map_err(|_| HostError::InvalidInput)?;
            let mask = match &config.provider {
                Provider::Web { .. } => 1,
                Provider::Native { channels, .. } => *channels,
            };
            let request = serde_json::to_vec(&call.request).map_err(|_| HostError::InvalidInput)?;
            let payload = match &config.provider {
                Provider::Web { .. } => tape(&[
                    &request,
                    &serde_json::to_vec(call.selection.as_ref().ok_or(HostError::InvalidInput)?)
                        .map_err(|_| HostError::InvalidInput)?,
                ]),
                Provider::Native { .. } => {
                    if call.selection.is_some() {
                        return Err(HostError::InvalidInput);
                    }
                    request
                }
            };
            let mut lease = host.reserve_input(session, payload.len())?;
            lease.bytes_mut().copy_from_slice(&payload);
            let output = OutputRequest {
                channels: mask,
                frame_bytes: frame,
                total_bytes: frame * mask.count_ones() as usize,
                input_format: 0,
                retained_partition: 0,
            };
            let end = begin + Duration::from_millis(deadline_ms);
            match &config.provider {
                Provider::Web { .. } => host.submit_web_observe(session, lease, output, end)?,
                Provider::Native { .. } => {
                    host.submit_native_observe(session, lease, output, end)?
                }
            };
            let HostEvent::Complete(completion) = next(&mut host, end + Duration::from_secs(1))?
            else {
                return Err(HostError::WorkerFailed);
            };
            // This timer includes caller decoding/encoding, dispatch, acquisition,
            // guarded normalization/validation/encoding, transport and parent ACK.
            // Driver separately includes canonical stdout delivery in its outer timer.
            let request_ms = begin.elapsed().as_secs_f64() * 1000.0;
            let frames: Vec<_> = (0..3).filter_map(|slot|completion.bytes(slot).map(|bytes|
                serde_json::json!({"slot":slot,"canonical":std::str::from_utf8(bytes).expect("canonical UTF8")}))).collect();
            emit(
                serde_json::json!({"kind":"sample","number":number,"request_ms":request_ms,
                "terminal":format!("{:?}",completion.terminal),"committed":completion.committed(),
                "missing":completion.missing(),"domain_usage":usage(&domain),"frames":frames}),
            );
        }
        Ok(())
    })();
    let released = cleanup(&mut host);
    emit(
        serde_json::json!({"kind":"closed","cleanup_confirmed":released,"domain_usage":usage(&domain),
        "error":outcome.as_ref().err().map(|e|format!("{e:?}"))}),
    );
    assert!(released && outcome.is_ok(), "Q02 caller or cleanup failed");
}
