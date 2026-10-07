#![cfg(all(target_os = "macos", feature = "web"))]
//! Actual public caller/worker against bounded synthetic CDP, never live UI.
#[path = "support/web_worker_data.rs"]
mod data;
#[path = "support/web_worker_peer.rs"]
mod peer;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::Ordering,
};
use uiblueprint_schema::model::*;
struct Files(PathBuf);
impl Drop for Files {
    fn drop(&mut self) {
        for name in ["connection.json", "request.json", "source.json"] {
            let p = self.0.join(name);
            if p.exists() {
                fs::remove_file(p).unwrap();
            }
        }
        fs::remove_dir(&self.0).unwrap();
        assert!(!self.0.exists());
    }
}
fn save(files: &Files, name: &str, document: &Document) {
    document.validate().unwrap();
    fs::write(files.0.join(name), serde_json::to_vec(document).unwrap()).unwrap();
}
fn run(files: &Files, worker: &Path, command: &str, json: bool, limit: usize) -> Output {
    let cli = worker.parent().unwrap().join("uiblueprint");
    assert!(
        cli.is_file(),
        "build CLI web in the same target before this test"
    );
    let mut cmd = Command::new(cli);
    if command == "observe" {
        cmd.arg(command);
    } else {
        cmd.args(["action", command]);
        cmd.arg(if command == "prepare" {
            "--snapshot"
        } else {
            "--plan"
        })
        .arg(files.0.join("source.json"));
    }
    cmd.arg("--connection")
        .arg(files.0.join("connection.json"))
        .arg("--request")
        .arg(files.0.join("request.json"))
        .arg("--worker")
        .arg(worker)
        .args([
            "--max-input-bytes",
            "131072",
            "--max-output-bytes",
            &limit.to_string(),
        ]);
    if json && command != "observe" {
        cmd.arg("--json");
    }
    cmd.output().unwrap()
}
fn connection(peer: &peer::Peer) -> serde_json::Value {
    let descriptor = data::descriptor();
    let Artifact::Session(session) = descriptor.artifact else {
        panic!("session")
    };
    let l = data::limits();
    serde_json::json!({"connection_version":"1.0.0","target":data::target(),"session":session,"attach_deadline_ms":2000,
    "host_limits":{"workers":l.workers,"worker_bytes":l.worker_bytes,"publication_reserve":l.publication_reserve,"bootstrap_bytes":l.bootstrap_bytes,"parent_bytes":l.parent_bytes,"input_bytes":l.input_bytes,"ingress_bytes":l.ingress_bytes,"output_bytes":l.output_bytes,"request_output_bytes":l.request_output_bytes,"completion_groups":l.completion_groups,"control_bytes":l.control_bytes,"cleanup_ms":l.cleanup_ms,"retained_domain_bytes":l.retained_domain_bytes,"retained_per_worker":l.retained_per_worker,"main_stack_bytes":l.main_stack_bytes,"watchdog_stack_bytes":l.watchdog_stack_bytes},
    "provider":{"backend":"web","setup":data::setup(peer.url.clone()),"selection":data::selection(false)}})
}
fn action_seed(snapshot: &Snapshot, clock: &str) -> Document {
    let node = &snapshot.nodes[0];
    let observation = &snapshot.observations[0];
    let evidence = Evidence {
        observation_id: observation.id.clone(),
        source_namespace: node.key.namespace.clone(),
        provenance: Provenance::Reported,
        method: Id("unresolved-preparation".into()),
        uncertainty: None,
    };
    let action = Action {
        id: Id("set-checkbox".into()),
        context: snapshot.context.clone(),
        backend_ref: BackendRef {
            session_id: snapshot.context.session_id.clone(),
            key: node.key.clone(),
            snapshot_id: snapshot.id.clone(),
            observation_id: observation.id.clone(),
            target: snapshot.context.target.clone(),
            surface: node.surface.clone(),
        },
        intent: Intent::SetChecked { value: true },
        modality: InputModality::Setter,
        input_space: None,
        required_enabled: true,
        authorized_scope: snapshot.context.scope_id.clone(),
        unique_match: false,
        resolution: Resolution {
            evidence,
            writable: Availability::Unknown {
                reason: Id("not_observed".into()),
            },
            value_allowed: Availability::Unknown {
                reason: Id("not_observed".into()),
            },
            available_intents: vec![],
        },
    };
    Document {
        schema_version: uiblueprint_schema::SchemaVersion::CURRENT,
        artifact: Artifact::Request(Box::new(Request {
            clock_domain: Id(clock.into()),
            request_id: Id("prepare-checkbox".into()),
            context: snapshot.context.clone(),
            limits: Limits {
                max_elements: 32,
                max_depth: 8,
                max_output_bytes: 65536,
                deadline_ms: 2000,
            },
            freshness_policy: FreshnessPolicy::CurrentRequired,
            operation: Operation::Prepare { action },
        })),
    }
}

#[test]
fn public_action_cli_keeps_delivery_verification_exits_and_original_evidence_separate() {
    let worker = Path::new(env!("CARGO_BIN_EXE_session-worker"));
    for mode in 0..8 {
        let peer = peer::Peer::new();
        peer.state.checkbox_native.store(true, Ordering::Release);
        peer.state.checkbox_enabled.store(true, Ordering::Release);
        peer.state.checkbox_writable.store(true, Ordering::Release);
        let files = Files(
            std::env::temp_dir().join(format!("uib-action-cli-{}-{mode}", std::process::id())),
        );
        fs::create_dir(&files.0).unwrap();
        fs::write(
            files.0.join("connection.json"),
            serde_json::to_vec(&connection(&peer)).unwrap(),
        )
        .unwrap();
        let mut observed = data::request("placeholder", vec![Channel::ExternalSemantics]);
        let Artifact::Request(request) = &mut observed.artifact else {
            panic!("request")
        };
        request.context.fields = vec![Field::Enabled, Field::Checked];
        save(&files, "request.json", &observed);
        let original = run(&files, worker, "observe", false, 65536);
        assert!(
            matches!(original.status.code(), Some(0 | 4)),
            "{}",
            String::from_utf8_lossy(&original.stderr)
        );
        let document = Document::from_json(&original.stdout, 65536).unwrap();
        let Artifact::ChannelResponse(response) = &document.artifact else {
            panic!("channel")
        };
        let ChannelResult::Observed(snapshot) = &response.result else {
            panic!("observed")
        };
        let seed = action_seed(snapshot, "saved-request-clock");
        save(&files, "request.json", &seed);
        fs::write(files.0.join("source.json"), &original.stdout).unwrap();
        if mode == 4 {
            peer.state.checkbox_native.store(false, Ordering::Release);
        }
        let prepared = run(&files, worker, "prepare", true, 65536);
        if mode == 4 {
            assert_eq!(
                prepared.status.code(),
                Some(4),
                "valid unsupported fresh preparation"
            );
            let d = Document::from_json(&prepared.stdout, 65536).unwrap();
            let Artifact::Error(issue) = d.artifact else {
                panic!("canonical provider issue")
            };
            assert_eq!(
                issue.code,
                ErrorCode::Unsupported,
                "provider reason unchanged"
            );
            assert_eq!(peer.state.setter_calls.load(Ordering::Acquire), 0);
            continue;
        }
        assert_eq!(
            prepared.status.code(),
            Some(0),
            "{}",
            String::from_utf8_lossy(&prepared.stderr)
        );
        assert_eq!(
            fs::read(files.0.join("source.json")).unwrap(),
            original.stdout
        );
        let prepared = Document::from_json(&prepared.stdout, 65536).unwrap();
        let Artifact::Action(case) = &prepared.artifact else {
            panic!("prepared")
        };
        let mut act = seed;
        let Artifact::Request(request) = &mut act.artifact else {
            panic!("request")
        };
        request.context = case.snapshot.context.clone();
        request.operation = Operation::Act {
            action: case.action.clone(),
        };
        save(&files, "request.json", &act);
        save(&files, "source.json", &prepared);
        if mode == 1 {
            peer.state.wrong_post_checked.store(true, Ordering::Release);
        }
        if mode == 2 {
            peer.state.lost_post_binding.store(true, Ordering::Release);
        }
        if mode == 3 {
            peer.state.checkbox_native.store(false, Ordering::Release);
        }
        if mode == 5 {
            fs::write(files.0.join("source.json"), b"{}").unwrap();
        }
        if mode == 7 {
            peer.state.stale_node.store(true, Ordering::Release);
        }
        let result = run(&files, worker, "execute", mode != 6, 65536);
        let expected = match mode {
            0 | 6 => 0,
            1 => 3,
            2 | 3 | 7 => 4,
            5 => 2,
            _ => unreachable!(),
        };
        assert_eq!(
            result.status.code(),
            Some(expected),
            "mode {mode}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            peer.state.setter_calls.load(Ordering::Acquire),
            usize::from(!matches!(mode, 3 | 5 | 7))
        );
        if mode == 6 {
            assert_eq!(
                String::from_utf8(result.stdout).unwrap(),
                "preparation=not_requested delivery=confirmed verification=pass completeness=complete protocol=completed cleanup=complete\n"
            );
        } else {
            let d = Document::from_json(&result.stdout, 65536).unwrap();
            if mode == 5 {
                assert!(matches!(d.artifact, Artifact::Error(_)));
            } else {
                let Artifact::TransitionContext(report) = d.artifact else {
                    panic!("transition")
                };
                assert_eq!(
                    report.transition.steps[0].outcome,
                    match mode {
                        0 => Outcome::Succeeded,
                        1 | 3 | 7 => Outcome::Failed,
                        2 => Outcome::ActionOutcomeUnknown,
                        _ => unreachable!(),
                    }
                );
            }
        }
    }
}

#[test]
fn public_prepare_compact_without_committed_frame_leaves_stdout_empty() {
    let worker = Path::new(env!("CARGO_BIN_EXE_session-worker"));
    let peer = peer::Peer::new();
    peer.state.checkbox_native.store(true, Ordering::Release);
    peer.state.checkbox_enabled.store(true, Ordering::Release);
    peer.state.checkbox_writable.store(true, Ordering::Release);
    let files = Files(
        std::env::temp_dir().join(format!("uib-action-cli-no-commit-{}", std::process::id())),
    );
    fs::create_dir(&files.0).unwrap();
    fs::write(
        files.0.join("connection.json"),
        serde_json::to_vec(&connection(&peer)).unwrap(),
    )
    .unwrap();
    let mut observed = data::request("placeholder", vec![Channel::ExternalSemantics]);
    let Artifact::Request(request) = &mut observed.artifact else {
        panic!("request")
    };
    request.context.fields = vec![Field::Enabled, Field::Checked];
    save(&files, "request.json", &observed);
    let original = run(&files, worker, "observe", false, 65536);
    assert!(matches!(original.status.code(), Some(0 | 4)));
    let document = Document::from_json(&original.stdout, 65536).unwrap();
    let Artifact::ChannelResponse(response) = &document.artifact else {
        panic!("channel")
    };
    let ChannelResult::Observed(snapshot) = &response.result else {
        panic!("observed")
    };
    let mut seed = action_seed(snapshot, "saved-request-clock");
    let Artifact::Request(request) = &mut seed.artifact else {
        panic!("request")
    };
    // Enough public compact budget, but canonical publication cannot fit. The
    // valid request reaches the provider and returns ResourceLimit without ACK.
    request.limits.max_output_bytes = 64;
    save(&files, "request.json", &seed);
    fs::write(files.0.join("source.json"), &original.stdout).unwrap();
    let calls = peer.state.calls.load(Ordering::Acquire);
    let result = run(&files, worker, "prepare", false, 65536);
    assert_eq!(
        result.status.code(),
        Some(2),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        result.stdout.is_empty(),
        "no ACKed result means no compact stdout"
    );
    assert!(
        peer.state.calls.load(Ordering::Acquire) > calls,
        "actual fresh provider path reached"
    );
    assert_eq!(peer.state.setter_calls.load(Ordering::Acquire), 0);
    assert_eq!(
        fs::read(files.0.join("source.json")).unwrap(),
        original.stdout
    );
}
