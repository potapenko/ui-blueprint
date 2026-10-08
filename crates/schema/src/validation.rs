//! Structural JSON validation plus semantic checks; no collection or cache engine.
use crate::model::*;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, fmt};

mod graph;
mod outcomes;
use graph::{node, property};
pub use graph::{validate_delta, validate_snapshot};
use outcomes::validate_chain;
pub use outcomes::{validate_action, validate_expectation, validate_finding, validate_transition};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationError {
    InvalidDocument,
    ResourceLimit,
    InvalidCoverage,
    InvalidEvidence,
    DuplicateIdentity,
    MissingProperty,
    ValueType,
    PrivateValue,
    InvalidGeometry,
    MissingTransform,
    DanglingReference,
    IncompatibleContext,
    ResyncRequired,
    StaleTarget,
    AmbiguousTarget,
    InvalidOutcome,
    UnknownMeasurement,
    InternalSchema,
}
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for ValidationError {}
type Result<T = ()> = std::result::Result<T, ValidationError>;
pub(crate) fn require(condition: bool, error: ValidationError) -> Result {
    if condition { Ok(()) } else { Err(error) }
}
fn unique<T: Ord>(items: impl IntoIterator<Item = T>) -> bool {
    let mut seen = BTreeSet::new();
    items.into_iter().all(|item| seen.insert(item))
}

pub fn json_schema() -> Result<serde_json::Value> {
    serde_json::to_value(schemars::schema_for!(Document))
        .map_err(|_| ValidationError::InternalSchema)
}

impl Document {
    /// Never loads paths/URLs from artifacts. The byte limit is caller-supplied.
    pub fn from_json(input: &[u8], max_bytes: usize) -> Result<Self> {
        require(input.len() <= max_bytes, ValidationError::ResourceLimit)?;
        // Strict map-only records retain duplicate-key rejection. Discard serde
        // errors rather than retaining an untrusted key/value in diagnostics.
        let document: Self =
            serde_json::from_slice(input).map_err(|_| ValidationError::InvalidDocument)?;
        validate_semantics(&document.artifact)?;
        Ok(document)
    }
    pub fn validate(&self) -> Result {
        let bytes = serde_json::to_vec(self).map_err(|_| ValidationError::InvalidDocument)?;
        Self::from_json(&bytes, bytes.len()).map(|_| ())
    }
}

fn validate_semantics(artifact: &Artifact) -> Result {
    match artifact {
        Artifact::Request(x) => validate_request(x),
        Artifact::Session(x) => {
            require(
                !x.supported_versions.is_empty()
                    && !x.surfaces.is_empty()
                    && !x.allowed_scopes.is_empty(),
                ValidationError::InvalidDocument,
            )?;
            require(unique(&x.surfaces), ValidationError::DuplicateIdentity)?;
            for c in &x.capabilities {
                validate_capability(c)?;
            }
            Ok(())
        }
        Artifact::SessionContext(x) => {
            validate_semantics(&Artifact::Session(Box::new(x.session.clone())))?;
            validate_request(&x.request)?;
            require(
                x.session
                    .allowed_scopes
                    .contains(&x.request.context.scope_id)
                    && x.session.session_id == x.request.context.session_id
                    && x.session.target == x.request.context.target
                    && x.request
                        .context
                        .surfaces
                        .iter()
                        .all(|s| x.session.surfaces.contains(s)),
                ValidationError::IncompatibleContext,
            )?;
            if let Some(r) = &x.backend_ref {
                require(
                    r.session_id == x.session.session_id
                        && r.target == x.session.target
                        && x.session.surfaces.contains(&r.surface),
                    ValidationError::StaleTarget,
                )?;
            }
            Ok(())
        }
        Artifact::ResolutionRefusal(x) => {
            validate_context(&x.requested)?;
            require(
                !x.dispatched
                    && x.issue.code == ErrorCode::TargetUnresolved
                    && x.issue.scope_id == x.requested.scope_id
                    && x.candidates.iter().all(|c| {
                        !matches!(c.process_continuity, Availability::Known { .. })
                            || !matches!(c.owner_binding, Availability::Known { .. })
                    }),
                ValidationError::InvalidOutcome,
            )
        }
        Artifact::ChannelResponse(x) => match &x.result {
            ChannelResult::Observed(snapshot) => {
                validate_snapshot(snapshot)?;
                require(
                    snapshot.context.session_id == x.session_id
                        && snapshot.context.target == x.target
                        && (snapshot.observations.iter().all(|o| o.channel == x.channel)
                            || (x.channel == Channel::OptInLayoutProbe
                                && snapshot.context.projection == Projection::Design
                                && snapshot
                                    .observations
                                    .iter()
                                    .any(|o| o.channel == Channel::ExternalSemantics)
                                && snapshot
                                    .observations
                                    .iter()
                                    .any(|o| o.channel == Channel::OptInLayoutProbe)
                                && snapshot.observations.iter().all(|o| {
                                    matches!(
                                        o.channel,
                                        Channel::ExternalSemantics | Channel::OptInLayoutProbe
                                    )
                                }))),
                    ValidationError::IncompatibleContext,
                )
            }
            ChannelResult::Failed(_) => Ok(()),
        },
        Artifact::Capability(x) => validate_capability(x),
        Artifact::Observation(x) => validate_observation(x),
        Artifact::Property(x) => {
            validate_observation(&x.observation)?;
            validate_property(&x.property, std::slice::from_ref(&x.observation))
        }
        Artifact::Geometry(x) => {
            validate_observation(&x.observation)?;
            validate_geometry(&x.geometry, std::slice::from_ref(&x.observation))?;
            geometry_context(&x.geometry, &x.context)
        }
        Artifact::Snapshot(x) => validate_snapshot(x),
        Artifact::Delta(x) => {
            validate_delta(&x.base, &x.update)?;
            if let Some(source) = &x.source_snapshot {
                graph::validate_delta_source(&x.base, &x.update, source)?;
            }
            Ok(())
        }
        Artifact::Action(x) => {
            validate_snapshot(&x.snapshot)?;
            validate_action(&x.snapshot, &x.action)
        }
        Artifact::Transition(x) => validate_transition(x),
        Artifact::Expectation(x) => validate_expectation(x),
        Artifact::Finding(x) => validate_finding(&x.snapshot, &x.expectation, &x.finding),
        Artifact::Error(_) => Ok(()),
        Artifact::GoldenChain(x) => validate_chain(x),
        Artifact::ActionResult(x) => outcomes::validate_action_result(x),
        Artifact::DeltaResult(x) => require(
            !x.published
                && x.issue.code == ErrorCode::ResyncRequired
                && x.base
                    .as_ref()
                    .is_none_or(|b| validate_delta(b, &x.update).is_err()),
            ValidationError::InvalidOutcome,
        ),
        Artifact::TemporalComparison(x) => validate_temporal(x),
        Artifact::TransitionContext(x) => outcomes::validate_transition_context(x),
    }
}

pub fn validate_context(x: &Context) -> Result {
    require(
        !x.surfaces.is_empty() && !x.fields.is_empty(),
        ValidationError::InvalidDocument,
    )?;
    require(
        unique(&x.surfaces) && unique(&x.fields),
        ValidationError::DuplicateIdentity,
    )
}
pub fn contexts_compatible(a: &Context, b: &Context) -> bool {
    a.schema_version == b.schema_version
        && a.session_id == b.session_id
        && a.target == b.target
        && a.scope_id == b.scope_id
        && a.projection == b.projection
        && a.plugin == b.plugin
        && a.environment_revision == b.environment_revision
        && a.surfaces.iter().collect::<BTreeSet<_>>() == b.surfaces.iter().collect()
        && a.fields.iter().collect::<BTreeSet<_>>() == b.fields.iter().collect()
}
fn validate_coverage(x: &Coverage) -> Result {
    require(
        unique(&x.fields) && !x.fields.is_empty(),
        ValidationError::InvalidCoverage,
    )?;
    require(
        x.status != CoverageStatus::Complete
            || (x.omitted_count.unwrap_or(0) == 0 && x.unknown_count.unwrap_or(0) == 0),
        ValidationError::InvalidCoverage,
    )
}
fn coverage_context(x: &Coverage, c: &Context) -> Result {
    validate_coverage(x)?;
    require(
        x.scope_id == c.scope_id
            && x.fields.iter().collect::<BTreeSet<_>>() == c.fields.iter().collect(),
        ValidationError::InvalidCoverage,
    )
}
pub fn validate_observation(x: &Observation) -> Result {
    require(
        x.start.is_finite() && x.end.is_finite() && x.end >= x.start,
        ValidationError::InvalidEvidence,
    )?;
    require(
        x.last_verified.is_none_or(|t| t.is_finite()),
        ValidationError::InvalidEvidence,
    )?;
    require(
        x.freshness != Freshness::Current || x.last_verified.is_some(),
        ValidationError::InvalidEvidence,
    )?;
    require(
        x.freshness != Freshness::Current
            || matches!(
                x.freshness_basis,
                FreshnessBasis::LiveRead | FreshnessBasis::Revalidated
            ),
        ValidationError::InvalidEvidence,
    )?;
    require(
        x.consistency == Consistency::Stable || x.consistency_reason.is_some(),
        ValidationError::InvalidEvidence,
    )?;
    validate_coverage(&x.coverage)
}
pub(crate) fn evidence<'a>(
    e: &Evidence,
    observations: &'a [Observation],
) -> Result<&'a Observation> {
    let o = observations
        .iter()
        .find(|o| o.id == e.observation_id)
        .ok_or(ValidationError::InvalidEvidence)?;
    if let Some(u) = &e.uncertainty {
        require(
            u.absolute.is_finite() && u.absolute >= 0.0,
            ValidationError::InvalidEvidence,
        )?;
    }
    require(
        o.source_namespace == e.source_namespace,
        ValidationError::InvalidEvidence,
    )?;
    Ok(o)
}
fn validate_capability(x: &Capability) -> Result {
    require(
        x.status == CapabilityStatus::Supported || x.reason.is_some(),
        ValidationError::InvalidDocument,
    )
}
pub fn validate_request(x: &Request) -> Result {
    validate_context(&x.context)?;
    require(
        x.limits.max_elements > 0
            && x.limits.max_depth > 0
            && x.limits.max_output_bytes > 0
            && x.limits.deadline_ms > 0,
        ValidationError::ResourceLimit,
    )?;
    match &x.operation {
        Operation::Observe { channels } => require(
            !channels.is_empty() && unique(channels),
            ValidationError::InvalidDocument,
        ),
        Operation::Measure { from, to, .. } => {
            validate_anchor(from)?;
            validate_anchor(to)
        }
        Operation::Act { action } | Operation::Prepare { action } => require(
            contexts_compatible(&x.context, &action.context),
            ValidationError::IncompatibleContext,
        ),
        _ => Ok(()),
    }
}

pub(crate) fn validate_anchor(x: &Anchor) -> Result {
    require(
        x.fraction.is_finite()
            && (0.0..=1.0).contains(&x.fraction)
            && matches!(x.axis.0.as_str(), "x" | "y" | "xy"),
        ValidationError::InvalidGeometry,
    )
}
fn validate_geometry(x: &Geometry, observations: &[Observation]) -> Result {
    let rect = |r: &Rect| {
        [r.x, r.y, r.width, r.height].iter().all(|n| n.is_finite())
            && r.width >= 0.0
            && r.height >= 0.0
    };
    let points = |p: &[Point]| p.iter().all(|p| p.x.is_finite() && p.y.is_finite());
    require(
        match &x.shape {
            Shape::Rect(r) => rect(r),
            Shape::Quad(q) => points(q),
            Shape::Polygon(p) => p.len() >= 3 && points(p),
            Shape::Fragments(rs) => !rs.is_empty() && rs.iter().all(rect),
        },
        ValidationError::InvalidGeometry,
    )?;
    if let TransformState::Known { transform: t } = &x.transform {
        require(
            t.from == x.coordinate_space
                && t.affine.iter().all(|n| n.is_finite())
                && (t.affine[0] * t.affine[3] - t.affine[1] * t.affine[2]).abs() > 0.0,
            ValidationError::InvalidGeometry,
        )?;
        evidence(&t.evidence, observations)?;
    }
    Ok(())
}
fn geometry_field(kind: FrameKind) -> Field {
    match kind {
        FrameKind::LayoutBounds => Field::LayoutBounds,
        FrameKind::AccessibilityBounds => Field::AccessibilityBounds,
        FrameKind::HitRegion => Field::HitRegion,
        FrameKind::VisibleRegion => Field::VisibleRegion,
        FrameKind::PaintBounds => Field::PaintBounds,
    }
}
fn field_value(field: Field, value: &Value) -> bool {
    match field {
        Field::Baseline => matches!(value, Value::Baseline { .. }),
        Field::Role => matches!(value, Value::Role(_)),
        Field::Required
        | Field::Enabled
        | Field::Readonly
        | Field::Checked
        | Field::Selected
        | Field::Expanded
        | Field::Focused
        | Field::Invalid => matches!(value, Value::Flag(_)),
        Field::LayoutBounds
        | Field::AccessibilityBounds
        | Field::HitRegion
        | Field::VisibleRegion
        | Field::PaintBounds => {
            matches!(value, Value::Geometry(g) if geometry_field(g.frame_kind) == field)
        }
        Field::Actions => matches!(value, Value::TextList(_)),
        Field::Value => matches!(value, Value::Text(_) | Value::Number(_) | Value::Flag(_)),
        _ => matches!(value, Value::Text(_)),
    }
}
fn validate_property(p: &Property, observations: &[Observation]) -> Result {
    if let Property::Requested {
        field,
        sensitivity,
        evidence: e,
        state,
    } = p
    {
        evidence(e, observations)?;
        require(
            *sensitivity != Sensitivity::Sensitive
                || matches!(state, Availability::Redacted { .. }),
            ValidationError::PrivateValue,
        )?;
        if let Availability::Known { value } = state {
            require(field_value(*field, value), ValidationError::ValueType)?;
            match value {
                Value::Number(n) => require(n.is_finite(), ValidationError::ValueType)?,
                Value::Baseline { coordinate, .. } => {
                    require(coordinate.is_finite(), ValidationError::InvalidGeometry)?
                }
                Value::Geometry(g) => validate_geometry(g, observations)?,
                _ => (),
            }
        }
    }
    Ok(())
}

fn geometry_context(g: &Geometry, c: &Context) -> Result {
    if let TransformState::Known { transform: t } = &g.transform {
        require(
            t.target == c.target
                && c.surfaces.contains(&t.surface)
                && t.environment_revision == c.environment_revision,
            ValidationError::MissingTransform,
        )?;
    }
    Ok(())
}
fn validate_temporal(x: &TemporalComparison) -> Result {
    require(
        x.observations.len() >= 2 && !x.atomic_claim,
        ValidationError::InvalidEvidence,
    )?;
    for o in &x.observations {
        validate_observation(o)?;
    }
    if let Some(overlap) = x.overlap {
        require(
            overlap.is_finite() && overlap >= 0.0,
            ValidationError::InvalidEvidence,
        )?;
        let first = &x.observations[0];
        if x.observations
            .iter()
            .any(|o| o.clock_domain != first.clock_domain || o.time_unit != first.time_unit)
        {
            let t = x
                .transform
                .as_ref()
                .ok_or(ValidationError::InvalidEvidence)?;
            require(
                t.scale.is_finite()
                    && t.scale > 0.0
                    && t.offset.is_finite()
                    && x.observations
                        .iter()
                        .all(|o| o.clock_domain == t.from_clock || o.clock_domain == t.to_clock),
                ValidationError::InvalidEvidence,
            )?;
        }
    }
    Ok(())
}
