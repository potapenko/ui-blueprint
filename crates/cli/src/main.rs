//! Bounded local analysis and explicit guarded runtime callers.
#![forbid(unsafe_code)]

mod action;
mod arguments;
#[cfg(all(target_os = "macos", any(feature = "macos", feature = "web")))]
mod connection;
mod export;
mod input;
mod observe;
mod output;

use arguments::{
    ActionArguments, Arguments, Command, DiffArguments, InspectArguments, ResultVersion,
};
use std::{
    io::{self, Write},
    process::ExitCode,
};
use uiblueprint_engine::{self as engine, MeasurementResult};
use uiblueprint_schema::{analysis::*, model::*, validation};

const HELP: &str = "UI Blueprint: local saved-snapshot geometry and engineering export\n\
Diff: uiblueprint diff --before FILE --after FILE --max-input-bytes N --max-output-bytes N --max-entries N [--json]\n\
Diff reports recorded node/property differences; missing records do not imply deletion. Entry cap0 is allowed; complete report0, truncated/context mismatch4.\n\
Observe: uiblueprint observe --connection FILE --request FILE --worker ABSOLUTE_PATH --max-input-bytes N --max-output-bytes N\n\
Observe needs a selected macos/web build and explicit trusted connection; emits committed canonical NDJSON and cleans only owned workers/helpers.\n\
Action: uiblueprint action prepare --connection FILE --snapshot FILE --request FILE --worker ABSOLUTE_PATH --max-input-bytes N --max-output-bytes N [--json]\n\
Action: uiblueprint action execute --connection FILE --plan FILE --request FILE --worker ABSOLUTE_PATH --max-input-bytes N --max-output-bytes N [--json]\n\
Actions support one Web SetChecked through a selected web build; saved plans are freshly revalidated, delivery and verified source state are separate.\n\
Usage: uiblueprint check|measure --snapshot FILE --expectation FILE --space SPACE_ID --max-input-bytes N --max-output-bytes N [--evaluation FILE] [--json --result-version VERSION]\n\
Inputs are canonical Snapshot/Expectation Documents. Bounds are explicit; no live collection.\n\
Measure also accepts --query FILE instead of --expectation. Measure JSON is analysis0.2; check JSON defaults to core0.1, with explicit0.2 for converted/conditional results.\n\
Inspect: uiblueprint inspect --snapshot FILE --ref SOURCE_KEY_JSON --view interaction|design --max-input-bytes N --max-output-bytes N [--json]\n\
Inspect accepts a saved Snapshot or observed ChannelResponse; selector is canonical {namespace,key} JSON. No live revalidation; JSON uses CLI inspection envelope1.0.0 with unchanged source Snapshot.\n\
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

fn execute_inspect(args: InspectArguments) -> Result<(Vec<u8>, u8), Failure> {
    let (snapshot, reference) = input::load_inspect(&args)?;
    let view = engine::scope::relation_neighbors(
        &snapshot,
        &reference,
        engine::scope::NeighborLimits {
            max_relations: snapshot.relations.len(),
        },
    )
    .map_err(|error| match error {
        engine::scope::ScopeError::MissingSeed => Failure {
            code: "target_unresolved",
            exit: 4,
        },
        engine::scope::ScopeError::InvalidSnapshot(_) => Failure::invalid("invalid_input"),
        engine::scope::ScopeError::AllocationFailure => Failure::io(),
    })?;
    Ok((output::inspect(&args, &view)?, 0))
}
fn execute_diff(args: DiffArguments) -> Result<(Vec<u8>, u8), Failure> {
    let (before, after) = input::load_diff(&args)?;
    let result = engine::diff::compare_recorded(
        &before,
        &after,
        engine::diff::DiffLimits {
            max_entries: args.max_entries,
        },
    )
    .map_err(|error| match error {
        engine::diff::DiffError::InvalidSnapshot(_) => Failure::invalid("invalid_input"),
        engine::diff::DiffError::IncompatibleContext => Failure {
            code: "context_mismatch",
            exit: 4,
        },
        engine::diff::DiffError::Capacity => Failure::io(),
    })?;
    let exit = if result.omitted_entries == 0 { 0 } else { 4 };
    Ok((output::recorded_diff(&args, &result)?, exit))
}

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.first().is_some_and(|arg| arg == "action") {
        let result = ActionArguments::parse(args.into_iter().skip(1))
            .and_then(|args| action::execute(args, &mut io::stdout().lock()));
        return match result {
            Ok(code) => ExitCode::from(code),
            Err(error) => {
                let _ = writeln!(io::stderr().lock(), "{}", error.code);
                ExitCode::from(error.exit)
            }
        };
    }
    if args.first().is_some_and(|arg| arg == "observe") {
        let result = arguments::ObserveArguments::parse(args.into_iter().skip(1))
            .and_then(|args| observe::execute(args, &mut io::stdout().lock()));
        return match result {
            Ok(code) => ExitCode::from(code),
            Err(error) => {
                let _ = writeln!(io::stderr().lock(), "{}", error.code);
                ExitCode::from(error.exit)
            }
        };
    }
    let result = if args.len() == 1 && args[0] == "--help" {
        Ok((HELP.as_bytes().to_vec(), 0))
    } else if args.first().is_some_and(|arg| arg == "imagegen-prompt") {
        export::execute(args.into_iter().skip(1).collect())
    } else if args.first().is_some_and(|arg| arg == "inspect") {
        InspectArguments::parse(args.into_iter().skip(1)).and_then(execute_inspect)
    } else if args.first().is_some_and(|arg| arg == "diff") {
        DiffArguments::parse(args.into_iter().skip(1)).and_then(execute_diff)
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
