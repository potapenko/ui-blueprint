//! Exact-window capture reuses the matching native helper; no new collector/worker.
use crate::Failure;
use std::ffi::OsString;

pub(crate) fn execute(args: impl Iterator<Item = OsString>) -> Failure {
    #[cfg(all(target_os = "macos", feature = "macos"))]
    {
        use std::{os::unix::process::CommandExt, process::Command};
        let Ok(executable) = std::env::current_exe() else {
            return Failure::io();
        };
        let Some(parent) = executable.parent() else {
            return Failure::io();
        };
        // The installed CLI/worker/helper are one verified immutable bundle.
        // exec replaces only this caller process. The helper owns its finite watchdog.
        let error = Command::new(parent.join("native-host-helper"))
            .arg("capture-window")
            .args(args)
            .exec();
        let _ = error;
        Failure::io()
    }
    #[cfg(not(all(target_os = "macos", feature = "macos")))]
    {
        let _ = args;
        Failure::unsupported("native_capture_requires_macos_feature")
    }
}
