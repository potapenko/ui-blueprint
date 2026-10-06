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
    assert!(
        c.package("dimensions.json")[0]["dimensions"]
            .as_array()
            .unwrap()
            .iter()
            .all(|d| d["value"].is_null() && d["unknown_reason"].is_string())
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
    assert_eq!(receipt["comparison_attribution"], "unresolved_g02");
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
