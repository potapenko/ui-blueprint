use uiblueprint_engine::scope::{
    NeighborLimits, RelationDirection as Direction, ScopeError, relation_neighbors,
};
use uiblueprint_schema::{
    model::*,
    validation::{self, ValidationError},
};

fn key(namespace: &str, value: &str) -> SourceKey {
    SourceKey {
        namespace: Id(namespace.into()),
        key: Id(value.into()),
    }
}
fn fixture(keys: &[(&str, &str)]) -> Snapshot {
    let document = Document::from_json(
        include_bytes!("../../../fixtures/golden/ENV-SNAPSHOT-VALID.json"),
        65536,
    )
    .unwrap();
    let Artifact::Snapshot(mut snapshot) = document.artifact else {
        panic!("snapshot fixture")
    };
    let node = snapshot.nodes[0].clone();
    let observation = snapshot.observations[0].clone();
    snapshot.nodes.clear();
    snapshot.observations.clear();
    for (index, (namespace, name)) in keys.iter().enumerate() {
        let mut observed = observation.clone();
        observed.id = Id(format!("source-{index}"));
        observed.source_namespace = Id((*namespace).into());
        let mut item = node.clone();
        item.key = key(namespace, name);
        for property in &mut item.properties {
            if let Property::Requested { evidence, .. } = property {
                evidence.observation_id = observed.id.clone();
                evidence.source_namespace = observed.source_namespace.clone();
            }
        }
        snapshot.nodes.push(item);
        snapshot.observations.push(observed);
    }
    *snapshot
}
fn edge(snapshot: &Snapshot, from: usize, to: usize, kind: RelationKind) -> Relation {
    Relation {
        kind,
        from: snapshot.nodes[from].key.clone(),
        to: snapshot.nodes[to].key.clone(),
        evidence: Evidence {
            observation_id: snapshot.observations[from].id.clone(),
            source_namespace: snapshot.observations[from].source_namespace.clone(),
            provenance: Provenance::Reported,
            method: Id("explicit_fixture_relation".into()),
            uncertainty: None,
        },
    }
}
fn select(
    snapshot: &Snapshot,
    index: usize,
    max_relations: usize,
) -> uiblueprint_engine::scope::NeighborView<'_> {
    relation_neighbors(
        snapshot,
        &snapshot.nodes[index].key,
        NeighborLimits { max_relations },
    )
    .unwrap()
}

#[test]
fn labelled_by_and_error_for_preserve_both_directions_and_source_order() {
    let mut snapshot = fixture(&[
        ("web.dom", "control"),
        ("web.dom", "label"),
        ("web.dom", "error"),
    ]);
    snapshot.relations = vec![
        edge(&snapshot, 0, 1, RelationKind::LabelledBy),
        edge(&snapshot, 2, 0, RelationKind::ErrorFor),
        edge(&snapshot, 1, 2, RelationKind::Controls),
    ];
    let before = snapshot.clone();
    let view = select(&snapshot, 0, 10);
    assert_eq!(view.neighbors.len(), 2);
    assert_eq!(view.omitted_relations, 0);
    assert_eq!(view.neighbors[0].direction, Direction::Outgoing);
    assert_eq!(view.neighbors[0].counterpart.key.key.0, "label");
    assert_eq!(view.neighbors[1].direction, Direction::Incoming);
    assert_eq!(view.neighbors[1].counterpart.key.key.0, "error");
    assert!(std::ptr::eq(view.snapshot, &snapshot));
    assert!(std::ptr::eq(view.seed, &snapshot.nodes[0]));
    for (index, neighbor) in view.neighbors.iter().enumerate() {
        assert!(std::ptr::eq(neighbor.relation, &snapshot.relations[index]));
    }
    let label = select(&snapshot, 1, 10);
    assert_eq!(label.neighbors[0].direction, Direction::Incoming);
    assert_eq!(label.neighbors[0].relation.kind, RelationKind::LabelledBy);
    let error = select(&snapshot, 2, 10);
    assert_eq!(error.neighbors[0].direction, Direction::Outgoing);
    assert_eq!(error.neighbors[0].relation.kind, RelationKind::ErrorFor);
    assert_eq!(snapshot, before);
}

#[test]
fn many_to_many_links_keep_full_keys_evidence_and_component_declarations() {
    let mut snapshot = fixture(&[
        ("macos.ax", "shared"),
        ("macos.ax", "second"),
        ("probe", "shared"),
        ("probe", "text"),
    ]);
    snapshot.relations = vec![
        edge(&snapshot, 0, 2, RelationKind::Represents),
        edge(&snapshot, 0, 3, RelationKind::Represents),
        edge(&snapshot, 1, 3, RelationKind::Represents),
        edge(&snapshot, 2, 3, RelationKind::CorrespondsTo),
    ];
    snapshot.relations[3].evidence.provenance = Provenance::Estimated;
    snapshot.components.push(ComponentMapping {
        logical_component_key: Id("explicit-component".into()),
        members: snapshot.nodes.iter().map(|n| n.key.clone()).collect(),
        declaration_source: Id("synthetic_debug_mapping".into()),
        provenance: Provenance::Reported,
    });
    let view = select(&snapshot, 3, 10);
    let keys: Vec<_> = view
        .neighbors
        .iter()
        .map(|n| {
            (
                n.counterpart.key.namespace.0.as_str(),
                n.counterpart.key.key.0.as_str(),
            )
        })
        .collect();
    assert_eq!(
        keys,
        [
            ("macos.ax", "shared"),
            ("macos.ax", "second"),
            ("probe", "shared")
        ]
    );
    assert!(
        view.neighbors
            .iter()
            .all(|n| n.direction == Direction::Incoming)
    );
    assert_eq!(
        view.neighbors[2].relation.evidence.provenance,
        Provenance::Estimated
    );
    assert!(std::ptr::eq(
        view.neighbors[2].counterpart,
        &snapshot.nodes[2]
    ));
    assert_eq!(view.snapshot.components.len(), 1);
    assert_eq!(view.snapshot.nodes.len(), 4);
}

#[test]
fn equal_names_bounds_children_and_membership_do_not_invent_relations() {
    let mut snapshot = fixture(&[("web.dom", "a"), ("web.dom", "b")]);
    snapshot.context.fields.push(Field::LayoutBounds);
    snapshot.coverage.fields.push(Field::LayoutBounds);
    for (node, observation) in snapshot.nodes.iter_mut().zip(&mut snapshot.observations) {
        observation.coverage.fields.push(Field::LayoutBounds);
        node.properties.push(Property::Requested {
            field: Field::LayoutBounds,
            sensitivity: Sensitivity::Public,
            evidence: Evidence {
                observation_id: observation.id.clone(),
                source_namespace: observation.source_namespace.clone(),
                provenance: Provenance::Reported,
                method: Id("literal_fixture_rect".into()),
                uncertainty: None,
            },
            state: Availability::Known {
                value: Value::Geometry(Box::new(Geometry {
                    frame_kind: FrameKind::LayoutBounds,
                    coordinate_space: Space {
                        id: Id("fixture-local".into()),
                        kind: SpaceKind::Local,
                        units: Unit::CssPx,
                        origin: Origin::TopLeft,
                    },
                    shape: Shape::Rect(Rect {
                        x: 10.0,
                        y: 20.0,
                        width: 30.0,
                        height: 40.0,
                    }),
                    transform: TransformState::LocalOnly {},
                })),
            },
        });
    }
    snapshot.nodes[0].children = vec![snapshot.nodes[1].key.clone()];
    snapshot.components.push(ComponentMapping {
        logical_component_key: Id("same-group".into()),
        members: snapshot.nodes.iter().map(|n| n.key.clone()).collect(),
        declaration_source: Id("fixture".into()),
        provenance: Provenance::Reported,
    });
    let view = select(&snapshot, 0, usize::MAX);
    assert!(view.neighbors.is_empty());
    assert_eq!(view.omitted_relations, 0);
    assert_eq!(view.snapshot.nodes[0].children.len(), 1);
}

#[test]
fn truncation_does_not_change_source_coverage_projection_or_private_properties() {
    for status in [CoverageStatus::Partial, CoverageStatus::Unknown] {
        for projection in [Projection::Interaction, Projection::Design] {
            let mut snapshot = fixture(&[
                ("web.dom", "seed"),
                ("web.dom", "private"),
                ("web.dom", "label"),
            ]);
            snapshot.context.projection = projection;
            snapshot.coverage.status = status;
            snapshot.coverage.omitted_count = None;
            snapshot.coverage.unknown_count = None;
            snapshot.nodes[1].properties[1] = Property::Requested {
                field: Field::Name,
                sensitivity: Sensitivity::Sensitive,
                evidence: edge(&snapshot, 1, 0, RelationKind::DescribedBy).evidence,
                state: Availability::Redacted {},
            };
            snapshot.nodes[1]
                .source_declarations
                .push(SourceDeclaration {
                    namespace: Id("fixture".into()),
                    name: Id("declared".into()),
                    state: Availability::Unknown {
                        reason: Id("not_exposed".into()),
                    },
                    sensitivity: Sensitivity::Public,
                    source: Id("fixture_source".into()),
                });
            snapshot.relations = vec![
                edge(&snapshot, 0, 1, RelationKind::DescribedBy),
                edge(&snapshot, 0, 2, RelationKind::LabelledBy),
            ];
            let before = snapshot.clone();
            let view = select(&snapshot, 0, 1);
            assert_eq!(view.neighbors.len(), 1);
            assert_eq!(view.omitted_relations, 1);
            assert_eq!(view.snapshot.coverage.status, status);
            assert_eq!(view.snapshot.context.projection, projection);
            assert!(matches!(
                view.neighbors[0].counterpart.properties[1],
                Property::Requested {
                    state: Availability::Redacted {},
                    ..
                }
            ));
            let full = select(&snapshot, 0, 2);
            assert_eq!(full.omitted_relations, 0);
            assert_eq!(full.snapshot.coverage.status, status);
            assert_eq!(snapshot, before);
        }
    }
}

#[test]
fn zero_limit_self_loop_and_repeated_relations_have_exact_counts() {
    let mut snapshot = fixture(&[("web.dom", "seed")]);
    let relation = edge(&snapshot, 0, 0, RelationKind::CorrespondsTo);
    snapshot.relations = vec![relation.clone(), relation];
    let zero = select(&snapshot, 0, 0);
    assert!(zero.neighbors.is_empty());
    assert_eq!(zero.omitted_relations, 2);
    let one = select(&snapshot, 0, 1);
    assert_eq!(one.neighbors.len(), 1);
    assert_eq!(one.omitted_relations, 1);
    assert_eq!(one.neighbors[0].direction, Direction::SelfLoop);
    assert!(std::ptr::eq(one.neighbors[0].counterpart, one.seed));
    let both = select(&snapshot, 0, 2);
    assert_eq!(both.neighbors.len(), 2);
    assert_eq!(both.omitted_relations, 0);
}

#[test]
fn absent_seed_and_invalid_canonical_graph_refuse_without_partial_output() {
    let mut snapshot = fixture(&[("web.dom", "seed")]);
    assert!(matches!(
        relation_neighbors(
            &snapshot,
            &key("web.ax", "seed"),
            NeighborLimits { max_relations: 1 }
        ),
        Err(ScopeError::MissingSeed)
    ));
    let mut bad = edge(&snapshot, 0, 0, RelationKind::LabelledBy);
    bad.to = key("web.dom", "absent");
    snapshot.relations.push(bad);
    let before = snapshot.clone();
    assert_eq!(
        validation::validate_snapshot(&snapshot),
        Err(ValidationError::DanglingReference)
    );
    assert!(matches!(
        relation_neighbors(
            &snapshot,
            &snapshot.nodes[0].key,
            NeighborLimits { max_relations: 0 }
        ),
        Err(ScopeError::InvalidSnapshot(
            ValidationError::DanglingReference
        ))
    ));
    assert_eq!(snapshot, before);
}
