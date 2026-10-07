//! Finite non-UI stand-in for the selected channel-specific Native producer.
//! It consumes actual private headers/Request, builds a small existing canonical
//! response only after real admission, and never calls a UI/SDK or writes pixels.
use std::{
    fs::File,
    io::{Read, Write},
    os::fd::FromRawFd,
};
use uiblueprint_host::{
    process::DarwinPlatform,
    process_api::{CHILD_INPUT_FD, CHILD_OUTPUT_FD, WorkerPlatform},
    *,
};
use uiblueprint_schema::{SchemaVersion, model::*};
fn control(input: &mut File) -> Control {
    let mut b = [0; CONTROL_BYTES];
    input.read_exact(&mut b).unwrap();
    Control::decode(&b).unwrap()
}
fn main() {
    DarwinPlatform::setup_main(8 * 1_048_576).unwrap();
    // SAFETY: the actual parent provider transfers these owned endpoints once.
    let (mut input, mut output) = unsafe {
        (
            File::from_raw_fd(CHILD_INPUT_FD),
            File::from_raw_fd(CHILD_OUTPUT_FD),
        )
    };
    let config = control(&mut input);
    assert_eq!(config.kind, ControlKind::Configure);
    assert_eq!(config.class, OperationClass::Observe);
    assert_eq!(
        config.value, 1,
        "real first observation Ticket, not parent operation2"
    );
    assert_eq!(config.length, 1);
    let mut mode = [0];
    input.read_exact(&mut mode).unwrap();
    let submit = control(&mut input);
    assert_eq!(submit.kind, ControlKind::Submit);
    assert_eq!(submit.correlation, config.correlation);
    assert_eq!(submit.value, config.value);
    assert_eq!(submit.slot, config.slot);
    assert!(submit.auxiliary > 0);
    let n = usize::try_from(submit.length).unwrap();
    assert!(n <= 65536);
    let mut bytes = vec![0; n];
    input.read_exact(&mut bytes).unwrap();
    let Artifact::Request(request) = Document::from_json(&bytes, 65536).unwrap().artifact else {
        panic!("canonical request")
    };
    if config.slot == 1 && mode[0] == b'X' {
        return;
    }
    if config.slot == 1 && mode[0] == b'D' {
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
    if config.slot == 2 && mode[0] == b'E' {
        return;
    }
    let channel = match config.slot {
        0 => Channel::ExternalSemantics,
        1 => Channel::RenderedCapture,
        2 => Channel::OptInLayoutProbe,
        _ => panic!("unsupported channel"),
    };
    let result = if matches!(mode[0], b'P' | b'T' | b'E' | b'C') {
        let Artifact::Finding(case) = Document::from_json(
            include_bytes!("../../../../fixtures/golden/GEO-SIZE-RATIO__width.json"),
            65536,
        )
        .unwrap()
        .artifact
        else {
            panic!("geometry fixture")
        };
        let mut snapshot = case.snapshot;
        snapshot.context = request.context.clone();
        snapshot.coverage.scope_id = request.context.scope_id.clone();
        for observation in &mut snapshot.observations {
            observation.channel = channel;
            observation.coverage.scope_id = request.context.scope_id.clone();
            if channel == Channel::OptInLayoutProbe {
                observation.answer_source = AnswerSource::Cache;
                observation.freshness = Freshness::Unverified;
                observation.freshness_basis = FreshnessBasis::Unverified;
                observation.last_verified = None;
            }
        }
        ChannelResult::Observed(Box::new(snapshot))
    } else if config.slot == 0 {
        let Artifact::Snapshot(mut snapshot) = Document::from_json(
            include_bytes!("../../../../fixtures/golden/ENV-SNAPSHOT-VALID.json"),
            65536,
        )
        .unwrap()
        .artifact
        else {
            panic!("snapshot")
        };
        snapshot.context = request.context.clone();
        if mode[0] == b'J' {
            snapshot.coverage.status = CoverageStatus::Partial;
            snapshot.coverage.omitted_count = None;
        }
        ChannelResult::Observed(snapshot)
    } else {
        ChannelResult::Failed(Issue {
            code: ErrorCode::PermissionRequired,
            scope_id: request.context.scope_id.clone(),
            failed_step: Some(Id("capture".into())),
            recovery_class: Id("explicit_permission".into()),
        })
    };
    let response = Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::ChannelResponse(Box::new(ChannelResponse {
            request_id: request.request_id.clone(),
            session_id: request.context.session_id.clone(),
            dispatch_sequence: if mode[0] == b'W' || (config.slot == 2 && mode[0] == b'T') {
                submit.correlation.operation
            } else {
                config.value
            },
            target: request.context.target.clone(),
            channel,
            result,
        })),
    };
    let bytes = serde_json::to_vec(&response).unwrap();
    // Mode B deliberately violates the private advertised reply cap to exercise
    // the parent's pre-parse byte bound, not an SDK acquisition-memory claim.
    if mode[0] != b'B' {
        assert!(bytes.len() < config.auxiliary as usize);
    }
    output.write_all(&bytes).unwrap();
    output.write_all(b"\n").unwrap();
}
