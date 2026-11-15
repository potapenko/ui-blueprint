use std::process::Command;

#[test]
fn capture_is_discoverable_without_starting_any_app() {
    let result = Command::new(env!("CARGO_BIN_EXE_uiblueprint")).arg("--help").output().unwrap();
    assert!(result.status.success());
    assert!(String::from_utf8(result.stdout).unwrap().contains("capture-window"));
}

#[cfg(not(all(target_os = "macos", feature = "macos")))]
#[test]
fn unavailable_capture_feature_never_dispatches_a_helper() {
    let result = Command::new(env!("CARGO_BIN_EXE_uiblueprint"))
        .args(["capture-window", "--pid", "PRIVATE_CANARY"]).output().unwrap();
    assert_eq!(result.status.code(), Some(5));
    assert!(result.stdout.is_empty());
    assert_eq!(result.stderr, b"native_capture_requires_macos_feature\n");
}
