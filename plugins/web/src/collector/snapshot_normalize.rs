use super::{snapshot_wire as raw, *};
use crate::normalize;
use uiblueprint_schema::validation;

fn malformed() -> Failure {
    Failure::new(ErrorKind::Malformed)
}
fn key(backend: u32) -> SourceKey {
    SourceKey {
        namespace: normalize::id("web.dom"),
        key: Id(backend.to_string()),
    }
}
fn known(value: Value) -> Availability {
    Availability::Known { value }
}
fn prop(field: Field, value: Availability, sensitive: bool, observation: &Observation) -> Property {
    Property::Requested {
        field,
        sensitivity: if sensitive {
            Sensitivity::Sensitive
        } else {
            Sensitivity::Public
        },
        evidence: normalize::evidence(observation, "cdp-dom-snapshot"),
        state: if sensitive {
            Availability::Redacted {}
        } else {
            value
        },
    }
}
fn ext(
    node: &mut Node,
    name: impl Into<String>,
    value: Availability,
    sensitive: bool,
    observation: &Observation,
) {
    let field = if matches!(
        &value,
        Availability::Known {
            value: Value::Geometry(_)
        }
    ) {
        Field::LayoutBounds
    } else {
        Field::Value
    };
    node.extensions.push(ExtensionProperty {
        namespace: normalize::id("web.dom_snapshot"),
        name: Id(name.into()),
        property: prop(field, value, sensitive, observation),
    });
}
fn text(strings: &[String], index: usize, cap: usize) -> Result<Value, Failure> {
    let value = strings.get(index).ok_or_else(malformed)?;
    if value.len() > cap {
        return Err(Failure::new(ErrorKind::Limit));
    }
    Ok(Value::Text(value.clone()))
}
fn rectangle(rect: &[f64], surface: &Identity, local: bool) -> Result<Availability, Failure> {
    if rect.is_empty() {
        return Ok(normalize::unknown("no-native-rect"));
    }
    if rect.len() != 4 || !rect.iter().all(|v| v.is_finite()) || rect[2] < 0.0 || rect[3] < 0.0 {
        return Err(malformed());
    }
    Ok(known(Value::Geometry(Box::new(Geometry {
        frame_kind: FrameKind::LayoutBounds,
        coordinate_space: Space {
            id: surface.id.clone(),
            kind: if local {
                SpaceKind::Local
            } else {
                SpaceKind::Document
            },
            units: Unit::CssPx,
            origin: Origin::TopLeft,
        },
        shape: Shape::Rect(Rect {
            x: rect[0],
            y: rect[1],
            width: rect[2],
            height: rect[3],
        }),
        transform: TransformState::Unknown {
            reason: normalize::id("native-rect-cross-space-transform-unconfirmed"),
        },
    }))))
}
fn indices(indices: &[usize], count: usize) -> Result<(), Failure> {
    if indices.len() > count
        || indices
            .iter()
            .enumerate()
            .any(|(i, n)| *n >= count || indices[..i].contains(n))
    {
        return Err(malformed());
    }
    Ok(())
}
fn rare(value: &raw::Rare, count: usize) -> Result<(), Failure> {
    if value.index.len() != value.value.len() {
        return Err(malformed());
    }
    indices(&value.index, count)
}
fn validate(d: &raw::Document, cap: usize) -> Result<(), Failure> {
    let n = &d.nodes;
    let count = n.backend_node_id.len();
    if count == 0
        || count > cap
        || [
            n.parent_index.len(),
            n.node_type.len(),
            n.node_name.len(),
            n.node_value.len(),
            n.attributes.len(),
        ]
        .iter()
        .any(|len| *len != count)
        || n.parent_index[0] != -1
        || n.node_type[0] != 9
        || n.parent_index
            .iter()
            .enumerate()
            .skip(1)
            .any(|(i, p)| *p < 0 || *p as usize >= i)
        || n.backend_node_id
            .iter()
            .any(|n| *n == 0 || *n > i32::MAX as u32)
        || n.attributes.iter().any(|attrs| attrs.len() % 2 != 0)
    {
        return Err(malformed());
    }
    for r in [
        &n.shadow_root_type,
        &n.text_value,
        &n.input_value,
        &n.content_document_index,
        &n.pseudo_type,
        &n.pseudo_identifier,
        &n.current_source_url,
        &n.origin_url,
    ] {
        rare(r, count)?;
    }
    for f in [&n.input_checked, &n.option_selected, &n.is_clickable] {
        indices(&f.index, count)?;
    }
    if !n.shadow_root_type.index.is_empty() || !n.pseudo_type.index.is_empty() {
        return Err(Failure::new(ErrorKind::Selection {
            status: SelectionStatus::Unsupported,
            visited_nodes: count as u32,
        }));
    }
    let l = &d.layout;
    let size = l.node_index.len();
    indices(&l.node_index, count)?;
    if [
        l.styles.len(),
        l.bounds.len(),
        l.text.len(),
        l.offset_rects.len(),
        l.scroll_rects.len(),
        l.client_rects.len(),
    ]
    .iter()
    .any(|v| *v != size)
        || l.styles.iter().any(|v| !v.is_empty())
    {
        return Err(malformed());
    }
    indices(&l.stacking_contexts.index, size)?;
    let t = &d.text_boxes;
    let boxes = t.layout_index.len();
    if boxes > cap
        || [t.bounds.len(), t.start.len(), t.length.len()]
            .iter()
            .any(|v| *v != boxes)
        || t.layout_index.iter().any(|v| *v >= size)
    {
        return Err(Failure::new(ErrorKind::Limit));
    }
    Ok(())
}
fn private_node(attrs: &[usize], strings: &[String]) -> Result<bool, Failure> {
    for pair in attrs.chunks_exact(2) {
        let name = strings
            .get(pair[0])
            .ok_or_else(malformed)?
            .to_ascii_lowercase();
        let value = strings
            .get(pair[1])
            .ok_or_else(malformed)?
            .to_ascii_lowercase();
        if (name == "type" && value == "password")
            || name == "data-private"
            || name == "data-sensitive"
            || name.contains("token")
            || name.contains("secret")
            || (name == "autocomplete"
                && value.split_whitespace().any(|v| {
                    matches!(
                        v,
                        "current-password"
                            | "new-password"
                            | "one-time-code"
                            | "cc-number"
                            | "cc-csc"
                    )
                }))
        {
            return Ok(true);
        }
    }
    Ok(false)
}

pub(super) fn snapshot(
    capture: raw::Capture,
    scope: &DocumentsScope,
    request: &Request,
    counts: &[usize],
    clock: &Id,
    stamp: (u64, u64),
    interval: [f64; 2],
    limits: Limits,
) -> Result<Snapshot, Failure> {
    if capture.documents.len() != scope.documents.len() {
        return Err(Failure::new(ErrorKind::StaleTarget));
    }
    let observation = normalize::observation(
        &request.context,
        clock,
        "web.dom",
        stamp,
        interval,
        true,
        false,
    );
    let mut nodes = Vec::new();
    let mut roots = Vec::new();
    let mut relations = Vec::new();
    let mut surfaces = Vec::new();
    let mut seen = Vec::new();
    for document in &capture.documents {
        let frame = capture
            .strings
            .get(document.frame_id)
            .ok_or_else(malformed)?;
        let index = scope
            .documents
            .iter()
            .position(|d| &d.surface.id.0 == frame)
            .ok_or(Failure::new(ErrorKind::StaleTarget))?;
        if seen.contains(&index) {
            return Err(malformed());
        }
        seen.push(index);
        let seed = &scope.documents[index];
        validate(document, limits.max_nodes)?;
        if document.nodes.backend_node_id.len() != counts[index]
            || document.nodes.backend_node_id[0] != seed.document_backend_id
        {
            return Err(Failure::new(ErrorKind::ResyncRequired));
        }
        roots.push(key(seed.document_backend_id));
        let offset = nodes.len();
        normalize_document(
            document,
            &capture.strings,
            seed,
            request,
            &observation,
            limits,
            &mut nodes,
        )?;
        let root = &mut nodes[offset];
        if request.context.fields.contains(&Field::Value) {
            for (name, idx) in [
                ("documentURL", document.document_url),
                ("title", document.title),
                ("baseURL", document.base_url),
                ("contentLanguage", document.content_language),
                ("encodingName", document.encoding_name),
                ("publicId", document.public_id),
                ("systemId", document.system_id),
            ] {
                ext(
                    root,
                    name,
                    known(text(&capture.strings, idx, limits.max_text_bytes)?),
                    seed.sensitivity == Sensitivity::Sensitive,
                    &observation,
                );
            }
        }
        if request.context.fields.contains(&Field::LayoutBounds) {
            for (name, value) in [
                ("scrollOffsetX", document.scroll_offset_x),
                ("scrollOffsetY", document.scroll_offset_y),
                ("contentWidth", document.content_width),
                ("contentHeight", document.content_height),
            ] {
                ext(
                    root,
                    name,
                    value
                        .map(|v| known(Value::Number(v)))
                        .unwrap_or_else(|| normalize::unknown("not-exposed-by-source")),
                    false,
                    &observation,
                );
            }
        }
        surfaces.push(SurfaceRecord {
            identity: seed.surface.clone(),
            native_owner: known(Value::Identity(request.context.target.clone())),
            initiated_by: None,
            anchor: None,
            evidence: normalize::evidence(&observation, "cdp-frame-loader-document-binding"),
        });
    }
    for document in &capture.documents {
        for (i, child) in document
            .nodes
            .content_document_index
            .index
            .iter()
            .zip(&document.nodes.content_document_index.value)
        {
            relations.push(Relation {
                kind: RelationKind::Owns,
                from: key(document.nodes.backend_node_id[*i]),
                to: roots.get(*child).ok_or_else(malformed)?.clone(),
                evidence: normalize::evidence(&observation, "cdp-contentDocumentIndex"),
            });
        }
    }
    if nodes.len() > scope.max_visited_nodes as usize {
        return Err(Failure::new(ErrorKind::Limit));
    }
    let mut result =
        normalize::snapshot(request, stamp, vec![observation], nodes, relations, false);
    result.surface_records = surfaces;
    result.coverage.omitted_count = Some(0);
    result.observations[0].coverage.omitted_count = Some(0);
    validation::validate_snapshot(&result).map_err(|_| malformed())?;
    Ok(result)
}

fn normalize_document(
    d: &raw::Document,
    strings: &[String],
    seed: &DocumentSeed,
    request: &Request,
    observation: &Observation,
    limits: Limits,
    nodes: &mut Vec<Node>,
) -> Result<(), Failure> {
    let n = &d.nodes;
    let offset = nodes.len();
    let values = request.context.fields.contains(&Field::Value);
    let layout = request.context.fields.contains(&Field::LayoutBounds);
    let mut private = Vec::new();
    for i in 0..n.backend_node_id.len() {
        let sensitive = seed.sensitivity == Sensitivity::Sensitive
            || private_node(&n.attributes[i], strings)?
            || (i > 0 && private[n.parent_index[i] as usize]);
        private.push(sensitive);
        // Whole capture cannot safely isolate srcdoc/inline-source aliases if
        // a private node appeared after preflight. Publish no channel in that case.
        if sensitive {
            return Err(Failure::new(ErrorKind::InvalidInput));
        }
        let layout_index = d.layout.node_index.iter().position(|idx| *idx == i);
        let mut node = Node {
            key: key(n.backend_node_id[i]),
            surface: seed.surface.clone(),
            native_role: known(text(strings, n.node_name[i], limits.max_text_bytes)?),
            properties: Vec::new(),
            children: Vec::new(),
            extensions: Vec::new(),
            source_declarations: Vec::new(),
        };
        for field in &request.context.fields {
            let state = match field {
                Field::Value => {
                    if sensitive {
                        Availability::Redacted {}
                    } else {
                        known(text(strings, n.node_value[i], limits.max_text_bytes)?)
                    }
                }
                Field::LayoutBounds => match layout_index {
                    Some(l) => rectangle(&d.layout.bounds[l], &seed.surface, false)?,
                    None => normalize::unknown("no-layout-object"),
                },
                _ => return Err(Failure::new(ErrorKind::InvalidInput)),
            };
            node.properties.push(prop(
                *field,
                state,
                sensitive && *field == Field::Value,
                observation,
            ));
        }
        if values {
            ext(
                &mut node,
                "nodeType",
                known(Value::Number(f64::from(n.node_type[i]))),
                false,
                observation,
            );
            // Preserve attribute names only on public nodes; private names may themselves contain data.
            if sensitive {
                ext(
                    &mut node,
                    "attributes",
                    Availability::Redacted {},
                    true,
                    observation,
                );
            } else {
                for (j, pair) in n.attributes[i].chunks_exact(2).enumerate() {
                    ext(
                        &mut node,
                        format!("attribute.{j}.name"),
                        known(text(strings, pair[0], limits.max_text_bytes)?),
                        false,
                        observation,
                    );
                    ext(
                        &mut node,
                        format!("attribute.{j}.value"),
                        known(text(strings, pair[1], limits.max_text_bytes)?),
                        false,
                        observation,
                    );
                }
            }
            for (name, r) in [
                ("textValue", &n.text_value),
                ("inputValue", &n.input_value),
                ("shadowRootType", &n.shadow_root_type),
                ("pseudoType", &n.pseudo_type),
                ("pseudoIdentifier", &n.pseudo_identifier),
                ("currentSourceURL", &n.current_source_url),
                ("originURL", &n.origin_url),
            ] {
                if let Some(pos) = r.index.iter().position(|idx| *idx == i) {
                    ext(
                        &mut node,
                        name,
                        if sensitive {
                            Availability::Redacted {}
                        } else {
                            known(text(strings, r.value[pos], limits.max_text_bytes)?)
                        },
                        sensitive,
                        observation,
                    );
                }
            }
            for (name, flags) in [
                ("inputChecked", &n.input_checked),
                ("optionSelected", &n.option_selected),
                ("isClickable", &n.is_clickable),
            ] {
                // RareBooleanData absence means false in this native table (not AX availability).
                ext(
                    &mut node,
                    name,
                    known(Value::Flag(flags.index.contains(&i))),
                    false,
                    observation,
                );
            }
        }
        if layout {
            if let Some(l) = layout_index {
                ext(
                    &mut node,
                    "layout.text",
                    if sensitive {
                        Availability::Redacted {}
                    } else {
                        known(text(strings, d.layout.text[l], limits.max_text_bytes)?)
                    },
                    sensitive,
                    observation,
                );
                ext(
                    &mut node,
                    "layout.stackingContext",
                    known(Value::Flag(d.layout.stacking_contexts.index.contains(&l))),
                    false,
                    observation,
                );
                for (name, rect) in [
                    ("layout.offsetRect", &d.layout.offset_rects[l]),
                    ("layout.scrollRect", &d.layout.scroll_rects[l]),
                    ("layout.clientRect", &d.layout.client_rects[l]),
                ] {
                    ext(
                        &mut node,
                        name,
                        rectangle(rect, &seed.surface, true)?,
                        false,
                        observation,
                    );
                }
                let mut ordinal = 0;
                for (b, _) in d
                    .text_boxes
                    .layout_index
                    .iter()
                    .enumerate()
                    .filter(|(_, idx)| **idx == l)
                {
                    ext(
                        &mut node,
                        format!("textBox.{ordinal}.bounds"),
                        rectangle(&d.text_boxes.bounds[b], &seed.surface, false)?,
                        false,
                        observation,
                    );
                    for (name, value) in [
                        ("startUtf16", d.text_boxes.start[b]),
                        ("lengthUtf16", d.text_boxes.length[b]),
                    ] {
                        ext(
                            &mut node,
                            format!("textBox.{ordinal}.{name}"),
                            known(Value::Number(f64::from(value))),
                            sensitive,
                            observation,
                        );
                    }
                    ordinal += 1;
                }
            }
        }
        nodes.push(node);
        if i > 0 {
            nodes[offset + n.parent_index[i] as usize]
                .children
                .push(key(n.backend_node_id[i]));
        }
    }
    Ok(())
}
