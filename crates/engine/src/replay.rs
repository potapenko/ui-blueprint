//! Pure, atomic application of canonical deltas to immutable snapshots.
//!
//! This owns replay only: no retention store, byte policy, clock or IO. A caller
//! supplies the identity for the newly materialized snapshot. Source evidence is
//! never restamped, and an unavailable replacement never inherits old values.
use std::collections::{BTreeMap, BTreeSet};
use uiblueprint_schema::{
    model::*,
    validation::{self, ValidationError},
};

/// A full compatible snapshot is needed before replay can continue.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResyncReason {
    ContextMismatch,
    RevisionMismatch,
    UnresolvedReference,
    ObservationConflict,
}

/// Payload-free replay failure. None of these variants modifies the base.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReplayError {
    ResyncRequired(ResyncReason),
    InvalidInput(ValidationError),
    InvalidResult(ValidationError),
    InvalidSnapshotId,
}
impl std::fmt::Display for ReplayError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl std::error::Error for ReplayError {}

/// Apply a whole canonical delta without changing either input.
///
/// The returned snapshot has the supplied distinct ID, delta revision/context/
/// source_state/coverage, complete replacement nodes, and the delta's complete
/// relations/focus. Unmentioned nodes and metadata are retained with their own
/// original evidence. No absence/partial coverage implies a removal.
///
/// Delta observations keep their order. Referenced historical observations not
/// present in the delta follow in base order; unused old observations are not
/// promoted into the new snapshot. Conflicting reuse of an Observation ID refuses
/// replay. Retained capture data is historical, not a refreshed pixel channel.
///
/// # Errors
/// Canonical validation rejects invalid input and dangling graph references.
/// Context or base-revision mismatch, observation identity conflict, and dangling
/// retained metadata require resync. Delta cannot update component mappings,
/// surface records or captures; use a full snapshot when those must change.
/// The result ID must be nonempty, at most 256 characters and differ from base.
/// No output is published until the complete candidate passes shared validation.
pub fn replay(base: &Snapshot, delta: &Delta, snapshot_id: Id) -> Result<Snapshot, ReplayError> {
    if snapshot_id == base.id || snapshot_id.0.is_empty() || snapshot_id.0.chars().count() > 256 {
        return Err(ReplayError::InvalidSnapshotId);
    }
    validation::validate_delta(base, delta).map_err(input_error)?;
    let observations = merge_observations(base, delta)?;
    let removed: BTreeSet<_> = delta.removed.iter().map(|r| &r.key).collect();
    let mut replacements: BTreeMap<_, _> =
        delta.upsert.iter().map(|node| (&node.key, node)).collect();
    let mut nodes = Vec::new();
    for old in &base.nodes {
        if removed.contains(&old.key) {
            continue;
        }
        nodes.push(replacements.remove(&old.key).unwrap_or(old).clone());
    }
    // Keep explicit delta order for newly introduced nodes, not map iteration.
    for node in &delta.upsert {
        if replacements.contains_key(&node.key) {
            nodes.push(node.clone());
        }
    }
    let mut candidate = Snapshot {
        id: snapshot_id,
        revision: delta.revision,
        source_state: delta.source_state.clone(),
        context: delta.context.clone(),
        observations: Vec::new(),
        nodes,
        relations: delta.relations.clone(),
        focus: delta.focus.clone(),
        coverage: delta.coverage.clone(),
        components: base.components.clone(),
        surface_records: base.surface_records.clone(),
        captures: base.captures.clone(),
    };
    let needed = referenced_observations(&candidate);
    candidate.observations = observations
        .into_iter()
        .filter(|observation| {
            needed.contains(&observation.id)
                || delta.observations.iter().any(|o| o.id == observation.id)
        })
        .collect();
    validation::validate_snapshot(&candidate).map_err(|error| match error {
        ValidationError::DanglingReference => {
            ReplayError::ResyncRequired(ResyncReason::UnresolvedReference)
        }
        error => ReplayError::InvalidResult(error),
    })?;
    Ok(candidate)
}

fn input_error(error: ValidationError) -> ReplayError {
    match error {
        ValidationError::IncompatibleContext => {
            ReplayError::ResyncRequired(ResyncReason::ContextMismatch)
        }
        ValidationError::ResyncRequired => {
            ReplayError::ResyncRequired(ResyncReason::RevisionMismatch)
        }
        ValidationError::DanglingReference => {
            ReplayError::ResyncRequired(ResyncReason::UnresolvedReference)
        }
        error => ReplayError::InvalidInput(error),
    }
}
fn merge_observations(base: &Snapshot, delta: &Delta) -> Result<Vec<Observation>, ReplayError> {
    let old: BTreeMap<_, _> = base.observations.iter().map(|o| (&o.id, o)).collect();
    for observation in &delta.observations {
        if old
            .get(&observation.id)
            .is_some_and(|previous| *previous != observation)
        {
            return Err(ReplayError::ResyncRequired(
                ResyncReason::ObservationConflict,
            ));
        }
    }
    let mut observations = delta.observations.clone();
    let incoming: BTreeSet<_> = delta.observations.iter().map(|o| &o.id).collect();
    observations.extend(
        base.observations
            .iter()
            .filter(|o| !incoming.contains(&o.id))
            .cloned(),
    );
    Ok(observations)
}

// Reference collection preserves evidence; all validity decisions still use the
// canonical validator. This is not a second graph or wire-schema validator.
fn referenced_observations(snapshot: &Snapshot) -> BTreeSet<Id> {
    let mut result = BTreeSet::new();
    for node in &snapshot.nodes {
        for property in &node.properties {
            property_observations(property, &mut result);
        }
        for extension in &node.extensions {
            property_observations(&extension.property, &mut result);
        }
        for declaration in &node.source_declarations {
            state_observations(&declaration.state, &mut result);
        }
    }
    for relation in &snapshot.relations {
        result.insert(relation.evidence.observation_id.clone());
    }
    for surface in &snapshot.surface_records {
        result.insert(surface.evidence.observation_id.clone());
    }
    for capture in &snapshot.captures {
        result.insert(capture.observation_id.clone());
        transform_observations(&capture.crop_transform, &mut result);
    }
    let focus = &snapshot.focus;
    for reference in [
        &focus.keyboard,
        &focus.accessibility,
        &focus.active_descendant,
    ] {
        match reference {
            FocusRef::Known { evidence, .. } | FocusRef::None { evidence } => {
                result.insert(evidence.observation_id.clone());
            }
            FocusRef::NotRequested {} | FocusRef::Unknown { .. } => {}
        }
    }
    if let Some(selection) = &focus.text_selection {
        result.insert(selection.evidence.observation_id.clone());
    }
    property_observations(&focus.composition_state, &mut result);
    result
}
fn property_observations(property: &Property, result: &mut BTreeSet<Id>) {
    if let Property::Requested {
        evidence, state, ..
    } = property
    {
        result.insert(evidence.observation_id.clone());
        state_observations(state, result);
    }
}
fn state_observations(state: &Availability, result: &mut BTreeSet<Id>) {
    if let Availability::Known {
        value: Value::Geometry(geometry),
    } = state
    {
        transform_observations(&geometry.transform, result);
    }
}
fn transform_observations(transform: &TransformState, result: &mut BTreeSet<Id>) {
    if let TransformState::Known { transform } = transform {
        result.insert(transform.evidence.observation_id.clone());
    }
}
