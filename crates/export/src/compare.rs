//! Presentation of the existing Rust diff, using only the reviewed scene projection.
use crate::{ExportError as E, Result, observed::Aliases, types::*};
use serde::Serialize;
use serde_json::{Value as Json, json};
use uiblueprint_engine::diff::{self, DiffError, DiffLimits, GraphEntry, GraphKind, ResolvedRect};
use uiblueprint_schema::{analysis::EvaluationInput, model::*};

#[derive(Clone, Debug, Serialize)]
pub struct ComparisonResult {
    pub before: String,
    pub after: String,
    pub status: &'static str,
    pub before_context: Option<Context>,
    pub after_context: Option<Context>,
    pub scope: &'static str,
    pub entries: Vec<ComparisonEntry>,
    pub omitted_entries: usize,
    pub geometry: Vec<Json>,
    pub limitations: &'static str,
}
#[derive(Clone, Debug, Serialize)]
pub struct ComparisonEntry {
    pub kind: &'static str,
    pub before_index: Option<usize>,
    pub after_index: Option<usize>,
    pub field: Option<Field>,
    pub before_present: bool,
    pub after_present: bool,
    pub content_changed: bool,
    pub evidence_changed: bool,
    pub before: Json,
    pub after: Json,
}
const SCOPE: &str = "nodes_properties_children_metadata_relations_components_focus";
const LIMITATIONS: &str = "Recorded source facts only; absent is not deleted or created. No freshness, chronology, causality, action authority or complete Snapshot equality. Snapshot envelope, surface_records and captures are not standalone compared. Text, values and identifiers follow the public export policy; withheld content cannot be recovered from change flags. Coverage on each source remains independent. Geometry displacement requires explicit Space selection; no pixel inference.";

pub(crate) fn compile(
    b: &DrawingBrief,
    scene: &Scene,
    aliases: &mut [Aliases],
    max_output_bytes: usize,
) -> Result<Vec<ComparisonResult>> {
    let mut results = vec![];
    for request in &b.comparisons {
        let i = b
            .views
            .iter()
            .position(|v| v.id == request.before)
            .ok_or(E::InvalidReference)?;
        let j = b
            .views
            .iter()
            .position(|v| v.id == request.after)
            .ok_or(E::InvalidReference)?;
        let mut result = ComparisonResult {
            before: request.before.clone(),
            after: request.after.clone(),
            status: "different_source_bases",
            before_context: None,
            after_context: None,
            scope: SCOPE,
            entries: vec![],
            omitted_entries: 0,
            geometry: vec![],
            limitations: LIMITATIONS,
        };
        if let (
            SourceInput::Observed { snapshot: a, .. },
            SourceInput::Observed { snapshot: z, .. },
        ) = (&b.views[i].source, &b.views[j].source)
        {
            result.before_context = Some(aliases[i].context(&a.context));
            result.after_context = Some(aliases[j].context(&z.context));
            // Every encoded entry consumes more than one byte. Refuse rather than
            // emit a truncated engineering package under the caller's byte cap.
            match diff::compare_graph(
                a,
                z,
                DiffLimits {
                    max_entries: max_output_bytes,
                },
            ) {
                Ok(report) => {
                    if report.omitted_entries != 0 {
                        return Err(E::OutputLimit);
                    }
                    result.status = "compared";
                    for entry in report.entries {
                        result.entries.push(ComparisonEntry {
                            kind: kind(entry.kind),
                            before_index: entry.before_index,
                            after_index: entry.after_index,
                            field: entry.field,
                            before_present: entry.before_present,
                            after_present: entry.after_present,
                            content_changed: entry.content_changed,
                            evidence_changed: entry.evidence_changed,
                            before: facts(&scene.views[i], &entry, true)?,
                            after: facts(&scene.views[j], &entry, false)?,
                        });
                    }
                    if let Some(space) = &request.geometry_space {
                        result.geometry = geometry(a, z, space, i, j, aliases)?;
                    }
                }
                Err(DiffError::IncompatibleContext) if request.different_basis.is_some() => {
                    result.status = "incompatible_context";
                }
                Err(DiffError::IncompatibleContext) => return Err(E::IncompatibleViews),
                Err(DiffError::InvalidSnapshot(_)) => return Err(E::InvalidSource),
                Err(DiffError::Capacity) => return Err(E::OutputLimit),
            }
        } else if request.geometry_space.is_some() {
            return Err(E::InvalidSource);
        }
        results.push(result);
    }
    Ok(results)
}
fn kind(kind: GraphKind) -> &'static str {
    match kind {
        GraphKind::NodePresence => "node_presence",
        GraphKind::Property => "property",
        GraphKind::Children => "children",
        GraphKind::NodeMetadata => "node_metadata",
        GraphKind::Relation => "relation",
        GraphKind::Component => "component",
        GraphKind::FocusKeyboard => "focus_keyboard",
        GraphKind::FocusAccessibility => "focus_accessibility",
        GraphKind::FocusActiveDescendant => "focus_active_descendant",
        GraphKind::FocusTextSelection => "focus_text_selection",
        GraphKind::FocusComposition => "focus_composition",
    }
}
fn value(v: &impl Serialize) -> Result<Json> {
    serde_json::to_value(v).map_err(|_| E::InvalidInput)
}
fn facts(v: &SceneView, e: &GraphEntry, before: bool) -> Result<Json> {
    let (index, present) = if before {
        (e.before_index, e.before_present)
    } else {
        (e.after_index, e.after_present)
    };
    if !present {
        return Ok(Json::Null);
    }
    let node = || {
        v.components
            .get(index.unwrap_or(usize::MAX))
            .ok_or(E::InvalidReference)
    };
    let focus = || v.focus.as_ref().ok_or(E::InvalidSource);
    match e.kind {
        GraphKind::NodePresence => value(node()?),
        GraphKind::Property => value(
            &node()?
                .properties
                .iter()
                .find(|p| Some(p.field()) == e.field)
                .ok_or(E::InvalidReference)?,
        ),
        GraphKind::Children => value(&node()?.children),
        GraphKind::NodeMetadata => {
            let n = node()?;
            Ok(
                json!({"surface":n.surface,"native_role":n.native_role,"extensions":n.extensions,"declarations":n.declarations}),
            )
        }
        GraphKind::Relation => value(
            v.relations
                .get(index.unwrap_or(usize::MAX))
                .ok_or(E::InvalidReference)?,
        ),
        GraphKind::Component => value(
            v.mappings
                .get(index.unwrap_or(usize::MAX))
                .ok_or(E::InvalidReference)?,
        ),
        GraphKind::FocusKeyboard => value(&focus()?.keyboard),
        GraphKind::FocusAccessibility => value(&focus()?.accessibility),
        GraphKind::FocusActiveDescendant => value(&focus()?.active_descendant),
        GraphKind::FocusTextSelection => value(&focus()?.text_selection),
        GraphKind::FocusComposition => value(&focus()?.composition_state),
    }
}
// Select a full existing source Space, never manufacture a transform or units.
fn evaluation(s: &Snapshot, id: &Id) -> Result<EvaluationInput> {
    let mut selected: Option<&Space> = None;
    let mut candidates = vec![];
    for node in &s.nodes {
        for p in &node.properties {
            match p.known() {
                Some(Value::Geometry(g)) => {
                    candidates.push(&g.coordinate_space);
                    if let TransformState::Known { transform } = &g.transform {
                        candidates.extend([&transform.from, &transform.to]);
                    }
                }
                Some(Value::Baseline { space, .. }) => candidates.push(space),
                _ => (),
            }
        }
    }
    for c in &s.captures {
        if let TransformState::Known { transform } = &c.crop_transform {
            candidates.extend([&transform.from, &transform.to]);
        }
    }
    // Full equality disambiguates same-ID spaces with different units/origins.
    for candidate in candidates.into_iter().filter(|space| &space.id == id) {
        if selected.is_some_and(|old| old != candidate) {
            return Err(E::InvalidGeometry);
        }
        selected = Some(candidate);
    }
    Ok(EvaluationInput {
        snapshot_id: s.id.clone(),
        revision: s.revision,
        context: s.context.clone(),
        result_space: selected.ok_or(E::InvalidGeometry)?.clone(),
        transforms: vec![],
        conditions: None,
    })
}
fn geometry(
    a: &Snapshot,
    z: &Snapshot,
    space: &Id,
    i: usize,
    j: usize,
    aliases: &mut [Aliases],
) -> Result<Vec<Json>> {
    let old = evaluation(a, space)?;
    let new = evaluation(z, space)?;
    if old.result_space != new.result_space {
        return Err(E::IncompatibleViews);
    }
    let mut out = vec![];
    for (n, node) in a.nodes.iter().enumerate() {
        let Some((m, other)) = z
            .nodes
            .iter()
            .enumerate()
            .find(|(_, other)| other.key == node.key)
        else {
            continue;
        };
        for (field, frame) in [
            (Field::LayoutBounds, FrameKind::LayoutBounds),
            (Field::AccessibilityBounds, FrameKind::AccessibilityBounds),
            (Field::HitRegion, FrameKind::HitRegion),
            (Field::VisibleRegion, FrameKind::VisibleRegion),
            (Field::PaintBounds, FrameKind::PaintBounds),
        ] {
            if !node
                .properties
                .iter()
                .chain(&other.properties)
                .any(|p| p.field() == field)
            {
                continue;
            }
            if node.surface != other.surface {
                out.push(json!({"before_index":n,"after_index":m,"field":field,"status":"incompatible_surface","displacement":null}));
                continue;
            }
            let report = diff::compare_geometry(a, z, &node.key, frame, &old, &new)
                .map_err(|_| E::InvalidGeometry)?;
            let before = rect(&report.before_geometry, &mut aliases[i]);
            let after = rect(&report.after_geometry, &mut aliases[j]);
            let displacement = report
                .displacement
                .map(|d| json!({"dx":d.dx,"dy":d.dy,"dwidth":d.dwidth,"dheight":d.dheight}));
            out.push(json!({"before_index":n,"after_index":m,"field":field,"status":if displacement.is_some(){"known"}else{"unknown"},
                "before_space":aliases[i].space(&old.result_space),"after_space":aliases[j].space(&new.result_space),
                "before":before,"after":after,"displacement":displacement}));
        }
    }
    Ok(out)
}
fn rect(r: &ResolvedRect, aliases: &mut Aliases) -> Json {
    match r {
        ResolvedRect::Known { rect, evidence } => {
            json!({"status":"known","rect":rect,"evidence":evidence.iter().map(|e|aliases.evidence(e)).collect::<Vec<_>>()})
        }
        ResolvedRect::Unknown { reason, evidence } => {
            json!({"status":"unknown","reason":reason.as_str(),"evidence":evidence.iter().map(|e|aliases.evidence(e)).collect::<Vec<_>>()})
        }
    }
}
