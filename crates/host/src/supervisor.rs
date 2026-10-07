//! Parent state machine. Canonical payload bytes remain opaque and are copied only
//! into fixed pre-reserved leases. No serde/graph decoding is permitted here.
use crate::{
    authority::TargetLease,
    domain::{DomainInner, HostDomain, RuntimeRoot, SessionHandle, SessionReservation, SlotPhase},
    host_types::*,
    process_api::{OwnedProcess, ProcessPlatform, ProcessState, SpawnSpec, Transfer},
    publication::Publication,
    worker_config::{MAX_CONFIG_BYTES, WorkerConfig},
    *,
};
use std::{
    mem::{forget, size_of},
    time::{Duration, Instant},
};

#[derive(Clone, Copy)]
enum TxStage {
    ConfigHeader,
    ConfigBody,
    SubmitHeader,
    Input,
    Ack,
    Idle,
}
struct Active<'a> {
    handle: OperationHandle<'a>,
    class: OperationClass,
    deadline: Instant,
    publish: Option<Publication<'a>>,
    effect: EffectReceipt,
    request: OutputRequest,
}
struct Worker<'a, C: OwnedProcess> {
    child: C,
    reservation: SessionReservation<'a>,
    config: [u8; MAX_CONFIG_BYTES],
    config_len: usize,
    tx: [u8; CONTROL_BYTES],
    tx_offset: usize,
    body_offset: usize,
    stage: TxStage,
    input: Option<InputLease<'a>>,
    rx: [u8; CONTROL_BYTES],
    rx_offset: usize,
    reading_body: bool,
    fatal: [u8; CONTROL_BYTES],
    fatal_offset: usize,
    active: Option<Active<'a>>,
    cleanup_deadline: Option<Instant>,
    quarantine_reported: bool,
    pending_terminal: Option<Terminal>,
    ready: bool,
}
struct RuntimeState<'a, P: ProcessPlatform> {
    platform: P,
    spec: SpawnSpec,
    workers: [Option<Worker<'a, P::Child>>; 4],
    shutting_down: bool,
    _root: RuntimeRoot<'a>,
}
pub struct RuntimeHost<'a, P: ProcessPlatform + 'static> {
    domain: &'a DomainInner,
    state: Vec<RuntimeState<'a, P>>,
}
impl<'a, P: ProcessPlatform + 'static> RuntimeHost<'a, P> {
    pub fn new(domain: &'a HostDomain, spec: SpawnSpec, platform: P) -> Result<Self, HostError> {
        let domain = domain.inner();
        if domain.platform != std::any::TypeId::of::<P>() {
            return Err(HostError::InvalidState);
        }
        let mut state = Vec::new();
        state
            .try_reserve_exact(1)
            .map_err(|_| HostError::AllocationFailure)?;
        let root = domain.reserve_runtime(add(
            size_of::<Self>(),
            mul(state.capacity(), size_of::<RuntimeState<'a, P>>())?,
        )?)?;
        state.push(RuntimeState {
            platform,
            spec,
            workers: std::array::from_fn(|_| None),
            shutting_down: false,
            _root: root,
        });
        Ok(Self { domain, state })
    }
    pub fn reserve_attach_input(
        &self,
        target: TargetLease,
        declared: usize,
    ) -> Result<AttachInput<'a>, HostError> {
        self.domain.check_reaping()?;
        if self.state[0].shutting_down {
            return Err(HostError::InvalidState);
        }
        let reservation = self.domain.reserve_session(target)?;
        let session = reservation.handle();
        let bytes = self.domain.buffers.reserve(
            BufferClass::Input {
                worker: session.slot,
            },
            declared,
        )?;
        Ok(AttachInput {
            input: InputLease { bytes, session },
            reservation,
            target,
        })
    }
    pub fn attach(
        &mut self,
        lease: AttachInput<'a>,
        deadline: Instant,
    ) -> Result<SessionHandle<'a>, HostError> {
        self.domain.check_reaping()?;
        if self.state[0].shutting_down {
            return Err(HostError::InvalidState);
        }
        remaining_ms(deadline, Instant::now())?;
        let AttachInput {
            input,
            mut reservation,
            target,
        } = lease;
        let session = reservation.handle();
        let mut config = [0; MAX_CONFIG_BYTES];
        let mut child_limits = self.domain.limits;
        child_limits.retained_per_worker = reservation.allowance();
        let config_len = WorkerConfig {
            limits: child_limits,
            target,
        }
        .encode(&mut config)?;
        let control = Control {
            kind: ControlKind::Configure,
            class: OperationClass::Attach,
            slot: 0,
            flags: 0,
            correlation: Correlation {
                session_epoch: session.epoch,
                operation: 0,
            },
            length: config_len as u64,
            value: 0,
            auxiliary: 0,
        };
        let state = &mut self.state[0];
        let child = state.platform.spawn(&state.spec)?;
        reservation.child_started();
        state.workers[session.slot] = Some(Worker {
            child,
            reservation,
            config,
            config_len,
            tx: control.encode(),
            tx_offset: 0,
            body_offset: 0,
            stage: TxStage::ConfigHeader,
            input: Some(input),
            rx: [0; CONTROL_BYTES],
            rx_offset: 0,
            reading_body: false,
            fatal: [0; CONTROL_BYTES],
            fatal_offset: 0,
            active: Some(Active {
                handle: OperationHandle {
                    session,
                    sequence: 0,
                },
                class: OperationClass::Attach,
                deadline,
                publish: None,
                effect: EffectReceipt::NotDispatched,
                request: OutputRequest {
                    channels: 0,
                    frame_bytes: self.domain.limits.output_bytes,
                    total_bytes: self.domain.limits.request_output_bytes,
                    input_format: 0,
                    retained_partition: 0,
                },
            }),
            cleanup_deadline: None,
            quarantine_reported: false,
            pending_terminal: None,
            ready: false,
        });
        Ok(session)
    }
    pub fn reserve_input(
        &self,
        session: SessionHandle<'a>,
        declared: usize,
    ) -> Result<InputLease<'a>, HostError> {
        self.domain.check_reaping()?;
        let state = self.domain.check(session)?;
        if state.phase != SlotPhase::Attached {
            return Err(HostError::Busy);
        }
        Ok(InputLease {
            bytes: self.domain.buffers.reserve(
                BufferClass::Input {
                    worker: session.slot,
                },
                declared,
            )?,
            session,
        })
    }
    pub fn submit(
        &mut self,
        session: SessionHandle<'a>,
        class: OperationClass,
        input: InputLease<'a>,
        request: OutputRequest,
        deadline: Instant,
    ) -> Result<OperationHandle<'a>, HostError> {
        self.domain.check_reaping()?;
        if self.state[0].shutting_down {
            return Err(HostError::InvalidState);
        }
        let mut slot = self.domain.check(session)?;
        if input.session != session
            || slot.phase != SlotPhase::Attached
            || class == OperationClass::Attach
        {
            return Err(HostError::StaleOperation);
        }
        if !slot.target.is_some_and(|target| target.permits(class)) {
            return Err(HostError::PermissionDenied);
        }
        if request.channels == 0
            || request.channels & !7 != 0
            || (class != OperationClass::Observe && request.channels != 1)
            || request.frame_bytes == 0
            || request.frame_bytes > self.domain.limits.output_bytes
            || request.total_bytes == 0
            || request.total_bytes > self.domain.limits.request_output_bytes
            || request.input_format > 1
            || request.retained_partition & !7 != 0
        {
            return Err(HostError::InvalidLimits);
        }
        let remaining = remaining_ms(deadline, Instant::now())?;
        let sequence = slot.sequence.checked_add(1).ok_or(HostError::Overflow)?;
        let handle = OperationHandle { session, sequence };
        let correlation = Correlation {
            session_epoch: session.epoch,
            operation: sequence,
        };
        let publish = Publication::new(
            self.domain.group(request.channels)?,
            correlation,
            class,
            request.channels,
            request.total_bytes,
        )?;
        let worker = self.state[0].workers[session.slot]
            .as_mut()
            .ok_or(HostError::StaleOperation)?;
        if worker.active.is_some() || !worker.ready {
            return Err(HostError::Busy);
        }
        let control = Control {
            kind: ControlKind::Submit,
            class,
            slot: 0,
            flags: request.channels
                | (request.input_format << 3)
                | (request.retained_partition << 4),
            correlation,
            length: input.bytes.len() as u64,
            value: remaining,
            auxiliary: request.frame_bytes as u64 | ((request.total_bytes as u64) << 32),
        };
        worker.tx = control.encode();
        worker.tx_offset = 0;
        worker.body_offset = 0;
        worker.stage = TxStage::SubmitHeader;
        worker.input = Some(input);
        worker.active = Some(Active {
            handle,
            class,
            deadline,
            publish: Some(publish),
            effect: EffectReceipt::NotDispatched,
            request,
        });
        slot.sequence = sequence;
        slot.phase = SlotPhase::Running;
        self.domain.slots[session.slot].set(slot);
        Ok(handle)
    }
    pub fn next_event(&mut self) -> Result<HostEvent<'a>, HostError> {
        if self.domain.check_reaping().is_err() {
            for worker in self.state[0].workers.iter_mut().flatten() {
                worker.child.close_input();
                worker.reservation.quarantine();
            }
            for worker in self.state[0].workers.iter_mut().flatten() {
                if worker.active.is_some() {
                    return Ok(HostEvent::Complete(quarantine_active(worker)));
                }
                if !worker.quarantine_reported {
                    worker.quarantine_reported = true;
                    return Ok(HostEvent::CleanupPending {
                        session: worker.reservation.handle(),
                    });
                }
            }
            return Err(HostError::CleanupPending);
        }
        let now = Instant::now();
        for index in 0..self.domain.limits.workers {
            let Some(worker) = self.state[0].workers[index].as_mut() else {
                continue;
            };
            if let Some(terminal) = worker.pending_terminal.take() {
                return Ok(HostEvent::Complete(take_completion(worker, terminal)));
            }
            if let Some(active) = &worker.active
                && now >= active.deadline
            {
                let event = terminalize(
                    worker,
                    Terminal::TimedOut,
                    self.domain.limits.cleanup_ms,
                    now,
                );
                return Ok(HostEvent::Complete(event));
            }
            if worker.cleanup_deadline.is_some() {
                match worker.child.try_reap() {
                    Ok(ProcessState::Exited { .. } | ProcessState::Signaled { .. }) => {
                        let handle = worker.reservation.handle();
                        worker.reservation.child_reaped();
                        self.state[0].workers[index] = None;
                        return Ok(HostEvent::Closed { session: handle });
                    }
                    Err(_) => worker.reservation.quarantine(),
                    _ => (),
                }
                if worker
                    .cleanup_deadline
                    .is_some_and(|deadline| now >= deadline)
                    && !worker.quarantine_reported
                {
                    worker.reservation.quarantine();
                    worker.quarantine_reported = true;
                    return Ok(HostEvent::CleanupPending {
                        session: worker.reservation.handle(),
                    });
                }
                continue;
            }
            match pump(worker, now, self.domain.limits.cleanup_ms) {
                Ok(Some(event)) => return Ok(event),
                Ok(None) => (),
                Err(error) => {
                    if worker.active.is_some() {
                        return Ok(HostEvent::Complete(terminalize(
                            worker,
                            Terminal::Failed(error),
                            self.domain.limits.cleanup_ms,
                            now,
                        )));
                    }
                    worker.child.close_input();
                    let _ = worker.child.terminate();
                    worker.cleanup_deadline =
                        now.checked_add(Duration::from_millis(self.domain.limits.cleanup_ms));
                }
            }
        }
        Ok(HostEvent::Pending)
    }
    pub fn cancel(
        &mut self,
        operation: OperationHandle<'a>,
    ) -> Result<HostCompletion<'a>, HostError> {
        self.domain.check_reaping()?;
        self.domain.check(operation.session)?;
        let worker = self.state[0].workers[operation.session.slot]
            .as_mut()
            .ok_or(HostError::StaleOperation)?;
        if worker.active.as_ref().is_none_or(|a| a.handle != operation) {
            return Err(HostError::StaleOperation);
        }
        Ok(terminalize(
            worker,
            Terminal::Cancelled,
            self.domain.limits.cleanup_ms,
            Instant::now(),
        ))
    }
    pub fn detach(
        &mut self,
        session: SessionHandle<'a>,
    ) -> Result<Option<HostCompletion<'a>>, HostError> {
        self.domain.check_reaping()?;
        self.domain.check(session)?;
        let worker = self.state[0].workers[session.slot]
            .as_mut()
            .ok_or(HostError::StaleOperation)?;
        if worker.active.is_some() {
            return Ok(Some(terminalize(
                worker,
                Terminal::Cancelled,
                self.domain.limits.cleanup_ms,
                Instant::now(),
            )));
        }
        worker.child.close_input();
        worker.child.terminate()?;
        worker.cleanup_deadline =
            Instant::now().checked_add(Duration::from_millis(self.domain.limits.cleanup_ms));
        Ok(None)
    }
    /// Initiate bounded cleanup and return terminal results one at a time.
    /// ACKed channels are transferred to the caller, never discarded by shutdown.
    pub fn shutdown(&mut self) -> Result<HostEvent<'a>, HostError> {
        self.state[0].shutting_down = true;
        if self.domain.check_reaping().is_err() {
            return self.next_event();
        }
        let now = Instant::now();
        for worker in self.state[0].workers.iter_mut().flatten() {
            if worker.cleanup_deadline.is_none() {
                worker.input = None;
                worker.stage = TxStage::Idle;
                worker.child.close_input();
                let _ = worker.child.terminate();
                worker.cleanup_deadline =
                    now.checked_add(Duration::from_millis(self.domain.limits.cleanup_ms));
                if worker.active.is_some() {
                    worker.pending_terminal = Some(Terminal::Cancelled);
                }
            }
        }
        if self.state[0].workers.iter().all(Option::is_none) {
            return Ok(HostEvent::ShutdownComplete);
        }
        self.next_event()
    }
}
impl<P: ProcessPlatform + 'static> Drop for RuntimeHost<'_, P> {
    fn drop(&mut self) {
        let until =
            Instant::now().checked_add(Duration::from_millis(self.domain.limits.cleanup_ms));
        while until.is_some_and(|deadline| Instant::now() < deadline) {
            match self.shutdown() {
                Ok(HostEvent::ShutdownComplete) => return,
                Ok(_) => std::thread::yield_now(),
                Err(_) => break,
            }
        }
        for worker in self.state[0].workers.iter_mut().flatten() {
            worker.child.close_input();
        }
        if self.state[0].workers.iter().any(Option::is_some) {
            self.domain.abandoned.set(true);
            forget(std::mem::take(&mut self.state));
        }
    }
}
fn take_completion<'a, C: OwnedProcess>(
    worker: &mut Worker<'a, C>,
    terminal: Terminal,
) -> HostCompletion<'a> {
    let active = worker
        .active
        .take()
        .expect("terminal publication requires an active operation");
    HostCompletion {
        operation: active.handle,
        class: active.class,
        terminal,
        effect: active.effect,
        frames: active.publish.map(Publication::finish),
    }
}

fn quarantine_active<'a, C: OwnedProcess>(worker: &mut Worker<'a, C>) -> HostCompletion<'a> {
    let active = worker
        .active
        .take()
        .expect("active checked before quarantine transfer");
    worker.input = None;
    worker.stage = TxStage::Idle;
    worker.child.close_input();
    worker.reservation.quarantine();
    // No kill/wait follows lost policy ownership. Already ACKed bytes move to
    // the caller while process/grant state remains quarantined in the host.
    HostCompletion {
        operation: active.handle,
        class: active.class,
        terminal: Terminal::Failed(HostError::CleanupPending),
        effect: active.effect,
        frames: active.publish.map(Publication::finish),
    }
}
fn terminalize<'a, C: OwnedProcess>(
    worker: &mut Worker<'a, C>,
    terminal: Terminal,
    cleanup_ms: u64,
    now: Instant,
) -> HostCompletion<'a> {
    let active = worker
        .active
        .take()
        .expect("terminalize called only for an active operation");
    worker.input = None;
    worker.stage = TxStage::Idle;
    worker.child.close_input();
    let _ = worker.child.terminate();
    worker.cleanup_deadline = now.checked_add(Duration::from_millis(cleanup_ms));
    HostCompletion {
        operation: active.handle,
        class: active.class,
        terminal,
        effect: active.effect,
        frames: active.publish.map(Publication::finish),
    }
}
fn written<C: OwnedProcess>(worker: &mut Worker<'_, C>, bytes: &[u8]) -> Result<usize, HostError> {
    match worker.child.write_input(bytes)? {
        Transfer::Bytes(0) | Transfer::Closed => Err(HostError::WorkerFailed),
        Transfer::Bytes(n) => Ok(n),
        Transfer::WouldBlock => Ok(0),
    }
}
fn pump<'a, C: OwnedProcess>(
    worker: &mut Worker<'a, C>,
    now: Instant,
    cleanup_ms: u64,
) -> Result<Option<HostEvent<'a>>, HostError> {
    // The authoritative deadline was checked before this bounded nonblocking step.
    if worker.fatal_offset < CONTROL_BYTES {
        if let Transfer::Bytes(n) = worker
            .child
            .read_fatal(&mut worker.fatal[worker.fatal_offset..])?
        {
            worker.fatal_offset += n;
        }
        if worker.fatal_offset == CONTROL_BYTES {
            let fatal = Control::decode(&worker.fatal)?;
            let active = worker.active.as_ref().ok_or(HostError::WorkerFailed)?;
            if fatal.kind != ControlKind::Fatal
                || fatal.correlation
                    != (Correlation {
                        session_epoch: active.handle.session.epoch,
                        operation: active.handle.sequence,
                    })
                || fatal.class != active.class
            {
                return Err(HostError::WorkerFailed);
            }
            let error = match fatal.value {
                1 => HostError::ResourceLimit,
                2 => HostError::SystemAllocationFailure,
                _ => HostError::WorkerFailed,
            };
            return Ok(Some(HostEvent::Complete(terminalize(
                worker,
                Terminal::Failed(error),
                cleanup_ms,
                now,
            ))));
        }
    }
    match worker.stage {
        TxStage::ConfigHeader | TxStage::SubmitHeader | TxStage::Ack => {
            let bytes = worker.tx;
            let count = written(worker, &bytes[worker.tx_offset..])?;
            worker.tx_offset += count;
            if worker.tx_offset == CONTROL_BYTES {
                worker.tx_offset = 0;
                worker.body_offset = 0;
                worker.stage = match worker.stage {
                    TxStage::ConfigHeader => TxStage::ConfigBody,
                    TxStage::SubmitHeader => TxStage::Input,
                    TxStage::Ack => {
                        let ack = Control::decode(&worker.tx)?;
                        worker
                            .active
                            .as_mut()
                            .and_then(|a| a.publish.as_mut())
                            .ok_or(HostError::InvalidState)?
                            .ack_sent(ack)?;
                        TxStage::Idle
                    }
                    _ => return Err(HostError::InvalidState),
                };
            }
        }
        TxStage::ConfigBody => {
            let count = match worker
                .child
                .write_input(&worker.config[worker.body_offset..worker.config_len])?
            {
                Transfer::Bytes(0) => return Err(HostError::WorkerFailed),
                Transfer::Bytes(n) => n,
                Transfer::WouldBlock => 0,
                Transfer::Closed => return Err(HostError::WorkerFailed),
            };
            worker.body_offset += count;
            if worker.body_offset == worker.config_len {
                let active = worker.active.as_ref().ok_or(HostError::InvalidState)?;
                worker.tx = Control {
                    kind: ControlKind::Submit,
                    class: OperationClass::Attach,
                    slot: 0,
                    flags: 0,
                    correlation: Correlation {
                        session_epoch: active.handle.session.epoch,
                        operation: 0,
                    },
                    length: worker
                        .input
                        .as_ref()
                        .ok_or(HostError::InvalidState)?
                        .bytes
                        .len() as u64,
                    value: remaining_ms(active.deadline, Instant::now())?,
                    auxiliary: 0,
                }
                .encode();
                worker.tx_offset = 0;
                worker.stage = TxStage::SubmitHeader;
            }
        }
        TxStage::Input => {
            let input = worker.input.as_ref().ok_or(HostError::InvalidState)?;
            let count = if worker.body_offset == input.bytes.len() {
                0
            } else {
                match worker
                    .child
                    .write_input(&input.bytes.as_slice()[worker.body_offset..])?
                {
                    Transfer::Bytes(0) => return Err(HostError::WorkerFailed),
                    Transfer::Bytes(n) => n,
                    Transfer::WouldBlock => 0,
                    Transfer::Closed => return Err(HostError::WorkerFailed),
                }
            };
            worker.body_offset += count;
            if worker.body_offset == input.bytes.len() {
                worker.input = None;
                worker.stage = TxStage::Idle;
            }
        }
        TxStage::Idle => (),
    }
    if worker.reading_body {
        let publication = worker
            .active
            .as_mut()
            .and_then(|a| a.publish.as_mut())
            .ok_or(HostError::InvalidState)?;
        match worker.child.read_output(publication.remaining_mut()?)? {
            Transfer::Bytes(n) => publication.advance(n)?,
            Transfer::Closed => return Err(HostError::WorkerFailed),
            Transfer::WouldBlock => (),
        }
        if publication.remaining_mut()?.is_empty() {
            worker.reading_body = false;
        }
        return Ok(None);
    }
    match worker
        .child
        .read_output(&mut worker.rx[worker.rx_offset..])?
    {
        Transfer::Bytes(n) => worker.rx_offset += n,
        Transfer::Closed => return Err(HostError::WorkerFailed),
        Transfer::WouldBlock => return Ok(None),
    }
    if worker.rx_offset < CONTROL_BYTES {
        return Ok(None);
    }
    worker.rx_offset = 0;
    let control = Control::decode(&worker.rx)?;
    let active = worker.active.as_mut().ok_or(HostError::StaleOperation)?;
    if control.correlation
        != (Correlation {
            session_epoch: active.handle.session.epoch,
            operation: active.handle.sequence,
        })
        || control.class != active.class
        || Instant::now() >= active.deadline
    {
        return Err(HostError::StaleOperation);
    }
    match control.kind {
        ControlKind::Ready if active.class == OperationClass::Attach => {
            if control.slot != 0
                || control.flags != 0
                || control.length != 0
                || control.auxiliary == 0
                || control.auxiliary > worker.reservation.domain.limits.watchdog_stack_bytes as u64
            {
                return Err(HostError::InvalidControl);
            }
            worker.ready = true;
            let handle = active.handle.session;
            worker.active = None;
            let mut slot = worker.reservation.domain.slots[handle.slot].get();
            slot.phase = SlotPhase::Attached;
            worker.reservation.domain.slots[handle.slot].set(slot);
            Ok(Some(HostEvent::Attached {
                session: handle,
                clock: clock_id(handle.epoch)?,
            }))
        }
        ControlKind::Frame => {
            if control.length > active.request.frame_bytes as u64 {
                return Err(HostError::ResourceLimit);
            }
            active
                .publish
                .as_mut()
                .ok_or(HostError::InvalidState)?
                .begin(control)?;
            worker.reading_body = true;
            Ok(None)
        }
        ControlKind::Commit => {
            let ack = active
                .publish
                .as_mut()
                .ok_or(HostError::InvalidState)?
                .commit(control)?;
            worker.tx = ack.encode();
            worker.tx_offset = 0;
            worker.stage = TxStage::Ack;
            Ok(None)
        }
        ControlKind::Terminal => {
            if control.slot != 0
                || control.flags != 0
                || control.length != 0
                || (control.value == 0
                    && active
                        .publish
                        .as_ref()
                        .is_none_or(|p| p.committed() != active.request.channels))
            {
                return Err(HostError::InvalidControl);
            }
            let terminal = match control.value {
                0 => Terminal::Completed,
                1 => Terminal::Failed(HostError::ResourceLimit),
                2 => Terminal::Failed(HostError::PermissionDenied),
                3 => Terminal::Failed(HostError::ResyncRequired),
                4 => Terminal::TimedOut,
                5 => Terminal::Failed(HostError::InvalidInput),
                _ => return Err(HostError::InvalidControl),
            };
            let active = worker.active.take().ok_or(HostError::InvalidState)?;
            let mut slot = worker.reservation.domain.slots[active.handle.session.slot].get();
            slot.phase = SlotPhase::Attached;
            worker.reservation.domain.slots[active.handle.session.slot].set(slot);
            Ok(Some(HostEvent::Complete(HostCompletion {
                operation: active.handle,
                class: active.class,
                terminal,
                effect: active.effect,
                frames: active.publish.map(Publication::finish),
            })))
        }
        _ => Err(HostError::InvalidControl),
    }
}
