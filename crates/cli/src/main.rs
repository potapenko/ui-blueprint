//! Local saved-data analysis. Never attaches to a runtime or loads input URLs.
#![forbid(unsafe_code)]

mod arguments;
mod export;
mod input;
mod output;

use arguments::{Arguments, Command, ResultVersion};
use std::{
    io::{self, Write},
    process::ExitCode,
};
use uiblueprint_engine::{self as engine, MeasurementResult};
use uiblueprint_schema::{analysis::*, model::*, validation};

const HELP: &str = "UI Blueprint: local saved-snapshot geometry and engineering export\n\
Usage: uiblueprint check|measure --snapshot FILE --expectation FILE --space SPACE_ID --max-input-bytes N --max-output-bytes N [--evaluation FILE] [--json --result-version VERSION]\n\
Inputs are canonical Snapshot/Expectation Documents. Bounds are explicit; no live collection.\n\
Measure also accepts --query FILE instead of --expectation. Measure JSON is analysis0.2; check JSON defaults to core0.1, with explicit0.2 for converted/conditional results.\n\
Export: uiblueprint imagegen-prompt --brief FILE --out NEW_DIRECTORY --max-input-bytes N --max-output-bytes N --max-components N --max-views N --components-per-detail N [--purpose MODE] [--profile blue-engineering] [--json]\n\
Export requires a complete DrawingBrief with canonical Snapshot or explicit ProposedLayout, public document metadata and caller limits. No model or live collection.\n\
Exits: 0 package written/pass/known; 1 IO/internal; 2 invalid/limit; 3 fail; 4 unknown; 5 unsupported/contract gap.\n";

#[derive(Clone, Copy, Debug)]
struct Failure {
    code: &'static str,
    exit: u8,
}
impl Failure {
    const fn invalid(code: &'static str) -> Self {
        Self { code, exit: 2 }
    }
    const fn unsupported(code: &'static str) -> Self {
        Self { code, exit: 5 }
    }
    const fn io() -> Self {
        Self {
            code: "io_error",
            exit: 1,
        }
    }
}

fn execute(args: Arguments) -> Result<(Vec<u8>, u8), Failure> {
    let loaded = input::load(&args)?;
    let input::Loaded {
        snapshot,
        query,
        expectation,
        evaluation,
    } = loaded;
    match args.command {
        Command::Check => {
            let expectation = expectation.ok_or(Failure::invalid("invalid_arguments"))?;
            if args.json
                && args.result_version == ResultVersion::Core
                && (!evaluation.transforms.is_empty()
                    || evaluation.conditions.is_some()
                    || query
                        .anchors
                        .iter()
                        .any(|a| a.coordinate_space != evaluation.result_space))
            {
                return Err(Failure::unsupported("unsupported_result_version"));
            }
            let result = engine::check_bound(&snapshot, &expectation, &evaluation)
                .map_err(|_| Failure::invalid("invalid_geometry"))?;
            let exit = match result.finding.status {
                CheckStatus::Pass => 0,
                CheckStatus::Fail => 3,
                CheckStatus::Unknown => 4,
            };
            let bytes = if args.json && args.result_version == ResultVersion::Core {
                validation::validate_finding(&snapshot, &expectation, &result.finding)
                    .map_err(|_| Failure::unsupported("unsupported_result_version"))?;
                output::json(snapshot, expectation, result.finding, args.max_output)?
            } else if args.json {
                output::analysis_json(
                    AnalysisDocument {
                        schema_version: AnalysisVersion::CURRENT,
                        artifact: AnalysisArtifact::GeometryCheck(Box::new(GeometryCheckCase {
                            snapshot,
                            expectation,
                            evaluation,
                            measurement: result.measurement,
                            finding: result.finding,
                        })),
                    },
                    args.max_output,
                )?
            } else {
                output::compact(
                    &args,
                    &snapshot,
                    &query,
                    Some(&expectation),
                    &evaluation,
                    &result.measurement,
                    Some(result.finding.status),
                )?
            };
            Ok((bytes, exit))
        }
        Command::Measure => {
            let result = engine::measure_query_bound(&snapshot, &query, &evaluation)
                .map_err(|_| Failure::invalid("invalid_geometry"))?;
            let exit = if matches!(result, MeasurementResult::Known { .. }) {
                0
            } else {
                4
            };
            let bytes = if args.json {
                output::analysis_json(
                    AnalysisDocument {
                        schema_version: AnalysisVersion::CURRENT,
                        artifact: AnalysisArtifact::Measurement(Box::new(MeasurementCase {
                            snapshot,
                            query,
                            evaluation,
                            result,
                        })),
                    },
                    args.max_output,
                )?
            } else {
                output::compact(
                    &args,
                    &snapshot,
                    &query,
                    expectation.as_ref(),
                    &evaluation,
                    &result,
                    None,
                )?
            };
            Ok((bytes, exit))
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let result = if args.len() == 1 && args[0] == "--help" {
        Ok((HELP.as_bytes().to_vec(), 0))
    } else if args.first().is_some_and(|arg| arg == "imagegen-prompt") {
        export::execute(args.into_iter().skip(1).collect())
    } else {
        Arguments::parse(args).and_then(execute)
    };
    let (bytes, exit) = match result {
        Ok(value) => value,
        Err(failure) => {
            let _ = writeln!(io::stderr().lock(), "{}", failure.code);
            return ExitCode::from(failure.exit);
        }
    };
    if io::stdout().lock().write_all(&bytes).is_err() {
        let _ = writeln!(io::stderr().lock(), "io_error");
        return ExitCode::from(1);
    }
    ExitCode::from(exit)
}
