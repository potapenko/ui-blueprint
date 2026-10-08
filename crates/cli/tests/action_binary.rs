#![cfg(all(target_os = "macos", feature = "web"))]
//! Pre-dispatch public syntax/strict connection/budget checks; no SDK or UI run.
#[path = "../../host/tests/support/web_worker_data.rs"]
#[allow(dead_code)]
mod data;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};
use uiblueprint_schema::{SchemaVersion, model::*};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Case {
    directory: PathBuf,
    input: usize,
}
impl Case {
    fn new(command: &str) -> Self {
        let source = Document::from_json(
            include_bytes!("../../../fixtures/golden/ENV-ACTION-VALID.json"),
            65536,
        )
        .unwrap();
        let Artifact::Action(case) = &source.artifact else {
            panic!("action")
        };
        let context = case.snapshot.context.clone();
        let mut descriptor = data::descriptor();
        let Artifact::Session(session) = &mut descriptor.artifact else {
            panic!("session")
        };
        session.session_id = context.session_id.clone();
        session.target = context.target.clone();
        session.plugin = context.plugin.clone();
        session.surfaces = context.surfaces.clone();
        session.allowed_scopes = vec![context.scope_id.clone()];
        let mut setup = data::setup("ws://127.0.0.1:9/never-connected".into());
        setup.surface = context.surfaces[0].clone();
        let l = data::limits();
        let connection = serde_json::json!({"connection_version":"1.0.0", "target":context.target, "session":session, "attach_deadline_ms":100,
            "host_limits":{"workers":l.workers,"worker_bytes":l.worker_bytes,"publication_reserve":l.publication_reserve,"bootstrap_bytes":l.bootstrap_bytes,"parent_bytes":l.parent_bytes,"input_bytes":l.input_bytes,"ingress_bytes":l.ingress_bytes,"output_bytes":l.output_bytes,"request_output_bytes":l.request_output_bytes,"completion_groups":l.completion_groups,"control_bytes":l.control_bytes,"cleanup_ms":l.cleanup_ms,"retained_domain_bytes":l.retained_domain_bytes,"retained_per_worker":l.retained_per_worker,"main_stack_bytes":l.main_stack_bytes,"watchdog_stack_bytes":l.watchdog_stack_bytes},
            "provider":{"backend":"web","setup":setup,"selection":data::selection(false)}});
        let request = Document {
            schema_version: SchemaVersion::CURRENT,
            artifact: Artifact::Request(Box::new(Request {
                request_id: Id("cli-action-input-test".into()),
                clock_domain: Id("saved-request-clock".into()),
                context,
                limits: Limits {
                    max_elements: 32,
                    max_depth: 8,
                    max_output_bytes: 65536,
                    deadline_ms: 100,
                },
                freshness_policy: FreshnessPolicy::CurrentRequired,
                operation: if command == "prepare" {
                    Operation::Prepare {
                        action: case.action.clone(),
                    }
                } else {
                    Operation::Act {
                        action: case.action.clone(),
                    }
                },
            })),
        };
        request.validate().unwrap();
        let source = if command == "prepare" {
            Document {
                schema_version: SchemaVersion::CURRENT,
                artifact: Artifact::Snapshot(Box::new(case.snapshot.clone())),
            }
        } else {
            source
        };
        let directory = std::env::temp_dir().join(format!(
            "uib-cli-actions-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&directory).unwrap();
        let mut input = 0;
        for (name, bytes) in [
            ("connection.json", serde_json::to_vec(&connection).unwrap()),
            ("request.json", serde_json::to_vec(&request).unwrap()),
            ("source.json", serde_json::to_vec(&source).unwrap()),
        ] {
            input += bytes.len();
            fs::write(directory.join(name), bytes).unwrap();
        }
        Self { directory, input }
    }
    fn run(&self, command: &str, input: usize, output: usize, extra: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_uiblueprint"))
            .args(["action", command, "--connection"])
            .arg(self.directory.join("connection.json"))
            .arg(if command == "prepare" {
                "--snapshot"
            } else {
                "--plan"
            })
            .arg(self.directory.join("source.json"))
            .arg("--request")
            .arg(self.directory.join("request.json"))
            .arg("--worker")
            .arg(self.directory.join("absent-worker"))
            .args([
                "--max-input-bytes",
                &input.to_string(),
                "--max-output-bytes",
                &output.to_string(),
            ])
            .args(extra)
            .output()
            .unwrap()
    }
}
impl Drop for Case {
    fn drop(&mut self) {
        for name in [
            "connection.json",
            "request.json",
            "source.json",
            "expected.json",
        ] {
            let path = self.directory.join(name);
            if path.exists() {
                fs::remove_file(path).unwrap();
            }
        }
        fs::remove_dir(&self.directory).unwrap();
        assert!(
            !self.directory.exists(),
            "owned non-image test temp removed"
        );
    }
}
#[test]
fn action_cli_rejects_wrong_syntax_and_aggregate_bounds_before_worker_spawn() {
    for command in ["prepare", "execute"] {
        let case = Case::new(command);
        for (input, output, extra) in [
            (case.input - 1, 65536, vec![]),
            (case.input, 1, vec![]),
            (case.input, 136, vec![]),
            (case.input, 65536, vec!["--json", "--json"]),
            (
                case.input,
                65536,
                vec![
                    if command == "prepare" {
                        "--plan"
                    } else {
                        "--snapshot"
                    },
                    "wrong",
                ],
            ),
        ] {
            let result = case.run(command, input, output, &extra);
            assert_eq!(
                result.status.code(),
                Some(2),
                "{}",
                String::from_utf8_lossy(&result.stderr)
            );
            assert!(result.stdout.is_empty());
        }
        let result = case.run(command, case.input, 65536, &["--json"]);
        assert_eq!(
            result.status.code(),
            Some(1),
            "missing worker remains IO, no SDK run"
        );
        assert!(result.stdout.is_empty());
    }
}
#[test]
fn action_cli_reuses_strict_connection_version_and_unknown_field_rejection() {
    for duplicate in [false, true] {
        let case = Case::new("prepare");
        let path = case.directory.join("connection.json");
        let old = fs::read_to_string(&path).unwrap();
        let changed = if duplicate {
            old.replacen('{', "{\"unrecognized\":true,", 1)
        } else {
            old.replace(
                "\"connection_version\":\"1.0.0\"",
                "\"connection_version\":\"2.0.0\"",
            )
        };
        fs::write(path, &changed).unwrap();
        let result = case.run("prepare", case.input + changed.len(), 65536, &[]);
        assert_eq!(result.status.code(), Some(2));
        assert!(result.stdout.is_empty());
    }
}

impl Case {
    fn forms_request(&self, focus: bool) {
        let path = self.directory.join("request.json");
        let mut document: Document = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        let Artifact::Request(request) = &mut document.artifact else {
            panic!("request")
        };
        let action = match &mut request.operation {
            Operation::Prepare { action } | Operation::Act { action } => action,
            _ => panic!("action"),
        };
        action.intent = if focus {
            Intent::Focus {}
        } else {
            Intent::Type {
                text: "tail-only".into(),
            }
        };
        action.modality = if focus {
            InputModality::Semantic
        } else {
            InputModality::Keyboard
        };
        let expected = Document {
            schema_version: SchemaVersion::CURRENT,
            artifact: Artifact::Expectation(Box::new(Expectation {
                id: Id("explicit-condition".into()),
                scope_id: action.authorized_scope.clone(),
                targets: vec![action.backend_ref.key.clone()],
                rule: Rule::PropertyEquals {
                    field: if focus { Field::Focused } else { Field::Value },
                    expected: if focus {
                        Value::Flag(true)
                    } else {
                        Value::Text("prefix-tail-only".into())
                    },
                },
                applies_when: ContextConditions {
                    platform: None,
                    input_mode: None,
                    text_scale: None,
                },
                expected_from: Id("caller-test-condition".into()),
            })),
        };
        document.validate().unwrap();
        expected.validate().unwrap();
        fs::write(path, serde_json::to_vec(&document).unwrap()).unwrap();
        fs::write(
            self.directory.join("expected.json"),
            serde_json::to_vec(&expected).unwrap(),
        )
        .unwrap();
    }
    fn total_input(&self) -> usize {
        [
            "connection.json",
            "request.json",
            "source.json",
            "expected.json",
        ]
        .iter()
        .map(|n| fs::metadata(self.directory.join(n)).unwrap().len() as usize)
        .sum()
    }
}
#[test]
fn forms_require_explicit_expectation_and_preserve_byte_budget_before_spawn() {
    for command in ["prepare", "execute"] {
        for focus in [true, false] {
            let c = Case::new(command);
            c.forms_request(focus);
            let expected_path = c.directory.join("expected.json");
            let path = expected_path.to_str().unwrap();
            let missing = c.run(command, 100_000, 65536, &["--json"]);
            assert_eq!(missing.status.code(), Some(2));
            assert_eq!(missing.stderr, b"expectation_required\n");
            assert!(missing.stdout.is_empty());
            let args = ["--json", "--expectation", path];
            let limited = c.run(command, c.total_input() - 1, 65536, &args);
            assert_eq!(limited.status.code(), Some(2));
            assert_eq!(limited.stderr, b"input_limit\n");
            assert!(limited.stdout.is_empty());
            // Reaches the absent-worker boundary rather than unsupported/missing
            // expectation. This is CLI admission proof, never delivery evidence.
            let ready = c.run(command, c.total_input(), 65536, &args);
            assert_eq!(ready.status.code(), Some(1));
            assert_eq!(ready.stderr, b"io_error\n");
            assert!(ready.stdout.is_empty());
            let duplicate = c.run(
                command,
                100_000,
                65536,
                &["--expectation", path, "--expectation", path],
            );
            assert_eq!(duplicate.status.code(), Some(2));
            assert_eq!(duplicate.stderr, b"invalid_arguments\n");
            assert!(!String::from_utf8_lossy(&ready.stderr).contains("tail-only"));
        }
    }
}
#[test]
fn action_cli_does_not_expand_intents_or_reinterpret_type_as_setter() {
    for (intent, modality) in [
        (Intent::Activate {}, InputModality::Semantic),
        (
            Intent::Type {
                text: "PRIVATE_UNSUPPORTED_CANARY".into(),
            },
            InputModality::Setter,
        ),
    ] {
        let c = Case::new("prepare");
        let path = c.directory.join("request.json");
        let mut d: Document = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        let Artifact::Request(request) = &mut d.artifact else {
            panic!()
        };
        let Operation::Prepare { action } = &mut request.operation else {
            panic!()
        };
        action.intent = intent;
        action.modality = modality;
        fs::write(path, serde_json::to_vec(&d).unwrap()).unwrap();
        let out = c.run("prepare", 100_000, 65536, &[]);
        assert_eq!(out.status.code(), Some(5));
        assert_eq!(out.stderr, b"unsupported_command\n");
        assert!(out.stdout.is_empty());
    }
}
