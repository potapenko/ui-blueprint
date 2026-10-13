use serde_json::{Value, json};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Case {
    dir: PathBuf,
    before: Value,
    after: Value,
    c: Value,
}
impl Case {
    fn new() -> Self {
        let fixture: Value =
            serde_json::from_slice(include_bytes!("../../../fixtures/golden/GEO-GAP.json"))
                .unwrap();
        let mut a = fixture["artifact"]["data"]["snapshot"].clone();
        a["nodes"].as_array_mut().unwrap().truncate(2);
        for node in a["nodes"].as_array_mut().unwrap() {
            node["children"] = json!([]);
        }
        a["relations"] = json!([]);
        a["components"] = json!([]);
        let mut extra = a["nodes"][1].clone();
        extra["key"]["key"] = json!("U");
        a["nodes"].as_array_mut().unwrap().push(extra);
        a["nodes"][0]["children"] = json!([a["nodes"][1]["key"], a["nodes"][2]["key"]]);
        let mut b = a.clone();
        b["id"] = json!("after-source");
        b["context"]["session_id"] = json!("after-session");
        b["context"]["surfaces"][0] = json!({"id":"after-window","generation":"after-generation"});
        let surface = b["context"]["surfaces"][0].clone();
        for n in b["nodes"].as_array_mut().unwrap() {
            n["key"]["key"] = json!(format!("after-{}", n["key"]["key"].as_str().unwrap()));
            n["surface"] = surface.clone();
        }
        b["nodes"][0]["children"] = json!([b["nodes"][2]["key"], b["nodes"][1]["key"]]);
        b["nodes"][1]["properties"][0]["state"]["value"]["value"]["shape"]["value"]["x"] =
            json!(60);
        let mut q = json!({"id":"gap","scope_id":a["context"]["scope_id"],"targets":[a["nodes"][0]["key"],a["nodes"][1]["key"]],"operation":"gap", "anchors":fixture["artifact"]["data"]["expectation"]["rule"]["anchors"],"quantity_kind":"length","units":"css_px","applies_when":{"platform":null,"input_mode":null,"text_scale":null}});
        let mut z = q.clone();
        z["targets"] = json!([b["nodes"][0]["key"], b["nodes"][1]["key"]]);
        for i in 0..2 {
            q["anchors"][i]["element"] = a["nodes"][i]["key"].clone();
            z["anchors"][i]["element"] = b["nodes"][i]["key"].clone();
        }
        let eval = |s: &Value| json!({"snapshot_id":s["id"],"revision":s["revision"],"context":s["context"],"result_space":{"id":"local-form","kind":"local","units":"css_px","origin":"top_left"},"transforms":[],"conditions":null});
        let bind = |s: &Value| json!({"snapshot_id":s["id"],"revision":s["revision"],"context":s["context"]});
        let c = json!({"correspondence_version":"1.0.0","before_binding":bind(&a),"after_binding":bind(&b),"surface_pairs":[{"before":a["context"]["surfaces"][0],"after":b["context"]["surfaces"][0],"declaration":"caller QA assumption, not observed identity"}],"node_pairs":[{"id":"container","before":a["nodes"][0]["key"],"after":b["nodes"][0]["key"],"declaration":"selected container"},{"id":"control","before":a["nodes"][1]["key"],"after":b["nodes"][1]["key"],"declaration":"selected control"}],"measurement_pairs":[{"id":"gap","before_query":q,"after_query":z,"before_evaluation":eval(&a),"after_evaluation":eval(&b)}]});
        let dir = std::env::temp_dir().join(format!(
            "uib-correspondence-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&dir).unwrap();
        let case = Self {
            dir,
            before: a,
            after: b,
            c,
        };
        case.save();
        case
    }
    fn save(&self) {
        for (n, s) in [("a", &self.before), ("b", &self.after)] {
            fs::write(
                self.dir.join(n),
                serde_json::to_vec(
                    &json!({"schema_version":"0.1.0","artifact":{"kind":"snapshot","data":s}}),
                )
                .unwrap(),
            )
            .unwrap();
        }
        fs::write(self.dir.join("c"), serde_json::to_vec(&self.c).unwrap()).unwrap();
    }
    fn run(&self, mode: &str, flags: &[&str]) -> Output {
        let mut p = Command::new(env!("CARGO_BIN_EXE_uiblueprint"));
        p.arg(mode)
            .args(["--before"])
            .arg(self.dir.join("a"))
            .arg("--after")
            .arg(self.dir.join("b"));
        p.args(flags);
        p.output().unwrap()
    }
    fn compare(&self, cap: usize, json_output: bool) -> Output {
        let cap = cap.to_string();
        let path = self.dir.join("c");
        let mut flags = vec![
            "--max-input-bytes",
            "1000000",
            "--max-output-bytes",
            "1000000",
            "--correspondence",
            path.to_str().unwrap(),
            "--max-entries",
            &cap,
        ];
        if json_output {
            flags.push("--json");
        }
        self.run("compare", &flags)
    }
    fn diagnose(&self, json_output: bool) -> Output {
        let mut flags = vec![
            "--diagnose",
            "--max-input-bytes",
            "1000000",
            "--max-output-bytes",
            "1000000",
        ];
        if json_output {
            flags.push("--json");
        }
        self.run("compare", &flags)
    }
}
impl Drop for Case {
    fn drop(&mut self) {
        for n in ["a", "b", "c"] {
            fs::remove_file(self.dir.join(n)).unwrap();
        }
        fs::remove_dir(&self.dir).unwrap();
        assert!(!self.dir.exists());
    }
}
fn report(output: Output, exit: i32) -> Value {
    assert_eq!(
        output.status.code(),
        Some(exit),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    serde_json::from_slice(&output.stdout).unwrap()
}
fn error(output: Output, exit: i32, code: &str) {
    assert_eq!(output.status.code(), Some(exit));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8(output.stderr).unwrap().starts_with(code));
}
#[test]
fn comparison_preserves_originals_and_exposes_selected_facts_and_unexamined_children() {
    let c = Case::new();
    let r = report(c.compare(100, true), 0);
    assert_eq!(
        r["before"],
        serde_json::to_value(
            serde_json::from_value::<uiblueprint_schema::model::Snapshot>(c.before.clone())
                .unwrap()
        )
        .unwrap()
    );
    assert_eq!(
        r["after"],
        serde_json::to_value(
            serde_json::from_value::<uiblueprint_schema::model::Snapshot>(c.after.clone()).unwrap()
        )
        .unwrap()
    );
    assert_eq!(r["correspondence"], c.c);
    assert_eq!(r["kind"], "corresponded_difference");
    assert_eq!(r["correspondence_status"], "caller_declared_unverified");
    let children = r["entries"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["kind"] == "children" && v["pair_index"] == 0)
        .unwrap();
    assert_eq!(children["status"], "reported_subset");
    assert_eq!(children["content_changed"], Value::Null);
    assert_eq!(children["mapped_order_changed"], false);
    assert_eq!(children["unmapped_before_children"], 1);
    assert_eq!(children["before_reported_children"], 2);
    assert_eq!(children["after_reported_children"], 2);
    assert_eq!(
        children["mapped_child_positions"],
        json!([{ "child_pair_index":1,"before_position":0,"after_position":1 }])
    );
    let m = &r["measurements"][0];
    assert_eq!(
        m["before"]["measurement"]["value"]["value"]["amount"].as_f64(),
        Some(8.0)
    );
    assert_eq!(
        m["after"]["measurement"]["value"]["value"]["amount"].as_f64(),
        Some(20.0)
    );
    assert_eq!(m["change"].as_f64(), Some(12.0));
    assert_eq!(r["unexamined_before_nodes"], 1);
    assert!(r["limitations"].as_str().unwrap().contains("unmapped"));
    let compact = c.compare(100, false);
    assert!(compact.status.success());
    let compact = String::from_utf8(compact.stdout).unwrap();
    assert!(compact.contains("before_reported_children="));
    assert!(compact.contains("length_change=12.0"));
    error(
        c.run(
            "diff",
            &[
                "--max-input-bytes",
                "1000000",
                "--max-output-bytes",
                "1000000",
                "--max-entries",
                "100",
            ],
        ),
        4,
        "context_mismatch\n",
    );
}
#[test]
fn diagnostic_gives_actual_axes_and_supported_next_steps_without_ui_or_paths() {
    let mut c = Case::new();
    c.before["nodes"][0]["native_role"] = json!({"availability":"known","value":{"type":"text","value":"private-canary\n\u{1b}[31m"}});
    c.save();
    let r = report(c.diagnose(true), 4);
    assert_eq!(
        r["strict_diff_blocking_axes"],
        json!(["session_id", "surfaces"])
    );
    assert_eq!(r["exact_key_matches"], 0);
    assert_eq!(
        r["next_steps"],
        json!([
            "explicit_correspondence_required",
            "per_side_measure_available"
        ])
    );
    assert!(!r.to_string().contains("private-canary"));
    assert!(!r.to_string().contains(c.dir.to_str().unwrap()));
    let compact = c.diagnose(false);
    assert_eq!(compact.status.code(), Some(4));
    assert!(
        String::from_utf8(compact.stdout)
            .unwrap()
            .contains("compare --correspondence FILE")
    );
    c.after["context"]["target"]["generation"] = json!("other");
    c.save();
    let r = report(c.diagnose(true), 4);
    assert_eq!(r["next_steps"], json!(["protected_basis_incompatible"]));
}
#[test]
fn duplicate_unknown_and_nested_sequence_records_refuse_before_publication() {
    let mut c = Case::new();
    let original = serde_json::to_string(&c.c).unwrap();
    for invalid in [
        original.replacen(
            "\"correspondence_version\":\"1.0.0\"",
            "\"correspondence_version\":\"1.0.0\",\"correspondence_version\":\"1.0.0\"",
            1,
        ),
        original.replacen(
            "\"declaration\":\"selected control\"",
            "\"declaration\":\"selected control\",\"declaration\":\"duplicate\"",
            1,
        ),
    ] {
        fs::write(c.dir.join("c"), invalid).unwrap();
        error(c.compare(100, true), 2, "invalid_correspondence;");
    }
    for variant in 0..4 {
        c = Case::new();
        match variant {
            0 => {
                c.c["before_binding"] =
                    json!([c.before["id"], c.before["revision"], c.before["context"]])
            }
            1 => {
                c.c["node_pairs"][0] = json!([
                    "container",
                    c.before["nodes"][0]["key"],
                    c.after["nodes"][0]["key"],
                    "declared"
                ])
            }
            2 => c.c["extra"] = json!(true),
            _ => c.c["measurement_pairs"][0]["before_query"] = json!([]),
        };
        c.save();
        error(c.compare(100, true), 2, "invalid_correspondence;");
    }
}
#[test]
fn pair_bindings_unknowns_redaction_and_escaping_are_explicit() {
    let mut c = Case::new();
    let base = c.c.clone();
    for variant in 0..4 {
        c.c = base.clone();
        match variant {
            0 => c.c["before_binding"]["revision"] = json!(999),
            1 => c.c["node_pairs"][1]["after"] = c.c["node_pairs"][0]["after"].clone(),
            2 => c.c["node_pairs"][0]["after"]["key"] = json!("dangling"),
            _ => c.c["surface_pairs"][0]["after"]["generation"] = json!("wrong"),
        };
        c.save();
        error(
            c.compare(100, true),
            2,
            if variant == 0 {
                "correspondence_binding_mismatch;"
            } else {
                "invalid_correspondence_pair;"
            },
        );
    }
    c.c = base;
    c.c["node_pairs"][0]["id"] = json!("pair\n\u{1b}[31m");
    c.after["nodes"][1]["properties"][0]["state"] = json!({"availability":"redacted"});
    c.after["nodes"][1]["properties"][0]["sensitivity"] = json!("sensitive");
    c.save();
    let r = report(c.compare(100, true), 4);
    assert_eq!(r["measurements"][0]["change"], Value::Null);
    assert_eq!(r["measurements"][0]["after"]["reason"], "redacted_property");
    let compact = c.compare(100, false);
    assert_eq!(compact.status.code(), Some(4));
    assert!(!compact.stdout.contains(&27));
    assert!(
        String::from_utf8(compact.stdout)
            .unwrap()
            .contains("pair\\n\\u001b")
    );
    c.after["nodes"][1]["properties"][0]["state"] = json!({"availability":"known","value":{"type":"geometry","value":c.before["nodes"][1]["properties"][0]["state"]["value"]["value"]}});
    c.save();
    error(c.compare(100, true), 2, "invalid_input");
}
#[test]
fn zero_entry_and_exact_byte_bounds_and_mode_conflicts_are_atomic() {
    let c = Case::new();
    let full = report(c.compare(100, true), 0);
    let zero = report(c.compare(0, true), 4);
    assert_eq!(zero["entries"], json!([]));
    assert_eq!(zero["measurements"], json!([]));
    assert_eq!(
        zero["omitted_entries"].as_u64().unwrap() as usize,
        full["entries"].as_array().unwrap().len() + 1
    );
    let n = c.compare(100, true).stdout.len();
    let input = fs::metadata(c.dir.join("a")).unwrap().len()
        + fs::metadata(c.dir.join("b")).unwrap().len()
        + fs::metadata(c.dir.join("c")).unwrap().len();
    let path = c.dir.join("c");
    let path = path.to_str().unwrap();
    let out = n.to_string();
    let exact = input.to_string();
    report(
        c.run(
            "compare",
            &[
                "--correspondence",
                path,
                "--max-entries",
                "100",
                "--max-input-bytes",
                &exact,
                "--max-output-bytes",
                &out,
                "--json",
            ],
        ),
        0,
    );
    let less = (n - 1).to_string();
    error(
        c.run(
            "compare",
            &[
                "--correspondence",
                path,
                "--max-entries",
                "100",
                "--max-input-bytes",
                &exact,
                "--max-output-bytes",
                &less,
                "--json",
            ],
        ),
        2,
        "output_limit\n",
    );
    let less = (input - 1).to_string();
    error(
        c.run(
            "compare",
            &[
                "--correspondence",
                path,
                "--max-entries",
                "100",
                "--max-input-bytes",
                &less,
                "--max-output-bytes",
                &out,
                "--json",
            ],
        ),
        2,
        "input_limit\n",
    );
    for flags in [
        vec!["--diagnose", "--max-entries", "0"],
        vec!["--diagnose", "--diagnose"],
        vec!["--diagnose", "--graph"],
        vec!["--correspondence", path, "--diagnose"],
        vec!["--diagnose", "--json", "--json"],
    ] {
        let mut f = flags;
        f.extend([
            "--max-input-bytes",
            "1000000",
            "--max-output-bytes",
            "1000000",
        ]);
        error(c.run("compare", &f), 2, "invalid_arguments\n");
    }
}
