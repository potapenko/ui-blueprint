use std::process::Command;

#[test]
fn capture_is_discoverable_without_starting_any_app() {
    let result = Command::new(env!("CARGO_BIN_EXE_uiblueprint"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(result.status.success());
    assert!(
        String::from_utf8(result.stdout)
            .unwrap()
            .contains("capture-window")
    );
}

#[cfg(not(all(target_os = "macos", feature = "macos")))]
#[test]
fn unavailable_capture_feature_never_dispatches_a_helper() {
    let result = Command::new(env!("CARGO_BIN_EXE_uiblueprint"))
        .args(["capture-window", "--pid", "PRIVATE_CANARY"])
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(5));
    assert!(result.stdout.is_empty());
    assert_eq!(result.stderr, b"native_capture_requires_macos_feature\n");
}

#[cfg(all(target_os = "macos", feature = "macos"))]
#[test]
fn installed_symlink_resolves_the_matching_bundle_helper() {
    use std::{
        fs,
        os::unix::{fs::PermissionsExt, fs::symlink},
        time::{SystemTime, UNIX_EPOCH},
    };
    let root = std::env::temp_dir().join(format!(
        "uib-capture-link-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let bundle = root.join("bundle");
    let bin = root.join("bin");
    fs::create_dir_all(&bundle).unwrap();
    fs::create_dir(&bin).unwrap();
    let executable = bundle.join("uiblueprint");
    fs::copy(env!("CARGO_BIN_EXE_uiblueprint"), &executable).unwrap();
    let helper = bundle.join("native-host-helper");
    fs::write(&helper,b"#!/bin/sh\n[ \"$1\" = capture-window ] && [ \"$2\" = --help ] || exit 9\nprintf 'matched-bundle\\n'\n").unwrap();
    fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).unwrap();
    let link = bin.join("uiblueprint");
    symlink(&executable, &link).unwrap();
    let result = Command::new(&link)
        .args(["capture-window", "--help"])
        .output()
        .unwrap();
    fs::remove_file(link).unwrap();
    fs::remove_file(helper).unwrap();
    fs::remove_file(executable).unwrap();
    fs::remove_dir(bin).unwrap();
    fs::remove_dir(bundle).unwrap();
    fs::remove_dir(root).unwrap();
    assert_eq!(result.status.code(), Some(0));
    assert_eq!(result.stdout, b"matched-bundle\n");
    assert!(result.stderr.is_empty());
}
