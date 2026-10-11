//! Explicitly selected relationships; never inferred design intent or UI mutation.
use crate::{GeometryError, GeometryQuery, MeasurementResult, finite, measure_query_bound};
use uiblueprint_schema::{analysis::EvaluationInput, model::*};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Composition {
    Equal,
    Ratio,
    Balance,
}

/// Exact arithmetic over bound measurements. Residual is not a check tolerance.
#[derive(Clone, Debug, PartialEq)]
pub struct Relationship {
    pub measurements: Vec<MeasurementResult>,
    pub value: Option<f64>,
    pub residual: Option<f64>,
    pub unavailable: Option<&'static str>,
}

/// Explicit-family descriptive candidate, not a normative expectation.
#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    pub measurements: Vec<MeasurementResult>,
    pub candidate: Option<f64>,
    pub support: usize,
    pub exceptions: Vec<(usize, f64)>,
    pub unavailable: Option<&'static str>,
}

fn measurements(
    snapshot: &Snapshot,
    queries: &[GeometryQuery],
    evaluation: &EvaluationInput,
) -> Result<Vec<MeasurementResult>, GeometryError> {
    let Some(first) = queries.first() else {
        return Err(GeometryError::InvalidRule);
    };
    let frame = first.anchors.first().map(|a| a.frame_kind);
    if queries.iter().any(|q| {
        q.quantity_kind != first.quantity_kind
            || q.units != first.units
            || q.anchors.iter().any(|a| Some(a.frame_kind) != frame)
    }) {
        return Err(GeometryError::InvalidRule);
    }
    queries
        .iter()
        .map(|q| measure_query_bound(snapshot, q, evaluation))
        .collect()
}

fn values(measurements: &[MeasurementResult]) -> Option<Vec<f64>> {
    measurements
        .iter()
        .map(|m| match m {
            MeasurementResult::Known { measurement } => match measurement.value {
                Value::Quantity { amount, .. } => Some(amount),
                _ => None,
            },
            MeasurementResult::Unknown { .. } => None,
        })
        .collect()
}

/// Recompute every operand against the same bound Snapshot and evaluation.
/// # Errors
/// Rejects bad arity, mixed dimensions/frames, invalid binding or nonfinite math.
pub fn compose(
    snapshot: &Snapshot,
    queries: &[GeometryQuery],
    evaluation: &EvaluationInput,
    operation: Composition,
) -> Result<Relationship, GeometryError> {
    if queries.len() < 2 || (operation == Composition::Ratio && queries.len() != 2) {
        return Err(GeometryError::InvalidRule);
    }
    let measurements = measurements(snapshot, queries, evaluation)?;
    let mut result = Relationship {
        measurements,
        value: None,
        residual: None,
        unavailable: Some("unknown_operand"),
    };
    let Some(v) = values(&result.measurements) else {
        return Ok(result);
    };
    let (value, residual) = match operation {
        Composition::Ratio if v[1] == 0.0 => {
            result.unavailable = Some("undefined_ratio");
            return Ok(result);
        }
        Composition::Ratio => (finite(v[0] / v[1])?, None),
        Composition::Equal => {
            let min = v.iter().copied().fold(f64::INFINITY, f64::min);
            let max = v.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            (v[0], Some(finite(max - min)?))
        }
        Composition::Balance => {
            let sum = v[..v.len() - 1]
                .iter()
                .try_fold(0.0, |sum, value| finite(sum + value))?;
            (sum, Some(finite(sum - v[v.len() - 1])?))
        }
    };
    result.value = Some(value);
    result.residual = residual;
    result.unavailable = None;
    Ok(result)
}

/// Describe exact repetition inside a caller-selected comparable family.
/// A strict majority is a disclosed heuristic, never confidence or correctness.
/// # Errors
/// Rejects fewer than three or duplicated/mixed-kind queries, invalid binding
/// and nonfinite calculations. Unknown/uncertain observations yield no candidate.
pub fn pattern(
    snapshot: &Snapshot,
    queries: &[GeometryQuery],
    evaluation: &EvaluationInput,
) -> Result<Pattern, GeometryError> {
    use GeometryRelation::{Gap, Height, Width};
    let Some(first) = queries.first() else {
        return Err(GeometryError::InvalidRule);
    };
    if queries.len() < 3
        || !matches!(first.operation, Gap | Width | Height)
        || queries.iter().enumerate().any(|(i, q)| {
            q.operation != first.operation
                || q.applies_when != first.applies_when
                || q.anchors
                    .iter()
                    .any(|a| Some(&a.axis) != first.anchors.first().map(|a| &a.axis))
                || q.anchors.first().map(|a| &a.axis) != first.anchors.first().map(|a| &a.axis)
                || queries[..i].iter().any(|other| q.anchors == other.anchors)
        })
    {
        return Err(GeometryError::InvalidRule);
    }
    let measurements = measurements(snapshot, queries, evaluation)?;
    let mut result = Pattern {
        measurements,
        candidate: None,
        support: 0,
        exceptions: vec![],
        unavailable: Some("unknown_operand"),
    };
    let Some(values) = values(&result.measurements) else {
        return Ok(result);
    };
    let mut observation = None;
    for m in &result.measurements {
        if let MeasurementResult::Known { measurement } = m {
            for e in &measurement.evidence {
                if e.uncertainty.as_ref().is_some_and(|u| u.absolute != 0.0)
                    || observation.is_some_and(|id| id != &e.observation_id)
                {
                    result.unavailable = Some("incomparable_observations_or_uncertainty");
                    return Ok(result);
                }
                observation = Some(&e.observation_id);
            }
        }
    }
    // Sort values only for counting; exceptions retain original operand indices.
    let mut sorted = values.clone();
    sorted.sort_by(f64::total_cmp);
    let mut start = 0;
    while start < sorted.len() {
        let mut end = start + 1;
        while end < sorted.len() && sorted[end] == sorted[start] {
            end += 1;
        }
        let count = end - start;
        if count > sorted.len() / 2 {
            result.candidate = Some(sorted[start]);
            result.support = count;
            break;
        }
        start = end;
    }
    result.unavailable = None;
    if let Some(candidate) = result.candidate {
        for (i, value) in values.into_iter().enumerate() {
            if value != candidate {
                result.exceptions.push((i, finite(value - candidate)?));
            }
        }
    }
    Ok(result)
}
