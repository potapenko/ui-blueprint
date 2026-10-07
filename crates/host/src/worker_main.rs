//! Real session-worker entry. Only this executable installs the quota allocator.
use crate::{
    quota_allocator as guard,
    worker_io::{self, WorkerIo},
    worker_ops::CanonicalSession,
};
use std::{
    io::{self, Write},
    sync::{
        OnceLock,
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
    },
    time::Instant,
};
use uiblueprint_engine::cache::{LedgerLimits, QuotaLedger};
use uiblueprint_host::{
    host_types::clock_id,
    process::DarwinPlatform,
    process_api::{CHILD_FATAL_FD, WorkerPlatform},
    worker_config::{MAX_CONFIG_BYTES, WorkerConfig},
    *,
};
static ORIGIN: OnceLock<Instant> = OnceLock::new();
static DEADLINE: AtomicU64 = AtomicU64::new(u64::MAX);
static STOP: AtomicBool = AtomicBool::new(false);
static WATCHDOG_STACK: AtomicUsize = AtomicUsize::new(0);
// Declared after WorkerIo, so every early-return path joins the FD borrower
// before the operation thread drops its inherited endpoint owners.
struct Watchdog(Option<std::thread::JoinHandle<()>>);
impl Drop for Watchdog {
    fn drop(&mut self) {
        STOP.store(true, Ordering::Release);
        if let Some(thread) = self.0.take() {
            let _ = thread.join();
        }
    }
}
fn now() -> u64 {
    u64::try_from(
        ORIGIN
            .get()
            .expect("worker origin installed before watchdog")
            .elapsed()
            .as_millis(),
    )
    .unwrap_or(u64::MAX)
}
pub(super) fn clock_origin() -> Instant {
    *ORIGIN
        .get()
        .expect("origin installed before producer attach")
}
pub(super) fn tighten_deadline(remaining: u64) -> Result<Instant, HostError> {
    if remaining == 0 {
        return Err(HostError::DeadlineExpired);
    }
    let end = now()
        .checked_add(remaining)
        .ok_or(HostError::Overflow)?
        .min(DEADLINE.load(Ordering::Acquire));
    DEADLINE.store(end, Ordering::Release);
    clock_origin()
        .checked_add(std::time::Duration::from_millis(end))
        .ok_or(HostError::Overflow)
}
pub(super) fn admit_observation(
    io: &mut WorkerIo,
    operation: Control,
    ticket: u64,
    request_deadline: u64,
    channels: u8,
) -> Result<Instant, HostError> {
    if operation.class != OperationClass::Observe {
        return Err(HostError::InvalidInput);
    }
    admit_request(io, operation, ticket, request_deadline, channels)
}
#[cfg(feature = "web")]
pub(super) fn admit_action(
    io: &mut WorkerIo,
    operation: Control,
    request_deadline: u64,
) -> Result<Instant, HostError> {
    if !matches!(
        operation.class,
        OperationClass::Prepare | OperationClass::Mutation
    ) || operation.flags & 8 == 0
    {
        return Err(HostError::InvalidInput);
    }
    admit_request(
        io,
        operation,
        operation.correlation.operation,
        request_deadline,
        1,
    )
}
fn admit_request(
    io: &mut WorkerIo,
    operation: Control,
    ticket: u64,
    request_deadline: u64,
    channels: u8,
) -> Result<Instant, HostError> {
    if ticket == 0 || request_deadline == 0 || channels != operation.flags & 7 {
        return Err(HostError::InvalidInput);
    }
    io.write_control(Control {
        kind: ControlKind::ObserveReady,
        class: operation.class,
        slot: 0,
        flags: channels,
        correlation: operation.correlation,
        length: 0,
        value: ticket,
        auxiliary: request_deadline,
    })?;
    let permit = io.control()?;
    if permit.kind != ControlKind::ObservePermit
        || permit.class != operation.class
        || permit.correlation != operation.correlation
        || permit.slot != 0
        || permit.flags != channels
        || permit.length != 0
        || permit.value != ticket
    {
        return Err(HostError::InvalidControl);
    }
    tighten_deadline(permit.auxiliary)
}
pub(super) struct FixedOutput<'a> {
    pub bytes: &'a mut [u8],
    pub used: usize,
}
impl Write for FixedOutput<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let end = self
            .used
            .checked_add(bytes.len())
            .ok_or(io::ErrorKind::WriteZero)?;
        if end > self.bytes.len() {
            return Err(io::ErrorKind::WriteZero.into());
        }
        self.bytes[self.used..end].copy_from_slice(bytes);
        self.used = end;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
fn fatal(error: HostError) -> ! {
    guard::fatal(
        match error {
            HostError::DeadlineExpired => guard::FatalReason::Deadline,
            HostError::ResourceLimit => guard::FatalReason::Quota,
            HostError::Io => guard::FatalReason::ParentGone,
            _ => guard::FatalReason::Configuration,
        },
        0,
    )
}
pub(super) fn publish(
    io: &mut WorkerIo,
    operation: Control,
    slot: u8,
    bytes: &[u8],
    flags: u8,
) -> Result<(), HostError> {
    if operation.flags & (1 << slot) == 0 || bytes.len() > (operation.auxiliary as u32) as usize {
        return Err(HostError::ResourceLimit);
    }
    let header = Control {
        kind: ControlKind::Frame,
        class: operation.class,
        slot,
        flags,
        correlation: operation.correlation,
        length: bytes.len() as u64,
        value: 0,
        auxiliary: 0,
    };
    io.write_control(header)?;
    io.write(bytes)?;
    io.write_control(Control {
        kind: ControlKind::Commit,
        ..header
    })?;
    let ack = io.control()?;
    if ack.kind != ControlKind::Ack
        || ack.flags != header.flags
        || ack.value != 0
        || ack.auxiliary != 0
        || !ack.matches(header)
    {
        return Err(HostError::InvalidControl);
    }
    Ok(())
}
fn control_error(error: HostError) -> u64 {
    match error {
        HostError::ResourceLimit => 1,
        HostError::PermissionDenied => 2,
        HostError::ResyncRequired => 3,
        HostError::DeadlineExpired => 4,
        HostError::Io | HostError::WorkerFailed => 6,
        HostError::InvalidControl => 7,
        HostError::CleanupPending => 8,
        HostError::ActionRefused => 9,
        _ => 5,
    }
}

pub fn run() -> Result<(), HostError> {
    // Pre-input bootstrap uses the adopted maximum; accepted trusted setup then
    // verifies the actual extent against the caller's possibly smaller ceiling.
    DarwinPlatform::setup_main(8 * 1_048_576)?;
    std::panic::set_hook(Box::new(|_| guard::fatal(guard::FatalReason::Panic, 0)));
    let mut io = WorkerIo::inherited()?;
    let configure = io.control()?;
    if configure.kind != ControlKind::Configure
        || configure.class != OperationClass::Attach
        || configure.correlation.session_epoch == 0
        || configure.correlation.operation != 0
        || configure.slot != 0
        || configure.flags != 0
        || configure.value != 0
        || configure.auxiliary != 0
        || configure.length as usize > MAX_CONFIG_BYTES
    {
        return Err(HostError::InvalidControl);
    }
    let mut configuration = [0; MAX_CONFIG_BYTES];
    let size = usize::try_from(configure.length).map_err(|_| HostError::Overflow)?;
    io.read(&mut configuration[..size])?;
    let configuration = WorkerConfig::decode(&configuration[..size])?;
    let limits = configuration.limits;
    DarwinPlatform::setup_main(limits.main_stack_bytes)?;
    guard::configure(
        configure.correlation.session_epoch,
        limits.ordinary_bytes()?,
        limits.worker_bytes,
        limits.bootstrap_bytes,
        CHILD_FATAL_FD,
    );
    ORIGIN
        .set(Instant::now())
        .map_err(|_| HostError::InvalidState)?;
    let request = DarwinPlatform::watchdog_stack_request(limits.watchdog_stack_bytes)?;
    let watchdog = Watchdog(Some(
        std::thread::Builder::new()
            .stack_size(request)
            .spawn(move || {
                let actual = DarwinPlatform::current_stack_bytes()
                    .unwrap_or_else(|_| guard::fatal(guard::FatalReason::Configuration, 0));
                if actual > limits.watchdog_stack_bytes {
                    guard::fatal(guard::FatalReason::Configuration, actual as u64);
                }
                WATCHDOG_STACK.store(actual, Ordering::Release);
                while !STOP.load(Ordering::Acquire) {
                    let current = now();
                    let deadline = DEADLINE.load(Ordering::Acquire);
                    if current >= deadline {
                        guard::fatal(guard::FatalReason::Deadline, 0);
                    }
                    let wait = deadline.saturating_sub(current).min(10) as i32;
                    if !worker_io::parent_alive(wait) {
                        guard::fatal(guard::FatalReason::ParentGone, 0);
                    }
                }
            })
            .map_err(|_| HostError::ResourceLimit)?,
    ));
    while WATCHDOG_STACK.load(Ordering::Acquire) == 0 {
        std::thread::yield_now();
    }
    // Fixed allocations occur before any untrusted decode/collection.
    let mut input = Vec::new();
    input
        .try_reserve_exact(limits.input_bytes)
        .map_err(|_| HostError::ResourceLimit)?;
    input.resize(limits.input_bytes, 0);
    let mut publication = Vec::new();
    publication
        .try_reserve_exact(limits.output_bytes)
        .map_err(|_| HostError::ResourceLimit)?;
    publication.resize(limits.output_bytes, 0);
    let ledger = QuotaLedger::new(LedgerLimits {
        retained_bytes: limits.retained_per_worker,
        session_slots: 1,
        grants: 1,
    })
    .map_err(|_| HostError::ResourceLimit)?;
    let attach = io.control()?;
    if attach.kind != ControlKind::Submit
        || attach.class != OperationClass::Attach
        || attach.correlation != configure.correlation
        || attach.length as usize > input.len()
        || attach.flags & !128 != 0
        || attach.slot != 0
        || attach.auxiliary != 0
    {
        return Err(HostError::InvalidControl);
    }
    DEADLINE.store(
        now().checked_add(attach.value).ok_or(HostError::Overflow)?,
        Ordering::Release,
    );
    guard::operation(0, OperationClass::Attach, guard::Phase::Decode);
    let size = usize::try_from(attach.length).map_err(|_| HostError::Overflow)?;
    io.read(&mut input[..size])?;
    let clock = clock_id(configure.correlation.session_epoch)?;
    let web_attach = attach.flags & 128 != 0;
    #[cfg(not(feature = "web"))]
    if web_attach {
        return Err(HostError::PermissionDenied);
    }
    let tape = if web_attach {
        Some(crate::worker_tape::Tape::decode(&input[..size])?)
    } else {
        None
    };
    if tape.as_ref().is_some_and(|t| t.count() != 2) {
        return Err(HostError::InvalidInput);
    }
    let descriptor = if let Some(t) = &tape {
        t.get(0)?
    } else {
        &input[..size]
    };
    let mut session = CanonicalSession::attach(
        descriptor,
        configuration.target,
        uiblueprint_schema::model::Id(clock.as_str().into()),
        now(),
        &ledger,
        limits,
    )?;
    #[cfg(feature = "web")]
    let mut web_session = if let Some(t) = &tape {
        let setup = uiblueprint_host::web_config::WebSetup::decode(t.get(1)?, limits.input_bytes)?;
        Some(crate::worker_web::WebSession::attach(
            descriptor,
            setup,
            configuration.target,
            uiblueprint_schema::model::Id(clock.as_str().into()),
            clock_origin(),
            limits,
            tighten_deadline(attach.value)?,
        )?)
    } else {
        None
    };
    io.write_control(Control {
        kind: ControlKind::Ready,
        class: OperationClass::Attach,
        slot: 0,
        flags: 0,
        correlation: attach.correlation,
        length: 0,
        value: now(),
        auxiliary: WATCHDOG_STACK.load(Ordering::Acquire) as u64,
    })?;
    DEADLINE.store(u64::MAX, Ordering::Release);
    let mut sequence = 0;
    loop {
        let operation = io.control()?;
        if operation.kind == ControlKind::Shutdown {
            break;
        }
        if operation.kind != ControlKind::Submit
            || operation.class == OperationClass::Attach
            || operation.correlation.session_epoch != configure.correlation.session_epoch
            || operation.correlation.operation <= sequence
            || operation.length as usize > input.len()
            || operation.slot != 0
            || (operation.flags & 128 != 0 && operation.class != OperationClass::Observe)
            || operation.flags & 7 == 0
            || (operation.class != OperationClass::Observe && operation.flags & 7 != 1)
            || operation.auxiliary as u32 == 0
            || operation.auxiliary as u32 as usize > limits.output_bytes
            || operation.auxiliary >> 32 == 0
            || operation.auxiliary >> 32 > limits.request_output_bytes as u64
        {
            return Err(HostError::InvalidControl);
        }
        sequence = operation.correlation.operation;
        // No failure metadata survives reuse, including a successful prior call.
        let _ = io.take_diagnostic();
        guard::operation(sequence, operation.class, guard::Phase::Decode);
        DEADLINE.store(
            now()
                .checked_add(operation.value)
                .ok_or(HostError::Overflow)?,
            Ordering::Release,
        );
        let size = usize::try_from(operation.length).map_err(|_| HostError::Overflow)?;
        io.read(&mut input[..size])?;
        let result = if matches!(
            operation.class,
            OperationClass::Prepare | OperationClass::Mutation
        ) {
            #[cfg(feature = "web")]
            {
                match web_session.as_mut() {
                    Some(web) if operation.class == OperationClass::Prepare => web.prepare_action(
                        &mut session,
                        &mut io,
                        &mut publication,
                        operation,
                        &input[..size],
                    ),
                    Some(web) => web.act(
                        &mut session,
                        &mut io,
                        &mut publication,
                        operation,
                        &input[..size],
                    ),
                    None => Err(HostError::PermissionDenied),
                }
            }
            #[cfg(not(feature = "web"))]
            {
                Err(HostError::PermissionDenied)
            }
        } else if operation.class == OperationClass::Observe
            && operation.flags & 128 != 0
            && operation.flags & 8 != 0
        {
            #[cfg(feature = "web")]
            {
                match web_session.as_mut() {
                    Some(web) => web
                        .observe(
                            &mut session,
                            &mut io,
                            &mut publication,
                            operation,
                            &input[..size],
                            now,
                        )
                        .map(|_| 0),
                    None => Err(HostError::PermissionDenied),
                }
            }
            #[cfg(not(feature = "web"))]
            {
                Err(HostError::PermissionDenied)
            }
        } else if operation.class == OperationClass::Observe && operation.flags & 128 != 0 {
            let mut exchange =
                crate::worker_native::NativeExchange::new(&mut io, &mut publication, operation);
            session
                .observe_native(&input[..size], now, operation.flags & 7, &mut exchange)
                .map(|_| 0)
        } else if operation.class == OperationClass::Observe {
            let mut total = 0usize;
            session
                .observe(&input[..size], now, operation.flags & 7, |slot, bytes| {
                    total = total.checked_add(bytes.len()).ok_or(HostError::Overflow)?;
                    if total > (operation.auxiliary >> 32) as usize {
                        return Err(HostError::ResourceLimit);
                    }
                    if bytes.len() > publication.len() {
                        return Err(HostError::ResourceLimit);
                    }
                    publication[..bytes.len()].copy_from_slice(bytes);
                    publish(&mut io, operation, slot, &publication[..bytes.len()], 0)
                })
                .map(|_| 0)
        } else {
            let mut encoded = FixedOutput {
                bytes: &mut publication,
                used: 0,
            };
            session
                .execute(
                    operation.class,
                    &input[..size],
                    now,
                    &mut encoded,
                    (operation.flags >> 3) & 1,
                    (operation.flags >> 4) & 7,
                )
                .and_then(|value| {
                    publish(&mut io, operation, 0, &encoded.bytes[..encoded.used], 0)?;
                    Ok(value)
                })
        };
        let (code, value) = match result {
            Ok(value) => (0, value),
            Err(error) => (control_error(error), 0),
        };
        let mut terminal = Control {
            kind: ControlKind::Terminal,
            class: operation.class,
            slot: 0,
            flags: 0,
            correlation: operation.correlation,
            length: 0,
            value: code,
            auxiliary: value,
        };
        if let Some(record) = io.take_diagnostic()
            && code != 0
            && operation.class == OperationClass::Observe
            && operation.flags & 136 == 136
        {
            record.apply(&mut terminal)?;
        }
        io.write_control(terminal)?;
        DEADLINE.store(u64::MAX, Ordering::Release);
        input[..size].fill(0);
    }
    drop(watchdog);
    drop(session);
    Ok(())
}
pub fn main_entry() {
    if let Err(error) = run() {
        fatal(error);
    }
}
