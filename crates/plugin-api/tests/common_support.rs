#[path = "../../../tests/bridges/common/session_support.rs"]
mod support;

use std::{fs, io::Cursor, path::PathBuf, thread, time::Duration};
use support::{FrameEvent, Harness};
use uiblueprint_plugin_api::{Error, Limits, Terminal};
use uiblueprint_schema::{SchemaVersion, model::*};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn fixture(name: &str) -> Document {
    support::read_document(&root().join("fixtures/golden").join(name), 1_048_576)
        .expect("canonical fixture")
}
fn harness(deadline: u64) -> Harness {
    let Artifact::Session(s) = fixture("ENV-CAPABILITY-VALID.json").artifact else {
        panic!("session")
    };
    let Artifact::Request(mut r) = fixture("ENV-REQUEST-VALID.json").artifact else {
        panic!("request")
    };
    r.limits.deadline_ms = deadline;
    r.operation = Operation::Observe {
        channels: vec![Channel::ExternalSemantics, Channel::RenderedCapture],
    };
    Harness::new(
        *s,
        *r,
        Limits {
            max_frame_bytes: 65_536,
            max_in_flight: 1,
            max_pending_encoded_bytes: 131_072,
        },
    )
    .expect("admission")
}
fn reply(h: &Harness, failed: bool) -> Vec<u8> {
    let Artifact::Snapshot(s) = fixture("ENV-SNAPSHOT-VALID.json").artifact else {
        panic!("snapshot")
    };
    let result = if failed {
        ChannelResult::Failed(Issue {
            code: ErrorCode::Timeout,
            scope_id: s.context.scope_id.clone(),
            failed_step: Some(Id("capture".into())),
            recovery_class: Id("reobserve".into()),
        })
    } else {
        ChannelResult::Observed(s.clone())
    };
    serde_json::to_vec(&Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::ChannelResponse(Box::new(ChannelResponse {
            request_id: h.ticket().request_id.clone(),
            session_id: h.ticket().session_id.clone(),
            dispatch_sequence: h.ticket().sequence,
            target: s.context.target.clone(),
            channel: if failed {
                Channel::RenderedCapture
            } else {
                Channel::ExternalSemantics
            },
            result,
        })),
    })
    .expect("encode")
}

#[test]
fn real_clock_ticket_and_retained_documents_use_the_existing_core() {
    let mut h = harness(2_000);
    assert_eq!(h.ticket().sequence, 1);
    assert_eq!(h.target().id.0, "native-app");
    assert_eq!(h.clock_domain().0, "fixture-parent-monotonic");
    let before = h.elapsed_ms().expect("clock");
    thread::sleep(Duration::from_millis(2));
    assert!(h.elapsed_ms().expect("clock") >= before);
    h.receive(&reply(&h, false)).expect("AX");
    h.receive(&reply(&h, true))
        .expect("capture timeout response");
    let completion = h.complete().expect("all channels");
    assert_eq!(completion.terminal, Terminal::Completed);
    let documents = h.retained_documents(completion);
    assert_eq!(documents.len(), 2);
    for document in documents {
        document.validate().expect("canonical retained response");
    }
}

#[test]
fn cancel_expire_and_detach_reject_late_frames_and_preserve_completed_ax() {
    for mode in 0..3 {
        let mut h = harness(if mode == 1 { 50 } else { 2_000 });
        h.receive(&reply(&h, false)).expect("AX before terminal");
        let late = reply(&h, true);
        let completion = match mode {
            0 => h.cancel().expect("cancel"),
            1 => {
                thread::sleep(h.remaining().expect("deadline"));
                h.expire().expect("real deadline")
            }
            _ => h.detach().pop().expect("detached completion"),
        };
        assert_eq!(completion.channels.len(), 1);
        assert!(matches!(
            h.receive(&late),
            Err(Error::StaleTicket | Error::Detached | Error::DeadlineExpired)
        ));
    }
}

#[test]
fn framing_has_byte_and_count_bounds_and_does_not_parse_commands() {
    let mut input = Cursor::new(b"123\nnext\n".to_vec());
    assert_eq!(
        support::read_frame(&mut input, 4),
        FrameEvent::Frame(b"123\n".to_vec())
    );
    assert_eq!(support::read_frame(&mut input, 4), FrameEvent::Oversize);
    let (rx, handle) = support::pump_frames(Cursor::new(b"one\ntwo\n".to_vec()), 8, 1);
    assert_eq!(
        rx.recv_timeout(Duration::from_secs(1)).expect("frame"),
        FrameEvent::Frame(b"one\n".to_vec())
    );
    assert_eq!(
        rx.recv_timeout(Duration::from_secs(1))
            .expect("count bound"),
        FrameEvent::FrameLimitReached
    );
    handle.join().expect("owned reader exit");
    let path = root().join("fixtures/golden/ENV-REQUEST-VALID.json");
    assert!(support::read_document(&path, 1).is_err());
    assert!(!fs::read(path).expect("fixture").is_empty());
}
