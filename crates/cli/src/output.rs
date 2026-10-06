use crate::{Failure, arguments::Arguments};
use std::io::{self, Write};
use uiblueprint_engine::MeasurementResult;
use uiblueprint_schema::{SchemaVersion, model::*};

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
        .map_err(|_| Failure::unsupported("consumer_contract_gap"))?;
    Ok(output.bytes)
}

pub(crate) fn compact(
    args: &Arguments,
    snapshot: &Snapshot,
    expectation: &Expectation,
    measurement: &MeasurementResult,
    status: Option<CheckStatus>,
) -> Result<Vec<u8>, Failure> {
    let mut output = Bounded::new(args.max_output);
    write_compact(
        &mut output,
        args,
        snapshot,
        expectation,
        measurement,
        status,
    )
    .map_err(|_| output.error())?;
    Ok(output.bytes)
}
fn write_compact(
    output: &mut impl Write,
    args: &Arguments,
    snapshot: &Snapshot,
    expectation: &Expectation,
    measurement: &MeasurementResult,
    status: Option<CheckStatus>,
) -> io::Result<()> {
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
        "expectation={:?} expected_from={:?} applies_when={:?}",
        expectation.id.0, expectation.expected_from.0, expectation.applies_when
    )?;
    if let Rule::Geometry {
        operation,
        anchors,
        expected,
        comparison,
        quantity_kind,
        units,
        tolerance,
    } = &expectation.rule
    {
        writeln!(
            output,
            "rule={operation:?} expected={expected} comparison={comparison:?} tolerance={tolerance} quantity={quantity_kind:?} units={units:?}"
        )?;
        for anchor in anchors {
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
    }
    if let Some(status) = status {
        writeln!(output, "check={status:?}")?;
    }
    match measurement {
        MeasurementResult::Known(measurement) => {
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
}
