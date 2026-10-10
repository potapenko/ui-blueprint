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
fn component_name(c: &SceneComponent, index: usize) -> String {
    let label = public_label(c).replace('\n', " / ");
    if label.is_empty() || !c.children.is_empty() && label.len() > 80 {
        format!("record {} ({})", index + 1, component_role(c))
    } else {
        format!("record {} ({})", index + 1, label)
    }
}
fn drawing_alignment(v: &SceneView, limit: usize) -> Result<String> {
    let mut out = String::new();
    let rects: Vec<_> = v
        .components
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            geometry(c).and_then(|g| match &g.shape {
                Shape::Rect(r) => Some((i, g, r)),
                _ => None,
            })
        })
        .collect();
    for (position, (i, g, r)) in rects.iter().enumerate() {
        for same_width in [false, true] {
            let matches = |other: &&(usize, &Geometry, &Rect)| {
                g.coordinate_space == other.1.coordinate_space
                    && v.components[*i].surface == v.components[other.0].surface
                    && g.frame_kind == other.1.frame_kind
                    && r.x == other.2.x
                    && (!same_width || r.width == other.2.width)
            };
            if rects[..position].iter().any(|other| matches(&other)) {
                continue;
            }
            let group = rects
                .iter()
                .filter(matches)
                .map(|(index, _, _)| index + 1)
                .collect::<Vec<_>>();
            if group.len() > 1 {
                append(
                    &mut out,
                    &format!(
                        "Records {:?}: keep exactly shared {} in overview AND details.\n",
                        group,
                        if same_width {
                            "left AND right edges (equal source width)"
                        } else {
                            "left edge"
                        }
                    ),
                    limit,
                )?;
            }
        }
    }
    Ok(out)
}
fn drawing_inventory(v: &SceneView, limit: usize) -> Result<String> {
    let mut out = String::new();
    let mut contexts: Vec<&Geometry> = Vec::new();
    for g in v.components.iter().filter_map(geometry) {
        if contexts.iter().any(|other| {
            other.coordinate_space == g.coordinate_space
                && other.frame_kind == g.frame_kind
                && other.transform == g.transform
        }) {
            continue;
        }
        contexts.push(g);
        let mapping = match &g.transform {
            TransformState::LocalOnly {} => "local only; no cross-space mapping".into(),
            TransformState::Unknown { .. } => "unknown; no cross-space mapping".into(),
            TransformState::Known { transform: t } => format!(
                "known {:?} {} {} {:?} -> {:?} {} {} {:?}, affine {:?}; source coordinates retained",
                t.from.kind,
                t.from.id.0,
                unit_name(t.from.units),
                t.from.origin,
                t.to.kind,
                t.to.id.0,
                unit_name(t.to.units),
                t.to.origin,
                t.affine
            ),
        };
        append(
            &mut out,
            &format!(
                "Geometry context {}: {:?}; Space {} {:?}, {}, {:?}; transform {}.\n",
                contexts.len(),
                g.frame_kind,
                g.coordinate_space.id.0,
                g.coordinate_space.kind,
                unit_name(g.coordinate_space.units),
                g.coordinate_space.origin,
                mapping
            ),
            limit,
        )?;
    }
    for (i, c) in v.components.iter().enumerate() {
        let label = c.label.as_deref().unwrap_or_else(|| {
            c.properties
                .iter()
                .find_map(|p| match p.known() {
                    Some(Value::Text(text)) if p.field() == Field::VisibleText => {
                        Some(text.as_str())
                    }
                    _ => None,
                })
                .unwrap_or("")
        });
        let accessible_name = c
            .properties
            .iter()
            .find_map(|p| match p.known() {
                Some(Value::Text(text)) if p.field() == Field::AccessibilityName => {
                    Some(text.as_str())
                }
                _ => None,
            })
            .unwrap_or("");
        let field_content = if matches!(
            component_role(c).to_ascii_lowercase().as_str(),
            "input" | "textbox" | "searchbox"
        ) {
            format!(
                " FIELD CONTENT: only the explicit reviewed draft from caller state ({}) may be drawn. Accessible name is identification, NEVER the field's content.",
                v.state
            )
        } else {
            String::new()
        };
        let mut display_facts = String::new();
        for field in [Field::Placeholder, Field::InputKind] {
            if let Some(Value::Text(value)) = c
                .properties
                .iter()
                .find(|p| p.field() == field)
                .and_then(Property::known)
            {
                append(
                    &mut display_facts,
                    &format!(
                        " {:?}={} (not a field value).",
                        field,
                        bounded_compact_json(value, limit)?
                    ),
                    limit,
                )?;
            }
        }
        if clipped(c) {
            for p in &c.properties {
                if let Some(Value::Geometry(g)) = p.known()
                    && p.field() == Field::VisibleRegion
                {
                    append(
                        &mut display_facts,
                        &format!(
                            " CLIPPED: reported visible-region {}; show only this portion, never reinterpret full layout height as visible height.",
                            bounded_compact_json(&g.shape, limit)?
                        ),
                        limit,
                    )?;
                }
            }
        }
        // Container text that repeats a child's text is an aggregate, not another label/control.
        let aggregate = !label.is_empty()
            && c.children
                .iter()
                .filter_map(|id| v.components.iter().find(|x| &x.id == id))
                .any(|child| {
                    !public_label(child).is_empty() && label.contains(public_label(child))
                });
        let label = if aggregate { "" } else { label };
        let bounds = geometry(c)
            .map(|g| match &g.shape {
                Shape::Rect(r) => format!(
                    "context {}: x {}, y {}, w {}, h {}",
                    contexts
                        .iter()
                        .position(|other| other.coordinate_space == g.coordinate_space
                            && other.frame_kind == g.frame_kind
                            && other.transform == g.transform)
                        // Every geometry was collected into contexts above.
                        .expect("geometry context collected above")
                        + 1,
                    display_number(r.x),
                    display_number(r.y),
                    display_number(r.width),
                    display_number(r.height)
                ),
                _ => "nonrectangular geometry; do not invent a rectangle".into(),
            })
            .unwrap_or_else(|| "geometry unavailable".into());
        let children = c
            .children
            .iter()
            .filter_map(|id| v.components.iter().position(|x| &x.id == id).map(|n| n + 1))
            .collect::<Vec<_>>();
        let parent = c
            .parent
            .as_ref()
            .and_then(|id| v.components.iter().position(|x| &x.id == id).map(|n| n + 1));
        let states = c
            .properties
            .iter()
            .filter_map(|p| match p.known() {
                Some(Value::Flag(value))
                    if matches!(
                        p.field(),
                        Field::Checked | Field::Selected | Field::Expanded | Field::Enabled
                    ) || p.field() == Field::Focused && *value =>
                {
                    Some(format!("{:?}={value}", p.field()))
                }
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(", ");
        append(
            &mut out,
            &format!(
                "{}: {}{} | visible text {}{} | {} | children {:?}, parent {:?} {} {}{}\n",
                i + 1,
                component_role(c),
                match &c.native_role {
                    Some(Availability::Known {
                        value: Value::Text(name),
                    }) if name != &component_role(c) =>
                        format!("; native {}", bounded_compact_json(name, limit)?),
                    _ => String::new(),
                },
                bounded_compact_json(&label, limit)?,
                if accessible_name.is_empty() || accessible_name == label {
                    String::new()
                } else {
                    format!(
                        " | accessible name (not field content) {}",
                        bounded_compact_json(&accessible_name, limit)?
                    )
                },
                bounds,
                children,
                parent,
                states,
                c.state_and_actions.as_deref().unwrap_or(""),
                format_args!("{field_content}{display_facts}")
            ),
            limit,
        )?;
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
fn clipped(c: &SceneComponent) -> bool {
    let Some(g) = geometry(c) else { return false };
    c.properties.iter().any(|p|matches!(p.known(),Some(Value::Geometry(visible))
        if p.field()==Field::VisibleRegion && visible.coordinate_space==g.coordinate_space && visible.shape!=g.shape))
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
fn drawing_dimensions(v: &SceneView, dimensions: &ViewDimensions, limit: usize) -> Result<String> {
    let index = |id: &str| v.components.iter().position(|c| c.id == id);
    let large = |i: usize| {
        control(&v.components[i])
            && !clipped(&v.components[i])
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
        if input_control(c) || attachment {
            overview_controls.push(i);
        } else {
            groups.push((detail_caption(v, &[i]), vec![i]));
        }
    }
    let owner = |i: usize| groups.iter().position(|(_, m)| m.contains(&i));
    let mut lines: Vec<(Option<usize>, String)> = Vec::new();
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
        if let Some(group) = destination {
            for node in [a, b] {
                if !members[group].contains(&node) {
                    members[group].push(node);
                }
            }
        }
        lines.push((
            destination,
            format!(
                "REQUIRED {}: start {} {:?} edge; end {} {:?} edge; label {} {}.\n",
                d.label,
                component_name(ca, a),
                d.anchors[0].edge,
                component_name(cb, b),
                d.anchors[1].edge,
                display_number(value),
                unit_name(d.units)
            ),
        ));
    }
    let mut out = String::new();
    append(
        &mut out,
        "VIEW PLAN — draw ONLY the views listed here. Complete overview preserves all meaningful source controls; details are explicitly partial, not additional UI.\nOVERVIEW: full declared visible scope; no invented offscreen content.\n",
        limit,
    )?;
    for (_, line) in lines.iter().filter(|(owner, _)| owner.is_none()) {
        append(&mut out, line, limit)?;
    }
    for (i, (title, controls)) in groups.iter().enumerate() {
        let owned = lines
            .iter()
            .filter(|(owner, _)| *owner == Some(i))
            .collect::<Vec<_>>();
        if owned.is_empty() {
            continue;
        }
        append(
            &mut out,
            &format!(
                "DETAIL {} — {:?}, PARTIAL. Draw source controls {:?}; additional named anchor context {:?}. Do not turn an anchor context into another control or extra wrapper. No other controls in this detail.\n",
                i + 1,
                title,
                controls.iter().map(|n| n + 1).collect::<Vec<_>>(),
                members[i]
                    .iter()
                    .filter(|n| !controls.contains(n))
                    .map(|n| n + 1)
                    .collect::<Vec<_>>()
            ),
            limit,
        )?;
        append(
            &mut out,
            "Draw only these controls on their recorded common axes; no enclosing parent frame or extra header. Gap arrows touch actual control borders, never an added separator. Any named text-layout edge is a short labelled construction edge, not the end of glyphs.\n",
            limit,
        )?;
        if members[i].iter().any(|&n|matches!(&v.components[n].native_role,Some(Availability::Known {value:Value::Text(name)}) if name=="FOOTER")) {
            append(&mut out,"This detail additionally needs the actual footer RIGHT and BOTTOM border fragments for its named insets; label those edges. Keep button WIDTH and HEIGHT present; no top inset or decorative enclosing frame.\n",limit)?;
        }
        for (_, line) in owned {
            append(&mut out, line, limit)?;
        }
    }
    for (d, a, b) in insets {
        append(
            &mut out,
            &format!(
                "INSET DETAIL — source {} to {}. Show ONLY two nested source corner fragments (parent left border plus short top; control left border plus short top). A single double arrow joins these uninterrupted borders directly. No enclosing frame, unrelated guides, or duplicate inset in any other view. REQUIRED left inset label {} {}.\n",
                component_name(&v.components[a], a),
                component_name(&v.components[b], b),
                display_number(d.value.unwrap_or(0.0)),
                unit_name(d.units)
            ),
            limit,
        )?;
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
        vars.insert("source", format!("{:?}; {}", v.source_kind, v.source));
        vars.insert(
            "coverage",
            v.coverage
                .as_ref()
                .map(|x| format!("{:?}", x.status))
                .unwrap_or_else(|| "proposed scope".into()),
        );
        vars.insert("approval", format!("{:?}", b.metadata.approval.status));
        vars.insert("state", v.state.clone());
        vars.insert(
            "scope",
            format!("{}. Not depicted: {}", v.scope, v.not_depicted.join("; ")),
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
        vars.insert("alignments", drawing_alignment(v, limit)?);
        vars.insert("components", drawing_inventory(v, limit)?);
        vars.insert("dimensions", drawing_dimensions(v, dims, limit)?);
        vars.insert(
            "sheets",
            selected
                .iter()
                .map(|x| format!("{} {} {}", x.id, x.kind, x.title))
                .collect::<Vec<_>>()
                .join("; "),
        );
        let mut rest = include_str!("prompt-template.txt");
        append(
            &mut result,
            &format!("\nVIEW {} / {}\n", index + 1, s.views.len()),
            limit,
        )?;
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
