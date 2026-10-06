use super::*;
use std::collections::BTreeMap;

fn nodes(nodes: &[Node], c: &Context, observations: &[Observation]) -> Result {
    require(
        unique(nodes.iter().map(|n| &n.key)),
        ValidationError::DuplicateIdentity,
    )?;
    for node in nodes {
        require(
            !matches!(&node.native_role,Availability::Known { value } if !matches!(value,Value::Text(_))),
            ValidationError::ValueType,
        )?;
        for declaration in &node.source_declarations {
            require(
                declaration.sensitivity != Sensitivity::Sensitive
                    || matches!(declaration.state, Availability::Redacted),
                ValidationError::PrivateValue,
            )?;
        }

        require(
            c.surfaces.contains(&node.surface),
            ValidationError::StaleTarget,
        )?;
        require(
            unique(node.properties.iter().map(Property::field)) && unique(&node.children),
            ValidationError::DuplicateIdentity,
        )?;
        for field in &c.fields {
            require(
                node.properties
                    .iter()
                    .any(|p| matches!(p, Property::Requested { field: f, .. } if f == field)),
                ValidationError::MissingProperty,
            )?;
        }
        require(
            unique(node.extensions.iter().map(|e| (&e.namespace, &e.name))),
            ValidationError::DuplicateIdentity,
        )?;
        for extension in &node.extensions {
            validate_property(&extension.property, observations)?;
        }
        for p in &node.properties {
            require(
                matches!(p, Property::Requested { .. }) == c.fields.contains(&p.field()),
                ValidationError::MissingProperty,
            )?;
            validate_property(p, observations)?;
            if let Some(Value::Geometry(g)) = p.known() {
                geometry_context(g, c)?;
            }
        }
    }
    Ok(())
}
fn graph_refs(
    nodes: &BTreeMap<&SourceKey, &Node>,
    relations: &[Relation],
    focus: &Focus,
    observations: &[Observation],
) -> Result {
    for node in nodes.values() {
        for child in &node.children {
            require(
                nodes.contains_key(child),
                ValidationError::DanglingReference,
            )?;
        }
        let mut pending = node.children.clone();
        let mut seen = BTreeSet::new();
        while let Some(key) = pending.pop() {
            require(key != node.key, ValidationError::DanglingReference)?;
            if seen.insert(key.clone()) {
                if let Some(child) = nodes.get(&key) {
                    pending.extend(child.children.iter().cloned());
                }
            }
        }
    }
    for relation in relations {
        require(
            nodes.contains_key(&relation.from) && nodes.contains_key(&relation.to),
            ValidationError::DanglingReference,
        )?;
        evidence(&relation.evidence, observations)?;
    }
    for reference in [
        &focus.keyboard,
        &focus.accessibility,
        &focus.active_descendant,
    ] {
        match reference {
            FocusRef::Known {
                target,
                evidence: e,
            } => {
                require(
                    nodes.contains_key(target),
                    ValidationError::DanglingReference,
                )?;
                evidence(e, observations)?;
            }
            FocusRef::None { evidence: e } => {
                evidence(e, observations)?;
            }
            _ => (),
        }
    }
    if let Some(selection) = &focus.text_selection {
        evidence(&selection.evidence, observations)?;
    }
    validate_property(&focus.composition_state, observations)
}

pub fn validate_snapshot(s: &Snapshot) -> Result {
    validate_context(&s.context)?;
    coverage_context(&s.coverage, &s.context)?;
    require(
        unique(s.observations.iter().map(|o| &o.id)),
        ValidationError::DuplicateIdentity,
    )?;
    require(!s.observations.is_empty(), ValidationError::InvalidEvidence)?;
    for o in &s.observations {
        validate_observation(o)?;
        require(
            o.coverage.scope_id == s.context.scope_id
                && o.coverage
                    .fields
                    .iter()
                    .all(|f| s.context.fields.contains(f)),
            ValidationError::InvalidCoverage,
        )?;
    }
    nodes(&s.nodes, &s.context, &s.observations)?;
    let node_map = s.nodes.iter().map(|n| (&n.key, n)).collect();
    graph_refs(&node_map, &s.relations, &s.focus, &s.observations)?;
    require(
        unique(s.components.iter().map(|c| &c.logical_component_key)),
        ValidationError::DuplicateIdentity,
    )?;
    for component in &s.components {
        require(
            component.provenance == Provenance::Reported,
            ValidationError::InvalidEvidence,
        )?;
        require(
            !component.members.is_empty()
                && unique(&component.members)
                && component.members.iter().all(|k| node_map.contains_key(k)),
            ValidationError::DanglingReference,
        )?;
    }
    require(
        unique(s.surface_records.iter().map(|s| &s.identity)),
        ValidationError::DuplicateIdentity,
    )?;
    for surface in &s.surface_records {
        require(
            s.context.surfaces.contains(&surface.identity),
            ValidationError::DanglingReference,
        )?;
        evidence(&surface.evidence, &s.observations)?;
        require(
            !matches!(&surface.native_owner,Availability::Known { value } if !matches!(value,Value::Identity(_))),
            ValidationError::ValueType,
        )?;
        require(
            surface
                .anchor
                .as_ref()
                .is_none_or(|k| node_map.contains_key(k)),
            ValidationError::DanglingReference,
        )?;
    }
    for capture in &s.captures {
        let observation = s
            .observations
            .iter()
            .find(|o| o.id == capture.observation_id)
            .ok_or(ValidationError::InvalidEvidence)?;
        require(
            observation.channel == Channel::RenderedCapture
                && s.context.surfaces.contains(&capture.capture_target),
            ValidationError::InvalidEvidence,
        )?;
        require(
            capture.pixel_width > 0 && capture.pixel_height > 0 && !capture.captures_audio,
            ValidationError::InvalidGeometry,
        )?;
        require(
            unique(
                capture
                    .included_surfaces
                    .iter()
                    .chain(&capture.excluded_surfaces)
                    .chain(&capture.unresolved_surfaces),
            ),
            ValidationError::DuplicateIdentity,
        )?;
        require(
            capture.surface_coverage != CoverageStatus::Complete
                || capture.unresolved_surfaces.is_empty(),
            ValidationError::InvalidCoverage,
        )?;
        if let TransformState::Known { transform } = &capture.crop_transform {
            require(
                transform.to.units == Unit::Px && transform.affine.iter().all(|n| n.is_finite()),
                ValidationError::InvalidGeometry,
            )?;
            evidence(&transform.evidence, &s.observations)?;
        }
    }
    Ok(())
}

/// Checks a complete update against explicit base data, without retaining or
/// publishing a graph. K01 owns application/replay/cache behavior.
pub fn validate_delta(base: &Snapshot, delta: &Delta) -> Result {
    validate_snapshot(base)?;
    validate_context(&delta.context)?;
    require(
        contexts_compatible(&base.context, &delta.context),
        ValidationError::IncompatibleContext,
    )?;
    require(
        delta.base_revision == base.revision
            && delta.revision
                == base
                    .revision
                    .checked_add(1)
                    .ok_or(ValidationError::ResyncRequired)?,
        ValidationError::ResyncRequired,
    )?;
    coverage_context(&delta.coverage, &delta.context)?;
    require(
        unique(delta.observations.iter().map(|o| &o.id)),
        ValidationError::DuplicateIdentity,
    )?;
    for o in &delta.observations {
        validate_observation(o)?;
        coverage_context(&o.coverage, &delta.context)?;
    }
    nodes(&delta.upsert, &delta.context, &delta.observations)?;
    require(
        unique(delta.removed.iter().map(|r| &r.key)),
        ValidationError::DuplicateIdentity,
    )?;
    let mut remaining: BTreeMap<_, _> = base.nodes.iter().map(|n| (&n.key, n)).collect();
    for removal in &delta.removed {
        require(
            remaining.remove(&removal.key).is_some()
                && !delta.upsert.iter().any(|n| n.key == removal.key),
            ValidationError::DanglingReference,
        )?;
        let observed = evidence(&removal.evidence, &delta.observations)?;
        require(
            observed.coverage.status == CoverageStatus::Complete
                && observed.coverage.scope_id == delta.context.scope_id,
            ValidationError::InvalidCoverage,
        )?;
    }
    for node in &delta.upsert {
        remaining.insert(&node.key, node);
    }
    graph_refs(
        &remaining,
        &delta.relations,
        &delta.focus,
        &delta.observations,
    )
}

pub(super) fn node<'a>(snapshot: &'a Snapshot, key: &SourceKey) -> Result<&'a Node> {
    snapshot
        .nodes
        .iter()
        .find(|n| &n.key == key)
        .ok_or(ValidationError::DanglingReference)
}
pub(super) fn property<'a>(node: &'a Node, field: Field) -> Option<&'a Property> {
    node.properties.iter().find(|p| p.field() == field)
}
