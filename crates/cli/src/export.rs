//! Thin explicit-file adapter; canonical brief validation and package IO stay in export.
use crate::{Failure, arguments::limit, input, output};
use std::{ffi::OsString, path::PathBuf};
use uiblueprint_export::{
    DrawingBrief, ExportError, ExportLimits, ObservedDocumentMetadata, Purpose, SourceInput,
    ViewInput,
};

enum ExportInput {
    Brief(PathBuf),
    Snapshot {
        snapshot: PathBuf,
        metadata: PathBuf,
    },
}
struct Arguments {
    input: ExportInput,
    destination: PathBuf,
    limits: ExportLimits,
    purpose: Option<Purpose>,
    json: bool,
}
impl Arguments {
    fn parse(args: Vec<OsString>) -> Result<Self, Failure> {
        let mut args = args.into_iter();
        let (
            mut brief,
            mut destination,
            mut max_input,
            mut max_output,
            mut components,
            mut views,
            mut detail,
        ) = (None, None, None, None, None, None, None);
        let mut snapshot = None;
        let mut metadata = None;
        let mut purpose = None;
        let mut profile = false;
        let mut json = false;
        while let Some(flag) = args.next() {
            if flag == "--json" {
                if json {
                    return Err(Failure::invalid("invalid_arguments"));
                }
                json = true;
                continue;
            }
            let value = args.next().ok_or(Failure::invalid("invalid_arguments"))?;
            match flag.to_str() {
                Some("--brief") if brief.is_none() => brief = Some(PathBuf::from(value)),
                Some("--snapshot") if snapshot.is_none() => snapshot = Some(PathBuf::from(value)),
                Some("--metadata") if metadata.is_none() => metadata = Some(PathBuf::from(value)),
                Some("--out") if destination.is_none() => destination = Some(PathBuf::from(value)),
                Some("--max-input-bytes") if max_input.is_none() => max_input = Some(limit(value)?),
                Some("--max-output-bytes") if max_output.is_none() => {
                    max_output = Some(limit(value)?)
                }
                Some("--max-components") if components.is_none() => {
                    components = Some(limit(value)?)
                }
                Some("--max-views") if views.is_none() => views = Some(limit(value)?),
                Some("--components-per-detail") if detail.is_none() => detail = Some(limit(value)?),
                Some("--purpose") if purpose.is_none() => {
                    purpose = Some(match value.to_str() {
                        Some("document" | "explain") => Purpose::Document,
                        Some("propose") => Purpose::Propose,
                        Some("detail") => Purpose::Detail,
                        Some("flow") => Purpose::Flow,
                        Some("compare") => Purpose::Compare,
                        _ => return Err(Failure::invalid("invalid_arguments")),
                    })
                }
                Some("--profile") if !profile => {
                    if value != "blue-engineering" {
                        return Err(Failure::unsupported("unsupported_profile"));
                    }
                    profile = true;
                }
                _ => return Err(Failure::invalid("invalid_arguments")),
            }
        }
        let missing = Failure::invalid("invalid_arguments");
        let input = match (brief, snapshot, metadata) {
            (Some(brief), None, None) => ExportInput::Brief(brief),
            (None, Some(snapshot), Some(metadata)) => {
                if purpose.is_some_and(|p| p != Purpose::Document) {
                    return Err(missing);
                }
                ExportInput::Snapshot { snapshot, metadata }
            }
            (Some(_), _, _) => return Err(missing),
            _ => return Err(Failure::invalid("export_metadata_required")),
        };
        Ok(Self {
            input,
            destination: destination.ok_or(missing)?,
            purpose,
            json,
            limits: ExportLimits {
                max_input_bytes: max_input.ok_or(missing)?,
                max_output_bytes: max_output.ok_or(missing)?,
                max_components: components.ok_or(missing)?,
                max_views: views.ok_or(missing)?,
                components_per_detail: detail.ok_or(missing)?,
            },
        })
    }
}

pub(crate) fn execute(args: Vec<OsString>) -> Result<(Vec<u8>, u8), Failure> {
    let args = Arguments::parse(args)?;
    let mut brief = load_brief(&args)?;
    if let Some(purpose) = args.purpose {
        brief.purpose = purpose;
    }
    let package = uiblueprint_export::compile(&brief, args.limits).map_err(failure)?;
    let size = package
        .files()
        .values()
        .try_fold(0usize, |total, bytes| total.checked_add(bytes.len()))
        .ok_or(Failure::invalid("output_limit"))?;
    let receipt = receipt(&brief, size, package.files().keys().collect());
    let stdout = output::export_receipt(
        &receipt,
        args.json,
        args.limits.max_output_bytes.saturating_sub(size),
    )?;
    // Prepare and bound stdout before IO, so a response limit cannot leave a
    // successfully written directory accompanied by a failed validation result.
    package.write_new(&args.destination).map_err(failure)?;
    Ok((stdout, 0))
}
fn decode_failure(error: serde_json::Error) -> Failure {
    if error.is_data() && error.to_string().starts_with("missing field `") {
        Failure::invalid("export_metadata_required")
    } else {
        Failure::invalid("export_invalid_input")
    }
}
fn load_brief(args: &Arguments) -> Result<DrawingBrief, Failure> {
    let mut remaining = args.limits.max_input_bytes;
    match &args.input {
        ExportInput::Brief(path) => {
            let bytes = input::read(path, &mut remaining)?;
            serde_json::from_slice(&bytes).map_err(decode_failure)
        }
        ExportInput::Snapshot { snapshot, metadata } => {
            let snapshot =
                input::read_snapshot(snapshot, &mut remaining, args.limits.max_input_bytes)?;
            let bytes = input::read(metadata, &mut remaining)?;
            if bytes.iter().find(|b| !b.is_ascii_whitespace()) != Some(&b'{') {
                return Err(Failure::invalid("export_invalid_input"));
            }
            let mut annotations: ObservedDocumentMetadata =
                serde_json::from_slice(&bytes).map_err(decode_failure)?;
            annotations.not_depicted.push("State, environment and scope labels are caller annotations; coverage and source facts come from the saved Snapshot. No fresh observation.".into());
            Ok(DrawingBrief {
                purpose: Purpose::Document,
                views: vec![ViewInput {
                    id: "observed".into(),
                    title: annotations.metadata.title.clone(),
                    state: annotations.state,
                    scope: annotations.scope,
                    environment: annotations.environment,
                    safe_source_reference: annotations.safe_source_reference,
                    not_depicted: annotations.not_depicted,
                    source: SourceInput::Observed {
                        snapshot: Box::new(snapshot),
                        public_text_fields: annotations.public_text_fields,
                    },
                }],
                metadata: annotations.metadata,
                details: vec![],
                comparisons: vec![],
                transitions: vec![],
            })
        }
    }
}
fn failure(error: ExportError) -> Failure {
    match error {
        ExportError::InputLimit => Failure::invalid("input_limit"),
        ExportError::OutputLimit => Failure::invalid("output_limit"),
        ExportError::InvalidInput => Failure::invalid("export_invalid_input"),
        ExportError::InvalidSource => Failure::invalid("export_invalid_source"),
        ExportError::InvalidReference => Failure::invalid("export_invalid_reference"),
        ExportError::InvalidGeometry => Failure::invalid("export_invalid_geometry"),
        ExportError::InvalidChain => Failure::invalid("export_invalid_chain"),
        ExportError::IncompatibleViews => Failure::invalid("export_incompatible_views"),
        ExportError::PrivateContent => Failure::invalid("export_private_content"),
        ExportError::UnsupportedVerification => Failure::invalid("export_approval_record_required"),
        ExportError::DestinationExists => Failure::invalid("export_destination_exists"),
        ExportError::Io => Failure::io(),
    }
}
fn receipt(b: &DrawingBrief, package_bytes: usize, files: Vec<&String>) -> serde_json::Value {
    let views:Vec<_>=b.views.iter().map(|v| match &v.source {
        SourceInput::Observed {snapshot,..}=>serde_json::json!({"view_id":v.id,"source_kind":"observed","coverage":snapshot.coverage.status,"omitted_count":snapshot.coverage.omitted_count,"unknown_count":snapshot.coverage.unknown_count,"components":snapshot.nodes.len()}),
        SourceInput::Proposed {layout}=>serde_json::json!({"view_id":v.id,"source_kind":"proposed","coverage":"proposed","omitted_count":null,"unknown_count":null,"components":layout.components.len()}),
    }).collect();
    serde_json::json!({"result_version":"0.1.0","command":"imagegen-prompt","status":"package_written","purpose":b.purpose,
        "views":views,"package_bytes":package_bytes,"files":files,"local_numeric_validation":"checked","validation_status":"unverified",
        "approval_status":b.metadata.approval.status,"generated_image":false,"references_count":0,
        "comparison_attribution":if b.comparisons.is_empty() {"not_requested"} else {"unresolved_g02"}})
}
