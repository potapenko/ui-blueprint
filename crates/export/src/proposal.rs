use crate::{
    ExportError as E, Result,
    types::*,
    validate::{identifier, require, text},
};
use std::collections::{BTreeMap, BTreeSet};
#[path = "proposal_arithmetic.rs"]
mod arithmetic;
use arithmetic::Interval;
use uiblueprint_schema::model::*;

pub(crate) fn validate(p: &ProposedLayout) -> Result<()> {
    require(
        !p.requirements.is_empty() && !p.components.is_empty(),
        E::InvalidSource,
    )?;
    for s in p.requirements.iter().chain(&p.unknowns) {
        text(s)?;
    }
    let mut objects = BTreeMap::new();
    for c in &p.components {
        require(
            identifier(&c.id) && objects.insert(&c.id, c).is_none(),
            E::InvalidReference,
        )?;
        text(&c.label)?;
        text(&c.state_and_actions)?;
        let g = &c.geometry;
        require(
            identifier(&g.coordinate_space.id.0)
                && matches!(g.transform, TransformState::LocalOnly {}),
            E::InvalidGeometry,
        )?;
        let Shape::Rect(r) = &g.shape else {
            return Err(E::InvalidGeometry);
        };
        require(
            [r.x, r.y, r.width, r.height, r.x + r.width, r.y + r.height]
                .iter()
                .all(|x| x.is_finite())
                && r.width >= 0.0
                && r.height >= 0.0,
            E::InvalidGeometry,
        )?;
    }
    for c in &p.components {
        let mut visited = BTreeSet::new();
        let mut next = Some(&c.id);
        while let Some(id) = next {
            require(visited.insert(id), E::InvalidReference)?;
            next = objects.get(id).ok_or(E::InvalidReference)?.parent.as_ref();
        }
    }
    let mut dimensions = BTreeMap::new();
    for d in &p.dimensions {
        require(
            identifier(&d.id) && dimensions.insert(&d.id, d).is_none(),
            E::InvalidReference,
        )?;
        text(&d.label)?;
        require(
            d.source_kind == SourceKind::Proposed && d.evidence.is_empty(),
            E::InvalidSource,
        )?;
        let requirement = d.requirement_ref.as_ref().ok_or(E::InvalidSource)?;
        require(p.requirements.contains(requirement), E::InvalidSource)?;
        require(
            d.check_tolerance.is_none_or(|x| x.is_finite() && x >= 0.0),
            E::InvalidGeometry,
        )?;
        let coords = d
            .anchors
            .iter()
            .map(|a| coordinate(a, &objects))
            .collect::<Result<Vec<_>>>()?;
        require(
            d.anchors[0].space == d.anchors[1].space
                && d.units == d.anchors[0].space.units
                && axis(d.anchors[0].edge) == axis(d.anchors[1].edge),
            E::InvalidGeometry,
        )?;
        match d.value {
            Some(v) => require(
                v.is_finite()
                    && v >= 0.0
                    && arithmetic::distance(coords[0], coords[1])
                        .is_some_and(|distance| distance.contains(v))
                    && d.unknown_reason.is_none(),
                E::InvalidGeometry,
            )?,
            None => {
                text(d.unknown_reason.as_deref().ok_or(E::InvalidGeometry)?)?;
            }
        }
    }
    for c in &p.chains {
        require(
            !c.terms.is_empty()
                && c.arithmetic_tolerance.is_finite()
                && c.arithmetic_tolerance >= 0.0
                && !c.terms.contains(&c.total),
            E::InvalidChain,
        )?;
        let total = dimensions.get(&c.total).ok_or(E::InvalidChain)?;
        let mut sum = 0.0;
        for id in &c.terms {
            let d = dimensions.get(id).ok_or(E::InvalidChain)?;
            require(
                d.units == total.units && d.anchors[0].space == total.anchors[0].space,
                E::InvalidChain,
            )?;
            sum += d.value.ok_or(E::InvalidChain)?;
        }
        require(
            sum.is_finite()
                && (sum - total.value.ok_or(E::InvalidChain)?).abs() <= c.arithmetic_tolerance,
            E::InvalidChain,
        )?;
    }
    Ok(())
}
fn axis(e: Edge) -> bool {
    matches!(e, Edge::Left | Edge::Right | Edge::CenterX)
}
fn coordinate(
    a: &DimensionAnchor,
    objects: &BTreeMap<&String, &ProposedComponent>,
) -> Result<(f64, Interval)> {
    let g = &objects
        .get(&a.component)
        .ok_or(E::InvalidReference)?
        .geometry;
    require(
        g.frame_kind == a.frame_kind && g.coordinate_space == a.space,
        E::InvalidGeometry,
    )?;
    let Shape::Rect(r) = &g.shape else {
        return Err(E::InvalidGeometry);
    };
    Ok(match a.edge {
        Edge::Left => (r.x, Interval::exact(0.0)),
        Edge::Right => (r.x, Interval::exact(r.width)),
        Edge::Top => (
            r.y,
            Interval::exact(if a.space.origin == Origin::BottomLeft {
                r.height
            } else {
                0.0
            }),
        ),
        Edge::Bottom => (
            r.y,
            Interval::exact(if a.space.origin == Origin::TopLeft {
                r.height
            } else {
                0.0
            }),
        ),
        Edge::CenterX => (r.x, Interval::half(r.width)),
        Edge::CenterY => (r.y, Interval::half(r.height)),
    })
}
pub(crate) fn components(p: &ProposedLayout) -> Vec<SceneComponent> {
    p.components
        .iter()
        .map(|c| SceneComponent {
            id: c.id.clone(),
            surface: None,
            source_key: None,
            native_role: None,
            parent: c.parent.clone(),
            children: p
                .components
                .iter()
                .filter(|child| child.parent.as_ref() == Some(&c.id))
                .map(|x| x.id.clone())
                .collect(),
            role: Some(c.role),
            label: Some(c.label.clone()),
            properties: vec![],
            geometry: Some(c.geometry.clone()),
            declarations: vec![],
            extensions: vec![],
            state_and_actions: Some(c.state_and_actions.clone()),
        })
        .collect()
}
