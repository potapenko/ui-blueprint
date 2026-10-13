//! Explicit saved-data correspondence; it never changes source identity or cache.
use super::{same_content, same_evidence, sources_compatible};
use crate::{GeometryError, MeasurementResult, measure_query_bound};
use uiblueprint_schema::{analysis::*, model::*, validation};

#[derive(Clone, Debug)]
pub struct SnapshotBinding {
    pub snapshot_id: Id,
    pub revision: u64,
    pub context: Context,
}
#[derive(Clone, Debug)]
pub struct SurfacePair {
    pub before: Identity,
    pub after: Identity,
    pub declaration: Id,
}
#[derive(Clone, Debug)]
pub struct NodePair {
    pub id: Id,
    pub before: SourceKey,
    pub after: SourceKey,
    pub declaration: Id,
}
#[derive(Clone, Debug)]
pub struct MeasurementPair {
    pub id: Id,
    pub before_query: GeometryQuery,
    pub after_query: GeometryQuery,
    pub before_evaluation: EvaluationInput,
    pub after_evaluation: EvaluationInput,
}
#[derive(Clone, Debug)]
pub struct Correspondence {
    pub before_binding: SnapshotBinding,
    pub after_binding: SnapshotBinding,
    pub surface_pairs: Vec<SurfacePair>,
    pub node_pairs: Vec<NodePair>,
    pub measurement_pairs: Vec<MeasurementPair>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CorrespondenceError {
    InvalidSnapshot,
    BindingMismatch,
    ProtectedBasis,
    InvalidPair,
    InvalidMeasurement,
    Capacity,
    NonFinite,
}
#[derive(Debug)]
pub struct ComparisonDiagnostic {
    pub differing_axes: Vec<&'static str>,
    pub strict_diff_blocking_axes: Vec<&'static str>,
    pub protected_basis_compatible: bool,
    pub strict_diff_compatible: bool,
    pub before_nodes: usize,
    pub after_nodes: usize,
    pub exact_key_matches: usize,
    pub next_steps: &'static [&'static str],
}
fn protected(a: &Context, b: &Context) -> bool {
    a.schema_version == b.schema_version
        && a.plugin == b.plugin
        && a.target == b.target
        && a.scope_id == b.scope_id
        && a.projection == b.projection
        && a.fields.len() == b.fields.len()
        && a.fields.iter().all(|f| b.fields.contains(f))
}
fn snapshots(a: &Snapshot, b: &Snapshot) -> Result<(), CorrespondenceError> {
    validation::validate_snapshot(a).map_err(|_| CorrespondenceError::InvalidSnapshot)?;
    validation::validate_snapshot(b).map_err(|_| CorrespondenceError::InvalidSnapshot)
}
/// Fixed axes/counts only, with no private source values in the diagnostic.
/// # Errors
/// Rejects invalid snapshots or fallible bounded diagnostic allocation.
pub fn diagnose_comparison(
    a: &Snapshot,
    b: &Snapshot,
) -> Result<ComparisonDiagnostic, CorrespondenceError> {
    snapshots(a, b)?;
    let x = &a.context;
    let y = &b.context;
    let mut differing_axes = Vec::new();
    let mut blocking = Vec::new();
    differing_axes
        .try_reserve_exact(9)
        .map_err(|_| CorrespondenceError::Capacity)?;
    blocking
        .try_reserve_exact(8)
        .map_err(|_| CorrespondenceError::Capacity)?;
    for (axis, differs) in [
        ("schema_version", x.schema_version != y.schema_version),
        ("plugin", x.plugin != y.plugin),
        ("target", x.target != y.target),
        ("scope_id", x.scope_id != y.scope_id),
        ("projection", x.projection != y.projection),
        (
            "fields",
            x.fields.len() != y.fields.len() || x.fields.iter().any(|f| !y.fields.contains(f)),
        ),
        ("session_id", x.session_id != y.session_id),
        (
            "surfaces",
            x.surfaces.len() != y.surfaces.len()
                || x.surfaces.iter().any(|s| !y.surfaces.contains(s)),
        ),
        (
            "environment_revision",
            x.environment_revision != y.environment_revision,
        ),
    ] {
        if differs {
            differing_axes.push(axis);
            if axis != "environment_revision" {
                blocking.push(axis);
            }
        }
    }
    let compatible = sources_compatible(x, y);
    let basis = protected(x, y);
    Ok(ComparisonDiagnostic {
        differing_axes,
        strict_diff_blocking_axes: blocking,
        protected_basis_compatible: basis,
        strict_diff_compatible: compatible,
        before_nodes: a.nodes.len(),
        after_nodes: b.nodes.len(),
        exact_key_matches: a
            .nodes
            .iter()
            .filter(|n| b.nodes.iter().any(|m| n.key == m.key))
            .count(),
        next_steps: if !basis {
            &["protected_basis_incompatible"]
        } else if compatible {
            &[
                "exact_key_diff_available",
                "explicit_correspondence_available",
                "per_side_measure_available",
            ]
        } else {
            &[
                "explicit_correspondence_required",
                "per_side_measure_available",
            ]
        },
    })
}
fn valid_id(id: &Id) -> bool {
    !id.0.is_empty() && id.0.chars().count() <= 256
}
fn bound(s: &Snapshot, b: &SnapshotBinding) -> bool {
    s.id == b.snapshot_id && s.revision == b.revision && s.context == b.context
}
fn mapped<'a>(
    c: &'a Correspondence,
    key: &SourceKey,
    before: bool,
) -> Option<(usize, &'a NodePair)> {
    c.node_pairs.iter().enumerate().find(|(_, p)| {
        if before {
            &p.before == key
        } else {
            &p.after == key
        }
    })
}
fn validate_pairs(
    a: &Snapshot,
    b: &Snapshot,
    c: &Correspondence,
) -> Result<(), CorrespondenceError> {
    if !bound(a, &c.before_binding) || !bound(b, &c.after_binding) {
        return Err(CorrespondenceError::BindingMismatch);
    }
    if !protected(&a.context, &b.context) {
        return Err(CorrespondenceError::ProtectedBasis);
    }
    if c.surface_pairs.len() != a.context.surfaces.len()
        || c.surface_pairs.len() != b.context.surfaces.len()
        || c.node_pairs.is_empty()
    {
        return Err(CorrespondenceError::InvalidPair);
    }
    for (i, p) in c.surface_pairs.iter().enumerate() {
        if !valid_id(&p.declaration)
            || !a.context.surfaces.contains(&p.before)
            || !b.context.surfaces.contains(&p.after)
            || c.surface_pairs[..i]
                .iter()
                .any(|q| q.before == p.before || q.after == p.after)
        {
            return Err(CorrespondenceError::InvalidPair);
        }
    }
    for (i, p) in c.node_pairs.iter().enumerate() {
        let old = a
            .nodes
            .iter()
            .find(|n| n.key == p.before)
            .ok_or(CorrespondenceError::InvalidPair)?;
        let new = b
            .nodes
            .iter()
            .find(|n| n.key == p.after)
            .ok_or(CorrespondenceError::InvalidPair)?;
        if !valid_id(&p.id)
            || !valid_id(&p.declaration)
            || p.before.namespace != p.after.namespace
            || c.node_pairs[..i]
                .iter()
                .any(|q| q.id == p.id || q.before == p.before || q.after == p.after)
            || !c
                .surface_pairs
                .iter()
                .any(|s| old.surface == s.before && new.surface == s.after)
        {
            return Err(CorrespondenceError::InvalidPair);
        }
    }
    for (i, p) in c.measurement_pairs.iter().enumerate() {
        if !valid_id(&p.id) || c.measurement_pairs[..i].iter().any(|q| p.id == q.id) {
            return Err(CorrespondenceError::InvalidMeasurement);
        }
        validate_measurement(a, b, c, p)?;
    }
    Ok(())
}
fn validate_measurement(
    a: &Snapshot,
    b: &Snapshot,
    c: &Correspondence,
    p: &MeasurementPair,
) -> Result<(), CorrespondenceError> {
    let x = &p.before_query;
    let y = &p.after_query;
    let old = &p.before_evaluation.result_space;
    let new = &p.after_evaluation.result_space;
    let reject = CorrespondenceError::InvalidMeasurement;
    validate_query(x).map_err(|_| reject)?;
    validate_query(y).map_err(|_| reject)?;
    validate_bound_evaluation(a, &p.before_evaluation).map_err(|_| reject)?;
    validate_bound_evaluation(b, &p.after_evaluation).map_err(|_| reject)?;
    if !matches!(
        x.operation,
        GeometryRelation::Width | GeometryRelation::Height | GeometryRelation::Gap
    ) || x.operation != y.operation
        || x.units != y.units
        || x.quantity_kind != QuantityKind::Length
        || x.quantity_kind != y.quantity_kind
        || x.applies_when != y.applies_when
        || x.scope_id != a.context.scope_id
        || y.scope_id != b.context.scope_id
        || old.kind != new.kind
        || old.origin != new.origin
        || old.units != new.units
        || x.units != old.units
        || y.units != new.units
        || x.targets.len() != y.targets.len()
        || x.anchors.len() != y.anchors.len()
    {
        return Err(reject);
    }
    for (old, new) in x.targets.iter().zip(&y.targets) {
        if mapped(c, old, true).is_none_or(|(_, p)| p.after != *new) {
            return Err(reject);
        }
    }
    for (old, new) in x.anchors.iter().zip(&y.anchors) {
        if mapped(c, &old.element, true).is_none_or(|(_, p)| p.after != new.element)
            || old.frame_kind != new.frame_kind
            || old.fraction != new.fraction
            || old.axis != new.axis
        {
            return Err(reject);
        }
    }
    Ok(())
}
#[derive(Clone, Copy, Debug)]
pub enum CorrespondedEntry {
    Property {
        pair_index: usize,
        before_index: usize,
        after_index: usize,
        field: Field,
        before_present: bool,
        after_present: bool,
        content_changed: bool,
        evidence_changed: bool,
    },
    Children {
        pair_index: usize,
        before_index: usize,
        after_index: usize,
        mapped_order_changed: bool,
        unmapped_before_children: usize,
        unmapped_after_children: usize,
    },
}
#[derive(Debug)]
pub struct ComparedMeasurement {
    pub pair_index: usize,
    pub before: MeasurementResult,
    pub after: MeasurementResult,
    pub change: Option<f64>,
}
pub struct CorrespondedDifference<'a> {
    pub before: &'a Snapshot,
    pub after: &'a Snapshot,
    pub correspondence: &'a Correspondence,
    pub entries: Vec<CorrespondedEntry>,
    pub measurements: Vec<ComparedMeasurement>,
    pub unexamined_before_nodes: usize,
    pub unexamined_after_nodes: usize,
    pub omitted_entries: usize,
    pub unknown_measurements: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MappedChildPosition {
    pub child_pair_index: usize,
    pub before_position: Option<usize>,
    pub after_position: Option<usize>,
}
/// Positions in original reported parent lists, including absent membership.
/// They are conditional on declared child correspondence, never deletion facts.
/// # Errors
/// Rejects invalid parent indices or fallible allocation; source graphs are borrowed.
pub fn mapped_child_positions(
    report: &CorrespondedDifference<'_>,
    before_index: usize,
    after_index: usize,
) -> Result<Vec<MappedChildPosition>, CorrespondenceError> {
    let old = &report
        .before
        .nodes
        .get(before_index)
        .ok_or(CorrespondenceError::InvalidPair)?
        .children;
    let new = &report
        .after
        .nodes
        .get(after_index)
        .ok_or(CorrespondenceError::InvalidPair)?
        .children;
    let count = old
        .iter()
        .filter(|k| mapped(report.correspondence, k, true).is_some())
        .count()
        .checked_add(
            new.iter()
                .filter(|k| {
                    mapped(report.correspondence, k, false)
                        .is_some_and(|(_, p)| !old.contains(&p.before))
                })
                .count(),
        )
        .ok_or(CorrespondenceError::Capacity)?;
    let mut positions = Vec::new();
    positions
        .try_reserve_exact(count)
        .map_err(|_| CorrespondenceError::Capacity)?;
    for (i, key) in old.iter().enumerate() {
        if let Some((child_pair_index, p)) = mapped(report.correspondence, key, true) {
            positions.push(MappedChildPosition {
                child_pair_index,
                before_position: Some(i),
                after_position: new.iter().position(|k| *k == p.after),
            });
        }
    }
    for (i, key) in new.iter().enumerate() {
        if let Some((child_pair_index, p)) = mapped(report.correspondence, key, false)
            && !old.contains(&p.before)
        {
            positions.push(MappedChildPosition {
                child_pair_index,
                before_position: None,
                after_position: Some(i),
            });
        }
    }
    Ok(positions)
}
fn visit(
    a: &Snapshot,
    b: &Snapshot,
    c: &Correspondence,
    mut emit: impl FnMut(CorrespondedEntry),
) -> Result<(), CorrespondenceError> {
    for (pair_index, p) in c.node_pairs.iter().enumerate() {
        let before_index = a
            .nodes
            .iter()
            .position(|n| n.key == p.before)
            .ok_or(CorrespondenceError::InvalidPair)?;
        let after_index = b
            .nodes
            .iter()
            .position(|n| n.key == p.after)
            .ok_or(CorrespondenceError::InvalidPair)?;
        let x = &a.nodes[before_index];
        let y = &b.nodes[after_index];
        for old in &x.properties {
            let field = old.field();
            let new = y.properties.iter().find(|v| v.field() == field);
            property(
                a,
                b,
                pair_index,
                before_index,
                after_index,
                field,
                Some(old),
                new,
                &mut emit,
            );
        }
        for new in &y.properties {
            let field = new.field();
            if !x.properties.iter().any(|v| v.field() == field) {
                property(
                    a,
                    b,
                    pair_index,
                    before_index,
                    after_index,
                    field,
                    None,
                    Some(new),
                    &mut emit,
                );
            }
        }
        let old = x
            .children
            .iter()
            .filter_map(|k| mapped(c, k, true).map(|(i, _)| i));
        let new = y
            .children
            .iter()
            .filter_map(|k| mapped(c, k, false).map(|(i, _)| i));
        emit(CorrespondedEntry::Children {
            pair_index,
            before_index,
            after_index,
            mapped_order_changed: !old.eq(new),
            unmapped_before_children: x
                .children
                .iter()
                .filter(|k| mapped(c, k, true).is_none())
                .count(),
            unmapped_after_children: y
                .children
                .iter()
                .filter(|k| mapped(c, k, false).is_none())
                .count(),
        });
    }
    Ok(())
}
#[allow(clippy::too_many_arguments)] // One literal borrowed entry; no new graph representation.
fn property(
    a: &Snapshot,
    b: &Snapshot,
    pair_index: usize,
    before_index: usize,
    after_index: usize,
    field: Field,
    old: Option<&Property>,
    new: Option<&Property>,
    emit: &mut impl FnMut(CorrespondedEntry),
) {
    let content_changed = !same_content(old, new);
    let evidence_changed = !same_evidence(a, old, b, new);
    if content_changed || evidence_changed {
        emit(CorrespondedEntry::Property {
            pair_index,
            before_index,
            after_index,
            field,
            before_present: old.is_some(),
            after_present: new.is_some(),
            content_changed,
            evidence_changed,
        });
    }
}
fn computation(error: GeometryError) -> CorrespondenceError {
    if error == GeometryError::NonFiniteCalculation {
        CorrespondenceError::NonFinite
    } else {
        CorrespondenceError::InvalidMeasurement
    }
}
/// Compare explicitly declared pairs and independent scalar measurements only.
/// # Errors
/// Rejects invalid source/bindings/mappings, incompatible queries, allocation or
/// nonfinite arithmetic. Missing measured facts remain typed unknown results.
pub fn compare_corresponded<'a>(
    a: &'a Snapshot,
    b: &'a Snapshot,
    c: &'a Correspondence,
    max_entries: usize,
) -> Result<CorrespondedDifference<'a>, CorrespondenceError> {
    snapshots(a, b)?;
    validate_pairs(a, b, c)?;
    let mut total = Some(0usize);
    visit(a, b, c, |_| {
        total = total.and_then(|n| n.checked_add(1));
    })?;
    let total = total.ok_or(CorrespondenceError::Capacity)?;
    let count = total.min(max_entries);
    let mut entries = Vec::new();
    entries
        .try_reserve_exact(count)
        .map_err(|_| CorrespondenceError::Capacity)?;
    visit(a, b, c, |entry| {
        if entries.len() < count {
            entries.push(entry);
        }
    })?;
    let count = c
        .measurement_pairs
        .len()
        .min(max_entries.saturating_sub(count));
    let mut measurements = Vec::new();
    measurements
        .try_reserve_exact(count)
        .map_err(|_| CorrespondenceError::Capacity)?;
    let mut unknown_measurements = 0;
    for (pair_index, p) in c.measurement_pairs.iter().enumerate() {
        let before =
            measure_query_bound(a, &p.before_query, &p.before_evaluation).map_err(computation)?;
        let after =
            measure_query_bound(b, &p.after_query, &p.after_evaluation).map_err(computation)?;
        let change = match (&before, &after) {
            (
                MeasurementResult::Known { measurement: x },
                MeasurementResult::Known { measurement: y },
            ) => {
                let (Value::Quantity { amount: x, .. }, Value::Quantity { amount: y, .. }) =
                    (&x.value, &y.value)
                else {
                    return Err(CorrespondenceError::InvalidMeasurement);
                };
                Some(crate::finite(y - x).map_err(computation)?)
            }
            _ => {
                unknown_measurements += 1;
                None
            }
        };
        if measurements.len() < count {
            measurements.push(ComparedMeasurement {
                pair_index,
                before,
                after,
                change,
            });
        }
    }
    let omitted_entries = total
        .checked_add(c.measurement_pairs.len())
        .ok_or(CorrespondenceError::Capacity)?
        - entries.len()
        - measurements.len();
    Ok(CorrespondedDifference {
        before: a,
        after: b,
        correspondence: c,
        entries,
        measurements,
        unexamined_before_nodes: a.nodes.len() - c.node_pairs.len(),
        unexamined_after_nodes: b.nodes.len() - c.node_pairs.len(),
        omitted_entries,
        unknown_measurements,
    })
}
