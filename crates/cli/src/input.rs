use crate::{Failure, arguments::Arguments};
use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};
use uiblueprint_schema::model::*;

pub(crate) fn read(path: &Path, remaining: &mut usize) -> Result<Vec<u8>, Failure> {
    let metadata = fs::metadata(path).map_err(|_| Failure::io())?;
    if !metadata.is_file() {
        return Err(Failure::invalid("invalid_input_file"));
    }
    if metadata.len() > *remaining as u64 {
        return Err(Failure::invalid("input_limit"));
    }
    let mut file = File::open(path)
        .map_err(|_| Failure::io())?
        .take(*remaining as u64 + 1);
    let mut bytes = Vec::new();
    let mut buffer = [0; 8192];
    loop {
        let count = file.read(&mut buffer).map_err(|_| Failure::io())?;
        if count == 0 {
            break;
        }
        if count > remaining.saturating_sub(bytes.len()) {
            return Err(Failure::invalid("input_limit"));
        }
        bytes.try_reserve(count).map_err(|_| Failure::io())?;
        bytes.extend_from_slice(&buffer[..count]);
    }
    *remaining -= bytes.len();
    Ok(bytes)
}

pub(crate) fn load(args: &Arguments) -> Result<(Snapshot, Expectation), Failure> {
    let mut remaining = args.max_input;
    let snapshot = Document::from_json(&read(&args.snapshot, &mut remaining)?, args.max_input)
        .map_err(|_| Failure::invalid("invalid_input"))?;
    let expectation =
        Document::from_json(&read(&args.expectation, &mut remaining)?, args.max_input)
            .map_err(|_| Failure::invalid("invalid_input"))?;
    match (snapshot.artifact, expectation.artifact) {
        (Artifact::Snapshot(snapshot), Artifact::Expectation(expectation)) => {
            Ok((*snapshot, *expectation))
        }
        _ => Err(Failure::invalid("invalid_input")),
    }
}

pub(crate) fn space(
    snapshot: &Snapshot,
    expectation: &Expectation,
    id: &str,
) -> Result<Space, Failure> {
    let Rule::Geometry { anchors, .. } = &expectation.rule else {
        return Err(Failure::unsupported("unsupported_rule"));
    };
    let mut candidates = Vec::new();
    for anchor in anchors {
        candidates.push(&anchor.coordinate_space);
        let field = match anchor.frame_kind {
            FrameKind::LayoutBounds => Field::LayoutBounds,
            FrameKind::AccessibilityBounds => Field::AccessibilityBounds,
            FrameKind::HitRegion => Field::HitRegion,
            FrameKind::VisibleRegion => Field::VisibleRegion,
            FrameKind::PaintBounds => Field::PaintBounds,
        };
        if let Some(node) = snapshot.nodes.iter().find(|n| n.key == anchor.element)
            && let Some(Value::Geometry(geometry)) = node
                .properties
                .iter()
                .find(|p| p.field() == field)
                .and_then(Property::known)
        {
            candidates.push(&geometry.coordinate_space);
            if let TransformState::Known { transform } = &geometry.transform {
                candidates.push(&transform.to);
            }
        }
    }
    let mut matches = candidates.into_iter().filter(|s| s.id.0 == id);
    let selected = matches.next().ok_or(Failure::invalid("unknown_space"))?;
    if matches.any(|s| s != selected) {
        return Err(Failure::invalid("ambiguous_space"));
    }
    Ok(selected.clone())
}
