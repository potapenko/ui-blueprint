//! Presentation of explicitly selected, engine-recomputed design relationships.
use crate::{ExportError as E, Result, observed::Aliases, types::*, validate};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uiblueprint_engine::{self as engine, design::Composition};
use uiblueprint_schema::{analysis::*, model::*};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationOperation {
    Equal,
    Ratio,
    Balance,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeclaredException {
    pub source: String,
    pub reason: String,
}

/// Text is explicitly reviewed public annotation, not collector-derived intent.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum DesignRequest {
    Relation {
        caption: String,
        operation: RelationOperation,
        operands: Vec<GeometryQuery>,
        evaluation: EvaluationInput,
    },
    Pattern {
        caption: String,
        basis: String,
        operands: Vec<GeometryQuery>,
        evaluation: EvaluationInput,
    },
    Check {
        caption: String,
        expectation: Box<Expectation>,
        evaluation: EvaluationInput,
        #[serde(default)]
        exception: Option<DeclaredException>,
    },
}

/// Exact safely attributed facts plus a concise human rendering.
#[derive(Clone, Debug, Serialize)]
pub struct Insight {
    pub caption: String,
    pub status: &'static str,
    pub summary: String,
    pub facts: serde_json::Value,
}

pub(crate) fn compile(
    v: &ViewInput,
    scene: &SceneView,
    aliases: &mut Aliases,
    limits: ExportLimits,
) -> Result<Vec<Insight>> {
    validate::require(
        v.design_analysis.len() <= limits.max_components,
        E::InputLimit,
    )?;
    if v.design_analysis.is_empty() {
        return Ok(vec![]);
    }
    let SourceInput::Observed { snapshot, .. } = &v.source else {
        return Err(E::InvalidSource);
    };
    let mut results = Vec::new();
    for request in &v.design_analysis {
        let (caption, evaluation, queries) = match request {
            DesignRequest::Relation {
                caption,
                evaluation,
                operands,
                ..
            }
            | DesignRequest::Pattern {
                caption,
                evaluation,
                operands,
                ..
            } => (caption, evaluation, operands.clone()),
            DesignRequest::Check {
                caption,
                evaluation,
                expectation,
                ..
            } => (
                caption,
                evaluation,
                vec![GeometryQuery::from_expectation(expectation).ok_or(E::InvalidGeometry)?],
            ),
        };
        validate::text(caption)?;
        validate::require(queries.len() <= limits.max_components, E::InputLimit)?;
        let names = queries
            .iter()
            .map(|q| operand_name(q, snapshot, scene))
            .collect::<Vec<_>>();
        let base_unit = crate::compile::unit_name(evaluation.result_space.units);
        let unit = match queries.first().map(|q| q.quantity_kind) {
            Some(QuantityKind::Area) => format!("{base_unit}²"),
            Some(QuantityKind::Ratio) => "dimensionless".into(),
            _ => base_unit.to_owned(),
        };
        let number = crate::compile::display_number;
        let (status, summary, measurements, details) = match request {
            DesignRequest::Relation { operation, .. } => {
                let op = match operation {
                    RelationOperation::Equal => Composition::Equal,
                    RelationOperation::Ratio => Composition::Ratio,
                    RelationOperation::Balance => Composition::Balance,
                };
                let r = engine::design::compose(snapshot, &queries, evaluation, op)
                    .map_err(|_| E::InvalidGeometry)?;
                let summary = if let Some(reason) = r.unavailable {
                    format!("Relationship unavailable: {reason}; no recommendation.")
                } else {
                    let labels = labels(&r.measurements);
                    match operation {
                        RelationOperation::Equal if r.residual == Some(0.0) => format!(
                            "{} equal measurements of {} {unit} in this selected group.",
                            labels.len(),
                            labels[0]
                        ),
                        RelationOperation::Equal => format!(
                            "Measured values {} {unit}; spread {} {unit}. No equality requirement was supplied.",
                            labels.join(", "),
                            number(r.residual.unwrap_or_default())
                        ),
                        RelationOperation::Ratio => format!(
                            "{} {unit} relative to {} {unit}: {} times as large.",
                            labels[0],
                            labels[1],
                            ratio_label(r.value.unwrap_or_default())
                        ),
                        RelationOperation::Balance
                            if r.residual == Some(0.0)
                                && labels.iter().all(|s| !s.contains('≈') && !s.contains('<')) =>
                        {
                            format!(
                                "{} = {} {unit}. The measured parts match the total.",
                                labels[..labels.len() - 1].join(" + "),
                                labels[labels.len() - 1]
                            )
                        }
                        RelationOperation::Balance => format!(
                            "Displayed parts: {}; total {} {unit}. Exact-data difference: {} {unit}; rounded labels are not an exact sum.",
                            labels[..labels.len() - 1].join(" + "),
                            labels[labels.len() - 1],
                            number(r.residual.unwrap_or_default())
                        ),
                    }
                };
                (
                    if r.unavailable.is_some() {
                        "unknown"
                    } else {
                        "measured_relation"
                    },
                    summary,
                    r.measurements,
                    json!({"operation":operation,"value":r.value,"residual":r.residual,"unavailable":r.unavailable}),
                )
            }
            DesignRequest::Pattern { basis, .. } => {
                validate::text(basis)?;
                let r = engine::design::pattern(snapshot, &queries, evaluation)
                    .map_err(|_| E::InvalidGeometry)?;
                let (status, summary) = if let Some(reason) = r.unavailable {
                    (
                        "unknown",
                        format!("Pattern unavailable: {reason}; no recommendation."),
                    )
                } else if let Some(candidate) = r.candidate {
                    if r.exceptions.is_empty() {
                        let quantity = match queries[0].operation {
                            GeometryRelation::Width => "widths",
                            GeometryRelation::Height => "heights",
                            _ => "gaps",
                        };
                        (
                            "repeated_value",
                            format!(
                                "{} measured {quantity} of {} {unit}. No outlier in this selected group.",
                                queries.len(),
                                number(candidate)
                            ),
                        )
                    } else {
                        let deviations = r
                            .exceptions
                            .iter()
                            .map(|(i, d)| {
                                format!(
                                    "{}: {} {unit}, deviation {} {unit}",
                                    names[*i],
                                    labels(&r.measurements)[*i],
                                    number(*d)
                                )
                            })
                            .collect::<Vec<_>>()
                            .join("; ");
                        (
                            "possible_inconsistency",
                            format!(
                                "Check exception: {} of {} selected measurements are {} {unit}; {deviations}. Matching the repeated value would make this group consistent; reason for exception unknown. Basis supplied by caller: {basis}. No automatic correction.",
                                r.support,
                                queries.len(),
                                number(candidate)
                            ),
                        )
                    }
                } else {
                    ("no_pattern", "No strict majority in the selected family; no preferred value or recommendation.".into())
                };
                (
                    status,
                    summary,
                    r.measurements,
                    json!({"basis":basis,"candidate":r.candidate,"support":r.support,"exceptions":r.exceptions,"unavailable":r.unavailable}),
                )
            }
            DesignRequest::Check {
                expectation,
                exception,
                ..
            } => {
                if let Some(e) = exception {
                    validate::text(&e.source)?;
                    validate::text(&e.reason)?;
                }
                let check = engine::check_bound(snapshot, expectation, evaluation)
                    .map_err(|_| E::InvalidGeometry)?;
                let Rule::Geometry {
                    expected,
                    comparison,
                    tolerance,
                    ..
                } = expectation.rule
                else {
                    return Err(E::InvalidGeometry);
                };
                let condition = match comparison {
                    Comparison::Equal => "equal to",
                    Comparison::AtLeast => "at least",
                    Comparison::AtMost => "at most",
                    Comparison::GreaterThan => "greater than",
                };
                let status = match check.finding.status {
                    CheckStatus::Pass => "pass",
                    CheckStatus::Fail => "rule_violation",
                    CheckStatus::Unknown => "unknown",
                };
                let actual = labels(std::slice::from_ref(&check.measurement)).join("");
                let advice = match check.finding.status {
                    CheckStatus::Pass => "Rule satisfied; no correction recommended.",
                    CheckStatus::Fail => {
                        "Check application of this sourced rule; preserve its comparison and tolerance."
                    }
                    CheckStatus::Unknown => {
                        "Measurement or applicability unknown; no violation inferred."
                    }
                };
                let mut summary = format!(
                    "Measured {actual} {unit}; required {condition} {} with tolerance {} {unit}. {advice}",
                    number(expected),
                    number(tolerance)
                );
                if let Some(e) = exception {
                    summary.push_str(&format!(" Caller-declared exception — {}: {}. Check result retained; decision not independently verified.",e.source,e.reason));
                }
                let mut safe = (**expectation).clone();
                safe.id = aliases.id(&safe.id);
                safe.scope_id = aliases.id(&safe.scope_id);
                safe.expected_from = aliases.id(&safe.expected_from);
                safe.targets = safe.targets.iter().map(|k| aliases.key(k)).collect();
                safe.applies_when = conditions(&safe.applies_when, aliases);
                if let Rule::Geometry { anchors, .. } = &mut safe.rule {
                    for a in anchors {
                        anchor(a, aliases);
                    }
                }
                (
                    status,
                    summary,
                    vec![check.measurement],
                    json!({"expectation":safe,"check_status":check.finding.status,"exception":exception}),
                )
            }
        };
        let safe_queries = queries
            .iter()
            .map(|q| query(q, aliases))
            .collect::<Vec<_>>();
        let safe_measurements = measurements
            .iter()
            .map(|m| measurement(m, aliases))
            .collect::<Vec<_>>();
        let facts = json!({"operand_labels":names,"queries":safe_queries,"evaluation":evaluation_alias(evaluation,aliases),"measurements":safe_measurements,"result":details});
        let insight = Insight {
            caption: caption.clone(),
            status,
            summary,
            facts,
        };
        crate::package::bounded_json(&insight, limits.max_output_bytes)?;
        results.push(insight);
    }
    Ok(results)
}

fn operand_name(q: &GeometryQuery, snapshot: &Snapshot, scene: &SceneView) -> String {
    let names = q
        .anchors
        .iter()
        .map(|a| {
            snapshot
                .nodes
                .iter()
                .position(|n| n.key == a.element)
                .map(|i| crate::compile::component_name(scene, i))
                .unwrap_or_else(|| "unresolved target".into())
        })
        .collect::<Vec<_>>();
    let axis = q.anchors.first().map(|a| a.axis.0.as_str()).unwrap_or("");
    format!("{:?} {axis} ({})", q.operation, names.join(" → "))
}

fn ratio_label(value: f64) -> String {
    if value == 0.0 {
        return "0".into();
    }
    if value.abs() < 0.001 {
        return if value < 0.0 { "−<0.001" } else { "<0.001" }.into();
    }
    let label = format!("{value:.3}");
    let label = label.trim_end_matches('0').trim_end_matches('.');
    if label.parse::<f64>().ok() == Some(value) {
        label.into()
    } else {
        format!("≈{label}")
    }
}

fn labels(ms: &[MeasurementResult]) -> Vec<String> {
    ms.iter()
        .map(|m| match m {
            MeasurementResult::Known { measurement } => match measurement.value {
                Value::Quantity { amount, .. } => crate::compile::display_number(amount),
                _ => "unknown".into(),
            },
            MeasurementResult::Unknown { .. } => "unknown".into(),
        })
        .collect()
}
fn anchor(a: &mut Anchor, aliases: &mut Aliases) {
    a.element = aliases.key(&a.element);
    a.coordinate_space = aliases.space(&a.coordinate_space);
}
fn conditions(c: &ContextConditions, aliases: &mut Aliases) -> ContextConditions {
    ContextConditions {
        platform: c.platform.as_ref().map(|p| aliases.id(p)),
        ..c.clone()
    }
}
fn query(q: &GeometryQuery, aliases: &mut Aliases) -> GeometryQuery {
    let mut q = q.clone();
    q.id = aliases.id(&q.id);
    q.scope_id = aliases.id(&q.scope_id);
    q.targets = q.targets.iter().map(|k| aliases.key(k)).collect();
    for a in &mut q.anchors {
        anchor(a, aliases);
    }
    q.applies_when = conditions(&q.applies_when, aliases);
    q
}
fn measurement(m: &MeasurementResult, aliases: &mut Aliases) -> MeasurementResult {
    match m {
        MeasurementResult::Known { measurement: m } => MeasurementResult::Known {
            measurement: Measurement {
                space: aliases.space(&m.space),
                evidence: m.evidence.iter().map(|e| aliases.evidence(e)).collect(),
                ..m.clone()
            },
        },
        MeasurementResult::Unknown { reason, evidence } => MeasurementResult::Unknown {
            reason: *reason,
            evidence: evidence.iter().map(|e| aliases.evidence(e)).collect(),
        },
    }
}
fn evaluation_alias(e: &EvaluationInput, aliases: &mut Aliases) -> EvaluationInput {
    let mut e = e.clone();
    e.snapshot_id = aliases.id(&e.snapshot_id);
    e.context = aliases.context(&e.context);
    e.result_space = aliases.space(&e.result_space);
    for t in &mut e.transforms {
        t.from = aliases.space(&t.from);
        t.to = aliases.space(&t.to);
        t.target.id = aliases.id(&t.target.id);
        t.target.generation = aliases.id(&t.target.generation);
        t.surface.id = aliases.id(&t.surface.id);
        t.surface.generation = aliases.id(&t.surface.generation);
        t.environment_revision = aliases.id(&t.environment_revision);
        t.evidence = aliases.evidence(&t.evidence);
    }
    if let Some(c) = &mut e.conditions {
        c.values = conditions(&c.values, aliases);
        c.evidence = aliases.evidence(&c.evidence);
    }
    e
}

pub(crate) fn text(insights: &[Insight], limit: usize) -> Result<String> {
    if insights.is_empty() {
        return Ok(String::new());
    }
    let mut out = String::from(
        "\nRELATIONSHIPS AND RECOMMENDATIONS — one compact attributed note per selected family, separate from UI. Preserve the family, frame qualifier and prepared result/≈; do not recompute from rounded labels or apply corrections. Full individual operands remain in machine evidence.\n",
    );
    for i in insights {
        let labels = i.facts["operand_labels"]
            .as_array()
            .ok_or(E::InvalidInput)?;
        let participants = if labels.len() <= 3 {
            let mut names = Vec::new();
            for label in labels {
                let label = label.as_str().ok_or(E::InvalidInput)?;
                let name = label
                    .split_once('(')
                    .and_then(|(_, s)| s.strip_suffix(')'))
                    .unwrap_or(label);
                if !names.contains(&name) {
                    names.push(name);
                }
            }
            names.join("; ")
        } else {
            format!(
                "{} source-selected operands in this family (full member list in machine evidence)",
                labels.len()
            )
        };
        let frames = i.facts["queries"]
            .as_array()
            .ok_or(E::InvalidInput)?
            .iter()
            .flat_map(|q| q["anchors"].as_array().into_iter().flatten())
            .filter_map(|a| a["frame_kind"].as_str())
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .join(", ");
        crate::compile::append(
            &mut out,
            &format!(
                "PRINT COMPACT NOTE — family {}. Selected {}. Frames: {}. Result {}.\n",
                crate::package::bounded_compact_json(&i.caption, limit)?,
                crate::package::bounded_compact_json(&participants, limit)?,
                frames,
                crate::package::bounded_compact_json(&i.summary, limit)?
            ),
            limit,
        )?;
    }
    Ok(out)
}
