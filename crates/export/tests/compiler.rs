use uiblueprint_export::*;
use uiblueprint_schema::model::*;
fn limits() -> ExportLimits {
    ExportLimits {
        max_input_bytes: 2_000_000,
        max_output_bytes: 4_000_000,
        max_components: 256,
        max_views: 8,
        components_per_detail: 12,
    }
}
fn fixture(name: &str) -> DrawingBrief {
    let path = format!(
        "{}/../../fixtures/export/{name}-brief.json",
        env!("CARGO_MANIFEST_DIR")
    );
    DrawingBrief::from_json(&std::fs::read(path).expect("fixture"), limits()).expect("valid brief")
}
fn data(p: &Package, file: &str) -> serde_json::Value {
    serde_json::from_slice(&p.files()[file]).expect("JSON")
}
fn proposed(b: &mut DrawingBrief) -> &mut ProposedLayout {
    match &mut b.views[0].source {
        SourceInput::Proposed { layout } => layout,
        _ => panic!("proposal"),
    }
}
#[test]
fn exact_example_full_template_and_independent_statuses() {
    let b = fixture("proposed");
    let p = compile(&b, limits()).expect("compile");
    assert_eq!(p.files().len(), 6);
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
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
    let manifest = data(&p, "manifest.json");
    assert_eq!(manifest["validation_status"], "unverified");
    assert_eq!(manifest["approval_status"], "draft");
    assert_eq!(manifest["local_numeric_validation"], "checked");
    let scene = data(&p, "scene.json");
    assert_eq!(
        scene["views"][0]["components"].as_array().unwrap().len(),
        10
    );
    assert_eq!(
        scene["views"][0]["components"][9]["geometry"]["shape"]["value"]["x"],
        1008.0
    );
    assert_eq!(data(&p, "sheets.json").as_array().unwrap().len(), 3);
}
#[test]
fn observed_full_scope_exact_fractional_geometry_and_unknowns() {
    let b = fixture("observed");
    let p = compile(&b, limits()).unwrap();
    let scene = data(&p, "scene.json");
    let v = &scene["views"][0];
    assert_eq!(v["components"].as_array().unwrap().len(), 32);
    assert_eq!(v["relations"].as_array().unwrap().len(), 17);
    assert_eq!(v["coverage"]["status"], "partial");
    assert!(v["coverage"]["unknown_count"].is_null());
    assert_eq!(
        v["components"][0]["properties"][2]["state"]["value"]["value"]["shape"]["value"]["width"],
        97.296875
    );
    assert_eq!(
        v["components"][0]["properties"][3]["state"]["availability"],
        "unknown"
    );
    assert!(data(&p, "sheets.json").as_array().unwrap().len() > 1);
    let all = String::from_utf8(p.files().values().flatten().copied().collect()).unwrap();
    assert!(!all.contains("fixture-ref-"));
    assert!(!all.contains("payload_ref"));
    // Unknown consistency remains attributed; it is not an affirmative unstable state.
    assert_eq!(v["observations"][0]["consistency"], "unknown");
}
#[test]
fn reject_wrong_anchor_unit_value_and_chain() {
    for variant in 0..5 {
        let mut b = fixture("proposed");
        let p = proposed(&mut b);
        match variant {
            0 => p.dimensions[0].anchors[0].component = "missing".into(),
            1 => p.dimensions[0].units = Unit::Pt,
            2 => p.dimensions[0].value = Some(1199.0),
            3 => p.chains[0].terms.pop().map(|_| ()).unwrap(),
            _ => p.dimensions[0].source_kind = SourceKind::Observed,
        }
        assert!(compile(&b, limits()).is_err(), "variant {variant}");
    }
}
#[test]
fn reject_cycles_nonfinite_and_duplicate_components() {
    let mut b = fixture("proposed");
    proposed(&mut b).components[0].parent = Some("N09".into());
    assert!(compile(&b, limits()).is_err());
    let mut b = fixture("proposed");
    let p = proposed(&mut b);
    p.components.push(p.components[0].clone());
    assert!(compile(&b, limits()).is_err());
    let mut b = fixture("proposed");
    proposed(&mut b).dimensions[0].value = Some(f64::NAN);
    assert!(compile(&b, limits()).is_err());
}
#[test]
fn bounds_and_private_diagnostics() {
    let b = fixture("proposed");
    let mut l = limits();
    l.max_output_bytes = 100;
    assert_eq!(compile(&b, l).unwrap_err(), ExportError::OutputLimit);
    l = limits();
    l.max_components = 9;
    assert_eq!(compile(&b, l).unwrap_err(), ExportError::InputLimit);
    l = limits();
    l.max_input_bytes = 10;
    assert!(DrawingBrief::from_json(b"secret-canary-invalid", l).is_err());
    let mut b = b;
    b.metadata.title = "/Users/private/secret-canary".into();
    let e = compile(&b, limits()).unwrap_err();
    assert_eq!(e, ExportError::PrivateContent);
    assert!(!format!("{e:?}").contains("canary"));
}
#[test]
fn explicit_text_policy_sensitive_properties_and_source_ids() {
    let mut b = fixture("observed");
    let SourceInput::Observed {
        snapshot,
        public_text_fields,
    } = &mut b.views[0].source
    else {
        panic!()
    };
    public_text_fields.clear();
    let p = &mut snapshot.nodes[0].properties[1];
    if let Property::Requested { state, .. } = p {
        *state = Availability::Known {
            value: Value::Text("CANARY_PUBLIC_NOT_APPROVED".into()),
        };
    }
    let p = compile(&b, limits()).unwrap();
    let all = String::from_utf8(p.files().values().flatten().copied().collect()).unwrap();
    assert!(!all.contains("CANARY_PUBLIC_NOT_APPROVED"));
    assert!(all.contains("redacted"));
    let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
        panic!()
    };
    if let Property::Requested { sensitivity, .. } = &mut snapshot.nodes[0].properties[1] {
        *sensitivity = Sensitivity::Sensitive;
    }
    assert_eq!(
        compile(&b, limits()).unwrap_err(),
        ExportError::InvalidSource
    );
}
#[test]
fn document_default_and_explain_alias_do_not_accept_proposal() {
    let mut value = serde_json::to_value(fixture("observed")).unwrap();
    value.as_object_mut().unwrap().remove("purpose");
    let bytes = serde_json::to_vec(&value).unwrap();
    assert_eq!(
        DrawingBrief::from_json(&bytes, limits()).unwrap().purpose,
        Purpose::Document
    );
    value["purpose"] = "explain".into();
    assert_eq!(
        DrawingBrief::from_json(&serde_json::to_vec(&value).unwrap(), limits())
            .unwrap()
            .purpose,
        Purpose::Document
    );
    let mut b = fixture("proposed");
    b.purpose = Purpose::Document;
    assert_eq!(
        compile(&b, limits()).unwrap_err(),
        ExportError::InvalidSource
    );
}
#[test]
fn named_approval_is_separate_from_ungenerated_image() {
    let mut b = fixture("proposed");
    b.metadata.approval.status = ApprovalStatus::Accepted;
    assert!(compile(&b, limits()).is_err());
    b.metadata.approval.named_record = Some("user-approved-intent-revision-1".into());
    let p = compile(&b, limits()).unwrap();
    assert_eq!(data(&p, "manifest.json")["validation_status"], "unverified");
}
#[test]
fn detail_and_compare_require_explicit_valid_references_and_bases() {
    let mut b = fixture("proposed");
    b.purpose = Purpose::Detail;
    assert!(compile(&b, limits()).is_ok());
    b.details[0].components.push("missing".into());
    assert!(compile(&b, limits()).is_err());
    let mut b = fixture("proposed");
    b.purpose = Purpose::Compare;
    b.views.push(fixture("observed").views.remove(0));
    b.comparisons.push(ComparisonRequest {
        geometry_space: None,
        before: "proposal".into(),
        after: "observed".into(),
        different_basis: None,
    });
    assert_eq!(
        compile(&b, limits()).unwrap_err(),
        ExportError::IncompatibleViews
    );
    b.comparisons[0].different_basis = Some(
        "Different synthetic requirement and historical F01 runtime; no identity equivalence"
            .into(),
    );
    assert!(compile(&b, limits()).is_ok());
}
#[test]
fn assumed_flow_is_not_confirmed_and_requires_separate_sheet() {
    let mut b = fixture("observed");
    b.purpose = Purpose::Flow;
    b.transitions.push(FlowInput {
        before: "observed".into(),
        after: None,
        description: "Available action; destination unknown".into(),
        transition: None,
        actions: vec![],
    });
    let p = compile(&b, limits()).unwrap();
    assert_eq!(
        data(&p, "scene.json")["flow"][0]["status"],
        "unverified_or_unknown_destination"
    );
    assert!(
        data(&p, "sheets.json")
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s["id"] == "F01")
    );
}
#[test]
fn output_refuses_existing_destination() {
    let dir = std::env::temp_dir().join(format!("uib-e01-existing-{}", std::process::id()));
    std::fs::create_dir(&dir).unwrap();
    std::fs::write(dir.join("baseline"), "keep").unwrap();
    let p = compile(&fixture("proposed"), limits()).unwrap();
    assert_eq!(
        p.write_new(&dir).unwrap_err(),
        ExportError::DestinationExists
    );
    assert_eq!(
        std::fs::read_to_string(dir.join("baseline")).unwrap(),
        "keep"
    );
    std::fs::remove_file(dir.join("baseline")).unwrap();
    std::fs::remove_dir(dir).unwrap();
}

#[test]
fn confirmed_flow_retains_modality_and_requires_action_and_after_evidence() {
    let doc = Document::from_json(
        include_bytes!("../../../fixtures/golden/GOLDEN01.json"),
        1_000_000,
    )
    .unwrap();
    let Artifact::GoldenChain(chain) = doc.artifact else {
        panic!("golden chain")
    };
    let mut b = fixture("observed");
    b.purpose = Purpose::Flow;
    b.views[0].id = "before".into();
    b.views[0].safe_source_reference = "GOLDEN01 synthetic evidence; not runtime proof".into();
    b.views[0].environment = "Synthetic canonical test data".into();
    b.views[0].source = SourceInput::Observed {
        snapshot: Box::new(chain.before.clone()),
        public_text_fields: vec![Field::Name],
    };
    let mut after = b.views[0].clone();
    after.id = "after".into();
    after.state = "checked".into();
    after.source = SourceInput::Observed {
        snapshot: Box::new(chain.after.clone()),
        public_text_fields: vec![Field::Name],
    };
    b.views.push(after);
    b.transitions.push(FlowInput {
        before: "before".into(),
        after: Some("after".into()),
        description: "Synthetic GOLDEN01 set_checked scenario, not live runtime proof".into(),
        transition: Some(chain.verification.clone()),
        actions: vec![chain.action.clone()],
    });
    let p = compile(&b, limits()).unwrap();
    let flow = data(&p, "scene.json");
    assert_eq!(flow["flow"][0]["status"], "confirmed_transition");
    assert_eq!(flow["flow"][0]["steps"][0]["modality"], "setter");
    assert_eq!(flow["flow"][0]["steps"][0]["intent"], "set_checked(true)");
    b.transitions[0].actions.clear();
    assert_eq!(
        compile(&b, limits()).unwrap_err(),
        ExportError::InvalidReference
    );
    b.transitions[0].actions.push(chain.action.clone());
    b.transitions[0].after = None;
    assert!(compile(&b, limits()).is_err());
}

// Literal widths/heights read from the retained F01 source rectangles, not compiler
// output. Partial coverage and unknown consistency do not erase these known anchors
// (accepted engine changes 256f2a2 and 2491dec). Historical package files stay intact.
fn current_observed_dimensions() -> (String, String) {
    let historical = std::fs::read_to_string(format!(
        "{}/../../fixtures/export/observed-package/dimensions.json",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap();
    let expected = [
        ("N000", 97.296875, 32.0),
        ("N002", 200.0, 60.0),
        ("N004", 98.78125, 32.0),
        ("N006", 120.0, 40.0),
        ("N008", 120.0, 40.0),
        ("N010", 100.0, 40.0),
        ("N012", 60.0, 30.0),
        ("N014", 360.0, 123.0),
        ("N016", 188.0, 21.0),
        ("N018", 360.0, 22.0),
        ("N020", 360.0, 22.0),
        ("N022", 87.625, 32.0),
        ("N024", 0.0, 18.0),
        ("N026", 146.9375, 32.0),
        ("N028", 100.0, 30.0),
        ("N030", 32.0, 16.0),
    ];
    let parsed: serde_json::Value = serde_json::from_str(&historical).unwrap();
    assert_eq!(parsed[0]["dimensions"].as_array().unwrap().len(), 32);
    assert_eq!(historical.matches("\"value\": null").count(), 32);
    assert_eq!(
        historical
            .matches("\"unknown_reason\": \"unstable_state\"")
            .count(),
        32
    );
    let mut current = historical.clone();
    for (index, (component, width, height)) in expected.into_iter().enumerate() {
        for (axis, value) in [width, height].into_iter().enumerate() {
            let dimension = &parsed[0]["dimensions"][index * 2 + axis];
            assert_eq!(dimension["id"], format!("M{:04}", index * 2 + axis + 1));
            for anchor in dimension["anchors"].as_array().unwrap() {
                assert_eq!(anchor["component"], component);
            }
            current = current.replacen("\"value\": null", &format!("\"value\": {value:?}"), 1);
        }
    }
    current = current.replace(
        "\"unknown_reason\": \"unstable_state\"",
        "\"unknown_reason\": null",
    );
    (historical, current)
}
#[test]
fn current_packages_preserve_history_except_accepted_known_anchor_results() {
    let (historical_dimensions, current_dimensions) = current_observed_dimensions();
    for name in ["observed", "proposed"] {
        let brief = fixture(name);
        let original = serde_json::to_vec(&brief).unwrap();
        let package = compile(&brief, limits()).expect("existing package");
        assert_eq!(serde_json::to_vec(&brief).unwrap(), original);
        assert_eq!(package.files().len(), 6);
        for (file, actual) in package.files() {
            if file == "prompt.txt" {
                // Prompt tables are independently decoded and checked below.
                continue;
            }
            let path = format!(
                "{}/../../fixtures/export/{name}-package/{file}",
                env!("CARGO_MANIFEST_DIR")
            );
            let mut expected =
                std::fs::read_to_string(path).expect("immutable historical artifact");
            if name == "observed" {
                let replacement = match file.as_str() {
                    "dimensions.json" | "drawing-brief.md" => {
                        Some((historical_dimensions.clone(), current_dimensions.clone()))
                    }
                    _ => None,
                };
                if let Some((old, new)) = replacement {
                    assert_eq!(
                        expected.matches(&old).count(),
                        1,
                        "{file}: exact historical dimension block"
                    );
                    expected = expected.replacen(&old, &new, 1);
                }
            }
            // The other five files, including every unrelated byte, remain checked.
            // No normalization of source facts, privacy, evidence, anchors or statuses.
            assert!(actual == expected.as_bytes(), "{name}/{file}");
        }
    }
}

#[test]
fn explicitly_unstable_observation_keeps_dimensions_unknown_with_evidence() {
    let mut brief = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut brief.views[0].source else {
        panic!("observed fixture")
    };
    for observation in &mut snapshot.observations {
        observation.consistency = Consistency::Unstable;
        observation.consistency_reason = Some(Id("synthetic_unstable_source".into()));
    }
    let original = serde_json::to_vec(&brief).unwrap();
    let package = compile(&brief, limits()).unwrap();
    assert_eq!(serde_json::to_vec(&brief).unwrap(), original);
    let dimensions = data(&package, "dimensions.json");
    let dimensions = dimensions[0]["dimensions"].as_array().unwrap();
    assert_eq!(dimensions.len(), 32);
    for dimension in dimensions {
        assert!(dimension["value"].is_null());
        assert_eq!(dimension["unknown_reason"], "unstable_state");
        assert!(!dimension["evidence"].as_array().unwrap().is_empty());
        assert!(dimension["requirement_ref"].is_null());
        assert!(dimension["check_tolerance"].is_null());
    }
    assert_eq!(
        data(&package, "scene.json")["views"][0]["coverage"]["status"],
        "partial"
    );
    assert_eq!(
        data(&package, "manifest.json")["validation_status"],
        "unverified"
    );
}

#[test]
fn factual_query_known_extents_keep_source_evidence_without_normative_fields() {
    let doc = Document::from_json(
        include_bytes!("../../../fixtures/golden/GEO-SIZE-RATIO__width.json"),
        100_000,
    )
    .unwrap();
    let Artifact::Finding(case) = doc.artifact else {
        panic!("canonical authored fixture")
    };
    let mut b = fixture("observed");
    b.views[0].source = SourceInput::Observed {
        snapshot: Box::new(case.snapshot),
        public_text_fields: vec![],
    };
    b.views[0].safe_source_reference =
        "GEO-SIZE-RATIO authored fixture; not runtime evidence".into();
    b.views[0].environment = "Synthetic canonical test data".into();
    let before = serde_json::to_vec(&b).unwrap();
    let package = compile(&b, limits()).unwrap();
    assert_eq!(serde_json::to_vec(&b).unwrap(), before);
    let dimensions = data(&package, "dimensions.json");
    let first = &dimensions[0]["dimensions"][0];
    assert_eq!(first["value"], 30.0);
    assert_eq!(dimensions[0]["dimensions"][1]["value"], 10.0);
    assert_eq!(first["source_kind"], "observed");
    assert_eq!(first["units"], "css_px");
    assert!(first["requirement_ref"].is_null());
    assert!(first["check_tolerance"].is_null());
    assert!(first["unknown_reason"].is_null());
    let scene = data(&package, "scene.json");
    assert_eq!(
        first["evidence"],
        serde_json::json!([scene["views"][0]["components"][0]["properties"][0]["evidence"]])
    );
    assert_eq!(
        data(&package, "manifest.json")["validation_status"],
        "unverified"
    );
}

// Independent reader for the human table notation. Reconstruct nested arrays/objects
// from the documented paths, rather than using the exporter's field factoring code.
fn prompt_tables(prompt: &str) -> Vec<Vec<serde_json::Value>> {
    use serde_json::Value;
    fn put(root: &mut Value, path: &[&str], value: Value) {
        if path.is_empty() {
            *root = value;
        } else if let Ok(index) = path[0].parse::<usize>() {
            if root.is_null() {
                *root = Value::Array(vec![]);
            }
            let array = root.as_array_mut().expect("array offset");
            array.resize(array.len().max(index + 1), Value::Null);
            put(&mut array[index], &path[1..], value);
        } else {
            if root.is_null() {
                *root = serde_json::json!({});
            }
            let key = path[0].replace("~1", "/").replace("~0", "~");
            put(
                root.as_object_mut()
                    .expect("named field")
                    .entry(key)
                    .or_insert(Value::Null),
                &path[1..],
                value,
            );
        }
    }
    let mut tables = vec![];
    for table in prompt.split("\nCOMMON ").skip(1) {
        let (common, table) = table.split_once("\nCOLUMNS ").unwrap();
        let common: serde_json::Map<String, Value> = serde_json::from_str(common).unwrap();
        let (columns, table) = table.split_once("\nROWS\n").unwrap();
        let columns: Vec<String> = serde_json::from_str(columns).unwrap();
        let (rows, _) = table.split_once("END TABLE").unwrap();
        let mut decoded = vec![];
        for line in rows.lines() {
            let mut fields = common.clone();
            let mut rest = line.strip_prefix('[').unwrap();
            for (i, path) in columns.iter().enumerate() {
                if i > 0 {
                    rest = rest.strip_prefix(',').unwrap();
                }
                if let Some(tail) = rest.strip_prefix("absent") {
                    rest = tail;
                } else {
                    let mut decoder = serde_json::Deserializer::from_str(rest).into_iter::<Value>();
                    let value = decoder.next().unwrap().unwrap();
                    rest = &rest[decoder.byte_offset()..];
                    assert!(
                        fields.insert(path.clone(), value).is_none(),
                        "no common override"
                    );
                }
            }
            assert_eq!(rest, "]");
            let mut record = Value::Null;
            for (path, value) in fields {
                let path = path
                    .strip_prefix('/')
                    .unwrap()
                    .split('/')
                    .collect::<Vec<_>>();
                put(&mut record, &path, value);
            }
            decoded.push(record);
        }
        tables.push(decoded);
    }
    tables
}

#[test]
fn prompt_tables_preserve_independent_historical_inventory_and_exact_dimensions() {
    for name in ["observed", "proposed"] {
        let p = compile(&fixture(name), limits()).unwrap();
        let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
        let tables = prompt_tables(prompt);
        assert_eq!(tables.len(), 2, "single inventory, single dimension table");
        let historical = |file: &str| {
            std::fs::read_to_string(format!(
                "{}/../../fixtures/export/{name}-package/{file}",
                env!("CARGO_MANIFEST_DIR")
            ))
            .unwrap()
        };
        let scene: serde_json::Value = serde_json::from_str(&historical("scene.json")).unwrap();
        let dimensions: serde_json::Value = serde_json::from_str(&if name == "observed" {
            current_observed_dimensions().1
        } else {
            historical("dimensions.json")
        })
        .unwrap();
        // String equality also protects signed zero and exact numeric representations.
        assert_eq!(
            serde_json::to_string(&tables[0]).unwrap(),
            scene["views"][0]["components"].to_string()
        );
        assert_eq!(
            serde_json::to_string(&tables[1]).unwrap(),
            dimensions[0]["dimensions"].to_string()
        );
        let chains = prompt.split_once("Derived chains: ").unwrap().1;
        let actual_chains = serde_json::Deserializer::from_str(chains)
            .into_iter::<serde_json::Value>()
            .next()
            .unwrap()
            .unwrap();
        assert_eq!(actual_chains, dimensions[0]["chains"]);
        assert!(prompt.contains("validation unverified; approval"));
        assert!(!prompt.contains("{{"));
    }
}

#[test]
fn prompt_details_follow_only_the_actual_sheet_plan() {
    let mut b = fixture("proposed");
    b.details.clear();
    let p = compile(&b, limits()).unwrap();
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert!(prompt.contains("Увеличенные детали со ссылками на G01: []."));
    assert_eq!(data(&p, "sheets.json").as_array().unwrap().len(), 1);
    let mut l = limits();
    l.components_per_detail = 6;
    let p = compile(&b, l).unwrap();
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    assert!(prompt.contains("Увеличенные детали со ссылками на G01: [D01, D02]."));
    let sheets = data(&p, "sheets.json");
    assert_eq!(sheets[1]["parent_view"], "G01");
    assert_eq!(
        sheets[2]["components"],
        serde_json::json!(["N06", "N07", "N08", "N09"])
    );
    assert_eq!(prompt_tables(prompt)[0].len(), 10);
}

#[test]
fn prompt_tables_keep_property_order_presence_and_signed_zero() {
    let mut b = fixture("observed");
    let SourceInput::Observed { snapshot, .. } = &mut b.views[0].source else {
        panic!()
    };
    let mut expected: serde_json::Value = serde_json::from_slice(include_bytes!(
        "../../../fixtures/export/observed-package/scene.json"
    ))
    .unwrap();
    // These are source changes, not expectations copied from the candidate output.
    for index in [0, 1] {
        let Property::Requested {
            state: Availability::Known {
                value: Value::Geometry(g),
            },
            ..
        } = &mut snapshot.nodes[index * 2].properties[2]
        else {
            panic!()
        };
        let Shape::Rect(rect) = &mut g.shape else {
            panic!()
        };
        rect.x = if index == 0 { -0.0 } else { 0.0 };
        expected["views"][0]["components"][index * 2]["properties"][2]["state"]["value"]["value"]
            ["shape"]["value"]["x"] = serde_json::json!(rect.x);
    }
    snapshot.nodes[2].properties.swap(0, 2);
    expected["views"][0]["components"][2]["properties"]
        .as_array_mut()
        .unwrap()
        .swap(0, 2);
    let p = compile(&b, limits()).unwrap();
    let prompt = std::str::from_utf8(&p.files()["prompt.txt"]).unwrap();
    let tables = prompt_tables(prompt);
    assert_eq!(
        serde_json::to_string(&tables[0]).unwrap(),
        expected["views"][0]["components"].to_string()
    );
    assert!(prompt.contains("absent"));
    assert!(prompt.contains("-0.0"));
    assert!(prompt.contains("0.0"));
}
