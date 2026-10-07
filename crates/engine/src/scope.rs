//! Borrowed one-step context from explicit canonical relations, without collection.
use uiblueprint_schema::{
    model::{Node, Relation, Snapshot, SourceKey},
    validation::{self, ValidationError},
};

/// Output bound only. Validation and lookup inspect the supplied stored snapshot;
/// this is not an acquisition, execution-time or whole-process memory budget.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NeighborLimits {
    /// Maximum relation entries, each with one borrowed counterpart. Zero is a
    /// valid empty selection; omitted entries are still counted explicitly.
    pub max_relations: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RelationDirection {
    Outgoing,
    Incoming,
    /// The recorded relation has the seed at both ends; returned once.
    SelfLoop,
}

#[derive(Clone, Copy, Debug)]
pub struct RelationNeighbor<'a> {
    pub relation: &'a Relation,
    pub counterpart: &'a Node,
    pub direction: RelationDirection,
}

/// A selection over the unchanged source, not a new Snapshot or projection.
/// One entry per incident relation, in source order, including repeated links.
/// Counterparts can therefore repeat; namespaces and evidence are never merged.
#[derive(Debug)]
pub struct NeighborView<'a> {
    pub snapshot: &'a Snapshot,
    pub seed: &'a Node,
    pub neighbors: Vec<RelationNeighbor<'a>>,
    /// Known incident relations omitted solely by max_relations. Zero does not
    /// upgrade snapshot.coverage or establish completeness of the live UI.
    pub omitted_relations: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScopeError {
    InvalidSnapshot(ValidationError),
    MissingSeed,
    AllocationFailure,
}
impl std::fmt::Display for ScopeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl std::error::Error for ScopeError {}

/// Select explicit incoming/outgoing relations of a source key within one stored
/// snapshot. No recursive walk, inferred edge, identity merge or actionable ref.
/// Properties, Evidence, surface generations and source coverage remain borrowed.
/// Children/component memberships are not silently converted into relations.
///
/// # Errors
/// Rejects invalid canonical input, an absent exact seed key or failed output
/// allocation. Validation uses its established allocations; no allocation-free
/// or RSS guarantee is made. Neither input nor a partial result is published on error.
pub fn relation_neighbors<'a>(
    snapshot: &'a Snapshot,
    seed: &SourceKey,
    limits: NeighborLimits,
) -> Result<NeighborView<'a>, ScopeError> {
    validation::validate_snapshot(snapshot).map_err(ScopeError::InvalidSnapshot)?;
    let seed = snapshot
        .nodes
        .iter()
        .find(|node| &node.key == seed)
        .ok_or(ScopeError::MissingSeed)?;
    let incident = || {
        snapshot
            .relations
            .iter()
            .filter(|relation| relation.from == seed.key || relation.to == seed.key)
    };
    let total = incident().count();
    let returned = total.min(limits.max_relations);
    let mut neighbors = Vec::new();
    neighbors
        .try_reserve_exact(returned)
        .map_err(|_| ScopeError::AllocationFailure)?;
    for relation in incident().take(returned) {
        let (key, direction) = if relation.from == seed.key && relation.to == seed.key {
            (&seed.key, RelationDirection::SelfLoop)
        } else if relation.from == seed.key {
            (&relation.to, RelationDirection::Outgoing)
        } else {
            (&relation.from, RelationDirection::Incoming)
        };
        let counterpart = snapshot.nodes.iter().find(|node| &node.key == key).ok_or(
            ScopeError::InvalidSnapshot(ValidationError::DanglingReference),
        )?;
        neighbors.push(RelationNeighbor {
            relation,
            counterpart,
            direction,
        });
    }
    Ok(NeighborView {
        snapshot,
        seed,
        neighbors,
        omitted_relations: total - returned,
    })
}
