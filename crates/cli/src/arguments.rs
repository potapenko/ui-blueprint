use crate::Failure;
use std::{ffi::OsString, path::PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Command {
    Check,
    Measure,
}
pub(crate) struct Arguments {
    pub command: Command,
    pub snapshot: PathBuf,
    pub expectation: PathBuf,
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
        let space = space
            .filter(|s| !s.is_empty() && s.chars().count() <= 256)
            .ok_or(missing)?;
        Ok(Self {
            command,
            snapshot: snapshot.ok_or(missing)?,
            expectation: expectation.ok_or(missing)?,
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
