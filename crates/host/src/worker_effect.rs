//! Real worker effect gate over the existing fixed parent protocol.
//! Kernel ports own typed lifecycle; parent alone owns claims/nonces/Possible.
use crate::worker_io::{self, WorkerIo};
use std::time::Instant;
use uiblueprint_host::{authority::TargetLease, *};
use uiblueprint_plugin_api::{
    ClockReading,
    actions::{ActionControl, DeliveryPermit, EffectGate, GateFailure},
};
use uiblueprint_schema::model::*;

pub(super) struct WorkerActionControl {
    clock: Id,
    origin: Instant,
}
impl WorkerActionControl {
    pub(super) fn new(clock: Id, origin: Instant) -> Self {
        Self { clock, origin }
    }
}
impl ActionControl for WorkerActionControl {
    fn now(&mut self) -> ClockReading {
        ClockReading {
            domain: self.clock.clone(),
            milliseconds: u64::try_from(self.origin.elapsed().as_millis()).unwrap_or(u64::MAX),
        }
    }
    fn cancelled(&self) -> bool {
        !worker_io::parent_alive(0)
    }
}

pub(super) struct WorkerEffectGate<'a> {
    io: &'a mut WorkerIo,
    operation: Control,
    target: TargetLease,
    clock: Id,
    deadline: Instant,
    requested: bool,
    nonce: Option<u64>,
}
impl<'a> WorkerEffectGate<'a> {
    /// Deadline/clock/TargetLease come from the real worker's trusted attach and
    /// parent-clamped operation owner, never from response/UI data.
    pub(super) fn new(
        io: &'a mut WorkerIo,
        operation: Control,
        target: TargetLease,
        clock: Id,
        deadline: Instant,
    ) -> Result<Self, HostError> {
        if operation.kind != ControlKind::Submit
            || operation.class != OperationClass::Mutation
            || operation.slot != 0
            || operation.flags & 7 != 1
            || operation.correlation.operation == 0
            || !target.permits(OperationClass::Mutation)
        {
            return Err(HostError::PermissionDenied);
        }
        if Instant::now() >= deadline {
            return Err(HostError::DeadlineExpired);
        }
        Ok(Self {
            io,
            operation,
            target,
            clock,
            deadline,
            requested: false,
            nonce: None,
        })
    }
    pub(super) fn nonce(&self) -> Option<u64> {
        self.nonce
    }
    pub(super) fn requested(&self) -> bool {
        self.requested
    }
    fn failure(&self, action: &Action, code: ErrorCode, recovery: &str) -> GateFailure {
        GateFailure {
            issue: Issue {
                code,
                scope_id: action.authorized_scope.clone(),
                failed_step: Some(action.id.clone()),
                recovery_class: Id(recovery.into()),
            },
            effect_possible: self.requested,
        }
    }
}

impl EffectGate for WorkerEffectGate<'_> {
    fn authorize(
        &mut self,
        action: &Action,
        now: &ClockReading,
    ) -> Result<DeliveryPermit, GateFailure> {
        if self.requested {
            return Err(self.failure(
                action,
                ErrorCode::ActionOutcomeUnknown,
                "effect_gate_already_used",
            ));
        }
        if !self.target.matches(&action.context.target) || now.domain != self.clock {
            return Err(self.failure(action, ErrorCode::StaleTarget, "invalid_effect_binding"));
        }
        if Instant::now() >= self.deadline || !worker_io::parent_alive(0) {
            return Err(self.failure(
                action,
                ErrorCode::Interrupted,
                "effect_gate_expired_or_cancelled",
            ));
        }
        let flags = u8::from(
            matches!(action.intent, Intent::Focus {})
                || matches!(
                    action.modality,
                    InputModality::Pointer
                        | InputModality::Keyboard
                        | InputModality::Touch
                        | InputModality::Remote
                ),
        );
        let ready = Control {
            kind: ControlKind::EffectReady,
            class: OperationClass::Mutation,
            slot: 0,
            flags,
            correlation: self.operation.correlation,
            length: 0,
            value: 0,
            auxiliary: 0,
        };
        // A partial request write/read failure cannot prove the parent did not
        // record Possible. There is no delivery permit on that uncertain path.
        self.requested = true;
        self.io.write_control(ready).map_err(|_| {
            self.failure(
                action,
                ErrorCode::ActionOutcomeUnknown,
                "effect_ready_transport_lost",
            )
        })?;
        let permit = self.io.control().map_err(|_| {
            self.failure(
                action,
                ErrorCode::ActionOutcomeUnknown,
                "effect_permit_transport_lost",
            )
        })?;
        if permit.kind != ControlKind::EffectPermit
            || permit.class != ready.class
            || permit.slot != 0
            || permit.flags != ready.flags
            || permit.correlation != ready.correlation
            || permit.length != 0
            || permit.value == 0
            || permit.auxiliary != 0
        {
            return Err(self.failure(
                action,
                ErrorCode::ActionOutcomeUnknown,
                "invalid_effect_permit",
            ));
        }
        self.nonce = Some(permit.value);
        if Instant::now() >= self.deadline || !worker_io::parent_alive(0) {
            return Err(self.failure(
                action,
                ErrorCode::ActionOutcomeUnknown,
                "effect_permit_late_or_cancelled",
            ));
        }
        DeliveryPermit::from_parent_nonce(permit.value).map_err(|_| {
            self.failure(
                action,
                ErrorCode::ActionOutcomeUnknown,
                "invalid_effect_nonce",
            )
        })
    }
}
