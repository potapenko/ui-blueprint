use crate::{
    Failure,
    arguments::{Arguments, InspectArguments},
};
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

fn inspect_property(
    output: &mut Bounded,
    field: Field,
    property: Option<&Property>,
) -> io::Result<()> {
    write!(output, "property={field:?} ")?;
    let Some(Property::Requested {
        state,
        sensitivity,
        evidence,
        ..
    }) = property
    else {
        return writeln!(output, "selection=not_requested");
    };
    write!(output, "selection=requested sensitivity={sensitivity:?} ")?;
    match state {
        Availability::Known { value } => write!(output, "availability=known value={value:?}"),
        Availability::Unknown { reason } => {
            write!(output, "availability=unknown reason={:?}", reason.0)
        }
        Availability::Unsupported { reason } => {
            write!(output, "availability=unsupported reason={:?}", reason.0)
        }
        Availability::Redacted {} => write!(output, "availability=redacted"),
    }?;
    writeln!(output, " evidence={evidence:?}")
}

pub(crate) fn inspect(
    args: &InspectArguments,
    view: &uiblueprint_engine::scope::NeighborView<'_>,
) -> Result<Vec<u8>, Failure> {
    let mut buffer = Bounded::new(args.max_output);
    if args.json {
        // CLI-owned envelope; stream borrowed canonical fields directly. No
        // unbounded Value, cloned Snapshot or new core/analysis artifact owner.
        buffer.write_all(b"{\"output_version\":\"1.0.0\",\"kind\":\"inspection\",\"source\":\"saved\",\"live_revalidated\":false,\"selector\":")
            .map_err(|_| buffer.error())?;
        serde_json::to_writer(&mut buffer, &view.seed.key).map_err(|_| buffer.error())?;
        buffer
            .write_all(b",\"requested_view\":")
            .map_err(|_| buffer.error())?;
        serde_json::to_writer(&mut buffer, &args.view).map_err(|_| buffer.error())?;
        buffer
            .write_all(b",\"snapshot\":")
            .map_err(|_| buffer.error())?;
        serde_json::to_writer(&mut buffer, view.snapshot).map_err(|_| buffer.error())?;
        buffer.write_all(b"}\n").map_err(|_| buffer.error())?;
        return Ok(buffer.bytes);
    }
    let written = (|| -> io::Result<()> {
        let snapshot = view.snapshot;
        let node = view.seed;
        let output = &mut buffer;
        writeln!(
            output,
            "inspection=saved_observation live_revalidation=not_performed requested_view={:?} original_projection={:?}",
            args.view, snapshot.context.projection
        )?;
        writeln!(
            output,
            "snapshot={:?} revision={} context={:?}",
            snapshot.id.0, snapshot.revision, snapshot.context
        )?;
        writeln!(
            output,
            "coverage={:?} limits input_bytes={} output_bytes={}",
            snapshot.coverage, args.max_input, args.max_output
        )?;
        for observation in &snapshot.observations {
            writeln!(output, "recorded_observation={observation:?}")?;
        }
        writeln!(
            output,
            "node={:?} surface={:?} native_role={:?}",
            node.key, node.surface, node.native_role
        )?;
        writeln!(output, "recorded_focus={:?}", snapshot.focus)?;
        const FIELDS: &[Field] = &[
            Field::Role,
            Field::Name,
            Field::AccessibilityName,
            Field::VisibleText,
            Field::Value,
            Field::Placeholder,
            Field::Focused,
            Field::Enabled,
            Field::LayoutBounds,
            Field::AccessibilityBounds,
            Field::HitRegion,
            Field::VisibleRegion,
            Field::PaintBounds,
        ];
        let geometry = |field| {
            matches!(
                field,
                Field::LayoutBounds
                    | Field::AccessibilityBounds
                    | Field::HitRegion
                    | Field::VisibleRegion
                    | Field::PaintBounds
                    | Field::Baseline
            )
        };
        for geometry_first in [
            args.view == Projection::Design,
            args.view != Projection::Design,
        ] {
            for &field in FIELDS.iter().filter(|&&f| geometry(f) == geometry_first) {
                inspect_property(
                    output,
                    field,
                    node.properties.iter().find(|p| p.field() == field),
                )?;
            }
            for property in node
                .properties
                .iter()
                .filter(|p| !FIELDS.contains(&p.field()) && geometry(p.field()) == geometry_first)
            {
                inspect_property(output, property.field(), Some(property))?;
            }
        }
        for extension in &node.extensions {
            writeln!(
                output,
                "extension_namespace={:?} name={:?} property={:?}",
                extension.namespace.0, extension.name.0, extension.property
            )?;
        }
        for declaration in &node.source_declarations {
            writeln!(output, "source_declaration={declaration:?}")?;
        }
        writeln!(output, "recorded_children={:?}", node.children)?;
        for component in snapshot
            .components
            .iter()
            .filter(|c| c.members.contains(&node.key))
        {
            writeln!(output, "declared_component={component:?}")?;
        }
        for neighbor in &view.neighbors {
            writeln!(
                output,
                "relation_direction={:?} relation={:?} counterpart={:?}",
                neighbor.direction, neighbor.relation, neighbor.counterpart.key
            )?;
        }
        Ok(())
    })();
    written.map_err(|_| buffer.error())?;
    Ok(buffer.bytes)
}
