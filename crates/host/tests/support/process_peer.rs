//! Non-UI test peer only. Commands are fixed bytes on the owned input lane.
use std::{
    os::fd::{AsRawFd, FromRawFd, OwnedFd},
    sync::atomic::{AtomicUsize, Ordering},
    time::{Duration, Instant},
};
use uiblueprint_host::{process::DarwinPlatform, process_api::*};

static WATCHDOG_STACK: AtomicUsize = AtomicUsize::new(0);
extern "C" fn interrupted(_: i32) {}
fn send(fd: i32, bytes: &[u8]) {
    // SAFETY: bytes remains readable for its full length; send borrows the inherited lane and suppresses SIGPIPE.
    unsafe {
        libc::send(fd, bytes.as_ptr().cast(), bytes.len(), libc::MSG_NOSIGNAL);
    }
}
fn byte() -> u8 {
    let mut value = 0;
    // SAFETY: The owned inherited input lane writes at most one byte into live value storage.
    if unsafe { libc::read(3, (&mut value as *mut u8).cast(), 1) } != 1 {
        // SAFETY: Terminate only this disposable peer after failed input; no Rust cleanup is required.
        unsafe { libc::_exit(80) }
    }
    value
}
fn info(main_stack: usize) -> [u8; 64] {
    let mut value = [0u8; 64];
    value[0] = 0xa1;
    value[1] = std::env::vars_os().count() as u8;
    value[2] = std::env::args_os().count() as u8;
    value[3] = (6..256)
        // SAFETY: F_GETFD only queries an integer descriptor and accepts closed descriptors with an error.
        .filter(|fd| unsafe { libc::fcntl(*fd, libc::F_GETFD) } >= 0)
        .count() as u8;
    // SAFETY: F_GETFL queries the inherited fatal lane without a pointer argument or ownership transfer.
    value[4] = u8::from(unsafe { libc::fcntl(5, libc::F_GETFL) } & libc::O_NONBLOCK != 0);
    let mut null_stat = std::mem::MaybeUninit::<libc::stat>::uninit();
    // SAFETY: The path is NUL-terminated and null_stat provides writable storage for one stat record.
    if unsafe { libc::stat(c"/dev/null".as_ptr(), null_stat.as_mut_ptr()) } != 0 {
        // SAFETY: Terminate this disposable peer rather than reading an uninitialized stat record.
        unsafe { libc::_exit(86) }
    }
    // SAFETY: The successful stat call above initialized the complete record.
    let null_device = unsafe { null_stat.assume_init() }.st_rdev;
    for fd in 0..3 {
        let mut stat = std::mem::MaybeUninit::<libc::stat>::uninit();
        // SAFETY: stat has writable storage for one record; invalid descriptors return an error.
        if unsafe { libc::fstat(fd, stat.as_mut_ptr()) } == 0 {
            // SAFETY: The preceding successful fstat initialized the complete record.
            let actual = unsafe { stat.assume_init() };
            if actual.st_mode & libc::S_IFMT == libc::S_IFCHR && actual.st_rdev == null_device {
                value[5 + fd as usize] = 1;
            }
        }
    }
    value[8..16].copy_from_slice(&(main_stack as u64).to_le_bytes());
    value[16..24].copy_from_slice(&(WATCHDOG_STACK.load(Ordering::Acquire) as u64).to_le_bytes());
    // SAFETY: getpid has no pointer inputs or ownership transfer.
    value[24..28].copy_from_slice(&unsafe { libc::getpid() }.to_le_bytes());
    let mut limit = libc::rlimit {
        rlim_cur: 1,
        rlim_max: 1,
    };
    // SAFETY: limit is a live writable record and RLIMIT_CORE is a valid resource selector.
    if unsafe { libc::getrlimit(libc::RLIMIT_CORE, &mut limit) } == 0
        && limit.rlim_cur == 0
        && limit.rlim_max == 0
    {
        value[28] = 1;
    }
    value
}
fn collision() {
    let path = std::env::current_exe().expect("test executable path");
    let spec = SpawnSpec::new(&path).expect("bounded own peer path");
    // SAFETY: Duplicate the inherited output lane into a new independently owned descriptor above the lanes.
    let output = unsafe { libc::fcntl(4, libc::F_DUPFD_CLOEXEC, 100) };
    if output < 0 {
        // SAFETY: Only this disposable peer exits when descriptor duplication fails.
        unsafe { libc::_exit(81) }
    }
    // SAFETY: Successful fcntl returned a new descriptor that has no other Rust owner.
    let output = unsafe { OwnedFd::from_raw_fd(output) };
    // Only this disposable peer's six inherited owned lanes are touched.
    for fd in 0..6 {
        // SAFETY: Only this disposable peer owns these inherited raw lanes; none has a Rust FD owner.
        unsafe {
            libc::close(fd);
        }
    }
    let mut platform = DarwinPlatform;
    let mut child = platform.spawn(&spec).expect("collision spawn");
    child.write_input(b"I").expect("mode");
    let end = Instant::now() + Duration::from_secs(1);
    let mut bytes = [0u8; 64];
    let mut used = 0;
    while used < 64 && Instant::now() < end {
        match child.read_output(&mut bytes[used..]).expect("peer output") {
            Transfer::Bytes(n) => used += n,
            Transfer::WouldBlock => std::thread::yield_now(),
            Transfer::Closed => break,
        }
    }
    let mut result = [0u8; 64];
    result[0] = u8::from(used == 64 && bytes[0] == 0xa1 && bytes[3] == 0);
    child.terminate().expect("owned termination");
    while Instant::now() < end {
        if child.try_reap().expect("reap") != ProcessState::Running {
            result[1] = 1;
            break;
        }
        std::thread::yield_now();
    }
    send(output.as_raw_fd(), &result);
}
fn partial_setup() {
    let spec = SpawnSpec::new(&std::env::current_exe().expect("own peer")).expect("spec");
    let count = || {
        (0..32)
            // SAFETY: F_GETFD queries integer descriptors without dereferencing pointers or taking ownership.
            .filter(|fd| unsafe { libc::fcntl(*fd, libc::F_GETFD) } >= 0)
            .count()
    };
    let before = count();
    let mut limit = libc::rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };
    // SAFETY: limit is writable initialized storage for the valid RLIMIT_NOFILE query.
    unsafe {
        libc::getrlimit(libc::RLIMIT_NOFILE, &mut limit);
    }
    limit.rlim_cur = 10;
    // SAFETY: limit is a live record; changing the limit affects only this disposable peer.
    if unsafe { libc::setrlimit(libc::RLIMIT_NOFILE, &limit) } != 0 {
        // SAFETY: Only this disposable peer exits on setup failure.
        unsafe { libc::_exit(85) }
    }
    let failed = DarwinPlatform.spawn(&spec).is_err();
    send(4, &[u8::from(failed && count() == before)]);
}
fn closed_sigpipe() {
    let spec = SpawnSpec::new(&std::env::current_exe().expect("own peer")).expect("spec");
    let mut child = DarwinPlatform.spawn(&spec).expect("own nested peer");
    child.write_input(b"D").expect("mode");
    let mut data = [0u8; 65];
    let mut used = 0;
    let end = Instant::now() + Duration::from_secs(1);
    while used < data.len() && Instant::now() < end {
        match child.read_output(&mut data[used..]).expect("read") {
            Transfer::Bytes(n) => used += n,
            Transfer::WouldBlock => std::thread::yield_now(),
            Transfer::Closed => break,
        }
    }
    // Only this owned test process: prove per-send suppression even with default SIGPIPE.
    // SAFETY: SIG_DFL is the valid sentinel for SIGPIPE; the change is confined to this owned test process.
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    let closed = child.write_input(b"x") == Ok(Transfer::Closed);
    child.terminate().expect("terminate");
    let mut reaped = false;
    while Instant::now() < end {
        if child.try_reap().expect("reap") != ProcessState::Running {
            reaped = true;
            break;
        }
        std::thread::yield_now();
    }
    send(4, &[u8::from(used == 65 && closed && reaped)]);
}
fn interrupt_poll() {
    // SAFETY: Darwin sigaction contains integer fields; the handler and empty signal mask are set below.
    let mut action: libc::sigaction = unsafe { std::mem::zeroed() };
    action.sa_sigaction = interrupted as *const () as usize;
    // SAFETY: The mask is writable and the installed extern C handler is non-unwinding and does no work.
    unsafe {
        libc::sigemptyset(&mut action.sa_mask);
        libc::sigaction(libc::SIGUSR1, &action, std::ptr::null_mut());
    }
    // SAFETY: pthread_self returns the identity of the live calling thread without transferring ownership.
    let main = unsafe { libc::pthread_self() } as usize;
    let signaler = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(20));
        // SAFETY: The main thread remains alive until join; SIGUSR1 uses the no-op handler installed above.
        unsafe {
            libc::pthread_kill(main as libc::pthread_t, libc::SIGUSR1);
        }
    });
    let start = Instant::now();
    let result = DarwinPlatform.poll(&mut [], 500);
    let elapsed = start.elapsed().as_millis() as u64;
    signaler.join().expect("bounded signaling thread");
    let mut report = [0u8; 64];
    report[0] = u8::from(result.is_ok());
    report[8..16].copy_from_slice(&elapsed.to_le_bytes());
    send(4, &report);
}
fn signal_action(handler: usize, flags: i32) -> libc::sigaction {
    // SAFETY: Darwin sigaction contains integer fields, initialized here before setting handler and mask.
    let mut action: libc::sigaction = unsafe { std::mem::zeroed() };
    action.sa_sigaction = handler;
    action.sa_flags = flags;
    // SAFETY: action.sa_mask is writable storage for a complete Darwin signal set.
    unsafe {
        libc::sigemptyset(&mut action.sa_mask);
    }
    action
}
fn query_chld() -> libc::sigaction {
    let mut action = std::mem::MaybeUninit::uninit();
    // SAFETY: A null new-action pointer performs a query into writable sigaction storage.
    if unsafe { libc::sigaction(libc::SIGCHLD, std::ptr::null(), action.as_mut_ptr()) } != 0 {
        // SAFETY: Terminate this peer rather than expose an uninitialized action record.
        unsafe { libc::_exit(88) }
    }
    // SAFETY: The successful sigaction query initialized the complete action record.
    unsafe { action.assume_init() }
}
fn apply_chld(action: &libc::sigaction) {
    // SAFETY: action is a live record built from valid signal constants or the no-op extern C handler.
    if unsafe { libc::sigaction(libc::SIGCHLD, action, std::ptr::null_mut()) } != 0 {
        // SAFETY: Only this disposable peer exits on a failed policy change.
        unsafe { libc::_exit(89) }
    }
}
fn policy_refusal(mode: u8) {
    let original = query_chld();
    let action = match mode {
        b'N' => signal_action(libc::SIG_DFL, libc::SA_NOCLDWAIT),
        b'G' => signal_action(libc::SIG_IGN, 0),
        _ => signal_action(interrupted as *const () as usize, 0),
    };
    // Only this disposable peer changes policy; no managed child exists yet.
    apply_chld(&action);
    let before = query_chld();
    let validation_refused =
        DarwinPlatform::validate_parent_reaping() == Err(uiblueprint_host::HostError::InvalidState);
    let fd_count = || {
        (0..256)
            // SAFETY: F_GETFD queries each integer descriptor without changing its ownership.
            .filter(|fd| unsafe { libc::fcntl(*fd, libc::F_GETFD) } >= 0)
            .count()
    };
    let count = fd_count();
    let spec = SpawnSpec::new(&std::env::current_exe().expect("own peer")).expect("spec");
    let refused = match DarwinPlatform.spawn(&spec) {
        Err(uiblueprint_host::HostError::InvalidState) => true,
        Ok(mut unexpected) => {
            // Safe regression failure: never invoke the potentially unsafe old
            // termination path while auto-reap is enabled. EOF bounds the peer.
            unexpected.close_input();
            std::mem::forget(unexpected);
            false
        }
        Err(_) => false,
    };
    let after = query_chld();
    let unchanged = before.sa_sigaction == after.sa_sigaction
        && before.sa_flags == after.sa_flags
        && before.sa_mask == after.sa_mask;
    let balanced = fd_count() == count;
    let mut status = 0;
    let no_child = refused
        // SAFETY: This isolated peer has no unmanaged children; status is writable and WNOHANG is nonblocking.
        && unsafe { libc::waitpid(-1, &mut status, libc::WNOHANG) } == -1
        // SAFETY: __error returns this thread's errno storage; it is read immediately after waitpid failed.
        && unsafe { *libc::__error() } == libc::ECHILD;
    apply_chld(&original);
    send(
        4,
        &[
            u8::from(refused && validation_refused),
            u8::from(unchanged),
            u8::from(balanced),
            u8::from(no_child),
        ],
    );
}
fn policy_drift() {
    let original = query_chld();
    let spec = SpawnSpec::new(&std::env::current_exe().expect("own peer")).expect("spec");
    let mut child = DarwinPlatform.spawn(&spec).expect("supported spawn");
    child.write_input(b"D").expect("bounded self-exiting peer");
    let mut header = [0u8; 65];
    let mut at = 0;
    let end = Instant::now() + Duration::from_secs(1);
    while at < header.len() && Instant::now() < end {
        match child.read_output(&mut header[at..]).expect("read") {
            Transfer::Bytes(n) => at += n,
            Transfer::WouldBlock => std::thread::yield_now(),
            Transfer::Closed => break,
        }
    }
    if at != 65 {
        // SAFETY: Only this disposable peer exits when its bounded child handshake fails.
        unsafe { libc::_exit(90) }
    }
    let pid = i32::from_le_bytes(header[24..28].try_into().expect("owned pid"));
    apply_chld(&signal_action(libc::SIG_DFL, libc::SA_NOCLDWAIT));
    let lost = DarwinPlatform::validate_parent_reaping()
        == Err(uiblueprint_host::HostError::InvalidState)
        && child.try_reap() == Err(uiblueprint_host::HostError::CleanupPending);
    let refused = child.terminate() == Err(uiblueprint_host::HostError::CleanupPending);
    // Let the small peer exit itself; never chase PID reuse or send after loss.
    let mut eof = false;
    while Instant::now() < end {
        match child
            .read_output(&mut [0u8; 1])
            .expect("bounded output EOF")
        {
            Transfer::Closed => {
                eof = true;
                break;
            }
            _ => std::thread::yield_now(),
        }
    }
    let mut status = 0;
    let mut auto_reaped = false;
    // EOF may precede final kernel exit cleanup. It is not reap evidence.
    while eof && Instant::now() < end {
        // SAFETY: pid was read from our own child handshake; status is writable and this call only waits.
        let waited = unsafe { libc::waitpid(pid, &mut status, libc::WNOHANG) };
        // SAFETY: __error returns current-thread errno storage, read only after waitpid reports an error.
        if waited == -1 && unsafe { *libc::__error() } == libc::ECHILD {
            auto_reaped = true;
            break;
        }
        if waited != 0 {
            break;
        }
        std::thread::yield_now();
    }
    apply_chld(&original);
    let latched = DarwinPlatform::validate_parent_reaping().is_ok()
        && child.try_reap() == Err(uiblueprint_host::HostError::CleanupPending)
        && child.terminate() == Err(uiblueprint_host::HostError::CleanupPending);
    send(
        4,
        &[
            u8::from(lost),
            u8::from(refused),
            u8::from(auto_reaped),
            u8::from(latched),
        ],
    );
}
fn main() {
    let stack = match DarwinPlatform::setup_main(8 * 1024 * 1024) {
        Ok(n) => n,
        // SAFETY: Only this disposable peer exits on its own stack setup failure.
        Err(_) => unsafe { libc::_exit(82) },
    };
    // Production caller must use the same provider and verify actual extent.
    let requested =
        DarwinPlatform::watchdog_stack_request(1024 * 1024).expect("validated watchdog request");
    std::thread::Builder::new()
        .stack_size(requested)
        .spawn(|| {
            if DarwinPlatform::setup_main(8 * 1024 * 1024)
                != Err(uiblueprint_host::HostError::InvalidState)
            {
                DarwinPlatform::fatal_exit(-1, &[0; 64], 84);
            }
            WATCHDOG_STACK.store(
                DarwinPlatform::current_stack_bytes().unwrap_or(0),
                Ordering::Release,
            );
            std::thread::sleep(Duration::from_secs(2));
            DarwinPlatform::fatal_exit(-1, &[0; 64], 99);
        })
        .expect("bounded watchdog thread");
    let until = Instant::now() + Duration::from_millis(200);
    while WATCHDOG_STACK.load(Ordering::Acquire) == 0 && Instant::now() < until {
        std::thread::yield_now();
    }
    send(4, &info(stack));
    let mode = byte();
    match mode {
        b'I' => {}
        b'E' => send(4, &[byte()]),
        // SAFETY: This fixed test mode exits only the disposable peer with the expected status.
        b'X' => unsafe { libc::_exit(17) },
        b'H' => std::thread::sleep(Duration::from_millis(1000)),
        b'D' => {
            // SAFETY: This peer solely owns raw input lane 3; it has no File or OwnedFd wrapper.
            unsafe {
                libc::close(3);
            }
            send(4, &[1]);
            std::thread::sleep(Duration::from_millis(200));
        }
        b'F' => DarwinPlatform::fatal_exit(5, &[0xa5; 64], 77),
        b'M' => DarwinPlatform::fatal_exit(-1, &[0xa5; 64], 78),
        b'B' => {
            for _ in 0..8192 {
                // SAFETY: The 64-byte temporary stays live through send; fd5 is inherited and send is nonblocking.
                if unsafe {
                    libc::send(
                        5,
                        [0u8; 64].as_ptr().cast(),
                        64,
                        libc::MSG_DONTWAIT | libc::MSG_NOSIGNAL,
                    )
                } < 0
                {
                    break;
                }
            }
            DarwinPlatform::fatal_exit(5, &[0xa5; 64], 79);
        }
        b'C' => collision(),
        b'P' => interrupt_poll(),
        b'R' => partial_setup(),
        b'S' => closed_sigpipe(),
        b'N' | b'G' | b'T' => policy_refusal(mode),
        b'L' => policy_drift(),
        // SAFETY: An unknown fixed command terminates only this disposable peer.
        _ => unsafe { libc::_exit(83) },
    }
}
