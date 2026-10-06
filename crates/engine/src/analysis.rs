//! Binding-aware analysis and recomputation of imported 0.2 results.
use crate::{Check, EvaluationContext, GeometryError, MeasurementResult, check, measure_query};
use uiblueprint_schema::{analysis as schema, model::*, validation::ValidationError};

fn borrowed(input: &schema::EvaluationInput) -> EvaluationContext<'_> {
    EvaluationContext {
        space: &input.result_space,
        transforms: &input.transforms,
        conditions: input
            .conditions
            .as_ref()
            .map(|value| (&value.values, &value.evidence)),
    }
}

/// Bind the entire canonical evaluation before measuring. The source snapshot is
/// never changed to match an imported ID, context, transform or observation.
/// # Errors
/// Rejects invalid/mismatched input binding and the same arithmetic errors as
/// measure_query; unavailable factual measurements remain explicit unknowns.
pub fn measure_query_bound(
    snapshot: &Snapshot,
    query: &schema::GeometryQuery,
    input: &schema::EvaluationInput,
) -> Result<MeasurementResult, GeometryError> {
    schema::validate_bound_evaluation(snapshot, input).map_err(GeometryError::InvalidInput)?;
    measure_query(snapshot, query, &borrowed(input))
}

/// Check a real sourced expectation against a bound canonical evaluation.
/// # Errors
/// Rejects invalid input binding or arithmetic. Does not apply the narrower core
/// 0.1 FindingCase gate to the full selected-space analysis computation.
pub fn check_bound(
    snapshot: &Snapshot,
    expectation: &Expectation,
    input: &schema::EvaluationInput,
) -> Result<Check, GeometryError> {
    schema::validate_bound_evaluation(snapshot, input).map_err(GeometryError::InvalidInput)?;
    check(snapshot, expectation, &borrowed(input))
}

/// Verification errors contain only typed categories, never private payloads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerificationError {
    InvalidContract(ValidationError),
    Computation(GeometryError),
    NotAResult,
    ResultMismatch,
}
impl std::fmt::Display for VerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for VerificationError {}

/// Validate declarations, then independently recompute their result with this
/// engine. Only the caller-assigned Finding.id may differ. No tolerance is added
/// to result equality and schema validity alone never establishes arithmetic.
/// # Errors
/// Returns InvalidContract, a typed computation failure, NotAResult for input
/// artifacts, or ResultMismatch for a contract-valid but unreproduced declaration.
pub fn verify_analysis_result(
    document: &schema::AnalysisDocument,
) -> Result<(), VerificationError> {
    document
        .validate()
        .map_err(VerificationError::InvalidContract)?;
    let matches = match &document.artifact {
        schema::AnalysisArtifact::Measurement(case) => {
            let computed = measure_query_bound(&case.snapshot, &case.query, &case.evaluation)
                .map_err(VerificationError::Computation)?;
            case.result == computed
        }
        schema::AnalysisArtifact::GeometryCheck(case) => {
            let computed = check_bound(&case.snapshot, &case.expectation, &case.evaluation)
                .map_err(VerificationError::Computation)?;
            let mut declared = case.finding.clone();
            declared.id = computed.finding.id.clone();
            case.measurement == computed.measurement && declared == computed.finding
        }
        _ => return Err(VerificationError::NotAResult),
    };
    if matches {
        Ok(())
    } else {
        Err(VerificationError::ResultMismatch)
    }
}
