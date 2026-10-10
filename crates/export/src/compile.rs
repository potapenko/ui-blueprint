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
                (observed::dimensions(s, &mut aliases)?, vec![])
            }
        };
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
    let manifest = serde_json::json!({"package_version":if brief.comparisons.is_empty() {"0.1.0"} else {"0.2.0"},"guide":GUIDE,"guide_reference":"docs engineering-blueprint-guide.md revision 1.2",
        "document":brief.metadata,"purpose":brief.purpose,"source_kinds":scene.views.iter().map(|v|v.source_kind).collect::<Vec<_>>(),
        "validation_status":"unverified","local_numeric_validation":"checked","approval_status":brief.metadata.approval.status,
        "scale_mode":"schematic","generated_image":false,"references":[],
        "files":["manifest.json","drawing-brief.md","scene.json","dimensions.json","sheets.json","prompt.txt"],
        "retention_owner":brief.metadata.owner,"retention_condition":brief.metadata.retention});
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
fn display_number(value: f64) -> String {
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
    )
}
fn unit_name(unit: Unit) -> &'static str {
    match unit {
        Unit::CssPx => "CSS px",
        Unit::Px => "px",
        Unit::Pt => "pt",
        Unit::Dp => "dp",
    }
}
fn component_name(v: &SceneView, index: usize) -> String {
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
    let visible = public_label(c);
    let aggregate = child_indices(v, index).iter().any(|&i| {
        let child_label = public_label(&v.components[i]);
        !child_label.is_empty() && visible.contains(child_label)
    });
    let label =
        accessible.or_else(|| (!aggregate && !visible.contains(['\n', '\r'])).then_some(visible));
    if let Some(label) = label.filter(|label| !label.is_empty() && label.len() <= 80) {
        return format!("{human_role} {:?}", label.replace('\n', " / "));
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
fn state_caption(v: &SceneView) -> String {
    // Only quote declared state clauses. Selected alone never implies applied.
    v.state
        .split(';')
        .map(str::trim)
        .filter(|clause| {
            clause
                .split(|c: char| !c.is_alphabetic() && c != '-')
                .any(|word| {
                    matches!(
                        word.to_ascii_lowercase().as_str(),
                        "active" | "inactive" | "keyboard-active" | "applied" | "not-applied"
                    )
                })
        })
        .collect::<Vec<_>>()
        .join("; ")
}
fn printable_state(v: &SceneView) -> String {
    let caption = state_caption(v);
    if caption.is_empty() {
        return String::new();
    }
    format!(
        "PRINT THIS VIEW'S STATE CAPTION exactly: {caption:?}. Keep the designated active object double-outlined in this view, with BLUE interior; repeated objects retain the SAME state.\n"
    )
}
fn object_text(v: &SceneView, index: usize, partial: &[bool], limit: usize) -> Result<String> {
    let c = &v.components[index];
    let aggregate = !visible_text(c).is_empty()
        && child_indices(v, index).iter().any(|&i| {
            !visible_text(&v.components[i]).is_empty()
                && visible_text(c).contains(visible_text(&v.components[i]))
        });
    let mut out = format!("Object {}: {}", component_name(v, index), component_role(c));
    if let Some(Availability::Known {
        value: Value::Text(native),
    }) = &c.native_role
        && native != &component_role(c)
    {
        out.push_str(&format!(
            " (native {})",
            bounded_compact_json(native, limit)?
        ));
    }
    if !aggregate && !visible_text(c).is_empty() {
        out.push_str(&format!(
            "; print {}",
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
        {
            out.push_str(&format!(
                "; {:?}={}",
                field,
                bounded_compact_json(value, limit)?
            ));
        }
    }
    if input_control(c) {
        out.push_str("; name is NOT field content; use only explicit caller draft, or placeholder when explicitly empty");
    }
    for p in &c.properties {
        if let Some(Value::Flag(value)) = p.known()
            && (matches!(
                p.field(),
                Field::Checked | Field::Selected | Field::Expanded | Field::Enabled
            ) || p.field() == Field::Focused && *value)
        {
            out.push_str(&format!("; {:?}={value}", p.field()));
            if p.field() == Field::Checked && !input_control(c) {
                out.push_str("; indicator interior BLUE, WHITE OUTLINE only");
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
        out.push_str(&format!("; placement context {}", geometry_context(v, g)));
        match &g.shape {
            Shape::Rect(r) => out.push_str(&format!(
                "; {:?} box[x {}, y {}, w {}, h {}] {} {:?}",
                g.frame_kind,
                display_number(r.x),
                display_number(r.y),
                display_number(r.width),
                display_number(r.height),
                unit_name(g.coordinate_space.units),
                g.coordinate_space.origin
            )),
            _ => out.push_str("; reported nonrectangular geometry; do not invent a rectangle"),
        }
        for p in &c.properties {
            if let Some(Value::Geometry(visible)) = p.known()
                && p.field() == Field::VisibleRegion
                && visible.shape != g.shape
            {
                out.push_str(&format!(
                    "; {} reported visible-region {} (Space {} {:?}, {}, {:?})",
                    if partial.get(index).copied().unwrap_or(false) {
                        "CLIPPED LAYOUT: draw only"
                    } else {
                        "DISTINCT FRAME (clipping not established):"
                    },
                    bounded_compact_json(&visible.shape, limit)?,
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
fn drawing_inventory(
    v: &SceneView,
    partial: &[bool],
    anchors: &[usize],
    limit: usize,
) -> Result<String> {
    let mut out = String::new();
    append(
        &mut out,
        "OVERVIEW CONTENT. Human object names below identify source shapes; only quoted UI text appears inside them. Placement references/coordinates are construction input, not captions.\n",
        limit,
    )?;
    append(&mut out, &printable_state(v), limit)?;
    let mut contexts = Vec::new();
    for g in v.components.iter().filter_map(geometry) {
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
                    "Placement context {}: {context}. No guessed conversions.\n",
                    geometry_context(v, g)
                ),
                limit,
            )?;
            contexts.push(context);
        }
    }
    for (i, c) in v.components.iter().enumerate() {
        let text = !visible_text(c).is_empty()
            && !child_indices(v, i).iter().any(|&j| {
                !visible_text(&v.components[j]).is_empty()
                    && visible_text(c).contains(visible_text(&v.components[j]))
            });
        let visual = matches!(
            component_role(c).to_ascii_lowercase().as_str(),
            "img" | "image" | "svg" | "canvas" | "media" | "custom"
        );
        if control(c) || text || visual || anchors.contains(&i) {
            append(&mut out, &object_text(v, i, partial, limit)?, limit)?;
        }
    }
    Ok(out)
}
fn local_measures(
    v: &SceneView,
    measures: &[(&Dimension, usize, usize)],
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
                    && m.anchors[0].edge == Edge::Left
                    && m.anchors[1].edge == Edge::Right
            });
            let height = measures.iter().find(|(m, x, y)| {
                *x == a
                    && *y == a
                    && m.anchors[0].edge == Edge::Top
                    && m.anchors[1].edge == Edge::Bottom
            });
            if let (Some((w, _, _)), Some((h, _, _))) = (width, height) {
                let enclosing = !control(&v.components[a]) && !v.components[a].children.is_empty();
                append(
                    &mut out,
                    &format!(
                        "DIMENSIONED RECTANGLE {}: WIDTH {} {} {}; HEIGHT {} {} on the LEFT. BOTH lines required, each attached to the real opposite borders.\n",
                        component_name(v, a),
                        display_number(w.value.ok_or(E::InvalidGeometry)?),
                        unit_name(w.units),
                        if enclosing {
                            "BELOW the enclosing shape"
                        } else {
                            "ABOVE"
                        },
                        display_number(h.value.ok_or(E::InvalidGeometry)?),
                        unit_name(h.units)
                    ),
                    limit,
                )?;
                if enclosing {
                    append(
                        &mut out,
                        &format!(
                            "PRINT BELOW-SHAPE WIDTH LABEL exactly: {:?}. This is the width line's own label, not another note or dimension.\n",
                            format!(
                                "{} — width {} {}",
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
                    "RECTANGLE EXTENT {}: {:?}→{:?}, {} {}.\n",
                    component_name(v, a),
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
    )
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
fn partial_visibility(source: &SourceInput) -> Vec<bool> {
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
                return false;
            };
            if layout.frame_kind != FrameKind::LayoutBounds
                || visible.frame_kind != FrameKind::VisibleRegion
                || layout.coordinate_space != visible.coordinate_space
                || le.observation_id != ve.observation_id
                || le.source_namespace != ve.source_namespace
                || !matches!(layout.shape, Shape::Rect(_))
                || !matches!(visible.shape, Shape::Rect(_))
            {
                return false;
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
                        [left, top, right, bottom].iter().all(|x| *x >= 0.0)
                            && [left, top, right, bottom].iter().any(|x| *x > 0.0)
                    }
                    _ => false,
                },
                _ => false,
            }
        })
        .collect()
}
fn detail_caption(v: &SceneView, members: &[usize]) -> String {
    let parent = members.first().and_then(|first| {
        v.components
            .iter()
            .position(|p| p.children.contains(&v.components[*first].id))
    });
    let ancestor = parent.and_then(|parent| {
        v.components
            .iter()
            .position(|p| p.children.contains(&v.components[parent].id))
    });
    if let Some(ancestor) = ancestor {
        let near = child_indices(v, ancestor);
        for i in near
            .iter()
            .copied()
            .chain(near.iter().flat_map(|&i| child_indices(v, i)))
        {
            let c = &v.components[i];
            if matches!(
                component_role(c).to_ascii_lowercase().as_str(),
                "label" | "text" | "heading"
            ) && !public_label(c).is_empty()
            {
                return format!("{} — dimensions", public_label(c));
            }
        }
    }
    if members.len() == 1 && !public_label(&v.components[members[0]]).is_empty() {
        return format!(
            "{} — dimensions",
            public_label(&v.components[members[0]]).replace('\n', " / ")
        );
    }
    "Control dimensions and spacing".into()
}
fn drawing_dimensions(
    v: &SceneView,
    dimensions: &ViewDimensions,
    partial: &[bool],
    limit: usize,
) -> Result<String> {
    let index = |id: &str| v.components.iter().position(|c| c.id == id);
    let large = |i: usize| {
        control(&v.components[i])
            && !partial.get(i).copied().unwrap_or(false)
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
                && !v.components.iter().any(|p| p.children.contains(&c.id))
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
            overview.push(i);
            input_containers.push(i);
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
                admitted = true;
            }
            if d.label == "Measured left inset"
                && input_containers.contains(&a)
                && input_control(cb)
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
            "{:?}|{}|{:?}|{:?}|{:?}|{}",
            destination,
            d.label,
            d.anchors[0].edge,
            d.anchors[1].edge,
            d.units,
            value.to_bits()
        );
        if seen.contains(&signature) {
            continue;
        }
        seen.push(signature);
        lines.push((destination, d, a, b));
    }
    let mut out = String::new();
    let anchor_objects = lines
        .iter()
        .flat_map(|(_, _, a, b)| [*a, *b])
        .chain(insets.iter().flat_map(|(_, a, b)| [*a, *b]))
        .collect::<Vec<_>>();
    append(
        &mut out,
        &drawing_inventory(v, partial, &anchor_objects, limit)?,
        limit,
    )?;
    append(
        &mut out,
        "VIEW PLAN: complete overview plus ONLY the following supported local details. Each label has one owner; never repeat measurements on another view.\nOVERVIEW MEASURES:\n",
        limit,
    )?;
    let overview_measures = lines
        .iter()
        .filter(|(owner, _, a, b)| owner.is_none() && a == b)
        .map(|(_, d, a, b)| (*d, *a, *b))
        .collect::<Vec<_>>();
    append(
        &mut out,
        &local_measures(v, &overview_measures, limit)?,
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
                "LOCAL DETAIL {:?} — partial, only these source objects. Apply the same source state as the overview.\n",
                title
            ),
            limit,
        )?;
        append(&mut out, &printable_state(v), limit)?;
        for &object in &members[i] {
            append(&mut out, &object_text(v, object, partial, limit)?, limit)?;
        }
        if controls.len() > 1 {
            append(
                &mut out,
                "Keep recorded row/column relationships. Do not add a wrapper/heading/control around these partial objects.\n",
                limit,
            )?;
        }
        append(&mut out, &local_measures(v, &owned, limit)?, limit)?;
    }
    append(
        &mut out,
        "LOCAL EDGE DETAILS — each block below is ONLY two source-edge fragments, never whole controls/parents.\n",
        limit,
    )?;
    let scalar_fragments = lines
        .iter()
        .filter(|(_, _, a, b)| a != b)
        .map(|(_, d, a, b)| (*d, *a, *b))
        .collect::<Vec<_>>();
    append(
        &mut out,
        &local_measures(v, &scalar_fragments, limit)?,
        limit,
    )?;
    for (d, a, b) in insets {
        append(
            &mut out,
            "LOCAL INSET DETAIL — one magnified source edge relation, not another UI panel.\n",
            limit,
        )?;
        append(&mut out, &local_measures(v, &[(d, a, b)], limit)?, limit)?;
    }
    Ok(out)
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
        let mut vars = BTreeMap::new();
        vars.insert("title", b.metadata.title.clone());
        vars.insert("language", b.metadata.language.clone());
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
            "Selected source view only; content outside this coverage is omitted".into(),
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
        let partial = partial_visibility(&b.views[index].source);
        vars.insert("dimensions", drawing_dimensions(v, dims, &partial, limit)?);
        vars.insert(
            "sheets",
            selected
                .iter()
                .map(|x| format!("{}: {}", x.kind, x.title))
                .collect::<Vec<_>>()
                .join("; "),
        );
        let mut rest = include_str!("prompt-template.txt");
        if s.views.len() > 1 {
            append(
                &mut result,
                &format!("\nSource view {} / {}\n", index + 1, s.views.len()),
                limit,
            )?;
        }
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
fn append(out: &mut String, s: &str, limit: usize) -> Result<()> {
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
