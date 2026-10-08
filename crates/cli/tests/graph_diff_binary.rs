use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};
use uiblueprint_schema::{SchemaVersion, model::*};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Case {
    dir: PathBuf,
    before: Snapshot,
    after: Snapshot,
}
fn evidence(s: &Snapshot) -> Evidence {
    let Property::Requested { evidence, .. } = &s.nodes[0].properties[0] else {
        panic!()
    };
    evidence.clone()
}
impl Case {
    fn new() -> Self {
        let doc = Document::from_json(
            include_bytes!("../../../fixtures/golden/GEO-SIZE-RATIO__width.json"),
            65536,
        )
        .unwrap();
        let Artifact::Finding(case) = doc.artifact else {
            panic!()
        };
        let mut before = case.snapshot;
        for key in ["B", "C", "D"] {
            let mut n = before.nodes[0].clone();
            n.key.key = Id(key.into());
            before.nodes.push(n);
        }
        before.nodes[0].children = vec![before.nodes[1].key.clone(), before.nodes[2].key.clone()];
        before.relations = vec![Relation {
            kind: RelationKind::Controls,
            from: before.nodes[0].key.clone(),
            to: before.nodes[1].key.clone(),
            evidence: evidence(&before),
        }];
        before.components = vec![ComponentMapping {
            logical_component_key: Id("row".into()),
            members: vec![before.nodes[0].key.clone(), before.nodes[1].key.clone()],
            declaration_source: Id("fixture".into()),
            provenance: Provenance::Reported,
        }];
        before.focus.keyboard = FocusRef::Known {
            target: before.nodes[0].key.clone(),
            evidence: evidence(&before),
        };
        before.focus.accessibility = FocusRef::None {
            evidence: evidence(&before),
        };
        before.focus.active_descendant = FocusRef::Unknown {
            reason: Id("not_observed".into()),
        };
        before.focus.text_selection = Some(TextSelection {
            anchor: 1,
            focus: 2,
            units: Id("utf16".into()),
            evidence: evidence(&before),
        });
        before.focus.composition_state = Property::Requested {
            field: Field::Name,
            sensitivity: Sensitivity::Public,
            evidence: evidence(&before),
            state: Availability::Known {
                value: Value::Text("".into()),
            },
        };
        let mut after = before.clone();
        let Property::Requested { state, .. } = &mut after.nodes[0].properties[0] else {
            panic!()
        };
        *state = Availability::Unknown {
            reason: Id("unobserved".into()),
        };
        after.nodes.pop();
        after.nodes[0].children.reverse();
        after.nodes[0].native_role = Availability::Known {
            value: Value::Text("AXGroup\n\u{1b}[31m".into()),
        };
        after.nodes[0].source_declarations.push(SourceDeclaration {
            namespace: Id("fixture".into()),
            name: Id("private".into()),
            sensitivity: Sensitivity::Sensitive,
            state: Availability::Redacted {},
            source: Id("app".into()),
        });
        after.relations[0].evidence.method = Id("second-read".into());
        after.components[0].members.push(after.nodes[2].key.clone());
        after.focus.keyboard = FocusRef::None {
            evidence: evidence(&after),
        };
        after.focus.accessibility = FocusRef::Known {
            target: after.nodes[1].key.clone(),
            evidence: evidence(&after),
        };
        after.focus.active_descendant = FocusRef::NotRequested {};
        after.focus.text_selection = None;
        after.focus.composition_state = Property::Requested {
            field: Field::Name,
            sensitivity: Sensitivity::Sensitive,
            evidence: evidence(&after),
            state: Availability::Redacted {},
        };
        after.coverage.status = CoverageStatus::Partial;
        after.coverage.omitted_count = None;
        after.context.environment_revision = Id("after-env".into());
        let dir = std::env::temp_dir().join(format!(
            "uib-g13-cli-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&dir).unwrap();
        let c = Self { dir, before, after };
        c.save();
        c
    }
    fn save(&self) {
        for (name, s) in [("before", &self.before), ("after", &self.after)] {
            fs::write(
                self.dir.join(name),
                serde_json::to_vec(&Document {
                    schema_version: SchemaVersion::CURRENT,
                    artifact: Artifact::Snapshot(Box::new(s.clone())),
                })
                .unwrap(),
            )
            .unwrap();
        }
    }
    fn command(&self, cap: usize, input: usize, output: usize, json: bool) -> Command {
        let mut c = Command::new(env!("CARGO_BIN_EXE_uiblueprint"));
        c.args(["diff", "--graph", "--before"])
            .arg(self.dir.join("before"))
            .arg("--after")
            .arg(self.dir.join("after"))
            .args([
                "--max-entries",
                &cap.to_string(),
                "--max-input-bytes",
                &input.to_string(),
                "--max-output-bytes",
                &output.to_string(),
            ]);
        if json {
            c.arg("--json");
        }
        c
    }
    fn run(&self) -> Output {
        self.command(100, 200000, 200000, true).output().unwrap()
    }
}
impl Drop for Case {
    fn drop(&mut self) {
        // Only these two run-owned non-image files ever exist here.
        for name in ["before", "after"] {
            fs::remove_file(self.dir.join(name)).unwrap();
        }
        fs::remove_dir(&self.dir).unwrap();
        assert!(!self.dir.exists());
    }
}
fn json(output: &Output) -> serde_json::Value {
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    serde_json::from_slice(&output.stdout).unwrap()
}
fn error(o: Output, exit: i32, code: &str) {
    assert_eq!(o.status.code(), Some(exit));
    assert!(o.stdout.is_empty());
    assert_eq!(String::from_utf8(o.stderr).unwrap(), format!("{code}\n"));
}
#[test]
fn public_graph_json_and_compact_preserve_all_originals_and_literal_changes() {
    let c = Case::new();
    let before_bytes = fs::read(c.dir.join("before")).unwrap();
    let after_bytes = fs::read(c.dir.join("after")).unwrap();
    let o = c.run();
    let v = json(&o);
    assert_eq!(v.as_object().unwrap().len(), 9);
    assert_eq!(v["kind"], "graph_difference");
    assert_eq!(v["output_version"], "1.0.0");
    assert_eq!(v["source"], "saved");
    assert_eq!(v["live_revalidated"], false);
    assert_eq!(
        v["comparison_scope"],
        "nodes_properties_children_metadata_relations_components_focus"
    );
    assert_eq!(v["before"], serde_json::to_value(&c.before).unwrap());
    assert_eq!(v["after"], serde_json::to_value(&c.after).unwrap());
    assert_eq!(v["after"]["coverage"]["status"], "partial");
    assert!(v["after"]["coverage"]["omitted_count"].is_null());
    let entries = v["entries"].as_array().unwrap();
    assert_eq!(
        entries
            .iter()
            .map(|e| e["kind"].as_str().unwrap())
            .collect::<Vec<_>>(),
        vec![
            "property",
            "node_presence",
            "children",
            "node_metadata",
            "relation",
            "component",
            "focus_keyboard",
            "focus_accessibility",
            "focus_active_descendant",
            "focus_text_selection",
            "focus_composition"
        ]
    );
    for e in entries {
        assert_eq!(e.as_object().unwrap().len(), 8);
    }
    assert_eq!(entries[0]["field"], "layout_bounds");
    assert_eq!(entries[1]["before_index"], 3);
    assert!(entries[1]["after_index"].is_null());
    assert_eq!(entries[1]["after_present"], false);
    assert_eq!(entries[4]["content_changed"], false);
    assert_eq!(entries[4]["evidence_changed"], true);
    assert!(entries[9]["before_index"].is_null());
    assert_eq!(entries[9]["before_present"], true);
    assert_eq!(entries[9]["after_present"], false);
    assert_eq!(v["omitted_entries"], 0);
    let compact = c.command(100, 200000, 200000, false).output().unwrap();
    assert_eq!(compact.status.code(), Some(0));
    let text = String::from_utf8(compact.stdout).unwrap();
    assert!(text.contains("deletion_claim=not_made"));
    assert!(text.contains("omitted_count: None"));
    assert!(text.contains("content_changed=false evidence_changed=true"));
    assert!(text.contains("Redacted"));
    assert!(text.contains("AXGroup\\n\\u{1b}[31m"));
    assert!(!text.contains('\u{1b}'));
    assert_eq!(fs::read(c.dir.join("before")).unwrap(), before_bytes);
    assert_eq!(fs::read(c.dir.join("after")).unwrap(), after_bytes);
}
#[test]
fn graph_caps_byte_boundaries_conflicts_and_sanitized_rejections() {
    let mut c = Case::new();
    let full = c.run();
    json(&full);
    let bytes = (fs::metadata(c.dir.join("before")).unwrap().len()
        + fs::metadata(c.dir.join("after")).unwrap().len()) as usize;
    for cap in [0, 1, 10] {
        let o = c.command(cap, bytes, 200000, true).output().unwrap();
        assert_eq!(o.status.code(), Some(4));
        assert!(o.stderr.is_empty());
        let v: serde_json::Value = serde_json::from_slice(&o.stdout).unwrap();
        assert_eq!(v["entries"].as_array().unwrap().len(), cap);
        assert_eq!(v["omitted_entries"], 11 - cap);
    }
    assert_eq!(
        c.command(100, bytes, full.stdout.len(), true)
            .output()
            .unwrap()
            .stdout,
        full.stdout
    );
    error(
        c.command(100, bytes - 1, 200000, true).output().unwrap(),
        2,
        "input_limit",
    );
    error(
        c.command(100, bytes, full.stdout.len() - 1, true)
            .output()
            .unwrap(),
        2,
        "output_limit",
    );
    for flags in [
        vec!["--graph"],
        vec!["--geometry"],
        vec!["--space", "local-form"],
        vec!["--before-evaluation", "private-path"],
        vec!["--ref", "{}"],
    ] {
        error(
            c.command(100, 200000, 200000, true)
                .args(flags)
                .output()
                .unwrap(),
            2,
            "invalid_arguments",
        );
    }
    c.after.context.target.generation = Id("g2".into());
    c.save();
    error(c.run(), 4, "context_mismatch");
    c.after = c.before.clone();
    c.after.nodes[0]
        .source_declarations
        .push(SourceDeclaration {
            namespace: Id("fixture".into()),
            name: Id("secret".into()),
            state: Availability::Known {
                value: Value::Text("SECRET-CANARY-G13".into()),
            },
            sensitivity: Sensitivity::Sensitive,
            source: Id("app".into()),
        });
    c.save();
    error(c.run(), 2, "invalid_input");
    fs::write(c.dir.join("after"), b"SECRET-CANARY-G13 invalid-json").unwrap();
    error(c.run(), 2, "invalid_input");
}
#[test]
fn observed_input_and_unchanged_graph_are_supported() {
    let mut c = Case::new();
    c.after = c.before.clone();
    c.save();
    let v = json(&c.run());
    assert!(v["entries"].as_array().unwrap().is_empty());
    assert_eq!(v["omitted_entries"], 0);
    let response = Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::ChannelResponse(Box::new(ChannelResponse {
            request_id: Id("g13-request".into()),
            session_id: c.before.context.session_id.clone(),
            dispatch_sequence: 1,
            target: c.before.context.target.clone(),
            channel: Channel::ExternalSemantics,
            result: ChannelResult::Observed(Box::new(c.before.clone())),
        })),
    };
    fs::write(c.dir.join("before"), serde_json::to_vec(&response).unwrap()).unwrap();
    let v = json(&c.run());
    assert_eq!(v["before"], v["after"]);
    assert!(v["entries"].as_array().unwrap().is_empty());
}
