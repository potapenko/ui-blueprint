use crate::Failure;
use std::{ffi::OsString, path::PathBuf};
use uiblueprint_schema::model::Projection;

pub(crate) struct ObserveArguments {
    pub connection: PathBuf,
    pub request: PathBuf,
    pub worker: PathBuf,
    pub max_input: usize,
    pub max_output: usize,
}
impl ObserveArguments {
    pub fn parse(mut args: impl Iterator<Item = OsString>) -> Result<Self, Failure> {
        let (mut connection, mut request, mut worker, mut max_input, mut max_output) =
            (None, None, None, None, None);
        let invalid = Failure::invalid("invalid_arguments");
        while let Some(flag) = args.next() {
            let value = args.next().ok_or(invalid)?;
            match flag.to_str() {
                Some("--connection") if connection.is_none() => {
                    connection = Some(PathBuf::from(value))
                }
                Some("--request") if request.is_none() => request = Some(PathBuf::from(value)),
                Some("--worker") if worker.is_none() => worker = Some(PathBuf::from(value)),
                Some("--max-input-bytes") if max_input.is_none() => max_input = Some(limit(value)?),
                Some("--max-output-bytes") if max_output.is_none() => {
                    max_output = Some(limit(value)?)
                }
                _ => return Err(invalid),
            }
        }
        Ok(Self {
            connection: connection.ok_or(invalid)?,
            request: request.ok_or(invalid)?,
            worker: worker.ok_or(invalid)?,
            max_input: max_input.ok_or(invalid)?,
            max_output: max_output.ok_or(invalid)?,
        })
    }
}

pub(crate) struct InspectArguments {
    pub snapshot: PathBuf,
    pub reference: String,
    pub view: Projection,
    pub max_input: usize,
    pub max_output: usize,
    pub json: bool,
}
impl InspectArguments {
    pub fn parse(mut args: impl Iterator<Item = OsString>) -> Result<Self, Failure> {
        let (mut snapshot, mut reference, mut view, mut max_input, mut max_output) =
            (None, None, None, None, None);
        let invalid = Failure::invalid("invalid_arguments");
        let mut json = false;
        while let Some(flag) = args.next() {
            if flag == "--json" {
                if json {
                    return Err(invalid);
                }
                json = true;
                continue;
            }
            let value = args.next().ok_or(invalid)?;
            match flag.to_str() {
                Some("--snapshot") if snapshot.is_none() => snapshot = Some(PathBuf::from(value)),
                Some("--ref") if reference.is_none() => {
                    reference = Some(value.into_string().map_err(|_| invalid)?);
                }
                Some("--view") if view.is_none() => {
                    view = Some(match value.to_str() {
                        Some("interaction") => Projection::Interaction,
                        Some("design") => Projection::Design,
                        _ => return Err(Failure::unsupported("unsupported_view")),
                    });
                }
                Some("--max-input-bytes") if max_input.is_none() => max_input = Some(limit(value)?),
                Some("--max-output-bytes") if max_output.is_none() => {
                    max_output = Some(limit(value)?)
                }
                _ => return Err(invalid),
            }
        }
        Ok(Self {
            snapshot: snapshot.ok_or(invalid)?,
            reference: reference.ok_or(invalid)?,
            view: view.ok_or(invalid)?,
            max_input: max_input.ok_or(invalid)?,
            max_output: max_output.ok_or(invalid)?,
            json,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Command {
    Check,
    Measure,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ResultVersion {
    Core,
    Analysis,
}
pub(crate) enum QueryFile {
    Expectation(PathBuf),
    Query(PathBuf),
}
pub(crate) struct Arguments {
    pub command: Command,
    pub snapshot: PathBuf,
    pub query: QueryFile,
    pub evaluation: Option<PathBuf>,
    pub result_version: ResultVersion,
    pub space: String,
    pub max_input: usize,
    pub max_output: usize,
    pub json: bool,
}
impl Arguments {
    pub fn parse(args: Vec<OsString>) -> Result<Self, Failure> {
        let mut args = args.into_iter();
        let command = match args.next().as_deref().and_then(|s| s.to_str()) {
            Some("check") => Command::Check,
            Some("measure") => Command::Measure,
            Some(_) => return Err(Failure::unsupported("unsupported_command")),
            None => return Err(Failure::invalid("invalid_arguments")),
        };
        let mut snapshot = None;
        let mut expectation = None;
        let mut query = None;
        let mut evaluation = None;
        let mut result_version = None;
        let mut space = None;
        let mut max_input = None;
        let mut max_output = None;
        let mut json = false;
        while let Some(flag) = args.next() {
            if flag == "--json" {
                if json {
                    return Err(Failure::invalid("invalid_arguments"));
                }
                json = true;
                continue;
            }
            let value = args.next().ok_or(Failure::invalid("invalid_arguments"))?;
            match flag.to_str() {
                Some("--snapshot") if snapshot.is_none() => snapshot = Some(PathBuf::from(value)),
                Some("--expectation") if expectation.is_none() => {
                    expectation = Some(PathBuf::from(value))
                }
                Some("--query") if query.is_none() => query = Some(PathBuf::from(value)),
                Some("--evaluation") if evaluation.is_none() => {
                    evaluation = Some(PathBuf::from(value))
                }
                Some("--result-version") if result_version.is_none() => {
                    result_version = Some(value)
                }
                Some("--space") if space.is_none() => {
                    space = Some(
                        value
                            .into_string()
                            .map_err(|_| Failure::invalid("invalid_arguments"))?,
                    )
                }
                Some("--max-input-bytes") if max_input.is_none() => max_input = Some(limit(value)?),
                Some("--max-output-bytes") if max_output.is_none() => {
                    max_output = Some(limit(value)?)
                }
                _ => return Err(Failure::invalid("invalid_arguments")),
            }
        }
        let missing = Failure::invalid("invalid_arguments");
        let query = match (command, expectation, query) {
            (_, Some(path), None) => QueryFile::Expectation(path),
            (Command::Measure, None, Some(path)) => QueryFile::Query(path),
            _ => return Err(missing),
        };
        if result_version.is_some() && !json {
            return Err(missing);
        }
        let result_version = match result_version.as_ref().and_then(|v| v.to_str()) {
            Some("0.2.0") => ResultVersion::Analysis,
            Some("0.1.0") if command == Command::Check => ResultVersion::Core,
            None if result_version.is_none() => {
                if command == Command::Check {
                    ResultVersion::Core
                } else {
                    ResultVersion::Analysis
                }
            }
            _ => return Err(Failure::unsupported("unsupported_result_version")),
        };
        let space = space
            .filter(|s| !s.is_empty() && s.chars().count() <= 256)
            .ok_or(missing)?;
        Ok(Self {
            command,
            snapshot: snapshot.ok_or(missing)?,
            query,
            evaluation,
            result_version,
            space,
            max_input: max_input.ok_or(missing)?,
            max_output: max_output.ok_or(missing)?,
            json,
        })
    }
}
pub(crate) fn limit(value: OsString) -> Result<usize, Failure> {
    value
        .to_str()
        .filter(|s| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()))
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|n| *n > 0 && *n < usize::MAX)
        .ok_or(Failure::invalid("invalid_arguments"))
}
