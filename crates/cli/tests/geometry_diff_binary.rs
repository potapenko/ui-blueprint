use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};
use uiblueprint_schema::{SchemaVersion, analysis::*, model::*};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Case {
    dir: PathBuf,
    before: Snapshot,
    after: Snapshot,
    key: String,
    space: Space,
}
fn geom(s: &mut Snapshot) -> &mut Geometry {
    let Property::Requested {
        state: Availability::Known {
            value: Value::Geometry(g),
        },
        ..
    } = &mut s.nodes[0].properties[0]
    else {
        panic!("geometry")
    };
    g
}
impl Case {
    fn new() -> Self {
        let doc = Document::from_json(
            include_bytes!("../../../fixtures/golden/GEO-SIZE-RATIO__width.json"),
            65536,
        )
        .unwrap();
        let Artifact::Finding(case) = doc.artifact else {
            panic!("fixture")
        };
        let mut before = case.snapshot;
        let mut source = geom(&mut before).coordinate_space.clone();
        source.id = Id("viewport".into());
        source.kind = SpaceKind::Viewport;
        let mut result = source.clone();
        result.id = Id("document".into());
        result.kind = SpaceKind::Document;
        let mut after = before.clone();
        after.context.environment_revision = Id("scrolled-environment".into());
        for (s, y, scroll) in [(&mut before, 100.0, 0.0), (&mut after, 50.0, 50.0)] {
            let Property::Requested { evidence, .. } = &s.nodes[0].properties[0] else {
                panic!("evidence")
            };
            let transform = Transform {
                from: source.clone(),
                to: result.clone(),
                affine: [1.0, 0.0, 0.0, 1.0, 0.0, scroll],
                target: s.context.target.clone(),
                surface: s.nodes[0].surface.clone(),
                environment_revision: s.context.environment_revision.clone(),
                evidence: evidence.clone(),
            };
            let g = geom(s);
            g.coordinate_space = source.clone();
            g.transform = TransformState::Known {
                transform: Box::new(transform),
            };
            let Shape::Rect(r) = &mut g.shape else {
                panic!("rect")
            };
            r.y = y;
        }
        let key = serde_json::to_string(&before.nodes[0].key).unwrap();
        let dir = std::env::temp_dir().join(format!(
            "uib-g12-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&dir).unwrap();
        let case = Self {
            dir,
            before,
            after,
            key,
            space: result,
        };
        case.save();
        case
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
    fn command(&self, space: &str, input: usize, output: usize) -> Command {
        let mut c = Command::new(env!("CARGO_BIN_EXE_uiblueprint"));
        c.args(["diff", "--geometry", "--before"])
            .arg(self.dir.join("before"))
            .arg("--after")
            .arg(self.dir.join("after"))
            .args([
                "--ref",
                &self.key,
                "--frame-kind",
                "layout_bounds",
                "--space",
                space,
                "--max-input-bytes",
                &input.to_string(),
                "--max-output-bytes",
                &output.to_string(),
                "--json",
            ]);
        c
    }
    fn run(&self, space: &str) -> Output {
        self.command(space, 200000, 200000).output().unwrap()
    }
    fn evaluation(&self, s: &Snapshot, name: &str) {
        fs::write(
            self.dir.join(name),
            serde_json::to_vec(&AnalysisDocument {
                schema_version: AnalysisVersion::CURRENT,
                artifact: AnalysisArtifact::EvaluationInput(Box::new(EvaluationInput {
                    snapshot_id: s.id.clone(),
                    revision: s.revision,
                    context: s.context.clone(),
                    result_space: self.space.clone(),
                    transforms: vec![],
                    conditions: None,
                })),
            })
            .unwrap(),
        )
        .unwrap();
    }
}
impl Drop for Case {
    fn drop(&mut self) {
        for name in ["before", "after", "old-evaluation", "new-evaluation"] {
            let p = self.dir.join(name);
            if p.exists() {
                fs::remove_file(p).unwrap();
            }
        }
        fs::remove_dir(&self.dir).unwrap();
        assert!(!self.dir.exists());
    }
}
fn json(output: &Output, exit: i32) -> serde_json::Value {
    assert_eq!(
        output.status.code(),
        Some(exit),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}
#[test]
fn direct_geometry_diff_discovers_existing_space_and_keeps_raw_mode_exact() {
    let case = Case::new();
    let old = fs::read(case.dir.join("before")).unwrap();
    let new = fs::read(case.dir.join("after")).unwrap();
    let document = json(&case.run("document"), 0);
    assert_eq!(document["kind"], "geometry_difference");
    assert_eq!(document["displacement"]["dy"], 0.0);
    assert_eq!(
        document["before"],
        serde_json::to_value(&case.before).unwrap()
    );
    assert_eq!(
        document["after"],
        serde_json::to_value(&case.after).unwrap()
    );
    assert!(
        !document["before_geometry"]["evidence"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert_eq!(json(&case.run("viewport"), 0)["displacement"]["dy"], -50.0);
    case.evaluation(&case.before, "old-evaluation");
    case.evaluation(&case.after, "new-evaluation");
    let explicit = case
        .command("document", 200000, 200000)
        .arg("--before-evaluation")
        .arg(case.dir.join("old-evaluation"))
        .arg("--after-evaluation")
        .arg(case.dir.join("new-evaluation"))
        .output()
        .unwrap();
    assert_eq!(json(&explicit, 0), document);
    let raw = Command::new(env!("CARGO_BIN_EXE_uiblueprint"))
        .args(["diff", "--before"])
        .arg(case.dir.join("before"))
        .arg("--after")
        .arg(case.dir.join("after"))
        .args([
            "--max-entries",
            "100",
            "--max-input-bytes",
            "200000",
            "--max-output-bytes",
            "200000",
            "--json",
        ])
        .output()
        .unwrap();
    let raw = json(&raw, 0);
    assert_eq!(raw["kind"], "recorded_difference");
    assert_eq!(raw.as_object().unwrap().len(), 9);
    assert!(raw.get("displacement").is_none());
    assert_eq!(fs::read(case.dir.join("before")).unwrap(), old);
    assert_eq!(fs::read(case.dir.join("after")).unwrap(), new);
}
#[test]
fn missing_mapping_is_unknown_not_zero_and_bounds_or_bad_binding_publish_nothing() {
    let mut case = Case::new();
    let mut other = case.after.nodes[0].clone();
    other.key.key = Id("known-document-space".into());
    let Property::Requested {
        state: Availability::Known {
            value: Value::Geometry(g),
        },
        ..
    } = &mut other.properties[0]
    else {
        panic!("geometry")
    };
    g.coordinate_space = case.space.clone();
    g.transform = TransformState::LocalOnly {};
    case.after.nodes.push(other);
    geom(&mut case.after).transform = TransformState::Unknown {
        reason: Id("not-mapped".into()),
    };
    case.after.coverage.status = CoverageStatus::Partial;
    case.save();
    let result = case.run("document");
    let value = json(&result, 4);
    assert!(value["displacement"].is_null());
    assert_eq!(value["after_geometry"]["reason"], "missing_transform");
    assert_eq!(value["after"]["coverage"]["status"], "partial");
    let total = fs::metadata(case.dir.join("before")).unwrap().len() as usize
        + fs::metadata(case.dir.join("after")).unwrap().len() as usize
        + case.key.len();
    for (input, output) in [(total - 1, 200000), (total, result.stdout.len() - 1)] {
        let bad = case.command("document", input, output).output().unwrap();
        assert_eq!(bad.status.code(), Some(2));
        assert!(bad.stdout.is_empty());
    }
    let mut bad = case.before.clone();
    bad.id = Id("wrong-binding".into());
    case.evaluation(&bad, "old-evaluation");
    let result = case
        .command("document", 200000, 200000)
        .arg("--before-evaluation")
        .arg(case.dir.join("old-evaluation"))
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(2));
    assert!(result.stdout.is_empty());
    let conflict = case
        .command("document", 200000, 200000)
        .args(["--max-entries", "1"])
        .output()
        .unwrap();
    assert_eq!(conflict.status.code(), Some(2));
    assert!(conflict.stdout.is_empty());
}
