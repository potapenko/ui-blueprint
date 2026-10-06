use uiblueprint_export::*;
use uiblueprint_schema::model::*;

fn limits() -> ExportLimits {
    ExportLimits {
        max_input_bytes: 1_000_000,
        max_output_bytes: 1_000_000,
        max_components: 10,
        max_views: 2,
        components_per_detail: 10,
    }
}
fn brief(
    origin: Origin,
    a: Rect,
    b: Rect,
    edges: [(&str, Edge); 2],
    value: Option<f64>,
) -> DrawingBrief {
    let space = Space {
        id: Id("test-local".into()),
        kind: SpaceKind::Local,
        units: Unit::CssPx,
        origin,
    };
    let requirement = "E02 explicit synthetic geometry".to_owned();
    DrawingBrief {
        metadata: Metadata {
            document_id: "E02".into(),
            revision: "1".into(),
            title: "Proposal arithmetic regression".into(),
            audience: "Engineering".into(),
            language: "en".into(),
            date: "2026-10-06".into(),
            owner: "E02 test".into(),
            retention: "Test duration".into(),
            specification_refs: vec!["UIB.DRAWING@1.1".into()],
            approval: Approval {
                status: ApprovalStatus::Draft,
                named_record: None,
            },
            page_format: "A3".into(),
            output_size: "1920 x 1080".into(),
        },
        purpose: Purpose::Propose,
        views: vec![ViewInput {
            id: "proposal".into(),
            title: "Two rectangles".into(),
            state: "proposed".into(),
            scope: "Two explicit test rectangles".into(),
            environment: "Synthetic test only".into(),
            safe_source_reference: requirement.clone(),
            not_depicted: vec!["No runtime or pixels".into()],
            source: SourceInput::Proposed {
                layout: ProposedLayout {
                    requirements: vec![requirement.clone()],
                    components: [("A", a), ("B", b)]
                        .into_iter()
                        .map(|(id, r)| ProposedComponent {
                            id: id.into(),
                            parent: None,
                            role: Role::Group,
                            label: id.into(),
                            geometry: Geometry {
                                frame_kind: FrameKind::LayoutBounds,
                                coordinate_space: space.clone(),
                                shape: Shape::Rect(r),
                                transform: TransformState::LocalOnly {},
                            },
                            state_and_actions: "Proposed; actions unknown".into(),
                        })
                        .collect(),
                    dimensions: vec![Dimension {
                        id: "distance".into(),
                        label: "Authored edge distance".into(),
                        anchors: edges.map(|(id, edge)| DimensionAnchor {
                            component: id.into(),
                            frame_kind: FrameKind::LayoutBounds,
                            space: space.clone(),
                            edge,
                        }),
                        value,
                        units: Unit::CssPx,
                        source_kind: SourceKind::Proposed,
                        evidence: vec![],
                        requirement_ref: Some(requirement),
                        unknown_reason: value.is_none().then(|| "Explicit unknown".into()),
                        check_tolerance: None,
                    }],
                    chains: vec![],
                    unknowns: vec!["radius unknown".into()],
                },
            },
        }],
        details: vec![],
        comparisons: vec![],
        transitions: vec![],
    }
}
fn rect(x: f64, y: f64, width: f64, height: f64) -> Rect {
    Rect {
        x,
        y,
        width,
        height,
    }
}
fn layout(b: &mut DrawingBrief) -> &mut ProposedLayout {
    let SourceInput::Proposed { layout } = &mut b.views[0].source else {
        panic!()
    };
    layout
}
#[test]
fn vertical_edges_honor_both_origins_and_both_directions() {
    for (origin, edges, want, wrong) in [
        (Origin::BottomLeft, [Edge::Top, Edge::Bottom], 20.0, 50.0),
        (Origin::BottomLeft, [Edge::Bottom, Edge::Top], 50.0, 20.0),
        (Origin::TopLeft, [Edge::Top, Edge::Bottom], 50.0, 20.0),
        (Origin::TopLeft, [Edge::Bottom, Edge::Top], 20.0, 50.0),
    ] {
        for reverse in [false, true] {
            let mut ends = [("A", edges[0]), ("B", edges[1])];
            if reverse {
                ends.reverse();
            }
            let mut b = brief(
                origin,
                rect(0.0, 0.0, 5.0, 10.0),
                rect(0.0, 30.0, 5.0, 20.0),
                ends,
                Some(want),
            );
            assert!(
                compile(&b, limits()).is_ok(),
                "{origin:?} reverse={reverse}"
            );
            layout(&mut b).dimensions[0].value = Some(wrong);
            assert_eq!(
                compile(&b, limits()).unwrap_err(),
                ExportError::InvalidGeometry
            );
        }
    }
}
#[test]
fn fractional_and_large_origin_extents_remain_exact_without_coordinate_epsilon() {
    for (x, width) in [
        (0.2, 0.1),
        (1e-12, 1e-14),
        (-1e12, 0.125),
        (1e16, 1.0),
        (0.0, f64::from_bits(1)),
    ] {
        for reverse in [false, true] {
            let mut edges = [("A", Edge::Left), ("A", Edge::Right)];
            if reverse {
                edges.reverse();
            }
            let mut b = brief(
                Origin::TopLeft,
                rect(x, 0.0, width, 1.0),
                rect(0.0, 0.0, 1.0, 1.0),
                edges,
                Some(width),
            );
            let p = compile(&b, limits()).expect("authored fractional extent");
            let dims: serde_json::Value =
                serde_json::from_slice(&p.files()["dimensions.json"]).unwrap();
            assert_eq!(dims[0]["dimensions"][0]["value"].as_f64(), Some(width));
            layout(&mut b).dimensions[0].value = Some(width.next_up());
            assert_eq!(
                compile(&b, limits()).unwrap_err(),
                ExportError::InvalidGeometry,
                "adjacent wrong value at x={x}"
            );
        }
    }
}
#[test]
fn rounding_enclosure_is_directional_and_does_not_add_measurement_tolerance() {
    let mut b = brief(
        Origin::TopLeft,
        rect(0.0, 0.0, 1.0, 1.0),
        rect(0.2, 0.0, 0.1, 1.0),
        [("A", Edge::Left), ("B", Edge::Right)],
        Some(0.3),
    );
    assert!(compile(&b, limits()).is_ok());
    // The exact binary sum lies between 0.3 and its next representable value;
    // the following value lies outside that one operation's rounding interval.
    layout(&mut b).dimensions[0].value = Some(0.3_f64.next_up().next_up());
    layout(&mut b).dimensions[0].check_tolerance = Some(100.0);
    assert_eq!(
        compile(&b, limits()).unwrap_err(),
        ExportError::InvalidGeometry
    );
    layout(&mut b).dimensions[0].value = Some(0.3_f64.next_down());
    assert_eq!(
        compile(&b, limits()).unwrap_err(),
        ExportError::InvalidGeometry
    );
}
#[test]
fn mixed_origins_reject_and_centers_and_unknowns_keep_their_meaning() {
    let mut b = brief(
        Origin::BottomLeft,
        rect(0.0, 0.0, 1.0, 10.0),
        rect(0.0, 30.0, 1.0, 20.0),
        [("A", Edge::CenterY), ("B", Edge::CenterY)],
        Some(35.0),
    );
    assert!(compile(&b, limits()).is_ok());
    layout(&mut b).dimensions[0].anchors[1].space.origin = Origin::TopLeft;
    assert_eq!(
        compile(&b, limits()).unwrap_err(),
        ExportError::InvalidGeometry
    );
    layout(&mut b).dimensions[0].anchors[1].space.origin = Origin::BottomLeft;
    layout(&mut b).dimensions[0].value = None;
    layout(&mut b).dimensions[0].unknown_reason = Some("Explicit unknown".into());
    let p = compile(&b, limits()).unwrap();
    let manifest: serde_json::Value = serde_json::from_slice(&p.files()["manifest.json"]).unwrap();
    assert_eq!(manifest["source_kinds"], serde_json::json!(["proposed"]));
    assert_eq!(manifest["approval_status"], "draft");
    assert_eq!(manifest["validation_status"], "unverified");
}
#[test]
fn chain_arithmetic_tolerance_remains_explicit_and_separate() {
    let mut b = brief(
        Origin::TopLeft,
        rect(0.0, 0.0, 0.1, 1.0),
        rect(0.0, 0.0, 0.1001, 1.0),
        [("A", Edge::Left), ("A", Edge::Right)],
        Some(0.1),
    );
    let p = layout(&mut b);
    let mut total = p.dimensions[0].clone();
    total.id = "total".into();
    total.value = Some(0.1001);
    for a in &mut total.anchors {
        a.component = "B".into();
    }
    p.dimensions.push(total);
    p.chains.push(DimensionChain {
        terms: vec!["distance".into()],
        total: "total".into(),
        arithmetic_tolerance: 0.0,
    });
    assert_eq!(
        compile(&b, limits()).unwrap_err(),
        ExportError::InvalidChain
    );
    layout(&mut b).chains[0].arithmetic_tolerance = 0.0002;
    assert!(compile(&b, limits()).is_ok());
}

#[test]
fn finite_endpoint_fallback_and_unknown_overflow_do_not_invent_dimensions() {
    let b = brief(
        Origin::TopLeft,
        rect(-1e308, 0.0, 1e308, 1.0),
        rect(1e308, 0.0, 0.0, 1.0),
        [("A", Edge::Right), ("B", Edge::Left)],
        Some(1e308),
    );
    assert!(compile(&b, limits()).is_ok());
    let b = brief(
        Origin::TopLeft,
        rect(-1e308, 0.0, 0.0, 1.0),
        rect(1e308, 0.0, 0.0, 1.0),
        [("A", Edge::Left), ("B", Edge::Left)],
        None,
    );
    assert!(compile(&b, limits()).is_ok());
}

#[test]
fn finite_cancellation_requires_exact_distance_in_both_directions() {
    for reverse in [false, true] {
        let mut edges = [("A", Edge::Right), ("B", Edge::Left)];
        if reverse {
            edges.reverse();
        }
        let mut b = brief(
            Origin::TopLeft,
            rect(-1e16, 0.0, 1e16, 1.0),
            rect(1.0, 0.0, 0.0, 1.0),
            edges,
            Some(1.0),
        );
        assert!(compile(&b, limits()).is_ok());
        for wrong in [0.0, 2.0] {
            layout(&mut b).dimensions[0].value = Some(wrong);
            assert_eq!(
                compile(&b, limits()).unwrap_err(),
                ExportError::InvalidGeometry
            );
        }
    }
}
