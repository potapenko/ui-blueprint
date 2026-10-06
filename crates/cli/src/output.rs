use crate::{Failure, arguments::Arguments};
use std::io::{self, Write};
use uiblueprint_engine::MeasurementResult;
use uiblueprint_schema::{SchemaVersion, analysis::*, model::*};

struct Bounded {
    bytes: Vec<u8>,
    limit: usize,
    failure: Option<Failure>,
}
impl Bounded {
    fn new(limit: usize) -> Self {
        Self {
            bytes: Vec::new(),
            limit,
            failure: None,
        }
    }
    fn error(&self) -> Failure {
        self.failure.unwrap_or(Failure {
            code: "internal_error",
            exit: 1,
        })
    }
}
impl Write for Bounded {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > self.limit.saturating_sub(self.bytes.len()) {
            self.failure = Some(Failure::invalid("output_limit"));
            return Err(io::Error::other("output_limit"));
        }
        if self.bytes.try_reserve(bytes.len()).is_err() {
            self.failure = Some(Failure::io());
            return Err(io::Error::other("allocation_failure"));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub(crate) fn json(
    snapshot: Snapshot,
    expectation: Expectation,
    finding: Finding,
    limit: usize,
) -> Result<Vec<u8>, Failure> {
    let document = Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Finding(Box::new(FindingCase {
            snapshot,
            expectation,
            finding,
        })),
    };
    let mut output = Bounded::new(limit);
    serde_json::to_writer(&mut output, &document).map_err(|_| output.error())?;
    output.write_all(b"\n").map_err(|_| output.error())?;
    Document::from_json(&output.bytes, limit)
        .map_err(|_| Failure::unsupported("unsupported_result_version"))?;
    Ok(output.bytes)
}

pub(crate) fn analysis_json(document: AnalysisDocument, limit: usize) -> Result<Vec<u8>, Failure> {
    let validation = match &document.artifact {
        AnalysisArtifact::Measurement(case) => validate_measurement_case(case),
        AnalysisArtifact::GeometryCheck(case) => validate_geometry_check_case(case),
        _ => return Err(Failure::invalid("invalid_analysis")),
    };
    validation.map_err(|_| Failure::invalid("invalid_analysis"))?;
    let mut output = Bounded::new(limit);
    serde_json::to_writer(&mut output, &document).map_err(|_| output.error())?;
    output.write_all(b"\n").map_err(|_| output.error())?;
    let decoded = AnalysisDocument::from_json(&output.bytes, limit)
        .map_err(|_| Failure::invalid("invalid_analysis"))?;
    if decoded != document {
        return Err(Failure {
            code: "analysis_roundtrip_mismatch",
            exit: 1,
        });
    }
    Ok(output.bytes)
}

pub(crate) fn compact(
    args: &Arguments,
    snapshot: &Snapshot,
    query: &GeometryQuery,
    expectation: Option<&Expectation>,
    evaluation: &EvaluationInput,
    measurement: &MeasurementResult,
    status: Option<CheckStatus>,
) -> Result<Vec<u8>, Failure> {
    let mut buffer = Bounded::new(args.max_output);
    let written = (|| -> io::Result<()> {
        let output = &mut buffer;
        writeln!(
            output,
            "analysis=saved_snapshot command={:?} snapshot={:?} revision={} scope={:?}",
            args.command, snapshot.id, snapshot.revision, snapshot.context.scope_id
        )?;
        writeln!(
            output,
            "target={:?} surfaces={:?}",
            snapshot.context.target, snapshot.context.surfaces
        )?;
        writeln!(
            output,
            "coverage={:?} selected_fields={:?}",
            snapshot.coverage, snapshot.context.fields
        )?;
        writeln!(
            output,
            "limits input_bytes={} output_bytes={}",
            args.max_input, args.max_output
        )?;
        for observation in &snapshot.observations {
            writeln!(
                output,
                "observation={:?} source={:?} stored_source={:?} freshness={:?} consistency={:?} coverage={:?} interval={}..{} {:?} clock={:?}",
                observation.id.0,
                observation.source_namespace.0,
                observation.answer_source,
                observation.freshness,
                observation.consistency,
                observation.coverage.status,
                observation.start,
                observation.end,
                observation.time_unit,
                observation.clock_domain.0
            )?;
        }
        writeln!(
            output,
            "query={:?} operation={:?} quantity={:?} units={:?} applies_when={:?}",
            query.id.0, query.operation, query.quantity_kind, query.units, query.applies_when
        )?;
        if let Some(expectation) = expectation {
            writeln!(
                output,
                "expectation={:?} expected_from={:?}",
                expectation.id.0, expectation.expected_from.0
            )?;
            if let Rule::Geometry {
                expected,
                comparison,
                tolerance,
                ..
            } = expectation.rule
            {
                writeln!(
                    output,
                    "expected={expected} comparison={comparison:?} tolerance={tolerance}"
                )?;
            }
        }
        for anchor in &query.anchors {
            writeln!(
                output,
                "anchor={:?}:{:?} frame={:?} space={:?} axis={:?} fraction={}",
                anchor.element.namespace.0,
                anchor.element.key.0,
                anchor.frame_kind,
                anchor.coordinate_space.id.0,
                anchor.axis.0,
                anchor.fraction
            )?;
        }
        writeln!(
            output,
            "selected_space={:?} supplied_transforms={} observed_conditions={:?}",
            evaluation.result_space,
            evaluation.transforms.len(),
            evaluation.conditions
        )?;
        if let Some(status) = status {
            writeln!(output, "check={status:?}")?;
        }
        match measurement {
            MeasurementResult::Known { measurement } => {
                writeln!(
                    output,
                    "measurement=known value={:?} space={:?} details={:?}",
                    measurement.value, measurement.space, measurement.details
                )?;
                for evidence in &measurement.evidence {
                    writeln!(output, "evidence={evidence:?}")?;
                }
            }
            MeasurementResult::Unknown { reason, evidence } => {
                writeln!(output, "measurement=unknown reason={}", reason.as_str())?;
                for evidence in evidence {
                    writeln!(output, "evidence={evidence:?}")?;
                }
            }
        }
        Ok(())
    })();
    written.map_err(|_| buffer.error())?;
    Ok(buffer.bytes)
}

/// The export-owned receipt is bounded using the same writer as existing output.
/// It is not a normalized measurement/observation schema.
pub(crate) fn export_receipt(
    receipt: &serde_json::Value,
    json: bool,
    limit: usize,
) -> Result<Vec<u8>, Failure> {
    let mut output = Bounded::new(limit);
    if json {
        serde_json::to_writer(&mut output, receipt).map_err(|_| output.error())?;
        output.write_all(b"\n").map_err(|_| output.error())?;
    } else {
        writeln!(output,
            "command=imagegen-prompt status=package_written purpose={} package_bytes={} local_numeric_validation=checked validation_status=unverified approval_status={} generated_image=false references_count=0 comparison_attribution={}\nviews={}",
            receipt["purpose"], receipt["package_bytes"], receipt["approval_status"],
            receipt["comparison_attribution"], receipt["views"]
        ).map_err(|_| output.error())?;
    }
    Ok(output.bytes)
}
