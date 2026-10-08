use crate::{
    Failure,
    arguments::{Arguments, DiffArguments, InspectArguments},
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

#[cfg(all(target_os = "macos", any(feature = "macos", feature = "web")))]
pub(crate) fn trusted_input<T: serde::Serialize>(
    value: &T,
    limit: usize,
) -> Result<Vec<u8>, Failure> {
    let mut buffer = Bounded::new(limit);
    serde_json::to_writer(&mut buffer, value).map_err(|_| buffer.error())?;
    Ok(buffer.bytes)
}

#[cfg(all(target_os = "macos", any(feature = "macos", feature = "web")))]
pub(crate) fn observation_lines(
    completion: &uiblueprint_host::host_types::HostCompletion<'_>,
    limit: usize,
    output: &mut impl Write,
) -> Result<(), Failure> {
    let mut remaining = limit;
    for slot in 0..3 {
        if let Some(bytes) = completion.bytes(slot) {
            let line = bytes
                .len()
                .checked_add(1)
                .filter(|n| *n <= remaining)
                .ok_or(Failure::invalid("output_limit"))?;
            output
                .write_all(bytes)
                .and_then(|_| output.write_all(b"\n"))
                .map_err(|_| Failure::io())?;
            remaining -= line;
        }
    }
    Ok(())
}

#[cfg(all(target_os = "macos", feature = "web"))]
pub(crate) const ACTION_COMPACT_BYTES: usize = 137;

#[cfg(all(target_os = "macos", feature = "web"))]
pub(crate) fn action_compact(
    command: crate::arguments::ActionCommand,
    completion: &uiblueprint_host::host_types::HostCompletion<'_>,
    cleaned: bool,
    limit: usize,
) -> Result<Vec<u8>, Failure> {
    use crate::arguments::ActionCommand;
    use uiblueprint_host::{
        host_types::{EffectReceipt, Terminal},
        publication::ActionPublicationStatus as Status,
    };
    let preparation = if completion.action_status() == Some(Status::Prepared) {
        "prepared"
    } else if command == ActionCommand::Prepare {
        "unavailable"
    } else {
        "not_requested"
    };
    let delivery = match completion.effect {
        EffectReceipt::NotDispatched => "not_dispatched",
        EffectReceipt::Possible { .. } => "unknown",
        EffectReceipt::Confirmed { .. } => "confirmed",
    };
    let verification = match completion.action_status() {
        Some(Status::VerifiedSuccess) => "pass",
        Some(Status::VerifiedMismatch) => "fail",
        Some(Status::Refused) => "refused",
        Some(Status::Prepared) => "not_requested",
        _ => "unknown",
    };
    let terminal = match completion.terminal {
        Terminal::Completed => "completed",
        Terminal::Failed(_) => "failed",
        Terminal::Cancelled => "cancelled",
        Terminal::TimedOut => "timed_out",
    };
    let completeness = if completion.missing() == 0 {
        "complete"
    } else {
        "incomplete"
    };
    let cleanup = if cleaned { "complete" } else { "pending" };
    let mut output = Bounded::new(limit);
    writeln!(output, "preparation={preparation} delivery={delivery} verification={verification} completeness={completeness} protocol={terminal} cleanup={cleanup}").map_err(|_| output.error())?;
    Ok(output.bytes)
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

fn inspect_node(output: &mut Bounded, node: &Node, view: Projection) -> io::Result<()> {
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
    for geometry_first in [view == Projection::Design, view != Projection::Design] {
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
    Ok(())
}

pub(crate) fn inspect(
    args: &InspectArguments,
    view: &uiblueprint_engine::scope::ComponentView<'_>,
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
        inspect_node(output, node, args.view)?;
        let total_parts = view.parts.len() + view.omitted_parts;
        writeln!(
            output,
            "component_selection mappings={} returned_parts={} omitted_parts={} returned_relations={} omitted_relations={} parts_exposure={} selection_truncated={}",
            view.mappings.len(),
            view.parts.len(),
            view.omitted_parts,
            view.relations.len(),
            view.omitted_relations,
            if total_parts == 0 {
                "not_exposed"
            } else {
                "recorded"
            },
            view.omitted_parts > 0 || view.omitted_relations > 0
        )?;
        for component in &view.mappings {
            writeln!(output, "declared_component={component:?}")?;
        }
        if args.view == Projection::Design {
            for part in &view.parts {
                writeln!(
                    output,
                    "part_node={:?} surface={:?} native_role={:?}",
                    part.key, part.surface, part.native_role
                )?;
                inspect_node(output, part, Projection::Design)?;
            }
        }
        for neighbor in &view.relations {
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

pub(crate) fn recorded_diff(
    args: &DiffArguments,
    result: &uiblueprint_engine::diff::RecordedDiff<'_>,
) -> Result<Vec<u8>, Failure> {
    use uiblueprint_engine::diff::{Difference, Presence};
    let mut buffer = Bounded::new(args.max_output);
    if args.json {
        buffer.write_all(b"{\"output_version\":\"1.0.0\",\"kind\":\"recorded_difference\",\"source\":\"saved\",\"live_revalidated\":false,\"comparison_scope\":\"node_presence_and_properties\",\"before\":").map_err(|_|buffer.error())?;
        serde_json::to_writer(&mut buffer, result.before).map_err(|_| buffer.error())?;
        buffer
            .write_all(b",\"after\":")
            .map_err(|_| buffer.error())?;
        serde_json::to_writer(&mut buffer, result.after).map_err(|_| buffer.error())?;
        buffer
            .write_all(b",\"entries\":[")
            .map_err(|_| buffer.error())?;
        for (index, entry) in result.entries.iter().enumerate() {
            let (kind, key, field, presence, content, evidence) = match *entry {
                Difference::NodePresence {
                    presence,
                    before,
                    after,
                } => {
                    let node = match (before, after) {
                        (Some(n), _) | (_, Some(n)) => n,
                        _ => {
                            return Err(Failure {
                                code: "internal_error",
                                exit: 1,
                            });
                        }
                    };
                    ("node_presence", &node.key, None, presence, true, false)
                }
                Difference::Property {
                    before_node,
                    field,
                    presence,
                    content_changed,
                    evidence_changed,
                    ..
                } => (
                    "property",
                    &before_node.key,
                    Some(field),
                    presence,
                    content_changed,
                    evidence_changed,
                ),
            };
            if index > 0 {
                buffer.write_all(b",").map_err(|_| buffer.error())?;
            }
            write!(buffer, "{{\"kind\":\"{kind}\",\"key\":").map_err(|_| buffer.error())?;
            serde_json::to_writer(&mut buffer, key).map_err(|_| buffer.error())?;
            buffer
                .write_all(b",\"field\":")
                .map_err(|_| buffer.error())?;
            serde_json::to_writer(&mut buffer, &field).map_err(|_| buffer.error())?;
            write!(buffer,",\"before_present\":{},\"after_present\":{},\"content_changed\":{content},\"evidence_changed\":{evidence}}}",
                presence!=Presence::AfterOnly,presence!=Presence::BeforeOnly).map_err(|_|buffer.error())?;
        }
        writeln!(buffer, "],\"omitted_entries\":{}}}", result.omitted_entries)
            .map_err(|_| buffer.error())?;
    } else {
        writeln!(buffer,"comparison=saved_observations live_revalidation=not_performed comparison_scope=node_presence_and_properties omitted_entries={}",result.omitted_entries).map_err(|_|buffer.error())?;
        for (side, snapshot) in [("before", result.before), ("after", result.after)] {
            writeln!(
                buffer,
                "{side} snapshot={:?} revision={} context={:?} coverage={:?}",
                snapshot.id.0, snapshot.revision, snapshot.context, snapshot.coverage
            )
            .map_err(|_| buffer.error())?;
            for observation in &snapshot.observations {
                writeln!(buffer, "{side}_observation={observation:?}")
                    .map_err(|_| buffer.error())?;
            }
        }
        for entry in &result.entries {
            match entry {
                Difference::NodePresence {
                    presence,
                    before,
                    after,
                } => {
                    writeln!(buffer,"difference=node_presence presence={presence:?} before_key={:?} after_key={:?} deletion_claim=not_made",before.map(|n|&n.key),after.map(|n|&n.key)).map_err(|_|buffer.error())?;
                }
                Difference::Property {
                    before_node,
                    field,
                    presence,
                    before,
                    after,
                    content_changed,
                    evidence_changed,
                    ..
                } => {
                    writeln!(buffer,"difference=property key={:?} field={field:?} presence={presence:?} content_changed={content_changed} evidence_changed={evidence_changed} before={before:?} after={after:?}",before_node.key).map_err(|_|buffer.error())?;
                }
            }
        }
    }
    Ok(buffer.bytes)
}

pub(crate) fn neighbors(
    args: &crate::arguments::NeighborArguments,
    view: &uiblueprint_engine::scope::NeighborView<'_>,
) -> Result<Vec<u8>, Failure> {
    use uiblueprint_engine::scope::RelationDirection;
    let mut buffer = Bounded::new(args.max_output);
    let direction = |d| match d {
        RelationDirection::Incoming => "incoming",
        RelationDirection::Outgoing => "outgoing",
        RelationDirection::SelfLoop => "self_loop",
    };
    if args.json {
        // Stream canonical borrowed objects through the existing bounded writer.
        // The original source is separate from, and never capped by, selection.
        buffer.write_all(b"{\"output_version\":\"1.0.0\",\"kind\":\"relation_neighbors\",\"source\":\"saved\",\"live_revalidated\":false,\"selector\":").map_err(|_|buffer.error())?;
        serde_json::to_writer(&mut buffer, &view.seed.key).map_err(|_| buffer.error())?;
        buffer
            .write_all(b",\"snapshot\":")
            .map_err(|_| buffer.error())?;
        serde_json::to_writer(&mut buffer, view.snapshot).map_err(|_| buffer.error())?;
        write!(buffer,",\"selection\":{{\"max_relations\":{},\"returned_relations\":{},\"omitted_relations\":{},\"truncated\":{}}},\"neighbors\":[",args.max_relations,view.neighbors.len(),view.omitted_relations,view.omitted_relations>0).map_err(|_|buffer.error())?;
        for (index, neighbor) in view.neighbors.iter().enumerate() {
            if index > 0 {
                buffer.write_all(b",").map_err(|_| buffer.error())?;
            }
            write!(
                buffer,
                "{{\"direction\":\"{}\",\"relation\":",
                direction(neighbor.direction)
            )
            .map_err(|_| buffer.error())?;
            serde_json::to_writer(&mut buffer, neighbor.relation).map_err(|_| buffer.error())?;
            buffer
                .write_all(b",\"counterpart\":")
                .map_err(|_| buffer.error())?;
            serde_json::to_writer(&mut buffer, neighbor.counterpart).map_err(|_| buffer.error())?;
            buffer.write_all(b"}").map_err(|_| buffer.error())?;
        }
        buffer.write_all(b"]}\n").map_err(|_| buffer.error())?;
    } else {
        let written = (|| -> io::Result<()> {
            writeln!(
                buffer,
                "neighbors=saved_observation live_revalidated=false snapshot={:?} revision={} context={:?}",
                view.snapshot.id, view.snapshot.revision, view.snapshot.context
            )?;
            writeln!(buffer, "source_coverage={:?}", view.snapshot.coverage)?;
            for observation in &view.snapshot.observations {
                writeln!(buffer, "recorded_observation={observation:?}")?;
            }
            writeln!(
                buffer,
                "selection max_relations={} returned_relations={} omitted_relations={} truncated={}",
                args.max_relations,
                view.neighbors.len(),
                view.omitted_relations,
                view.omitted_relations > 0
            )?;
            writeln!(buffer, "seed={:?}", view.seed)?;
            for neighbor in &view.neighbors {
                writeln!(
                    buffer,
                    "direction={} relation={:?} counterpart={:?}",
                    direction(neighbor.direction),
                    neighbor.relation,
                    neighbor.counterpart
                )?;
            }
            Ok(())
        })();
        written.map_err(|_| buffer.error())?;
    }
    Ok(buffer.bytes)
}

pub(crate) fn geometry_diff(
    args: &DiffArguments,
    result: &uiblueprint_engine::diff::GeometryDifference<'_>,
) -> Result<Vec<u8>, Failure> {
    use uiblueprint_engine::diff::ResolvedRect;
    let mut buffer = Bounded::new(args.max_output);
    if args.json {
        buffer.write_all(b"{\"output_version\":\"1.0.0\",\"kind\":\"geometry_difference\",\"source\":\"saved\",\"live_revalidated\":false,\"selector\":").map_err(|_|buffer.error())?;
        serde_json::to_writer(&mut buffer, result.key).map_err(|_| buffer.error())?;
        buffer
            .write_all(b",\"frame_kind\":")
            .map_err(|_| buffer.error())?;
        serde_json::to_writer(&mut buffer, &result.frame_kind).map_err(|_| buffer.error())?;
        for (label, snapshot) in [("before", result.before), ("after", result.after)] {
            write!(buffer, ",\"{label}\":").map_err(|_| buffer.error())?;
            serde_json::to_writer(&mut buffer, snapshot).map_err(|_| buffer.error())?;
        }
        for (label, input) in [
            ("before_evaluation", result.before_evaluation),
            ("after_evaluation", result.after_evaluation),
        ] {
            write!(buffer, ",\"{label}\":").map_err(|_| buffer.error())?;
            serde_json::to_writer(&mut buffer, input).map_err(|_| buffer.error())?;
        }
        buffer
            .write_all(b",\"result_space\":")
            .map_err(|_| buffer.error())?;
        serde_json::to_writer(&mut buffer, &result.before_evaluation.result_space)
            .map_err(|_| buffer.error())?;
        for (label, geometry) in [
            ("before_geometry", &result.before_geometry),
            ("after_geometry", &result.after_geometry),
        ] {
            write!(buffer, ",\"{label}\":").map_err(|_| buffer.error())?;
            match geometry {
                ResolvedRect::Known { rect, evidence } => {
                    buffer
                        .write_all(b"{\"status\":\"known\",\"rect\":")
                        .map_err(|_| buffer.error())?;
                    serde_json::to_writer(&mut buffer, rect).map_err(|_| buffer.error())?;
                    buffer
                        .write_all(b",\"evidence\":")
                        .map_err(|_| buffer.error())?;
                    serde_json::to_writer(&mut buffer, evidence).map_err(|_| buffer.error())?;
                }
                ResolvedRect::Unknown { reason, evidence } => {
                    buffer
                        .write_all(b"{\"status\":\"unknown\",\"reason\":")
                        .map_err(|_| buffer.error())?;
                    serde_json::to_writer(&mut buffer, reason).map_err(|_| buffer.error())?;
                    buffer
                        .write_all(b",\"evidence\":")
                        .map_err(|_| buffer.error())?;
                    serde_json::to_writer(&mut buffer, evidence).map_err(|_| buffer.error())?;
                }
            }
            buffer.write_all(b"}").map_err(|_| buffer.error())?;
        }
        buffer
            .write_all(b",\"displacement\":")
            .map_err(|_| buffer.error())?;
        if let Some(d) = result.displacement {
            buffer.write_all(b"{").map_err(|_| buffer.error())?;
            for (i, (name, value)) in [
                ("dx", d.dx),
                ("dy", d.dy),
                ("dwidth", d.dwidth),
                ("dheight", d.dheight),
            ]
            .into_iter()
            .enumerate()
            {
                if i > 0 {
                    buffer.write_all(b",").map_err(|_| buffer.error())?;
                }
                write!(buffer, "\"{name}\":").map_err(|_| buffer.error())?;
                serde_json::to_writer(&mut buffer, &value).map_err(|_| buffer.error())?;
            }
            buffer.write_all(b"}").map_err(|_| buffer.error())?;
        } else {
            buffer.write_all(b"null").map_err(|_| buffer.error())?;
        }
        buffer.write_all(b"}\n").map_err(|_| buffer.error())?;
    } else {
        writeln!(buffer,"comparison=saved_geometry live_revalidation=not_performed selector={:?} frame_kind={:?} result_space={:?}",result.key,result.frame_kind,result.before_evaluation.result_space).map_err(|_|buffer.error())?;
        for (side, snapshot, geometry) in [
            ("before", result.before, &result.before_geometry),
            ("after", result.after, &result.after_geometry),
        ] {
            writeln!(
                buffer,
                "{side} snapshot={:?} context={:?} coverage={:?} geometry={geometry:?}",
                snapshot.id, snapshot.context, snapshot.coverage
            )
            .map_err(|_| buffer.error())?;
        }
        writeln!(buffer, "displacement={:?}", result.displacement).map_err(|_| buffer.error())?;
    }
    Ok(buffer.bytes)
}
