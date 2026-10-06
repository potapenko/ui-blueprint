use uiblueprint_engine::replay::{ReplayError, ResyncReason, replay};
use uiblueprint_schema::{
    model::*,
    validation::{self, ValidationError},
};

fn fixture(name: &str) -> DeltaCase {
    let path = format!(
        "{}/../../fixtures/golden/{name}.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let document: Document =
        serde_json::from_slice(&std::fs::read(path).expect("independent canonical fixture"))
            .expect("canonical document");
    let Artifact::Delta(case) = document.artifact else {
        panic!("delta fixture")
    };
    *case
}
fn result_id() -> Id {
    Id("S11".into())
}
fn evidence(observation: &Observation) -> Evidence {
    Evidence {
        observation_id: observation.id.clone(),
        source_namespace: observation.source_namespace.clone(),
        provenance: Provenance::Reported,
        method: Id("authored_replay_fixture".into()),
        uncertainty: None,
    }
}
fn set_field(node: &mut Node, field: Field, state: Availability) {
    let property = node
        .properties
        .iter_mut()
        .find(|p| p.field() == field)
        .expect("requested field");
    let Property::Requested { state: current, .. } = property else {
        panic!("requested")
    };
    *current = state;
}
fn stamped(mut node: Node, observation: &Observation) -> Node {
    for property in &mut node.properties {
        if let Property::Requested { evidence, .. } = property {
            evidence.observation_id = observation.id.clone();
        }
    }
    node
}
fn unchanged_on_error(base: &Snapshot, delta: &Delta, expected: ReplayError) {
    let before = serde_json::to_vec(base).expect("base");
    assert_eq!(replay(base, delta, result_id()), Err(expected));
    assert_eq!(
        serde_json::to_vec(base).expect("base after refusal"),
        before
    );
}

#[test]
fn independent_full_delta_oracles_match_exactly_at_one_recorded_checkpoint() {
    for name in ["ENV-DELTA-VALID", "DELTA-UNKNOWN-REPLACES-KNOWN"] {
        let case = fixture(name);
        let expected = case
            .source_snapshot
            .as_ref()
            .expect("independently supplied full source snapshot");
        let output = replay(&case.base, &case.update, expected.id.clone()).expect("replay");
        assert_eq!(&output, expected, "{name}");
        assert_eq!(
            replay(&case.base, &case.update, expected.id.clone()).expect("deterministic replay"),
            output
        );
        validation::validate_snapshot(&output).expect("canonical resulting snapshot");
    }
}
#[test]
fn full_replacement_preserves_false_empty_unknown_unsupported_redacted() {
    for state in [
        Availability::Known {
            value: Value::Flag(false),
        },
        Availability::Unknown {
            reason: Id("read_failed".into()),
        },
        Availability::Unsupported {
            reason: Id("not_exposed".into()),
        },
        Availability::Redacted {},
    ] {
        let mut case = fixture("ENV-DELTA-VALID");
        case.base.nodes[0]
            .source_declarations
            .push(SourceDeclaration {
                namespace: Id("fixture".into()),
                name: Id("old".into()),
                state: Availability::Known {
                    value: Value::Text("old declaration".into()),
                },
                sensitivity: Sensitivity::Public,
                source: Id("fixture".into()),
            });
        set_field(&mut case.update.upsert[0], Field::Checked, state.clone());
        set_field(
            &mut case.update.upsert[0],
            Field::Name,
            Availability::Known {
                value: Value::Text(String::new()),
            },
        );
        let output = replay(&case.base, &case.update, result_id()).expect("whole replacement");
        assert_eq!(output.nodes[0], case.update.upsert[0]);
        assert!(output.nodes[0].source_declarations.is_empty());
        assert_eq!(
            output.nodes[0]
                .properties
                .iter()
                .find(|p| p.field() == Field::Name)
                .and_then(Property::known),
            Some(&Value::Text(String::new()))
        );
        assert_eq!(output.observations, case.update.observations);
        assert_eq!(
            case.base.nodes[0]
                .properties
                .iter()
                .find(|p| p.field() == Field::Checked)
                .and_then(Property::known),
            Some(&Value::Flag(false))
        );
    }
}
#[test]
fn every_representable_context_dimension_and_revision_requires_resync() {
    for dimension in 0..9 {
        let mut case = fixture("ENV-DELTA-VALID");
        let context = &mut case.update.context;
        match dimension {
            0 => context.session_id = Id("different-session".into()),
            1 => context.target.id = Id("different-target".into()),
            2 => context.target.generation = Id("different-generation".into()),
            3 => context.surfaces[0].generation = Id("different-surface-generation".into()),
            4 => context.scope_id = Id("different-scope".into()),
            5 => context.projection = Projection::Design,
            6 => context.fields = vec![Field::Role],
            7 => context.plugin.version = Id("different-plugin-version".into()),
            _ => context.environment_revision = Id("different-display".into()),
        }
        unchanged_on_error(
            &case.base,
            &case.update,
            ReplayError::ResyncRequired(ResyncReason::ContextMismatch),
        );
    }
    for (base_revision, revision) in [(9, 11), (10, 12), (10, 10)] {
        let mut case = fixture("ENV-DELTA-VALID");
        case.update.base_revision = base_revision;
        case.update.revision = revision;
        unchanged_on_error(
            &case.base,
            &case.update,
            ReplayError::ResyncRequired(ResyncReason::RevisionMismatch),
        );
    }
    let mut case = fixture("ENV-DELTA-VALID");
    case.base.revision = u64::MAX;
    case.update.base_revision = u64::MAX;
    case.update.revision = 0;
    unchanged_on_error(
        &case.base,
        &case.update,
        ReplayError::ResyncRequired(ResyncReason::RevisionMismatch),
    );
}
#[test]
fn omitted_requested_property_rejects_without_mutating_base() {
    let mut case = fixture("ENV-DELTA-VALID");
    case.update.upsert[0]
        .properties
        .retain(|p| p.field() != Field::Checked);
    unchanged_on_error(
        &case.base,
        &case.update,
        ReplayError::InvalidInput(ValidationError::MissingProperty),
    );
}

fn reparent_case() -> DeltaCase {
    let mut case = fixture("ENV-DELTA-VALID");
    let template = case.base.nodes[0].clone();
    let key = |name: &str| SourceKey {
        namespace: template.key.namespace.clone(),
        key: Id(name.into()),
    };
    let mut nodes = Vec::new();
    for name in ["root", "a", "b", "c"] {
        let mut node = template.clone();
        node.key = key(name);
        node.children = match name {
            "root" => vec![key("a"), key("b")],
            "a" => vec![key("c")],
            _ => vec![],
        };
        nodes.push(node);
    }
    case.base.nodes = nodes;
    case.base.focus.keyboard = FocusRef::Known {
        target: key("c"),
        evidence: evidence(&case.base.observations[0]),
    };
    let mut root = stamped(case.base.nodes[0].clone(), &case.update.observations[0]);
    root.children = vec![key("b")];
    let mut b = stamped(case.base.nodes[2].clone(), &case.update.observations[0]);
    b.children = vec![key("c")];
    case.update.upsert = vec![root, b];
    case.update.removed = vec![Removal {
        key: key("a"),
        evidence: evidence(&case.update.observations[0]),
    }];
    case.update.focus.keyboard = FocusRef::Known {
        target: key("c"),
        evidence: evidence(&case.update.observations[0]),
    };
    case.source_snapshot = None;
    case
}
#[test]
fn r03_u2_reparenting_keeps_child_and_publishes_children_relations_focus_atomically() {
    let mut case = reparent_case();
    case.update.relations = vec![Relation {
        kind: RelationKind::Owns,
        from: case.base.nodes[2].key.clone(),
        to: case.base.nodes[3].key.clone(),
        evidence: evidence(&case.update.observations[0]),
    }];
    let output = replay(&case.base, &case.update, result_id()).expect("reparenting");
    assert_eq!(
        output
            .nodes
            .iter()
            .map(|n| n.key.key.0.as_str())
            .collect::<Vec<_>>(),
        vec!["root", "b", "c"]
    );
    assert_eq!(
        output.nodes[0].children,
        vec![case.base.nodes[2].key.clone()]
    );
    assert_eq!(
        output.nodes[1].children,
        vec![case.base.nodes[3].key.clone()]
    );
    assert_eq!(output.nodes[2], case.base.nodes[3]);
    assert_eq!(output.focus, case.update.focus);
    assert_eq!(output.relations, case.update.relations);
    assert_eq!(
        output.observations,
        vec![
            case.update.observations[0].clone(),
            case.base.observations[0].clone()
        ]
    );
    assert_eq!(case.base.nodes.len(), 4);
    let mut bad = case.update.clone();
    bad.upsert.remove(0);
    unchanged_on_error(
        &case.base,
        &bad,
        ReplayError::ResyncRequired(ResyncReason::UnresolvedReference),
    );
    let mut bad = case.update.clone();
    bad.focus.keyboard = FocusRef::Known {
        target: case.base.nodes[1].key.clone(),
        evidence: evidence(&bad.observations[0]),
    };
    unchanged_on_error(
        &case.base,
        &bad,
        ReplayError::ResyncRequired(ResyncReason::UnresolvedReference),
    );
    let mut bad = case.update.clone();
    bad.relations[0].to = case.base.nodes[1].key.clone();
    unchanged_on_error(
        &case.base,
        &bad,
        ReplayError::ResyncRequired(ResyncReason::UnresolvedReference),
    );
}
#[test]
fn independent_removal_oracles_and_partial_absence_never_imply_deletion() {
    let valid = fixture("DELTA-JUSTIFIED-REMOVAL");
    let output = replay(&valid.base, &valid.update, result_id()).expect("justified removal");
    assert_eq!(output.nodes.len(), 1);
    assert!(output.nodes[0].children.is_empty());
    assert!(output.relations.is_empty());
    let invalid = fixture("DELTA-ATOMIC-RELATIONS");
    unchanged_on_error(
        &invalid.base,
        &invalid.update,
        ReplayError::ResyncRequired(ResyncReason::UnresolvedReference),
    );
    let mut partial = valid.update.clone();
    partial.observations[0].coverage.status = CoverageStatus::Partial;
    unchanged_on_error(
        &valid.base,
        &partial,
        ReplayError::InvalidInput(ValidationError::InvalidCoverage),
    );
    for status in [
        CoverageStatus::Complete,
        CoverageStatus::Partial,
        CoverageStatus::Unknown,
    ] {
        let mut case = fixture("ENV-DELTA-VALID");
        case.update.upsert.clear();
        case.update.removed.clear();
        case.update.coverage.status = status;
        let output = replay(&case.base, &case.update, result_id()).expect("absence is not removal");
        assert_eq!(output.nodes, case.base.nodes);
        assert_eq!(output.coverage.status, status);
        assert!(output.observations.contains(&case.base.observations[0]));
    }
}
#[test]
fn conflicting_observation_identity_cannot_restamp_retained_facts() {
    let mut case = fixture("ENV-DELTA-VALID");
    let mut collision = case.base.observations[0].clone();
    collision.end += 1.0;
    case.update.observations.push(collision);
    unchanged_on_error(
        &case.base,
        &case.update,
        ReplayError::ResyncRequired(ResyncReason::ObservationConflict),
    );
    case.update.observations.pop();
    case.update
        .observations
        .push(case.base.observations[0].clone());
    assert!(replay(&case.base, &case.update, result_id()).is_ok());
}
#[test]
fn retained_metadata_references_require_full_snapshot_instead_of_silent_cleanup() {
    let mut case = reparent_case();
    case.base.components.push(ComponentMapping {
        logical_component_key: Id("component".into()),
        members: vec![case.base.nodes[1].key.clone()],
        declaration_source: Id("fixture".into()),
        provenance: Provenance::Reported,
    });
    unchanged_on_error(
        &case.base,
        &case.update,
        ReplayError::ResyncRequired(ResyncReason::UnresolvedReference),
    );
    case.base.components.clear();
    case.base.surface_records.push(SurfaceRecord {
        identity: case.base.context.surfaces[0].clone(),
        native_owner: Availability::Unknown {
            reason: Id("not_observed".into()),
        },
        initiated_by: None,
        anchor: Some(case.base.nodes[1].key.clone()),
        evidence: evidence(&case.base.observations[0]),
    });
    unchanged_on_error(
        &case.base,
        &case.update,
        ReplayError::ResyncRequired(ResyncReason::UnresolvedReference),
    );
}
#[test]
fn retained_capture_keeps_original_observation_and_is_not_refreshed_by_semantics() {
    let mut case = fixture("ENV-DELTA-VALID");
    let mut capture_observation = case.base.observations[0].clone();
    capture_observation.id = Id("C10".into());
    capture_observation.channel = Channel::RenderedCapture;
    case.base.observations.push(capture_observation.clone());
    case.base.captures.push(CaptureFrame {
        observation_id: capture_observation.id.clone(),
        capture_target: case.base.context.surfaces[0].clone(),
        capture_kind: CaptureKind::WindowIsolated,
        pixel_width: 100,
        pixel_height: 100,
        crop_transform: TransformState::LocalOnly {},
        included_surfaces: vec![case.base.context.surfaces[0].clone()],
        excluded_surfaces: vec![],
        unresolved_surfaces: vec![],
        surface_coverage: CoverageStatus::Complete,
        captures_audio: false,
        filter_id: Id("owned-fixture".into()),
        payload_ref: Id("historical-frame-reference-not-loaded".into()),
    });
    let output = replay(&case.base, &case.update, result_id()).expect("semantic update");
    assert_eq!(output.captures, case.base.captures);
    assert_eq!(
        output.observations,
        vec![case.update.observations[0].clone(), capture_observation]
    );
    assert_eq!(output.source_state, case.update.source_state);
}
#[test]
fn caller_supplies_a_distinct_bounded_snapshot_id() {
    let case = fixture("ENV-DELTA-VALID");
    for id in [case.base.id.clone(), Id(String::new()), Id("x".repeat(257))] {
        assert_eq!(
            replay(&case.base, &case.update, id),
            Err(ReplayError::InvalidSnapshotId)
        );
    }
}

#[test]
fn new_nodes_append_in_delta_order_and_complete_empty_requires_explicit_removal() {
    let mut case = fixture("ENV-DELTA-VALID");
    let mut first = case.update.upsert[0].clone();
    first.key.key = Id("new-z".into());
    let mut second = first.clone();
    second.key.key = Id("new-a".into());
    case.update.upsert = vec![first.clone(), second.clone()];
    let output = replay(&case.base, &case.update, result_id()).expect("new nodes");
    assert_eq!(
        output.nodes,
        vec![case.base.nodes[0].clone(), first, second]
    );
    assert_eq!(
        output.observations,
        vec![
            case.update.observations[0].clone(),
            case.base.observations[0].clone()
        ]
    );

    let mut case = fixture("ENV-DELTA-VALID");
    case.update.upsert.clear();
    case.update.removed = vec![Removal {
        key: case.base.nodes[0].key.clone(),
        evidence: evidence(&case.update.observations[0]),
    }];
    let mut expected = case.source_snapshot.take().expect("full fixture");
    expected.nodes.clear();
    assert_eq!(
        replay(&case.base, &case.update, expected.id.clone())
            .expect("explicitly confirmed empty scope"),
        expected
    );
    assert_eq!(case.base.nodes.len(), 1);
}
