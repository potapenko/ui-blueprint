use serde_json::{Value, json};
use std::{fs, path::PathBuf};
use uiblueprint_schema::{model::Document, validation::ValidationError};

fn fixture(name: &str) -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/golden")
        .join(format!("{name}.json"));
    serde_json::from_slice(&fs::read(path).expect("authored fixture")).expect("fixture JSON")
}
fn validate(value: &Value) -> Result<Document, ValidationError> {
    let bytes = serde_json::to_vec(value).expect("encode authored mutation");
    Document::from_json(&bytes, bytes.len())
}
fn geometry(value: &mut Value, index: usize) -> &mut Value {
    &mut value["artifact"]["data"]["snapshot"]["nodes"][index]["properties"][0]["state"]["value"]["value"]
}

#[test]
fn equal_units_need_a_common_space_or_explicit_compatible_transform() {
    let mut v = fixture("GEO-GAP");
    validate(&v).expect("common local space");
    geometry(&mut v, 1)["coordinate_space"]["id"] = json!("other-local");
    v["artifact"]["data"]["expectation"]["rule"]["anchors"][1]["coordinate_space"]["id"] =
        json!("other-local");
    assert_eq!(validate(&v), Err(ValidationError::MissingTransform));
    let d = &v["artifact"]["data"];
    let destination =
        d["snapshot"]["nodes"][0]["properties"][0]["state"]["value"]["value"]["coordinate_space"]
            .clone();
    let from = d["expectation"]["rule"]["anchors"][1]["coordinate_space"].clone();
    let transform = json!({"status":"known", "transform":{
        "from":from,"to":destination,"affine":[1,0,0,1,0,0],
        "evidence":d["snapshot"]["nodes"][1]["properties"][0]["evidence"],
        "target":d["snapshot"]["context"]["target"],
        "surface":d["snapshot"]["nodes"][1]["surface"],
        "environment_revision":d["snapshot"]["context"]["environment_revision"]
    }});
    geometry(&mut v, 1)["transform"] = transform;
    validate(&v).expect("explicit transform reaches the same actual destination");
    let original_surface = geometry(&mut v, 1)["transform"]["transform"]["surface"].clone();
    let other_surface = json!({"id":"another-window","generation":"w1"});
    v["artifact"]["data"]["snapshot"]["context"]["surfaces"]
        .as_array_mut()
        .unwrap()
        .push(other_surface.clone());
    geometry(&mut v, 1)["transform"]["transform"]["surface"] = other_surface;
    assert_eq!(validate(&v), Err(ValidationError::MissingTransform));
    geometry(&mut v, 1)["transform"]["transform"]["surface"] = original_surface;
    geometry(&mut v, 1)["transform"]["transform"]["to"]["id"] = json!("third-local");
    assert_eq!(validate(&v), Err(ValidationError::MissingTransform));
}

#[test]
fn geometric_property_keys_match_every_frame_kind() {
    let kinds = [
        "layout_bounds",
        "accessibility_bounds",
        "hit_region",
        "visible_region",
        "paint_bounds",
    ];
    for kind in kinds {
        let mut v = fixture("GEO-GAP");
        let d = &mut v["artifact"]["data"];
        d["snapshot"]["context"]["fields"] = json!([kind]);
        d["snapshot"]["coverage"]["fields"] = json!([kind]);
        d["snapshot"]["observations"][0]["coverage"]["fields"] = json!([kind]);
        for n in d["snapshot"]["nodes"].as_array_mut().unwrap() {
            n["properties"][0]["field"] = json!(kind);
            n["properties"][0]["state"]["value"]["value"]["frame_kind"] = json!(kind);
        }
        for a in d["expectation"]["rule"]["anchors"].as_array_mut().unwrap() {
            a["frame_kind"] = json!(kind);
        }
        validate(&v).expect("matching property/frame kind remains valid");
        geometry(&mut v, 0)["frame_kind"] = json!(if kind == "layout_bounds" {
            "accessibility_bounds"
        } else {
            "layout_bounds"
        });
        assert_eq!(validate(&v), Err(ValidationError::ValueType), "{kind}");
    }
}

#[test]
fn delta_oracle_checks_unchanged_nodes_not_only_upserts() {
    let mut v = fixture("ENV-DELTA-VALID");
    validate(&v).expect("authored full replacement");
    v["artifact"]["data"]["update"]["upsert"] = json!([]);
    assert_eq!(validate(&v), Err(ValidationError::InvalidEvidence));
    let d = &mut v["artifact"]["data"];
    let mut source = d["base"].clone();
    source["id"] = json!("unchanged-full");
    source["revision"] = d["update"]["revision"].clone();
    source["source_state"] = d["update"]["source_state"].clone();
    source["observations"].as_array_mut().unwrap().extend(
        d["update"]["observations"]
            .as_array()
            .unwrap()
            .iter()
            .cloned(),
    );
    d["source_snapshot"] = source;
    validate(&v).expect("empty upsert with genuinely unchanged selected nodes");
    v["artifact"]["data"]["source_snapshot"]["context"]["fields"]
        .as_array_mut()
        .unwrap()
        .reverse();
    validate(&v).expect("canonical context field-set ordering is immaterial");
}

#[test]
fn delta_oracle_checks_revision_focus_relations_coverage_and_metadata() {
    let original = fixture("ENV-DELTA-VALID");
    for case in [
        "revision",
        "focus",
        "relations",
        "coverage",
        "components",
        "observations",
        "context",
        "source_state",
    ] {
        let mut v = original.clone();
        let s = &mut v["artifact"]["data"]["source_snapshot"];
        let key = s["nodes"][0]["key"].clone();
        let evidence = s["nodes"][0]["properties"][0]["evidence"].clone();
        match case {
            "revision" => s["revision"] = json!(999),
            "focus" => {
                s["focus"]["keyboard"] = json!({"status":"known","target":key,"evidence":evidence})
            }
            "relations" => {
                s["relations"] =
                    json!([{"kind":"labelled_by","from":key,"to":key,"evidence":evidence}])
            }
            "coverage" => {
                s["coverage"]["status"] = json!("partial");
                s["coverage"]["unknown_count"] = json!(1);
            }
            "components" => {
                s["components"] = json!([{"logical_component_key":"unexplained","members":[key],"declaration_source":"authored","provenance":"reported"}])
            }
            "observations" => s["observations"][0]["start"] = json!(119),
            "context" => s["context"]["environment_revision"] = json!("other-environment"),
            "source_state" => s["source_state"] = json!("another-source"),
            _ => unreachable!(),
        }
        assert_eq!(
            validate(&v),
            Err(ValidationError::InvalidEvidence),
            "{case}"
        );
    }
}

#[test]
fn delta_oracle_accounts_for_removed_added_and_extra_nodes() {
    let mut v = fixture("ENV-DELTA-VALID");
    let d = &mut v["artifact"]["data"];
    let mut removed = d["base"]["nodes"][0].clone();
    removed["key"]["key"] = json!("removed-node");
    d["base"]["nodes"]
        .as_array_mut()
        .unwrap()
        .push(removed.clone());
    d["update"]["removed"] = json!([{"key":removed["key"],"evidence":d["update"]["upsert"][0]["properties"][0]["evidence"]}]);
    validate(&v).expect("justified removal yields exact source graph");
    let valid = v.clone();
    let d = &mut v["artifact"]["data"];
    d["source_snapshot"]["nodes"]
        .as_array_mut()
        .unwrap()
        .push(removed);
    let historical = d["base"]["observations"][0].clone();
    d["source_snapshot"]["observations"]
        .as_array_mut()
        .unwrap()
        .push(historical);
    assert_eq!(validate(&v), Err(ValidationError::InvalidEvidence));
    let mut v = valid;
    let d = &mut v["artifact"]["data"];
    let mut added = d["update"]["upsert"][0].clone();
    added["key"]["key"] = json!("new-node");
    d["update"]["upsert"]
        .as_array_mut()
        .unwrap()
        .push(added.clone());
    assert_eq!(validate(&v), Err(ValidationError::InvalidEvidence));
    v["artifact"]["data"]["source_snapshot"]["nodes"]
        .as_array_mut()
        .unwrap()
        .push(added.clone());
    validate(&v).expect("explicit new node is represented in the source");
    added["key"]["key"] = json!("unexplained-extra");
    v["artifact"]["data"]["source_snapshot"]["nodes"]
        .as_array_mut()
        .unwrap()
        .push(added);
    assert_eq!(validate(&v), Err(ValidationError::InvalidEvidence));
}

fn add_observation(v: &mut Value, id: &str, namespace: &str) {
    let observations = v["artifact"]["data"]["snapshot"]["observations"]
        .as_array_mut()
        .unwrap();
    let mut o = observations[0].clone();
    o["id"] = json!(id);
    o["source_namespace"] = json!(namespace);
    observations.push(o);
}
fn enabled(v: &mut Value) -> &mut Value {
    v["artifact"]["data"]["snapshot"]["nodes"][0]["properties"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|p| p["field"] == "enabled")
        .unwrap()
}

#[test]
fn current_ref_cannot_mask_stale_resolution_or_enabled_evidence() {
    let mut v = fixture("ENV-ACTION-VALID");
    validate(&v).expect("authored current action");
    add_observation(&mut v, "fresh-ref", "macos.ax");
    v["artifact"]["data"]["snapshot"]["observations"][0]["freshness"] = json!("stale");
    v["artifact"]["data"]["action"]["backend_ref"]["observation_id"] = json!("fresh-ref");
    enabled(&mut v)["evidence"]["observation_id"] = json!("fresh-ref");
    assert_eq!(
        validate(&v),
        Err(ValidationError::StaleTarget),
        "resolution covers writable/value_allowed/intents"
    );
    v["artifact"]["data"]["action"]["resolution"]["evidence"]["observation_id"] =
        json!("fresh-ref");
    validate(&v).expect("all required proof now current");
    enabled(&mut v)["evidence"]["observation_id"] = json!("O10");
    assert_eq!(
        validate(&v),
        Err(ValidationError::StaleTarget),
        "enabled remains a required precondition"
    );
}

#[test]
fn action_evidence_is_bound_to_the_ref_source_and_covered_property() {
    let mut v = fixture("ENV-ACTION-VALID");
    add_observation(&mut v, "other-current", "other.backend");
    v["artifact"]["data"]["action"]["resolution"]["evidence"]["observation_id"] =
        json!("other-current");
    v["artifact"]["data"]["action"]["resolution"]["evidence"]["source_namespace"] =
        json!("other.backend");
    assert_eq!(validate(&v), Err(ValidationError::StaleTarget));
    let mut v = fixture("ENV-ACTION-VALID");
    add_observation(&mut v, "property-current", "macos.ax");
    enabled(&mut v)["evidence"]["observation_id"] = json!("property-current");
    validate(&v).expect("separately observed current property remains node-bound");
    v["artifact"]["data"]["snapshot"]["observations"][1]["coverage"]["fields"] = json!(["role"]);
    assert_eq!(validate(&v), Err(ValidationError::StaleTarget));
}
