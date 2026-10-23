//! Synthetic boundary vectors, separate from app-owner live Compose acceptance.
use serde_json::{Value, json};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};
use uiblueprint_schema::{analysis::*, model::*};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Case {
    dir: PathBuf,
}
impl Case {
    fn new() -> Self {
        let dir = std::env::temp_dir().join(format!(
            "uib-compose-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&dir).unwrap();
        Self { dir }
    }
    fn save(&self, name: &str, value: &Value) {
        fs::write(self.dir.join(name), serde_json::to_vec(value).unwrap()).unwrap();
    }
    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_uiblueprint"))
            .current_dir(&self.dir)
            .args(args)
            .output()
            .unwrap()
    }
    fn import(&self) -> Output {
        self.run(&[
            "import-compose",
            "--input",
            "raw",
            "--max-input-bytes",
            "1048576",
            "--max-output-bytes",
            "1048576",
        ])
    }
}
impl Drop for Case {
    fn drop(&mut self) {
        for item in fs::read_dir(&self.dir).unwrap() {
            fs::remove_file(item.unwrap().path()).unwrap();
        }
        fs::remove_dir(&self.dir).unwrap();
        assert!(!self.dir.exists());
    }
}
fn probe() -> Value {
    json!({"probe_version":"1.0.0","session_id":"session-1","target":{"id":"test-app","generation":"launch-1"},
        "surface":{"id":"main","generation":"surface-1"},"platform":"android","request_id":"capture-1",
        "scope_id":"player","root_marker_id":"root","sequence":1,"start_ms":10.0,"end_ms":11.0,
        "density":2.0,"font_scale":1.3,"nodes":[
            {"marker_id":"root","mount_id":"root-1","bounds":{"x":0.0,"y":0.0,"width":800.0,"height":600.0,"density":2.0,"font_scale":1.3}},
            {"marker_id":"video","mount_id":"video-1","bounds":{"x":20.5,"y":40.0,"width":400.0,"height":200.0,"density":2.0,"font_scale":1.3}}]})
}
fn snapshot(output: &Output) -> Snapshot {
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let doc = Document::from_json(&output.stdout, 1_048_576).unwrap();
    let Artifact::Snapshot(s) = doc.artifact else {
        panic!("snapshot")
    };
    *s
}
fn query(s: &Snapshot) -> Value {
    json!({"schema_version":"0.2.0","artifact":{"kind":"geometry_query","data":{
        "id":"width","scope_id":s.context.scope_id,"targets":[s.nodes[1].key],"operation":"width",
        "anchors":[{"element":s.nodes[1].key,"frame_kind":"layout_bounds","coordinate_space":{"id":"compose-root-px","kind":"local","units":"px","origin":"top_left"},"fraction":0.0,"axis":"x"}],
        "quantity_kind":"length","units":"dp","applies_when":{"platform":null,"text_scale":null,"input_mode":null}}}})
}
#[test]
fn compose_import_to_canonical_measure_and_compatible_diff() {
    let c = Case::new();
    c.save("raw", &probe());
    let imported = c.import();
    let s = snapshot(&imported);
    assert_eq!(s.coverage.status, CoverageStatus::Partial);
    assert_eq!(s.nodes[1].key.key.0, "video-video-1");
    assert_eq!(s.observations[0].channel, Channel::OptInLayoutProbe);
    fs::write(c.dir.join("before"), &imported.stdout).unwrap();
    c.save("query", &query(&s));
    let measured = c.run(&[
        "measure",
        "--snapshot",
        "before",
        "--query",
        "query",
        "--space",
        "compose-root-dp",
        "--max-input-bytes",
        "1048576",
        "--max-output-bytes",
        "1048576",
        "--json",
    ]);
    assert_eq!(
        measured.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&measured.stderr)
    );
    let doc = AnalysisDocument::from_json(&measured.stdout, 1_048_576).unwrap();
    let AnalysisArtifact::Measurement(case) = doc.artifact else {
        panic!("measurement")
    };
    let MeasurementResult::Known { measurement } = case.result else {
        panic!("known")
    };
    assert_eq!(
        measurement.value,
        uiblueprint_schema::model::Value::Quantity {
            amount: 200.0,
            kind: QuantityKind::Length,
            source_units: Unit::Dp
        }
    );
    // Independent synthetic fixture requirement, not an inferred app requirement.
    let q = query(&s)["artifact"]["data"].clone();
    c.save("expectation", &json!({"schema_version":"0.1.0", "artifact":{"kind":"expectation","data":{
        "id":"width-fixture", "scope_id":s.context.scope_id, "targets":q["targets"],
        "applies_when":q["applies_when"], "expected_from":"synthetic_fixture_literal_200dp",
        "rule":{"relation":"geometry","operation":"width","anchors":q["anchors"],
            "expected":200.0,"comparison":"equal","quantity_kind":"length","units":"dp","tolerance":0.0}
    }}}));
    let checked = c.run(&[
        "check",
        "--snapshot",
        "before",
        "--expectation",
        "expectation",
        "--space",
        "compose-root-dp",
        "--max-input-bytes",
        "1048576",
        "--max-output-bytes",
        "1048576",
        "--json",
        "--result-version",
        "0.2.0",
    ]);
    assert_eq!(
        checked.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&checked.stderr)
    );
    let mut next = probe();
    next["sequence"] = json!(2);
    next["nodes"][1]["bounds"]["width"] = json!(480.0);
    c.save("raw", &next);
    let after = c.import();
    snapshot(&after);
    fs::write(c.dir.join("after"), &after.stdout).unwrap();
    let key = serde_json::to_string(&s.nodes[1].key).unwrap();
    let diff = c.run(&[
        "diff",
        "--geometry",
        "--before",
        "before",
        "--after",
        "after",
        "--ref",
        &key,
        "--frame-kind",
        "layout_bounds",
        "--space",
        "compose-root-dp",
        "--max-input-bytes",
        "1048576",
        "--max-output-bytes",
        "1048576",
        "--json",
    ]);
    assert_eq!(
        diff.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&diff.stderr)
    );
    let result: Value = serde_json::from_slice(&diff.stdout).unwrap();
    assert_eq!(result["displacement"]["dwidth"], 40.0);
}
#[test]
fn compose_unknown_bounds_remain_unknown() {
    let c = Case::new();
    let mut p = probe();
    p["nodes"][1]["bounds"] = Value::Null;
    c.save("raw", &p);
    let s = snapshot(&c.import());
    assert_eq!(s.coverage.unknown_count, Some(1));
    assert!(matches!(
        s.nodes[1].properties[0],
        Property::Requested {
            state: Availability::Unknown { .. },
            ..
        }
    ));
}
#[test]
fn compose_strict_shape_identity_density_and_privacy_refusals() {
    let c = Case::new();
    for changed in [
        {
            let mut p = probe();
            p["density"] = json!(0);
            p
        },
        {
            let mut p = probe();
            p["density"] = json!(1e-320);
            p
        },
        {
            let mut p = probe();
            p["nodes"][1]["marker_id"] = json!("root");
            p
        },
        {
            let mut p = probe();
            p["nodes"][1]["mount_id"] = json!("root-1");
            p
        },
        {
            let mut p = probe();
            p["nodes"][1]["bounds"]["width"] = json!(-1);
            p
        },
        {
            let mut p = probe();
            p["nodes"][1]["text"] = json!("SECRET-CANARY");
            p
        },
        {
            let mut p = probe();
            p["nodes"][1]["bounds"] = json!([20, 40, 400, 200, 2, 1.3]);
            p
        },
        {
            let mut p = probe();
            p["platform"] = json!("windows");
            p
        },
        {
            let mut p = probe();
            p["nodes"] = json!([]);
            p
        },
    ] {
        c.save("raw", &changed);
        let out = c.import();
        assert_eq!(out.status.code(), Some(2));
        assert!(out.stdout.is_empty());
        assert!(!String::from_utf8_lossy(&out.stderr).contains("SECRET-CANARY"));
    }
    let bytes = serde_json::to_string(&probe()).unwrap().replacen(
        "\"probe_version\":",
        "\"probe_version\":\"1.0.0\",\"probe_version\":",
        1,
    );
    fs::write(c.dir.join("raw"), bytes).unwrap();
    assert_eq!(c.import().status.code(), Some(2));
}
#[test]
fn compose_bounds_and_import_are_prepublication() {
    let c = Case::new();
    c.save("raw", &probe());
    let out = c.import();
    snapshot(&out);
    for (input, output) in [
        ("16".to_owned(), "1048576".to_owned()),
        ("1048576".to_owned(), (out.stdout.len() - 1).to_string()),
    ] {
        let failed = c.run(&[
            "import-compose",
            "--input",
            "raw",
            "--max-input-bytes",
            &input,
            "--max-output-bytes",
            &output,
        ]);
        assert_eq!(failed.status.code(), Some(2));
        assert!(failed.stdout.is_empty());
    }
}

#[test]
fn compose_unknown_root_retains_independent_density_and_font_scale() {
    let c = Case::new();
    let mut p = probe();
    p["density"] = json!(2.625);
    p["font_scale"] = json!(1.375);
    p["nodes"][0]["bounds"] = Value::Null;
    p["nodes"][1]["bounds"]["font_scale"] = json!(1.0);
    c.save("raw", &p);
    let s = snapshot(&c.import());
    let number = |node: &Node, name: &str| {
        let declaration = node
            .source_declarations
            .iter()
            .find(|d| d.name.0 == name)
            .unwrap();
        let Availability::Known {
            value: uiblueprint_schema::model::Value::Number(n),
        } = declaration.state
        else {
            panic!("known independent source fact")
        };
        n
    };
    assert_eq!(number(&s.nodes[0], "density"), 2.625);
    assert_eq!(number(&s.nodes[0], "font_scale"), 1.375);
    assert_eq!(number(&s.nodes[1], "density"), 2.0);
    assert_eq!(number(&s.nodes[1], "font_scale"), 1.0);
    assert!(matches!(
        s.nodes[0].properties[0],
        Property::Requested {
            state: Availability::Unknown { .. },
            ..
        }
    ));
    let Some(uiblueprint_schema::model::Value::Geometry(g)) = s.nodes[1].properties[0].known()
    else {
        panic!("child geometry")
    };
    let TransformState::Known { transform } = &g.transform else {
        panic!("density transform")
    };
    assert_eq!(transform.affine[0], 1.0 / 2.625);
}
