use serde_json::{Value, json};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};
use uiblueprint_schema::{SchemaVersion, model::*};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Case {
    root: PathBuf,
    snapshot: Snapshot,
}
impl Case {
    fn new() -> Self {
        let doc = Document::from_json(
            include_bytes!("../../../fixtures/golden/ENV-SNAPSHOT-VALID.json"),
            100_000,
        )
        .unwrap();
        let Artifact::Snapshot(mut s) = doc.artifact else {
            panic!("snapshot")
        };
        let template = s.nodes[0].clone();
        s.nodes = vec![template.clone(), template.clone(), template];
        s.nodes[1].key.key = Id("label".into());
        s.nodes[2].key.key = Id("error".into());
        let evidence = Evidence {
            observation_id: s.observations[0].id.clone(),
            source_namespace: s.observations[0].source_namespace.clone(),
            provenance: Provenance::Reported,
            method: Id("literal_test_relation".into()),
            uncertainty: None,
        };
        let edge = |from: usize, to: usize, kind| Relation {
            kind,
            from: s.nodes[from].key.clone(),
            to: s.nodes[to].key.clone(),
            evidence: evidence.clone(),
        };
        s.relations = vec![
            edge(0, 1, RelationKind::LabelledBy),
            edge(2, 0, RelationKind::ErrorFor),
            edge(0, 0, RelationKind::Controls),
            edge(0, 1, RelationKind::CorrespondsTo),
            edge(1, 2, RelationKind::Controls),
        ];
        s.relations[3].evidence.provenance = Provenance::Estimated;
        s.coverage.status = CoverageStatus::Partial;
        s.coverage.unknown_count = None;
        let root = std::env::temp_dir().join(format!(
            "uib-neighbors-cli-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let c = Self { root, snapshot: *s };
        c.save(false);
        c
    }
    fn save(&self, response: bool) {
        let artifact = if response {
            Artifact::ChannelResponse(Box::new(ChannelResponse {
                request_id: Id("fixture-request".into()),
                session_id: self.snapshot.context.session_id.clone(),
                dispatch_sequence: 1,
                target: self.snapshot.context.target.clone(),
                channel: self.snapshot.observations[0].channel,
                result: ChannelResult::Observed(Box::new(self.snapshot.clone())),
            }))
        } else {
            Artifact::Snapshot(Box::new(self.snapshot.clone()))
        };
        let doc = Document {
            schema_version: SchemaVersion::CURRENT,
            artifact,
        };
        doc.validate().unwrap();
        fs::write(
            self.root.join("source.json"),
            serde_json::to_vec(&doc).unwrap(),
        )
        .unwrap();
    }
    fn selector(&self) -> String {
        serde_json::to_string(&self.snapshot.nodes[0].key).unwrap()
    }
    fn run(&self, selector: &str, cap: usize, input: usize, output: usize, json: bool) -> Output {
        let mut c = Command::new(env!("CARGO_BIN_EXE_uiblueprint"));
        c.args(["neighbors", "--snapshot"])
            .arg(self.root.join("source.json"))
            .args([
                "--ref",
                selector,
                "--max-relations",
                &cap.to_string(),
                "--max-input-bytes",
                &input.to_string(),
                "--max-output-bytes",
                &output.to_string(),
            ]);
        if json {
            c.arg("--json");
        }
        c.output().unwrap()
    }
}
impl Drop for Case {
    fn drop(&mut self) {
        fs::remove_file(self.root.join("source.json")).unwrap();
        fs::remove_dir(&self.root).unwrap();
    }
}
fn success(out: Output) -> Value {
    assert_eq!(out.status.code(), Some(0), "{:?}", out.stderr);
    assert!(out.stderr.is_empty());
    serde_json::from_slice(&out.stdout).unwrap()
}
fn fail(out: Output, exit: i32, code: &str) {
    assert_eq!(out.status.code(), Some(exit));
    assert!(out.stdout.is_empty());
    assert_eq!(out.stderr, format!("{code}\n").as_bytes());
}
#[test]
fn saved_neighbors_preserve_direction_order_evidence_counterparts_and_source() {
    let c = Case::new();
    let original = fs::read(c.root.join("source.json")).unwrap();
    let r = success(c.run(&c.selector(), 8, 100_000, 100_000, true));
    let keys: Vec<_> = r.as_object().unwrap().keys().map(String::as_str).collect();
    assert_eq!(
        keys,
        [
            "kind",
            "live_revalidated",
            "neighbors",
            "output_version",
            "selection",
            "selector",
            "snapshot",
            "source"
        ]
    );
    assert_eq!(r["kind"], "relation_neighbors");
    assert_eq!(r["output_version"], "1.0.0");
    assert_eq!(r["live_revalidated"], false);
    assert_eq!(
        serde_json::from_value::<Snapshot>(r["snapshot"].clone()).unwrap(),
        c.snapshot
    );
    assert_eq!(
        r["selection"],
        json!({"max_relations":8,"returned_relations":4,"omitted_relations":0,"truncated":false})
    );
    for (i, (direction, other)) in [
        ("outgoing", 1),
        ("incoming", 2),
        ("self_loop", 0),
        ("outgoing", 1),
    ]
    .iter()
    .enumerate()
    {
        assert_eq!(r["neighbors"][i]["direction"], *direction);
        assert_eq!(
            r["neighbors"][i]["relation"],
            serde_json::to_value(&c.snapshot.relations[i]).unwrap()
        );
        assert_eq!(
            r["neighbors"][i]["counterpart"],
            serde_json::to_value(&c.snapshot.nodes[*other]).unwrap()
        );
    }
    assert_eq!(
        r["neighbors"][3]["relation"]["evidence"]["provenance"],
        "estimated"
    );
    assert_eq!(fs::read(c.root.join("source.json")).unwrap(), original);
}
#[test]
fn cap_zero_and_one_are_separate_from_partial_or_unknown_source_coverage() {
    let mut c = Case::new();
    c.snapshot.coverage.status = CoverageStatus::Unknown;
    if let Property::Requested { state, .. } = &mut c.snapshot.nodes[1].properties[0] {
        *state = Availability::Redacted {};
    }
    c.save(true);
    for cap in [0, 1, 4] {
        let r = success(c.run(&c.selector(), cap, 100_000, 100_000, true));
        assert_eq!(r["selection"]["returned_relations"], cap);
        assert_eq!(r["selection"]["omitted_relations"], 4 - cap);
        assert_eq!(r["selection"]["truncated"], cap < 4);
        assert_eq!(r["snapshot"]["coverage"]["status"], "unknown");
        assert!(r["snapshot"]["coverage"]["unknown_count"].is_null());
        if cap > 0 {
            assert_eq!(
                r["neighbors"][0]["counterpart"]["properties"][0]["state"]["availability"],
                "redacted"
            );
        }
    }
    let out = c.run(&c.selector(), 1, 100_000, 100_000, false);
    assert_eq!(out.status.code(), Some(0));
    let text = String::from_utf8(out.stdout).unwrap();
    for s in [
        "neighbors=saved_observation",
        "live_revalidated=false",
        "source_coverage=",
        "Unknown",
        "returned_relations=1 omitted_relations=3 truncated=true",
        "direction=outgoing",
        "counterpart=",
    ] {
        assert!(text.contains(s), "{s}");
    }
}
#[test]
fn exact_missing_seed_and_invalid_input_return_no_partial_output() {
    let c = Case::new();
    let mut key = c.snapshot.nodes[0].key.clone();
    key.namespace = Id("wrong.namespace".into());
    fail(
        c.run(
            &serde_json::to_string(&key).unwrap(),
            4,
            100_000,
            100_000,
            true,
        ),
        4,
        "target_unresolved",
    );
    fail(
        c.run("PRIVATE_INVALID_SELECTOR", 4, 100_000, 100_000, true),
        2,
        "invalid_input",
    );
    fs::write(c.root.join("source.json"), "PRIVATE_INVALID_SOURCE").unwrap();
    fail(
        c.run(&c.selector(), 4, 100_000, 100_000, true),
        2,
        "invalid_input",
    );
}
#[test]
fn byte_limits_include_selector_and_newline_and_never_truncate_source() {
    let c = Case::new();
    let selector = c.selector();
    let size = fs::metadata(c.root.join("source.json")).unwrap().len() as usize + selector.len();
    fail(
        c.run(&selector, 0, size - 1, 100_000, true),
        2,
        "input_limit",
    );
    let full = c.run(&selector, 0, size, 100_000, true);
    assert_eq!(full.status.code(), Some(0));
    fail(
        c.run(&selector, 0, size, full.stdout.len() - 1, true),
        2,
        "output_limit",
    );
    assert_eq!(
        c.run(&selector, 0, size, full.stdout.len(), true).stdout,
        full.stdout
    );
    let r: Value = serde_json::from_slice(&full.stdout).unwrap();
    assert_eq!(r["snapshot"]["relations"].as_array().unwrap().len(), 5);
    assert_eq!(r["neighbors"], json!([]));
}
