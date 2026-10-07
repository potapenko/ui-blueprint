use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};
use uiblueprint_schema::{SchemaVersion, model::*};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Case {
    directory: PathBuf,
    snapshot: Snapshot,
    expectation: Expectation,
}
impl Case {
    fn new(name: &str) -> Self {
        let path = format!(
            "{}/../../fixtures/golden/{name}.json",
            env!("CARGO_MANIFEST_DIR")
        );
        let document: Document =
            serde_json::from_slice(&fs::read(path).expect("fixture")).expect("canonical fixture");
        let Artifact::Finding(case) = document.artifact else {
            panic!("finding fixture")
        };
        let directory = std::env::temp_dir().join(format!(
            "uiblueprint-l01-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&directory).expect("unique task temporary directory");
        let result = Self {
            directory,
            snapshot: case.snapshot,
            expectation: case.expectation,
        };
        result.save();
        result
    }
    fn save(&self) {
        let document = |artifact| Document {
            schema_version: SchemaVersion::CURRENT,
            artifact,
        };
        fs::write(
            self.directory.join("snapshot.json"),
            serde_json::to_vec(&document(Artifact::Snapshot(Box::new(
                self.snapshot.clone(),
            ))))
            .expect("serialize snapshot"),
        )
        .expect("save snapshot");
        fs::write(
            self.directory.join("expectation.json"),
            serde_json::to_vec(&document(Artifact::Expectation(Box::new(
                self.expectation.clone(),
            ))))
            .expect("serialize expectation"),
        )
        .expect("save expectation");
    }
    fn run(&self, command: &str, json: bool, input: usize, output: usize) -> Output {
        let Rule::Geometry { anchors, .. } = &self.expectation.rule else {
            panic!("geometry fixture")
        };
        self.run_at(
            command,
            json,
            input,
            output,
            &anchors[0].coordinate_space.id.0,
        )
    }
    fn run_at(
        &self,
        command: &str,
        json: bool,
        input: usize,
        output: usize,
        space: &str,
    ) -> Output {
        let mut child = Command::new(env!("CARGO_BIN_EXE_uiblueprint"));
        child
            .arg(command)
            .arg("--snapshot")
            .arg(self.directory.join("snapshot.json"))
            .arg("--expectation")
            .arg(self.directory.join("expectation.json"))
            .args([
                "--space",
                space,
                "--max-input-bytes",
                &input.to_string(),
                "--max-output-bytes",
                &output.to_string(),
            ]);
        if json {
            child.arg("--json");
        }
        child.output().expect("invoke CLI")
    }
    fn inspect(
        &self,
        reference: &str,
        view: &str,
        input: usize,
        output: usize,
        extra: &[&str],
    ) -> Output {
        Command::new(env!("CARGO_BIN_EXE_uiblueprint"))
            .arg("inspect")
            .arg("--snapshot")
            .arg(self.directory.join("snapshot.json"))
            .args([
                "--ref",
                reference,
                "--view",
                view,
                "--max-input-bytes",
                &input.to_string(),
                "--max-output-bytes",
                &output.to_string(),
            ])
            .args(extra)
            .output()
            .expect("inspect CLI")
    }
    fn diff(&self, entries: &str, input: usize, output: usize, json: bool) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_uiblueprint"));
        command
            .arg("diff")
            .arg("--before")
            .arg(self.directory.join("snapshot.json"))
            .arg("--after")
            .arg(self.directory.join("after.json"))
            .args([
                "--max-input-bytes",
                &input.to_string(),
                "--max-output-bytes",
                &output.to_string(),
                "--max-entries",
                entries,
            ]);
        if json {
            command.arg("--json");
        }
        command.output().expect("diff CLI")
    }
}
impl Drop for Case {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.directory).expect("remove only this case's temporary directory");
        assert!(
            !self.directory.exists(),
            "verify task temporary directory removal"
        );
    }
}
fn assert_error(output: Output, exit: i32, code: &str) {
    assert_eq!(output.status.code(), Some(exit));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, format!("{code}\n").as_bytes());
}

#[test]
fn diff_cli_preserves_originals_and_distinguishes_content_evidence_and_absence() {
    let mut case = Case::new("GEO-SIZE-RATIO__width");
    let mut second = case.snapshot.nodes[0].clone();
    second.key.key = Id("recorded-B".into());
    case.snapshot.nodes.push(second);
    let Property::Requested {
        state: Availability::Known {
            value: Value::Geometry(g),
        },
        ..
    } = &mut case.snapshot.nodes[0].properties[0]
    else {
        panic!("geometry")
    };
    let Shape::Rect(rect) = &mut g.shape else {
        panic!("rect")
    };
    rect.width = 32.0;
    case.save();
    let mut after = case.snapshot.clone();
    after.nodes.pop();
    after.coverage.status = CoverageStatus::Partial;
    after.coverage.omitted_count = None;
    let Property::Requested {
        state: Availability::Known {
            value: Value::Geometry(g),
        },
        ..
    } = &mut after.nodes[0].properties[0]
    else {
        panic!("geometry")
    };
    let Shape::Rect(rect) = &mut g.shape else {
        panic!("rect")
    };
    rect.width = 48.0;
    let save_after = |snapshot: &Snapshot| {
        fs::write(
            case.directory.join("after.json"),
            serde_json::to_vec(&Document {
                schema_version: SchemaVersion::CURRENT,
                artifact: Artifact::Snapshot(Box::new(snapshot.clone())),
            })
            .unwrap(),
        )
        .unwrap()
    };
    save_after(&after);
    let before_bytes = fs::read(case.directory.join("snapshot.json")).unwrap();
    let after_bytes = fs::read(case.directory.join("after.json")).unwrap();
    let output = case.diff("10", 65536, 65536, true);
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value.as_object().unwrap().len(), 9);
    assert_eq!(value["output_version"], "1.0.0");
    assert_eq!(value["kind"], "recorded_difference");
    assert_eq!(value["source"], "saved");
    assert_eq!(value["live_revalidated"], false);
    assert_eq!(value["comparison_scope"], "node_presence_and_properties");
    assert_eq!(
        value["before"],
        serde_json::to_value(&case.snapshot).unwrap()
    );
    assert_eq!(value["after"], serde_json::to_value(&after).unwrap());
    assert_eq!(value["omitted_entries"], 0);
    assert_eq!(value["entries"].as_array().unwrap().len(), 2);
    let property = &value["entries"][0];
    assert_eq!(property["kind"], "property");
    assert_eq!(property["field"], "layout_bounds");
    assert_eq!(property["content_changed"], true);
    assert_eq!(property["evidence_changed"], false);
    assert_eq!(property["before_present"], true);
    assert_eq!(property["after_present"], true);
    let absence = &value["entries"][1];
    assert_eq!(absence["kind"], "node_presence");
    assert!(absence["field"].is_null());
    assert_eq!(absence["key"]["key"], "recorded-B");
    assert_eq!(absence["before_present"], true);
    assert_eq!(absence["after_present"], false);
    for entry in value["entries"].as_array().unwrap() {
        assert_eq!(entry.as_object().unwrap().len(), 7);
        assert!(entry.get("deleted").is_none() && entry.get("removed").is_none());
    }
    let compact = case.diff("10", 65536, 65536, false);
    assert_eq!(compact.status.code(), Some(0));
    let compact = String::from_utf8(compact.stdout).unwrap();
    assert!(compact.contains("comparison=saved_observations"));
    assert!(compact.contains("width: 32.0") && compact.contains("width: 48.0"));
    assert!(compact.contains("presence=BeforeOnly"));
    assert!(compact.contains("status: Partial"));
    assert_eq!(
        fs::read(case.directory.join("snapshot.json")).unwrap(),
        before_bytes
    );
    assert_eq!(
        fs::read(case.directory.join("after.json")).unwrap(),
        after_bytes
    );
    let response = Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::ChannelResponse(Box::new(ChannelResponse {
            request_id: Id("after-request".into()),
            session_id: after.context.session_id.clone(),
            dispatch_sequence: 1,
            target: after.context.target.clone(),
            channel: Channel::ExternalSemantics,
            result: ChannelResult::Observed(Box::new(after.clone())),
        })),
    };
    fs::write(
        case.directory.join("after.json"),
        serde_json::to_vec(&response).unwrap(),
    )
    .unwrap();
    let channel = case.diff("10", 65536, 65536, true);
    assert_eq!(channel.stdout, output.stdout);
    let mut evidence = case.snapshot.clone();
    evidence.observations[0].end += 1.0;
    save_after(&evidence);
    let output = case.diff("10", 65536, 65536, true);
    assert_eq!(output.status.code(), Some(0));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(
        value["entries"]
            .as_array()
            .unwrap()
            .iter()
            .all(|e| e["content_changed"] == false && e["evidence_changed"] == true)
    );
    let mut unavailable = case.snapshot.clone();
    let Property::Requested { state, .. } = &mut unavailable.nodes[0].properties[0] else {
        panic!("property")
    };
    *state = Availability::Unknown {
        reason: Id("missing_current_measurement".into()),
    };
    save_after(&unavailable);
    let output = case.diff("10", 65536, 65536, true);
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["entries"][0]["content_changed"], true);
    assert_eq!(
        value["after"]["nodes"][0]["properties"][0]["state"]["availability"],
        "unknown"
    );
    assert!(
        value["after"]["nodes"][0]["properties"][0]["state"]
            .get("value")
            .is_none()
    );
}

#[test]
fn diff_cli_entry_and_byte_limits_context_and_invalid_input_are_explicit() {
    let case = Case::new("GEO-GAP");
    let mut after = case.snapshot.clone();
    after.observations[0].end += 1.0;
    let save = |s: &Snapshot| {
        fs::write(
            case.directory.join("after.json"),
            serde_json::to_vec(&Document {
                schema_version: SchemaVersion::CURRENT,
                artifact: Artifact::Snapshot(Box::new(s.clone())),
            })
            .unwrap(),
        )
        .unwrap()
    };
    save(&after);
    let complete = case.diff("100", 65536, 65536, true);
    assert_eq!(complete.status.code(), Some(0));
    let full: serde_json::Value = serde_json::from_slice(&complete.stdout).unwrap();
    let count = full["entries"].as_array().unwrap().len();
    assert!(count > 1);
    for (cap, returned) in [("0", 0), ("1", 1)] {
        let output = case.diff(cap, 65536, 65536, true);
        assert_eq!(output.status.code(), Some(4));
        assert!(output.stderr.is_empty());
        let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(value["entries"].as_array().unwrap().len(), returned);
        assert_eq!(value["omitted_entries"], count - returned);
        assert_eq!(value["after"], serde_json::to_value(&after).unwrap());
    }
    let exact = case.diff("100", 65536, complete.stdout.len(), true);
    assert_eq!(exact.stdout, complete.stdout);
    assert_eq!(exact.status.code(), Some(0));
    assert_error(
        case.diff("100", 65536, complete.stdout.len() - 1, true),
        2,
        "output_limit",
    );
    let input = fs::metadata(case.directory.join("snapshot.json"))
        .unwrap()
        .len() as usize
        + fs::metadata(case.directory.join("after.json"))
            .unwrap()
            .len() as usize;
    assert_eq!(case.diff("100", input, 65536, true).status.code(), Some(0));
    assert_error(case.diff("100", input - 1, 65536, true), 2, "input_limit");
    for invalid in ["-1", "x", ""] {
        assert_error(
            case.diff(invalid, 65536, 65536, true),
            2,
            "invalid_arguments",
        );
    }
    after.context.environment_revision = Id("another_environment".into());
    save(&after);
    let environment = case.diff("100", 65536, 65536, true);
    assert_eq!(environment.status.code(), Some(0));
    let environment: serde_json::Value = serde_json::from_slice(&environment.stdout).unwrap();
    assert_eq!(
        environment["before"],
        serde_json::to_value(&case.snapshot).unwrap()
    );
    assert_eq!(environment["after"], serde_json::to_value(&after).unwrap());
    after.context.target.generation = Id("other-target-generation".into());
    save(&after);
    assert_error(case.diff("100", 65536, 65536, true), 4, "context_mismatch");
    fs::write(case.directory.join("after.json"), b"PRIVATE_DIFF_CANARY").unwrap();
    assert_error(case.diff("100", 65536, 65536, true), 2, "invalid_input");
}

#[cfg(all(target_os = "macos", feature = "macos"))]
#[test]
fn observe_cli_uses_real_guarded_peer_and_preserves_failed_partial_and_timed_out_channels() {
    use serde_json::json;
    let case = Case::new("GEO-GAP");
    let directory = PathBuf::from(env!("CARGO_BIN_EXE_uiblueprint"))
        .parent()
        .unwrap()
        .to_path_buf();
    let worker = directory.join("session-worker");
    let helper = directory.join("examples/native_peer");
    assert!(
        worker.is_file() && helper.is_file(),
        "build existing host worker/native_peer before live caller tests"
    );
    let descriptor: serde_json::Value = serde_json::from_slice(include_bytes!(
        "../../../fixtures/golden/ENV-CAPABILITY-VALID.json"
    ))
    .unwrap();
    let base_request: serde_json::Value = serde_json::from_slice(include_bytes!(
        "../../../fixtures/golden/ENV-REQUEST-VALID.json"
    ))
    .unwrap();
    let base_connection = json!({"connection_version":"1.0.0", "target":descriptor["artifact"]["data"]["target"],
        "session":descriptor["artifact"]["data"],"attach_deadline_ms":2000,
        "host_limits":{"workers":1,"worker_bytes":67108864,"publication_reserve":1048576,"bootstrap_bytes":1048576,
            "parent_bytes":33554432,"input_bytes":2097152,"ingress_bytes":524288,"output_bytes":524288,
            "request_output_bytes":2097152,"completion_groups":2,"control_bytes":4096,"cleanup_ms":1000,
            "retained_domain_bytes":67108864,"retained_per_worker":15728640,"main_stack_bytes":8388608,"watchdog_stack_bytes":1048576},
        "provider":{"backend":"native_fixture","helper_executable":helper,"configuration":"F","channels":3}});
    let connection_file = case.directory.join("connection.json");
    let request_file = case.directory.join("request.json");
    let run = |worker: &std::path::Path, cap: usize| {
        Command::new(env!("CARGO_BIN_EXE_uiblueprint"))
            .arg("observe")
            .arg("--connection")
            .arg(&connection_file)
            .arg("--request")
            .arg(&request_file)
            .arg("--worker")
            .arg(worker)
            .args([
                "--max-input-bytes",
                "65536",
                "--max-output-bytes",
                &cap.to_string(),
            ])
            .output()
            .unwrap()
    };
    for (mode, channels, exit, lines) in [
        ("F", 1, 0, 1),
        ("F", 3, 4, 2),
        ("J", 1, 4, 1),
        ("X", 3, 1, 1),
        ("D", 3, 4, 1),
    ] {
        let mut connection = base_connection.clone();
        connection["provider"]["configuration"] = json!(mode);
        let mut request = base_request.clone();
        request["artifact"]["data"]["limits"]["deadline_ms"] =
            json!(if mode == "D" { 500 } else { 2000 });
        request["artifact"]["data"]["limits"]["max_output_bytes"] = json!(65536);
        request["artifact"]["data"]["operation"]["channels"] = if channels == 1 {
            json!(["external_semantics"])
        } else {
            json!(["external_semantics", "rendered_capture"])
        };
        fs::write(&connection_file, serde_json::to_vec(&connection).unwrap()).unwrap();
        fs::write(&request_file, serde_json::to_vec(&request).unwrap()).unwrap();
        let before_request = fs::read(&request_file).unwrap();
        let output = run(&worker, 65536);
        assert_eq!(
            output.status.code(),
            Some(exit),
            "{mode}: {:?}",
            output.stderr
        );
        assert!(output.stdout.ends_with(b"\n"));
        let records: Vec<_> = output
            .stdout
            .split(|b| *b == b'\n')
            .filter(|b| !b.is_empty())
            .collect();
        assert_eq!(records.len(), lines, "{mode}");
        for (i, bytes) in records.iter().enumerate() {
            let doc = Document::from_json(bytes, 65536).unwrap();
            let Artifact::ChannelResponse(response) = doc.artifact else {
                panic!("canonical response")
            };
            assert_eq!(response.dispatch_sequence, 1);
            assert_eq!(
                response.channel,
                if i == 0 {
                    Channel::ExternalSemantics
                } else {
                    Channel::RenderedCapture
                }
            );
            assert_eq!(matches!(response.result, ChannelResult::Failed(_)), i == 1);
            // Exact payloads are the existing peer's canonical source records,
            // not a CLI wrapper or reparsed/rewritten graph.
            assert_eq!(
                serde_json::to_vec(&Document {
                    schema_version: doc.schema_version,
                    artifact: Artifact::ChannelResponse(response)
                })
                .unwrap(),
                *bytes
            );
        }
        assert_eq!(
            fs::read(&request_file).unwrap(),
            before_request,
            "clock bound only in owned request copy"
        );
        if exit == 0 {
            assert!(output.stderr.is_empty());
        }
    }
    fs::write(
        &connection_file,
        serde_json::to_vec(&base_connection).unwrap(),
    )
    .unwrap();
    assert_error(run(&worker, 1), 2, "output_limit");
    assert_error(
        run(&case.directory.join("missing-worker"), 65536),
        1,
        "io_error",
    );
    for mutation in ["version", "identity", "unknown", "array"] {
        let mut bad = base_connection.clone();
        match mutation {
            "version" => bad["connection_version"] = json!("9"),
            "identity" => bad["target"]["generation"] = json!("wrong"),
            "unknown" => bad["private_unknown"] = json!("CANARY_CONNECTION"),
            _ => bad["host_limits"] = json!([1, 2, 3]),
        }
        fs::write(&connection_file, serde_json::to_vec(&bad).unwrap()).unwrap();
        let output = run(&case.directory.join("must-not-spawn"), 65536);
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert!(
            !String::from_utf8(output.stderr)
                .unwrap()
                .contains("CANARY_CONNECTION")
        );
    }
    let duplicate = serde_json::to_string(&base_connection).unwrap().replacen(
        "{",
        "{\"connection_version\":\"1.0.0\",",
        1,
    );
    fs::write(&connection_file, duplicate).unwrap();
    assert_error(run(&worker, 65536), 2, "invalid_connection");
    let mut unsupported = base_connection.clone();
    unsupported["provider"] = json!({"backend":"other"});
    fs::write(&connection_file, serde_json::to_vec(&unsupported).unwrap()).unwrap();
    assert_error(run(&worker, 65536), 5, "unsupported_observe_backend");
    #[cfg(not(feature = "web"))]
    {
        unsupported["provider"] = json!({"backend":"web","setup":{},"selection":{}});
        fs::write(&connection_file, serde_json::to_vec(&unsupported).unwrap()).unwrap();
        assert_error(run(&worker, 65536), 5, "unsupported_observe_backend");
    }
}

#[test]
fn inspect_keeps_exact_identity_availability_evidence_and_escaped_text() {
    let mut case = Case::new("GEO-GAP");
    let Artifact::Snapshot(snapshot) = Document::from_json(
        include_bytes!("../../../fixtures/golden/ENV-SNAPSHOT-VALID.json"),
        65536,
    )
    .unwrap()
    .artifact
    else {
        panic!("snapshot")
    };
    case.snapshot = *snapshot;
    let Property::Requested {
        evidence, state, ..
    } = &mut case.snapshot.nodes[0].properties[1]
    else {
        panic!("name")
    };
    *state = Availability::Known {
        value: Value::Text("Director\nFORGED_LINE".into()),
    };
    let evidence = evidence.clone();
    for (field, sensitivity, state) in [
        (
            Field::Placeholder,
            Sensitivity::Public,
            Availability::Known {
                value: Value::Text(String::new()),
            },
        ),
        (
            Field::Focused,
            Sensitivity::Public,
            Availability::Known {
                value: Value::Flag(false),
            },
        ),
        (
            Field::Value,
            Sensitivity::Sensitive,
            Availability::Redacted {},
        ),
        (
            Field::Description,
            Sensitivity::Public,
            Availability::Unknown {
                reason: Id("not_observed".into()),
            },
        ),
        (
            Field::AccessibilityName,
            Sensitivity::Public,
            Availability::Unsupported {
                reason: Id("not_exposed".into()),
            },
        ),
    ] {
        case.snapshot.context.fields.push(field);
        case.snapshot.coverage.fields.push(field);
        case.snapshot.observations[0].coverage.fields.push(field);
        case.snapshot.nodes[0].properties.push(Property::Requested {
            field,
            sensitivity,
            evidence: evidence.clone(),
            state,
        });
    }
    let mut duplicate_label = case.snapshot.nodes[0].clone();
    duplicate_label.key.key = Id("check-2".into());
    case.snapshot.nodes.push(duplicate_label);
    case.snapshot.relations.push(Relation {
        kind: RelationKind::LabelledBy,
        from: case.snapshot.nodes[0].key.clone(),
        to: case.snapshot.nodes[1].key.clone(),
        evidence: evidence.clone(),
    });
    case.snapshot.coverage.status = CoverageStatus::Partial;
    case.save();
    let input_before = fs::read(case.directory.join("snapshot.json")).unwrap();
    let reference = serde_json::to_string(&case.snapshot.nodes[0].key).unwrap();
    let output = case.inspect(&reference, "interaction", 65536, 65536, &[]);
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let text = String::from_utf8(output.stdout).unwrap();
    for required in [
        "inspection=saved_observation live_revalidation=not_performed",
        "requested_view=Interaction original_projection=Interaction",
        "snapshot=\"S10\" revision=10",
        "status: Partial",
        "clock_domain: Id(\"fixture-parent-monotonic\")",
        "node=SourceKey { namespace: Id(\"macos.ax\"), key: Id(\"check-1\") }",
        "property=Name selection=requested sensitivity=Public availability=known value=Text(\"Director\\nFORGED_LINE\")",
        "property=Placeholder selection=requested sensitivity=Public availability=known value=Text(\"\")",
        "property=Focused selection=requested sensitivity=Public availability=known value=Flag(false)",
        "property=Enabled selection=requested sensitivity=Public availability=known value=Flag(true)",
        "property=Checked selection=requested sensitivity=Public availability=known value=Flag(false)",
        "property=Value selection=requested sensitivity=Sensitive availability=redacted",
        "property=Description selection=requested sensitivity=Public availability=unknown",
        "property=AccessibilityName selection=requested sensitivity=Public availability=unsupported",
        "property=LayoutBounds selection=not_requested",
        "method: Id(\"authored_fixture\")",
        "relation_direction=Outgoing relation=Relation { kind: LabelledBy",
    ] {
        assert!(text.contains(required), "missing {required}: {text}");
    }
    assert!(!text.contains("\nFORGED_LINE"));
    let json = case.inspect(&reference, "interaction", 65536, 65536, &["--json"]);
    assert_eq!(json.status.code(), Some(0));
    let parsed: serde_json::Value = serde_json::from_slice(&json.stdout).unwrap();
    assert_eq!(
        parsed["snapshot"],
        serde_json::to_value(&case.snapshot).unwrap()
    );
    assert_eq!(
        parsed["selector"],
        serde_json::to_value(&case.snapshot.nodes[0].key).unwrap()
    );
    assert_eq!(
        fs::read(case.directory.join("snapshot.json")).unwrap(),
        input_before
    );
    assert_error(
        case.inspect("Director", "interaction", 65536, 65536, &[]),
        2,
        "invalid_input",
    );
    let other = serde_json::to_string(&case.snapshot.nodes[1].key).unwrap();
    let output = case.inspect(&other, "interaction", 65536, 65536, &[]);
    assert_eq!(output.status.code(), Some(0));
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("node=SourceKey { namespace: Id(\"macos.ax\"), key: Id(\"check-2\") }")
    );
}

#[test]
fn inspect_design_keeps_geometry_kinds_and_accepts_original_observed_channel() {
    let mut case = Case::new("GEO-GAP");
    case.snapshot
        .context
        .fields
        .push(Field::AccessibilityBounds);
    case.snapshot
        .coverage
        .fields
        .push(Field::AccessibilityBounds);
    for observation in &mut case.snapshot.observations {
        observation.coverage.fields.push(Field::AccessibilityBounds);
    }
    for node in &mut case.snapshot.nodes {
        let mut property = node.properties[0].clone();
        let Property::Requested {
            field,
            state: Availability::Known {
                value: Value::Geometry(g),
            },
            ..
        } = &mut property
        else {
            panic!("geometry")
        };
        *field = Field::AccessibilityBounds;
        g.frame_kind = FrameKind::AccessibilityBounds;
        let Shape::Rect(rect) = &mut g.shape else {
            panic!("rect")
        };
        rect.width = 123.0;
        node.properties.push(property);
    }
    case.save();
    let reference = serde_json::to_string(&case.snapshot.nodes[0].key).unwrap();
    let design = case.inspect(&reference, "design", 65536, 65536, &[]);
    assert_eq!(design.status.code(), Some(0));
    let text = String::from_utf8(design.stdout).unwrap();
    assert!(text.contains("requested_view=Design original_projection=Interaction"));
    assert!(text.contains("frame_kind: LayoutBounds"));
    assert!(text.contains("frame_kind: AccessibilityBounds"));
    assert!(text.contains("width: 123.0"));
    assert!(text.contains("units: CssPx"));
    assert!(text.find("property=LayoutBounds ").unwrap() < text.find("property=Role ").unwrap());
    let response = Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::ChannelResponse(Box::new(ChannelResponse {
            request_id: Id("inspect-fixture".into()),
            session_id: case.snapshot.context.session_id.clone(),
            dispatch_sequence: 1,
            target: case.snapshot.context.target.clone(),
            channel: Channel::ExternalSemantics,
            result: ChannelResult::Observed(Box::new(case.snapshot.clone())),
        })),
    };
    let bytes = serde_json::to_vec(&response).unwrap();
    fs::write(case.directory.join("snapshot.json"), &bytes).unwrap();
    let output = case.inspect(&reference, "design", 65536, 65536, &[]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), text);
    let output = case.inspect(&reference, "design", 65536, 65536, &["--json"]);
    assert_eq!(output.status.code(), Some(0));
    let parsed: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        parsed["snapshot"],
        serde_json::to_value(&case.snapshot).unwrap()
    );
    assert_eq!(parsed["requested_view"], "design");
    assert_eq!(
        fs::read(case.directory.join("snapshot.json")).unwrap(),
        bytes
    );
}

#[test]
fn inspect_json_envelope_keeps_uncertainty_and_exact_output_boundary() {
    let mut case = Case::new("GEO-GAP");
    case.snapshot.observations[0].consistency = Consistency::Unknown;
    case.snapshot.observations[0].consistency_reason = Some(Id("not_atomic".into()));
    case.save();
    let reference = serde_json::to_string(&case.snapshot.nodes[0].key).unwrap();
    for view in ["interaction", "design"] {
        let output = case.inspect(&reference, view, 65536, 65536, &["--json"]);
        assert_eq!(output.status.code(), Some(0));
        assert!(output.stderr.is_empty());
        assert!(output.stdout.ends_with(b"\n"));
        let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(result.as_object().unwrap().len(), 7);
        assert_eq!(result["output_version"], "1.0.0");
        assert_eq!(result["kind"], "inspection");
        assert_eq!(result["source"], "saved");
        assert_eq!(result["live_revalidated"], false);
        assert_eq!(
            result["selector"],
            serde_json::to_value(&case.snapshot.nodes[0].key).unwrap()
        );
        assert_eq!(result["requested_view"], view);
        assert_eq!(
            result["snapshot"],
            serde_json::to_value(&case.snapshot).unwrap()
        );
        assert_eq!(result["snapshot"]["context"]["schema_version"], "0.1.0");
        let exact = case.inspect(&reference, view, 65536, output.stdout.len(), &["--json"]);
        assert_eq!(exact.status.code(), Some(0));
        assert_eq!(exact.stdout, output.stdout);
        assert_error(
            case.inspect(
                &reference,
                view,
                65536,
                output.stdout.len() - 1,
                &["--json"],
            ),
            2,
            "output_limit",
        );
    }
    assert_error(
        case.inspect(
            r#"{"namespace":"absent","key":"A"}"#,
            "interaction",
            65536,
            65536,
            &["--json"],
        ),
        4,
        "target_unresolved",
    );
    assert_error(
        case.inspect(
            "PRIVATE_JSON_CANARY",
            "interaction",
            65536,
            65536,
            &["--json"],
        ),
        2,
        "invalid_input",
    );
}

#[test]
fn inspect_refusals_and_aggregate_bounds_never_publish_partial_or_private_input() {
    let case = Case::new("GEO-GAP");
    let reference = serde_json::to_string(&case.snapshot.nodes[0].key).unwrap();
    for invalid in [
        "PRIVATE_INSPECT_CANARY",
        r#"["ns","key"]"#,
        r#"{"namespace":"n","key":"a","key":"b"}"#,
        r#"{"namespace":"n","key":"a","extra":"PRIVATE_INSPECT_CANARY"}"#,
        r#"{"namespace":"n","key":""}"#,
    ] {
        assert_error(
            case.inspect(invalid, "interaction", 65536, 65536, &[]),
            2,
            "invalid_input",
        );
    }
    assert_error(
        case.inspect(
            r#"{"namespace":"absent","key":"A"}"#,
            "interaction",
            65536,
            65536,
            &[],
        ),
        4,
        "target_unresolved",
    );
    assert_error(
        case.inspect(&reference, "interaction", 65536, 1, &[]),
        2,
        "output_limit",
    );
    let needed = fs::metadata(case.directory.join("snapshot.json"))
        .unwrap()
        .len() as usize
        + reference.len();
    assert_error(
        case.inspect(&reference, "interaction", needed - 1, 65536, &[]),
        2,
        "input_limit",
    );
    assert_eq!(
        case.inspect(&reference, "interaction", needed, 65536, &[])
            .status
            .code(),
        Some(0)
    );
    assert_error(
        case.inspect(
            &reference,
            "interaction",
            65536,
            65536,
            &["--json", "--json"],
        ),
        2,
        "invalid_arguments",
    );
    assert_error(
        case.inspect(&reference, "other", 65536, 65536, &[]),
        5,
        "unsupported_view",
    );
    assert_error(
        case.inspect(
            &reference,
            "interaction",
            65536,
            65536,
            &["--ref", &reference],
        ),
        2,
        "invalid_arguments",
    );
    fs::write(
        case.directory.join("snapshot.json"),
        b"PRIVATE_INSPECT_CANARY",
    )
    .unwrap();
    assert_error(
        case.inspect(&reference, "interaction", 65536, 65536, &[]),
        2,
        "invalid_input",
    );
    let expectation = fs::read(case.directory.join("expectation.json")).unwrap();
    fs::write(case.directory.join("snapshot.json"), expectation).unwrap();
    assert_error(
        case.inspect(&reference, "interaction", 65536, 65536, &[]),
        2,
        "invalid_input",
    );
}

#[test]
fn independent_geo_checks_emit_canonical_full_evidence_and_distinct_exits() {
    for (name, exit, status, amount) in [
        ("GEO-GAP", 0, CheckStatus::Pass, Some(8.0)),
        ("GEO-GAP-FAIL", 3, CheckStatus::Fail, Some(8.0)),
        ("GEO-BASELINE-UNKNOWN", 4, CheckStatus::Unknown, None),
        ("GEO-FRAME-KIND", 4, CheckStatus::Unknown, None),
    ] {
        let case = Case::new(name);
        let output = case.run("check", true, 100_000, 100_000);
        assert_eq!(
            output.status.code(),
            Some(exit),
            "{name}: {:?}",
            output.stderr
        );
        assert!(output.stderr.is_empty());
        let document = Document::from_json(&output.stdout, 100_000).expect("canonical output");
        let Artifact::Finding(result) = document.artifact else {
            panic!("finding output")
        };
        assert_eq!(result.snapshot, case.snapshot);
        assert_eq!(result.expectation, case.expectation);
        assert_eq!(result.finding.status, status);
        let measured = result.finding.measured.as_ref().map(|v| match v {
            Value::Quantity { amount, .. } => *amount,
            _ => panic!("quantity"),
        });
        assert_eq!(measured, amount);
    }
}
#[test]
fn compact_is_default_saved_analysis_with_source_and_limits() {
    let case = Case::new("GEO-GAP");
    let output = case.run("check", false, 100_000, 100_000);
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let text = String::from_utf8(output.stdout).expect("utf8");
    for expected in [
        "analysis=saved_snapshot",
        "check=Pass",
        "snapshot=Id(\"S11\")",
        "coverage=",
        "selected_fields=",
        "input_bytes=100000",
        "observation=\"OG1\"",
        "freshness=Current",
        "expected_from=\"fixture_literal\"",
        "amount: 8.0",
        "evidence=",
    ] {
        assert!(text.contains(expected), "missing {expected}");
    }
}
#[test]
fn measure_returns_fact_without_inventing_success_for_failed_expectation() {
    let case = Case::new("GEO-GAP-FAIL");
    let output = case.run("measure", false, 100_000, 100_000);
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let text = String::from_utf8(output.stdout).expect("utf8");
    assert!(text.contains("measurement=known"));
    assert!(text.contains("amount: 8.0"));
    assert!(!text.contains("check="));
    let unknown = Case::new("GEO-BASELINE-UNKNOWN").run("measure", false, 100_000, 100_000);
    assert_eq!(unknown.status.code(), Some(4));
    assert!(
        String::from_utf8(unknown.stdout)
            .expect("utf8")
            .contains("measurement=unknown")
    );
}
#[test]
fn measurement_json_uses_the_registered_canonical_analysis_record() {
    let output = Case::new("GEO-GAP").run("measure", true, 100_000, 100_000);
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let document =
        uiblueprint_schema::analysis::AnalysisDocument::from_json(&output.stdout, 100_000)
            .expect("analysis result");
    assert!(matches!(
        document.artifact,
        uiblueprint_schema::analysis::AnalysisArtifact::Measurement(_)
    ));
}
#[test]
fn input_limit_is_aggregate_and_output_limit_has_no_partial_json() {
    let case = Case::new("GEO-GAP");
    let total = (fs::metadata(case.directory.join("snapshot.json"))
        .expect("size")
        .len()
        + fs::metadata(case.directory.join("expectation.json"))
            .expect("size")
            .len()) as usize;
    assert_error(
        case.run("check", true, total - 1, 100_000),
        2,
        "input_limit",
    );
    let exact = case.run("check", true, total, 100_000);
    assert_eq!(exact.status.code(), Some(0));
    let size = exact.stdout.len();
    assert_error(case.run("check", true, total, size - 1), 2, "output_limit");
    assert_eq!(case.run("check", true, total, size).stdout, exact.stdout);
    assert_error(case.run("check", false, total, 5), 2, "output_limit");
}
#[test]
fn invalid_documents_and_io_errors_never_echo_private_payloads_or_paths() {
    let case = Case::new("GEO-GAP");
    for bytes in [
        b"PRIVATE_CANARY_L01_not_json".as_slice(),
        b"{\"schema_version\":\"9.0.0\",\"private\":\"PRIVATE_CANARY_L01\"}".as_slice(),
    ] {
        fs::write(case.directory.join("snapshot.json"), bytes).expect("test input");
        assert_error(
            case.run("check", true, 100_000, 100_000),
            2,
            "invalid_input",
        );
    }
    fs::remove_file(case.directory.join("snapshot.json")).expect("remove own file");
    assert_error(case.run("check", true, 100_000, 100_000), 1, "io_error");
}
#[test]
fn compact_escapes_untrusted_control_text() {
    let mut case = Case::new("GEO-GAP");
    case.expectation.expected_from = Id("fixture\n\u{1b}[31mPRIVATE_CANARY_L01".into());
    case.save();
    let output = case.run("check", false, 100_000, 100_000);
    assert_eq!(output.status.code(), Some(0));
    assert!(!output.stdout.contains(&0x1b));
    let text = String::from_utf8(output.stdout).expect("utf8");
    assert!(text.contains("fixture\\n\\u{1b}"));
}
#[test]
fn applicability_is_unknown_without_new_observation_or_inference() {
    let mut case = Case::new("GEO-GAP");
    case.expectation.applies_when.platform = Some(Id("web".into()));
    case.save();
    let output = case.run("check", true, 100_000, 100_000);
    assert_eq!(output.status.code(), Some(4));
    let document = Document::from_json(&output.stdout, 100_000).expect("unknown canonical result");
    let Artifact::Finding(result) = document.artifact else {
        panic!("finding")
    };
    assert_eq!(
        result.finding.reason,
        Some(Id("applicability_unknown".into()))
    );
}
#[test]
fn finite_command_and_argument_contract() {
    for (args, exit, code) in [
        (
            vec!["observe", "PRIVATE_CANARY_L01"],
            2,
            "invalid_arguments",
        ),
        (vec!["check"], 2, "invalid_arguments"),
        (
            vec!["check", "--max-input-bytes", "0"],
            2,
            "invalid_arguments",
        ),
        (vec!["check", "--json", "--json"], 2, "invalid_arguments"),
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_uiblueprint"))
            .args(args)
            .output()
            .expect("invoke");
        assert_error(output, exit, code);
    }
    let output = Command::new(env!("CARGO_BIN_EXE_uiblueprint"))
        .arg("--help")
        .output()
        .expect("help");
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    assert!(output.stdout.starts_with(b"UI Blueprint"));
}

#[test]
fn converted_json_does_not_drop_selected_result_space() {
    let mut case = Case::new("GEO-SIZE-RATIO__width");
    let Property::Requested {
        evidence,
        state: Availability::Known {
            value: Value::Geometry(geometry),
        },
        ..
    } = &mut case.snapshot.nodes[0].properties[0]
    else {
        panic!("geometry")
    };
    let mut to = geometry.coordinate_space.clone();
    to.id = Id("converted".into());
    geometry.transform = TransformState::Known {
        transform: Box::new(Transform {
            from: geometry.coordinate_space.clone(),
            to,
            affine: [2.0, 0.0, 0.0, 2.0, 0.0, 0.0],
            target: case.snapshot.context.target.clone(),
            surface: case.snapshot.context.surfaces[0].clone(),
            environment_revision: case.snapshot.context.environment_revision.clone(),
            evidence: evidence.clone(),
        }),
    };
    case.save();
    assert_error(
        case.run_at("check", true, 100_000, 100_000, "converted"),
        5,
        "unsupported_result_version",
    );
    let compact = case.run_at("measure", false, 100_000, 100_000, "converted");
    assert_eq!(compact.status.code(), Some(0));
    let text = String::from_utf8(compact.stdout).expect("utf8");
    assert!(text.contains("amount: 60.0"));
    assert!(text.contains("converted"));
}

#[test]
fn unknown_space_and_sensitive_canonical_input_fail_without_payload_output() {
    let mut case = Case::new("GEO-GAP");
    assert_error(
        case.run_at("check", true, 100_000, 100_000, "missing-space"),
        2,
        "unknown_space",
    );
    case.snapshot.nodes[0]
        .source_declarations
        .push(SourceDeclaration {
            namespace: Id("fixture.private".into()),
            name: Id("secret".into()),
            state: Availability::Known {
                value: Value::Text("PRIVATE_CANARY_L01_SENSITIVE".into()),
            },
            sensitivity: Sensitivity::Sensitive,
            source: Id("fixture".into()),
        });
    case.save();
    assert_error(
        case.run("check", true, 100_000, 100_000),
        2,
        "invalid_input",
    );
}
