use serde_json::{Value, json};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Case {
    root: PathBuf,
    brief: Value,
}
impl Case {
    fn new(name: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "uib-export-cli-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).expect("unique test directory");
        let p = format!(
            "{}/../../fixtures/export/{name}-brief.json",
            env!("CARGO_MANIFEST_DIR")
        );
        let case = Self {
            root,
            brief: serde_json::from_slice(&fs::read(p).unwrap()).unwrap(),
        };
        case.save();
        case
    }
    fn save(&self) {
        fs::write(
            self.root.join("brief.json"),
            serde_json::to_vec_pretty(&self.brief).unwrap(),
        )
        .unwrap();
    }
    fn run(&self, extra: &[&str], input: usize, output: usize) -> Output {
        Command::new(env!("CARGO_BIN_EXE_uiblueprint"))
            .args(["imagegen-prompt", "--brief"])
            .arg(self.root.join("brief.json"))
            .arg("--out")
            .arg(self.root.join("package"))
            .args([
                "--max-input-bytes",
                &input.to_string(),
                "--max-output-bytes",
                &output.to_string(),
                "--max-components",
                "256",
                "--max-views",
                "8",
                "--components-per-detail",
                "12",
            ])
            .args(extra)
            .output()
            .unwrap()
    }
    fn package(&self, file: &str) -> Value {
        serde_json::from_slice(&fs::read(self.root.join("package").join(file)).unwrap()).unwrap()
    }
}
impl Drop for Case {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).unwrap();
    }
}
fn fail(out: Output, exit: i32, code: &str) {
    assert_eq!(out.status.code(), Some(exit));
    assert!(out.stdout.is_empty());
    assert_eq!(out.stderr, format!("{code}\n").as_bytes());
}
fn success(out: Output) -> Value {
    assert_eq!(out.status.code(), Some(0), "stderr {:?}", out.stderr);
    assert!(out.stderr.is_empty());
    serde_json::from_slice(&out.stdout).unwrap()
}
fn check_files(c: &Case) {
    let mut names: Vec<_> = fs::read_dir(c.root.join("package"))
        .unwrap()
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    assert_eq!(
        names,
        [
            "dimensions.json",
            "drawing-brief.md",
            "manifest.json",
            "prompt.txt",
            "scene.json",
            "sheets.json"
        ]
    );
    let prompt = fs::read_to_string(c.root.join("package/prompt.txt")).unwrap();
    for section in [
        "ЗАДАЧА",
        "ОСНОВАНИЕ",
        "ПРОЕКЦИЯ И ЛИСТ",
        "ГРАФИЧЕСКИЙ ЯЗЫК",
        "ОБЪЕКТЫ И ИЕРАРХИЯ",
        "ГЕОМЕТРИЯ",
        "НАНЕСЕНИЕ РАЗМЕРОВ",
        "СОСТОЯНИЯ И ДЕЙСТВИЯ",
        "ОСНОВНАЯ НАДПИСЬ И ВЕДОМОСТЬ",
        "РЕЗУЛЬТАТ",
    ] {
        assert!(prompt.contains(section));
    }
    assert!(!prompt.contains("{{"));
    assert!(prompt.contains("Размеры по подписям; не измерять по изображению"));
}
#[test]
fn proposed_public_command_writes_full_package_and_versioned_json_receipt() {
    let c = Case::new("proposed");
    let receipt = success(c.run(
        &[
            "--json",
            "--purpose",
            "propose",
            "--profile",
            "blue-engineering",
        ],
        2_000_000,
        4_000_000,
    ));
    check_files(&c);
    let mut keys: Vec<_> = receipt
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort();
    assert_eq!(
        keys,
        [
            "approval_status",
            "command",
            "comparison_attribution",
            "files",
            "generated_image",
            "local_numeric_validation",
            "package_bytes",
            "purpose",
            "references_count",
            "result_version",
            "status",
            "validation_status",
            "views"
        ]
    );
    assert_eq!(receipt["result_version"], "0.1.0");
    assert_eq!(receipt["command"], "imagegen-prompt");
    assert_eq!(receipt["status"], "package_written");
    assert_eq!(receipt["purpose"], "propose");
    assert_eq!(receipt["approval_status"], "draft");
    assert_eq!(receipt["validation_status"], "unverified");
    assert_eq!(receipt["generated_image"], false);
    assert_eq!(receipt["references_count"], 0);
    assert_eq!(
        receipt["views"],
        json!([{"view_id":"proposal","source_kind":"proposed","coverage":"proposed","omitted_count":null,"unknown_count":null,"components":10}])
    );
    assert_eq!(c.package("sheets.json").as_array().unwrap().len(), 3);
    assert_eq!(
        c.package("scene.json")["views"][0]["components"][9]["geometry"]["shape"]["value"]["x"],
        1008.0
    );
    let package_bytes: usize = fs::read_dir(c.root.join("package"))
        .unwrap()
        .map(|e| e.unwrap().metadata().unwrap().len() as usize)
        .sum();
    assert_eq!(receipt["package_bytes"], package_bytes);
}
#[test]
fn observed_default_document_preserves_partial_fractional_unknown_and_dense_scope() {
    let mut c = Case::new("observed");
    c.brief.as_object_mut().unwrap().remove("purpose");
    c.save();
    let receipt = success(c.run(&["--json"], 2_000_000, 4_000_000));
    check_files(&c);
    assert_eq!(receipt["purpose"], "document");
    assert_eq!(receipt["views"][0]["coverage"], "partial");
    assert!(receipt["views"][0]["unknown_count"].is_null());
    let scene = c.package("scene.json");
    let view = &scene["views"][0];
    assert_eq!(view["components"].as_array().unwrap().len(), 32);
    assert_eq!(view["relations"].as_array().unwrap().len(), 17);
    assert_eq!(
        view["components"][0]["properties"][2]["state"]["value"]["value"]["shape"]["value"]["width"],
        97.296875
    );
    assert_eq!(
        view["components"][0]["properties"][3]["state"]["availability"],
        "unknown"
    );
    let sheets = c.package("sheets.json");
    assert_eq!(sheets.as_array().unwrap().len(), 4);
    let mut details: Vec<_> = sheets
        .as_array()
        .unwrap()
        .iter()
        .skip(1)
        .flat_map(|s| {
            s["components"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap())
        })
        .collect();
    details.sort();
    details.dedup();
    assert_eq!(details.len(), 32);
    // Partial coverage does not erase explicitly reported anchor geometry.
    let dimensions = c.package("dimensions.json");
    assert_eq!(dimensions[0]["dimensions"][0]["value"], 97.296875);
    assert_eq!(dimensions[0]["dimensions"][1]["value"], 32.0);
    assert!(dimensions[0]["dimensions"][0]["unknown_reason"].is_null());
    assert!(
        !dimensions[0]["dimensions"][0]["evidence"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}
#[test]
fn compact_is_truthful_bounded_and_does_not_print_paths_or_source_identifiers() {
    let c = Case::new("observed");
    let out = c.run(&["--purpose", "explain"], 2_000_000, 4_000_000);
    assert_eq!(out.status.code(), Some(0));
    assert!(out.stderr.is_empty());
    let text = String::from_utf8(out.stdout).unwrap();
    for part in [
        "status=package_written",
        "purpose=\"document\"",
        "validation_status=unverified",
        "generated_image=false",
        "\"coverage\":\"partial\"",
    ] {
        assert!(text.contains(part), "{part}");
    }
    assert!(!text.contains(c.root.to_str().unwrap()));
    assert!(!text.contains("fixture-ref-"));
}
#[test]
fn purpose_override_is_explicit_and_other_modes_keep_compiler_gates() {
    let mut c = Case::new("proposed");
    c.brief.as_object_mut().unwrap().remove("purpose");
    c.save();
    fail(c.run(&[], 2_000_000, 4_000_000), 2, "export_invalid_source");
    assert!(!c.root.join("package").exists());
    assert_eq!(
        success(c.run(&["--purpose", "detail", "--json"], 2_000_000, 4_000_000))["purpose"],
        "detail"
    );
    let mut c = Case::new("observed");
    c.brief["transitions"] = json!([{"before":"observed","after":null,"description":"Available action; destination unknown","transition":null,"actions":[]}]);
    c.save();
    let receipt = success(c.run(&["--purpose", "flow", "--json"], 2_000_000, 4_000_000));
    assert_eq!(receipt["purpose"], "flow");
    assert_eq!(
        c.package("scene.json")["flow"][0]["status"],
        "unverified_or_unknown_destination"
    );
    let mut c = Case::new("proposed");
    let mut second = c.brief["views"][0].clone();
    second["id"] = "proposal-next".into();
    c.brief["views"].as_array_mut().unwrap().push(second);
    c.brief["comparisons"] =
        json!([{"before":"proposal","after":"proposal-next","different_basis":null}]);
    c.save();
    let receipt = success(c.run(&["--purpose", "compare", "--json"], 2_000_000, 4_000_000));
    assert_eq!(receipt["comparison_attribution"], "not_compared");
    assert_eq!(receipt["views"].as_array().unwrap().len(), 2);
}
#[test]
fn invalid_sensitive_and_private_input_have_no_success_or_payload_diagnostics() {
    let mut c = Case::new("observed");
    c.brief["views"][0]["source"]["snapshot"]["nodes"][0]["properties"][1]["sensitivity"] =
        "sensitive".into();
    c.brief["views"][0]["source"]["snapshot"]["nodes"][0]["properties"][1]["state"] =
        json!({"availability":"known","value":{"type":"text","value":"PRIVATE_EXPORT_CANARY"}});
    c.save();
    fail(
        c.run(&["--json"], 2_000_000, 4_000_000),
        2,
        "export_invalid_source",
    );
    assert!(!c.root.join("package").exists());
    fs::write(c.root.join("brief.json"), b"{PRIVATE_EXPORT_CANARY").unwrap();
    fail(c.run(&[], 2_000_000, 4_000_000), 2, "export_invalid_input");
    c.brief = Case::new("proposed").brief.clone();
    c.brief["metadata"]["title"] = "/Users/private/PRIVATE_EXPORT_CANARY".into();
    c.save();
    fail(
        c.run(&[], 2_000_000, 4_000_000),
        2,
        "export_private_content",
    );
    c.brief.as_object_mut().unwrap().remove("metadata");
    c.save();
    fail(
        c.run(&[], 2_000_000, 4_000_000),
        2,
        "export_metadata_required",
    );
}
#[test]
fn explicit_input_and_total_output_limits_fail_before_destination_creation() {
    let c = Case::new("proposed");
    let input = fs::metadata(c.root.join("brief.json")).unwrap().len() as usize;
    fail(c.run(&[], input - 1, 4_000_000), 2, "input_limit");
    fail(c.run(&[], 2_000_000, 100), 2, "output_limit");
    assert!(!c.root.join("package").exists());
    let out = c.run(&["--json"], 2_000_000, 4_000_000);
    let len = out.stdout.len();
    let receipt = success(out);
    let total = receipt["package_bytes"].as_u64().unwrap() as usize + len;
    let other = Case::new("proposed");
    fail(
        other.run(&["--json"], 2_000_000, total - 1),
        2,
        "output_limit",
    );
    assert!(!other.root.join("package").exists());
    assert_eq!(success(other.run(&["--json"], 2_000_000, total)), receipt);
}
#[test]
fn destinations_are_not_overwritten_and_io_failure_is_not_success() {
    let c = Case::new("proposed");
    fs::create_dir(c.root.join("package")).unwrap();
    fs::write(c.root.join("package/baseline"), "keep").unwrap();
    fail(
        c.run(&[], 2_000_000, 4_000_000),
        2,
        "export_destination_exists",
    );
    assert_eq!(
        fs::read_to_string(c.root.join("package/baseline")).unwrap(),
        "keep"
    );
    fs::remove_file(c.root.join("brief.json")).unwrap();
    fail(c.run(&[], 2_000_000, 4_000_000), 1, "io_error");
    let c = Case::new("proposed");
    let out = Command::new(env!("CARGO_BIN_EXE_uiblueprint"))
        .args(["imagegen-prompt", "--brief"])
        .arg(c.root.join("brief.json"))
        .arg("--out")
        .arg(c.root.join("missing-parent/package"))
        .args([
            "--max-input-bytes",
            "2000000",
            "--max-output-bytes",
            "4000000",
            "--max-components",
            "256",
            "--max-views",
            "8",
            "--components-per-detail",
            "12",
        ])
        .output()
        .unwrap();
    fail(out, 1, "io_error");
    assert!(!c.root.join("missing-parent").exists());
}
#[test]
fn export_argument_and_profile_errors_are_sanitized_and_help_names_real_command() {
    for (args, exit, code) in [
        (
            vec!["imagegen-prompt", "--snapshot", "PRIVATE_EXPORT_CANARY"],
            2,
            "export_metadata_required",
        ),
        (
            vec!["imagegen-prompt", "--profile", "PRIVATE_EXPORT_CANARY"],
            5,
            "unsupported_profile",
        ),
        (
            vec!["imagegen-prompt", "--max-views", "0"],
            2,
            "invalid_arguments",
        ),
        (
            vec!["imagegen-prompt", "--json", "--json"],
            2,
            "invalid_arguments",
        ),
        (
            vec!["imagegen-prompt", "--purpose", "PRIVATE_EXPORT_CANARY"],
            2,
            "invalid_arguments",
        ),
    ] {
        fail(
            Command::new(env!("CARGO_BIN_EXE_uiblueprint"))
                .args(args)
                .output()
                .unwrap(),
            exit,
            code,
        );
    }
    let out = Command::new(env!("CARGO_BIN_EXE_uiblueprint"))
        .arg("--help")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    assert!(
        String::from_utf8(out.stdout)
            .unwrap()
            .contains("imagegen-prompt --brief")
    );
}

#[test]
fn stdout_failure_is_io_failure_and_preserves_the_completed_package() {
    use std::process::Stdio;
    let c = Case::new("proposed");
    let mut child = Command::new(env!("CARGO_BIN_EXE_uiblueprint"))
        .args(["imagegen-prompt", "--brief"])
        .arg(c.root.join("brief.json"))
        .arg("--out")
        .arg(c.root.join("package"))
        .args([
            "--max-input-bytes",
            "2000000",
            "--max-output-bytes",
            "4000000",
            "--max-components",
            "256",
            "--max-views",
            "8",
            "--components-per-detail",
            "12",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    drop(child.stdout.take());
    fail(child.wait_with_output().unwrap(), 1, "io_error");
    check_files(&c);
}

fn proposal_geometry_case(origin: &str, x: f64, width: f64, vertical: bool, value: f64) -> Case {
    let mut c = Case::new("proposed");
    let space = json!({"id":"e02-local","kind":"local","units":"css_px","origin":origin});
    let component = |id: &str, x: f64, y: f64, width: f64, height: f64| {
        json!({
            "id":id,"parent":null,"role":"group","label":id,"state_and_actions":"Synthetic proposal; no runtime",
            "geometry":{"frame_kind":"layout_bounds","coordinate_space":space,"shape":{"shape":"rect","value":{"x":x,"y":y,"width":width,"height":height}},"transform":{"status":"local_only"}}
        })
    };
    c.brief["metadata"]["title"] = "E02 geometry regression".into();
    c.brief["views"][0]["safe_source_reference"] = "E02 explicit synthetic arithmetic case".into();
    c.brief["views"][0]["scope"] = "Two explicit test rectangles".into();
    c.brief["details"] = json!([]);
    c.brief["views"][0]["source"]["layout"] = json!({
        "requirements":["E02 explicit target geometry"],
        "components":[component("A",x,0.0,width,10.0),component("B",0.0,30.0,1.0,20.0)],
        "dimensions":[{"id":"distance","label":"Authored edge distance","anchors":[
            {"component":"A","frame_kind":"layout_bounds","space":space,"edge":if vertical {"top"} else {"left"}},
            {"component":if vertical {"B"} else {"A"},"frame_kind":"layout_bounds","space":space,"edge":if vertical {"bottom"} else {"right"}}
        ],"value":value,"units":"css_px","source_kind":"proposed","evidence":[],"requirement_ref":"E02 explicit target geometry","unknown_reason":null,"check_tolerance":null}],
        "chains":[],"unknowns":["radius unknown"]
    });
    c.save();
    c
}
#[test]
fn proposal_origin_repair_reaches_actual_command_in_both_directions() {
    for (origin, correct, wrong) in [("bottom_left", 20.0, 50.0), ("top_left", 50.0, 20.0)] {
        for reverse in [false, true] {
            let mut c = proposal_geometry_case(origin, 0.0, 1.0, true, correct);
            if reverse {
                c.brief["views"][0]["source"]["layout"]["dimensions"][0]["anchors"]
                    .as_array_mut()
                    .unwrap()
                    .reverse();
                c.save();
            }
            success(c.run(&["--json"], 2_000_000, 4_000_000));
            assert_eq!(
                c.package("dimensions.json")[0]["dimensions"][0]["value"],
                correct
            );
            let mut bad = proposal_geometry_case(origin, 0.0, 1.0, true, wrong);
            if reverse {
                bad.brief["views"][0]["source"]["layout"]["dimensions"][0]["anchors"]
                    .as_array_mut()
                    .unwrap()
                    .reverse();
                bad.save();
            }
            fail(
                bad.run(&[], 2_000_000, 4_000_000),
                2,
                "export_invalid_geometry",
            );
            assert!(!bad.root.join("package").exists());
        }
    }
}
#[test]
fn proposal_fractional_repair_preserves_source_values_unknowns_and_statuses() {
    for (x, width) in [(0.2, 0.1), (1e16, 1.0), (1e-12, 1e-14)] {
        let c = proposal_geometry_case("top_left", x, width, false, width);
        let receipt = success(c.run(&["--json"], 2_000_000, 4_000_000));
        assert_eq!(receipt["validation_status"], "unverified");
        assert_eq!(receipt["approval_status"], "draft");
        assert_eq!(receipt["views"][0]["source_kind"], "proposed");
        assert_eq!(
            c.package("dimensions.json")[0]["dimensions"][0]["value"].as_f64(),
            Some(width)
        );
        let scene = c.package("scene.json");
        assert_eq!(scene["views"][0]["unknowns"], json!(["radius unknown"]));
        assert_eq!(
            scene["views"][0]["components"][0]["geometry"]["shape"]["value"]["width"].as_f64(),
            Some(width)
        );
        let bad = proposal_geometry_case("top_left", x, width, false, width * 2.0);
        fail(
            bad.run(&[], 2_000_000, 4_000_000),
            2,
            "export_invalid_geometry",
        );
        assert!(!bad.root.join("package").exists());
    }
}

#[test]
fn proposal_finite_cancellation_rejects_zero_and_two_in_actual_command() {
    for reverse in [false, true] {
        for value in [1.0, 0.0, 2.0] {
            let mut c = proposal_geometry_case("top_left", -1e16, 1e16, false, value);
            let layout = &mut c.brief["views"][0]["source"]["layout"];
            layout["components"][1]["geometry"]["shape"]["value"] =
                json!({"x":1.0,"y":0.0,"width":0.0,"height":1.0});
            layout["dimensions"][0]["anchors"][0]["edge"] = "right".into();
            layout["dimensions"][0]["anchors"][1]["component"] = "B".into();
            layout["dimensions"][0]["anchors"][1]["edge"] = "left".into();
            if reverse {
                layout["dimensions"][0]["anchors"]
                    .as_array_mut()
                    .unwrap()
                    .reverse();
            }
            c.save();
            if value == 1.0 {
                let receipt = success(c.run(&["--json"], 2_000_000, 4_000_000));
                assert_eq!(receipt["validation_status"], "unverified");
                assert_eq!(receipt["approval_status"], "draft");
                assert_eq!(
                    c.package("dimensions.json")[0]["dimensions"][0]["value"],
                    1.0
                );
            } else {
                fail(
                    c.run(&[], 2_000_000, 4_000_000),
                    2,
                    "export_invalid_geometry",
                );
                assert!(!c.root.join("package").exists());
            }
        }
    }
}

impl Case {
    // Existing historical F01 Snapshot, with an explicitly synthetic response
    // envelope when requested. Neither this helper nor the CLI recollects UI.
    fn observed_inputs(&self, response: bool) {
        let view = &self.brief["views"][0];
        let snapshot = &view["source"]["snapshot"];
        let artifact = if response {
            json!({"kind":"channel_response","data":{
                "request_id":"export-test-request","session_id":snapshot["context"]["session_id"],
                "dispatch_sequence":1,"target":snapshot["context"]["target"],"channel":"external_semantics",
                "result":{"status":"observed","data":snapshot}
            }})
        } else {
            json!({"kind":"snapshot","data":snapshot})
        };
        fs::write(
            self.root.join("source.json"),
            serde_json::to_vec_pretty(&json!({"schema_version":"0.1.0","artifact":artifact}))
                .unwrap(),
        )
        .unwrap();
        let metadata = json!({
            "metadata":self.brief["metadata"],"state":view["state"],"scope":view["scope"],
            "environment":view["environment"],"safe_source_reference":view["safe_source_reference"],
            "not_depicted":view["not_depicted"],"public_text_fields":view["source"]["public_text_fields"]
        });
        fs::write(
            self.root.join("metadata.json"),
            serde_json::to_vec_pretty(&metadata).unwrap(),
        )
        .unwrap();
    }
    fn run_observed(&self, extra: &[&str], input: usize, output: usize) -> Output {
        Command::new(env!("CARGO_BIN_EXE_uiblueprint"))
            .args(["imagegen-prompt", "--snapshot"])
            .arg(self.root.join("source.json"))
            .arg("--metadata")
            .arg(self.root.join("metadata.json"))
            .arg("--out")
            .arg(self.root.join("package"))
            .args([
                "--max-input-bytes",
                &input.to_string(),
                "--max-output-bytes",
                &output.to_string(),
                "--max-components",
                "256",
                "--max-views",
                "8",
                "--components-per-detail",
                "12",
            ])
            .args(extra)
            .output()
            .unwrap()
    }
    fn mutate_metadata(&self, edit: impl FnOnce(&mut Value)) {
        let path = self.root.join("metadata.json");
        let mut metadata: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        edit(&mut metadata);
        fs::write(path, serde_json::to_vec_pretty(&metadata).unwrap()).unwrap();
    }
}

#[test]
fn direct_observed_snapshot_and_response_compile_full_scope_without_geometry_reentry() {
    for response in [false, true] {
        let c = Case::new("observed");
        c.observed_inputs(response);
        let original = fs::read(c.root.join("source.json")).unwrap();
        let out = c.run_observed(&["--json"], 2_000_000, 4_000_000);
        let receipt = success(out);
        check_files(&c);
        assert_eq!(receipt["purpose"], "document");
        assert_eq!(receipt["views"][0]["source_kind"], "observed");
        assert_eq!(receipt["views"][0]["coverage"], "partial");
        assert!(receipt["views"][0]["unknown_count"].is_null());
        assert_eq!(receipt["validation_status"], "unverified");
        assert_eq!(receipt["approval_status"], "draft");
        let scene = c.package("scene.json");
        let v = &scene["views"][0];
        assert_eq!(v["components"].as_array().unwrap().len(), 32);
        assert_eq!(v["relations"].as_array().unwrap().len(), 17);
        assert_eq!(v["title"], c.brief["metadata"]["title"]);
        assert_eq!(
            v["components"][0]["properties"][2]["state"]["value"]["value"]["shape"]["value"]["width"],
            97.296875
        );
        assert_eq!(
            v["components"][0]["properties"][2]["state"]["value"]["value"]["coordinate_space"]["units"],
            "css_px"
        );
        assert_eq!(
            v["components"][0]["properties"][3]["state"]["availability"],
            "unknown"
        );
        assert_eq!(v["observations"][0]["consistency"], "unknown");
        assert_eq!(c.package("sheets.json").as_array().unwrap().len(), 4);
        assert_eq!(fs::read(c.root.join("source.json")).unwrap(), original);
        let prompt = fs::read_to_string(c.root.join("package/prompt.txt")).unwrap();
        assert!(prompt.contains("caller annotations"));
        assert!(!prompt.contains("COMMON "));
        assert!(!prompt.contains("observation_id"));
        assert!(prompt.contains("rounded labels"));
        assert!(!prompt.contains("fixture-ref-"));
    }
}
#[test]
fn direct_observed_metadata_is_explicit_and_cannot_inject_geometry_or_sensitive_fields() {
    for (variant, code) in [
        (0, "export_metadata_required"),
        (1, "export_invalid_input"),
        (2, "export_private_content"),
        (3, "export_private_content"),
        (4, "export_approval_record_required"),
    ] {
        let c = Case::new("observed");
        c.observed_inputs(false);
        c.mutate_metadata(|m| match variant {
            0 => {
                m.as_object_mut().unwrap().remove("state");
            }
            1 => {
                m["geometry"] = json!({"width":123});
            }
            2 => {
                m["metadata"]["title"] = "/Users/private/CANARY".into();
            }
            3 => {
                m["public_text_fields"] = json!(["value"]);
            }
            _ => {
                m["metadata"]["approval"] = json!({"status":"accepted","named_record":null});
            }
        });
        fail(c.run_observed(&[], 2_000_000, 4_000_000), 2, code);
        assert!(!c.root.join("package").exists());
    }
    let c = Case::new("observed");
    c.observed_inputs(false);
    c.mutate_metadata(|m| {
        m["state"] = "unknown".into();
        m["environment"] = "unknown".into();
        m["public_text_fields"] = json!([]);
    });
    success(c.run_observed(&["--json", "--purpose", "explain"], 2_000_000, 4_000_000));
    assert_eq!(c.package("scene.json")["views"][0]["state"], "unknown");
    assert_eq!(
        c.package("scene.json")["views"][0]["environment"],
        "unknown"
    );
}
#[test]
fn direct_observed_invalid_failed_multiple_and_sensitive_source_are_rejected() {
    for variant in 0..4 {
        let c = Case::new("observed");
        c.observed_inputs(true);
        let path = c.root.join("source.json");
        let mut source: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        match variant {
            0 => {
                source["artifact"]["data"]["result"] = json!({"status":"failed","data":{"code":"unsupported","scope_id":"selected","failed_step":null,"recovery_class":"retry"}})
            }
            1 => source["schema_version"] = "9.0.0".into(),
            2 => {
                let p =
                    &mut source["artifact"]["data"]["result"]["data"]["nodes"][0]["properties"][1];
                p["sensitivity"] = "sensitive".into();
                p["state"] = json!({"availability":"known","value":{"type":"text","value":"PRIVATE_SOURCE_CANARY"}});
            }
            _ => (),
        }
        let mut bytes = serde_json::to_vec(&source).unwrap();
        if variant == 3 {
            bytes.push(b'\n');
            bytes.extend(serde_json::to_vec(&source).unwrap());
        }
        fs::write(path, bytes).unwrap();
        fail(
            c.run_observed(&["--json"], 2_000_000, 4_000_000),
            2,
            "invalid_input",
        );
        assert!(!c.root.join("package").exists());
    }
}
#[test]
fn direct_observed_limits_modes_and_no_overwrite_reuse_existing_boundary() {
    let c = Case::new("observed");
    c.observed_inputs(true);
    let total = ["source.json", "metadata.json"]
        .iter()
        .map(|p| fs::metadata(c.root.join(p)).unwrap().len() as usize)
        .sum::<usize>();
    fail(c.run_observed(&[], total - 1, 4_000_000), 2, "input_limit");
    fail(c.run_observed(&[], 2_000_000, 100), 2, "output_limit");
    fail(
        c.run_observed(&["--purpose", "propose"], 2_000_000, 4_000_000),
        2,
        "invalid_arguments",
    );
    fail(
        c.run_observed(&["--brief", "irrelevant"], 2_000_000, 4_000_000),
        2,
        "invalid_arguments",
    );
    assert!(!c.root.join("package").exists());
    success(c.run_observed(&["--json"], 2_000_000, 4_000_000));
    let before = fs::read(c.root.join("package/scene.json")).unwrap();
    fail(
        c.run_observed(&[], 2_000_000, 4_000_000),
        2,
        "export_destination_exists",
    );
    assert_eq!(fs::read(c.root.join("package/scene.json")).unwrap(), before);
}

impl Case {
    fn comparison_inputs(&self, response: bool) {
        let golden: Value =
            serde_json::from_slice(include_bytes!("../../../fixtures/golden/GOLDEN01.json"))
                .unwrap();
        for side in ["before", "after"] {
            let snapshot = &golden["artifact"]["data"][side];
            let artifact = if response {
                json!({"kind":"channel_response","data":{
                "request_id":"synthetic-export","session_id":snapshot["context"]["session_id"],
                "dispatch_sequence":1,"target":snapshot["context"]["target"],"channel":"external_semantics",
                "result":{"status":"observed","data":snapshot}}})
            } else {
                json!({"kind":"snapshot","data":snapshot})
            };
            fs::write(
                self.root.join(format!("{side}.json")),
                serde_json::to_vec_pretty(&json!({"schema_version":"0.1.0","artifact":artifact}))
                    .unwrap(),
            )
            .unwrap();
        }
        let annotation = json!({"title":"Synthetic checkbox","state":"recorded synthetic state","scope":"fixture form",
            "environment":"Synthetic data; no live observation","safe_source_reference":"GOLDEN01 controlled synthetic pair",
            "not_depicted":["No geometry or pixels collected"],"public_text_fields":[]});
        fs::write(
            self.root.join("metadata.json"),
            serde_json::to_vec_pretty(&json!({"metadata":self.brief["metadata"],
            "before":annotation,"after":annotation,"different_basis":null,"geometry_space":null}))
            .unwrap(),
        )
        .unwrap();
    }
    fn run_compare(&self, extra: &[&str], input: usize, output: usize) -> Output {
        Command::new(env!("CARGO_BIN_EXE_uiblueprint"))
            .args(["imagegen-prompt", "--before"])
            .arg(self.root.join("before.json"))
            .arg("--after")
            .arg(self.root.join("after.json"))
            .arg("--metadata")
            .arg(self.root.join("metadata.json"))
            .arg("--out")
            .arg(self.root.join("package"))
            .args([
                "--max-input-bytes",
                &input.to_string(),
                "--max-output-bytes",
                &output.to_string(),
                "--max-components",
                "256",
                "--max-views",
                "8",
                "--components-per-detail",
                "12",
            ])
            .args(extra)
            .output()
            .unwrap()
    }
}
#[test]
fn observed_pair_public_cli_emits_full_attributed_checkbox_package() {
    for response in [false, true] {
        let c = Case::new("observed");
        c.comparison_inputs(response);
        let original = fs::read(c.root.join("after.json")).unwrap();
        let r = success(c.run_compare(&["--json"], 2_000_000, 4_000_000));
        check_files(&c);
        assert_eq!(r["result_version"], "0.2.0");
        assert_eq!(r["purpose"], "compare");
        assert_eq!(r["comparison_attribution"], "engine_recorded_graph");
        let s = c.package("scene.json");
        let result = &s["comparison_results"][0];
        assert_eq!(result["status"], "compared");
        assert_eq!(result["omitted_entries"], 0);
        let content: Vec<_> = result["entries"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|e| e["content_changed"] == true)
            .collect();
        assert_eq!(content.len(), 1);
        assert_eq!(content[0]["field"], "checked");
        assert_eq!(content[0]["before"]["state"]["value"]["value"], false);
        assert_eq!(content[0]["after"]["state"]["value"]["value"], true);
        assert_eq!(fs::read(c.root.join("after.json")).unwrap(), original);
        let prompt = fs::read_to_string(c.root.join("package/prompt.txt")).unwrap();
        assert!(prompt.contains("ENGINE RECORDED COMPARISON"));
        assert!(!prompt.contains("unresolved_g02"));
        assert!(!prompt.contains("native-app"));
    }
}
#[test]
fn observed_pair_aggregate_bounds_and_no_overwrite_are_exact() {
    let c = Case::new("observed");
    c.comparison_inputs(false);
    // Whitespace makes the file sum dominate the separately checked assembled brief.
    let pad = |case: &Case| {
        use std::io::Write;
        fs::OpenOptions::new()
            .append(true)
            .open(case.root.join("metadata.json"))
            .unwrap()
            .write_all(&vec![b' '; 10_000])
            .unwrap();
    };
    pad(&c);
    let total = ["before.json", "after.json", "metadata.json"]
        .iter()
        .map(|p| fs::metadata(c.root.join(p)).unwrap().len() as usize)
        .sum::<usize>();
    fail(c.run_compare(&[], total - 1, 4_000_000), 2, "input_limit");
    fail(c.run_compare(&[], 2_000_000, 100), 2, "output_limit");
    assert!(!c.root.join("package").exists());
    let output = c.run_compare(&["--json"], total, 4_000_000);
    let stdout = output.stdout.len();
    let r = success(output);
    let limit = r["package_bytes"].as_u64().unwrap() as usize + stdout;
    let other = Case::new("observed");
    other.comparison_inputs(false);
    pad(&other);
    fail(
        other.run_compare(&["--json"], total, limit - 1),
        2,
        "output_limit",
    );
    assert!(!other.root.join("package").exists());
    assert_eq!(success(other.run_compare(&["--json"], total, limit)), r);
    let original = fs::read(other.root.join("package/scene.json")).unwrap();
    fail(
        other.run_compare(&["--json"], total, limit),
        2,
        "export_destination_exists",
    );
    assert_eq!(
        fs::read(other.root.join("package/scene.json")).unwrap(),
        original
    );
}
#[test]
fn observed_pair_invalid_modes_metadata_and_sources_fail_without_publication() {
    for extra in [
        vec!["--brief", "private"],
        vec!["--snapshot", "private"],
        vec!["--before", "private"],
        vec!["--purpose", "document"],
    ] {
        let c = Case::new("observed");
        c.comparison_inputs(false);
        fail(
            c.run_compare(&extra, 2_000_000, 4_000_000),
            2,
            "invalid_arguments",
        );
        assert!(!c.root.join("package").exists());
    }
    for (variant, code) in [
        (0, "export_metadata_required"),
        (1, "export_invalid_input"),
        (2, "export_private_content"),
        (3, "export_private_content"),
        (4, "export_approval_record_required"),
    ] {
        let c = Case::new("observed");
        c.comparison_inputs(false);
        c.mutate_metadata(|m| match variant {
            0 => {
                m["after"].as_object_mut().unwrap().remove("environment");
            }
            1 => m["before"]["geometry"] = json!({"width":123}),
            2 => m["after"]["public_text_fields"] = json!(["value"]),
            3 => m["before"]["title"] = "/private/PAIR_CANARY".into(),
            _ => m["metadata"]["approval"] = json!({"status":"accepted","named_record":null}),
        });
        fail(c.run_compare(&["--json"], 2_000_000, 4_000_000), 2, code);
        assert!(!c.root.join("package").exists());
    }
    for variant in 0..4 {
        let c = Case::new("observed");
        c.comparison_inputs(false);
        let path = c.root.join("after.json");
        let mut d: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        match variant {
            0 => d["artifact"]["data"]["context"]["session_id"] = "other".into(),
            1 => d["schema_version"] = "9.0.0".into(),
            2 => {
                let p = &mut d["artifact"]["data"]["nodes"][0]["properties"][1];
                p["sensitivity"] = "sensitive".into();
                p["state"] =
                    json!({"availability":"known","value":{"type":"text","value":"PAIR_CANARY"}});
            }
            _ => (),
        }
        let mut bytes = serde_json::to_vec(&d).unwrap();
        if variant == 3 {
            bytes.extend(serde_json::to_vec(&d).unwrap());
        }
        fs::write(path, bytes).unwrap();
        fail(
            c.run_compare(&["--json"], 2_000_000, 4_000_000),
            2,
            if variant == 0 {
                "export_incompatible_views"
            } else {
                "invalid_input"
            },
        );
        assert!(!c.root.join("package").exists());
    }
}

#[test]
fn observed_pair_geometry_selection_and_strict_metadata_reach_public_binary() {
    let c = Case::new("observed");
    c.comparison_inputs(false);
    let source: Value = serde_json::from_slice(include_bytes!(
        "../../../fixtures/golden/GEO-SIZE-RATIO__width.json"
    ))
    .unwrap();
    let before = &source["artifact"]["data"]["snapshot"];
    let mut after = before.clone();
    after["nodes"][0]["properties"][0]["state"]["value"]["value"]["shape"]["value"]["width"] =
        48.0.into();
    for (side, snapshot) in [("before", before), ("after", &after)] {
        fs::write(
            c.root.join(format!("{side}.json")),
            serde_json::to_vec(
                &json!({"schema_version":"0.1.0","artifact":{"kind":"snapshot","data":snapshot}}),
            )
            .unwrap(),
        )
        .unwrap();
    }
    c.mutate_metadata(|m| {
        m["geometry_space"] =
            before["nodes"][0]["properties"][0]["state"]["value"]["value"]["coordinate_space"]["id"]
                .clone()
    });
    success(c.run_compare(&["--purpose", "compare", "--json"], 2_000_000, 4_000_000));
    assert_eq!(
        c.package("scene.json")["comparison_results"][0]["geometry"][0]["displacement"],
        json!({"dx":0.0,"dy":0.0,"dwidth":18.0,"dheight":0.0})
    );
    for side in ["before", "after", "metadata"] {
        let c = Case::new("observed");
        c.comparison_inputs(false);
        c.mutate_metadata(|m| m[side] = json!([]));
        fail(
            c.run_compare(&[], 2_000_000, 4_000_000),
            2,
            "export_invalid_input",
        );
        assert!(!c.root.join("package").exists());
    }
    let c = Case::new("observed");
    c.comparison_inputs(false);
    let path = c.root.join("metadata.json");
    let mut text = fs::read_to_string(&path).unwrap();
    text.pop();
    text.push_str(",\"before\":{} }");
    fs::write(path, text).unwrap();
    fail(
        c.run_compare(&[], 2_000_000, 4_000_000),
        2,
        "export_invalid_input",
    );
    assert!(!c.root.join("package").exists());
}
