#![cfg(all(target_os = "macos", feature = "web"))]
//! Real guarded startup and typed parent terminal; synthetic CDP only, no browser.
#[allow(dead_code)]
#[path = "support/web_worker_data.rs"]
mod data;
#[allow(dead_code)]
#[path = "support/web_worker_peer.rs"]
mod peer;
use std::{
    path::Path,
    thread,
    time::{Duration, Instant},
};
use uiblueprint_host::{
    HostError,
    authority::TargetLease,
    domain::HostDomain,
    host_types::{HostEvent, Terminal},
    process::DarwinPlatform,
    process_api::SpawnSpec,
    supervisor::RuntimeHost,
    worker_tape,
};

#[test]
fn pre_ready_stale_invalid_permission_are_typed_and_reaped_without_output() {
    for mode in 0..3 {
        let peer = peer::Peer::new();
        let mut setup = data::setup(peer.url.clone());
        if mode == 0 {
            setup.surface.generation = data::id("old-loader");
        }
        let mut descriptor = data::descriptor();
        if mode == 0
            && let uiblueprint_schema::model::Artifact::Session(s) = &mut descriptor.artifact
        {
            s.surfaces = vec![setup.surface.clone()];
        }
        let descriptor = serde_json::to_vec(&descriptor).unwrap();
        let setup = if mode == 1 {
            b"{}".to_vec()
        } else {
            serde_json::to_vec(&setup).unwrap()
        };
        let parts = [descriptor.as_slice(), setup.as_slice()];
        let domain = HostDomain::new::<DarwinPlatform>(data::limits()).unwrap();
        let mut host = RuntimeHost::new(
            &domain,
            SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap(),
            DarwinPlatform,
        )
        .unwrap();
        let mut target = data::target();
        if mode == 2 {
            target.generation = data::id("wrong-authority");
        }
        let mut input = host
            .reserve_attach_input(
                TargetLease::authorized(&target, false).unwrap(),
                40 + parts.iter().map(|p| p.len()).sum::<usize>(),
            )
            .unwrap();
        worker_tape::encode(&parts, input.bytes_mut()).unwrap();
        let end = Instant::now() + Duration::from_secs(5);
        host.attach_web(input, end).unwrap();
        loop {
            assert!(Instant::now() < end);
            match host.next_event().unwrap() {
                HostEvent::Pending => thread::sleep(Duration::from_millis(1)),
                HostEvent::Complete(c) => {
                    let expected = match mode {
                        0 => HostError::ResyncRequired,
                        1 => HostError::InvalidInput,
                        _ => HostError::PermissionDenied,
                    };
                    assert_eq!(c.terminal, Terminal::Failed(expected));
                    assert_eq!(c.committed(), 0);
                    assert!(!c.effect_unknown());
                    break;
                }
                _ => panic!("no Ready on rejected startup"),
            }
        }
        loop {
            assert!(Instant::now() < end);
            if matches!(host.shutdown().unwrap(), HostEvent::ShutdownComplete) {
                break;
            }
            thread::sleep(Duration::from_millis(1));
        }
    }
}
