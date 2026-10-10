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
                field_content
            ),
            limit,
        )?;
    }
    Ok(out)
}
fn drawing_dimensions(v: &SceneView, dimensions: &ViewDimensions, limit: usize) -> Result<String> {
    let mut out = String::new();
    let mut repeated = Vec::new();
    for d in &dimensions.dimensions {
        let Some(value) = d.value else {
            continue;
        };
        let Some(a) = v
            .components
            .iter()
            .position(|c| c.id == d.anchors[0].component)
        else {
            continue;
        };
        let Some(b) = v
            .components
            .iter()
            .position(|c| c.id == d.anchors[1].component)
        else {
            continue;
        };
        let ca = &v.components[a];
        let cb = &v.components[b];
        if [ca, cb].iter().zip(&d.anchors).any(|(c, anchor)| {
            geometry(c).is_some_and(|g| {
                g.frame_kind != anchor.frame_kind || g.coordinate_space != anchor.space
            })
        }) {
            continue;
        }
        let extent = a == b;
        if d.source_kind == SourceKind::Observed {
            if extent {
                let contains_input = ca
                    .children
                    .iter()
                    .filter_map(|id| v.components.iter().find(|c| &c.id == id))
                    .any(|c| {
                        matches!(
                            component_role(c).to_ascii_lowercase().as_str(),
                            "input" | "searchbox" | "textbox"
                        )
                    });
                if !component_role(ca).eq_ignore_ascii_case("form")
                    && !control(ca)
                    && !contains_input
                {
                    continue;
                }
                if value < 12.0 {
                    continue;
                }
            } else {
                if value <= 0.5 {
                    continue;
                }
                let large_control = |c: &SceneComponent| {
                    control(c)
                        && geometry(c).is_some_and(|g| match &g.shape {
                            Shape::Rect(r) => r.width >= 12.0 && r.height >= 12.0,
                            _ => false,
                        })
                };
                let has_input = |c: &SceneComponent| {
                    c.children
                        .iter()
                        .filter_map(|id| v.components.iter().find(|x| &x.id == id))
                        .any(|x| {
                            matches!(
                                component_role(x).to_ascii_lowercase().as_str(),
                                "input" | "searchbox" | "textbox"
                            )
                        })
                };
                let footer = component_role(cb).eq_ignore_ascii_case("footer")
                    || matches!(&cb.native_role,
                    Some(Availability::Known {value:Value::Text(name)}) if name=="FOOTER");
                let required = match d.label.as_str() {
                    "Vertical edge gap" => {
                        large_control(ca)
                            && (large_control(cb)
                                || has_input(cb)
                                || cb
                                    .children
                                    .iter()
                                    .filter_map(|id| v.components.iter().find(|x| &x.id == id))
                                    .any(control))
                    }
                    "Repeated control clear gap" => matches!(
                        component_role(ca).to_ascii_lowercase().as_str(),
                        "select" | "combobox"
                    ),
                    "Horizontal edge gap" => {
                        matches!(
                            component_role(ca).to_ascii_lowercase().as_str(),
                            "label" | "text"
                        ) && matches!(
                            component_role(cb).to_ascii_lowercase().as_str(),
                            "select" | "combobox"
                        )
                    }
                    "Root left edge inset" => {
                        component_role(ca).eq_ignore_ascii_case("form")
                            && matches!(
                                component_role(cb).to_ascii_lowercase().as_str(),
                                "label" | "text"
                            )
                    }
                    "Root right edge inset" => {
                        component_role(cb).eq_ignore_ascii_case("form")
                            && matches!(
                                component_role(ca).to_ascii_lowercase().as_str(),
                                "select" | "combobox"
                            )
                    }
                    "Measured left inset" => has_input(ca) && large_control(cb),
                    "Measured right inset" => large_control(ca) && footer,
                    "Measured bottom inset" => large_control(ca) && footer,
                    _ => false,
                };
                if !required {
                    continue;
                }
            }
        }
        // One field-width annotation also describes source-equal aligned result controls.
        // This is display selection only; all exact dimensions stay in the machine package.
        if extent
            && d.anchors[0].edge == Edge::Left
            && d.anchors[1].edge == Edge::Right
            && control(ca)
            && v.components[..a].iter().any(|other| {
                matches!(
                    component_role(other).to_ascii_lowercase().as_str(),
                    "input" | "textbox" | "searchbox"
                ) && other.surface == ca.surface
                    && match (geometry(other), geometry(ca)) {
                        (Some(g), Some(h))
                            if g.coordinate_space == h.coordinate_space
                                && g.frame_kind == h.frame_kind =>
                        {
                            match (&g.shape, &h.shape) {
                                (Shape::Rect(r), Shape::Rect(q)) => {
                                    r.x == q.x && r.width == q.width
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    }
            })
        {
            continue;
        }
        let popup_extent = extent
            && ca
                .children
                .iter()
                .filter_map(|id| v.components.iter().find(|c| &c.id == id))
                .any(|c| {
                    matches!(
                        component_role(c).to_ascii_lowercase().as_str(),
                        "input" | "textbox" | "searchbox"
                    )
                });
        // Only exact equal values with the same role/edge pair share a representative.
        // This never merges source components or claims equality from rounded labels.
        let signature = format!(
            "{}|{}|{:?}|{:?}|{:?}|{:?}|{}",
            d.label,
            component_role(ca),
            component_role(cb),
            d.anchors[0].edge,
            d.anchors[1].edge,
            d.units,
            value.to_bits()
        );
        if repeated.contains(&signature) {
            continue;
        }
        repeated.push(signature);
        append(
            &mut out,
            &format!(
                "REQUIRED {}{}: start {} {:?} edge; end {} {:?} edge; label {} {}.\n",
                if popup_extent {
                    "OVERVIEW ONLY — "
                } else {
                    ""
                },
                d.label,
                component_name(ca, a),
                d.anchors[0].edge,
                component_name(cb, b),
                d.anchors[1].edge,
                display_number(value),
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
        vars.insert("scope", v.scope.clone());
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
