//! Test-only transparent Darwin wrapper: delay actual controls, never forge ACKs.
use std::{
    os::fd::BorrowedFd,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
    },
};
use uiblueprint_host::{process::DarwinPlatform, process_api::*, *};
#[derive(Default)]
pub struct Trace {
    pub hold_permit: AtomicBool,
    pub permit_waiting: AtomicBool,
    pub permits: AtomicUsize,
    pub ticket: AtomicU64,
    pub observe_acks: AtomicUsize,
    pub hold_second_ack: AtomicBool,
    pub second_ack_waiting: AtomicBool,
    pub effect_permits: AtomicUsize,
    pub mutation_acks: AtomicUsize,
    pub prepare_acks: AtomicUsize,
    pub hold_effect_permit: AtomicBool,
    pub effect_waiting: AtomicBool,
}
pub struct Platform(pub Arc<Trace>);
pub struct Child {
    inner: <DarwinPlatform as ProcessPlatform>::Child,
    trace: Arc<Trace>,
    pending: Option<(ControlKind, u64, usize)>,
}
impl ProcessPlatform for Platform {
    type Child = Child;
    fn validate_parent_reaping() -> Result<(), HostError> {
        DarwinPlatform::validate_parent_reaping()
    }
    fn spawn(&mut self, spec: &SpawnSpec) -> Result<Child, HostError> {
        Ok(Child {
            inner: DarwinPlatform.spawn(spec)?,
            trace: self.0.clone(),
            pending: None,
        })
    }
    fn poll(&mut self, interests: &mut [PollInterest<'_>], wait_ms: u32) -> Result<(), HostError> {
        DarwinPlatform.poll(interests, wait_ms)
    }
}
impl OwnedProcess for Child {
    fn write_input(&mut self, bytes: &[u8]) -> Result<Transfer, HostError> {
        if self.pending.is_none() && bytes.len() == CONTROL_BYTES {
            let fixed: &[u8; CONTROL_BYTES] =
                bytes.try_into().map_err(|_| HostError::InvalidControl)?;
            if let Ok(control) = Control::decode(fixed) {
                if control.kind == ControlKind::ObservePermit {
                    if self.trace.hold_permit.load(Ordering::Acquire) {
                        self.trace.permit_waiting.store(true, Ordering::Release);
                        return Ok(Transfer::WouldBlock);
                    }
                    self.pending = Some((control.kind, control.value, CONTROL_BYTES));
                } else if control.kind == ControlKind::EffectPermit {
                    if self.trace.hold_effect_permit.load(Ordering::Acquire) {
                        self.trace.effect_waiting.store(true, Ordering::Release);
                        return Ok(Transfer::WouldBlock);
                    }
                    self.pending = Some((control.kind, control.value, CONTROL_BYTES));
                } else if control.kind == ControlKind::Ack
                    && matches!(
                        control.class,
                        OperationClass::Mutation | OperationClass::Prepare
                    )
                {
                    self.pending = Some((
                        control.kind,
                        u64::from(control.class == OperationClass::Mutation) + 1,
                        CONTROL_BYTES,
                    ));
                } else if control.kind == ControlKind::Ack
                    && control.class == OperationClass::Observe
                {
                    if self.trace.hold_second_ack.load(Ordering::Acquire)
                        && self.trace.observe_acks.load(Ordering::Acquire) >= 1
                    {
                        self.trace.second_ack_waiting.store(true, Ordering::Release);
                        return Ok(Transfer::WouldBlock);
                    }
                    self.pending = Some((control.kind, 0, CONTROL_BYTES));
                }
            }
        }
        self.forward(bytes)
    }
    fn read_output(&mut self, bytes: &mut [u8]) -> Result<Transfer, HostError> {
        self.inner.read_output(bytes)
    }
    fn read_fatal(&mut self, bytes: &mut [u8]) -> Result<Transfer, HostError> {
        self.inner.read_fatal(bytes)
    }
    fn close_input(&mut self) {
        self.inner.close_input()
    }
    fn terminate(&mut self) -> Result<(), HostError> {
        self.inner.terminate()
    }
    fn try_reap(&mut self) -> Result<ProcessState, HostError> {
        self.inner.try_reap()
    }
    fn input_fd(&self) -> Option<BorrowedFd<'_>> {
        self.inner.input_fd()
    }
    fn output_fd(&self) -> BorrowedFd<'_> {
        self.inner.output_fd()
    }
    fn fatal_fd(&self) -> BorrowedFd<'_> {
        self.inner.fatal_fd()
    }
}
impl Child {
    fn forward(&mut self, bytes: &[u8]) -> Result<Transfer, HostError> {
        let transfer = self.inner.write_input(bytes)?;
        if let Transfer::Bytes(count) = transfer
            && let Some((kind, ticket, remaining)) = self.pending
        {
            let remaining = remaining
                .checked_sub(count)
                .ok_or(HostError::InvalidControl)?;
            if remaining == 0 {
                match kind {
                    ControlKind::ObservePermit => {
                        self.trace.ticket.store(ticket, Ordering::Release);
                        self.trace.permits.fetch_add(1, Ordering::AcqRel);
                    }
                    ControlKind::Ack => match ticket {
                        1 => {
                            self.trace.prepare_acks.fetch_add(1, Ordering::AcqRel);
                        }
                        2 => {
                            self.trace.mutation_acks.fetch_add(1, Ordering::AcqRel);
                        }
                        _ => {
                            self.trace.observe_acks.fetch_add(1, Ordering::AcqRel);
                        }
                    },
                    ControlKind::EffectPermit => {
                        self.trace.effect_permits.fetch_add(1, Ordering::AcqRel);
                    }
                    _ => (),
                };
                self.pending = None;
            } else {
                self.pending = Some((kind, ticket, remaining));
            }
        }
        Ok(transfer)
    }
}
