//! Private supervised worker executable; no public command-line protocol.
#![deny(unsafe_code)]

#[cfg(target_os = "macos")]
mod quota_allocator;
#[cfg(target_os = "macos")]
mod worker_action;
#[cfg(target_os = "macos")]
mod worker_effect;
#[cfg(target_os = "macos")]
mod worker_io;
#[cfg(target_os = "macos")]
mod worker_main;
#[cfg(target_os = "macos")]
mod worker_native;
#[cfg(target_os = "macos")]
mod worker_native_action;
#[cfg(target_os = "macos")]
mod worker_ops;
#[cfg(all(target_os = "macos", feature = "web"))]
mod worker_web;
#[cfg(target_os = "macos")]
use uiblueprint_host::worker_tape;

#[cfg(target_os = "macos")]
#[global_allocator]
static ALLOCATOR: quota_allocator::GuardedAllocator = quota_allocator::GuardedAllocator;

#[cfg(target_os = "macos")]
fn main() {
    worker_main::main_entry();
}

#[cfg(not(target_os = "macos"))]
fn main() {
    std::process::exit(1);
}
