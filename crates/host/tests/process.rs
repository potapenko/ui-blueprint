#![cfg(target_os = "macos")]
use std::{
    os::fd::{AsRawFd, FromRawFd, OwnedFd},
    path::PathBuf,
    time::{Duration, Instant},
};
use uiblueprint_host::{
    HostError,
    process::{DarwinChild, DarwinPlatform},
    process_api::*,
};

fn executable() -> PathBuf {
    std::env::var_os("H01_PROCESS_PEER")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            std::env::current_exe()
                .expect("test path")
                .parent()
                .expect("deps")
                .parent()
                .expect("debug")
                .join("examples/process_peer")
        })
}
fn child(mode: &[u8]) -> DarwinChild {
    let mut child = DarwinPlatform
        .spawn(&SpawnSpec::new(&executable()).expect("spec"))
        .expect("owned peer spawn");
    assert_eq!(
        child.write_input(mode).expect("command"),
        Transfer::Bytes(mode.len())
    );
    child
}
fn output(child: &mut DarwinChild, count: usize) -> Vec<u8> {
    let mut bytes = vec![0; count];
    let mut at = 0;
    let end = Instant::now() + Duration::from_secs(1);
    while at < count && Instant::now() < end {
        match child.read_output(&mut bytes[at..]).expect("read") {
            Transfer::Bytes(n) => at += n,
            Transfer::WouldBlock => std::thread::yield_now(),
            Transfer::Closed => break,
        }
    }
    assert_eq!(at, count, "bounded peer output");
    bytes
}
fn reaped(child: &mut DarwinChild) -> ProcessState {
    let end = Instant::now() + Duration::from_secs(1);
    while Instant::now() < end {
        let state = child.try_reap().expect("owned wait");
        if state != ProcessState::Running {
            return state;
        }
        std::thread::yield_now();
    }
    let _ = child.terminate();
    panic!("owned peer missed reap deadline")
}
fn u64_at(data: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(data[at..at + 8].try_into().expect("fixed record"))
}
fn descriptors() -> usize {
    (0..512)
        .filter(|fd| unsafe { libc::fcntl(*fd, libc::F_GETFD) } >= 0)
        .count()
}

#[test]
fn mapped_lanes_empty_environment_actual_stack_and_no_inheritance() {
    let raw = unsafe { libc::open(c"/dev/null".as_ptr(), libc::O_RDONLY) };
    assert!(raw >= 0);
    let extra = unsafe { OwnedFd::from_raw_fd(raw) };
    unsafe {
        libc::fcntl(extra.as_raw_fd(), libc::F_SETFD, 0);
    }
    let mut c = child(b"I");
    for fd in [c.input_fd().expect("input"), c.output_fd(), c.fatal_fd()] {
        assert_ne!(
            unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_GETFD) } & libc::FD_CLOEXEC,
            0
        );
        assert_ne!(
            unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_GETFL) } & libc::O_NONBLOCK,
            0
        );
    }
    let data = output(&mut c, 64);
    assert_eq!(&data[..8], &[0xa1, 0, 1, 0, 1, 1, 1, 1]);
    assert!(u64_at(&data, 8) > 0 && u64_at(&data, 8) <= 8 * 1024 * 1024);
    assert!(
        u64_at(&data, 16) > 0 && u64_at(&data, 16) <= 1024 * 1024,
        "actual watchdog stack {}",
        u64_at(&data, 16)
    );
    assert_eq!(data[28], 1);
    eprintln!(
        "owned-process inventory: main_stack={} watchdog_stack={} platform={} child={} spawn_spec={} poll_backing={}",
        u64_at(&data, 8),
        u64_at(&data, 16),
        std::mem::size_of::<DarwinPlatform>(),
        std::mem::size_of::<DarwinChild>(),
        std::mem::size_of::<SpawnSpec>(),
        MAX_POLL_FDS * std::mem::size_of::<libc::pollfd>()
    );
    assert_eq!(reaped(&mut c), ProcessState::Exited { code: 0 });
}
#[test]
fn low_descriptor_collision_is_confined_to_owned_peer() {
    let mut c = child(b"C");
    let _ = output(&mut c, 64);
    let result = output(&mut c, 64);
    assert_eq!(&result[..2], &[1, 1]);
    assert_eq!(reaped(&mut c), ProcessState::Exited { code: 0 });
}
#[test]
fn nonblocking_closed_peer_and_zero_length_are_distinct() {
    let mut c = child(b"D");
    let _ = output(&mut c, 64);
    assert_eq!(output(&mut c, 1), [1]);
    assert_eq!(c.read_output(&mut []).expect("empty"), Transfer::Bytes(0));
    assert_eq!(
        c.read_output(&mut [0; 1]).expect("nonblocking"),
        Transfer::WouldBlock
    );
    assert_eq!(
        c.write_input(b"x").expect("SIGPIPE suppressed per send"),
        Transfer::Closed
    );
    c.close_input();
    assert!(c.input_fd().is_none());
    assert_eq!(c.write_input(b"x").expect("closed"), Transfer::Closed);
    assert_eq!(reaped(&mut c), ProcessState::Exited { code: 0 });
    assert_eq!(c.read_output(&mut [0; 1]).expect("EOF"), Transfer::Closed);
}
#[test]
fn saturation_independent_children_and_repeated_reap() {
    let mut a = child(b"H");
    let _ = output(&mut a, 64);
    let mut blocked = false;
    for _ in 0..1024 {
        match a.write_input(&[0; 4096]).expect("bounded send") {
            Transfer::WouldBlock => {
                blocked = true;
                break;
            }
            Transfer::Bytes(_) => {}
            Transfer::Closed => panic!("peer closed"),
        }
    }
    assert!(blocked);
    let mut b = child(&[b'E', 42]);
    let _ = output(&mut b, 64);
    assert_eq!(output(&mut b, 1), [42]);
    assert_eq!(reaped(&mut b), ProcessState::Exited { code: 0 });
    assert_eq!(a.try_reap().expect("running"), ProcessState::Running);
    a.terminate().expect("kill owned");
    let state = reaped(&mut a);
    assert_eq!(
        state,
        ProcessState::Signaled {
            signal: libc::SIGKILL
        }
    );
    a.terminate().expect("no resignal");
    assert_eq!(a.try_reap().expect("cached"), state);
}
#[test]
fn failed_spawn_releases_only_attempt_fds() {
    let spec = SpawnSpec::new(std::path::Path::new("/nonexistent-uiblueprint-owned-peer"))
        .expect("absolute spec");
    let before = descriptors();
    for _ in 0..10 {
        assert!(DarwinPlatform.spawn(&spec).is_err());
    }
    assert_eq!(descriptors(), before);
}
#[test]
fn poll_limit_relative_timeout_and_eintr() {
    let mut c = child(b"H");
    let _ = output(&mut c, 64);
    let fd = c.output_fd();
    let mut interests: Vec<_> = (0..37)
        .map(|_| PollInterest {
            fd,
            read: true,
            write: false,
            ready_read: false,
            ready_write: false,
            closed: false,
        })
        .collect();
    assert_eq!(
        DarwinPlatform.poll(&mut interests, 0),
        Err(HostError::InvalidLimits)
    );
    assert_eq!(
        DarwinPlatform.poll(&mut [], u32::MAX),
        Err(HostError::InvalidLimits)
    );
    let start = Instant::now();
    DarwinPlatform.poll(&mut interests[..1], 20).expect("poll");
    assert!(start.elapsed() < Duration::from_millis(500));
    assert!(!interests[0].ready_read);
    drop(interests);
    c.terminate().expect("stop");
    let _ = reaped(&mut c);
    let mut peer = child(b"P");
    let _ = output(&mut peer, 64);
    let data = output(&mut peer, 64);
    assert_eq!(data[0], 1);
    assert!(u64_at(&data, 8) < 400, "EINTR must not restart500ms wait");
    let _ = reaped(&mut peer);
}
#[test]
fn fatal_status_and_missing_status_exit_without_unwind_or_retry() {
    for (mode, code) in [(b'F', 77), (b'M', 78), (b'B', 79)] {
        let mut c = child(&[mode]);
        let _ = output(&mut c, 64);
        assert_eq!(reaped(&mut c), ProcessState::Exited { code });
        let mut buf = [0; 64];
        let n = c.read_fatal(&mut buf).expect("fatal");
        if mode == b'F' {
            assert_eq!(n, Transfer::Bytes(64));
            assert_eq!(buf, [0xa5; 64]);
        }
        if mode == b'M' {
            assert_eq!(n, Transfer::Closed);
        }
        if mode == b'B' {
            assert_eq!(n, Transfer::Bytes(64));
            assert_eq!(buf, [0; 64]);
        }
    }
}
#[test]
fn externally_reaped_child_is_lost_not_a_signal_target() {
    let mut c = child(b"X");
    let header = output(&mut c, 64);
    let pid = i32::from_le_bytes(header[24..28].try_into().expect("pid"));
    let end = Instant::now() + Duration::from_secs(1);
    let mut status = 0;
    let mut result = 0;
    while Instant::now() < end {
        result = unsafe { libc::waitpid(pid, &mut status, libc::WNOHANG) };
        if result == pid {
            break;
        }
        std::thread::yield_now();
    }
    assert_eq!(result, pid);
    assert_eq!(c.try_reap(), Err(HostError::CleanupPending));
    assert_eq!(c.terminate(), Err(HostError::CleanupPending));
}

#[test]
fn partial_fd_setup_and_default_sigpipe_are_confined_to_owned_peers() {
    for mode in [b'R', b'S'] {
        let mut c = child(&[mode]);
        let _ = output(&mut c, 64);
        assert_eq!(output(&mut c, 1), [1]);
        assert_eq!(reaped(&mut c), ProcessState::Exited { code: 0 });
    }
}

#[test]
fn watchdog_request_rejects_invalid_ceilings_without_changing_process_limits() {
    let page = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as usize;
    for ceiling in [0, 1, page, 1024 * 1024 + 1, usize::MAX] {
        assert_eq!(
            DarwinPlatform::watchdog_stack_request(ceiling),
            Err(HostError::InvalidLimits)
        );
    }
    for ceiling in [1024 * 1024, 1024 * 1024 - 1] {
        let requested = DarwinPlatform::watchdog_stack_request(ceiling).expect("finite request");
        assert_eq!(requested % page, 0);
        assert!(requested >= libc::PTHREAD_STACK_MIN);
        assert!(requested.checked_add(page).expect("bounded sum") <= ceiling);
    }
}

#[test]
fn incompatible_reaping_is_refused_before_spawn_without_mutating_parent_policy() {
    for mode in [b'N', b'G', b'T'] {
        let mut c = child(&[mode]);
        let _ = output(&mut c, 64);
        assert_eq!(
            output(&mut c, 4),
            [1, 1, 1, 1],
            "disposable peer mode {}",
            mode
        );
        assert_eq!(reaped(&mut c), ProcessState::Exited { code: 0 });
    }
}
#[test]
fn detected_reaping_policy_loss_latches_without_pid_signal_or_false_reap() {
    let mut c = child(b"L");
    let _ = output(&mut c, 64);
    assert_eq!(output(&mut c, 4), [1, 1, 1, 1]);
    assert_eq!(reaped(&mut c), ProcessState::Exited { code: 0 });
}
