use super::{types::*, validation::*};
use crate::{
    model::*,
    validation::{self as core, ValidationError, require},
};

type Result<T = ()> = std::result::Result<T, ValidationError>;

fn field(query: &GeometryQuery, anchor: &Anchor) -> Field {
    if query.operation == GeometryRelation::Baseline {
        return Field::Baseline;
    }
    match anchor.frame_kind {
        FrameKind::LayoutBounds => Field::LayoutBounds,
        FrameKind::AccessibilityBounds => Field::AccessibilityBounds,
        FrameKind::HitRegion => Field::HitRegion,
        FrameKind::VisibleRegion => Field::VisibleRegion,
        FrameKind::PaintBounds => Field::PaintBounds,
    }
}
fn property<'a>(
    snapshot: &'a Snapshot,
    query: &GeometryQuery,
    anchor: &Anchor,
) -> Option<(&'a Node, &'a Property)> {
    let node = snapshot.nodes.iter().find(|n| n.key == anchor.element)?;
    Some((
        node,
        node.properties
            .iter()
            .find(|p| p.field() == field(query, anchor))?,
    ))
}
fn sources<'a>(
    snapshot: &'a Snapshot,
    query: &GeometryQuery,
    input: &'a EvaluationInput,
) -> Vec<&'a Evidence> {
    let mut sources = Vec::new();
    for anchor in &query.anchors {
        if let Some((
            _,
            Property::Requested {
                evidence, state, ..
            },
        )) = property(snapshot, query, anchor)
        {
            sources.push(evidence);
            if let Availability::Known {
                value: Value::Geometry(geometry),
            } = state
                && let TransformState::Known { transform } = &geometry.transform
            {
                sources.push(&transform.evidence);
            }
        }
    }
    sources.extend(input.transforms.iter().map(|t| &t.evidence));
    if let Some(condition) = &input.conditions {
        sources.push(&condition.evidence);
    }
    sources
}
fn validate_sources(
    snapshot: &Snapshot,
    query: &GeometryQuery,
    input: &EvaluationInput,
    evidence: &[Evidence],
    known: bool,
) -> Result {
    let eligible = sources(snapshot, query, input);
    require(
        !known || !evidence.is_empty(),
        ValidationError::InvalidEvidence,
    )?;
    for (i, source) in evidence.iter().enumerate() {
        require(
            eligible.contains(&source) && !evidence[..i].contains(source),
            ValidationError::InvalidEvidence,
        )?;
        let observation = core::evidence(source, &snapshot.observations)?;
        require(
            !known || observation.consistency == Consistency::Stable,
            ValidationError::UnknownMeasurement,
        )?;
    }
    Ok(())
}

// Establish declared connectivity only, using contributing transform evidence.
// No coordinates are transformed, no path is asserted to be the engine's chosen
// path, and no value is recomputed here. Full equality remains engine verification.
fn connected(
    from: &Space,
    surface: &Identity,
    snapshot: &Snapshot,
    attached: Option<&Transform>,
    input: &EvaluationInput,
    evidence: &[Evidence],
    baseline: bool,
) -> bool {
    let transforms: Vec<_> = attached
        .into_iter()
        .chain(&input.transforms)
        .filter(|t| {
            t.target == snapshot.context.target
                && &t.surface == surface
                && t.environment_revision == snapshot.context.environment_revision
                && evidence.contains(&t.evidence)
                && (!baseline || t.affine[1] == 0.0)
        })
        .collect();
    let mut reached = vec![from];
    let mut cursor = 0;
    while cursor < reached.len() {
        let current = reached[cursor];
        if current == &input.result_space {
            return true;
        }
        for transform in &transforms {
            if &transform.from == current && !reached.contains(&&transform.to) {
                reached.push(&transform.to);
            }
        }
        cursor += 1;
    }
    false
}
fn known_inputs(
    snapshot: &Snapshot,
    query: &GeometryQuery,
    input: &EvaluationInput,
    evidence: &[Evidence],
) -> Result {
    require(
        query
            .targets
            .iter()
            .all(|key| snapshot.nodes.iter().any(|n| &n.key == key)),
        ValidationError::UnknownMeasurement,
    )?;
    if matches!(
        query.operation,
        GeometryRelation::Aligned | GeometryRelation::EqualSpacing
    ) {
        require(
            snapshot.coverage.status == CoverageStatus::Complete
                && snapshot.coverage.omitted_count.is_none_or(|n| n == 0)
                && snapshot.coverage.unknown_count.is_none_or(|n| n == 0),
            ValidationError::UnknownMeasurement,
        )?;
    }
    let required = &query.applies_when;
    if required.platform.is_some() || required.input_mode.is_some() || required.text_scale.is_some()
    {
        let actual = input
            .conditions
            .as_ref()
            .ok_or(ValidationError::UnknownMeasurement)?;
        require(
            required
                .platform
                .as_ref()
                .is_none_or(|v| actual.values.platform.as_ref() == Some(v))
                && required
                    .input_mode
                    .is_none_or(|v| actual.values.input_mode == Some(v))
                && required
                    .text_scale
                    .is_none_or(|v| actual.values.text_scale == Some(v))
                && evidence.contains(&actual.evidence),
            ValidationError::UnknownMeasurement,
        )?;
    }
    for anchor in &query.anchors {
        let Some((
            node,
            Property::Requested {
                sensitivity,
                evidence: source,
                state: Availability::Known { value },
                ..
            },
        )) = property(snapshot, query, anchor)
        else {
            return Err(ValidationError::UnknownMeasurement);
        };
        require(
            *sensitivity != Sensitivity::Sensitive && evidence.contains(source),
            ValidationError::InvalidEvidence,
        )?;
        let (space, attached) = match value {
            Value::Baseline { space, .. } if query.operation == GeometryRelation::Baseline => {
                (space, None)
            }
            Value::Geometry(g)
                if g.frame_kind == anchor.frame_kind
                    && matches!(&g.shape, Shape::Rect(_))
                    && query.operation != GeometryRelation::Baseline =>
            {
                let attached = match &g.transform {
                    TransformState::Known { transform } => Some(transform.as_ref()),
                    _ => None,
                };
                (&g.coordinate_space, attached)
            }
            _ => return Err(ValidationError::UnknownMeasurement),
        };
        require(
            space == &anchor.coordinate_space
                && connected(
                    space,
                    &node.surface,
                    snapshot,
                    attached,
                    input,
                    evidence,
                    query.operation == GeometryRelation::Baseline,
                ),
            ValidationError::MissingTransform,
        )?;
    }
    Ok(())
}
fn details(
    value: &MeasurementDetails,
    operation: GeometryRelation,
    anchors: usize,
    amount: f64,
) -> Result {
    use GeometryRelation::*;
    let valid = match (value, operation) {
        (
            MeasurementDetails::Scalar {},
            Width | Height | Ratio | Gap | Distance | Aligned | Baseline,
        ) => true,
        (
            MeasurementDetails::Insets {
                left,
                top,
                right,
                bottom,
            },
            Inside | Overflow,
        ) => [left, top, right, bottom].iter().all(|v| v.is_finite()),
        (MeasurementDetails::Intersection { rect: None }, Intersects) => amount == 0.0,
        (MeasurementDetails::Intersection { rect: Some(r) }, Intersects) => {
            [r.x, r.y, r.width, r.height].iter().all(|v| v.is_finite())
                && r.width > 0.0
                && r.height > 0.0
                && amount > 0.0
        }
        (MeasurementDetails::Gaps { values }, EqualSpacing) => {
            values.len() == anchors.saturating_sub(1) && values.iter().all(|v| v.is_finite())
        }
        _ => false,
    };
    require(valid, ValidationError::InvalidGeometry)
}
fn result(
    snapshot: &Snapshot,
    query: &GeometryQuery,
    input: &EvaluationInput,
    value: &MeasurementResult,
) -> Result {
    validate_query(query)?;
    validate_bound_evaluation(snapshot, input)?;
    require(
        query.scope_id == snapshot.context.scope_id,
        ValidationError::IncompatibleContext,
    )?;
    match value {
        MeasurementResult::Unknown { evidence, .. } => {
            validate_sources(snapshot, query, input, evidence, false)
        }
        MeasurementResult::Known { measurement: m } => {
            let Value::Quantity {
                amount,
                kind,
                source_units,
            } = m.value
            else {
                return Err(ValidationError::ValueType);
            };
            require(
                amount.is_finite()
                    && kind == query.quantity_kind
                    && source_units == query.units
                    && source_units == input.result_space.units
                    && m.space == input.result_space,
                ValidationError::ValueType,
            )?;
            details(&m.details, query.operation, query.anchors.len(), amount)?;
            validate_sources(snapshot, query, input, &m.evidence, true)?;
            known_inputs(snapshot, query, input, &m.evidence)
        }
    }
}

/// Contract/evidence validation only; importing a computed result additionally
/// requires the engine's independent recomputation of values/details/provenance.
pub fn validate_measurement_case(case: &MeasurementCase) -> Result {
    result(&case.snapshot, &case.query, &case.evaluation, &case.result)
}
fn finite(value: f64) -> Result<f64> {
    require(value.is_finite(), ValidationError::InvalidOutcome)?;
    Ok(value)
}
/// Validate declared check consistency without recomputing source geometry.
/// The original 0.1 FindingCase validator is deliberately unchanged.
pub fn validate_geometry_check_case(case: &GeometryCheckCase) -> Result {
    core::validate_expectation(&case.expectation)?;
    let query = GeometryQuery::from_expectation(&case.expectation)
        .ok_or(ValidationError::InvalidDocument)?;
    result(&case.snapshot, &query, &case.evaluation, &case.measurement)?;
    let finding = &case.finding;
    require(
        !finding.id.0.is_empty()
            && finding.id.0.chars().count() <= 256
            && finding.expectation_id == case.expectation.id
            && finding.snapshot_id == case.snapshot.id,
        ValidationError::IncompatibleContext,
    )?;
    let evidence = match &case.measurement {
        MeasurementResult::Known { measurement } => &measurement.evidence,
        MeasurementResult::Unknown { evidence, .. } => evidence,
    };
    require(
        finding.observation_id.as_ref() == evidence.first().map(|e| &e.observation_id),
        ValidationError::InvalidEvidence,
    )?;
    match &case.measurement {
        MeasurementResult::Unknown { reason, .. } => require(
            finding.status == CheckStatus::Unknown
                && finding.measured.is_none()
                && finding
                    .reason
                    .as_ref()
                    .is_some_and(|r| r.0 == reason.as_str()),
            ValidationError::InvalidOutcome,
        ),
        MeasurementResult::Known { measurement } => {
            let Value::Quantity { amount, .. } = measurement.value else {
                return Err(ValidationError::ValueType);
            };
            let Rule::Geometry {
                expected,
                comparison,
                tolerance,
                ..
            } = case.expectation.rule
            else {
                return Err(ValidationError::InvalidDocument);
            };
            let pass = match comparison {
                Comparison::Equal => finite(amount - expected)?.abs() <= tolerance,
                Comparison::AtLeast => amount >= finite(expected - tolerance)?,
                Comparison::AtMost => amount <= finite(expected + tolerance)?,
                Comparison::GreaterThan => amount > finite(expected + tolerance)?,
            };
            let status = if pass {
                CheckStatus::Pass
            } else {
                CheckStatus::Fail
            };
            let reason = if pass {
                "expectation_satisfied"
            } else {
                "expectation_mismatch"
            };
            require(
                finding.measured.as_ref() == Some(&measurement.value)
                    && finding.status == status
                    && finding.reason.as_ref().is_none_or(|r| r.0 == reason),
                ValidationError::InvalidOutcome,
            )
        }
    }
}
