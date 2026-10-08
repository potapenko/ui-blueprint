//! Source-owned normalization. No geometry/name/identity inference across sources.
mod components;
use crate::collector::wire::{AxNode, AxValue, DomRead, Scalar, SelectionDirection};
pub(crate) use components::components;
use uiblueprint_schema::model::*;

pub(crate) fn id(value: &str) -> Id {
    Id(value.into())
}
pub(crate) fn unknown(reason: &str) -> Availability {
    Availability::Unknown { reason: id(reason) }
}
fn known(value: Value) -> Availability {
    Availability::Known { value }
}
pub(crate) fn evidence(observation: &Observation, method: &str) -> Evidence {
    Evidence {
        observation_id: observation.id.clone(),
        source_namespace: observation.source_namespace.clone(),
        provenance: Provenance::Reported,
        method: id(method),
        uncertainty: None,
    }
}
fn text(value: Option<&str>, cap: usize) -> Availability {
    match value {
        Some(s) if s.len() <= cap => known(Value::Text(s.into())),
        Some(_) => unknown("source-text-limit"),
        None => unknown("not-exposed-by-source"),
    }
}
fn flag(value: Option<bool>) -> Availability {
    value
        .map(|v| known(Value::Flag(v)))
        .unwrap_or_else(|| unknown("not-exposed-by-source"))
}
fn is_text(field: Field) -> bool {
    matches!(
        field,
        Field::Name
            | Field::AccessibilityName
            | Field::VisibleText
            | Field::Description
            | Field::Value
            | Field::Placeholder
            | Field::InputKind
    )
}
fn property(
    field: Field,
    sensitive: bool,
    observation: &Observation,
    method: &str,
    state: Availability,
) -> Property {
    let redacted = sensitive && is_text(field);
    Property::Requested {
        field,
        sensitivity: if redacted {
            Sensitivity::Sensitive
        } else {
            Sensitivity::Public
        },
        evidence: evidence(observation, method),
        state: if redacted {
            Availability::Redacted {}
        } else {
            state
        },
    }
}
fn viewport_transform(
    rect: &crate::collector::wire::LayoutRect,
    context: &Context,
    observation: &Observation,
) -> TransformState {
    let Some(samples) = &rect.viewport else {
        return TransformState::LocalOnly {};
    };
    let unknown = || TransformState::Unknown {
        reason: id("web-viewport-mapping-unconfirmed"),
    };
    let (Some(before), Some(after)) = (&samples.before, &samples.after) else {
        return unknown();
    };
    if before.values() != after.values() || !before.supported() {
        return unknown();
    }
    let surface = &context.surfaces[0];
    let document_id = format!(
        "document:{}:{}:{}",
        surface.id.0.len(),
        surface.id.0,
        surface.generation.0
    );
    if document_id.chars().count() > 256 {
        return unknown();
    }
    let mut source = evidence(observation, "cssom-scroll-viewport-to-document");
    source.provenance = Provenance::Derived;
    TransformState::Known {
        transform: Box::new(Transform {
            from: Space {
                id: surface.id.clone(),
                kind: SpaceKind::Viewport,
                units: Unit::CssPx,
                origin: Origin::TopLeft,
            },
            to: Space {
                id: Id(document_id),
                kind: SpaceKind::Document,
                units: Unit::CssPx,
                origin: Origin::TopLeft,
            },
            affine: [1.0, 0.0, 0.0, 1.0, before.scroll_x, before.scroll_y],
            target: context.target.clone(),
            surface: surface.clone(),
            environment_revision: context.environment_revision.clone(),
            evidence: source,
        }),
    }
}
pub(crate) fn dom(
    backend: u32,
    read: &DomRead,
    context: &Context,
    observation: &Observation,
    cap: usize,
) -> Node {
    let properties = context
        .fields
        .iter()
        .map(|&field| {
            let state = if read.sensitive && is_text(field) {
                Availability::Redacted {}
            } else {
                match field {
                    Field::LayoutBounds => read
                        .rect
                        .as_ref()
                        .map(|rect| {
                            known(Value::Geometry(Box::new(Geometry {
                                frame_kind: FrameKind::LayoutBounds,
                                coordinate_space: Space {
                                    id: context.surfaces[0].id.clone(),
                                    kind: SpaceKind::Viewport,
                                    units: Unit::CssPx,
                                    origin: Origin::TopLeft,
                                },
                                shape: Shape::Rect(Rect {
                                    x: rect.x,
                                    y: rect.y,
                                    width: rect.width,
                                    height: rect.height,
                                }),
                                transform: viewport_transform(rect, context, observation),
                            })))
                        })
                        .unwrap_or_else(|| unknown("no-layout-box-reported")),
                    Field::HitRegion => read
                        .hit
                        .as_ref()
                        .filter(|hit| hit.matches)
                        .map(|hit| {
                            known(web_geometry(
                                FrameKind::HitRegion,
                                [hit.x, hit.y, 0.0, 0.0],
                                context,
                            ))
                        })
                        .unwrap_or_else(|| {
                            unknown("single-hit-sample-does-not-establish-hit-region")
                        }),
                    Field::VisibleRegion => {
                        unknown("intersection-clipping-is-not-occlusion-or-paint")
                    }
                    Field::Value
                        if read.tag.as_deref() == Some("OUTPUT") && read.value.is_none() =>
                    {
                        unknown("output-value-not-qualified-within-read-bound")
                    }
                    Field::Value => text(read.value.as_deref(), cap),
                    Field::Placeholder => text(read.placeholder.as_deref(), cap),
                    Field::InputKind => text(read.input_kind.as_deref(), cap),
                    Field::Required => flag(read.required),
                    Field::Enabled => flag(read.enabled),
                    Field::Readonly => flag(read.readonly),
                    Field::Checked => flag(read.checked),
                    Field::Selected => flag(read.selected),
                    Field::Expanded => flag(read.expanded),
                    Field::Focused => flag(read.focused),
                    Field::Invalid => flag(read.invalid),
                    Field::AccessibilityName | Field::Name => {
                        unknown("dom-text-is-not-accessible-name")
                    }
                    Field::VisibleText => unknown("visible-text-not-qualified"),
                    Field::Role => unknown("dom-tag-is-not-accessibility-role"),
                    _ => unknown("not-exposed-by-selected-dom-read"),
                }
            };
            property(
                field,
                read.sensitive,
                observation,
                if field == Field::LayoutBounds {
                    "cssom-getBoundingClientRect"
                } else if field == Field::HitRegion {
                    "cssom-elementFromPoint-single-exact-sample"
                } else {
                    "isolated-dom-native-read"
                },
                state,
            )
        })
        .collect();
    Node {
        key: SourceKey {
            namespace: id("web.dom"),
            key: Id(backend.to_string()),
        },
        surface: context.surfaces[0].clone(),
        native_role: if read.sensitive {
            Availability::Redacted {}
        } else {
            text(read.tag.as_deref(), cap)
        },
        properties,
        children: vec![],
        extensions: geometry_facts(read, context, observation),
        source_declarations: vec![],
    }
}
// Native Web facts are separate from canonical visible/paint area. The existing
// extension envelope preserves their typed geometry and per-fact provenance.
fn web_geometry(kind: FrameKind, rect: [f64; 4], context: &Context) -> Value {
    Value::Geometry(Box::new(Geometry {
        frame_kind: kind,
        coordinate_space: Space {
            id: context.surfaces[0].id.clone(),
            kind: SpaceKind::Viewport,
            units: Unit::CssPx,
            origin: Origin::TopLeft,
        },
        shape: Shape::Rect(Rect {
            x: rect[0],
            y: rect[1],
            width: rect[2],
            height: rect[3],
        }),
        // No transform sampled concurrently with this asynchronous browser fact.
        transform: TransformState::LocalOnly {},
    }))
}
fn geometry_facts(
    read: &DomRead,
    context: &Context,
    observation: &Observation,
) -> Vec<ExtensionProperty> {
    let mut result = Vec::new();
    let mut add = |name: &str, field, value, method| {
        result.push(ExtensionProperty {
            namespace: id("web.dom"),
            name: id(name),
            property: property(field, false, observation, method, known(value)),
        })
    };
    if context.fields.contains(&Field::HitRegion)
        && let Some(hit) = &read.hit
    {
        add(
            "hit_sample_point",
            Field::HitRegion,
            web_geometry(FrameKind::HitRegion, [hit.x, hit.y, 0.0, 0.0], context),
            "cssom-elementFromPoint-sample-location",
        );
        add(
            "hit_sample_matches",
            Field::Value,
            Value::Flag(hit.matches),
            "cssom-elementFromPoint-single-exact-sample",
        );
    }
    if context.fields.contains(&Field::VisibleRegion)
        && let Some(clip) = &read.clip
    {
        add(
            "intersection_rect_not_occlusion",
            Field::VisibleRegion,
            web_geometry(FrameKind::VisibleRegion, clip.rect.values(), context),
            "intersection-observer-single-target-zero-margin",
        );
        add(
            "intersection_ratio_not_visibility",
            Field::Value,
            Value::Number(clip.ratio),
            "intersection-observer-single-target-zero-margin",
        );
        add(
            "is_intersecting_not_visible",
            Field::Value,
            Value::Flag(clip.intersects),
            "intersection-observer-single-target-zero-margin",
        );
    }
    result
}
fn ax_text(value: Option<&AxValue>) -> Option<&str> {
    let v = value?;
    match (&*v.r#type, v.value.as_ref()?) {
        ("string" | "computedString" | "token" | "role" | "internalRole", Scalar::Text(s)) => {
            Some(s)
        }
        _ => None,
    }
}
fn ax_bool(value: Option<&AxValue>) -> Option<bool> {
    let v = value?;
    match (&*v.r#type, v.value.as_ref()?) {
        ("boolean" | "booleanOrUndefined", Scalar::Flag(b)) => Some(*b),
        ("tristate" | "token", Scalar::Text(s)) if s == "true" => Some(true),
        ("tristate" | "token", Scalar::Text(s)) if s == "false" => Some(false),
        _ => None,
    }
}
fn role(value: &str) -> Option<Role> {
    Some(match value {
        "StaticText" | "text" => Role::Text,
        "heading" => Role::Heading,
        "textbox" => Role::Textbox,
        "searchbox" => Role::Searchbox,
        "button" => Role::Button,
        "link" => Role::Link,
        "checkbox" => Role::Checkbox,
        "radio" => Role::Radio,
        "switch" => Role::Switch,
        "combobox" => Role::Combobox,
        "option" => Role::Option,
        "list" => Role::List,
        "listitem" => Role::Listitem,
        "menu" => Role::Menu,
        "menuitem" => Role::Menuitem,
        "tab" => Role::Tab,
        "dialog" => Role::Dialog,
        "form" => Role::Form,
        "group" => Role::Group,
        "slider" => Role::Slider,
        "image" => Role::Image,
        _ => return None,
    })
}
pub(crate) fn ax(read: &AxNode, context: &Context, observation: &Observation, cap: usize) -> Node {
    let raw_role = ax_text(read.role.as_ref());
    let properties = context
        .fields
        .iter()
        .map(|&field| {
            let property_name = match field {
                Field::Required => Some("required"),
                Field::Enabled => Some("disabled"),
                Field::Readonly => Some("readonly"),
                Field::Checked => Some("checked"),
                Field::Selected => Some("selected"),
                Field::Expanded => Some("expanded"),
                Field::Focused => Some("focused"),
                Field::Invalid => Some("invalid"),
                _ => None,
            };
            let source = property_name
                .and_then(|name| read.properties.as_ref()?.iter().find(|p| p.name == name))
                .map(|p| &p.value);
            let state = if read.ignored {
                unknown("source-ax-node-ignored")
            } else {
                match field {
                    Field::Role => raw_role
                        .and_then(role)
                        .map(|r| known(Value::Role(r)))
                        .unwrap_or_else(|| unknown("ax-role-not-mapped")),
                    Field::Name | Field::AccessibilityName => {
                        text(ax_text(read.name.as_ref()), cap)
                    }
                    Field::Description => text(ax_text(read.description.as_ref()), cap),
                    Field::Value => match read.value.as_ref() {
                        Some(AxValue {
                            r#type: kind,
                            value: Some(Scalar::Number(n)),
                        }) if (kind == "number" || kind == "integer") && n.is_finite() => {
                            known(Value::Number(*n))
                        }
                        Some(AxValue {
                            r#type: kind,
                            value: Some(Scalar::Flag(b)),
                        }) if kind == "boolean" => known(Value::Flag(*b)),
                        _ => text(ax_text(read.value.as_ref()), cap),
                    },
                    Field::Enabled => flag(ax_bool(source).map(|disabled| !disabled)),
                    _ if property_name.is_some() => flag(ax_bool(source)),
                    _ => unknown("not-exposed-by-addressed-ax"),
                }
            };
            property(field, false, observation, "cdp-addressed-partial-ax", state)
        })
        .collect();
    Node {
        key: SourceKey {
            namespace: id("web.ax"),
            key: Id(read.node_id.clone()),
        },
        surface: context.surfaces[0].clone(),
        native_role: text(raw_role, cap),
        properties,
        children: vec![],
        extensions: if context.fields.contains(&Field::Focused) {
            vec![ExtensionProperty {
                namespace: id("web.ax"),
                name: id("focusable"),
                property: property(
                    Field::Value,
                    false,
                    observation,
                    "cdp-addressed-partial-ax-focusable",
                    flag(read.properties.as_ref().and_then(|properties| {
                        ax_bool(
                            properties
                                .iter()
                                .find(|p| p.name == "focusable")
                                .map(|p| &p.value),
                        )
                    })),
                ),
            }]
        } else {
            vec![]
        },
        source_declarations: vec![],
    }
}

pub(crate) fn coverage(context: &Context, empty: bool) -> Coverage {
    Coverage {
        status: if empty {
            CoverageStatus::Complete
        } else {
            CoverageStatus::Partial
        },
        scope_id: context.scope_id.clone(),
        fields: context.fields.clone(),
        omitted_count: if empty { Some(0) } else { None },
        unknown_count: if empty { Some(0) } else { None },
    }
}
pub(crate) fn observation(
    context: &Context,
    clock: &Id,
    namespace: &str,
    stamp: (u64, u64),
    interval: [f64; 2],
    current: bool,
    empty: bool,
) -> Observation {
    Observation {
        id: Id(format!("{namespace}:{}:{}", stamp.0, stamp.1)),
        source_namespace: id(namespace),
        channel: Channel::ExternalSemantics,
        start: interval[0],
        end: interval[1],
        clock_domain: clock.clone(),
        time_unit: TimeUnit::Milliseconds,
        freshness_basis: if current {
            FreshnessBasis::LiveRead
        } else {
            FreshnessBasis::Unverified
        },
        consistency_reason: Some(id("sequential-reads-not-atomic")),
        answer_source: AnswerSource::Live,
        freshness: if current {
            Freshness::Current
        } else {
            Freshness::Unverified
        },
        last_verified: current.then_some(interval[1]),
        consistency: Consistency::Unknown,
        coverage: coverage(context, empty),
    }
}
pub(crate) fn snapshot(
    request: &Request,
    stamp: (u64, u64),
    observations: Vec<Observation>,
    nodes: Vec<Node>,
    relations: Vec<Relation>,
    empty: bool,
) -> Snapshot {
    let first = &observations[0];
    let mut coverage = coverage(&request.context, empty);
    coverage.unknown_count = Some(
        nodes
            .iter()
            .flat_map(|n| &n.properties)
            .filter(|p| {
                matches!(
                    p,
                    Property::Requested {
                        state: Availability::Unknown { .. },
                        ..
                    }
                )
            })
            .count() as u64,
    );
    Snapshot {
        surface_records: vec![SurfaceRecord {
            identity: request.context.surfaces[0].clone(),
            native_owner: known(Value::Identity(request.context.target.clone())),
            initiated_by: None,
            anchor: None,
            evidence: evidence(first, "cdp-target-frame-document-binding"),
        }],
        id: Id(format!("web-snapshot:{}:{}", stamp.0, stamp.1)),
        revision: stamp.1,
        source_state: None,
        context: request.context.clone(),
        observations,
        nodes,
        relations,
        components: vec![],
        focus: Focus {
            keyboard: if request.context.fields.contains(&Field::Focused) {
                FocusRef::Unknown {
                    reason: id("scope-does-not-establish-global-focus-owner"),
                }
            } else {
                FocusRef::NotRequested {}
            },
            accessibility: FocusRef::NotRequested {},
            active_descendant: FocusRef::NotRequested {},
            text_selection: None,
            composition_state: Property::NotRequested {
                field: Field::Value,
            },
        },
        captures: vec![],
        coverage,
    }
}

/// Endpoints are indices into the same request's original selected DOM objects.
pub(crate) fn dom_relations(
    records: &[(u32, DomRead)],
    observation: &Observation,
    result: &mut Vec<Relation>,
) {
    for (backend, read) in records {
        if read.sensitive {
            continue;
        }
        let controls = read
            .controls
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .copied()
            .map(|index| (index, RelationKind::Controls, "dom-aria-controls"));
        let anchor = read.declared_anchor.into_iter().map(|index| {
            (
                index,
                RelationKind::AnchoredTo,
                "fixture-data-anchor-attribute",
            )
        });
        for (index, kind, method) in controls.chain(anchor) {
            if let Some((target, _)) = records.get(index).filter(|(_, r)| !r.sensitive) {
                result.push(Relation {
                    kind,
                    from: dom_key(*backend),
                    to: dom_key(*target),
                    evidence: evidence(observation, method),
                });
            }
        }
    }
}
fn dom_key(backend: u32) -> SourceKey {
    SourceKey {
        namespace: id("web.dom"),
        key: Id(backend.to_string()),
    }
}
/// A native text control can own keyboard focus even without a known selection.
pub(crate) fn keyboard_focus(
    records: &[(u32, DomRead)],
    observation: &Observation,
    context: &Context,
) -> FocusRef {
    if !context.fields.contains(&Field::Focused) {
        return FocusRef::NotRequested {};
    }
    let mut focused = records.iter().filter(|(_, r)| r.focused == Some(true));
    if let Some((backend, read)) = focused.next()
        && focused.next().is_none()
        && !read.sensitive
        && read.connected
        && read.same_document
        && read.document_focused == Some(true)
        && matches!(read.tag.as_deref(), Some("INPUT" | "TEXTAREA"))
    {
        return FocusRef::Known {
            target: dom_key(*backend),
            evidence: evidence(observation, "dom-active-element-document-has-focus"),
        };
    }
    FocusRef::Unknown {
        reason: id("scope-does-not-establish-global-focus-owner"),
    }
}
/// The optional canonical selection has no target of its own: publish it only
/// together with the unique, source-confirmed keyboard focus owner in this scope.
pub(crate) fn text_selection(
    records: &[(u32, DomRead)],
    observation: &Observation,
    context: &Context,
    cap: usize,
) -> Option<(FocusRef, TextSelection)> {
    if !context.fields.contains(&Field::Focused) || !context.fields.contains(&Field::Value) {
        return None;
    }
    if !matches!(
        keyboard_focus(records, observation, context),
        FocusRef::Known { .. }
    ) {
        return None;
    }
    let mut focused = records.iter().filter(|(_, r)| r.focused == Some(true));
    let (backend, read) = focused.next()?;
    if focused.next().is_some()
        || read.sensitive
        || !read.connected
        || !read.same_document
        || !matches!(read.tag.as_deref(), Some("INPUT" | "TEXTAREA"))
    {
        return None;
    }
    let value = read.value.as_ref().filter(|value| value.len() <= cap)?;
    let selection = read.selection.as_ref()?;
    if !selection.document_focused
        || selection.start > selection.end
        || selection.end > value.encode_utf16().count() as u64
    {
        return None;
    }
    let (anchor, focus) = match selection.direction {
        SelectionDirection::Backward => (selection.end, selection.start),
        SelectionDirection::Forward => (selection.start, selection.end),
        SelectionDirection::None if selection.start == selection.end => {
            (selection.start, selection.end)
        }
        _ => return None,
    };
    Some((
        FocusRef::Known {
            target: dom_key(*backend),
            evidence: evidence(observation, "dom-active-element-document-has-focus"),
        },
        TextSelection {
            anchor,
            focus,
            units: id("utf16_code_units"),
            evidence: evidence(observation, "dom-native-text-control-selection"),
        },
    ))
}
pub(crate) fn active_descendant(
    records: &[(u32, DomRead)],
    observation: &Observation,
    context: &Context,
) -> FocusRef {
    if !context.fields.contains(&Field::Focused) {
        return FocusRef::NotRequested {};
    }
    let mut focused = records.iter().filter(|(_, r)| r.focused == Some(true));
    if let Some((_, read)) = focused.next()
        && focused.next().is_none()
        && !read.sensitive
        && let Some((backend, _)) = read
            .active_descendant
            .and_then(|i| records.get(i))
            .filter(|(_, r)| !r.sensitive)
    {
        return FocusRef::Known {
            target: dom_key(*backend),
            evidence: evidence(observation, "dom-focused-aria-activedescendant"),
        };
    }
    FocusRef::Unknown {
        reason: id("selected-scope-has-no-confirmed-active-descendant"),
    }
}
