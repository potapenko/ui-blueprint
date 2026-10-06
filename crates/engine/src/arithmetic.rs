use crate::{Available, Details, EvaluationContext, GeometryError, UnknownReason, finite, resolve};
use uiblueprint_schema::model::*;

pub(crate) fn validate_rule(
    op: GeometryRelation,
    anchors: &[Anchor],
    kind: QuantityKind,
) -> Result<(), GeometryError> {
    use GeometryRelation::*;
    let valid_arity = match op {
        Width | Height | Ratio => anchors.len() == 1,
        Aligned | Baseline => anchors.len() >= 2,
        EqualSpacing => anchors.len() >= 3,
        _ => anchors.len() == 2,
    };
    let expected_kind = match op {
        Ratio => QuantityKind::Ratio,
        Intersects => QuantityKind::Area,
        _ => QuantityKind::Length,
    };
    let common_axis = anchors.windows(2).all(|a| a[0].axis == a[1].axis);
    let axis_valid = match op {
        Gap | Aligned | EqualSpacing => {
            common_axis
                && anchors
                    .iter()
                    .all(|a| matches!(a.axis.0.as_str(), "x" | "y"))
        }
        Distance => common_axis,
        _ => true,
    };
    if !valid_arity || kind != expected_kind || !axis_valid {
        return Err(GeometryError::InvalidRule);
    }
    Ok(())
}
fn coordinate(rect: &Rect, anchor: &Anchor) -> Result<f64, GeometryError> {
    if anchor.axis.0 == "x" {
        finite(rect.x + finite(rect.width * anchor.fraction)?)
    } else {
        finite(rect.y + finite(rect.height * anchor.fraction)?)
    }
}
fn spread(values: &[f64]) -> Result<f64, GeometryError> {
    finite(
        values.iter().copied().fold(f64::NEG_INFINITY, f64::max)
            - values.iter().copied().fold(f64::INFINITY, f64::min),
    )
}

pub(crate) fn calculate(
    snapshot: &Snapshot,
    op: GeometryRelation,
    anchors: &[Anchor],
    context: &EvaluationContext<'_>,
    evidence: &mut Vec<Evidence>,
) -> Available<(f64, Details)> {
    if op == GeometryRelation::Baseline {
        let mut values = Vec::new();
        for anchor in anchors {
            match resolve::baseline(snapshot, anchor, context, evidence)? {
                Ok(v) => values.push(v),
                Err(reason) => return Ok(Err(reason)),
            }
        }
        return Ok(Ok((spread(&values)?, Details::Scalar)));
    }
    let mut rects = Vec::new();
    for anchor in anchors {
        match resolve::rect(snapshot, anchor, context, evidence)? {
            Ok(rect) => rects.push(rect),
            Err(reason) => return Ok(Err(reason)),
        }
    }
    calculate_rects(op, anchors, &rects, context.space.origin)
}

fn calculate_rects(
    op: GeometryRelation,
    anchors: &[Anchor],
    rects: &[Rect],
    origin: Origin,
) -> Available<(f64, Details)> {
    use GeometryRelation::*;
    // validate_rule guarantees arity before entering calculation.
    let first = &rects[0];
    let scalar = match op {
        Width => first.width,
        Height => first.height,
        Ratio => {
            if first.height == 0.0 {
                return Ok(Err(UnknownReason::UndefinedRatio));
            }
            finite(first.width / first.height)?
        }
        Gap => finite(coordinate(&rects[1], &anchors[1])? - coordinate(first, &anchors[0])?)?,
        Distance => {
            let second = &rects[1];
            let dx = finite(
                finite(second.x + finite(second.width * anchors[1].fraction)?)?
                    - finite(first.x + finite(first.width * anchors[0].fraction)?)?,
            )?;
            let dy = finite(
                finite(second.y + finite(second.height * anchors[1].fraction)?)?
                    - finite(first.y + finite(first.height * anchors[0].fraction)?)?,
            )?;
            match anchors[0].axis.0.as_str() {
                "x" => dx.abs(),
                "y" => dy.abs(),
                _ => finite(dx.hypot(dy))?,
            }
        }
        Aligned => {
            let values: Result<Vec<_>, _> = rects
                .iter()
                .zip(anchors)
                .map(|(r, a)| coordinate(r, a))
                .collect();
            spread(&values?)?
        }
        EqualSpacing => {
            let mut gaps = Vec::new();
            for pair in rects.windows(2) {
                let gap = if anchors[0].axis.0 == "x" {
                    finite(pair[1].x - finite(pair[0].x + pair[0].width)?)?
                } else {
                    finite(pair[1].y - finite(pair[0].y + pair[0].height)?)?
                };
                gaps.push(gap);
            }
            return Ok(Ok((spread(&gaps)?, Details::Gaps(gaps))));
        }
        Inside | Overflow => {
            let outer = &rects[1];
            let left = finite(first.x - outer.x)?;
            let low = finite(first.y - outer.y)?;
            let right = finite(finite(outer.x + outer.width)? - finite(first.x + first.width)?)?;
            let high = finite(finite(outer.y + outer.height)? - finite(first.y + first.height)?)?;
            let (top, bottom) = if origin == Origin::TopLeft {
                (low, high)
            } else {
                (high, low)
            };
            let minimum = left.min(right).min(low).min(high);
            let value = if op == Inside {
                minimum
            } else {
                (-minimum).max(0.0)
            };
            return Ok(Ok((
                value,
                Details::Insets {
                    left,
                    top,
                    right,
                    bottom,
                },
            )));
        }
        Intersects => {
            let other = &rects[1];
            let x = first.x.max(other.x);
            let y = first.y.max(other.y);
            let right = finite(first.x + first.width)?.min(finite(other.x + other.width)?);
            let bottom = finite(first.y + first.height)?.min(finite(other.y + other.height)?);
            let width = finite(right - x)?.max(0.0);
            let height = finite(bottom - y)?.max(0.0);
            let area = finite(width * height)?;
            let intersection = if width > 0.0 && height > 0.0 {
                Some(Rect {
                    x,
                    y,
                    width,
                    height,
                })
            } else {
                None
            };
            return Ok(Ok((area, Details::Intersection(intersection))));
        }
        Baseline => unreachable!("baseline resolved without rectangles"),
    };
    Ok(Ok((finite(scalar)?, Details::Scalar)))
}
