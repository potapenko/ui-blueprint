use crate::{ExportError as E, Result, types::*, validate::public_text};
use std::collections::BTreeMap;
use uiblueprint_engine::{self as engine, EvaluationContext, GeometryQuery, MeasurementResult};
use uiblueprint_schema::model::*;

/// Per-view aliases preserve equality without exposing process/collector identifiers.
#[derive(Default)]
pub(crate) struct Aliases {
    ids: BTreeMap<String, Id>,
}
impl Aliases {
    pub(crate) fn id(&mut self, id: &Id) -> Id {
        let n = self.ids.len() + 1;
        self.ids
            .entry(id.0.clone())
            .or_insert_with(|| Id(format!("REF{n:04}")))
            .clone()
    }
    pub(crate) fn key(&mut self, k: &SourceKey) -> SourceKey {
        SourceKey {
            namespace: self.id(&k.namespace),
            key: self.id(&k.key),
        }
    }
    fn identity(&mut self, i: &Identity) -> Identity {
        Identity {
            id: self.id(&i.id),
            generation: self.id(&i.generation),
        }
    }
    fn space(&mut self, s: &Space) -> Space {
        Space {
            id: self.id(&s.id),
            ..s.clone()
        }
    }
    pub(crate) fn evidence(&mut self, e: &Evidence) -> Evidence {
        Evidence {
            observation_id: self.id(&e.observation_id),
            source_namespace: self.id(&e.source_namespace),
            provenance: e.provenance,
            method: self.safe_id(&e.method),
            uncertainty: e.uncertainty.as_ref().map(|u| Uncertainty {
                method: self.safe_id(&u.method),
                ..u.clone()
            }),
        }
    }
    fn safe_id(&mut self, id: &Id) -> Id {
        self.id(id)
    }
    fn geometry(&mut self, g: &Geometry) -> Geometry {
        Geometry {
            coordinate_space: self.space(&g.coordinate_space),
            transform: match &g.transform {
                TransformState::LocalOnly {} => TransformState::LocalOnly {},
                TransformState::Unknown { reason } => TransformState::Unknown {
                    reason: self.safe_id(reason),
                },
                TransformState::Known { transform: t } => TransformState::Known {
                    transform: Box::new(Transform {
                        from: self.space(&t.from),
                        to: self.space(&t.to),
                        target: self.identity(&t.target),
                        surface: self.identity(&t.surface),
                        environment_revision: self.id(&t.environment_revision),
                        evidence: self.evidence(&t.evidence),
                        affine: t.affine,
                    }),
                },
            },
            ..g.clone()
        }
    }
    fn availability(&mut self, a: &Availability, permit_text: bool) -> Availability {
        match a {
            Availability::Known { value } => {
                let v = match value {
                    Value::Text(s) if permit_text && public_text(s) => Value::Text(s.clone()),
                    Value::TextList(s) if permit_text && s.iter().all(|t| public_text(t)) => {
                        Value::TextList(s.clone())
                    }
                    Value::Text(_) | Value::TextList(_) => return Availability::Redacted {},
                    Value::Geometry(g) => Value::Geometry(Box::new(self.geometry(g))),
                    Value::Baseline { coordinate, space } => Value::Baseline {
                        coordinate: *coordinate,
                        space: self.space(space),
                    },
                    Value::Identity(i) => Value::Identity(self.identity(i)),
                    x => x.clone(),
                };
                Availability::Known { value: v }
            }
            Availability::Unknown { reason } => Availability::Unknown {
                reason: self.safe_id(reason),
            },
            Availability::Unsupported { reason } => Availability::Unsupported {
                reason: self.safe_id(reason),
            },
            Availability::Redacted {} => Availability::Redacted {},
        }
    }
    fn property(&mut self, p: &Property, allowed: &[Field]) -> Property {
        match p {
            Property::NotRequested { .. } => p.clone(),
            Property::Requested {
                field,
                sensitivity,
                evidence,
                state,
            } => Property::Requested {
                field: *field,
                sensitivity: *sensitivity,
                evidence: self.evidence(evidence),
                state: if *sensitivity == Sensitivity::Sensitive || *field == Field::Value {
                    Availability::Redacted {}
                } else {
                    self.availability(state, allowed.contains(field))
                },
            },
        }
    }
    pub(crate) fn coverage(&mut self, c: &Coverage) -> Coverage {
        Coverage {
            scope_id: self.id(&c.scope_id),
            ..c.clone()
        }
    }
    pub(crate) fn observations(&mut self, s: &Snapshot) -> Vec<Observation> {
        s.observations
            .iter()
            .map(|o| Observation {
                id: self.id(&o.id),
                source_namespace: self.id(&o.source_namespace),
                clock_domain: self.id(&o.clock_domain),
                consistency_reason: o.consistency_reason.as_ref().map(|r| self.safe_id(r)),
                coverage: self.coverage(&o.coverage),
                ..o.clone()
            })
            .collect()
    }
    pub(crate) fn surfaces(&mut self, s: &Snapshot) -> Vec<SurfaceRecord> {
        s.surface_records
            .iter()
            .map(|r| SurfaceRecord {
                identity: self.identity(&r.identity),
                native_owner: self.availability(&r.native_owner, false),
                initiated_by: r.initiated_by.as_ref().map(|i| self.identity(i)),
                anchor: r.anchor.as_ref().map(|k| self.key(k)),
                evidence: self.evidence(&r.evidence),
            })
            .collect()
    }
    pub(crate) fn relations(&mut self, s: &Snapshot) -> Vec<Relation> {
        s.relations
            .iter()
            .map(|r| Relation {
                kind: r.kind,
                from: self.key(&r.from),
                to: self.key(&r.to),
                evidence: self.evidence(&r.evidence),
            })
            .collect()
    }
    pub(crate) fn mappings(&mut self, s: &Snapshot) -> Vec<ComponentMapping> {
        s.components
            .iter()
            .map(|m| ComponentMapping {
                logical_component_key: self.id(&m.logical_component_key),
                members: m.members.iter().map(|k| self.key(k)).collect(),
                declaration_source: self.id(&m.declaration_source),
                provenance: m.provenance,
            })
            .collect()
    }
    fn focus_ref(&mut self, r: &FocusRef) -> FocusRef {
        match r {
            FocusRef::Known { target, evidence } => FocusRef::Known {
                target: self.key(target),
                evidence: self.evidence(evidence),
            },
            FocusRef::None { evidence } => FocusRef::None {
                evidence: self.evidence(evidence),
            },
            FocusRef::Unknown { reason } => FocusRef::Unknown {
                reason: self.safe_id(reason),
            },
            FocusRef::NotRequested {} => FocusRef::NotRequested {},
        }
    }
    pub(crate) fn focus(&mut self, f: &Focus) -> Focus {
        Focus {
            keyboard: self.focus_ref(&f.keyboard),
            accessibility: self.focus_ref(&f.accessibility),
            active_descendant: self.focus_ref(&f.active_descendant),
            text_selection: f.text_selection.as_ref().map(|s| TextSelection {
                units: self.safe_id(&s.units),
                evidence: self.evidence(&s.evidence),
                ..s.clone()
            }),
            composition_state: self.property(&f.composition_state, &[]),
        }
    }
}
pub(crate) fn components(
    s: &Snapshot,
    allowed: &[Field],
    aliases: &mut Aliases,
) -> Vec<SceneComponent> {
    let ids: BTreeMap<_, _> = s
        .nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (&n.key, format!("N{i:03}")))
        .collect();
    s.nodes
        .iter()
        .map(|n| SceneComponent {
            id: ids[&n.key].clone(),
            surface: Some(aliases.identity(&n.surface)),
            source_key: Some(aliases.key(&n.key)),
            native_role: Some(aliases.availability(&n.native_role, true)),
            parent: None,
            children: n
                .children
                .iter()
                .filter_map(|k| ids.get(k).cloned())
                .collect(),
            role: None,
            label: None,
            properties: n
                .properties
                .iter()
                .map(|p| aliases.property(p, allowed))
                .collect(),
            geometry: None,
            declarations: n
                .source_declarations
                .iter()
                .map(|d| SourceDeclaration {
                    namespace: aliases.id(&d.namespace),
                    name: aliases.id(&d.name),
                    state: aliases.availability(&d.state, false),
                    sensitivity: d.sensitivity,
                    source: aliases.id(&d.source),
                })
                .collect(),
            extensions: n
                .extensions
                .iter()
                .map(|e| ExtensionProperty {
                    namespace: aliases.id(&e.namespace),
                    name: aliases.id(&e.name),
                    property: aliases.property(&e.property, &[]),
                })
                .collect(),
            state_and_actions: None,
        })
        .collect()
}
/// Uses G01 rather than implementing a second observed-geometry calculator.
pub(crate) fn dimensions(s: &Snapshot, aliases: &mut Aliases) -> Result<Vec<Dimension>> {
    let mut out = vec![];
    for (i, n) in s.nodes.iter().enumerate() {
        for p in &n.properties {
            let Some(Value::Geometry(g)) = p.known() else {
                continue;
            };
            for (op, axis, edges) in [
                (GeometryRelation::Width, "x", [Edge::Left, Edge::Right]),
                (GeometryRelation::Height, "y", [Edge::Top, Edge::Bottom]),
            ] {
                let anchor = Anchor {
                    element: n.key.clone(),
                    frame_kind: g.frame_kind,
                    coordinate_space: g.coordinate_space.clone(),
                    fraction: 0.0,
                    axis: Id(axis.into()),
                };
                let query = GeometryQuery {
                    id: Id("export-dimension".into()),
                    scope_id: s.context.scope_id.clone(),
                    targets: vec![n.key.clone()],
                    operation: op,
                    anchors: vec![anchor],
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
                let measured =
                    engine::measure_query(s, &query, &context).map_err(|_| E::InvalidGeometry)?;
                let (value, reason, evidence) = match measured {
                    MeasurementResult::Known { measurement: m } => {
                        let Value::Quantity { amount, .. } = m.value else {
                            return Err(E::InvalidGeometry);
                        };
                        (Some(amount), None, m.evidence)
                    }
                    MeasurementResult::Unknown { reason, evidence } => {
                        (None, Some(reason.as_str().into()), evidence)
                    }
                };
                out.push(Dimension {
                    id: format!("M{:04}", out.len() + 1),
                    label: format!("{op:?} {field:?}", field = p.field()),
                    anchors: edges.map(|edge| DimensionAnchor {
                        component: format!("N{i:03}"),
                        frame_kind: g.frame_kind,
                        space: aliases.space(&g.coordinate_space),
                        edge,
                    }),
                    value,
                    units: g.coordinate_space.units,
                    source_kind: SourceKind::Observed,
                    evidence: evidence.iter().map(|e| aliases.evidence(e)).collect(),
                    requirement_ref: None,
                    unknown_reason: reason,
                    check_tolerance: None,
                });
            }
        }
    }
    Ok(out)
}
