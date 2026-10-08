use crate::{
    Failure,
    arguments::{Arguments, DiffArguments, DiffMode, InspectArguments, QueryFile},
};
use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};
use uiblueprint_schema::{analysis::*, model::*};

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

pub(crate) struct Loaded {
    pub snapshot: Snapshot,
    pub query: GeometryQuery,
    pub expectation: Option<Expectation>,
    pub evaluation: EvaluationInput,
}

pub(crate) fn load_inspect(args: &InspectArguments) -> Result<(Snapshot, SourceKey), Failure> {
    load_selected(&args.snapshot, &args.reference, args.max_input)
}
pub(crate) fn load_selected(
    path: &Path,
    selector: &str,
    max_input: usize,
) -> Result<(Snapshot, SourceKey), Failure> {
    let mut remaining = max_input
        .checked_sub(selector.len())
        .ok_or(Failure::invalid("input_limit"))?;
    let reference = serde_json::from_str::<SourceKey>(selector)
        .map_err(|_| Failure::invalid("invalid_input"))?;
    Ok((read_snapshot(path, &mut remaining, max_input)?, reference))
}
pub(crate) fn read_snapshot(
    path: &Path,
    remaining: &mut usize,
    limit: usize,
) -> Result<Snapshot, Failure> {
    let document = Document::from_json(&read(path, remaining)?, limit)
        .map_err(|_| Failure::invalid("invalid_input"))?;
    let snapshot = match document.artifact {
        Artifact::Snapshot(snapshot) => snapshot,
        Artifact::ChannelResponse(response) => match response.result {
            ChannelResult::Observed(snapshot) => snapshot,
            ChannelResult::Failed(_) => return Err(Failure::invalid("invalid_input")),
        },
        _ => return Err(Failure::invalid("invalid_input")),
    };
    Ok(*snapshot)
}
pub(crate) fn load_diff(args: &DiffArguments) -> Result<(Snapshot, Snapshot), Failure> {
    let mut remaining = args.max_input;
    let before = read_snapshot(&args.before, &mut remaining, args.max_input)?;
    let after = read_snapshot(&args.after, &mut remaining, args.max_input)?;
    Ok((before, after))
}
pub(crate) fn load(args: &Arguments) -> Result<Loaded, Failure> {
    let mut remaining = args.max_input;
    let snapshot = read_snapshot(&args.snapshot, &mut remaining, args.max_input)?;
    let (query, expectation) = match &args.query {
        QueryFile::Expectation(path) => {
            let document = Document::from_json(&read(path, &mut remaining)?, args.max_input)
                .map_err(|_| Failure::invalid("invalid_input"))?;
            let Artifact::Expectation(expectation) = document.artifact else {
                return Err(Failure::invalid("invalid_input"));
            };
            let query = GeometryQuery::from_expectation(&expectation)
                .ok_or(Failure::unsupported("unsupported_rule"))?;
            (query, Some(*expectation))
        }
        QueryFile::Query(path) => {
            let document =
                AnalysisDocument::from_json(&read(path, &mut remaining)?, args.max_input)
                    .map_err(|_| Failure::invalid("invalid_input"))?;
            let AnalysisArtifact::GeometryQuery(query) = document.artifact else {
                return Err(Failure::invalid("invalid_input"));
            };
            (*query, None)
        }
    };
    let supplied = read_evaluation(args.evaluation.as_deref(), &mut remaining, args.max_input)?;
    let evaluation = evaluation_for(&snapshot, &query.anchors, supplied, &args.space)?;
    Ok(Loaded {
        snapshot,
        query,
        expectation,
        evaluation,
    })
}

fn transform_spaces<'a>(transform: &'a TransformState, spaces: &mut Vec<&'a Space>) {
    if let TransformState::Known { transform } = transform {
        spaces.push(&transform.from);
        spaces.push(&transform.to);
    }
}
fn space(
    snapshot: &Snapshot,
    anchors: &[Anchor],
    evaluation: Option<&EvaluationInput>,
    id: &str,
) -> Result<Space, Failure> {
    let mut candidates: Vec<_> = anchors.iter().map(|a| &a.coordinate_space).collect();
    for node in &snapshot.nodes {
        for property in &node.properties {
            match property.known() {
                Some(Value::Geometry(geometry)) => {
                    candidates.push(&geometry.coordinate_space);
                    transform_spaces(&geometry.transform, &mut candidates);
                }
                Some(Value::Baseline { space, .. }) => candidates.push(space),
                _ => (),
            }
        }
    }
    for capture in &snapshot.captures {
        transform_spaces(&capture.crop_transform, &mut candidates);
    }
    if let Some(evaluation) = evaluation {
        for transform in &evaluation.transforms {
            candidates.push(&transform.from);
            candidates.push(&transform.to);
        }
    }
    let mut matches = candidates.into_iter().filter(|s| s.id.0 == id);
    let selected = matches.next().ok_or(Failure::invalid("unknown_space"))?;
    if matches.any(|s| s != selected) {
        return Err(Failure::invalid("ambiguous_space"));
    }
    Ok(selected.clone())
}

pub(crate) struct GeometryDiffInput {
    pub before: Snapshot,
    pub after: Snapshot,
    pub key: SourceKey,
    pub before_evaluation: EvaluationInput,
    pub after_evaluation: EvaluationInput,
}
pub(crate) fn load_geometry_diff(args: &DiffArguments) -> Result<GeometryDiffInput, Failure> {
    let DiffMode::Geometry {
        reference,
        space,
        before_evaluation,
        after_evaluation,
        ..
    } = &args.mode
    else {
        return Err(Failure::invalid("invalid_arguments"));
    };
    let mut remaining = args
        .max_input
        .checked_sub(reference.len())
        .ok_or(Failure::invalid("input_limit"))?;
    let key = serde_json::from_str::<SourceKey>(reference)
        .map_err(|_| Failure::invalid("invalid_input"))?;
    let before = read_snapshot(&args.before, &mut remaining, args.max_input)?;
    let after = read_snapshot(&args.after, &mut remaining, args.max_input)?;
    let old = read_evaluation(before_evaluation.as_deref(), &mut remaining, args.max_input)?;
    let new = read_evaluation(after_evaluation.as_deref(), &mut remaining, args.max_input)?;
    let before_evaluation = evaluation_for(&before, &[], old, space)?;
    let after_evaluation = evaluation_for(&after, &[], new, space)?;
    Ok(GeometryDiffInput {
        before,
        after,
        key,
        before_evaluation,
        after_evaluation,
    })
}
fn read_evaluation(
    path: Option<&Path>,
    remaining: &mut usize,
    limit: usize,
) -> Result<Option<EvaluationInput>, Failure> {
    let Some(path) = path else {
        return Ok(None);
    };
    let doc = AnalysisDocument::from_json(&read(path, remaining)?, limit)
        .map_err(|_| Failure::invalid("invalid_input"))?;
    let AnalysisArtifact::EvaluationInput(input) = doc.artifact else {
        return Err(Failure::invalid("invalid_input"));
    };
    Ok(Some(*input))
}
fn evaluation_for(
    snapshot: &Snapshot,
    anchors: &[Anchor],
    supplied: Option<EvaluationInput>,
    id: &str,
) -> Result<EvaluationInput, Failure> {
    let selected = space(snapshot, anchors, supplied.as_ref(), id)?;
    let input = if let Some(input) = supplied {
        if input.result_space != selected {
            return Err(Failure::invalid("invalid_input"));
        }
        input
    } else {
        EvaluationInput {
            snapshot_id: snapshot.id.clone(),
            revision: snapshot.revision,
            context: snapshot.context.clone(),
            result_space: selected,
            transforms: vec![],
            conditions: None,
        }
    };
    validate_bound_evaluation(snapshot, &input).map_err(|_| Failure::invalid("invalid_input"))?;
    Ok(input)
}
