use crate::{
    ExportError as E, Package, Result,
    observed::{self, Aliases},
    package::{bounded_compact_json, bounded_json},
    proposal,
    types::*,
    validate::{self, require, text, view},
};
use serde::Serialize;
use std::collections::BTreeMap;
use uiblueprint_engine::{self as engine, EvaluationContext, GeometryQuery, MeasurementResult};
use uiblueprint_schema::{SchemaVersion, model::*, validation};
const GUIDE: &str = "UIB.DRAWING@1.2";

/// Compile the full selected scope. Performs no IO, collection, generation or approval.
pub fn compile(brief: &DrawingBrief, limits: ExportLimits) -> Result<Package> {
    validate::validate(brief, limits)?;
    let mut scene = Scene {
        guide: GUIDE,
        views: vec![],
        flow: vec![],
        comparisons: brief.comparisons.clone(),
        comparison_results: vec![],
    };
    let mut dims = vec![];
    let mut alias_maps = vec![];
    for v in &brief.views {
        let mut aliases = Aliases::default();
        let mut sv = SceneView {
            design_analysis: vec![],
            snapshot_ref: None,
            snapshot_revision: None,
            id: v.id.clone(),
            title: v.title.clone(),
            source_kind: SourceKind::Proposed,
            source: v.safe_source_reference.clone(),
            state: v.state.clone(),
            scope: v.scope.clone(),
            environment: v.environment.clone(),
            coverage: None,
            surfaces: vec![],
            observations: vec![],
            not_depicted: v.not_depicted.clone(),
            components: vec![],
            relations: vec![],
            mappings: vec![],
            focus: None,
            requirements: vec![],
            unknowns: vec![],
        };
        let (dimensions, chains) = match &v.source {
            SourceInput::Proposed { layout } => {
                sv.components = proposal::components(layout);
                sv.requirements = layout.requirements.clone();
                sv.unknowns = layout.unknowns.clone();
                (layout.dimensions.clone(), layout.chains.clone())
            }
            SourceInput::Observed {
                snapshot: s,
                public_text_fields,
            } => {
                sv.source_kind = SourceKind::Observed;
                sv.snapshot_ref = Some(aliases.id(&s.id));
                sv.snapshot_revision = Some(s.revision);
                sv.components = observed::components(s, public_text_fields, &mut aliases);
                sv.surfaces = aliases.surfaces(s);
                sv.coverage = Some(aliases.coverage(&s.coverage));
                sv.observations = aliases.observations(s);
                sv.relations = aliases.relations(s);
                sv.mappings = aliases.mappings(s);
                sv.focus = Some(aliases.focus(&s.focus));
                sv.not_depicted.push("Pixels and unreviewed text are excluded; stored observation, not a new measurement".into());
                sv.unknowns.push("Unavailable properties retain unknown, unsupported, redacted or not_requested. Padding, radius and baseline are unknown unless explicitly reported.".into());
                let mut dimensions = observed::dimensions(s, &mut aliases)?;
                control_dimensions(s, &sv, &mut aliases, &mut dimensions)?;
                (dimensions, vec![])
            }
        };
        sv.design_analysis = crate::design::compile(v, &sv, &mut aliases, limits)?;
        dims.push(ViewDimensions {
            view: v.id.clone(),
            dimensions,
            chains,
        });
        scene.views.push(sv);
        alias_maps.push(aliases);
    }
    scene.comparison_results =
        crate::compare::compile(brief, &scene, &mut alias_maps, limits.max_output_bytes)?;
    for request in &mut scene.comparisons {
        if let Some(space) = &request.geometry_space {
            let index = brief
                .views
                .iter()
                .position(|v| v.id == request.before)
                .ok_or(E::InvalidReference)?;
            request.geometry_space = Some(alias_maps[index].id(space));
        }
    }
    scene.flow = flow(brief, &mut alias_maps)?;
    let sheets = sheets(brief, &scene, limits)?;
    let prompt = prompt(brief, &scene, &dims, &sheets, limits.max_output_bytes)?;
    let mut sheet_prompts = Vec::new();
    let mut sheet_files = Vec::new();
    let mut required_dimension_ids = Vec::new();
    if scene.views.len() == 1
        && sheets.iter().any(|s| s.kind == "detail")
        && matches!(
            brief.purpose,
            Purpose::Document | Purpose::Propose | Purpose::Detail
        )
    {
        let v = &scene.views[0];
        let partial = clipped_edges(&brief.views[0].source);
        required_dimension_ids = drawing_dimensions(
            v,
            &dims[0],
            &partial,
            None,
            &sheets,
            limits.max_output_bytes,
        )?
        .dimension_ids;
        let mut assigned = Vec::new();
        let mut sheet_remaining = limits.max_output_bytes;
        for sheet in &sheets {
            let p = drawing_dimensions(
                v,
                &dims[0],
                &partial,
                Some(sheet),
                &sheets,
                limits.max_output_bytes,
            )?;
            assigned.extend(p.dimension_ids.clone());
            let file = format!("prompt-{}.txt", sheet.id);
            let instructions = format!(
                "Generate ONLY sheet {}: {}. {} No other sheets or montage.",
                sheet.id,
                if sheet.title == "Scope detail; preserve every listed object" {
                    if p.assignment == Assignment::Measurements {
                        "Assigned frame measurements"
                    } else {
                        "Selected source fragment"
                    }
                } else {
                    &sheet.title
                },
                if sheet.kind == "general" {
                    "Complete selected-scope artwork overview; only this sheet's assigned measurements. Other measurements belong to separate detail sheets."
                } else if p.dimension_ids.is_empty() {
                    "Selected source detail only; no prepared measurements on this sheet."
                } else {
                    "Measurement sheet only: empty source-frame sketches and exact outside captions, no UI artwork or whole-window copy. All selected UI artwork remains on G01."
                }
            );
            let analysis = if sheet.kind == "general" {
                crate::design::text(&v.design_analysis, limits.max_output_bytes)?
            } else {
                String::new()
            };
            let prompt = render_view(brief, v, &p, instructions, analysis, sheet_remaining)?;
            sheet_remaining = sheet_remaining
                .checked_sub(prompt.len())
                .ok_or(E::OutputLimit)?;
            sheet_prompts.push(serde_json::json!({"sheet_id":sheet.id,"view":v.id,"file":file,"component_ids":p.component_ids,"dimension_ids":p.dimension_ids}));
            sheet_files.push((file, prompt.into_bytes()));
        }
        let mut required = required_dimension_ids.clone();
        required.sort();
        assigned.sort();
        require(
            assigned == required && assigned.windows(2).all(|w| w[0] != w[1]),
            E::InvalidReference,
        )?;
    }
    let version = if !sheet_prompts.is_empty() {
        "0.4.0"
    } else if brief.views.iter().any(|v| !v.design_analysis.is_empty()) {
        "0.3.0"
    } else if brief.comparisons.is_empty() {
        "0.1.0"
    } else {
        "0.2.0"
    };
    let mut files = BTreeMap::new();
    let mut remaining = limits.max_output_bytes;
    for (name, bytes) in [
        ("scene.json", bounded_json(&scene, remaining)?),
        ("dimensions.json", bounded_json(&dims, remaining)?),
        ("sheets.json", bounded_json(&sheets, remaining)?),
        ("prompt.txt", prompt.into_bytes()),
        (
            "drawing-brief.md",
            drawing_brief(brief, &scene, &dims, &sheets, limits.max_output_bytes)?.into_bytes(),
        ),
    ] {
        remaining = remaining.checked_sub(bytes.len()).ok_or(E::OutputLimit)?;
        files.insert(name.into(), bytes);
    }
    for (name, bytes) in sheet_files {
        remaining = remaining.checked_sub(bytes.len()).ok_or(E::OutputLimit)?;
        files.insert(name, bytes);
    }
    let mut manifest = serde_json::json!({"package_version":version,"guide":GUIDE,"guide_reference":"docs engineering-blueprint-guide.md revision 1.2",
        "document":brief.metadata,"purpose":brief.purpose,"source_kinds":scene.views.iter().map(|v|v.source_kind).collect::<Vec<_>>(),
        "validation_status":"unverified","local_numeric_validation":"checked","approval_status":brief.metadata.approval.status,
        "scale_mode":"schematic","generated_image":false,"references":[],
        "files":["manifest.json","drawing-brief.md","scene.json","dimensions.json","sheets.json","prompt.txt"],
        "retention_owner":brief.metadata.owner,"retention_condition":brief.metadata.retention});
    if !sheet_prompts.is_empty() {
        let mut names = files.keys().cloned().collect::<Vec<_>>();
        names.push("manifest.json".into());
        names.sort();
        manifest["files"] = serde_json::json!(names);
        manifest["sheet_prompts"] = serde_json::json!(sheet_prompts);
        manifest["required_dimension_ids"] = serde_json::json!(required_dimension_ids);
    }
    files.insert("manifest.json".into(), bounded_json(&manifest, remaining)?);
    let compared = scene
        .comparison_results
        .iter()
        .filter(|r| r.status == "compared")
        .count();
    let comparison_attribution = if scene.comparison_results.is_empty() {
        "not_requested"
    } else if compared == scene.comparison_results.len() {
        "engine_recorded_graph"
    } else if compared == 0 {
        "not_compared"
    } else {
        "partially_compared"
    };
    Ok(Package {
        files,
        comparison_attribution,
        version,
    })
}
fn flow(b: &DrawingBrief, aliases: &mut [Aliases]) -> Result<Vec<FlowLink>> {
    let mut links = vec![];
    for f in &b.transitions {
        text(&f.description)?;
        let before = view(b, &f.before)?;
        let after = f.after.as_ref().map(|id| view(b, id)).transpose()?;
        let mut steps = vec![];
        let mut confirmed = false;
        if let Some(t) = &f.transition {
            let SourceInput::Observed { snapshot: a, .. } = &before.source else {
                return Err(E::InvalidSource);
            };
            require(
                validation::contexts_compatible(&t.context, &a.context),
                E::IncompatibleViews,
            )?;
            for action in &f.actions {
                validation::validate_action(a, action).map_err(|_| E::InvalidSource)?;
            }
            require(
                t.steps.iter().all(|step| {
                    f.actions
                        .iter()
                        .filter(|action| action.id == step.action_id)
                        .count()
                        == 1
                }),
                E::InvalidReference,
            )?;
            let z = match after.map(|v| &v.source) {
                Some(SourceInput::Observed { snapshot, .. }) => Some((**snapshot).clone()),
                None => None,
                _ => return Err(E::InvalidSource),
            };
            Document {
                schema_version: SchemaVersion::CURRENT,
                artifact: Artifact::TransitionContext(Box::new(TransitionCase {
                    transition: t.clone(),
                    before: (**a).clone(),
                    after: z,
                })),
            }
            .validate()
            .map_err(|_| E::InvalidSource)?;
            let before_index = b
                .views
                .iter()
                .position(|v| v.id == f.before)
                .ok_or(E::InvalidReference)?;
            let index = b
                .views
                .iter()
                .position(|v| Some(&v.id) == f.after.as_ref())
                .unwrap_or(0);
            confirmed = !t.steps.is_empty()
                && after.is_some()
                && t.steps.iter().all(|s| {
                    s.delivery == DeliveryStatus::Confirmed
                        && s.outcome == Outcome::Succeeded
                        && s.verification_observation.is_some()
                });
            steps = t
                .steps
                .iter()
                .map(|s| {
                    // Unique matching action was checked above; do not serialize input values.
                    let action = f
                        .actions
                        .iter()
                        .find(|action| action.id == s.action_id)
                        .ok_or(E::InvalidReference)?;
                    Ok(FlowStep {
                        action_ref: aliases[before_index].id(&action.id),
                        target: aliases[before_index].key(&action.backend_ref.key),
                        intent: intent_name(&action.intent).into(),
                        modality: action.modality,
                        delivery: s.delivery,
                        outcome: s.outcome,
                        verification_observation: s
                            .verification_observation
                            .as_ref()
                            .map(|id| aliases[index].id(id)),
                    })
                })
                .collect::<Result<Vec<_>>>()?;
        }
        links.push(FlowLink {
            before: f.before.clone(),
            after: f.after.clone(),
            description: f.description.clone(),
            status: if confirmed {
                "confirmed_transition"
            } else {
                "unverified_or_unknown_destination"
            },
            steps,
        });
    }
    Ok(links)
}
fn intent_name(intent: &Intent) -> &'static str {
    match intent {
        Intent::Focus {} => "focus",
        Intent::Activate {} => "activate",
        Intent::SetChecked { value: true } => "set_checked(true)",
        Intent::SetChecked { value: false } => "set_checked(false)",
        Intent::Fill { .. } => "fill (content omitted)",
        Intent::Type { .. } => "type (content omitted)",
        Intent::FillSecret { .. } => "fill_secret (reference omitted)",
        Intent::SelectOption { .. } => "select_option (value omitted)",
        Intent::Scroll {} => "scroll",
        Intent::Press { .. } => "press (key omitted)",
        Intent::Submit {} => "submit",
        Intent::Dismiss {} => "dismiss",
    }
}
fn sheets(b: &DrawingBrief, s: &Scene, l: ExportLimits) -> Result<Vec<Sheet>> {
    let mut out = vec![];
    let mut dn = 0;
    for (i, v) in s.views.iter().enumerate() {
        let general = format!("G{:02}", i + 1);
        let units = units(v);
        out.push(Sheet { id:general.clone(),view:v.id.clone(),title:v.title.clone(),kind:"general",parent_view:None,components:v.components.iter().map(|c|c.id.clone()).collect(),placement:"central full scope; external dimension margins; legend; title block bottom right",state:v.state.clone(),units:units.clone() });
        let mut groups = vec![];
        for d in b.details.iter().filter(|d| d.view == v.id) {
            text(&d.title)?;
            require(
                !d.components.is_empty()
                    && d.components
                        .iter()
                        .all(|id| v.components.iter().any(|c| &c.id == id)),
                E::InvalidReference,
            )?;
            groups.push((d.title.clone(), d.components.clone()));
        }
        if v.components.len() > l.components_per_detail {
            for chunk in v.components.chunks(l.components_per_detail) {
                groups.push((
                    "Scope detail; preserve every listed object".into(),
                    chunk.iter().map(|c| c.id.clone()).collect(),
                ));
            }
        }
        for (title, components) in groups {
            dn += 1;
            out.push(Sheet {
                id: format!("D{dn:02}"),
                view: v.id.clone(),
                title,
                kind: "detail",
                parent_view: Some(general.clone()),
                components,
                placement: "separate enlarged view; reference to general view is not a user action",
                state: v.state.clone(),
                units: units.clone(),
            });
        }
    }
    require(
        b.details
            .iter()
            .all(|d| s.views.iter().any(|v| v.id == d.view)),
        E::InvalidReference,
    )?;
    if b.purpose == Purpose::Flow {
        out.push(Sheet {
            id: "F01".into(),
            view: "flow".into(),
            title: "States and evidenced transitions".into(),
            kind: "flow",
            parent_view: None,
            components: vec![],
            placement: "separate state-map; unknown destinations marked ?",
            state: "separate named states; never a combined observation".into(),
            units: vec![],
        });
    }
    Ok(out)
}
fn units(v: &SceneView) -> Vec<Unit> {
    let mut out = vec![];
    for c in &v.components {
        for g in c
            .geometry
            .iter()
            .chain(c.properties.iter().filter_map(|p| match p.known() {
                Some(Value::Geometry(g)) => Some(g.as_ref()),
                _ => None,
            }))
        {
            if !out.contains(&g.coordinate_space.units) {
                out.push(g.coordinate_space.units);
            }
        }
    }
    out
}
fn json(value: &impl Serialize, limit: usize) -> Result<String> {
    String::from_utf8(bounded_json(value, limit)?).map_err(|_| E::InvalidInput)
}
// These labels are presentation only. No rounded number is written back to geometry.
pub(crate) fn display_number(value: f64) -> String {
    if value == 0.0 {
        return "0".into();
    }
    if value.abs() < 0.5 {
        return if value < 0.0 { "−<0.5" } else { "<0.5" }.into();
    }
    if value == value.round() {
        format!("{value:.0}")
    } else {
        format!("≈{:.0}", value.round())
    }
}
fn geometry(c: &SceneComponent) -> Option<&Geometry> {
    c.geometry.as_ref().or_else(|| {
        [
            FrameKind::LayoutBounds,
            FrameKind::AccessibilityBounds,
            FrameKind::HitRegion,
            FrameKind::VisibleRegion,
            FrameKind::PaintBounds,
        ]
        .iter()
        .find_map(|kind| {
            c.properties.iter().find_map(|p| match p.known() {
                Some(Value::Geometry(g)) if g.frame_kind == *kind => Some(g.as_ref()),
                _ => None,
            })
        })
    })
}
fn component_role(c: &SceneComponent) -> String {
    if let Some(role) = c.role {
        return format!("{role:?}");
    }
    if let Some(role) = c.properties.iter().find_map(|p| match p.known() {
        Some(Value::Role(role)) if p.field() == Field::Role => Some(role),
        _ => None,
    }) {
        return format!("{role:?}");
    }
    match &c.native_role {
        Some(Availability::Known {
            value: Value::Text(role),
        }) => role.clone(),
        _ => "unknown role".into(),
    }
}
fn public_label(c: &SceneComponent) -> &str {
    c.label.as_deref().unwrap_or_else(|| {
        [Field::VisibleText, Field::AccessibilityName]
            .iter()
            .find_map(|field| {
                c.properties.iter().find_map(|p| match p.known() {
                    Some(Value::Text(text)) if p.field() == *field && !text.is_empty() => {
                        Some(text.as_str())
                    }
                    _ => None,
                })
            })
            .unwrap_or("")
    })
}
fn control(c: &SceneComponent) -> bool {
    matches!(
        component_role(c).to_ascii_lowercase().as_str(),
        "button"
            | "select"
            | "input"
            | "textbox"
            | "searchbox"
            | "combobox"
            | "checkbox"
            | "radio"
            | "switch"
            | "slider"
            | "option"
    )
}
pub(crate) fn unit_name(unit: Unit) -> &'static str {
    match unit {
        Unit::CssPx => "CSS px",
        Unit::Px => "px",
        Unit::Pt => "pt",
        Unit::Dp => "dp",
    }
}
pub(crate) fn component_name(v: &SceneView, index: usize) -> String {
    let c = &v.components[index];
    let role = component_role(c).to_ascii_lowercase();
    let human_role = match role.to_ascii_lowercase().as_str() {
        "div" | "group" => "container",
        "form" => "form outline",
        "footer" => "footer",
        "label" | "text" => "label layout",
        "input" | "textbox" | "searchbox" => "input",
        "select" | "combobox" => "select",
        _ => &role,
    };
    let accessible = c.properties.iter().find_map(|p| match p.known() {
        Some(Value::Text(text)) if p.field() == Field::AccessibilityName && !text.is_empty() => {
            Some(text.as_str())
        }
        _ => None,
    });
    // Visible control text can be an icon-font token; its payload is retained,
    // but a semantic measurement caption needs an explicit name or label owner.
    let visible = if control(c) && c.label.is_none() {
        ""
    } else {
        public_label(c)
    };
    let aggregate = child_indices(v, index).iter().any(|&i| {
        let child_label = public_label(&v.components[i]);
        !child_label.is_empty() && visible.contains(child_label)
    });
    let label =
        accessible.or_else(|| (!aggregate && !visible.contains(['\n', '\r'])).then_some(visible));
    if let Some(label) = label.filter(|label| !label.is_empty() && label.len() <= 80) {
        return format!("{human_role} {:?}", label.replace('\n', " / "));
    }
    if human_role == "select" {
        // A reviewed select's rendered text is source context, not a raw Value
        // or an inferred product name. A wrapping LABEL is an explicit owner.
        let displayed = visible_text(c);
        let owner = v.components.iter().find(|p| p.children.contains(&c.id)
            && matches!(&p.native_role, Some(Availability::Known { value: Value::Text(role) }) if role.eq_ignore_ascii_case("label")));
        let label = owner.map(public_label).unwrap_or("");
        if !displayed.is_empty() && displayed.len() <= 80 && !displayed.contains(['\n', '\r']) {
            return if label.is_empty() {
                format!("select displaying {displayed:?}")
            } else {
                format!("select under {label:?}, displaying {displayed:?}")
            };
        }
        if !label.is_empty() {
            return format!("select under {label:?}");
        }
    }
    if control(c) {
        if c.properties
            .iter()
            .any(|p| p.field() == Field::Expanded && matches!(p.known(), Some(Value::Flag(true))))
        {
            return format!("expanded {human_role}");
        }
        if let Some(parent) = v.components.iter().position(|p| p.children.contains(&c.id)) {
            let siblings = child_indices(v, parent)
                .into_iter()
                .filter(|&i| control(&v.components[i]))
                .collect::<Vec<_>>();
            if let Some(position) = siblings.iter().position(|&i| i == index) {
                return format!("{human_role}, item {} in its group", position + 1);
            }
        }
    }
    if human_role == "container"
        && !c.children.is_empty()
        && child_indices(v, index)
            .iter()
            .all(|&i| control(&v.components[i]))
    {
        return "control group".into();
    }
    if human_role == "container"
        && !c.children.is_empty()
        && !v.components.iter().any(|p| p.children.contains(&c.id))
    {
        return "outer source container".into();
    }
    human_role.into()
}
fn geometry_context(v: &SceneView, g: &Geometry) -> usize {
    v.components
        .iter()
        .position(|c| {
            geometry(c).is_some_and(|h| {
                h.frame_kind == g.frame_kind
                    && h.coordinate_space == g.coordinate_space
                    && h.transform == g.transform
            })
        })
        .map_or(0, |i| i + 1)
}
fn visible_text(c: &SceneComponent) -> &str {
    c.label.as_deref().unwrap_or_else(|| {
        c.properties
            .iter()
            .find_map(|p| match p.known() {
                Some(Value::Text(text)) if p.field() == Field::VisibleText => Some(text.as_str()),
                _ => None,
            })
            .unwrap_or("")
    })
}
fn object_text(
    v: &SceneView,
    index: usize,
    partial: &[Vec<Edge>],
    assignment: Assignment,
    limit: usize,
) -> Result<String> {
    let c = &v.components[index];
    let aggregate = !visible_text(c).is_empty()
        && child_indices(v, index).iter().any(|&i| {
            !visible_text(&v.components[i]).is_empty()
                && visible_text(c).contains(visible_text(&v.components[i]))
        });
    let mut out = if assignment == Assignment::SourceDetail {
        let position =
            if let Some(parent) = v.components.iter().position(|p| p.children.contains(&c.id)) {
                let siblings = child_indices(v, parent);
                let position = siblings
                    .iter()
                    .position(|&i| i == index)
                    .ok_or(E::InvalidInput)?;
                format!(", source child {} of {}", position + 1, siblings.len())
            } else {
                String::new()
            };
        format!("- Source member {}{position}", component_name(v, index))
    } else {
        format!(
            "Object {}: {} [#{}]",
            component_name(v, index),
            component_role(c),
            index + 1
        )
    };
    if assignment == Assignment::SourceDetail
        && partial.get(index).is_some_and(|edges| !edges.is_empty())
    {
        out.push_str(&format!(", unprinted clipping reference [#{}]", index + 1));
    }
    if !aggregate && !visible_text(c).is_empty() {
        out.push_str(&format!(
            "; visible-text payload{} {}",
            if partial.get(index).is_some_and(|edges| !edges.is_empty()) {
                " (draw only its proved clipped fragment)"
            } else {
                ""
            },
            bounded_compact_json(&visible_text(c), limit)?
        ));
    }
    for field in [
        Field::AccessibilityName,
        Field::Placeholder,
        Field::InputKind,
    ] {
        if let Some(Value::Text(value)) = c
            .properties
            .iter()
            .find(|p| p.field() == field)
            .and_then(Property::known)
            && !value.is_empty()
            // A short accessible name is already quoted in the object caption.
            && !(field == Field::AccessibilityName
                && component_name(v, index).contains(&format!("{value:?}")))
        {
            out.push_str(&format!(
                "; {:?}={}",
                field,
                bounded_compact_json(value, limit)?
            ));
        }
    }
    for p in &c.properties {
        if let Some(Value::Flag(value)) = p.known()
            && matches!(
                p.field(),
                Field::Checked
                    | Field::Selected
                    | Field::Expanded
                    | Field::Enabled
                    | Field::Focused
            )
        {
            out.push_str(&format!("; {:?}={value}", p.field()));
            if p.field() == Field::Focused && *value {
                out.push_str("; print a separate outside state tag meaning \"Focused\" in the document language in every view containing this object");
            }
            if p.field() == Field::Checked && !input_control(c) {
                out.push_str(
                    "; indicator is a hollow WHITE outline with a BLUE center in this state",
                );
            }
            if p.field() == Field::Selected && *value {
                out.push_str("; double WHITE outline, BLUE interior; preserve caller distinction between active and applied");
            }
        }
    }
    if let Some(state) = &c.state_and_actions {
        out.push_str(&format!(
            "; reported state/actions (not executed): {}",
            bounded_compact_json(state, limit)?
        ));
    }
    if let Some(g) = geometry(c) {
        // AX-only source fragments have no assigned dimensions or proved paint
        // mapping. Their tree membership identifies the fragment; coordinates
        // remain exact in the machine records, not an alternate drawing plan.
        if assignment != Assignment::SourceDetail || g.frame_kind != FrameKind::AccessibilityBounds
        {
            out.push_str(&format!(
                "; {} context {}",
                if g.frame_kind == FrameKind::AccessibilityBounds {
                    "measurement"
                } else {
                    "placement"
                },
                geometry_context(v, g)
            ));
            match &g.shape {
                Shape::Rect(r) => out.push_str(&format!(
                    "; box[x {}, y {}, w {}, h {}]",
                    display_number(r.x),
                    display_number(r.y),
                    display_number(r.width),
                    display_number(r.height)
                )),
                _ => out.push_str("; reported nonrectangular geometry; do not invent a rectangle"),
            }
        }
        for p in &c.properties {
            if let Some(Value::Geometry(visible)) = p.known()
                && p.field() == Field::VisibleRegion
                && visible.shape != g.shape
            {
                out.push_str(&format!(
                    "; {} reported visible-region {} (Space {} {:?}, {}, {:?})",
                    if partial.get(index).is_some_and(|edges| !edges.is_empty()) {
                        "CLIPPED LAYOUT: draw only"
                    } else {
                        "DISTINCT FRAME (clipping not established):"
                    },
                    if partial.get(index).is_some_and(|edges| !edges.is_empty()) {
                        format!("from COMPONENT CLIP #{}", index + 1)
                    } else if let Shape::Rect(r) = &visible.shape {
                        format!(
                            "rectangle[x {}, y {}, w {}, h {}]",
                            display_number(r.x),
                            display_number(r.y),
                            display_number(r.width),
                            display_number(r.height)
                        )
                    } else {
                        bounded_compact_json(&visible.shape, limit)?
                    },
                    visible.coordinate_space.id.0,
                    visible.coordinate_space.kind,
                    unit_name(visible.coordinate_space.units),
                    visible.coordinate_space.origin
                ));
            }
        }
    } else {
        out.push_str("; geometry unavailable");
    }
    out.push('\n');
    require(out.len() <= limit, E::OutputLimit)?;
    Ok(out)
}
fn ax_fragment_order(previous: &SceneComponent, current: &SceneComponent) -> Option<String> {
    let a = geometry(previous)?;
    let b = geometry(current)?;
    if a.frame_kind != FrameKind::AccessibilityBounds
        || b.frame_kind != a.frame_kind
        || a.coordinate_space != b.coordinate_space
        || a.coordinate_space.origin != Origin::TopLeft
        || a.transform != b.transform
        || previous.surface != current.surface
    {
        return None;
    }
    let observation = |c: &SceneComponent, g: &Geometry| {
        c.properties.iter().find_map(|p| match p {
            Property::Requested {
                evidence,
                state:
                    Availability::Known {
                        value: Value::Geometry(value),
                    },
                ..
            } if value.as_ref() == g => Some(evidence.observation_id.clone()),
            _ => None,
        })
    };
    if observation(previous, a)? != observation(current, b)? {
        return None;
    }
    let (Shape::Rect(a_rect), Shape::Rect(b_rect)) = (&a.shape, &b.shape) else {
        return None;
    };
    if a_rect.width <= 0.0 || a_rect.height <= 0.0 || b_rect.width <= 0.0 || b_rect.height <= 0.0 {
        return None;
    }
    let (ar, ab, br, bb) = (
        a_rect.x + a_rect.width,
        a_rect.y + a_rect.height,
        b_rect.x + b_rect.width,
        b_rect.y + b_rect.height,
    );
    if ![ar, ab, br, bb].iter().all(|n| n.is_finite()) {
        return None;
    }
    let horizontal = if b_rect.x >= ar {
        "right"
    } else if br <= a_rect.x {
        "left"
    } else {
        "horizontal spans overlap"
    };
    let vertical = if b_rect.y >= ab {
        "below"
    } else if bb <= a_rect.y {
        "above"
    } else {
        "vertical spans overlap"
    };
    if horizontal == "horizontal spans overlap" && vertical == "vertical spans overlap" {
        return None;
    }
    let transform = match b.transform {
        TransformState::Unknown { .. } => "unknown",
        TransformState::LocalOnly {} => "local only",
        TransformState::Known { .. } => "supplied",
    };
    Some(format!(
        "  Unprinted AX-frame order relative to preceding source member: {vertical}; {horizontal}. Same Space {} ({:?}, {}, {:?}), transform {transform}; same Surface and Observation.\n",
        b.coordinate_space.id.0,
        b.coordinate_space.kind,
        unit_name(b.coordinate_space.units),
        b.coordinate_space.origin
    ))
}

fn drawing_inventory(
    v: &SceneView,
    partial: &[Vec<Edge>],
    anchors: &[usize],
    selected: Option<&[String]>,
    assignment: Assignment,
    limit: usize,
) -> Result<String> {
    let mut out = String::new();
    append(
        &mut out,
        if assignment == Assignment::SourceDetail {
            "SOURCE FRAGMENT MEMBERS. Each listed member is a source record assigned to this sheet, not proof of visible artwork. Depict only portions permitted by the declared scope, original reference and supplied visibility/cut constraints; keep unknown visibility unknown. Source hierarchy provides structural orientation only, not proved visual correspondence. Ancestors and child positions are not extra artwork or captions. AX frames do not establish painted geometry; their exact measurements remain in the machine package.\n"
        } else {
            "SOURCE RECORDS. #numbers are unprinted local references. Placement context is only for the reported layout/visible frame. AX measurement coordinates are NEVER artwork placement: use the original reference for painted composition and keep AX rectangles in separate EMPTY sketches. Source/reference disagreement stays explicit; no guessed transform. Semantic names are outside measurement captions, never new visible UI.\n"
        },
        limit,
    )?;
    let mut contexts = Vec::new();
    for g in v
        .components
        .iter()
        .filter(|c| selected.is_none_or(|s| s.contains(&c.id)))
        .filter_map(geometry)
        .filter(|g| {
            assignment != Assignment::SourceDetail || g.frame_kind != FrameKind::AccessibilityBounds
        })
    {
        let context = format!(
            "{:?}; {} {:?}, {}, {:?}; transform {}",
            g.frame_kind,
            g.coordinate_space.id.0,
            g.coordinate_space.kind,
            unit_name(g.coordinate_space.units),
            g.coordinate_space.origin,
            match &g.transform {
                TransformState::LocalOnly {} => "local only".into(),
                TransformState::Unknown { .. } => "unknown".into(),
                TransformState::Known { transform: t } => format!(
                    "known {} -> {}, affine {:?}",
                    t.from.id.0, t.to.id.0, t.affine
                ),
            }
        );
        if !contexts.contains(&context) {
            append(
                &mut out,
                &format!(
                    "{} context {}: {context}. No guessed conversions.\n",
                    if g.frame_kind == FrameKind::AccessibilityBounds {
                        "Measurement"
                    } else {
                        "Placement"
                    },
                    geometry_context(v, g)
                ),
                limit,
            )?;
            contexts.push(context);
        }
    }
    let mut previous_fragment_item = None;
    let mut previous_fragment_parent = None;
    let mut previous_fragment_path: Vec<(usize, String)> = Vec::new();
    for (i, c) in v.components.iter().enumerate() {
        if selected.is_some_and(|s| !s.contains(&c.id)) {
            continue;
        }
        if let Some(edges) = partial.get(i).filter(|edges| !edges.is_empty())
            && let Some(Value::Geometry(visible)) = c
                .properties
                .iter()
                .find(|p| p.field() == Field::VisibleRegion)
                .and_then(Property::known)
            && let Shape::Rect(r) = &visible.shape
        {
            append(
                &mut out,
                &format!(
                    "COMPONENT CLIP #{} (construction only), {}: known clipped edges {:?}; schematic visible rectangle x {}, y {}, width {}, height {} {} in source Space {} ({:?}, {:?}). Apply this cut ONLY to this component, not a global crop of the generated sheet or unrelated components. Keep partial glyphs only at these proved edges; do not reflow or complete this component beyond them. Full-layout dimensions refer to its separate full frame, never to this visible intersection.\n",
                    i + 1,
                    component_name(v, i),
                    edges,
                    display_number(r.x),
                    display_number(r.y),
                    display_number(r.width),
                    display_number(r.height),
                    unit_name(visible.coordinate_space.units),
                    visible.coordinate_space.id.0,
                    visible.coordinate_space.kind,
                    visible.coordinate_space.origin
                ),
                limit,
            )?;
        }
        if assignment == Assignment::SourceDetail
            && !control(c)
            && visible_text(c).is_empty()
            && selected.is_some_and(|s| c.children.iter().any(|id| !s.contains(id)))
        {
            append(
                &mut out,
                &format!(
                    "BOUNDARY CONTEXT: {}. Only listed descendant records belong to this sheet, subject to its artwork scope; its unlisted children and complete parent region are not artwork on this sheet. Full geometry stays in machine evidence.\n",
                    component_name(v, i)
                ),
                limit,
            )?;
            continue;
        }
        let text = !visible_text(c).is_empty()
            && !child_indices(v, i).iter().any(|&j| {
                !visible_text(&v.components[j]).is_empty()
                    && visible_text(c).contains(visible_text(&v.components[j]))
            });
        let visual = matches!(
            component_role(c).to_ascii_lowercase().as_str(),
            "img" | "image" | "svg" | "canvas" | "media" | "custom"
        );
        let semantic_text = matches!(
            component_role(c).to_ascii_lowercase().as_str(),
            "text" | "heading" | "link"
        );
        if selected.is_some()
            || control(c)
            || text
            || semantic_text
            || visual
            || anchors.contains(&i)
        {
            if assignment == Assignment::SourceDetail {
                let parent = v.components.iter().position(|p| p.children.contains(&c.id));
                if parent != previous_fragment_parent {
                    let mut location = Vec::new();
                    let mut current = parent;
                    while let Some(index) = current {
                        let name = component_name(v, index);
                        let ancestor = v
                            .components
                            .iter()
                            .position(|p| p.children.contains(&v.components[index].id));
                        if let Some(ancestor) = ancestor {
                            let siblings = child_indices(v, ancestor);
                            let position = siblings
                                .iter()
                                .position(|&i| i == index)
                                .ok_or(E::InvalidInput)?;
                            location.push((
                                index,
                                format!("{name} (child {} of {})", position + 1, siblings.len()),
                            ));
                        } else {
                            location.push((index, name));
                        }
                        current = ancestor;
                    }
                    location.reverse();
                    // Reuse structure by source identity, never by equal labels.
                    let shared = location
                        .iter()
                        .zip(&previous_fragment_path)
                        .take_while(|(a, b)| a.0 == b.0)
                        .count();
                    let suffix = location[shared..]
                        .iter()
                        .map(|(_, name)| name.as_str())
                        .collect::<Vec<_>>()
                        .join(" → ");
                    let levels = if shared == 1 { "level" } else { "levels" };
                    let context = if shared == 0 {
                        format!(
                            "Source hierarchy (orientation only): {}.\n",
                            if suffix.is_empty() {
                                "top level"
                            } else {
                                &suffix
                            }
                        )
                    } else if suffix.is_empty() {
                        format!(
                            "Source hierarchy: return to first {shared} {levels} of previous context.\n"
                        )
                    } else if shared == previous_fragment_path.len() {
                        format!("Source hierarchy: within previous context → {suffix}.\n")
                    } else {
                        format!(
                            "Source hierarchy: keep first {shared} {levels} of previous context → {suffix}.\n"
                        )
                    };
                    append(&mut out, &context, limit)?;
                    previous_fragment_path = location;
                    previous_fragment_parent = parent;
                }
            }
            append(
                &mut out,
                &object_text(v, i, partial, assignment, limit)?,
                limit,
            )?;
            if assignment == Assignment::SourceDetail {
                if let Some(previous) = previous_fragment_item
                    && let Some(hint) = ax_fragment_order(&v.components[previous], c)
                {
                    append(&mut out, &hint, limit)?;
                }
                previous_fragment_item = Some(i);
            }
        }
    }
    Ok(out)
}
fn measurement_inventory(
    v: &SceneView,
    targets: &std::collections::BTreeSet<usize>,
    limit: usize,
) -> Result<String> {
    let mut out = String::from(
        "ASSIGNED MEASUREMENT TARGETS. Context is attribution only, not artwork or additional labels. Unassigned source facts remain in scene/dimensions. No absent property means empty or enabled.\n",
    );
    let mut contexts = Vec::new();
    for &i in targets {
        let c = &v.components[i];
        if let Some(g) = geometry(c) {
            let context = geometry_context(v, g);
            if !contexts.contains(&context) {
                append(
                    &mut out,
                    &format!(
                        "Measurement context {context}: {:?}; Space {} {:?}, {}, {:?}; transform {}. No inferred conversion.\n",
                        g.frame_kind,
                        g.coordinate_space.id.0,
                        g.coordinate_space.kind,
                        unit_name(g.coordinate_space.units),
                        g.coordinate_space.origin,
                        match g.transform {
                            TransformState::LocalOnly {} => "local only",
                            TransformState::Unknown { .. } => "unknown",
                            TransformState::Known { .. } => "supplied mapping",
                        }
                    ),
                    limit,
                )?;
                contexts.push(context);
            }
            append(
                &mut out,
                &format!(
                    "TARGET {}: {:?}, context {context}; this frame only.\n",
                    component_name(v, i),
                    g.frame_kind
                ),
                limit,
            )?;
        }
        if let Some(parent) = v.components.iter().position(|p| p.children.contains(&c.id)) {
            append(
                &mut out,
                &format!(
                    "Boundary: {} excludes its reported parent {} and {} sibling source objects; they are context only, never contents of this sketch.\n",
                    component_name(v, i),
                    component_name(v, parent),
                    child_indices(v, parent).len().saturating_sub(1)
                ),
                limit,
            )?;
        }
        for p in &c.properties {
            if let Some(Value::Flag(value)) = p.known()
                && matches!(
                    p.field(),
                    Field::Checked
                        | Field::Selected
                        | Field::Expanded
                        | Field::Enabled
                        | Field::Focused
                )
            {
                append(
                    &mut out,
                    &format!(
                        "Recorded {:?}={value} for {}; caption outside only.\n",
                        p.field(),
                        component_name(v, i)
                    ),
                    limit,
                )?;
            }
        }
        if let Some(state) = &c.state_and_actions {
            append(
                &mut out,
                &format!(
                    "Reported state/actions for {}: {}; not executed.\n",
                    component_name(v, i),
                    bounded_compact_json(state, limit)?
                ),
                limit,
            )?;
        }
    }
    Ok(out)
}
fn local_measures(
    v: &SceneView,
    measures: &[(&Dimension, usize, usize)],
    clipped: &[Vec<Edge>],
    limit: usize,
) -> Result<String> {
    let mut out = String::new();
    let mut done = Vec::new();
    for &(d, a, b) in measures {
        if done.contains(&d.id) {
            continue;
        }
        let value = d.value.ok_or(E::InvalidGeometry)?;
        if a == b
            && matches!(
                (d.anchors[0].edge, d.anchors[1].edge),
                (Edge::Left, Edge::Right) | (Edge::Top, Edge::Bottom)
            )
        {
            let width = measures.iter().find(|(m, x, y)| {
                *x == a
                    && *y == a
                    && m.anchors[0].frame_kind == d.anchors[0].frame_kind
                    && m.anchors[0].space == d.anchors[0].space
                    && m.anchors[0].edge == Edge::Left
                    && m.anchors[1].edge == Edge::Right
            });
            let height = measures.iter().find(|(m, x, y)| {
                *x == a
                    && *y == a
                    && m.anchors[0].frame_kind == d.anchors[0].frame_kind
                    && m.anchors[0].space == d.anchors[0].space
                    && m.anchors[0].edge == Edge::Top
                    && m.anchors[1].edge == Edge::Bottom
            });
            if let (Some((w, _, _)), Some((h, _, _))) = (width, height) {
                if w.anchors[0].frame_kind != FrameKind::AccessibilityBounds {
                    append(
                        &mut out,
                        &format!("FRAME OWNER (unprinted): source record #{}.\n", a + 1),
                        limit,
                    )?;
                }
                if w.anchors[0].frame_kind == FrameKind::LayoutBounds
                    && clipped.get(a).is_some_and(|edges| !edges.is_empty())
                {
                    append(
                        &mut out,
                        &format!(
                            "SEPARATE FULL-LAYOUT SKETCH — {} (LayoutBounds): EMPTY measurement rectangle, no UI contents. WIDTH {} {} ABOVE; HEIGHT {} {} LEFT. Both dimensions end on this full-layout frame's opposite edges, never on the visible overview. Caption outside: full layout frame.\n",
                            component_name(v, a),
                            display_number(w.value.ok_or(E::InvalidGeometry)?),
                            unit_name(w.units),
                            display_number(h.value.ok_or(E::InvalidGeometry)?),
                            unit_name(h.units)
                        ),
                        limit,
                    )?;
                    if let Some(Property::Requested {
                        state:
                            Availability::Known {
                                value: Value::Geometry(visible),
                            },
                        evidence,
                        ..
                    }) = v.components[a]
                        .properties
                        .iter()
                        .find(|p| p.field() == Field::VisibleRegion)
                    {
                        append(
                            &mut out,
                            &format!(
                                "OVERVIEW ASSIGNMENT — same component {}, different frame: {:?} VisibleRegion in Space {} ({:?}, {}, {:?}). Render only that sourced visible intersection with its proved component cut. It is not a standalone viewport, paint bounds or occlusion proof. Do not stretch the visible fragment to the full-layout height, complete offscreen content or attach the full-layout labels to it. Visible-frame data stays distinct even when its width equals the full-layout width.\n",
                                component_name(v, a),
                                evidence.provenance,
                                visible.coordinate_space.id.0,
                                visible.coordinate_space.kind,
                                unit_name(visible.coordinate_space.units),
                                visible.coordinate_space.origin
                            ),
                            limit,
                        )?;
                    }
                    done.extend([w.id.clone(), h.id.clone()]);
                    continue;
                }
                let container = !control(&v.components[a]) && !v.components[a].children.is_empty();
                let accessibility = w.anchors[0].frame_kind == FrameKind::AccessibilityBounds;
                append(
                    &mut out,
                    &format!(
                        "{} {}: WIDTH {} {} {}; HEIGHT {} {} on the LEFT. BOTH lines required, each attached to this frame's opposite borders.\n",
                        if accessibility {
                            "AX FRAME MEASUREMENT (not painted UI), EXACT OUTSIDE CAPTION"
                        } else {
                            "DIMENSIONED RECTANGLE"
                        },
                        if accessibility {
                            bounded_compact_json(&component_name(v, a), limit)?
                        } else {
                            component_name(v, a)
                        },
                        display_number(w.value.ok_or(E::InvalidGeometry)?),
                        unit_name(w.units),
                        if container && !accessibility {
                            "BELOW this measured source frame"
                        } else {
                            "ABOVE"
                        },
                        display_number(h.value.ok_or(E::InvalidGeometry)?),
                        unit_name(h.units)
                    ),
                    limit,
                )?;
                if container && !accessibility {
                    append(
                        &mut out,
                        &format!(
                            "PRINT BELOW-FRAME WIDTH LABEL exactly: {:?}. Both extents belong only to this measured source frame. Descendants may overflow it; keep their frames separate.\n",
                            format!(
                                "{} — measured frame width {} {}",
                                component_name(v, a),
                                display_number(w.value.ok_or(E::InvalidGeometry)?),
                                unit_name(w.units)
                            )
                        ),
                        limit,
                    )?;
                }
                done.extend([w.id.clone(), h.id.clone()]);
                continue;
            }
        }
        if a == b
            && matches!(
                (d.anchors[0].edge, d.anchors[1].edge),
                (Edge::Left, Edge::Right)
                    | (Edge::Right, Edge::Left)
                    | (Edge::Top, Edge::Bottom)
                    | (Edge::Bottom, Edge::Top)
            )
        {
            append(
                &mut out,
                &format!(
                    "{} {}: {:?}→{:?}, {} {}.\n",
                    if d.anchors[0].frame_kind == FrameKind::AccessibilityBounds {
                        "AX FRAME EXTENT (not painted UI), EXACT OUTSIDE CAPTION"
                    } else {
                        "RECTANGLE EXTENT"
                    },
                    if d.anchors[0].frame_kind == FrameKind::AccessibilityBounds {
                        bounded_compact_json(&component_name(v, a), limit)?
                    } else {
                        component_name(v, a)
                    },
                    d.anchors[0].edge,
                    d.anchors[1].edge,
                    display_number(value),
                    unit_name(d.units)
                ),
                limit,
            )?;
            done.push(d.id.clone());
            continue;
        }
        let axis = |edge| match edge {
            Edge::Left | Edge::Right | Edge::CenterX => "HORIZONTAL",
            Edge::Top | Edge::Bottom | Edge::CenterY => "VERTICAL",
        };
        let direction = if axis(d.anchors[0].edge) == axis(d.anchors[1].edge) {
            axis(d.anchors[0].edge)
        } else {
            "MIXED AXES"
        };
        let border_pair = matches!(
            (d.anchors[0].edge, d.anchors[1].edge),
            (Edge::Left | Edge::Right, Edge::Left | Edge::Right)
                | (Edge::Top | Edge::Bottom, Edge::Top | Edge::Bottom)
        );
        // Proposed distances are absolute, so their sign cannot establish ordering.
        let sketch = border_pair
            && d.source_kind == SourceKind::Observed
            && value > 0.0
            && d.anchors[0].space.origin == Origin::TopLeft
            && d.anchors[0].space == d.anchors[1].space;
        let start = format!(
            "{} {:?} {}",
            component_name(v, a),
            d.anchors[0].edge,
            if border_pair { "boundary" } else { "anchor" }
        );
        let end = format!(
            "{} {:?} {}",
            component_name(v, b),
            d.anchors[1].edge,
            if border_pair { "boundary" } else { "anchor" }
        );
        if !sketch {
            append(
                &mut out,
                &format!(
                    "NAMED-ANCHOR RELATION: {direction}; START {start} → END {end}; {} {}. Preserve signed ordering; no border-corner sketch or invented straight separation.\n",
                    display_number(value),
                    unit_name(d.units)
                ),
                limit,
            )?;
            done.push(d.id.clone());
            continue;
        }
        {
            let label = format!("{} {}", display_number(value), unit_name(d.units));
            let horizontal = matches!(d.anchors[0].edge, Edge::Left | Edge::Right);
            let topology = if horizontal {
                let arrow = format!("←─ {label} ─→");
                let right = arrow.chars().count() + 3;
                let mut corners = vec![' '; right + 2];
                for (column, edge) in [(2, d.anchors[0].edge), (right, d.anchors[1].edge)] {
                    if edge == Edge::Left {
                        corners[column] = '┌';
                        corners[column + 1] = '─';
                    } else {
                        corners[column] = '┐';
                        corners[column - 1] = '─';
                    }
                }
                format!("{}\n  │{arrow}│", corners.into_iter().collect::<String>())
            } else {
                let first = if d.anchors[0].edge == Edge::Top {
                    "┌────────"
                } else {
                    "└────────"
                };
                let second = if d.anchors[1].edge == Edge::Top {
                    "┌────────"
                } else {
                    "└────────"
                };
                format!("{first}\n    ↕ {label}\n{second}")
            };
            append(
                &mut out,
                &format!(
                    "EDGE FRAGMENT ({direction}; placement contexts {} / {}): print START {start:?}; print END {end:?}. START is left/upper; END is right/lower.\n{topology}\n",
                    geometry(&v.components[a]).map_or(0, |g| geometry_context(v, g)),
                    geometry(&v.components[b]).map_or(0, |g| geometry_context(v, g))
                ),
                limit,
            )?;
        }
        done.push(d.id.clone());
    }
    Ok(out)
}
fn child_indices(v: &SceneView, index: usize) -> Vec<usize> {
    v.components[index]
        .children
        .iter()
        .filter_map(|id| v.components.iter().position(|c| &c.id == id))
        .collect()
}
fn input_control(c: &SceneComponent) -> bool {
    matches!(
        component_role(c).to_ascii_lowercase().as_str(),
        "input" | "textbox" | "searchbox"
    ) || component_role(c).eq_ignore_ascii_case("combobox")
        && c.properties.iter().any(|p| p.field() == Field::InputKind
            && matches!(p.known(), Some(Value::Text(kind)) if matches!(kind.as_str(), "text" | "search")))
}
fn rect(c: &SceneComponent) -> Option<&Rect> {
    geometry(c).and_then(|g| match &g.shape {
        Shape::Rect(r) => Some(r),
        _ => None,
    })
}
fn same_frame(a: &SceneComponent, b: &SceneComponent) -> bool {
    a.surface == b.surface
        && match (geometry(a), geometry(b)) {
            (Some(g), Some(h)) => {
                g.coordinate_space == h.coordinate_space && g.frame_kind == h.frame_kind
            }
            _ => false,
        }
}
fn clipped_edges(source: &SourceInput) -> Vec<Vec<Edge>> {
    let SourceInput::Observed { snapshot, .. } = source else {
        return vec![];
    };
    snapshot
        .nodes
        .iter()
        .map(|node| {
            let property = |field| node.properties.iter().find(|p| p.field() == field);
            let (
                Some(Property::Requested {
                    state:
                        Availability::Known {
                            value: Value::Geometry(layout),
                        },
                    evidence: le,
                    ..
                }),
                Some(Property::Requested {
                    state:
                        Availability::Known {
                            value: Value::Geometry(visible),
                        },
                    evidence: ve,
                    ..
                }),
            ) = (
                property(Field::LayoutBounds),
                property(Field::VisibleRegion),
            )
            else {
                return vec![];
            };
            if layout.frame_kind != FrameKind::LayoutBounds
                || visible.frame_kind != FrameKind::VisibleRegion
                || layout.coordinate_space != visible.coordinate_space
                || le.observation_id != ve.observation_id
                || le.source_namespace != ve.source_namespace
                || !matches!(layout.shape, Shape::Rect(_))
                || !matches!(visible.shape, Shape::Rect(_))
            {
                return vec![];
            }
            let query = GeometryQuery {
                id: Id("export-visible-subset".into()),
                scope_id: snapshot.context.scope_id.clone(),
                targets: vec![node.key.clone()],
                operation: GeometryRelation::Inside,
                anchors: [FrameKind::VisibleRegion, FrameKind::LayoutBounds]
                    .map(|frame_kind| Anchor {
                        element: node.key.clone(),
                        frame_kind,
                        coordinate_space: layout.coordinate_space.clone(),
                        fraction: 0.0,
                        axis: Id("x".into()),
                    })
                    .to_vec(),
                quantity_kind: QuantityKind::Length,
                units: layout.coordinate_space.units,
                applies_when: ContextConditions {
                    platform: None,
                    input_mode: None,
                    text_scale: None,
                },
            };
            let context = EvaluationContext {
                space: &layout.coordinate_space,
                transforms: &[],
                conditions: None,
            };
            match engine::measure_query(snapshot, &query, &context) {
                Ok(MeasurementResult::Known { measurement }) => match measurement.details {
                    engine::Details::Insets {
                        left,
                        top,
                        right,
                        bottom,
                    } => {
                        if [left, top, right, bottom].iter().all(|x| *x >= 0.0) {
                            [
                                (Edge::Left, left),
                                (Edge::Top, top),
                                (Edge::Right, right),
                                (Edge::Bottom, bottom),
                            ]
                            .into_iter()
                            .filter_map(|(edge, inset)| (inset > 0.0).then_some(edge))
                            .collect()
                        } else {
                            vec![]
                        }
                    }
                    _ => vec![],
                },
                _ => vec![],
            }
        })
        .collect()
}
fn control_dimensions(
    source: &Snapshot,
    view: &SceneView,
    aliases: &mut Aliases,
    out: &mut Vec<Dimension>,
) -> Result<()> {
    let mut pairs = Vec::new();
    for dialog in 0..view.components.len() {
        if !component_role(&view.components[dialog]).eq_ignore_ascii_case("dialog") {
            continue;
        }
        for body in std::iter::once(dialog).chain(child_indices(view, dialog)) {
            let children = child_indices(view, body);
            let Some(input) = children
                .iter()
                .copied()
                .find(|&i| input_control(&view.components[i]))
            else {
                continue;
            };
            let Some(row) = children.iter().find_map(|&list| {
                let rows = child_indices(view, list);
                (component_role(&view.components[list]).eq_ignore_ascii_case("listbox")
                    && rows.len() >= 2
                    && rows.iter().all(|&i| {
                        component_role(&view.components[i]).eq_ignore_ascii_case("option")
                    }))
                .then(|| rows[0])
            }) else {
                continue;
            };
            pairs.extend([
                (dialog, input, Edge::Left, Edge::Left, "Measured left inset"),
                (
                    input,
                    row,
                    Edge::Bottom,
                    Edge::Top,
                    "Input to first control gap",
                ),
            ]);
        }
    }

    // Append only missing relations, preserving all earlier measurement IDs.
    let select = |i: usize| {
        matches!(
            component_role(&view.components[i])
                .to_ascii_lowercase()
                .as_str(),
            "select" | "combobox"
        ) && !input_control(&view.components[i])
    };
    for parent in 0..view.components.len() {
        let rows = child_indices(view, parent)
            .into_iter()
            .map(|i| {
                if select(i) {
                    return Some(i);
                }
                let children = child_indices(view, i);
                let mut controls = children.into_iter().filter(|&j| select(j));
                let first = controls.next();
                first.filter(|_| controls.next().is_none())
            })
            .collect::<Vec<_>>();
        for pair in rows.windows(2) {
            let [Some(a), Some(b)] = pair else {
                continue;
            };
            if same_frame(&view.components[*a], &view.components[*b])
                && matches!((rect(&view.components[*a]), rect(&view.components[*b])),
                    (Some(r), Some(q)) if r.x == q.x && r.width == q.width && q.y > r.y)
            {
                pairs.push((
                    *a,
                    *b,
                    Edge::Bottom,
                    Edge::Top,
                    "Repeated control clear gap",
                ));
            }
        }
    }
    for (a, b, first, last, label) in pairs {
        if out.iter().any(|d| {
            d.anchors[0].component == view.components[a].id
                && d.anchors[1].component == view.components[b].id
                && d.anchors[0].edge == first
                && d.anchors[1].edge == last
                && d.anchors
                    .iter()
                    .all(|x| x.frame_kind == FrameKind::LayoutBounds)
        }) {
            continue;
        }
        let layout = |i: usize| {
            source.nodes[i]
                .properties
                .iter()
                .find_map(|p| match p.known() {
                    Some(Value::Geometry(g))
                        if p.field() == Field::LayoutBounds
                            && g.frame_kind == FrameKind::LayoutBounds =>
                    {
                        Some(g.as_ref())
                    }
                    _ => None,
                })
        };
        let (Some(g), Some(h)) = (layout(a), layout(b)) else {
            continue;
        };
        if g.coordinate_space != h.coordinate_space
            || g.coordinate_space.origin != Origin::TopLeft
            || source.nodes[a].surface != source.nodes[b].surface
        {
            continue;
        }
        let horizontal = matches!(first, Edge::Left | Edge::Right);
        let query = GeometryQuery {
            id: Id("export-popup-spacing".into()),
            scope_id: source.context.scope_id.clone(),
            targets: vec![source.nodes[a].key.clone(), source.nodes[b].key.clone()],
            operation: GeometryRelation::Gap,
            anchors: [(a, first), (b, last)]
                .map(|(i, edge)| Anchor {
                    element: source.nodes[i].key.clone(),
                    frame_kind: FrameKind::LayoutBounds,
                    coordinate_space: g.coordinate_space.clone(),
                    fraction: if matches!(edge, Edge::Right | Edge::Bottom) {
                        1.0
                    } else {
                        0.0
                    },
                    axis: Id(if horizontal { "x" } else { "y" }.into()),
                })
                .to_vec(),
            quantity_kind: QuantityKind::Length,
            units: g.coordinate_space.units,
            applies_when: ContextConditions {
                platform: None,
                input_mode: None,
                text_scale: None,
            },
        };
        let context = EvaluationContext {
            space: &g.coordinate_space,
            transforms: &[],
            conditions: None,
        };
        let (value, unknown_reason, evidence) = match engine::measure_query(
            source, &query, &context,
        )
        .map_err(|_| E::InvalidGeometry)?
        {
            MeasurementResult::Known { measurement } => {
                let Value::Quantity { amount, .. } = measurement.value else {
                    return Err(E::InvalidGeometry);
                };
                (Some(amount), None, measurement.evidence)
            }
            MeasurementResult::Unknown { reason, evidence } => {
                (None, Some(reason.as_str().into()), evidence)
            }
        };
        out.push(Dimension {
            id: format!("M{:04}", out.len() + 1),
            label: label.into(),
            anchors: [(a, first), (b, last)].map(|(i, edge)| DimensionAnchor {
                component: view.components[i].id.clone(),
                frame_kind: FrameKind::LayoutBounds,
                space: aliases.space(&g.coordinate_space),
                edge,
            }),
            value,
            units: g.coordinate_space.units,
            source_kind: SourceKind::Observed,
            evidence: evidence.iter().map(|e| aliases.evidence(e)).collect(),
            requirement_ref: None,
            unknown_reason,
            check_tolerance: None,
        });
    }
    Ok(())
}

fn detail_caption(v: &SceneView, members: &[usize]) -> String {
    if let [index] = members {
        let c = &v.components[*index];
        let name = c.label.as_deref().or_else(|| {
            c.properties.iter().find_map(|p| match p.known() {
                Some(Value::Text(text))
                    if p.field() == Field::AccessibilityName && !text.is_empty() =>
                {
                    Some(text.as_str())
                }
                _ => None,
            })
        });
        if let Some(name) = name.filter(|s| !s.is_empty()) {
            return format!("{} — dimensions", name.replace('\n', " / "));
        }
        let labels = v
            .relations
            .iter()
            .filter(|r| {
                r.kind == RelationKind::LabelledBy && c.source_key.as_ref() == Some(&r.from)
            })
            .filter_map(|r| {
                v.components
                    .iter()
                    .find(|c| c.source_key.as_ref() == Some(&r.to))
            })
            .map(public_label)
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        if !labels.is_empty() {
            return format!("{} — dimensions", labels.join(" ").replace('\n', " / "));
        }
    }
    let parent = members.first().and_then(|first| {
        v.components
            .iter()
            .position(|p| p.children.contains(&v.components[*first].id))
    });
    if let Some(parent) = parent {
        let label = &v.components[parent];
        // A reported HTML LABEL wrapping these controls is an association.
        // A nearby text node under a common ancestor is not.
        if matches!(&label.native_role, Some(Availability::Known { value: Value::Text(role) }) if role.eq_ignore_ascii_case("label"))
            && members
                .iter()
                .all(|&i| label.children.contains(&v.components[i].id))
            && !public_label(label).is_empty()
        {
            return format!("{} — dimensions", public_label(label).replace('\n', " / "));
        }
    }
    "Control dimensions and spacing".into()
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Assignment {
    Overview,
    SourceDetail,
    Measurements,
}
struct Presentation {
    text: String,
    component_ids: Vec<String>,
    dimension_ids: Vec<String>,
    assignment: Assignment,
}
fn drawing_dimensions(
    v: &SceneView,
    dimensions: &ViewDimensions,
    partial: &[Vec<Edge>],
    sheet: Option<&Sheet>,
    sheets: &[Sheet],
    limit: usize,
) -> Result<Presentation> {
    let index = |id: &str| v.components.iter().position(|c| c.id == id);
    let large = |i: usize| {
        control(&v.components[i])
            && partial.get(i).is_none_or(|edges| edges.is_empty())
            && rect(&v.components[i]).is_some_and(|r| r.width >= 12.0 && r.height >= 12.0)
    };
    let frame_matches = |d: &Dimension| {
        d.anchors.iter().all(|a| {
            index(&a.component).is_some_and(|i| {
                geometry(&v.components[i])
                    .is_some_and(|g| g.frame_kind == a.frame_kind && g.coordinate_space == a.space)
            })
        })
    };
    let available: Vec<_> = dimensions
        .dimensions
        .iter()
        .filter(|d| d.value.is_some() && frame_matches(d))
        .collect();
    // The engine's existing parent/child inset results identify an overflowing wrapper.
    // No union, new quantity, source identity or invented container is calculated here.
    let mut overview: Vec<usize> = v
        .components
        .iter()
        .enumerate()
        .filter(|(i, c)| {
            !control(c)
                && !c.children.is_empty()
                && (component_role(c).eq_ignore_ascii_case("dialog")
                    || !v.components.iter().any(|p| p.children.contains(&c.id)))
                && !available.iter().any(|d| {
                    d.label.starts_with("Measured")
                        && d.value.is_some_and(|n| n < 0.0)
                        && d.anchors.iter().any(|a| a.component == c.id)
                        && d.anchors.iter().any(|a| c.children.contains(&a.component))
                })
                && rect(&v.components[*i]).is_some()
        })
        .map(|(i, _)| i)
        .collect();
    let mut groups: Vec<(String, Vec<usize>)> = Vec::new();
    let mut input_containers = Vec::new();
    for i in 0..v.components.len() {
        let children = child_indices(v, i);
        let input = children
            .iter()
            .copied()
            .find(|&j| large(j) && input_control(&v.components[j]));
        let direct_rows = children
            .iter()
            .copied()
            .filter(|&j| large(j) && !input_control(&v.components[j]))
            .collect::<Vec<_>>();
        let rows = children.iter().find_map(|&j| {
            let controls = child_indices(v, j)
                .into_iter()
                .filter(|&k| large(k) && !input_control(&v.components[k]))
                .collect::<Vec<_>>();
            (controls.len() >= 2).then_some(controls)
        });
        let rows = rows.or_else(|| (direct_rows.len() >= 2).then_some(direct_rows));
        if let (Some(input), Some(rows)) = (input, rows) {
            if !same_frame(&v.components[input], &v.components[rows[0]])
                || !same_frame(&v.components[input], &v.components[rows[1]])
            {
                continue;
            }
            let container = v
                .components
                .iter()
                .position(|c| {
                    component_role(c).eq_ignore_ascii_case("dialog")
                        && c.children.contains(&v.components[i].id)
                        && same_frame(c, &v.components[input])
                })
                .unwrap_or(i);
            overview.push(container);
            input_containers.push(container);
            groups.push((
                "Input and adjacent controls — partial spacing".into(),
                vec![input, rows[0], rows[1]],
            ));
        }
    }
    for i in 0..v.components.len() {
        let controls = child_indices(v, i)
            .into_iter()
            .filter(|&j| large(j))
            .collect::<Vec<_>>();
        if controls.len() < 2
            || controls
                .iter()
                .any(|j| groups.iter().any(|(_, members)| members.contains(j)))
        {
            continue;
        }
        let first = controls[0];
        let members = controls
            .into_iter()
            .filter(|&i| same_frame(&v.components[first], &v.components[i]))
            .take(4)
            .collect::<Vec<_>>();
        if members.len() < 2 {
            continue;
        }
        groups.push((detail_caption(v, &members), members));
    }
    for d in &available {
        if d.label != "Repeated control clear gap" || d.value.is_none_or(|x| x <= 0.0) {
            continue;
        }
        let (Some(a), Some(b)) = (
            index(&d.anchors[0].component),
            index(&d.anchors[1].component),
        ) else {
            continue;
        };
        if !matches!(
            component_role(&v.components[a])
                .to_ascii_lowercase()
                .as_str(),
            "select" | "combobox"
        ) || !large(a)
            || !large(b)
            || groups.iter().flat_map(|(_, m)| m).any(|&i| {
                component_role(&v.components[i]) == component_role(&v.components[a])
                    && same_frame(&v.components[i], &v.components[a])
                    && match (rect(&v.components[i]), rect(&v.components[a])) {
                        (Some(r), Some(q)) => r.width == q.width && r.height == q.height,
                        _ => false,
                    }
            })
            || groups.iter().any(|(_, m)| m.contains(&a) || m.contains(&b))
        {
            continue;
        }
        let members = vec![a, b];
        groups.push((detail_caption(v, &members), members));
    }
    let mut overview_controls = Vec::new();
    for i in 0..v.components.len() {
        if !large(i) || groups.iter().any(|(_, m)| m.contains(&i)) {
            continue;
        }
        let c = &v.components[i];
        let represented = groups
            .iter()
            .flat_map(|(_, m)| m)
            .chain(overview_controls.iter())
            .any(|&j| {
                let other = &v.components[j];
                component_role(other) == component_role(c)
                    && same_frame(other, c)
                    && v.components.iter().any(|parent| {
                        parent.children.contains(&c.id) && parent.children.contains(&other.id)
                    })
                    && match (geometry(other), geometry(c), rect(other), rect(c)) {
                        (Some(g), Some(h), Some(a), Some(b)) => {
                            g.coordinate_space == h.coordinate_space
                                && g.frame_kind == h.frame_kind
                                && a.width == b.width
                                && a.height == b.height
                        }
                        _ => false,
                    }
            });
        if represented {
            continue;
        }
        let attachment = available.iter().any(|d| {
            d.label == "Vertical edge gap"
                && d.anchors[0].component == c.id
                && index(&d.anchors[1].component).is_some_and(|j| input_containers.contains(&j))
        });
        if attachment {
            overview_controls.push(i);
        } else {
            groups.push((detail_caption(v, &[i]), vec![i]));
        }
    }
    let owner = |i: usize| groups.iter().position(|(_, m)| m.contains(&i));
    let mut lines: Vec<(Option<usize>, &Dimension, usize, usize)> = Vec::new();
    let mut members = groups.iter().map(|(_, m)| m.clone()).collect::<Vec<_>>();
    for (group, (_, controls)) in groups.iter().enumerate() {
        for &control in controls {
            if let Some(parent) = v
                .components
                .iter()
                .position(|p| p.children.contains(&v.components[control].id))
            {
                for label in child_indices(v, parent) {
                    if matches!(
                        component_role(&v.components[label])
                            .to_ascii_lowercase()
                            .as_str(),
                        "label" | "text"
                    ) && !members[group].contains(&label)
                    {
                        members[group].push(label);
                    }
                }
            }
        }
    }
    let mut insets: Vec<(&Dimension, usize, usize)> = Vec::new();
    let mut seen = Vec::new();
    for d in available {
        let (Some(a), Some(b), Some(value)) = (
            index(&d.anchors[0].component),
            index(&d.anchors[1].component),
            d.value,
        ) else {
            continue;
        };
        let (ca, cb) = (&v.components[a], &v.components[b]);
        let extent = a == b;
        let mut destination = owner(a).or_else(|| owner(b));
        let mut admitted = d.source_kind == SourceKind::Proposed;
        if extent {
            admitted |=
                overview.contains(&a) || overview_controls.contains(&a) || owner(a).is_some();
            if overview.contains(&a) || overview_controls.contains(&a) {
                destination = None;
            }
        } else if value > 0.0 {
            let same_group = owner(a).is_some() && owner(a) == owner(b);
            let horizontal = matches!(d.anchors[0].edge, Edge::Left | Edge::Right);
            let aligned = match (rect(ca), rect(cb)) {
                (Some(r), Some(q)) => {
                    if horizontal {
                        r.y == q.y
                    } else {
                        r.x == q.x
                    }
                }
                _ => false,
            };
            let label_to_control = matches!(
                component_role(ca).to_ascii_lowercase().as_str(),
                "label" | "text"
            ) && owner(b).is_some();
            if d.label.contains("gap") && ((same_group && aligned) || label_to_control) {
                admitted = true;
            }
            if d.label == "Vertical edge gap"
                && overview_controls.contains(&a)
                && input_containers.contains(&b)
            {
                admitted = true;
                destination = None;
            }
            // A reported list's top may coincide with its first control's top. Keep
            // the original dimension anchors, and include that source context in the detail.
            if d.label == "Vertical edge gap"
                && owner(a).is_some()
                && child_indices(v, b).iter().any(|i| owner(*i) == owner(a))
            {
                admitted = !dimensions.dimensions.iter().any(|direct| {
                    direct.label == "Input to first control gap"
                        && direct.value == d.value
                        && frame_matches(direct)
                        && direct.anchors[0].component == ca.id
                        && cb.children.contains(&direct.anchors[1].component)
                });
            }
            if d.label == "Measured left inset"
                && ((input_containers.contains(&a) && input_control(cb))
                    || (overview.contains(&a)
                        && component_role(ca).eq_ignore_ascii_case("dialog")
                        && !input_containers.contains(&a)
                        && ca.children.contains(&cb.id)
                        && same_frame(ca, cb)))
            {
                if !insets.iter().any(|(_, parent, _)| *parent == a) {
                    insets.push((d, a, b));
                }
                continue;
            }
            let footer = matches!(&cb.native_role,Some(Availability::Known {value:Value::Text(name)}) if name=="FOOTER");
            if footer
                && owner(a).is_some()
                && matches!(
                    d.label.as_str(),
                    "Measured right inset" | "Measured bottom inset"
                )
            {
                admitted = true;
            }
            if d.label == "Root left edge inset"
                && overview.contains(&a)
                && matches!(
                    component_role(cb).to_ascii_lowercase().as_str(),
                    "label" | "text"
                )
            {
                admitted = true;
                destination = None;
            }
            if d.label == "Root right edge inset"
                && overview.contains(&b)
                && owner(a).is_some()
                && matches!(
                    component_role(ca).to_ascii_lowercase().as_str(),
                    "select" | "combobox"
                )
            {
                admitted = true;
                destination = None;
            }
        }
        if !admitted {
            continue;
        }
        let signature = format!(
            "{:?}|{}|{:?}|{:?}|{:?}|{}|{}→{}",
            destination,
            d.label,
            d.anchors[0].edge,
            d.anchors[1].edge,
            d.units,
            value.to_bits(),
            component_role(ca),
            component_role(cb)
        );
        if seen.contains(&signature) {
            continue;
        }
        seen.push(signature);
        lines.push((destination, d, a, b));
    }
    // Every selected scalar has one owner. Paired width/height share their
    // component, so splitting cannot separate the two labels of one frame.
    let owns = |owner: Option<usize>, a: usize| {
        sheet.is_none_or(|sheet| {
            let destination = if owner.is_some() {
                sheets.iter().find(|s| {
                    s.view == v.id
                        && s.kind == "detail"
                        && s.components.contains(&v.components[a].id)
                })
            } else {
                None
            };
            destination.map_or(sheet.kind == "general", |s| s.id == sheet.id)
        })
    };
    lines.retain(|(owner, _, a, _)| owns(*owner, *a));
    insets.retain(|(_, a, _)| owns(Some(0), *a));
    let dimension_ids: Vec<_> = lines
        .iter()
        .map(|(_, d, _, _)| d.id.clone())
        .chain(insets.iter().map(|(d, _, _)| d.id.clone()))
        .collect();
    let assignment = if sheet.is_some_and(|s| s.kind == "detail") {
        if dimension_ids.is_empty() {
            Assignment::SourceDetail
        } else {
            Assignment::Measurements
        }
    } else {
        Assignment::Overview
    };
    let mut out = String::new();
    let anchor_objects = lines
        .iter()
        .flat_map(|(_, _, a, b)| [*a, *b])
        .chain(insets.iter().flat_map(|(_, a, b)| [*a, *b]))
        .chain(
            members
                .iter()
                .enumerate()
                .filter(|(i, _)| lines.iter().any(|(owner, _, _, _)| *owner == Some(*i)))
                .flat_map(|(_, m)| m.iter().copied()),
        )
        .collect::<Vec<_>>();
    let component_ids = v
        .components
        .iter()
        .enumerate()
        .filter(|(i, c)| {
            sheet.is_none_or(|s| s.components.contains(&c.id)) || anchor_objects.contains(i)
        })
        .map(|(_, c)| c.id.clone())
        .collect::<Vec<_>>();
    let measurement_targets = lines
        .iter()
        .flat_map(|(_, _, a, b)| [*a, *b])
        .chain(insets.iter().flat_map(|(_, a, b)| [*a, *b]))
        .collect::<std::collections::BTreeSet<_>>();
    append(
        &mut out,
        &if assignment == Assignment::Measurements {
            measurement_inventory(v, &measurement_targets, limit)?
        } else {
            drawing_inventory(
                v,
                partial,
                &anchor_objects,
                sheet.map(|_| component_ids.as_slice()),
                assignment,
                limit,
            )?
        },
        limit,
    )?;
    if assignment == Assignment::SourceDetail {
        append(
            &mut out,
            "INCLUSION BOUNDARY: only listed source members are candidates for artwork. Original scope, offscreen exclusions and known cuts still apply. The full reference constrains visible appearance; it does not authorize unlisted parents or siblings. Do not complete clipped or excluded content.\n",
            limit,
        )?;
    }
    append(
        &mut out,
        if sheet.is_some_and(|s| s.kind == "detail") && !dimension_ids.is_empty() {
            "VIEW PLAN: ONLY this detail sheet's assigned measurement blocks, no overview or UI artwork. Each label has one owner; never repeat measurements from another sheet.\nASSIGNED MEASURES:\n"
        } else if sheet.is_some_and(|s| s.kind == "detail") {
            "VIEW PLAN: selected source-detail artwork only. This detail has no assigned measurements; do not invent measurement blocks or dimensions. Preserve this source selection and its proved component clipping.\n"
        } else {
            "VIEW PLAN: complete overview plus ONLY the following supported local details. In each detail, Reuse lists visual context; print only the dimensions explicitly assigned below it. Each label has one owner.\nOVERVIEW MEASURES:\n"
        },
        limit,
    )?;
    let overview_measures = lines
        .iter()
        .filter(|(owner, _, a, b)| owner.is_none() && a == b)
        .map(|(_, d, a, b)| (*d, *a, *b))
        .collect::<Vec<_>>();
    append(
        &mut out,
        &local_measures(v, &overview_measures, partial, limit)?,
        limit,
    )?;
    for (i, (title, controls)) in groups.iter().enumerate() {
        let owned = lines
            .iter()
            .filter(|(owner, _, a, b)| *owner == Some(i) && a == b)
            .map(|(_, d, a, b)| (*d, *a, *b))
            .collect::<Vec<_>>();
        if owned.is_empty() {
            continue;
        }
        append(
            &mut out,
            &format!(
                "LOCAL DETAIL {:?} — partial; assigned frames and source state:\n",
                title
            ),
            limit,
        )?;
        append(
            &mut out,
            &format!(
                "Reuse {}.\n",
                members[i]
                    .iter()
                    .copied()
                    .filter(|index| assignment != Assignment::Measurements
                        || measurement_targets.contains(index))
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|i| format!("#{}", i + 1))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            limit,
        )?;
        if controls.len() > 1 && assignment != Assignment::Measurements {
            append(
                &mut out,
                "Keep recorded row/column relationships. Do not add a wrapper/heading/control around these partial objects.\n",
                limit,
            )?;
        }
        append(&mut out, &local_measures(v, &owned, partial, limit)?, limit)?;
    }
    let scalar_fragments = lines
        .iter()
        .filter(|(_, _, a, b)| a != b)
        .map(|(_, d, a, b)| (*d, *a, *b))
        .collect::<Vec<_>>();
    if !scalar_fragments.is_empty() || !insets.is_empty() {
        append(
            &mut out,
            "LOCAL EDGE DETAILS — each block below is ONLY two source-edge fragments, never whole controls/parents.\n",
            limit,
        )?;
    }
    append(
        &mut out,
        &local_measures(v, &scalar_fragments, partial, limit)?,
        limit,
    )?;
    for (d, a, b) in insets {
        append(
            &mut out,
            "LOCAL INSET DETAIL — one magnified source edge relation, not another UI panel.\n",
            limit,
        )?;
        append(
            &mut out,
            &local_measures(v, &[(d, a, b)], partial, limit)?,
            limit,
        )?;
    }
    Ok(Presentation {
        text: out,
        component_ids,
        dimension_ids,
        assignment,
    })
}

const INPUT_ARTWORK_GUIDANCE: &str = r#"Accessible names identify controls; they are NOT values. Draw sourced placeholders
only for explicitly empty inputs, and caller draft only when explicitly reported.
The name is NOT field content. Never replace visible public draft with its accessible
name. "#;
const ARTWORK_GUIDANCE: &str = r#"Exact visible UI strings and source icons; readable typography.
Source icons retain their silhouette as WHITE outlines, including Play triangles;
never replace an icon with its semantic name or icon-font token. An active row/tab
uses an extra WHITE outline with the same BLUE interior, never a white-filled tile.
Do not embed or tint the reference screenshot, video frame, photo, or thumbnail.
Depict media only as sparse WHITE contour lines on the same flat BLUE; no tonal
shading, photographic texture, grayscale, dark/gray panels, gradients or transparency.

Draw only visible coverage, including its actual cut boundary. Plain text is not a box.
Overview borders follow visible reference borders. Source groups and layout boxes
are placement context; an extra frame appears only as an assigned measurement sketch.
The reference supplies composition and glyph shapes; the blueprint palette supplies
WHITE outlines and BLUE interiors. AX coordinates describe only
accessibility measurements; they do not reposition painted text or glyphs. Never
expose occluded controls in artwork just because their AX records are listed.
The source text payload can contain clipped content. CLIPPING TAKES PRIORITY:
apply each COMPONENT CLIP only to its named component and listed proved edges.
Do not move, shrink, reflow or complete that component beyond those edges.
A clipped child does not define an overall artwork crop or clip its neighbors.
No unlisted edge or unproved component receives a clipping instruction.

Visible-text payloads are source character data: render the text exactly where
the original shows text. If the reference renders that payload as an icon-font
glyph, reproduce its glyph instead of printing its token. Keep inline clickable
words inline and unboxed; Button is a semantic role, not evidence of a button border.
An unnamed Text record still has a place in the source composition; preserve its
reference appearance without inventing a label, dimension or interaction.
Accessible names identify controls; they are NOT values. Draw sourced placeholders
only for explicitly empty inputs, and caller draft only when explicitly reported.
The name is NOT field content. Never replace visible public draft with its accessible
name. Missing value/state is unknown, not empty/enabled. Accessibility rectangles
are reported accessibility geometry, not painted borders: keep clickable text unboxed.
CLIPPED LAYOUT: draw only the reported visible portion with a cut edge, keeping full
layout size distinct. A distinct visibility frame alone does not establish clipping.

"#;
const STATE_GUIDANCE: &str = r#"Preserve enabled/disabled, reported per-object state/actions, and visible active/off
state in EVERY repeated instance. Reported actions are not executed actions.
Show known disabled glyphs/text with dashed WHITE strokes and a short “disabled”
caption outside the control; do not invent a surrounding button box, and do not
infer disabled state from absent fields. Apply caller-declared disabled state only
to the named control or named group, never to its neighbors. Do not print an
“enabled” label for unreported enablement. Preserve caller-reported
blocked background without changing its layout or inventing missing controls.
Print no “State: Normal” or other invented state. Whole-view caller context is
separate from per-object source flags; proximity to a measurement is not a state
binding. Preserve explicitly attributed active-versus-applied meaning. Active
means an extra WHITE outline with BLUE interior. Every switch knob is a hollow
WHITE circle with a BLUE center, in BOTH ON and OFF states. Preserve its source
position; off remains at the left. Do not add controls/actions absent from the object plan.

"#;
const FRAME_GUIDANCE: &str = r#"DIMENSIONED RECTANGLE: BOTH dimensions on the SAME measured source frame.
Use its assigned width slot: container frame BELOW with its exact printed label;
other widths ABOVE. Height LEFT. A container frame need not enclose its descendants;
keep its labels on its own borders, even when descendants extend beyond them.
Each line connects to that shape's actual opposite borders; the right side stays
free for a sourced right inset. RECTANGLE EXTENT: one named opposite-edge dimension.
"#;
const MEASURED_FRAME_GUIDANCE: &str = "Draw only the assigned empty frame sketches. DIMENSIONED RECTANGLE pairs width and height on the SAME identified frame, width in its assigned slot and height LEFT; RECTANGLE EXTENT shows only its named edges. Captions stay outside. Never enclose a parent bar or sibling icons in a child's frame. CSS px and approximation marks stay literal.\n";
const AX_GUIDANCE: &str = r#"AX FRAME MEASUREMENT or AX FRAME EXTENT is a separate measurement sketch, not a picture of a control.
Draw an EMPTY WHITE dashed rectangle. Its interior is only flat BLUE: NO text,
icon, button, word, glyph or UI reconstruction inside it. Put “AX frame” and the
EXACT OUTSIDE CAPTION below the sketch; width ABOVE, height LEFT.
Use that exact caption for every reference to its measured object,
without an explanatory suffix or category.
Recorded state captions remain separate.
This ABOVE rule applies to every AX sketch including enclosing/window frames.
Keep sketches separate from artwork; never use AX coordinates to rearrange artwork.
Never turn an AX rectangle into a new painted button, visible glyph, text-field
border or hit region. In particular a window-sized semantic Play/Search region
does not create a window-sized glyph or an additional visible Search button.
In the overview keep the exact original icon/text appearance, without inventing size.
Only explicitly prepared caller/observed state may be captioned outside the frame.
No Normal/enabled/disabled default state exists.
"#;
const EDGE_GUIDANCE: &str = r#"EDGE FRAGMENT blocks are independent of the full-shape size views. Redraw ONLY
their two named edge-corner fragments and the one dimension arrow, preserving
orientation and named source boundaries. The two printed names are their captions.
Do not import the full parent/control rectangles from other views into a fragment.
An unpainted label-layout boundary belongs to the named text; include that short
text context by the boundary. An outer boundary never means an inner control edge.
NAMED-ANCHOR RELATION is conservative attribution, never a border-corner sketch.
"#;

fn render_view(
    b: &DrawingBrief,
    v: &SceneView,
    plan: &Presentation,
    sheet_plan: String,
    design_analysis: String,
    limit: usize,
) -> Result<String> {
    let mut result = String::new();
    let assignment = plan.assignment;
    let presentation = &plan.text;
    let mut vars = BTreeMap::new();
    vars.insert("title", b.metadata.title.clone());
    vars.insert("language", b.metadata.language.clone());
    vars.insert("output_size", b.metadata.output_size.clone());
    vars.insert("page_format", b.metadata.page_format.clone());
    vars.insert("purpose", format!("{:?}", b.purpose));
    vars.insert("source", format!("{:?}", v.source_kind));
    vars.insert(
        "coverage",
        v.coverage
            .as_ref()
            .map(|x| format!("{:?}", x.status))
            .unwrap_or_else(|| "proposed scope".into()),
    );
    vars.insert("approval", format!("{:?}", b.metadata.approval.status));
    vars.insert(
        "source_limit",
        if v.observations.is_empty() {
            "proposed, not observed".into()
        } else {
            "stored observation; atomicity not asserted".into()
        },
    );
    vars.insert("state", v.state.clone());
    vars.insert(
        "scope",
        format!(
            "Declared scope: {}. Explicit exclusions: {}",
            v.scope,
            v.not_depicted.join("; ")
        ),
    );
    vars.insert("requirements", v.requirements.join("; "));
    vars.insert(
        "units",
        units(v)
            .into_iter()
            .map(unit_name)
            .collect::<Vec<_>>()
            .join(", "),
    );
    vars.insert(
        "frame_guidance",
        if presentation.lines().any(|line| {
            line.starts_with("DIMENSIONED RECTANGLE ") || line.starts_with("RECTANGLE EXTENT ")
        }) {
            if assignment == Assignment::Measurements {
                MEASURED_FRAME_GUIDANCE
            } else {
                FRAME_GUIDANCE
            }
        } else {
            ""
        }
        .into(),
    );
    vars.insert(
        "ax_label",
        if presentation
            .lines()
            .any(|line| line.starts_with("AX FRAME "))
        {
            "Print “AX frame” for the prepared accessibility measurements.\n"
        } else {
            ""
        }
        .into(),
    );
    vars.insert(
        "ax_guidance",
        if presentation
            .lines()
            .any(|line| line.starts_with("AX FRAME "))
        {
            AX_GUIDANCE
        } else {
            ""
        }
        .into(),
    );
    vars.insert(
        "edge_guidance",
        if presentation.lines().any(|line| {
            line.starts_with("EDGE FRAGMENT (") || line.starts_with("NAMED-ANCHOR RELATION:")
        }) {
            EDGE_GUIDANCE
        } else {
            ""
        }
        .into(),
    );
    vars.insert("printable_content", match assignment {
        Assignment::Measurements => "only assigned dimension labels, frame/anchor names and explicitly supplied state outside EMPTY sketches; no UI contents or source inventory",
        Assignment::SourceDetail => "only the included source fragment's visible labels/text/icons and its declared state; no dimensions or unlisted source content",
        Assignment::Overview => "verbatim visible labels/draft, assigned dimensions/anchor captions and explicit state of the selected overview",
    }.into());
    vars.insert("dimensions", presentation.clone());
    vars.insert("assignment", match assignment {
        Assignment::Overview => "One selected-scope overview with only this sheet's assigned measurements. Other source regions in the reference are context, not permission to extend the scope.",
        Assignment::SourceDetail => "SOURCE FRAGMENT ONLY. Listed members bound this sheet's source scope, not their visible paint. Apply original scope/offscreen exclusions and supplied reference/known-cut constraints before depicting any portion. The parent screen/card and unlisted siblings remain excluded. No measurements are assigned.",
        Assignment::Measurements => "MEASUREMENT ONLY. Draw the assigned EMPTY frame/edge sketches, with all semantic captions OUTSIDE. No UI artwork, screenshots, controls, text or icons inside frames. The original reference is attribution context only. Unassigned source objects, parents and siblings are not drawings on this sheet.",
    }.into());
    let mut artwork_guidance = if assignment == Assignment::Measurements {
        String::new()
    } else {
        ARTWORK_GUIDANCE.into()
    };
    if assignment == Assignment::SourceDetail
        && (!v.components.iter().any(input_control)
            || v.components
                .iter()
                .filter(|c| input_control(c))
                .any(|c| !plan.component_ids.contains(&c.id)))
    {
        // Caller state has no structured field binding. Do not guess which
        // omitted input its draft describes from prose or from the reference.
        vars.insert("state", format!("NON-RENDERING WHOLE-VIEW PROVENANCE: {:?}. Input ownership is incomplete in this fragment; this provenance does not authorize draft or input artwork here", v.state));
        artwork_guidance = artwork_guidance.replace(INPUT_ARTWORK_GUIDANCE,
            "Accessible names are not field contents. Caller-draft rendering does not apply to this fragment: input ownership is incomplete. Whole-view state is non-rendering provenance, not permission to add an input, its draft or associated content.\n");
    }
    vars.insert("artwork_guidance", artwork_guidance);
    vars.insert("state_guidance", format!("{}For an assigned object with reported Focused=true, print a separate outside state annotation meaning ‘Focused’ in the document language in each view containing it. For caller-only focus explicitly identifying an included input, annotate ‘Focused (caller-reported)’ in that language instead; it remains caller attribution, not a reported flag. Keep the source caption unchanged and AX interiors empty. Focusable or selected alone does not establish focus. Unknown/withheld focus is not reported true; reported Focused=false or conflicting statements must not become an unconditional Focused tag. Preserve the disagreement. No inferred caret position or focus shape.\n", if assignment == Assignment::Measurements { "Caption only recorded per-target flags or caller statements that explicitly identify that target, outside its empty frame. Whole-view context is not a state for every target; never spread group state to other frames. No inferred enabled/Normal state; no reconstruction of controls.\n" } else { STATE_GUIDANCE }));
    vars.insert("design_analysis", design_analysis);
    vars.insert("sheets", sheet_plan);
    let mut rest = include_str!("prompt-template.txt");
    while let Some(start) = rest.find("{{") {
        append(&mut result, &rest[..start], limit)?;
        let end = rest[start + 2..].find("}}").ok_or(E::InvalidInput)? + start + 2;
        append(
            &mut result,
            vars.get(&rest[start + 2..end]).ok_or(E::InvalidInput)?,
            limit,
        )?;
        rest = &rest[end + 2..];
    }
    append(&mut result, rest, limit)?;
    Ok(result)
}

fn prompt(
    b: &DrawingBrief,
    s: &Scene,
    d: &[ViewDimensions],
    sheets: &[Sheet],
    limit: usize,
) -> Result<String> {
    let mut result = String::new();
    for (index, v) in s.views.iter().enumerate() {
        let dims = d
            .iter()
            .find(|x| x.view == v.id)
            .ok_or(E::InvalidReference)?;
        let selected = sheets.iter().filter(|x| x.view == v.id).collect::<Vec<_>>();
        let partial = clipped_edges(&b.views[index].source);
        let presentation = drawing_dimensions(v, dims, &partial, None, sheets, limit)?;
        if s.views.len() > 1 {
            append(
                &mut result,
                &format!("\nSource view {} / {}\n", index + 1, s.views.len()),
                limit,
            )?;
        }
        let sheet_plan = selected
            .iter()
            .map(|x| format!("{}: {}", x.kind, x.title))
            .collect::<Vec<_>>()
            .join("; ");
        append(
            &mut result,
            &render_view(
                b,
                v,
                &presentation,
                sheet_plan,
                crate::design::text(&v.design_analysis, limit)?,
                limit,
            )?,
            limit,
        )?;
    }
    if !s.flow.is_empty() {
        append(
            &mut result,
            &format!(
                "\nConfirmed/unknown flow: {}",
                bounded_compact_json(&s.flow, limit)?
            ),
            limit,
        )?;
    }
    if !s.comparison_results.is_empty() {
        append(
            &mut result,
            &format!(
                "\nENGINE RECORDED COMPARISON\n{}",
                bounded_compact_json(&s.comparison_results, limit)?
            ),
            limit,
        )?;
    }
    Ok(result)
}
pub(crate) fn append(out: &mut String, s: &str, limit: usize) -> Result<()> {
    require(s.len() <= limit.saturating_sub(out.len()), E::OutputLimit)?;
    out.push_str(s);
    Ok(())
}
fn drawing_brief(
    b: &DrawingBrief,
    s: &Scene,
    d: &[ViewDimensions],
    sheets: &[Sheet],
    limit: usize,
) -> Result<String> {
    let mut out = String::new();
    append(
        &mut out,
        &format!(
            "# {}\n\nGuide: {GUIDE}. Purpose: {:?}.\n\n",
            b.metadata.title, b.purpose
        ),
        limit,
    )?;
    for v in &s.views {
        append(
            &mut out,
            &crate::design::text(&v.design_analysis, limit)?,
            limit,
        )?;
    }
    for (heading, data) in [
        (
            "Identity, audience, approval, owner and retention",
            json(&b.metadata, limit)?,
        ),
        (
            "Scope, coverage, environment, named states, source, components, relations and unknowns",
            json(s, limit)?,
        ),
        (
            "Dimensions, anchors, units, evidence and derived chains",
            json(&d, limit)?,
        ),
        ("Sheet plan", json(&sheets, limit)?),
    ] {
        append(
            &mut out,
            &format!("## {heading}\n\n```json\n{data}\n```\n\n"),
            limit,
        )?;
    }
    append(
        &mut out,
        "## Style, exact labels and forbidden changes\n\nBlue Engineering: flat #0B5E9E background, white contours and text, no grid or control fills, orthogonal front view, line hierarchy and compact footer. Use exact quoted labels as data. Preserve every selected object, relation, source, frame kind, unit and unknown. No invented controls, radius, padding, baseline, tolerance or geometry from pixels. Requirements remain proposed. Scale: schematic. Размеры по подписям; не измерять по изображению.\n\n## Verification\n\nLocal references, arithmetic, units and explicit public-text policy checked. validation_status=unverified for the ungenerated image; approval is a separate supplied record. Generation is a separate user action. Review every ID, number, anchor, state, source, scope, readability, coverage and privacy against these files before marking an image checked. Never replace an accepted baseline with current runtime. No CAD-scale guarantee, model call, pixel reference, or runtime capture.\n",
        limit,
    )?;
    Ok(out)
}

#[cfg(test)]
mod presentation_tests {
    use super::display_number;
    #[test]
    fn labels_round_without_false_zero_or_precision() {
        for (value, label) in [
            (421.640625, "≈422"),
            (0.5, "≈1"),
            (-0.5, "≈-1"),
            (0.25, "<0.5"),
            (-0.25, "−<0.5"),
            (-0.0, "0"),
            (16.0, "16"),
        ] {
            assert_eq!(display_number(value), label);
        }
    }
}
