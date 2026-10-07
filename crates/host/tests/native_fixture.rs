//! Explicit opt-in SDK consumer. Merely compiling/running the ordinary suite
//! never launches the fixture or invokes a Native helper.
#![cfg(target_os = "macos")]
use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::unix::fs::OpenOptionsExt,
    path::{Path, PathBuf},
    thread,
    time::{Duration, Instant},
};
use uiblueprint_host::{
    HostError, HostLimits, OperationClass,
    authority::TargetLease,
    domain::HostDomain,
    host_types::{HostEvent, OutputRequest, Terminal},
    native_binding::NativeHelperBinding,
    process::DarwinPlatform,
    process_api::SpawnSpec,
    supervisor::RuntimeHost,
    worker_tape,
};
use uiblueprint_schema::{
    SchemaVersion,
    analysis::{AnalysisArtifact, AnalysisDocument, MeasurementResult},
    model::{Artifact, Channel, ChannelResult, Document, Id, Operation},
};

const FRAME: usize = 512 * 1024;
const MIB: usize = 1_048_576;
fn limits() -> HostLimits {
    HostLimits {
        workers: 1,
        worker_bytes: 64 * MIB,
        publication_reserve: MIB,
        bootstrap_bytes: MIB,
        parent_bytes: 32 * MIB,
        input_bytes: 2 * MIB,
        ingress_bytes: FRAME,
        output_bytes: FRAME,
        request_output_bytes: 2 * MIB,
        completion_groups: 1,
        control_bytes: 4096,
        cleanup_ms: 1000,
        retained_domain_bytes: 64 * MIB,
        retained_per_worker: 15 * MIB,
        main_stack_bytes: 8 * MIB,
        watchdog_stack_bytes: MIB,
    }
}
fn path(name: &str) -> PathBuf {
    let value = PathBuf::from(env::var_os(name).expect("explicit Native fixture path"));
    assert!(value.is_absolute(), "Native paths must be absolute");
    value
}
fn read(path: &Path, cap: usize) -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    File::open(path)?
        .take((cap + 1) as u64)
        .read_to_end(&mut bytes)?;
    if bytes.len() > cap {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "bounded fixture input",
        ));
    }
    Ok(bytes)
}
fn new_file(path: &Path) -> std::io::Result<File> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
}
fn next<'a>(
    host: &mut RuntimeHost<'a, DarwinPlatform>,
    until: Instant,
) -> Result<HostEvent<'a>, HostError> {
    loop {
        if Instant::now() >= until {
            return Err(HostError::DeadlineExpired);
        }
        match host.next_event()? {
            HostEvent::Pending | HostEvent::HelperClosed { .. } => {
                thread::sleep(Duration::from_millis(1))
            }
            event => return Ok(event),
        }
    }
}

#[test]
#[ignore = "requires the explicitly authorized owned F02 A runtime packet"]
fn actual_owned_f02_ax_observation() {
    assert_eq!(
        env::var("UIB_NATIVE_FIXTURE_LIVE").as_deref(),
        Ok("owned_f02_a")
    );
    let input_dir = path("UIB_NATIVE_INPUT_DIR");
    let output_dir = path("UIB_NATIVE_OUTPUT_DIR");
    let helper = path("UIB_NATIVE_HELPER");
    fs::create_dir(&output_dir).expect("new own output directory");
    let descriptor = read(&input_dir.join("session.json"), FRAME).expect("bounded descriptor");
    let request = read(&input_dir.join("request.json"), FRAME).expect("bounded request");
    let configuration =
        read(&input_dir.join("helper.json"), 4032).expect("bounded trusted configuration");
    let Artifact::Session(session_descriptor) = Document::from_json(&descriptor, FRAME)
        .expect("valid fixture descriptor")
        .artifact
    else {
        panic!("fixture Session artifact");
    };
    let target = TargetLease::authorized(&session_descriptor.target, false)
        .expect("trusted fixture authority");
    let mut request = Document::from_json(&request, FRAME).expect("valid fixture Request");
    let Artifact::Request(r) = &request.artifact else {
        panic!("fixture Request artifact");
    };
    assert_eq!(r.context.target, session_descriptor.target);
    let requested_channel = match &r.operation {
        Operation::Observe { channels } if channels.len() == 1 => channels[0],
        _ => panic!("one Native observation channel"),
    };
    let (mask, slot) = match requested_channel {
        Channel::ExternalSemantics => (1, 0),
        Channel::OptInLayoutProbe => (4, 2),
        _ => panic!("capture outside this fixture caller"),
    };
    assert_eq!(r.limits.max_elements, 160);
    assert_eq!(r.limits.max_depth, 9);
    assert_eq!(r.limits.deadline_ms, 1000);
    assert_eq!(r.limits.max_output_bytes, FRAME as u64);
    // Setup/caller DTO allocation is explicit test tooling, outside the parent's
    // bounded steady-state loop. Completed graph JSON is verified by the launcher.
    let domain = HostDomain::new::<DarwinPlatform>(limits()).expect("host domain");
    let mut host = RuntimeHost::new(
        &domain,
        SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).expect("saved worker"),
        DarwinPlatform,
    )
    .expect("runtime host");
    let mut clock_id = String::new();
    let mut committed = 0_u8;
    let mut missing = mask;
    let mut terminal = "not_completed";
    let mut diagnostic = None;
    let mut caller_stage = "attach";
    let outcome = (|| -> Result<(), HostError> {
        let mut lease = host.reserve_attach_input(target, descriptor.len())?;
        lease.bytes_mut().copy_from_slice(&descriptor);
        let session = host.attach(lease, Instant::now() + Duration::from_secs(5))?;
        let HostEvent::Attached { clock, .. } =
            next(&mut host, Instant::now() + Duration::from_secs(5))?
        else {
            return Err(HostError::WorkerFailed);
        };
        clock_id = clock.as_str().to_owned();
        let Artifact::Request(r) = &mut request.artifact else {
            unreachable!("checked Request");
        };
        r.clock_domain = Id(clock_id.clone());
        let bytes = serde_json::to_vec(&request).map_err(|_| HostError::InvalidInput)?;
        new_file(&output_dir.join("submitted-request.json"))
            .and_then(|mut file| file.write_all(&bytes))
            .map_err(|_| HostError::Io)?;
        caller_stage = "configure_native";
        host.configure_native_helpers(
            session,
            NativeHelperBinding::authorized(SpawnSpec::new(&helper)?, mask, &configuration)?,
        )?;
        caller_stage = "reserve_observe_input";
        let mut lease = host.reserve_input(session, bytes.len())?;
        lease.bytes_mut().copy_from_slice(&bytes);
        caller_stage = "submit_native_observe";
        host.submit_native_observe(
            session,
            lease,
            OutputRequest {
                channels: mask,
                frame_bytes: FRAME,
                total_bytes: FRAME,
                input_format: 0,
                retained_partition: 0,
            },
            Instant::now() + Duration::from_millis(1000),
        )?;
        caller_stage = "await_observe_completion";
        let HostEvent::Complete(completion) =
            next(&mut host, Instant::now() + Duration::from_secs(2))?
        else {
            return Err(HostError::WorkerFailed);
        };
        committed = completion.committed();
        missing = completion.missing();
        diagnostic = completion.diagnostic().map(|d| serde_json::json!({
            "stage": d.stage as u8, "cause": d.cause as u8, "code": d.code,
            "count": d.count, "remote_cleanup": d.remote_cleanup, "send_progress": d.send_progress
        }));
        terminal = match completion.terminal {
            Terminal::Completed => "completed",
            Terminal::Failed(_) => "failed",
            Terminal::Cancelled => "cancelled",
            Terminal::TimedOut => "timed_out",
        };
        caller_stage = "write_committed_channel";
        if completion.bytes(slot).is_some() {
            let mut file =
                new_file(&output_dir.join("channel-0.json")).map_err(|_| HostError::Io)?;
            completion
                .write_channel(slot, &mut file)
                .map_err(|_| HostError::Io)?;
        }
        if completion.terminal != Terminal::Completed || committed != mask || missing != 0 {
            return Err(HostError::WorkerFailed);
        }
        caller_stage = "measure_input";
        if requested_channel == Channel::OptInLayoutProbe {
            let bytes = completion.bytes(slot).ok_or(HostError::WorkerFailed)?;
            let source = Document::from_json(bytes, FRAME).map_err(|_| HostError::InvalidInput)?;
            let Artifact::ChannelResponse(response) = source.artifact else {
                return Err(HostError::InvalidInput);
            };
            let ChannelResult::Observed(snapshot) = response.result else {
                return Err(HostError::WorkerFailed);
            };
            let query = read(&input_dir.join("query.json"), FRAME).map_err(|_| HostError::Io)?;
            let query =
                AnalysisDocument::from_json(&query, FRAME).map_err(|_| HostError::InvalidInput)?;
            let snapshot_bytes = serde_json::to_vec(&Document {
                schema_version: SchemaVersion::CURRENT,
                artifact: Artifact::Snapshot(snapshot.clone()),
            })
            .map_err(|_| HostError::InvalidInput)?;
            // Evaluation binds the unchanged real Snapshot. No added transform,
            // conditions, expected amount or caller geometry arithmetic.
            let evaluation = serde_json::json!({"schema_version":"0.2.0","artifact":{"kind":"evaluation_input","data":{
                "snapshot_id":snapshot.id,"revision":snapshot.revision,"context":snapshot.context,
                "result_space":{"id":"f02-fixture-local","kind":"local","units":"pt","origin":"top_left"},
                "transforms":[],"conditions":null}}});
            let query_bytes = serde_json::to_vec(&query).map_err(|_| HostError::InvalidInput)?;
            let evaluation_bytes =
                serde_json::to_vec(&evaluation).map_err(|_| HostError::InvalidInput)?;
            let parts = [
                snapshot_bytes.as_slice(),
                query_bytes.as_slice(),
                evaluation_bytes.as_slice(),
            ];
            let size = 40 + parts.iter().map(|p| p.len()).sum::<usize>();
            let mut input = host.reserve_input(session, size)?;
            worker_tape::encode(&parts, input.bytes_mut())?;
            caller_stage = "submit_measure";
            host.submit(
                session,
                OperationClass::Measure,
                input,
                OutputRequest {
                    channels: 1,
                    frame_bytes: FRAME,
                    total_bytes: FRAME,
                    input_format: 1,
                    retained_partition: 0,
                },
                Instant::now() + Duration::from_millis(1000),
            )?;
            caller_stage = "await_measure_completion";
            let HostEvent::Complete(measured) =
                next(&mut host, Instant::now() + Duration::from_secs(2))?
            else {
                return Err(HostError::WorkerFailed);
            };
            if measured.terminal != Terminal::Completed || measured.committed() != 1 {
                return Err(HostError::WorkerFailed);
            }
            let data = measured.bytes(0).ok_or(HostError::WorkerFailed)?;
            let artifact =
                AnalysisDocument::from_json(data, FRAME).map_err(|_| HostError::InvalidInput)?;
            let AnalysisArtifact::Measurement(case) = artifact.artifact else {
                return Err(HostError::InvalidInput);
            };
            if !matches!(case.result, MeasurementResult::Known { .. }) {
                return Err(HostError::InvalidInput);
            }
            let mut file =
                new_file(&output_dir.join("measurement.json")).map_err(|_| HostError::Io)?;
            measured
                .write_channel(0, &mut file)
                .map_err(|_| HostError::Io)?;
        }
        Ok(())
    })();
    // Every outcome follows explicit cleanup; assertions come afterwards.
    let stop = Instant::now() + Duration::from_secs(2);
    let mut cleanup_confirmed = false;
    while Instant::now() < stop {
        match host.shutdown() {
            Ok(HostEvent::ShutdownComplete) => {
                cleanup_confirmed = true;
                break;
            }
            Ok(HostEvent::CleanupPending { .. } | HostEvent::HelperCleanupPending { .. })
            | Err(_) => break,
            Ok(_) => thread::sleep(Duration::from_millis(1)),
        }
    }
    let usage = domain.usage();
    let cleanup_confirmed = cleanup_confirmed && usage.reserved_sessions == 0 && !usage.abandoned;
    let caller_ok = outcome.is_ok();
    let report = serde_json::json!({
        "caller_stage": caller_stage, "terminal": terminal, "committed": committed, "missing": missing,
        "attached_clock": clock_id, "cleanup_confirmed": cleanup_confirmed,
        "reserved_sessions": usage.reserved_sessions, "abandoned": usage.abandoned,
        "diagnostic": diagnostic, "caller_ok": caller_ok, "caller_error": outcome.err().map(|e| format!("{e:?}")),
        "scope": "owned_f02_a_ax_only", "reply_bytes_including_lf": FRAME
    });
    serde_json::to_writer(
        new_file(&output_dir.join("host-report.json")).expect("own report"),
        &report,
    )
    .expect("bounded fixed report");
    assert!(caller_ok, "caller operation failed; see bounded report");
    assert_eq!(
        terminal, "completed",
        "Native terminal result; see bounded report"
    );
    assert_eq!(committed, mask, "canonical Native ACK missing");
    assert!(
        cleanup_confirmed,
        "owned cleanup unconfirmed; no false release"
    );
}
