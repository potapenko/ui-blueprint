#![cfg(target_os = "macos")]
//! Finite parent-thread allocation observation, not an enforcing parent allocator
//! or an RSS/SDK bound. The production child still installs its actual quota guard.
use std::{
    alloc::{GlobalAlloc, Layout, System},
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
    time::{Duration, Instant},
};
use uiblueprint_host::{
    authority::TargetLease,
    domain::HostDomain,
    host_types::{HostCompletion, HostEvent, OutputRequest, Terminal},
    native_binding::NativeHelperBinding,
    process::DarwinPlatform,
    process_api::SpawnSpec,
    supervisor::RuntimeHost,
    *,
};
use uiblueprint_schema::model::{Artifact, Channel, Document, Id, Operation};
static OWNER: AtomicUsize = AtomicUsize::new(0);
static CALLS: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);
struct ObservedSystem;
fn observe(bytes: usize) {
    // SAFETY: pthread_self reads this thread's identity, with no pointer inputs,
    // Rust allocation, lock, formatting or lifetime transfer.
    if OWNER.load(Ordering::Relaxed) == unsafe { libc::pthread_self() as usize } {
        CALLS.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(bytes, Ordering::Relaxed);
    }
}
// SAFETY: exact valid GlobalAlloc pointer/layout arguments are forwarded to System.
// Observation uses only fixed atomics/thread identity and never unwinds or allocates.
unsafe impl GlobalAlloc for ObservedSystem {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        observe(l.size());
        unsafe { System.alloc(l) }
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        observe(l.size());
        unsafe { System.alloc_zeroed(l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        observe(n);
        unsafe { System.realloc(p, l, n) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        unsafe { System.dealloc(p, l) }
    }
}
#[global_allocator]
static ALLOCATOR: ObservedSystem = ObservedSystem;
struct Measuring;
impl Drop for Measuring {
    fn drop(&mut self) {
        OWNER.store(0, Ordering::SeqCst);
    }
}
fn measured<T>(action: impl FnOnce() -> Result<T, HostError>) -> (T, (usize, usize)) {
    CALLS.store(0, Ordering::Relaxed);
    BYTES.store(0, Ordering::Relaxed);
    // SAFETY: this test exclusively measures its own current thread. Other test
    // harness threads and owned child processes are not part of this observation.
    OWNER.store(unsafe { libc::pthread_self() as usize }, Ordering::SeqCst);
    let measuring = Measuring;
    let value = action();
    drop(measuring);
    (
        value.expect("measured bounded operation"),
        (CALLS.load(Ordering::Relaxed), BYTES.load(Ordering::Relaxed)),
    )
}
const MIB: usize = 1_048_576;
const DESCRIPTOR: &[u8] = include_bytes!("../../../fixtures/golden/ENV-CAPABILITY-VALID.json");
const QUERY: &[u8] = include_bytes!("../../../fixtures/analysis/query-gap.json");
fn limits() -> HostLimits {
    HostLimits {
        workers: 2,
        worker_bytes: 64 * MIB,
        publication_reserve: MIB,
        bootstrap_bytes: MIB,
        parent_bytes: 32 * MIB,
        input_bytes: 2 * MIB,
        ingress_bytes: 512 * 1024,
        output_bytes: 512 * 1024,
        request_output_bytes: 2 * MIB,
        completion_groups: 2,
        control_bytes: 4096,
        cleanup_ms: 1000,
        retained_domain_bytes: 64 * MIB,
        retained_per_worker: 15 * MIB,
        main_stack_bytes: 8 * MIB,
        watchdog_stack_bytes: MIB,
    }
}
fn deadline() -> Instant {
    Instant::now() + Duration::from_secs(5)
}
fn next<'a>(host: &mut RuntimeHost<'a, DarwinPlatform>) -> Result<HostEvent<'a>, HostError> {
    let end = deadline();
    loop {
        if Instant::now() >= end {
            return Err(HostError::DeadlineExpired);
        }
        match host.next_event()? {
            HostEvent::Pending | HostEvent::HelperClosed { .. } => std::thread::yield_now(),
            event => return Ok(event),
        }
    }
}
fn completion<'a>(
    host: &mut RuntimeHost<'a, DarwinPlatform>,
) -> Result<HostCompletion<'a>, HostError> {
    match next(host)? {
        HostEvent::Complete(c) => Ok(c),
        _ => Err(HostError::InvalidState),
    }
}
fn request(channels: u8) -> OutputRequest {
    OutputRequest {
        channels,
        frame_bytes: 65536,
        total_bytes: 131072,
        input_format: 1,
        retained_partition: 0,
    }
}
#[test]
fn precharged_parent_paths_do_not_add_rust_heap_allocations() {
    let (probe, observed) = measured(|| {
        let v = vec![0x5a_u8; 64];
        std::hint::black_box(&v);
        Ok(v)
    });
    assert!(
        observed.0 > 0 && observed.1 >= 64,
        "positive control must see real storage"
    );
    drop(probe);
    let Artifact::Session(descriptor) = Document::from_json(DESCRIPTOR, DESCRIPTOR.len())
        .unwrap()
        .artifact
    else {
        panic!("session")
    };
    let authority = TargetLease::authorized(&descriptor.target, false).unwrap();
    let worker = SpawnSpec::new(Path::new(env!("CARGO_BIN_EXE_session-worker"))).unwrap();
    let peer_path = Path::new(env!("CARGO_BIN_EXE_session-worker"))
        .parent()
        .unwrap()
        .join("examples/native_peer");
    assert!(peer_path.is_file());
    let binding =
        NativeHelperBinding::authorized(SpawnSpec::new(&peer_path).unwrap(), 3, b"F").unwrap();
    let (domain, domain_setup) = measured(|| HostDomain::new::<DarwinPlatform>(limits()));
    let (mut host, runtime_setup) = measured(|| RuntimeHost::new(&domain, worker, DarwinPlatform));
    let inventory = domain.usage().parent_owned_bytes;
    let measured_backing = domain_setup.1 + runtime_setup.1;
    let inline_roots =
        std::mem::size_of::<HostDomain>() + std::mem::size_of::<RuntimeHost<'_, DarwinPlatform>>();
    assert_eq!(
        inventory,
        measured_backing + inline_roots,
        "reported inventory must include every observed Rust setup allocation plus inline roots"
    );
    eprintln!(
        "parent_inventory owned={inventory} requested={measured_backing} inline={inline_roots} allocations={}",
        domain_setup.0 + runtime_setup.0
    );
    let ((first, clock), counts) = measured(|| {
        let mut lease = host.reserve_attach_input(authority, DESCRIPTOR.len())?;
        lease.bytes_mut().copy_from_slice(DESCRIPTOR);
        let first = host.attach(lease, deadline())?;
        let HostEvent::Attached { clock, .. } = next(&mut host)? else {
            return Err(HostError::WorkerFailed);
        };
        Ok((first, clock))
    });
    assert_eq!(counts, (0, 0), "attach and clock framing");
    let (_, counts) = measured(|| {
        let mut input = host.reserve_input(first, QUERY.len())?;
        input.bytes_mut().copy_from_slice(QUERY);
        host.submit(
            first,
            OperationClass::Validate,
            input,
            request(1),
            deadline(),
        )?;
        let c = completion(&mut host)?;
        if c.terminal != Terminal::Completed {
            return Err(HostError::WorkerFailed);
        };
        Ok(())
    });
    assert_eq!(counts, (0, 0), "ordinary request/ACK/release");
    let mut doc = Document::from_json(
        include_bytes!("../../../fixtures/golden/ENV-REQUEST-VALID.json"),
        65536,
    )
    .unwrap();
    let Artifact::Request(r) = &mut doc.artifact else {
        panic!("request")
    };
    r.clock_domain = Id(clock.as_str().into());
    r.limits.deadline_ms = 1000;
    r.limits.max_output_bytes = 131072;
    r.operation = Operation::Observe {
        channels: vec![Channel::ExternalSemantics, Channel::RenderedCapture],
    };
    let encoded = serde_json::to_vec(&doc).unwrap();
    let (held, counts) = measured(|| {
        host.configure_native_helpers(first, binding)?;
        let mut input = host.reserve_input(first, encoded.len())?;
        input.bytes_mut().copy_from_slice(&encoded);
        host.submit_native_observe(first, input, request(3), deadline())?;
        completion(&mut host)
    });
    assert_eq!(
        counts,
        (0, 0),
        "Native helper exchange/validation bytes/ACK"
    );
    assert_eq!(held.committed(), 3);
    let (second, counts) = measured(|| {
        let mut input = host.reserve_attach_input(authority, DESCRIPTOR.len())?;
        input.bytes_mut().copy_from_slice(DESCRIPTOR);
        let s = host.attach(input, deadline())?;
        if !matches!(next(&mut host)?, HostEvent::Attached { .. }) {
            return Err(HostError::WorkerFailed);
        }
        let mut input = host.reserve_input(s, QUERY.len())?;
        input.bytes_mut().copy_from_slice(QUERY);
        host.submit(s, OperationClass::Validate, input, request(1), deadline())?;
        completion(&mut host)
    });
    assert_eq!(counts, (0, 0), "B progresses while A result retained");
    assert_eq!(second.bytes(0), Some(QUERY));
    let (refusal, counts) = measured(|| {
        let input = host.reserve_input(first, 1)?;
        Ok(host
            .submit(
                first,
                OperationClass::Validate,
                input,
                request(1),
                deadline(),
            )
            .err())
    });
    assert_eq!(refusal, Some(HostError::ResourceLimit));
    assert_eq!(counts, (0, 0), "saturation before dispatch");
    assert_eq!(domain.usage().parent_owned_bytes, inventory);
    let (_, counts) = measured(|| {
        drop(held);
        drop(second);
        let end = deadline();
        loop {
            if Instant::now() >= end {
                return Err(HostError::DeadlineExpired);
            }
            if matches!(host.shutdown()?, HostEvent::ShutdownComplete) {
                return Ok(());
            }
            std::thread::yield_now();
        }
    });
    assert_eq!(counts, (0, 0), "completion and confirmed cleanup");
    assert_eq!(domain.usage().reserved_sessions, 0);
    assert_eq!(domain.usage().parent_owned_bytes, inventory);
}
