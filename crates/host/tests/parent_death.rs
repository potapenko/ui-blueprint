#![cfg(all(target_os = "macos", feature = "web"))]
//! Actual supervisor process death while the guarded operation thread waits for
//! an owned network peer. No UI/browser, worker-PID signal, or production test API.
#[path = "support/web_worker_data.rs"]
mod data;
#[path = "support/web_worker_peer.rs"]
mod peer;
use std::{
    path::Path,
    process::{Child, Command, Stdio},
    sync::atomic::Ordering,
    thread,
    time::{Duration, Instant},
};
use uiblueprint_host::{
    authority::TargetLease,
    domain::HostDomain,
    host_types::{HostEvent, OutputRequest},
    process::DarwinPlatform,
    process_api::SpawnSpec,
    supervisor::RuntimeHost,
    worker_tape,
};
use uiblueprint_schema::model::{Artifact, Channel};
fn end() -> Instant {
    Instant::now() + Duration::from_secs(10)
}
fn process_query(program: &str, args: &[&str]) -> std::process::Output {
    let mut child = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("owned read-only query");
    let deadline = Instant::now() + Duration::from_secs(1);
    while child.try_wait().expect("query status").is_none() {
        if Instant::now() >= deadline {
            child.kill().expect("terminate owned query");
            child.wait().expect("reap query");
            panic!("bounded read-only process query timed out");
        }
        thread::sleep(Duration::from_millis(1));
    }
    child.wait_with_output().expect("bounded PID/state output")
}
fn worker_running(pid: u32) -> bool {
    // Read-only exact PID query. A recycled PID can only make the test refuse to
    // claim exit; this code never signals or cleans a numeric worker PID.
    let result = process_query(
        "/bin/ps",
        &["-p", &pid.to_string(), "-o", "pid=", "-o", "stat="],
    );
    if result.status.success() {
        let text = std::str::from_utf8(&result.stdout).unwrap();
        let mut fields = text.split_whitespace();
        assert_eq!(fields.next().unwrap().parse::<u32>().unwrap(), pid);
        let state = fields.next().unwrap();
        assert!(fields.next().is_none());
        // Z means the process has exited but init has not reaped its record.
        // Parent-death liveness requires exit; this is not our waitpid/reap claim.
        if state.starts_with('Z') {
            return false;
        }
        true
    } else {
        assert_eq!(result.status.code(), Some(1));
        assert!(result.stdout.is_empty() && result.stderr.is_empty());
        false
    }
}
fn only_worker(parent: u32) -> u32 {
    // Parent is an owned live Child handle and launches exactly one worker; no
    // other application/process family is inspected or selected as a target.
    let output = process_query("/usr/bin/pgrep", &["-P", &parent.to_string()]);
    assert!(output.status.success());
    let text = std::str::from_utf8(&output.stdout).unwrap();
    let mut pids = text.split_whitespace();
    let pid = pids.next().unwrap().parse().unwrap();
    assert!(pids.next().is_none());
    pid
}
struct OwnedRun {
    parent: Child,
    peer: Option<peer::Peer>,
    worker: Option<u32>,
}
impl Drop for OwnedRun {
    fn drop(&mut self) {
        if self.parent.try_wait().ok().flatten().is_none() {
            let _ = self.parent.kill();
        }
        let _ = self.parent.wait();
        // On a failed liveness assertion, closing ONLY our peer unblocks the
        // worker's network wait; its ordinary parent-IO/deadline path can retire.
        self.peer.take();
        if let Some(pid) = self.worker {
            let stop = Instant::now() + Duration::from_secs(11);
            while worker_running(pid) && Instant::now() < stop {
                thread::sleep(Duration::from_millis(10));
            }
            if !thread::panicking() {
                assert!(
                    !worker_running(pid),
                    "owned worker cleanup remains incomplete"
                );
            }
        }
    }
}
#[test]
fn actual_parent_death_stops_worker_during_network_acquisition() {
    let peer = peer::Peer::new();
    peer.state.stall.store(true, Ordering::Release);
    let parent = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "parent_actor", "--ignored", "--test-threads=1"])
        .env("UIB_PARENT_DEATH_PEER", "1")
        .env("UIB_PARENT_DEATH_ENDPOINT", &peer.url)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut run = OwnedRun {
        parent,
        peer: Some(peer),
        worker: None,
    };
    let setup = end();
    while !run
        .peer
        .as_ref()
        .unwrap()
        .state
        .stalled
        .load(Ordering::Acquire)
    {
        assert!(
            run.parent.try_wait().unwrap().is_none(),
            "supervisor must remain alive through real admission"
        );
        assert!(Instant::now() < setup, "bounded acquisition admission");
        thread::sleep(Duration::from_millis(1));
    }
    let stalled = Instant::now();
    let pid = only_worker(run.parent.id());
    run.worker = Some(pid);
    assert!(worker_running(pid));
    assert!(run.parent.try_wait().unwrap().is_none());
    // Only our owned supervisor is signalled. RuntimeHost Drop cannot run after
    // SIGKILL; this is neither explicit shutdown nor a manually closed input FD.
    run.parent.kill().unwrap();
    let status = run.parent.wait().unwrap();
    assert!(!status.success());
    let until = Instant::now() + Duration::from_secs(1);
    while worker_running(pid) && Instant::now() < until {
        thread::sleep(Duration::from_millis(5));
    }
    assert!(
        !worker_running(pid),
        "watchdog must stop worker before the blocked network operation can finish"
    );
    assert!(
        stalled.elapsed() < Duration::from_secs(2),
        "before peer3s and request10s timeouts"
    );
    assert_eq!(
        run.peer
            .as_ref()
            .unwrap()
            .state
            .releases
            .load(Ordering::Acquire),
        0,
        "collection never reached normal release/publication"
    );
    // Peer is deliberately still open here. Its shutdown is teardown, not the
    // event credited for worker exit. Zombie/absence establishes exit; the OS reaps the orphan, not this test.
}
#[test]
#[ignore = "only the bounded outer case launches this disposable supervisor"]
fn parent_actor() {
    assert_eq!(std::env::var("UIB_PARENT_DEATH_PEER").unwrap(), "1");
    let endpoint = std::env::var("UIB_PARENT_DEATH_ENDPOINT").unwrap();
    let domain = HostDomain::new::<DarwinPlatform>(data::limits()).unwrap();
    let mut host = RuntimeHost::new(
        &domain,
        SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap(),
        DarwinPlatform,
    )
    .unwrap();
    let descriptor = serde_json::to_vec(&data::descriptor()).unwrap();
    let setup = serde_json::to_vec(&data::setup(endpoint)).unwrap();
    let length = 40 + descriptor.len() + setup.len();
    let mut lease = host
        .reserve_attach_input(
            TargetLease::authorized(&data::target(), false).unwrap(),
            length,
        )
        .unwrap();
    worker_tape::encode(&[&descriptor, &setup], lease.bytes_mut()).unwrap();
    let session = host.attach_web(lease, end()).unwrap();
    let attach_end = end();
    let clock = loop {
        assert!(Instant::now() < attach_end);
        match host.next_event().unwrap() {
            HostEvent::Attached { clock, .. } => break clock,
            HostEvent::Pending => thread::yield_now(),
            _ => panic!("real attach"),
        }
    };
    let mut request = data::request(clock.as_str(), vec![Channel::ExternalSemantics]);
    let Artifact::Request(r) = &mut request.artifact else {
        panic!("request")
    };
    r.limits.deadline_ms = 10_000;
    let request = serde_json::to_vec(&request).unwrap();
    let selection = serde_json::to_vec(&data::selection(false)).unwrap();
    let mut input = host
        .reserve_input(session, 40 + request.len() + selection.len())
        .unwrap();
    worker_tape::encode(&[&request, &selection], input.bytes_mut()).unwrap();
    host.submit_web_observe(
        session,
        input,
        OutputRequest {
            channels: 1,
            frame_bytes: 65536,
            total_bytes: 65536,
            input_format: 0,
            retained_partition: 0,
        },
        end(),
    )
    .unwrap();
    let operation_end = end();
    loop {
        assert!(Instant::now() < operation_end);
        assert!(
            matches!(host.next_event().unwrap(), HostEvent::Pending),
            "no normal completion before parent death"
        );
        thread::yield_now();
    }
}
