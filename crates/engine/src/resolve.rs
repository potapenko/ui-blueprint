use crate::{Available, EvaluationContext, GeometryError, UnknownReason, finite};
use uiblueprint_schema::{model::*, validation::ValidationError};

pub(crate) fn record_evidence(
    snapshot: &Snapshot,
    source: &Evidence,
    evidence: &mut Vec<Evidence>,
) -> Result<Option<UnknownReason>, GeometryError> {
    let observation = snapshot
        .observations
        .iter()
        .find(|o| o.id == source.observation_id)
        .ok_or(GeometryError::InvalidInput(
            ValidationError::InvalidEvidence,
        ))?;
    if observation.source_namespace != source.source_namespace || source.method.0.is_empty() {
        return Err(GeometryError::InvalidInput(
            ValidationError::InvalidEvidence,
        ));
    }
    if source
        .uncertainty
        .as_ref()
        .is_some_and(|u| !u.absolute.is_finite() || u.absolute < 0.0)
    {
        return Err(GeometryError::InvalidInput(
            ValidationError::InvalidEvidence,
        ));
    }
    if !evidence.contains(source) {
        evidence.push(source.clone());
    }
    if observation.consistency == Consistency::Unstable {
        return Ok(Some(UnknownReason::UnstableState));
    }
    Ok(None)
}

pub(crate) fn applicability(
    snapshot: &Snapshot,
    required: &ContextConditions,
    context: &EvaluationContext<'_>,
    evidence: &mut Vec<Evidence>,
) -> Result<Option<UnknownReason>, GeometryError> {
    if required.platform.is_none() && required.input_mode.is_none() && required.text_scale.is_none()
    {
        return Ok(None);
    }
    let Some((actual, source)) = context.conditions else {
        return Ok(Some(UnknownReason::ApplicabilityUnknown));
    };
    if actual
        .text_scale
        .is_some_and(|v| !v.is_finite() || v <= 0.0)
    {
        return Err(GeometryError::InvalidRule);
    }
    if let Some(reason) = record_evidence(snapshot, source, evidence)? {
        return Ok(Some(reason));
    }
    if (required.platform.is_some() && actual.platform.is_none())
        || (required.input_mode.is_some() && actual.input_mode.is_none())
        || (required.text_scale.is_some() && actual.text_scale.is_none())
    {
        return Ok(Some(UnknownReason::ApplicabilityUnknown));
    }
    if (required.platform.is_some() && required.platform != actual.platform)
        || (required.input_mode.is_some() && required.input_mode != actual.input_mode)
        || (required.text_scale.is_some() && required.text_scale != actual.text_scale)
    {
        return Ok(Some(UnknownReason::NotApplicable));
    }
    Ok(None)
}

pub(crate) fn field(kind: FrameKind) -> Field {
    match kind {
        FrameKind::LayoutBounds => Field::LayoutBounds,
        FrameKind::AccessibilityBounds => Field::AccessibilityBounds,
        FrameKind::HitRegion => Field::HitRegion,
        FrameKind::VisibleRegion => Field::VisibleRegion,
        FrameKind::PaintBounds => Field::PaintBounds,
    }
}

fn property<'a>(
    snapshot: &'a Snapshot,
    anchor: &Anchor,
    field: Field,
    evidence: &mut Vec<Evidence>,
) -> Available<(&'a Node, &'a Value)> {
    let Some(node) = snapshot.nodes.iter().find(|n| n.key == anchor.element) else {
        return Ok(Err(UnknownReason::MissingTarget));
    };
    let Some(property) = node.properties.iter().find(|p| p.field() == field) else {
        return Ok(Err(UnknownReason::NotRequested));
    };
    let Property::Requested {
        sensitivity,
        state,
        evidence: source,
        ..
    } = property
    else {
        return Ok(Err(UnknownReason::NotRequested));
    };
    let source_state = record_evidence(snapshot, source, evidence)?;
    if *sensitivity == Sensitivity::Sensitive {
        return Ok(Err(UnknownReason::RedactedProperty));
    }
    if let Some(reason) = source_state {
        return Ok(Err(reason));
    }
    Ok(match state {
        Availability::Known { value } => Ok((node, value)),
        Availability::Unknown { .. } => Err(UnknownReason::UnknownProperty),
        Availability::Unsupported { .. } => Err(UnknownReason::UnsupportedProperty),
        Availability::Redacted {} => Err(UnknownReason::RedactedProperty),
    })
}

// Only directional, evidenced paths are traversed. BFS preserves input order;
// no implicit inverses or guessed conversion between equal-named units.
fn path<'a>(
    snapshot: &Snapshot,
    surface: &Identity,
    from: &Space,
    to: &Space,
    initial: Option<&'a Transform>,
    extra: &'a [Transform],
) -> Available<Vec<&'a Transform>> {
    if from == to {
        return Ok(Ok(Vec::new()));
    }
    let transforms: Vec<_> = initial.into_iter().chain(extra).collect();
    let mut queue = vec![(from.clone(), Vec::<&Transform>::new())];
    let mut visited = vec![from.clone()];
    let mut cursor = 0;
    while cursor < queue.len() {
        let (space, prefix) = queue[cursor].clone();
        cursor += 1;
        for transform in &transforms {
            if transform.from != space {
                continue;
            }
            if transform.target != snapshot.context.target
                || &transform.surface != surface
                || transform.environment_revision != snapshot.context.environment_revision
            {
                continue;
            }
            let [a, b, c, d, _, _] = transform.affine;
            if !transform.affine.iter().all(|v| v.is_finite())
                || finite(finite(a * d)? - finite(b * c)?)? == 0.0
            {
                return Err(GeometryError::InvalidTransform);
            }
            if visited.contains(&transform.to) {
                continue;
            }
            let mut next = prefix.clone();
            next.push(transform);
            if &transform.to == to {
                return Ok(Ok(next));
            }
            visited.push(transform.to.clone());
            queue.push((transform.to.clone(), next));
        }
    }
    Ok(Err(UnknownReason::MissingTransform))
}

fn apply(point: &Point, affine: [f64; 6]) -> Result<Point, GeometryError> {
    let [a, b, c, d, e, f] = affine;
    Ok(Point {
        x: finite(finite(finite(a * point.x)? + finite(c * point.y)?)? + e)?,
        y: finite(finite(finite(b * point.x)? + finite(d * point.y)?)? + f)?,
    })
}

pub(crate) fn rect(
    snapshot: &Snapshot,
    anchor: &Anchor,
    context: &EvaluationContext<'_>,
    evidence: &mut Vec<Evidence>,
) -> Available<Rect> {
    let (node, value) = match property(snapshot, anchor, field(anchor.frame_kind), evidence)? {
        Ok(value) => value,
        Err(reason) => return Ok(Err(reason)),
    };
    let Value::Geometry(geometry) = value else {
        return Err(GeometryError::InvalidRule);
    };
    if geometry.frame_kind != anchor.frame_kind {
        return Ok(Err(UnknownReason::FrameKindMismatch));
    }
    if geometry.coordinate_space != anchor.coordinate_space {
        return Ok(Err(UnknownReason::MissingTransform));
    }
    let Shape::Rect(rect) = &geometry.shape else {
        return Ok(Err(UnknownReason::UnsupportedShape));
    };
    let right = finite(rect.x + rect.width)?;
    let bottom = finite(rect.y + rect.height)?;
    let mut points = [
        Point {
            x: rect.x,
            y: rect.y,
        },
        Point {
            x: right,
            y: rect.y,
        },
        Point {
            x: right,
            y: bottom,
        },
        Point {
            x: rect.x,
            y: bottom,
        },
    ];
    let initial = match &geometry.transform {
        TransformState::Known { transform } => Some(transform.as_ref()),
        _ => None,
    };
    let transforms = match path(
        snapshot,
        &node.surface,
        &geometry.coordinate_space,
        context.space,
        initial,
        context.transforms,
    )? {
        Ok(value) => value,
        Err(reason) => return Ok(Err(reason)),
    };
    for transform in transforms {
        if let Some(reason) = record_evidence(snapshot, &transform.evidence, evidence)? {
            return Ok(Err(reason));
        }
        for point in &mut points {
            *point = apply(point, transform.affine)?;
        }
    }
    // Never turn a rotated/sheared polygon's bounding envelope into containment
    // or intersection truth. Axis-preserving flips/quarter turns remain exact.
    for i in 0..4 {
        let p = &points[i];
        let q = &points[(i + 1) % 4];
        if p.x != q.x && p.y != q.y {
            return Ok(Err(UnknownReason::UnsupportedShape));
        }
    }
    let left = points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
    let right = points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
    let low = points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
    let high = points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
    Ok(Ok(Rect {
        x: left,
        y: low,
        width: finite(right - left)?,
        height: finite(high - low)?,
    }))
}

pub(crate) fn baseline(
    snapshot: &Snapshot,
    anchor: &Anchor,
    context: &EvaluationContext<'_>,
    evidence: &mut Vec<Evidence>,
) -> Available<f64> {
    let (node, value) = match property(snapshot, anchor, Field::Baseline, evidence)? {
        Ok(value) => value,
        Err(reason) => return Ok(Err(reason)),
    };
    let Value::Baseline { coordinate, space } = value else {
        return Err(GeometryError::InvalidRule);
    };
    finite(*coordinate)?;
    if space != &anchor.coordinate_space {
        return Ok(Err(UnknownReason::MissingTransform));
    }
    let transforms = match path(
        snapshot,
        &node.surface,
        space,
        context.space,
        None,
        context.transforms,
    )? {
        Ok(value) => value,
        Err(reason) => return Ok(Err(reason)),
    };
    let mut y = *coordinate;
    for transform in transforms {
        if let Some(reason) = record_evidence(snapshot, &transform.evidence, evidence)? {
            return Ok(Err(reason));
        }
        // Baseline is a y coordinate, not a point with a measured x coordinate.
        if transform.affine[1] != 0.0 {
            return Ok(Err(UnknownReason::UnsupportedShape));
        }
        y = finite(finite(transform.affine[3] * y)? + transform.affine[5])?;
    }
    Ok(Ok(y))
}
