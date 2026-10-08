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

/// Output selection only; original source coverage remains independent.
#[derive(Clone, Copy, Debug)]
pub struct ComponentLimits {
    pub max_parts: usize,
    pub max_relations: usize,
}
/// Borrowed reported groups/members and attributed context, never another graph.
#[derive(Debug)]
pub struct ComponentView<'a> {
    pub snapshot: &'a Snapshot,
    pub seed: &'a Node,
    pub mappings: Vec<&'a uiblueprint_schema::model::ComponentMapping>,
    pub parts: Vec<&'a Node>,
    /// Direction is relative to an endpoint in the selected recorded group.
    /// Both endpoints remain explicit in the original Relation.
    pub relations: Vec<RelationNeighbor<'a>>,
    pub omitted_parts: usize,
    pub omitted_relations: usize,
}
/// Select groups containing the exact seed and their unique source-order members.
/// Relations touching the seed/group are context; counterpart nodes do not become
/// members. No recursive expansion, inferred grouping or new actionable ref.
/// # Errors
/// Refuses invalid source/missing seed/allocation, as relation_neighbors does.
pub fn component_view<'a>(
    snapshot: &'a Snapshot,
    seed: &SourceKey,
    limits: ComponentLimits,
) -> Result<ComponentView<'a>, ScopeError> {
    validation::validate_snapshot(snapshot).map_err(ScopeError::InvalidSnapshot)?;
    let seed = snapshot
        .nodes
        .iter()
        .find(|n| &n.key == seed)
        .ok_or(ScopeError::MissingSeed)?;
    let matching = |c: &&uiblueprint_schema::model::ComponentMapping| c.members.contains(&seed.key);
    let is_member = |key: &SourceKey| {
        key == &seed.key
            || snapshot
                .components
                .iter()
                .filter(matching)
                .any(|c| c.members.contains(key))
    };
    let mut mappings = Vec::new();
    mappings
        .try_reserve_exact(snapshot.components.iter().filter(matching).count())
        .map_err(|_| ScopeError::AllocationFailure)?;
    mappings.extend(snapshot.components.iter().filter(matching));
    let total_parts = snapshot
        .nodes
        .iter()
        .filter(|n| n.key != seed.key && is_member(&n.key))
        .count();
    let mut parts = Vec::new();
    parts
        .try_reserve_exact(total_parts.min(limits.max_parts))
        .map_err(|_| ScopeError::AllocationFailure)?;
    parts.extend(
        snapshot
            .nodes
            .iter()
            .filter(|n| n.key != seed.key && is_member(&n.key))
            .take(limits.max_parts),
    );
    let incident = || {
        snapshot
            .relations
            .iter()
            .filter(|r| is_member(&r.from) || is_member(&r.to))
    };
    let total_relations = incident().count();
    let mut relations = Vec::new();
    relations
        .try_reserve_exact(total_relations.min(limits.max_relations))
        .map_err(|_| ScopeError::AllocationFailure)?;
    for relation in incident().take(limits.max_relations) {
        let (key, direction) = if relation.from == relation.to {
            (&relation.to, RelationDirection::SelfLoop)
        } else if is_member(&relation.from) {
            (&relation.to, RelationDirection::Outgoing)
        } else {
            (&relation.from, RelationDirection::Incoming)
        };
        let counterpart =
            snapshot
                .nodes
                .iter()
                .find(|n| &n.key == key)
                .ok_or(ScopeError::InvalidSnapshot(
                    ValidationError::DanglingReference,
                ))?;
        relations.push(RelationNeighbor {
            relation,
            counterpart,
            direction,
        });
    }
    Ok(ComponentView {
        snapshot,
        seed,
        mappings,
        omitted_parts: total_parts - parts.len(),
        parts,
        omitted_relations: total_relations - relations.len(),
        relations,
    })
}
