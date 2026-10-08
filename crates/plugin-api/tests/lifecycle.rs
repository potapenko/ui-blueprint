use std::{fs, path::PathBuf};
use uiblueprint_plugin_api::{ClockReading, Error, Limits, ObservationSession, Terminal, Ticket};
use uiblueprint_schema::{SchemaVersion, model::*};

fn fixture(name: &str) -> Document {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/golden")
        .join(name);
    Document::from_json(&fs::read(path).expect("authored fixture"), 1_048_576)
        .expect("valid fixture")
}
fn request() -> Request {
    let Artifact::Request(mut r) = fixture("ENV-REQUEST-VALID.json").artifact else {
        panic!("request fixture")
    };
    r.operation = Operation::Observe {
        channels: vec![Channel::ExternalSemantics, Channel::RenderedCapture],
    };
    *r
}
fn descriptor() -> SessionDescriptor {
    let Artifact::Session(s) = fixture("ENV-CAPABILITY-VALID.json").artifact else {
        panic!("session fixture")
    };
    *s
}
fn clock(t: u64) -> ClockReading {
    ClockReading {
        domain: Id("fixture-parent-monotonic".into()),
        milliseconds: t,
    }
}
fn session() -> ObservationSession {
    ObservationSession::attach(
        descriptor(),
        clock(0).domain,
        Limits {
            max_frame_bytes: 65_536,
            max_in_flight: 1,
            max_pending_encoded_bytes: 131_072,
        },
    )
    .expect("explicit test limits")
}
fn encode(artifact: Artifact) -> Vec<u8> {
    serde_json::to_vec(&Document {
        schema_version: SchemaVersion::CURRENT,
        artifact,
    })
    .expect("encode")
}
fn reply(ticket: &Ticket, failed: bool) -> Vec<u8> {
    let Artifact::Snapshot(snapshot) = fixture("ENV-SNAPSHOT-VALID.json").artifact else {
        panic!("snapshot fixture")
    };
    encode(Artifact::ChannelResponse(Box::new(ChannelResponse {
        request_id: ticket.request_id.clone(),
        session_id: ticket.session_id.clone(),
        dispatch_sequence: ticket.sequence,
        target: snapshot.context.target.clone(),
        channel: if failed {
            Channel::RenderedCapture
        } else {
            Channel::ExternalSemantics
        },
        result: if failed {
            ChannelResult::Failed(Issue {
                code: ErrorCode::PermissionRequired,
                scope_id: snapshot.context.scope_id.clone(),
                failed_step: Some(Id("capture".into())),
                recovery_class: Id("explicit_permission".into()),
            })
        } else {
            ChannelResult::Observed(snapshot)
        },
    })))
}

#[test]
fn completed_semantics_survives_capture_failure_without_whole_platform_failure() {
    let mut s = session();
    let ticket = s
        .begin(&encode(Artifact::Request(Box::new(request()))), &clock(100))
        .expect("begin");
    s.receive(&ticket, &reply(&ticket, false), &clock(120))
        .expect("AX complete");
    s.receive(&ticket, &reply(&ticket, true), &clock(130))
        .expect("capture failure is data");
    let completion = s
        .complete(&ticket, &clock(140))
        .expect("both channels replied");
    assert_eq!(completion.terminal, Terminal::Completed);
    assert!(completion.missing_channels.is_empty());
    assert!(matches!(
        completion.channels[&Channel::ExternalSemantics],
        ChannelResult::Observed(_)
    ));
    assert!(matches!(
        completion.channels[&Channel::RenderedCapture],
        ChannelResult::Failed(_)
    ));
    assert_eq!(s.pending_encoded_bytes(), 0);
}

#[test]
fn cancel_preserves_completed_channel_and_same_request_id_does_not_revive_old_ticket() {
    let mut s = session();
    let frame = encode(Artifact::Request(Box::new(request())));
    let old = s.begin(&frame, &clock(100)).expect("begin");
    s.receive(&old, &reply(&old, false), &clock(110))
        .expect("AX");
    let result = s.cancel(&old).expect("cancel");
    assert_eq!(result.terminal, Terminal::Cancelled);
    assert_eq!(result.channels.len(), 1);
    let fresh = s.begin(&frame, &clock(120)).expect("new explicit request");
    assert_ne!(old.sequence, fresh.sequence);
    assert_eq!(
        s.receive(&old, &reply(&old, true), &clock(130)),
        Err(Error::StaleTicket)
    );
    assert!(
        s.cancel(&fresh)
            .expect("cancel new request")
            .channels
            .is_empty()
    );
}

#[test]
fn deadline_and_detach_reject_late_success_without_discarding_completed_channels() {
    let mut s = session();
    let ticket = s
        .begin(&encode(Artifact::Request(Box::new(request()))), &clock(100))
        .expect("begin");
    s.receive(&ticket, &reply(&ticket, false), &clock(120))
        .expect("AX");
    assert_eq!(
        s.receive(&ticket, &reply(&ticket, true), &clock(350)),
        Err(Error::DeadlineExpired)
    );
    let result = s.expire(&ticket, &clock(350)).expect("expire");
    assert_eq!(result.terminal, Terminal::TimedOut);
    assert_eq!(result.channels.len(), 1);
    let ticket = s
        .begin(&encode(Artifact::Request(Box::new(request()))), &clock(400))
        .expect("next request");
    let drained = s.detach();
    assert_eq!(drained.len(), 1);
    assert_eq!(drained[0].terminal, Terminal::Detached);
    assert_eq!(
        s.receive(&ticket, &reply(&ticket, false), &clock(410)),
        Err(Error::Detached)
    );
    assert_eq!(
        s.begin(&encode(Artifact::Request(Box::new(request()))), &clock(420)),
        Err(Error::Detached)
    );
}

#[test]
fn routing_generation_scope_version_and_clock_are_checked_before_admission() {
    for mutation in 0..4 {
        let mut r = request();
        match mutation {
            0 => r.context.session_id = Id("other-session".into()),
            1 => r.context.target.generation = Id("g2".into()),
            2 => r.context.surfaces[0].generation = Id("w2".into()),
            _ => r.context.scope_id = Id("not-authorized".into()),
        }
        assert_eq!(
            session().begin(&encode(Artifact::Request(Box::new(r))), &clock(100)),
            Err(Error::OutsideSession)
        );
    }
    let mut d = descriptor();
    d.supported_versions.clear();
    assert!(matches!(
        ObservationSession::attach(
            d,
            clock(0).domain,
            Limits {
                max_frame_bytes: 100,
                max_in_flight: 1,
                max_pending_encoded_bytes: 200
            }
        ),
        Err(Error::IncompatibleVersion)
    ));
    let mut s = session();
    let frame = encode(Artifact::Request(Box::new(request())));
    assert_eq!(
        s.begin(
            &frame,
            &ClockReading {
                domain: Id("helper-clock".into()),
                milliseconds: 100
            }
        ),
        Err(Error::InvalidClock)
    );
    let ticket = s.begin(&frame, &clock(100)).expect("begin");
    assert_eq!(
        s.receive(&ticket, &reply(&ticket, false), &clock(99)),
        Err(Error::InvalidClock)
    );
}

#[test]
fn frame_admission_and_duplicate_reply_limits_leave_accepted_data_intact() {
    let mut s = session();
    let frame = encode(Artifact::Request(Box::new(request())));
    let ticket = s.begin(&frame, &clock(100)).expect("begin");
    assert_eq!(s.begin(&frame, &clock(101)), Err(Error::Busy));
    assert_eq!(
        s.receive(&ticket, &vec![b'x'; 65_537], &clock(105)),
        Err(Error::ResourceLimit)
    );
    assert_eq!(
        s.receive(
            &ticket,
            b"{\"untrusted\":\"PRIVATE_TEST_VALUE\"}",
            &clock(106)
        ),
        Err(Error::InvalidFrame)
    );
    s.receive(&ticket, &reply(&ticket, false), &clock(110))
        .expect("AX");
    assert_eq!(
        s.receive(&ticket, &reply(&ticket, false), &clock(120)),
        Err(Error::DuplicateChannel)
    );
    assert_eq!(s.cancel(&ticket).expect("cancel").channels.len(), 1);
}

#[test]
fn blocked_target_does_not_own_another_sessions_progress() {
    let mut first = session();
    let frame = encode(Artifact::Request(Box::new(request())));
    let waiting = first.begin(&frame, &clock(100)).expect("waiting target");
    let mut d = descriptor();
    d.session_id = Id("a2".into());
    d.target.id = Id("second-target".into());
    let mut second = ObservationSession::attach(
        d.clone(),
        clock(0).domain,
        Limits {
            max_frame_bytes: 65_536,
            max_in_flight: 1,
            max_pending_encoded_bytes: 131_072,
        },
    )
    .expect("independent owner");
    let mut r = request();
    r.context.session_id = d.session_id;
    r.context.target = d.target;
    let ticket = second
        .begin(&encode(Artifact::Request(Box::new(r))), &clock(100))
        .expect("second admits without first progress");
    assert_eq!(
        second.cancel(&ticket).expect("second terminates").terminal,
        Terminal::Cancelled
    );
    assert_eq!(
        first
            .expire(&waiting, &clock(350))
            .expect("first timeout")
            .terminal,
        Terminal::TimedOut
    );
}

#[test]
fn response_budget_counts_output_and_rejects_overflow_without_erasing_ax() {
    let mut r = request();
    let expected_ticket = Ticket {
        request_id: r.request_id.clone(),
        session_id: r.context.session_id.clone(),
        sequence: 1,
    };
    let ax = reply(&expected_ticket, false);
    let capture = reply(&expected_ticket, true);
    r.limits.max_output_bytes = (ax.len() + capture.len()) as u64;
    let frame = encode(Artifact::Request(Box::new(r.clone())));
    let mut s = session();
    let ticket = s.begin(&frame, &clock(100)).expect("begin");
    s.receive(&ticket, &ax, &clock(110)).expect("AX fits");
    s.receive(&ticket, &capture, &clock(120))
        .expect("inclusive output boundary");
    assert_eq!(
        s.complete(&ticket, &clock(130))
            .expect("complete")
            .channels
            .len(),
        2
    );

    r.limits.max_output_bytes -= 1;
    let mut s = session();
    let ticket = s
        .begin(&encode(Artifact::Request(Box::new(r))), &clock(100))
        .expect("begin");
    s.receive(&ticket, &ax, &clock(110)).expect("AX still fits");
    assert_eq!(
        s.receive(&ticket, &capture, &clock(120)),
        Err(Error::ResourceLimit)
    );
    assert_eq!(
        s.cancel(&ticket)
            .expect("preserve accepted channel")
            .channels
            .len(),
        1
    );
}

#[test]
fn reply_correlation_and_channel_permissions_cannot_be_forged() {
    let mut s = session();
    let ticket = s
        .begin(&encode(Artifact::Request(Box::new(request()))), &clock(100))
        .expect("begin");
    let frame = reply(&ticket, false);
    let mut d = Document::from_json(&frame, frame.len()).expect("valid reply");
    let Artifact::ChannelResponse(r) = &mut d.artifact else {
        panic!("reply")
    };
    r.request_id = Id("wrong-request".into());
    assert_eq!(
        s.receive(
            &ticket,
            &serde_json::to_vec(&d).expect("encode"),
            &clock(110)
        ),
        Err(Error::StaleTicket)
    );
    let mut d = Document::from_json(&frame, frame.len()).expect("valid reply");
    let Artifact::ChannelResponse(r) = &mut d.artifact else {
        panic!("reply")
    };
    r.channel = Channel::OptInLayoutProbe;
    r.result = ChannelResult::Failed(Issue {
        code: ErrorCode::Unsupported,
        scope_id: Id("form-1".into()),
        failed_step: None,
        recovery_class: Id("none".into()),
    });
    assert_eq!(
        s.receive(
            &ticket,
            &serde_json::to_vec(&d).expect("encode"),
            &clock(120)
        ),
        Err(Error::UnexpectedChannel)
    );
    assert!(
        s.cancel(&ticket)
            .expect("none admitted")
            .channels
            .is_empty()
    );
}

fn composed_setup() -> (Request, SessionDescriptor, Snapshot) {
    let Artifact::Snapshot(mut snapshot) = fixture("GRAPH-MANY-TO-MANY.json").artifact else {
        panic!("mixed graph")
    };
    snapshot.context.projection = Projection::Design;
    let mut r = request();
    r.context = snapshot.context.clone();
    r.operation = Operation::Observe {
        channels: vec![Channel::ExternalSemantics, Channel::OptInLayoutProbe],
    };
    let mut d = descriptor();
    d.capabilities.push(Capability {
        channel: Channel::OptInLayoutProbe,
        operation: Id("observe".into()),
        status: CapabilityStatus::Supported,
        reason: None,
    });
    (r, d, *snapshot)
}
fn composed_session(d: SessionDescriptor) -> ObservationSession {
    ObservationSession::attach(
        d,
        clock(0).domain,
        Limits {
            max_frame_bytes: 65_536,
            max_in_flight: 1,
            max_pending_encoded_bytes: 131_072,
        },
    )
    .unwrap()
}
fn composed_reply(ticket: &Ticket, s: Snapshot) -> Vec<u8> {
    encode(Artifact::ChannelResponse(Box::new(ChannelResponse {
        request_id: ticket.request_id.clone(),
        session_id: ticket.session_id.clone(),
        dispatch_sequence: ticket.sequence,
        target: s.context.target.clone(),
        channel: Channel::OptInLayoutProbe,
        result: ChannelResult::Observed(Box::new(s)),
    })))
}
#[test]
fn composed_sources_require_each_requested_channel_and_observe_capability() {
    for variant in 0..6 {
        let (mut r, mut d, snapshot) = composed_setup();
        match variant {
            0 => {
                r.operation = Operation::Observe {
                    channels: vec![Channel::OptInLayoutProbe],
                }
            }
            1 => d
                .capabilities
                .retain(|c| c.channel != Channel::ExternalSemantics),
            2 | 3 => {
                let c = d
                    .capabilities
                    .iter_mut()
                    .find(|c| c.channel == Channel::ExternalSemantics)
                    .unwrap();
                c.status = if variant == 2 {
                    CapabilityStatus::Unsupported
                } else {
                    CapabilityStatus::PermissionRequired
                };
                c.reason = Some(Id("explicit-refusal".into()));
            }
            4 => {
                d.capabilities
                    .iter_mut()
                    .find(|c| c.channel == Channel::ExternalSemantics)
                    .unwrap()
                    .operation = Id("inspect".into())
            }
            _ => {
                let c = d
                    .capabilities
                    .iter_mut()
                    .find(|c| c.channel == Channel::OptInLayoutProbe)
                    .unwrap();
                c.status = CapabilityStatus::Unsupported;
                c.reason = Some(Id("explicit-refusal".into()));
            }
        }
        let mut session = composed_session(d);
        let ticket = session
            .begin(&encode(Artifact::Request(Box::new(r))), &clock(1))
            .unwrap();
        let frame = composed_reply(&ticket, snapshot);
        Document::from_json(&frame, 65_536).unwrap(); // schema validity is not permission
        assert_eq!(
            session.receive(&ticket, &frame, &clock(2)),
            Err(if variant == 0 {
                Error::UnexpectedChannel
            } else {
                Error::InvalidChannel
            })
        );
        assert!(session.cancel(&ticket).unwrap().channels.is_empty());
    }
}
#[test]
fn accepted_composition_keeps_source_partial_and_never_completes_another_slot() {
    for partial_capability in [false, true] {
        let (r, mut d, mut snapshot) = composed_setup();
        if partial_capability {
            for c in &mut d.capabilities {
                if matches!(
                    c.channel,
                    Channel::ExternalSemantics | Channel::OptInLayoutProbe
                ) {
                    c.status = CapabilityStatus::Partial;
                    c.reason = Some(Id("limited".into()));
                }
            }
        }
        snapshot.coverage.status = CoverageStatus::Partial;
        snapshot.coverage.unknown_count = None;
        let original = snapshot.clone();
        let mut s = composed_session(d);
        let ticket = s
            .begin(&encode(Artifact::Request(Box::new(r))), &clock(1))
            .unwrap();
        s.receive(&ticket, &composed_reply(&ticket, snapshot), &clock(2))
            .unwrap();
        let result = s.cancel(&ticket).unwrap();
        assert_eq!(result.channels.len(), 1);
        assert_eq!(result.missing_channels, vec![Channel::ExternalSemantics]);
        assert_eq!(
            result.channels[&Channel::OptInLayoutProbe],
            ChannelResult::Observed(Box::new(original))
        );
    }
}
#[test]
fn composed_validation_retains_limits_freshness_and_earlier_channel_on_refusal() {
    for variant in 0..4 {
        let (mut r, d, mut snapshot) = composed_setup();
        match variant {
            0 => snapshot.observations[0].freshness = Freshness::Stale,
            1 => r.limits.max_elements = 1,
            2 => snapshot.context.environment_revision = Id("different-environment".into()),
            _ => r.limits.max_output_bytes = 1,
        }
        let mut s = composed_session(d);
        let ticket = s
            .begin(&encode(Artifact::Request(Box::new(r))), &clock(1))
            .unwrap();
        let result = s.receive(&ticket, &composed_reply(&ticket, snapshot), &clock(2));
        assert!(result.is_err(), "variant{variant}");
        assert!(s.cancel(&ticket).unwrap().channels.is_empty());
    }
    // A prior valid AX result remains available when a mixed probe is refused.
    let (r, d, mut snapshot) = composed_setup();
    let mut s = composed_session(d);
    let ticket = s
        .begin(&encode(Artifact::Request(Box::new(r))), &clock(1))
        .unwrap();
    let mut ax = snapshot.clone();
    for o in &mut ax.observations {
        o.channel = Channel::ExternalSemantics;
    }
    let ax_response = encode(Artifact::ChannelResponse(Box::new(ChannelResponse {
        request_id: ticket.request_id.clone(),
        session_id: ticket.session_id.clone(),
        dispatch_sequence: ticket.sequence,
        target: ax.context.target.clone(),
        channel: Channel::ExternalSemantics,
        result: ChannelResult::Observed(Box::new(ax.clone())),
    })));
    s.receive(&ticket, &ax_response, &clock(2)).unwrap();
    snapshot.observations[0].freshness = Freshness::Stale;
    assert!(
        s.receive(&ticket, &composed_reply(&ticket, snapshot), &clock(3))
            .is_err()
    );
    let remaining = s.cancel(&ticket).unwrap();
    assert_eq!(
        remaining.channels[&Channel::ExternalSemantics],
        ChannelResult::Observed(Box::new(ax))
    );
    assert_eq!(remaining.missing_channels, vec![Channel::OptInLayoutProbe]);
}
