//! Disposable process exercises actual Darwin policy changes. The outer test
//! runner/operator never changes SIGCHLD. The audit wrapper forwards every OS
//! operation and predicate; it only pauses reads after a complete ACK for proof.
use super::*;
use std::{
    os::fd::BorrowedFd,
    process::Command,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};
use uiblueprint_host::{
    process::DarwinChild,
    process_api::{OwnedProcess, PollInterest, ProcessPlatform, ProcessState, Transfer},
};
static LOSE_ON_SPAWN: AtomicBool = AtomicBool::new(false);
static WRITES: AtomicUsize = AtomicUsize::new(0);
static READS: AtomicUsize = AtomicUsize::new(0);
static ACK: AtomicBool = AtomicBool::new(false);
static KILLS: AtomicUsize = AtomicUsize::new(0);
static WAITS: AtomicUsize = AtomicUsize::new(0);
struct Audit;
struct Child(DarwinChild);
impl ProcessPlatform for Audit {
    type Child = Child;
    fn validate_parent_reaping() -> Result<(), HostError> {
        DarwinPlatform::validate_parent_reaping()
    }
    fn spawn(&mut self, spec: &SpawnSpec) -> Result<Child, HostError> {
        let mut child = DarwinPlatform.spawn(spec)?;
        if LOSE_ON_SPAWN.load(Ordering::SeqCst) {
            let original = set_policy(changed_policy as *const () as usize);
            // Actual Native owner latches Lost before any PID wait under this
            // unsupported policy. Restore policy, but not the lost owner.
            assert_eq!(child.try_reap(), Err(HostError::CleanupPending));
            unsafe {
                assert_eq!(
                    libc::sigaction(libc::SIGCHLD, &original, std::ptr::null_mut()),
                    0
                );
            }
            assert_eq!(DarwinPlatform::validate_parent_reaping(), Ok(()));
        }
        Ok(Child(child))
    }
    fn poll(&mut self, interests: &mut [PollInterest<'_>], wait_ms: u32) -> Result<(), HostError> {
        DarwinPlatform.poll(interests, wait_ms)
    }
}
impl OwnedProcess for Child {
    fn write_input(&mut self, b: &[u8]) -> Result<Transfer, HostError> {
        WRITES.fetch_add(1, Ordering::SeqCst);
        let result = self.0.write_input(b)?;
        if b.len() == CONTROL_BYTES && matches!(result, Transfer::Bytes(CONTROL_BYTES)) {
            let bytes: &[u8; CONTROL_BYTES] = b.try_into().unwrap();
            if Control::decode(bytes).is_ok_and(|c| c.kind == ControlKind::Ack) {
                ACK.store(true, Ordering::SeqCst);
            }
        }
        Ok(result)
    }
    fn read_output(&mut self, b: &mut [u8]) -> Result<Transfer, HostError> {
        READS.fetch_add(1, Ordering::SeqCst);
        if ACK.load(Ordering::SeqCst) {
            Ok(Transfer::WouldBlock)
        } else {
            self.0.read_output(b)
        }
    }
    fn read_fatal(&mut self, b: &mut [u8]) -> Result<Transfer, HostError> {
        READS.fetch_add(1, Ordering::SeqCst);
        self.0.read_fatal(b)
    }
    fn close_input(&mut self) {
        self.0.close_input();
    }
    fn terminate(&mut self) -> Result<(), HostError> {
        KILLS.fetch_add(1, Ordering::SeqCst);
        self.0.terminate()
    }
    fn try_reap(&mut self) -> Result<ProcessState, HostError> {
        WAITS.fetch_add(1, Ordering::SeqCst);
        self.0.try_reap()
    }
    fn input_fd(&self) -> Option<BorrowedFd<'_>> {
        self.0.input_fd()
    }
    fn output_fd(&self) -> BorrowedFd<'_> {
        self.0.output_fd()
    }
    fn fatal_fd(&self) -> BorrowedFd<'_> {
        self.0.fatal_fd()
    }
}
extern "C" fn changed_policy(_: libc::c_int) {}
fn set_policy(handler: usize) -> libc::sigaction {
    // SAFETY: initialized public sigaction records; only this disposable peer's
    // process policy is modified, with the original returned for exact restore.
    unsafe {
        let mut action: libc::sigaction = std::mem::zeroed();
        let mut old: libc::sigaction = std::mem::zeroed();
        action.sa_sigaction = handler;
        assert_eq!(libc::sigemptyset(&mut action.sa_mask), 0);
        assert_eq!(libc::sigaction(libc::SIGCHLD, &action, &mut old), 0);
        old
    }
}
#[test]
fn actual_policy_drift_quarantines_without_losing_acked_bytes() {
    disposable("1");
}
#[test]
fn returned_lost_owner_is_not_admitted_after_current_policy_recovers() {
    disposable("lost");
}
fn disposable(mode: &str) {
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "reaping::disposable_parent",
            "--ignored",
            "--test-threads=1",
        ])
        .env("UIB_REAPING_PEER", mode)
        .spawn()
        .unwrap();
    let stop = deadline();
    loop {
        if let Some(status) = child.try_wait().unwrap() {
            assert!(status.success());
            break;
        }
        if Instant::now() >= stop {
            child.kill().unwrap();
            child.wait().unwrap();
            panic!("reaping proof timeout");
        }
        thread::sleep(Duration::from_millis(5));
    }
}
#[test]
#[ignore = "only the bounded outer test may run this disposable signal-policy peer"]
fn disposable_parent() {
    let mode = std::env::var("UIB_REAPING_PEER").unwrap();
    if mode == "lost" {
        returned_lost();
        return;
    }
    assert_eq!(mode, "1");
    let original = set_policy(changed_policy as *const () as usize);
    assert!(matches!(
        HostDomain::new::<DarwinPlatform>(limits()),
        Err(HostError::InvalidState)
    ));
    // SAFETY: restore exactly this peer's prior disposition before managed work.
    unsafe {
        assert_eq!(
            libc::sigaction(libc::SIGCHLD, &original, std::ptr::null_mut()),
            0
        );
    }
    let domain = HostDomain::new::<Audit>(limits()).unwrap();
    let spec = SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap();
    let mut host = RuntimeHost::new(&domain, spec, Audit).unwrap();
    let mut input = host
        .reserve_attach_input(target(), DESCRIPTOR.len())
        .unwrap();
    input.bytes_mut().copy_from_slice(DESCRIPTOR);
    let session = host.attach(input, deadline()).unwrap();
    let stop = deadline();
    loop {
        assert!(Instant::now() < stop);
        match host.next_event().unwrap() {
            HostEvent::Attached { .. } => break,
            HostEvent::Pending => thread::sleep(Duration::from_millis(1)),
            _ => panic!("attach"),
        }
    }
    let mut input = host.reserve_input(session, QUERY.len()).unwrap();
    input.bytes_mut().copy_from_slice(QUERY);
    host.submit(
        session,
        OperationClass::Validate,
        input,
        OutputRequest {
            channels: 1,
            frame_bytes: 65536,
            total_bytes: 65536,
            input_format: 1,
            retained_partition: 0,
        },
        deadline(),
    )
    .unwrap();
    while !ACK.load(Ordering::SeqCst) {
        assert!(Instant::now() < stop);
        assert!(matches!(host.next_event().unwrap(), HostEvent::Pending));
        thread::sleep(Duration::from_millis(1));
    }
    let reserved = domain.usage().retained_reserved_bytes;
    let waits_before_loss = WAITS.load(Ordering::SeqCst);
    set_policy(changed_policy as *const () as usize);
    let HostEvent::Complete(completion) = host.next_event().unwrap() else {
        panic!("preserved result")
    };
    assert_eq!(
        completion.terminal,
        Terminal::Failed(HostError::CleanupPending)
    );
    assert_eq!(completion.bytes(0), Some(QUERY));
    assert_eq!(completion.committed(), 1);
    assert_eq!(domain.usage().retained_reserved_bytes, reserved);
    assert!(domain.usage().reaping_poisoned && domain.usage().abandoned);
    assert!(matches!(
        host.reserve_input(session, 1),
        Err(HostError::CleanupPending)
    ));
    assert!(matches!(
        host.reserve_attach_input(target(), 1),
        Err(HostError::CleanupPending)
    ));
    assert!(matches!(
        host.next_event().unwrap(),
        HostEvent::CleanupPending { .. }
    ));
    assert_eq!(KILLS.load(Ordering::SeqCst), 0);
    assert_eq!(WAITS.load(Ordering::SeqCst), waits_before_loss);
    drop(host);
    assert_eq!(completion.bytes(0), Some(QUERY));
    assert_eq!(domain.usage().retained_reserved_bytes, reserved);
    drop(completion);
    drop(domain);
    // Host deliberately retains poisoned roots. This disposable test now restores
    // policy and independently reaps its sole child after protocol EOF. No PID
    // signalling, shared runner children, or unregistered grandchildren exist.
    unsafe {
        assert_eq!(
            libc::sigaction(libc::SIGCHLD, &original, std::ptr::null_mut()),
            0
        );
    }
    assert!(matches!(
        HostDomain::new::<Audit>(limits()),
        Err(HostError::Busy)
    ));
    let stop = deadline();
    loop {
        let mut status = 0;
        // SAFETY: isolated test peer owns exactly one child; only WNOHANG reaping,
        // never a kill by an externally supplied PID. Deadline bounds this loop.
        let result = unsafe { libc::waitpid(-1, &mut status, libc::WNOHANG) };
        if result > 0 {
            break;
        }
        assert_eq!(result, 0, "owned worker not already reaped elsewhere");
        assert!(
            Instant::now() < stop,
            "worker watchdog must exit on parent EOF"
        );
        thread::sleep(Duration::from_millis(1));
    }
}

fn returned_lost() {
    LOSE_ON_SPAWN.store(true, Ordering::SeqCst);
    let domain = HostDomain::new::<Audit>(limits()).unwrap();
    let spec = SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap();
    let mut host = RuntimeHost::new(&domain, spec, Audit).unwrap();
    let mut input = host
        .reserve_attach_input(target(), DESCRIPTOR.len())
        .unwrap();
    input.bytes_mut().copy_from_slice(DESCRIPTOR);
    let reserved = domain.usage().retained_reserved_bytes;
    assert!(matches!(
        host.attach(input, deadline()),
        Err(HostError::CleanupPending)
    ));
    assert_eq!(DarwinPlatform::validate_parent_reaping(), Ok(()));
    assert!(matches!(
        host.next_event().unwrap(),
        HostEvent::CleanupPending { .. }
    ));
    assert!(matches!(
        host.reserve_attach_input(target(), 1),
        Err(HostError::CleanupPending)
    ));
    assert_eq!(
        WRITES.load(Ordering::SeqCst),
        0,
        "no configuration or input"
    );
    assert_eq!(
        READS.load(Ordering::SeqCst),
        0,
        "no readiness or payload reads"
    );
    assert_eq!(KILLS.load(Ordering::SeqCst), 0);
    assert_eq!(
        WAITS.load(Ordering::SeqCst),
        1,
        "one Native Lost query, no waitpid"
    );
    assert_eq!(domain.usage().retained_reserved_bytes, reserved);
    assert_eq!(domain.usage().reserved_sessions, 1);
    assert!(domain.usage().abandoned);
    drop(host);
    assert_eq!(domain.usage().retained_reserved_bytes, reserved);
    assert_eq!(WAITS.load(Ordering::SeqCst), 1);
    assert_eq!(KILLS.load(Ordering::SeqCst), 0);
    drop(domain);
    assert!(matches!(
        HostDomain::new::<Audit>(limits()),
        Err(HostError::Busy)
    ));
    // This isolated test's sole child exits on closed input even before worker
    // configuration. Explicit test teardown reaps it outside the poisoned host.
    let stop = deadline();
    loop {
        let mut status = 0;
        // SAFETY: sole child in this disposable peer, bounded nonblocking wait.
        let pid = unsafe { libc::waitpid(-1, &mut status, libc::WNOHANG) };
        if pid > 0 {
            break;
        }
        assert_eq!(pid, 0);
        assert!(Instant::now() < stop);
        thread::sleep(Duration::from_millis(1));
    }
}
