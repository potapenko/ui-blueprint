//! Literal recorded membership/property differences; never generated removals.
use uiblueprint_schema::{
    model::*,
    validation::{self, ValidationError},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiffLimits {
    /// Output entries only, not validation/work/RSS budget; no implicit default.
    pub max_entries: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Presence {
    Both,
    BeforeOnly,
    AfterOnly,
}
#[derive(Clone, Copy, Debug)]
pub enum Difference<'a> {
    /// Absence in a record is not proof of creation/deletion in the application.
    NodePresence {
        presence: Presence,
        before: Option<&'a Node>,
        after: Option<&'a Node>,
    },
    Property {
        before_node: &'a Node,
        after_node: &'a Node,
        field: Field,
        presence: Presence,
        before: Option<&'a Property>,
        after: Option<&'a Property>,
        content_changed: bool,
        evidence_changed: bool,
    },
}
#[derive(Debug)]
pub struct RecordedDiff<'a> {
    pub before: &'a Snapshot,
    pub after: &'a Snapshot,
    pub entries: Vec<Difference<'a>>,
    /// Differences omitted by max_entries; independent of both source coverages.
    pub omitted_entries: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiffError {
    InvalidSnapshot(ValidationError),
    IncompatibleContext,
    Capacity,
}
impl std::fmt::Display for DiffError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for DiffError {}

/// Compare exact source keys then fields, without mutating or cloning either graph.
/// Before node/property order comes first, followed by after-only fields/nodes in
/// their source order. Unchanged entries are omitted; source-only changes are not.
/// Node absence occupies one entry, not one entry per absent property.
///
/// This slice compares properties and node presence, not relations, children,
/// node metadata or full graph equality. Both original Snapshots remain available.
/// It does not infer chronology, atomicity, stable identity across remount, cause,
/// removal Evidence or a replayable Delta. A zero omitted count is not live coverage.
///
/// # Errors
/// Rejects invalid snapshots, incompatible source bindings or output allocation/count
/// failure. Existing canonical validation allocations are not an allocation-free
/// guarantee. No partial result is returned on failure.
pub fn compare_recorded<'a>(
    before: &'a Snapshot,
    after: &'a Snapshot,
    limits: DiffLimits,
) -> Result<RecordedDiff<'a>, DiffError> {
    validation::validate_snapshot(before).map_err(DiffError::InvalidSnapshot)?;
    validation::validate_snapshot(after).map_err(DiffError::InvalidSnapshot)?;
    // Recorded comparison spans layout/environment changes; Delta applicability
    // remains stricter. Both original environments/spaces remain in the result.
    if !sources_compatible(&before.context, &after.context) {
        return Err(DiffError::IncompatibleContext);
    }
    let mut total = Some(0usize);
    visit(before, after, |_| {
        total = total.and_then(|n| n.checked_add(1))
    });
    let total = total.ok_or(DiffError::Capacity)?;
    let count = total.min(limits.max_entries);
    let mut entries = Vec::new();
    entries
        .try_reserve_exact(count)
        .map_err(|_| DiffError::Capacity)?;
    visit(before, after, |entry| {
        if entries.len() < count {
            entries.push(entry);
        }
    });
    Ok(RecordedDiff {
        before,
        after,
        entries,
        omitted_entries: total - count,
    })
}

fn visit<'a>(before: &'a Snapshot, after: &'a Snapshot, mut emit: impl FnMut(Difference<'a>)) {
    for node in &before.nodes {
        let Some(other) = after.nodes.iter().find(|n| n.key == node.key) else {
            emit(Difference::NodePresence {
                presence: Presence::BeforeOnly,
                before: Some(node),
                after: None,
            });
            continue;
        };
        let fields = node.properties.iter().map(Property::field).chain(
            other
                .properties
                .iter()
                .filter(|p| !node.properties.iter().any(|old| old.field() == p.field()))
                .map(Property::field),
        );
        for field in fields {
            let old = node.properties.iter().find(|p| p.field() == field);
            let new = other.properties.iter().find(|p| p.field() == field);
            let content_changed = !same_content(old, new);
            let evidence_changed = !same_evidence(before, old, after, new);
            if content_changed || evidence_changed {
                emit(Difference::Property {
                    before_node: node,
                    after_node: other,
                    field,
                    presence: match (old, new) {
                        (Some(_), Some(_)) => Presence::Both,
                        (Some(_), None) => Presence::BeforeOnly,
                        _ => Presence::AfterOnly,
                    },
                    before: old,
                    after: new,
                    content_changed,
                    evidence_changed,
                });
            }
        }
    }
    for node in after
        .nodes
        .iter()
        .filter(|n| !before.nodes.iter().any(|old| old.key == n.key))
    {
        emit(Difference::NodePresence {
            presence: Presence::AfterOnly,
            before: None,
            after: Some(node),
        });
    }
}
fn same_content(a: Option<&Property>, b: Option<&Property>) -> bool {
    match (a, b) {
        (None, None)
        | (Some(Property::NotRequested { .. }), Some(Property::NotRequested { .. })) => true,
        (
            Some(Property::Requested {
                sensitivity: a,
                state: x,
                ..
            }),
            Some(Property::Requested {
                sensitivity: b,
                state: y,
                ..
            }),
        ) => a == b && same_state(x, y),
        _ => false,
    }
}
fn same_state(a: &Availability, b: &Availability) -> bool {
    match (a, b) {
        (
            Availability::Known {
                value: Value::Geometry(a),
            },
            Availability::Known {
                value: Value::Geometry(b),
            },
        ) => {
            a.frame_kind == b.frame_kind
                && a.coordinate_space == b.coordinate_space
                && a.shape == b.shape
                && same_transform(&a.transform, &b.transform)
        }
        _ => a == b,
    }
}
fn same_transform(a: &TransformState, b: &TransformState) -> bool {
    match (a, b) {
        (TransformState::Known { transform: a }, TransformState::Known { transform: b }) => {
            a.from == b.from
                && a.to == b.to
                && a.affine == b.affine
                && a.target == b.target
                && a.surface == b.surface
                && a.environment_revision == b.environment_revision
        }
        _ => a == b,
    }
}
fn sources(property: Option<&Property>) -> [Option<&Evidence>; 2] {
    match property {
        Some(Property::Requested {
            evidence, state, ..
        }) => {
            let transform = match state {
                Availability::Known {
                    value: Value::Geometry(g),
                } => match &g.transform {
                    TransformState::Known { transform } => Some(&transform.evidence),
                    _ => None,
                },
                _ => None,
            };
            [Some(evidence), transform]
        }
        _ => [None, None],
    }
}
fn same_evidence(
    before: &Snapshot,
    a: Option<&Property>,
    after: &Snapshot,
    b: Option<&Property>,
) -> bool {
    sources(a)
        .into_iter()
        .zip(sources(b))
        .all(|(a, b)| match (a, b) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                a == b
                    && before
                        .observations
                        .iter()
                        .find(|o| o.id == a.observation_id)
                        == after.observations.iter().find(|o| o.id == b.observation_id)
            }
            _ => false,
        })
}

fn sources_compatible(a: &Context, b: &Context) -> bool {
    !(a.schema_version != b.schema_version
        || a.session_id != b.session_id
        || a.target != b.target
        || a.scope_id != b.scope_id
        || a.projection != b.projection
        || a.plugin != b.plugin
        || a.surfaces.len() != b.surfaces.len()
        || a.surfaces
            .iter()
            .any(|surface| !b.surfaces.contains(surface))
        || a.fields.len() != b.fields.len()
        || a.fields.iter().any(|field| !b.fields.contains(field)))
}

/// Resolved factual frame on one side, retaining reached/contributing sources.
#[derive(Clone, Debug, PartialEq)]
pub enum ResolvedRect {
    Known {
        rect: Rect,
        evidence: Vec<Evidence>,
    },
    Unknown {
        reason: crate::UnknownReason,
        evidence: Vec<Evidence>,
    },
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RectDisplacement {
    pub dx: f64,
    pub dy: f64,
    pub dwidth: f64,
    pub dheight: f64,
}
/// Original records remain borrowed; numerical frames are in one explicit Space.
pub struct GeometryDifference<'a> {
    pub before: &'a Snapshot,
    pub after: &'a Snapshot,
    pub key: &'a SourceKey,
    pub frame_kind: FrameKind,
    pub before_evaluation: &'a uiblueprint_schema::analysis::EvaluationInput,
    pub after_evaluation: &'a uiblueprint_schema::analysis::EvaluationInput,
    pub before_geometry: ResolvedRect,
    pub after_geometry: ResolvedRect,
    pub displacement: Option<RectDisplacement>,
}
/// Compare one exact frame with independently bound sourced transforms.
/// # Errors
/// Invalid sources/evaluations, incompatible source/result Spaces, malformed
/// mappings or nonfinite calculations refuse. Missing facts remain typed unknown.
pub fn compare_geometry<'a>(
    before: &'a Snapshot,
    after: &'a Snapshot,
    key: &'a SourceKey,
    frame_kind: FrameKind,
    before_input: &'a uiblueprint_schema::analysis::EvaluationInput,
    after_input: &'a uiblueprint_schema::analysis::EvaluationInput,
) -> Result<GeometryDifference<'a>, crate::GeometryError> {
    use crate::{GeometryError, finite};
    use uiblueprint_schema::analysis::validate_bound_evaluation;
    validation::validate_snapshot(before).map_err(GeometryError::InvalidInput)?;
    validation::validate_snapshot(after).map_err(GeometryError::InvalidInput)?;
    validate_bound_evaluation(before, before_input).map_err(GeometryError::InvalidInput)?;
    validate_bound_evaluation(after, after_input).map_err(GeometryError::InvalidInput)?;
    if !sources_compatible(&before.context, &after.context)
        || before_input.result_space != after_input.result_space
    {
        return Err(GeometryError::InvalidInput(
            ValidationError::IncompatibleContext,
        ));
    }
    if let (Some(a), Some(b)) = (
        before.nodes.iter().find(|n| &n.key == key),
        after.nodes.iter().find(|n| &n.key == key),
    ) && a.surface != b.surface
    {
        return Err(GeometryError::InvalidInput(
            ValidationError::IncompatibleContext,
        ));
    }
    let old = resolve_geometry(before, key, frame_kind, before_input)?;
    let new = resolve_geometry(after, key, frame_kind, after_input)?;
    let displacement = match (&old, &new) {
        (ResolvedRect::Known { rect: a, .. }, ResolvedRect::Known { rect: b, .. }) => {
            Some(RectDisplacement {
                dx: finite(b.x - a.x)?,
                dy: finite(b.y - a.y)?,
                dwidth: finite(b.width - a.width)?,
                dheight: finite(b.height - a.height)?,
            })
        }
        _ => None,
    };
    Ok(GeometryDifference {
        before,
        after,
        key,
        frame_kind,
        before_evaluation: before_input,
        after_evaluation: after_input,
        before_geometry: old,
        after_geometry: new,
        displacement,
    })
}
fn resolve_geometry(
    snapshot: &Snapshot,
    key: &SourceKey,
    frame_kind: FrameKind,
    input: &uiblueprint_schema::analysis::EvaluationInput,
) -> Result<ResolvedRect, crate::GeometryError> {
    use crate::{EvaluationContext, resolve};
    let mut evidence = Vec::new();
    let field = match frame_kind {
        FrameKind::LayoutBounds => Field::LayoutBounds,
        FrameKind::AccessibilityBounds => Field::AccessibilityBounds,
        FrameKind::HitRegion => Field::HitRegion,
        FrameKind::VisibleRegion => Field::VisibleRegion,
        FrameKind::PaintBounds => Field::PaintBounds,
    };
    let property = snapshot
        .nodes
        .iter()
        .find(|n| &n.key == key)
        .and_then(|n| n.properties.iter().find(|p| p.field() == field));
    let source_space = property
        .and_then(Property::known)
        .and_then(|v| {
            if let Value::Geometry(g) = v {
                Some(g.coordinate_space.clone())
            } else {
                None
            }
        })
        .unwrap_or_else(|| input.result_space.clone());
    let anchor = Anchor {
        element: key.clone(),
        frame_kind,
        coordinate_space: source_space,
        fraction: 0.0,
        axis: Id("x".into()),
    };
    let context = EvaluationContext {
        space: &input.result_space,
        transforms: &input.transforms,
        conditions: input.conditions.as_ref().map(|c| (&c.values, &c.evidence)),
    };
    // Preserve the existing resolver's precise availability/binding/evidence rules.
    match resolve::rect(snapshot, &anchor, &context, &mut evidence)? {
        Ok(rect) => Ok(ResolvedRect::Known { rect, evidence }),
        Err(reason) => Ok(ResolvedRect::Unknown { reason, evidence }),
    }
}
