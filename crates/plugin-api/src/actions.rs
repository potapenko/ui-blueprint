//! One SetChecked step; trusted ports own current UI/authority/effect evidence.
//! No SDK, input transport, nonce generator or automatic retry exists here.
use crate::{ClockReading, Error};
use uiblueprint_schema::{
    model::*,
    validation::{self, ValidationError},
};

/// Move-only authorization from the existing parent effect owner. Construction
/// is for trusted gate code after checking actual correlation/permission, never
/// from a canonical/UI payload. The parent retains its Possible receipt outside
/// the worker before granting this nonce. This type does not enforce OS authority.
pub struct DeliveryPermit {
    nonce: u64,
}
impl DeliveryPermit {
    pub fn from_parent_nonce(nonce: u64) -> Result<Self, Error> {
        if nonce == 0 {
            return Err(Error::InvalidFrame);
        }
        Ok(Self { nonce })
    }
    pub fn nonce(&self) -> u64 {
        self.nonce
    }
}
/// Gate failure must truthfully retain uncertainty if the parent's effect
/// boundary was crossed before transport failed. It cannot become NotDispatched.
pub struct GateFailure {
    pub issue: Issue,
    pub effect_possible: bool,
}
pub trait EffectGate {
    fn authorize(
        &mut self,
        action: &Action,
        now: &ClockReading,
    ) -> Result<DeliveryPermit, GateFailure>;
}
pub trait ActionControl {
    fn now(&mut self) -> ClockReading;
    fn cancelled(&self) -> bool;
}
/// Implementations must freshly revalidate the requested exact opaque backend
/// identity. They may refresh Observation/Snapshot bindings for that same handle,
/// but must refuse lost/remounted/ambiguous handles rather than resolve by label.
/// Read/dispatch calls must respect the passed remaining local deadline and the
/// real parent's watchdog/cleanup. Saved Current flags do not satisfy this port.
/// The caller pins the provider to authorized Target/Surface/scope before invoking
/// it; requested UI records cannot broaden that attachment. observe_after must
/// perform a new exact-target read, not return a saved Current-labeled snapshot.
/// deliver uses the same held identity and checks continuity at the setter point;
/// a lost object cannot cause fallback to a replacement with the same label/ID.
pub trait SetCheckedProvider {
    fn resolve_exact(
        &mut self,
        requested: &ActionCase,
        now: &ClockReading,
        remaining_ms: u64,
    ) -> Result<ActionCase, Issue>;
    fn deliver(
        &mut self,
        action: &Action,
        permit: DeliveryPermit,
        remaining_ms: u64,
    ) -> DeliveryStatus;
    fn observe_after(
        &mut self,
        action: &Action,
        now: &ClockReading,
        remaining_ms: u64,
    ) -> Result<Snapshot, Issue>;
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Prepared,
    PendingVerification,
    Finished,
}

/// Owns existing canonical records; called inside the caller's admitted memory
/// boundary. A resolver's returned ActionCase/after Snapshot are existing DTOs,
/// not a second graph. The caller/worker guard owns transient overlap allocations.
pub struct SetCheckedExecution {
    current: ActionCase,
    after: Option<Snapshot>,
    transition_id: Id,
    step: Step,
    clock: Id,
    last_clock: u64,
    deadline: u64,
    possible: bool,
    phase: Phase,
    issue: Option<Issue>,
}
impl SetCheckedExecution {
    /// Preparation validates declarations; it does not grant mutation authority
    /// or claim that the saved source is still current. Dispatch invokes fresh
    /// trusted resolution and the parent's effect gate independently.
    pub fn prepare(
        case: ActionCase,
        transition_id: Id,
        step_id: Id,
        start: ClockReading,
        remaining_ms: u64,
    ) -> Result<Self, ValidationError> {
        validation::validate_snapshot(&case.snapshot)?;
        validation::validate_action(&case.snapshot, &case.action)?;
        if !matches!(case.action.intent, Intent::SetChecked { .. })
            || [&transition_id, &step_id, &start.domain]
                .iter()
                .any(|id| id.0.is_empty() || id.0.chars().count() > 256)
        {
            return Err(ValidationError::InvalidDocument);
        }
        let deadline = start
            .milliseconds
            .checked_add(remaining_ms)
            .filter(|_| remaining_ms > 0)
            .ok_or(ValidationError::ResourceLimit)?;
        let step = Step {
            id: step_id,
            action_id: case.action.id.clone(),
            delivery: DeliveryStatus::NotDispatched,
            outcome: Outcome::PendingVerification,
            before_snapshot: case.snapshot.id.clone(),
            after_snapshot: None,
            verification_observation: None,
        };
        Ok(Self {
            current: case,
            after: None,
            transition_id,
            step,
            clock: start.domain,
            last_clock: start.milliseconds,
            deadline,
            possible: false,
            phase: Phase::Prepared,
            issue: None,
        })
    }
    pub fn step(&self) -> &Step {
        &self.step
    }
    pub fn issue(&self) -> Option<&Issue> {
        self.issue.as_ref()
    }
    fn issue_for(&self, code: ErrorCode, recovery: &str) -> Issue {
        Issue {
            code,
            scope_id: self.current.snapshot.context.scope_id.clone(),
            failed_step: Some(self.step.id.clone()),
            recovery_class: Id(recovery.into()),
        }
    }
    fn stop(&mut self, issue: Issue) {
        self.step.outcome = if self.possible {
            Outcome::ActionOutcomeUnknown
        } else {
            Outcome::Interrupted
        };
        self.issue = Some(issue);
        self.phase = Phase::Finished;
    }
    fn check(&mut self, now: &ClockReading, cancelled: bool) -> Result<u64, Error> {
        if now.domain != self.clock || now.milliseconds < self.last_clock {
            self.stop(self.issue_for(ErrorCode::Interrupted, "invalid_clock"));
            return Err(Error::InvalidClock);
        }
        self.last_clock = now.milliseconds;
        if cancelled {
            self.stop(self.issue_for(ErrorCode::Interrupted, "reobserve_before_continue"));
            return Err(Error::Detached);
        }
        if now.milliseconds >= self.deadline {
            self.stop(self.issue_for(ErrorCode::Timeout, "reobserve_before_continue"));
            return Err(Error::DeadlineExpired);
        }
        Ok(self.deadline - now.milliseconds)
    }
    fn same_requested_identity(&self, fresh: &ActionCase) -> bool {
        let a = &self.current.action;
        let b = &fresh.action;
        validation::contexts_compatible(&self.current.snapshot.context, &fresh.snapshot.context)
            && a.id == b.id
            && a.intent == b.intent
            && a.modality == b.modality
            && a.input_space == b.input_space
            && a.required_enabled == b.required_enabled
            && a.authorized_scope == b.authorized_scope
            && a.backend_ref.session_id == b.backend_ref.session_id
            && a.backend_ref.key == b.backend_ref.key
            && a.backend_ref.target == b.backend_ref.target
            && a.backend_ref.surface == b.backend_ref.surface
    }
    /// Resolves the exact selected handle immediately before the parent permit,
    /// then invokes delivery once. Calls after any attempt are refused. Errors
    /// retain the Step/Issue, including effect uncertainty, for finish().
    pub fn dispatch(
        &mut self,
        provider: &mut impl SetCheckedProvider,
        gate: &mut impl EffectGate,
        control: &mut impl ActionControl,
    ) -> Result<DeliveryStatus, Error> {
        if self.phase != Phase::Prepared {
            return Err(Error::StaleTicket);
        }
        let now = control.now();
        let remaining = self.check(&now, control.cancelled())?;
        let fresh = match provider.resolve_exact(&self.current, &now, remaining) {
            Ok(case) => case,
            Err(issue) => {
                self.step.outcome = Outcome::Failed;
                self.issue = Some(issue);
                self.phase = Phase::Finished;
                return Ok(DeliveryStatus::NotDispatched);
            }
        };
        let invalid = validation::validate_snapshot(&fresh.snapshot)
            .and_then(|_| validation::validate_action(&fresh.snapshot, &fresh.action))
            .err();
        if invalid.is_some() || !self.same_requested_identity(&fresh) {
            let code = match invalid {
                Some(ValidationError::AmbiguousTarget) => ErrorCode::AmbiguousTarget,
                Some(ValidationError::UnknownMeasurement) => ErrorCode::Unsupported,
                _ => ErrorCode::StaleTarget,
            };
            self.step.outcome = Outcome::Failed;
            self.issue = Some(self.issue_for(code, "resolve_exact_current_target"));
            self.phase = Phase::Finished;
            return Ok(DeliveryStatus::NotDispatched);
        }
        self.current = fresh;
        self.step.before_snapshot = self.current.snapshot.id.clone();
        let now = control.now();
        self.check(&now, control.cancelled())?;
        let permit = match gate.authorize(&self.current.action, &now) {
            Ok(permit) => permit,
            Err(failure) => {
                self.possible = failure.effect_possible;
                self.step.delivery = if self.possible {
                    DeliveryStatus::Unknown
                } else {
                    DeliveryStatus::NotDispatched
                };
                self.step.outcome = if self.possible {
                    Outcome::ActionOutcomeUnknown
                } else {
                    Outcome::Failed
                };
                self.issue = Some(failure.issue);
                self.phase = Phase::Finished;
                return Ok(self.step.delivery);
            }
        };
        self.possible = true;
        self.step.delivery = DeliveryStatus::Unknown;
        let now = control.now();
        let remaining = self.check(&now, control.cancelled())?;
        let delivered = provider.deliver(&self.current.action, permit, remaining);
        self.step.delivery = match delivered {
            DeliveryStatus::Accepted | DeliveryStatus::Confirmed => delivered,
            // Parent already crossed Possible; lacking confirmation cannot reset
            // that authoritative uncertainty or permit a retry.
            DeliveryStatus::NotDispatched | DeliveryStatus::Unknown => DeliveryStatus::Unknown,
        };
        if self.step.delivery == DeliveryStatus::Unknown {
            self.stop(self.issue_for(
                ErrorCode::ActionOutcomeUnknown,
                "establish_outcome_before_retry",
            ));
        } else {
            self.phase = Phase::PendingVerification;
            let now = control.now();
            self.check(&now, control.cancelled())?;
        }
        Ok(self.step.delivery)
    }
    /// Freshly observes the exact target through the trusted provider. Matching
    /// Checked is insufficient without confirmed delivery and current, correctly
    /// bound Evidence. Unknown/changed binding never manufactures success.
    pub fn verify(
        &mut self,
        provider: &mut impl SetCheckedProvider,
        control: &mut impl ActionControl,
    ) -> Result<CheckStatus, Error> {
        if self.phase != Phase::PendingVerification {
            return Err(Error::StaleTicket);
        }
        let now = control.now();
        let remaining = self.check(&now, control.cancelled())?;
        let after = match provider.observe_after(&self.current.action, &now, remaining) {
            Ok(snapshot) => snapshot,
            Err(issue) => {
                self.stop(issue);
                return Ok(CheckStatus::Unknown);
            }
        };
        let now = control.now();
        self.check(&now, control.cancelled())?;
        if validation::validate_snapshot(&after).is_err()
            || after.id == self.current.snapshot.id
            || !validation::contexts_compatible(&self.current.snapshot.context, &after.context)
        {
            self.stop(self.issue_for(ErrorCode::StaleTarget, "reobserve_exact_target"));
            return Ok(CheckStatus::Unknown);
        }
        let node = after.nodes.iter().find(|node| {
            node.key == self.current.action.backend_ref.key
                && node.surface == self.current.action.backend_ref.surface
        });
        let checked =
            node.and_then(|node| node.properties.iter().find(|p| p.field() == Field::Checked));
        let observed = match checked {
            Some(Property::Requested {
                sensitivity: Sensitivity::Public,
                evidence,
                state:
                    Availability::Known {
                        value: Value::Flag(value),
                    },
                ..
            }) => after
                .observations
                .iter()
                .find(|o| {
                    o.id == evidence.observation_id
                        && o.source_namespace == self.current.action.backend_ref.key.namespace
                        && evidence.source_namespace == o.source_namespace
                        && o.freshness == Freshness::Current
                        && o.coverage.scope_id == after.context.scope_id
                        && o.coverage.fields.contains(&Field::Checked)
                })
                .map(|o| (*value, o.id.clone())),
            _ => None,
        };
        self.step.after_snapshot = Some(after.id.clone());
        self.after = Some(after);
        let Some((value, observation)) = observed else {
            self.stop(self.issue_for(ErrorCode::ActionOutcomeUnknown, "reobserve_checked_state"));
            return Ok(CheckStatus::Unknown);
        };
        self.step.verification_observation = Some(observation);
        if self.step.delivery != DeliveryStatus::Confirmed {
            self.stop(self.issue_for(
                ErrorCode::ActionOutcomeUnknown,
                "establish_delivery_confirmation",
            ));
            return Ok(CheckStatus::Unknown);
        }
        let Intent::SetChecked { value: wanted } = self.current.action.intent else {
            unreachable!("prepare restricts intent")
        };
        self.step.outcome = if value == wanted {
            Outcome::Succeeded
        } else {
            Outcome::Failed
        };
        self.phase = Phase::Finished;
        Ok(if value == wanted {
            CheckStatus::Pass
        } else {
            CheckStatus::Fail
        })
    }
    pub fn cancel(&mut self, now: &ClockReading) -> Result<(), Error> {
        if self.phase == Phase::Finished {
            return Err(Error::StaleTicket);
        }
        match self.check(now, true) {
            Err(Error::Detached) => Ok(()),
            Err(error) => Err(error),
            Ok(_) => unreachable!("cancel gate always stops"),
        }
    }
    /// Existing canonical TransitionCase only. It must be terminal; incomplete
    /// preparation/verification cannot be silently labeled successful.
    pub fn finish(self) -> Result<TransitionCase, Error> {
        if self.phase != Phase::Finished {
            return Err(Error::Incomplete);
        }
        let completed = matches!(
            self.step.delivery,
            DeliveryStatus::Accepted | DeliveryStatus::Confirmed
        );
        let stopped = self.step.outcome != Outcome::Succeeded;
        let case = TransitionCase {
            transition: Transition {
                id: self.transition_id,
                context: self.current.snapshot.context.clone(),
                completed_steps: if completed {
                    vec![self.step.id.clone()]
                } else {
                    Vec::new()
                },
                stopped_at: if stopped {
                    Some(self.step.id.clone())
                } else {
                    None
                },
                stop_on_error: true,
                steps: vec![self.step],
            },
            before: self.current.snapshot,
            after: self.after,
        };
        let document = Document {
            schema_version: uiblueprint_schema::SchemaVersion::CURRENT,
            artifact: Artifact::TransitionContext(Box::new(case)),
        };
        document.validate().map_err(|_| Error::InvalidFrame)?;
        let Artifact::TransitionContext(case) = document.artifact else {
            unreachable!("constructed TransitionContext")
        };
        Ok(*case)
    }
}
