//! Explicit saved Compose probe normalization. No platform SDK, collection or analytics.
use crate::{Failure, arguments::limit, input, output};
use serde::Deserialize;
use std::{collections::BTreeSet, ffi::OsString, path::PathBuf};
use uiblueprint_schema::{SchemaVersion, model::*, validation};

// Like canonical records, private collector records are map-only and reject duplicates.
macro_rules! record {
    ($name:ident { $($field:ident: $ty:ty),* $(,)? }) => {
        struct $name { $($field: $ty),* }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                #[derive(Deserialize)]
                #[serde(deny_unknown_fields)]
                struct Fields { $($field: $ty),* }
                struct MapOnly;
                impl<'de> serde::de::Visitor<'de> for MapOnly {
                    type Value = $name;
                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("an object")
                    }
                    fn visit_map<A: serde::de::MapAccess<'de>>(self, a: A) -> Result<$name,A::Error> {
                        let fields = Fields::deserialize(serde::de::value::MapAccessDeserializer::new(a))?;
                        Ok($name { $($field: fields.$field),* })
                    }
                }
                d.deserialize_map(MapOnly)
            }
        }
    };
}
record!(Probe {
    probe_version: String, session_id: Id, target: Identity, surface: Identity,
    platform: String, request_id: Id, scope_id: Id, root_marker_id: Id, sequence: u64,
    start_ms: f64, end_ms: f64, density: f64, font_scale: f64, nodes: Vec<Marker>
});
record!(Marker { marker_id: Id, mount_id: Id, bounds: Option<Bounds> });
record!(Bounds {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    density: f64,
    font_scale: f64
});

pub(crate) fn execute(mut args: impl Iterator<Item = OsString>) -> Result<(Vec<u8>, u8), Failure> {
    let (mut path, mut max_input, mut max_output) = (None, None, None);
    let invalid = Failure::invalid("invalid_arguments");
    while let Some(flag) = args.next() {
        let value = args.next().ok_or(invalid)?;
        match flag.to_str() {
            Some("--input") if path.is_none() => path = Some(PathBuf::from(value)),
            Some("--max-input-bytes") if max_input.is_none() => max_input = Some(limit(value)?),
            Some("--max-output-bytes") if max_output.is_none() => max_output = Some(limit(value)?),
            _ => return Err(invalid),
        }
    }
    let mut remaining = max_input.ok_or(invalid)?;
    // Producer ceiling also bounds malformed/adversarial DTO allocation.
    remaining = remaining.min(1_048_576);
    let bytes = input::read(&path.ok_or(invalid)?, &mut remaining)?;
    let probe: Probe =
        serde_json::from_slice(&bytes).map_err(|_| Failure::invalid("invalid_compose"))?;
    let snapshot = normalize(probe)?;
    Ok((
        output::snapshot_json(snapshot, max_output.ok_or(invalid)?)?,
        0,
    ))
}
fn id(s: &str) -> Id {
    Id(s.to_owned())
}
fn public_id(s: &Id) -> bool {
    (1..=96).contains(&s.0.len())
        && s.0
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"._-".contains(&c))
}
fn positive(n: f64) -> bool {
    n.is_finite() && n > 0.0
}
fn normalize(p: Probe) -> Result<Snapshot, Failure> {
    let invalid = Failure::invalid("invalid_compose");
    if p.probe_version != "1.0.0"
        || !matches!(p.platform.as_str(), "desktop_macos" | "android")
        || p.sequence == 0
        || !p.start_ms.is_finite()
        || p.start_ms < 0.0
        || !p.end_ms.is_finite()
        || p.end_ms < p.start_ms
        || p.end_ms - p.start_ms > 10_000.0
        || !positive(p.density)
        || !positive(p.font_scale)
        || !positive(1.0 / p.density)
        || p.nodes.is_empty()
        || p.nodes.len() > 256
        || [
            &p.session_id,
            &p.target.id,
            &p.target.generation,
            &p.surface.id,
            &p.surface.generation,
            &p.request_id,
            &p.scope_id,
            &p.root_marker_id,
        ]
        .into_iter()
        .any(|s| !public_id(s))
    {
        return Err(invalid);
    }
    let mut ids = BTreeSet::new();
    let mut mounts = BTreeSet::new();
    for n in &p.nodes {
        if !public_id(&n.marker_id)
            || !public_id(&n.mount_id)
            || !ids.insert(&n.marker_id)
            || !mounts.insert(&n.mount_id)
        {
            return Err(invalid);
        }
        if let Some(b) = &n.bounds
            && (![b.x, b.y, b.width, b.height]
                .into_iter()
                .all(f64::is_finite)
                || b.width < 0.0
                || b.height < 0.0
                || !positive(b.density)
                || !positive(b.font_scale))
        {
            return Err(invalid);
        }
    }
    let root = p
        .nodes
        .iter()
        .find(|n| n.marker_id == p.root_marker_id)
        .ok_or(invalid)?;
    if let Some(b) = &root.bounds
        && (b.x != 0.0 || b.y != 0.0 || b.density != p.density || b.font_scale != p.font_scale)
    {
        return Err(invalid);
    }
    let observation_id = id(&format!("compose-{}", p.sequence));
    let evidence = Evidence {
        observation_id: observation_id.clone(),
        source_namespace: id("compose.layout"),
        provenance: Provenance::Reported,
        method: id("LayoutCoordinates.localBoundingBoxOf-unclipped"),
        uncertainty: None,
    };
    let unknown_count = p.nodes.iter().filter(|n| n.bounds.is_none()).count() as u64;
    let coverage = Coverage {
        status: CoverageStatus::Partial,
        scope_id: p.scope_id.clone(),
        fields: vec![Field::LayoutBounds],
        omitted_count: None,
        unknown_count: Some(unknown_count),
    };
    let environment = id(&format!("compose-environment-{}", p.sequence));
    let px = Space {
        id: id("compose-root-px"),
        kind: SpaceKind::Local,
        units: Unit::Px,
        origin: Origin::TopLeft,
    };
    let dp = Space {
        id: id("compose-root-dp"),
        units: Unit::Dp,
        ..px.clone()
    };
    let mut nodes = Vec::with_capacity(p.nodes.len());
    for n in &p.nodes {
        nodes.push(node(n, &p, &evidence, &environment, &px, &dp));
    }
    let snapshot = Snapshot {
        surface_records: vec![],
        id: id(&format!("compose-{}-{}", p.session_id.0, p.sequence)),
        revision: p.sequence,
        source_state: None,
        context: Context {
            schema_version: SchemaVersion::CURRENT,
            session_id: p.session_id.clone(),
            target: p.target,
            surfaces: vec![p.surface],
            scope_id: p.scope_id,
            projection: Projection::Design,
            fields: vec![Field::LayoutBounds],
            plugin: PluginIdentity {
                id: id("compose"),
                version: id("1.0.0"),
            },
            environment_revision: environment,
        },
        observations: vec![Observation {
            id: observation_id,
            source_namespace: id("compose.layout"),
            channel: Channel::OptInLayoutProbe,
            start: p.start_ms,
            end: p.end_ms,
            clock_domain: id(&format!("compose-{}", p.session_id.0)),
            time_unit: TimeUnit::Milliseconds,
            freshness_basis: FreshnessBasis::LiveRead,
            consistency_reason: Some(id("ui-thread-sequential-not-atomic")),
            answer_source: AnswerSource::Live,
            freshness: Freshness::Current,
            last_verified: Some(p.end_ms),
            consistency: Consistency::Unknown,
            coverage: coverage.clone(),
        }],
        nodes,
        relations: vec![],
        components: vec![],
        captures: vec![],
        coverage,
        focus: Focus {
            keyboard: FocusRef::NotRequested {},
            accessibility: FocusRef::NotRequested {},
            active_descendant: FocusRef::NotRequested {},
            text_selection: None,
            composition_state: Property::NotRequested {
                field: Field::Value,
            },
        },
    };
    validation::validate_snapshot(&snapshot).map_err(|_| invalid)?;
    Ok(snapshot)
}
fn declaration(name: &str, value: Value) -> SourceDeclaration {
    SourceDeclaration {
        namespace: id("compose.layout"),
        name: id(name),
        state: Availability::Known { value },
        sensitivity: Sensitivity::Public,
        source: id("explicit-debug-probe"),
    }
}
fn node(
    n: &Marker,
    p: &Probe,
    evidence: &Evidence,
    environment: &Id,
    px: &Space,
    dp: &Space,
) -> Node {
    let mut declarations = vec![
        declaration("marker_id", Value::Text(n.marker_id.0.clone())),
        declaration("platform", Value::Text(p.platform.clone())),
        declaration("environment", Value::Text("instrumented_debug".into())),
        declaration("request_id", Value::Text(p.request_id.0.clone())),
    ];
    // Root Density was collected independently, even when its rectangle is unknown.
    if n.marker_id == p.root_marker_id {
        declarations.push(declaration("density", Value::Number(p.density)));
        declarations.push(declaration("font_scale", Value::Number(p.font_scale)));
    } else if let Some(b) = &n.bounds {
        declarations.push(declaration("density", Value::Number(b.density)));
        declarations.push(declaration("font_scale", Value::Number(b.font_scale)));
    }
    let state = if let Some(b) = &n.bounds {
        let mut transform_evidence = evidence.clone();
        transform_evidence.method = id("Compose-root-Density-px-to-dp");
        transform_evidence.provenance = Provenance::Derived;
        Availability::Known {
            value: Value::Geometry(Box::new(Geometry {
                frame_kind: FrameKind::LayoutBounds,
                coordinate_space: px.clone(),
                shape: Shape::Rect(Rect {
                    x: b.x,
                    y: b.y,
                    width: b.width,
                    height: b.height,
                }),
                transform: TransformState::Known {
                    transform: Box::new(Transform {
                        from: px.clone(),
                        to: dp.clone(),
                        affine: [1.0 / p.density, 0.0, 0.0, 1.0 / p.density, 0.0, 0.0],
                        target: p.target.clone(),
                        surface: p.surface.clone(),
                        environment_revision: environment.clone(),
                        evidence: transform_evidence,
                    }),
                },
            })),
        }
    } else {
        Availability::Unknown {
            reason: id("layout_coordinates_unavailable"),
        }
    };
    Node {
        key: SourceKey {
            namespace: id("compose.layout"),
            key: id(&format!("{}-{}", n.marker_id.0, n.mount_id.0)),
        },
        surface: p.surface.clone(),
        native_role: Availability::Unsupported {
            reason: id("layout_marker_not_semantics"),
        },
        properties: vec![Property::Requested {
            field: Field::LayoutBounds,
            sensitivity: Sensitivity::Public,
            evidence: evidence.clone(),
            state,
        }],
        children: vec![],
        extensions: vec![],
        source_declarations: declarations,
    }
}
