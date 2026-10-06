use std::error::Error;
use uiblueprint_schema::{SchemaVersion, VersionDocument, VersionError};

const CURRENT: &[u8] = include_bytes!("fixtures/current.json");
const INCOMPATIBLE: &[u8] = include_bytes!("fixtures/incompatible.json");
const MALFORMED: &[u8] = include_bytes!("fixtures/malformed.json");

#[test]
fn authored_current_example_round_trips_without_changing_wire_identity() {
    let parsed =
        VersionDocument::from_json(CURRENT, CURRENT.len()).expect("valid authored example");
    assert_eq!(parsed.schema_version(), SchemaVersion::CURRENT);
    let output = parsed.to_json().expect("encode current version");
    assert_eq!(output, r#"{"schema_version":"0.1.0"}"#);
    assert_eq!(
        VersionDocument::from_json(output.as_bytes(), output.len()),
        Ok(parsed)
    );
}

#[test]
fn valid_but_unsupported_versions_are_never_coerced_to_current() {
    assert_eq!(
        VersionDocument::from_json(INCOMPATIBLE, INCOMPATIBLE.len()),
        Err(VersionError::IncompatibleVersion)
    );
    for token in ["0.0.0", "0.1.1", "1.0.0", "999999999999999999999.0.0"] {
        assert_eq!(
            token.parse::<SchemaVersion>(),
            Err(VersionError::IncompatibleVersion)
        );
    }
}

#[test]
fn malformed_tokens_are_distinct_from_incompatible_versions() {
    assert_eq!(
        VersionDocument::from_json(MALFORMED, MALFORMED.len()),
        Err(VersionError::MalformedVersion)
    );
    for token in [
        "",
        "0.1",
        "0.1.0.0",
        "00.1.0",
        "0.01.0",
        "-1.0.0",
        " 0.1.0",
        "0.1.0\n",
        "0.1.0-beta",
        "0.1.0+build",
        "０.1.0",
    ] {
        assert_eq!(
            token.parse::<SchemaVersion>(),
            Err(VersionError::MalformedVersion)
        );
    }
}

#[test]
fn strict_shape_rejects_missing_extra_duplicate_and_mistyped_fields() {
    for input in [
        r#"{}"#,
        r#"{"schema_version":null}"#,
        r#"{"schema_version":false}"#,
        r#"{"schema_version":1}"#,
        r#"{"schema_version":"0.1.0","extra":true}"#,
        r#"{"schema_version":"0.1.0","schema_version":"0.1.0"}"#,
        r#"["0.1.0"]"#,
        r#"{"schema_version":"0.1.0"} {}"#,
        r#"{"schema_version":"0.1.0""#,
    ] {
        assert_eq!(
            VersionDocument::from_json(input.as_bytes(), input.len()),
            Err(VersionError::InvalidDocument)
        );
    }
}

#[test]
fn caller_byte_limit_is_inclusive_and_checked_before_parsing() {
    assert!(VersionDocument::from_json(CURRENT, CURRENT.len()).is_ok());
    assert_eq!(
        VersionDocument::from_json(CURRENT, CURRENT.len() - 1),
        Err(VersionError::InputTooLarge)
    );
    assert_eq!(
        VersionDocument::from_json(b"invalid", 0),
        Err(VersionError::InputTooLarge)
    );
    assert_eq!(
        VersionDocument::from_json(b"", 0),
        Err(VersionError::InvalidDocument)
    );
}

#[test]
fn diagnostics_do_not_echo_untrusted_version_keys_values_or_source_errors() {
    let canary = "t01-untrusted-value-canary";
    for input in [
        format!(r#"{{"schema_version":"{canary}"}}"#),
        format!(r#"{{"schema_version":"0.1.0","{canary}":true}}"#),
        format!(r#"{{"schema_version":{{"value":"{canary}"}}}}"#),
    ] {
        let error =
            VersionDocument::from_json(input.as_bytes(), input.len()).expect_err("invalid example");
        assert!(!error.to_string().contains(canary));
        assert!(!format!("{error:?}").contains(canary));
        assert!(error.source().is_none());
    }
}
