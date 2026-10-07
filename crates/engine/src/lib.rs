//! Deterministic geometry over canonical snapshots. No collection or side effects.
#![forbid(unsafe_code)]

pub mod analysis;
mod arithmetic;
pub mod cache;
pub mod replay;
mod resolve;
pub mod scope;

pub use analysis::{VerificationError, check_bound, measure_query_bound, verify_analysis_result};
use uiblueprint_schema::{model::*, validation};

/// Calculation failure, distinct from an unavailable measurement or a failed check.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeometryError {
    InvalidInput(validation::ValidationError),
    InvalidRule,
    InvalidTransform,
    NonFiniteCalculation,
}
impl std::fmt::Display for GeometryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for GeometryError {}

pub use uiblueprint_schema::analysis::{
    GeometryQuery, Measurement, MeasurementDetails as Details, MeasurementResult,
    MeasurementUnknownReason as UnknownReason,
};

/// Explicit calculation inputs; these are borrowed canonical types, not wire fields.
/// `space` is the requested result space. Additional directional transforms must
/// be evidenced in this snapshot; no implicit inverse, scale or origin conversion.
/// Conditions are observed facts with evidence, never copied from the expectation.
pub struct EvaluationContext<'a> {
    pub space: &'a Space,
    pub transforms: &'a [Transform],
    pub conditions: Option<(&'a ContextConditions, &'a Evidence)>,
}

/// Canonical finding plus the richer local measurement provenance/details.
/// The finding ID equals its expectation ID; callers can assign a storage ID.
#[derive(Clone, Debug, PartialEq)]
pub struct Check {
    pub finding: Finding,
    pub measurement: MeasurementResult,
    pub tolerance: f64,
}

/// Measure one geometry relation; malformed records/calculations are typed errors.
/// Unrequested, unsupported and unavailable inputs remain explicit unknowns.
/// # Errors
/// Rejects invalid canonical inputs, invalid relation arity/dimensions and
/// nonfinite arithmetic. Does not modify the schema validator or its contracts.
pub fn measure(
    snapshot: &Snapshot,
    expectation: &Expectation,
    context: &EvaluationContext<'_>,
) -> Result<MeasurementResult, GeometryError> {
    validation::validate_expectation(expectation).map_err(GeometryError::InvalidInput)?;
    let query = GeometryQuery::from_expectation(expectation).ok_or(GeometryError::InvalidRule)?;
    measure_query(snapshot, &query, context)
}

/// Compute a factual query with no expected value, tolerance or normative source.
/// Borrowed local callers retain the existing geometry facade; serialized/imported
/// evaluation inputs must use the binding-checked analysis entrypoints.
/// # Errors
/// Rejects invalid canonical snapshots/query declarations and nonfinite arithmetic.
pub fn measure_query(
    snapshot: &Snapshot,
    query: &GeometryQuery,
    context: &EvaluationContext<'_>,
) -> Result<MeasurementResult, GeometryError> {
    validation::validate_snapshot(snapshot).map_err(GeometryError::InvalidInput)?;
    uiblueprint_schema::analysis::validate_query(query).map_err(|error| {
        if error == validation::ValidationError::InvalidGeometry {
            GeometryError::InvalidRule
        } else {
            GeometryError::InvalidInput(error)
        }
    })?;
    if snapshot.context.scope_id != query.scope_id {
        return Err(GeometryError::InvalidRule);
    }
    let GeometryQuery {
        operation,
        anchors,
        quantity_kind,
        units,
        ..
    } = query;
    let mut evidence = Vec::new();
    if *units != context.space.units {
        return Ok(MeasurementResult::Unknown {
            reason: UnknownReason::MissingTransform,
            evidence,
        });
    }
    if let Some(reason) =
        resolve::applicability(snapshot, &query.applies_when, context, &mut evidence)?
    {
        return Ok(MeasurementResult::Unknown { reason, evidence });
    }
    if matches!(
        operation,
        GeometryRelation::EqualSpacing | GeometryRelation::Aligned
    ) && (snapshot.coverage.status != CoverageStatus::Complete
        || snapshot.coverage.omitted_count.is_some_and(|n| n > 0)
        || snapshot.coverage.unknown_count.is_some_and(|n| n > 0))
    {
        return Ok(MeasurementResult::Unknown {
            reason: UnknownReason::IncompleteScope,
            evidence,
        });
    }
    for target in &query.targets {
        if !snapshot.nodes.iter().any(|n| &n.key == target) {
            return Ok(MeasurementResult::Unknown {
                reason: UnknownReason::MissingTarget,
                evidence,
            });
        }
    }
    let result = arithmetic::calculate(snapshot, *operation, anchors, context, &mut evidence)?;
    match result {
        Ok((amount, details)) => {
            finite(amount)?;
            Ok(MeasurementResult::Known {
                measurement: Measurement {
                    value: Value::Quantity {
                        amount,
                        kind: *quantity_kind,
                        source_units: *units,
                    },
                    space: context.space.clone(),
                    details,
                    evidence,
                },
            })
        }
        Err(reason) => Ok(MeasurementResult::Unknown { reason, evidence }),
    }
}

/// Evaluate with the exact explicit comparison/tolerance from the expectation.
/// Observation IDs and sources remain available in `measurement.evidence`;
/// the scalar wire finding references its first contributing Observation.
/// # Errors
/// Returns the same typed input/calculation errors as [`measure`].
pub fn check(
    snapshot: &Snapshot,
    expectation: &Expectation,
    context: &EvaluationContext<'_>,
) -> Result<Check, GeometryError> {
    let result = measure(snapshot, expectation, context)?;
    let Rule::Geometry {
        expected,
        comparison,
        tolerance,
        ..
    } = expectation.rule
    else {
        return Err(GeometryError::InvalidRule);
    };
    let (status, measured, observation_id, reason) = match &result {
        MeasurementResult::Known { measurement: m } => {
            let Value::Quantity { amount, .. } = m.value else {
                unreachable!("measure only constructs quantities")
            };
            let pass = match comparison {
                Comparison::Equal => finite(amount - expected)?.abs() <= tolerance,
                Comparison::AtLeast => amount >= finite(expected - tolerance)?,
                Comparison::AtMost => amount <= finite(expected + tolerance)?,
                Comparison::GreaterThan => amount > finite(expected + tolerance)?,
            };
            (
                if pass {
                    CheckStatus::Pass
                } else {
                    CheckStatus::Fail
                },
                Some(m.value.clone()),
                m.evidence.first().map(|e| e.observation_id.clone()),
                Some(Id(if pass {
                    "expectation_satisfied"
                } else {
                    "expectation_mismatch"
                }
                .into())),
            )
        }
        MeasurementResult::Unknown {
            reason,
            evidence: sources,
        } => (
            CheckStatus::Unknown,
            None,
            sources.first().map(|e| e.observation_id.clone()),
            Some(Id(reason.as_str().into())),
        ),
    };
    Ok(Check {
        finding: Finding {
            id: expectation.id.clone(),
            expectation_id: expectation.id.clone(),
            snapshot_id: snapshot.id.clone(),
            observation_id,
            status,
            measured,
            reason,
        },
        measurement: result,
        tolerance,
    })
}

fn finite(value: f64) -> Result<f64, GeometryError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(GeometryError::NonFiniteCalculation)
    }
}

type Available<T> = Result<Result<T, UnknownReason>, GeometryError>;
