use crate::{Capture, Error};
use std::io::{self, Write};
use uiblueprint_schema::{SchemaVersion, model::*};

const NAMESPACE: &str = "gpui.layout";
fn id(value: impl Into<String>) -> Id {
    Id(value.into())
}
fn key(sample: &crate::session::Sample) -> SourceKey {
    SourceKey {
        namespace: id(NAMESPACE),
        key: id(format!("{}-{}", sample.id, sample.mount)),
    }
}
fn declaration(name: &str, value: Value) -> SourceDeclaration {
    SourceDeclaration {
        namespace: id(NAMESPACE),
        name: id(name),
        state: Availability::Known { value },
        sensitivity: Sensitivity::Public,
        source: id("explicit-gpui-probe"),
    }
}
fn text(name: &str, value: &str) -> SourceDeclaration {
    declaration(name, Value::Text(value.into()))
}

pub(crate) fn encode(c: &Capture) -> Result<Vec<u8>, Error> {
    let scope = c
        .request
        .component
        .as_ref()
        .map_or_else(|| c.config.scope.clone(), |id| format!("component-{id}"));
    let fields = vec![
        Field::LayoutBounds,
        Field::AccessibilityBounds,
        Field::HitRegion,
        Field::VisibleRegion,
        Field::PaintBounds,
    ];
    let surface = Identity {
        id: id(c.config.scope.clone()),
        generation: id(format!("{}-{}-{}", c.config.run, c.instance, c.surface)),
    };
    let observation_id = id(format!("gpui-{}", c.sequence));
    let coverage = Coverage {
        status: CoverageStatus::Partial,
        scope_id: id(scope.clone()),
        fields: fields.clone(),
        omitted_count: None,
        unknown_count: Some((c.samples.len() * 4) as u64),
    };
    let evidence = Evidence {
        observation_id: observation_id.clone(),
        source_namespace: id(NAMESPACE),
        provenance: Provenance::Reported,
        method: id("GPUI.Element.prepaint.explicit-capture"),
        uncertainty: None,
    };
    let nodes = c
        .samples
        .iter()
        .map(|sample| {
            let bounds = sample.bounds;
            let geometry = Geometry {
                frame_kind: FrameKind::LayoutBounds,
                coordinate_space: Space {
                    id: id("gpui-window-logical-px"),
                    kind: SpaceKind::Window,
                    units: Unit::Px,
                    origin: Origin::TopLeft,
                },
                shape: Shape::Rect(Rect {
                    x: f32::from(bounds.origin.x).into(),
                    y: f32::from(bounds.origin.y).into(),
                    width: f32::from(bounds.size.width).into(),
                    height: f32::from(bounds.size.height).into(),
                }),
                transform: TransformState::Unknown {
                    reason: id("native-image-transform-not-established"),
                },
            };
            let properties = fields
                .iter()
                .map(|field| Property::Requested {
                    field: *field,
                    sensitivity: Sensitivity::Public,
                    evidence: evidence.clone(),
                    state: if *field == Field::LayoutBounds {
                        Availability::Known {
                            value: Value::Geometry(Box::new(geometry.clone())),
                        }
                    } else {
                        Availability::Unknown {
                            reason: id("not-exposed-by-layout-probe"),
                        }
                    },
                })
                .collect();
            let mut declarations = vec![
                text("marker_id", &sample.id),
                text("mount_id", &sample.mount.to_string()),
                text("request_id", &c.request.id),
                text("state_id", &c.request.state),
                text("environment", "instrumented_debug"),
                text(
                    "parent_mapping",
                    if c.config.window_root {
                        "native-id-ancestry-or-window-host"
                    } else {
                        "declared-and-checked-marked-ancestry"
                    },
                ),
                text("source_units", "GPUI.Pixels-logical-not-DevicePixels"),
            ];
            if let Some(parent) = &sample.parent {
                declarations.push(text("parent_marker_id", &parent.id));
            }
            if c.samples
                .first()
                .is_some_and(|first| first.mount == sample.mount)
            {
                declarations.push(text(
                    "selection",
                    if c.request.component.is_some() {
                        "component-subtree"
                    } else if c.config.window_root {
                        "window-root-all-marked"
                    } else {
                        "explicit-subset"
                    },
                ));
                declarations.push(text("unmarked_coverage", "unknown"));
                declarations.push(declaration(
                    "returned_marker_occurrences",
                    Value::Number(c.samples.len() as f64),
                ));
                if c.config.window_root {
                    declarations.push(text(
                        "root_binding",
                        "actual-gpui-base-Root-and-WindowHandle",
                    ));
                    declarations.push(text(
                        "excluded_sources",
                        "native-chrome;unmarked-internals;other-windows",
                    ));
                }
                declarations.extend([
                    text("application_id", &c.config.app),
                    text("build_id", &c.config.build),
                    text("gpui_window_id", &c.window_id),
                    text("layout_pass", &c.frame.to_string()),
                    declaration("process_id", Value::Number(std::process::id().into())),
                    declaration("display_scale", Value::Number(c.scale)),
                    declaration("rem_size", Value::Number(c.rem_size)),
                    declaration("viewport_width", Value::Number(c.viewport[0])),
                    declaration("viewport_height", Value::Number(c.viewport[1])),
                ]);
            }
            Node {
                key: key(sample),
                surface: surface.clone(),
                native_role: Availability::Unsupported {
                    reason: id("layout-marker-not-accessibility"),
                },
                properties,
                children: c
                    .samples
                    .iter()
                    .filter(|s| s.parent.as_ref().is_some_and(|p| p.mount == sample.mount))
                    .map(key)
                    .collect(),
                extensions: Vec::new(),
                source_declarations: declarations,
            }
        })
        .collect();
    let document = Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Snapshot(Box::new(Snapshot {
            surface_records: Vec::new(),
            id: id(format!(
                "gpui-{}-{}-{}",
                c.config.run, c.instance, c.sequence
            )),
            revision: c.sequence,
            source_state: Some(id(c.request.state.clone())),
            context: Context {
                schema_version: SchemaVersion::CURRENT,
                session_id: id(format!("{}-{}", c.config.run, c.instance)),
                target: Identity {
                    id: id(c.config.app.clone()),
                    generation: id(c.config.run.clone()),
                },
                surfaces: vec![surface],
                scope_id: id(scope.clone()),
                projection: Projection::Design,
                fields: fields.clone(),
                plugin: PluginIdentity {
                    id: id("gpui-probe"),
                    version: id("1.0.0"),
                },
                environment_revision: id(format!("gpui-{}-{}-{}", c.instance, c.surface, c.frame)),
            },
            observations: vec![Observation {
                id: observation_id,
                source_namespace: id(NAMESPACE),
                channel: Channel::OptInLayoutProbe,
                start: c.start,
                end: c.end,
                clock_domain: id(format!("gpui-{}-{}", c.config.run, c.instance)),
                time_unit: TimeUnit::Seconds,
                freshness_basis: FreshnessBasis::LiveRead,
                answer_source: AnswerSource::Cache,
                freshness: Freshness::Unverified,
                last_verified: None,
                consistency: Consistency::Unknown,
                consistency_reason: Some(id("layout-pass-not-presented-frame")),
                coverage: coverage.clone(),
            }],
            nodes,
            relations: Vec::new(),
            components: vec![ComponentMapping {
                logical_component_key: id(scope.clone()),
                members: c.samples.iter().map(key).collect(),
                declaration_source: id("explicit-gpui-markers"),
                provenance: Provenance::Reported,
            }],
            focus: Focus {
                keyboard: FocusRef::NotRequested {},
                accessibility: FocusRef::NotRequested {},
                active_descendant: FocusRef::NotRequested {},
                text_selection: None,
                composition_state: Property::NotRequested {
                    field: Field::Value,
                },
            },
            captures: Vec::new(),
            coverage,
        })),
    };
    let mut output = Bounded {
        bytes: Vec::new(),
        cap: c.request.max_bytes,
    };
    serde_json::to_writer(&mut output, &document).map_err(|_| Error::Limit)?;
    output.write_all(b"\n").map_err(|_| Error::Limit)?;
    Document::from_json(&output.bytes, c.request.max_bytes).map_err(|_| Error::Encoding)?;
    Ok(output.bytes)
}

struct Bounded {
    bytes: Vec<u8>,
    cap: usize,
}
impl Write for Bounded {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > self.cap.saturating_sub(self.bytes.len()) {
            return Err(io::ErrorKind::OutOfMemory.into());
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
