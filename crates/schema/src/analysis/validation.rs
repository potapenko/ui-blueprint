use super::types::*;
use crate::{
    model::*,
    validation::{self as core, ValidationError, require},
};
use std::collections::BTreeSet;

type Result = std::result::Result<(), ValidationError>;

fn id(value: &Id) -> Result {
    require(
        !value.0.is_empty() && value.0.chars().count() <= 256,
        ValidationError::InvalidDocument,
    )
}
fn conditions(value: &ContextConditions) -> Result {
    if let Some(platform) = &value.platform {
        id(platform)?;
    }
    require(
        value.text_scale.is_none_or(|v| v.is_finite() && v > 0.0),
        ValidationError::InvalidDocument,
    )
}
fn evidence_declaration(value: &Evidence) -> Result {
    id(&value.observation_id)?;
    id(&value.source_namespace)?;
    id(&value.method)?;
    require(
        value
            .uncertainty
            .as_ref()
            .is_none_or(|v| v.absolute.is_finite() && v.absolute >= 0.0),
        ValidationError::InvalidEvidence,
    )
}

/// Validate factual query declarations without requiring a source Snapshot.
/// Missing actual nodes/properties are evaluated as unknown by the engine.
pub fn validate_query(query: &GeometryQuery) -> Result {
    id(&query.id)?;
    id(&query.scope_id)?;
    conditions(&query.applies_when)?;
    require(
        !query.targets.is_empty()
            && query.targets.iter().collect::<BTreeSet<_>>().len() == query.targets.len(),
        ValidationError::InvalidDocument,
    )?;
    for key in &query.targets {
        id(&key.namespace)?;
        id(&key.key)?;
    }
    for anchor in &query.anchors {
        core::validate_anchor(anchor)?;
        id(&anchor.coordinate_space.id)?;
        require(
            query.targets.contains(&anchor.element),
            ValidationError::DanglingReference,
        )?;
    }
    use GeometryRelation::*;
    let arity = match query.operation {
        Width | Height | Ratio => query.anchors.len() == 1,
        Aligned | Baseline => query.anchors.len() >= 2,
        EqualSpacing => query.anchors.len() >= 3,
        _ => query.anchors.len() == 2,
    };
    let quantity = match query.operation {
        Ratio => QuantityKind::Ratio,
        Intersects => QuantityKind::Area,
        _ => QuantityKind::Length,
    };
    let shared_axis = query
        .anchors
        .windows(2)
        .all(|pair| pair[0].axis == pair[1].axis);
    let axes = match query.operation {
        Gap | Aligned | EqualSpacing => {
            shared_axis
                && query
                    .anchors
                    .iter()
                    .all(|a| matches!(a.axis.0.as_str(), "x" | "y"))
        }
        Distance => shared_axis,
        _ => true,
    };
    require(
        arity && axes && query.quantity_kind == quantity,
        ValidationError::InvalidGeometry,
    )
}

/// Validate an unbound evaluation's own declarations. Evidence references are
/// not claimed resolved until validate_bound_evaluation receives its Snapshot.
pub fn validate_evaluation_input(input: &EvaluationInput) -> Result {
    id(&input.snapshot_id)?;
    id(&input.result_space.id)?;
    core::validate_context(&input.context)?;
    for value in [
        &input.context.session_id,
        &input.context.target.id,
        &input.context.target.generation,
        &input.context.scope_id,
        &input.context.plugin.id,
        &input.context.plugin.version,
        &input.context.environment_revision,
    ] {
        id(value)?;
    }
    for surface in &input.context.surfaces {
        id(&surface.id)?;
        id(&surface.generation)?;
    }
    for transform in &input.transforms {
        id(&transform.from.id)?;
        id(&transform.to.id)?;
        require(
            transform.target == input.context.target
                && input.context.surfaces.contains(&transform.surface)
                && transform.environment_revision == input.context.environment_revision,
            ValidationError::IncompatibleContext,
        )?;
        let [a, b, c, d, _, _] = transform.affine;
        let determinant = a * d - b * c;
        require(
            transform.affine.iter().all(|x| x.is_finite())
                && determinant.is_finite()
                && determinant != 0.0,
            ValidationError::InvalidGeometry,
        )?;
        evidence_declaration(&transform.evidence)?;
    }
    if let Some(observed) = &input.conditions {
        conditions(&observed.values)?;
        evidence_declaration(&observed.evidence)?;
    }
    Ok(())
}

/// Bind evaluation to an unchanged canonical source. This checks references,
/// not transform-path consumption, arithmetic or real-world observation truth.
pub fn validate_bound_evaluation(snapshot: &Snapshot, input: &EvaluationInput) -> Result {
    core::validate_snapshot(snapshot)?;
    validate_evaluation_input(input)?;
    require(
        input.snapshot_id == snapshot.id
            && input.revision == snapshot.revision
            && core::contexts_compatible(&snapshot.context, &input.context),
        ValidationError::IncompatibleContext,
    )?;
    for transform in &input.transforms {
        core::evidence(&transform.evidence, &snapshot.observations)?;
    }
    if let Some(conditions) = &input.conditions {
        core::evidence(&conditions.evidence, &snapshot.observations)?;
    }
    Ok(())
}
