use std::collections::VecDeque;
use uiblueprint_plugin_api::{ClockReading, Error, actions::*};
use uiblueprint_schema::{model::*, validation::ValidationError};

fn before() -> ActionCase {
    let doc = Document::from_json(
        include_bytes!("../../../fixtures/golden/ENV-ACTION-VALID.json"),
        65536,
    )
    .unwrap();
    let Artifact::Action(case) = doc.artifact else {
        panic!("action")
    };
    *case
}
fn after() -> Snapshot {
    let doc = Document::from_json(
        include_bytes!("../../../fixtures/golden/ENV-TRANSITION-VALID.json"),
        65536,
    )
    .unwrap();
    let Artifact::TransitionContext(case) = doc.artifact else {
        panic!("transition")
    };
    case.after.unwrap()
}
fn reading(ms: u64) -> ClockReading {
    ClockReading {
        domain: Id("action-clock".into()),
        milliseconds: ms,
    }
}
fn issue(code: ErrorCode) -> Issue {
    Issue {
        code,
        scope_id: Id("form-1".into()),
        failed_step: Some(Id("set-step".into())),
        recovery_class: Id("explicit_reobserve".into()),
    }
}
struct Control {
    calls: usize,
    cancellation: Option<usize>,
    times: VecDeque<u64>,
    last: u64,
}
impl Control {
    fn new() -> Self {
        Self {
            calls: 0,
            cancellation: None,
            times: VecDeque::new(),
            last: 0,
        }
    }
}
impl ActionControl for Control {
    fn now(&mut self) -> ClockReading {
        self.calls += 1;
        self.last = self.times.pop_front().unwrap_or(self.last + 1);
        reading(self.last)
    }
    fn cancelled(&self) -> bool {
        self.cancellation.is_some_and(|at| self.calls >= at)
    }
}
struct Provider {
    resolved: Result<ActionCase, Issue>,
    observed: Result<Snapshot, Issue>,
    delivered: DeliveryStatus,
    resolves: usize,
    deliveries: usize,
    observations: usize,
    nonces: Vec<u64>,
}
impl Provider {
    fn new() -> Self {
        Self {
            resolved: Ok(before()),
            observed: Ok(after()),
            delivered: DeliveryStatus::Confirmed,
            resolves: 0,
            deliveries: 0,
            observations: 0,
            nonces: Vec::new(),
        }
    }
}
impl SetCheckedProvider for Provider {
    fn resolve_exact(
        &mut self,
        requested: &ActionCase,
        _: &ClockReading,
        remaining: u64,
    ) -> Result<ActionCase, Issue> {
        assert!(remaining > 0);
        assert!(matches!(requested.action.intent, Intent::SetChecked { .. }));
        self.resolves += 1;
        self.resolved.clone()
    }
    fn deliver(
        &mut self,
        action: &Action,
        permit: DeliveryPermit,
        remaining: u64,
    ) -> DeliveryStatus {
        assert!(remaining > 0);
        assert!(matches!(action.intent, Intent::SetChecked { value: true }));
        self.deliveries += 1;
        self.nonces.push(permit.nonce());
        self.delivered
    }
    fn observe_after(
        &mut self,
        action: &Action,
        _: &ClockReading,
        remaining: u64,
    ) -> Result<Snapshot, Issue> {
        assert!(remaining > 0);
        assert_eq!(action.modality, InputModality::Setter);
        self.observations += 1;
        self.observed.clone()
    }
}
struct Gate {
    calls: usize,
    denial: Option<(ErrorCode, bool)>,
}
impl Gate {
    fn new() -> Self {
        Self {
            calls: 0,
            denial: None,
        }
    }
}
impl EffectGate for Gate {
    fn authorize(
        &mut self,
        action: &Action,
        _: &ClockReading,
    ) -> Result<DeliveryPermit, GateFailure> {
        assert_eq!(action.modality, InputModality::Setter);
        self.calls += 1;
        if let Some((code, possible)) = self.denial {
            Err(GateFailure {
                issue: issue(code),
                effect_possible: possible,
            })
        } else {
            Ok(DeliveryPermit::from_parent_nonce(17).unwrap())
        }
    }
}
fn prepared(case: ActionCase) -> SetCheckedExecution {
    SetCheckedExecution::prepare(
        case,
        Id("transition-test".into()),
        Id("set-step".into()),
        reading(0),
        100,
    )
    .unwrap()
}
fn checked(snapshot: &mut Snapshot, value: Availability) {
    let property = snapshot.nodes[0]
        .properties
        .iter_mut()
        .find(|p| p.field() == Field::Checked)
        .unwrap();
    let Property::Requested { state, .. } = property else {
        panic!("checked")
    };
    *state = value;
}
fn validate(case: TransitionCase) -> TransitionCase {
    Document {
        schema_version: uiblueprint_schema::SchemaVersion::CURRENT,
        artifact: Artifact::TransitionContext(Box::new(case.clone())),
    }
    .validate()
    .unwrap();
    case
}

#[test]
fn single_confirmed_set_checked_verifies_exact_evidence_including_already_equal() {
    for already_equal in [false, true] {
        let mut input = before();
        let mut provider = Provider::new();
        if already_equal {
            checked(
                &mut input.snapshot,
                Availability::Known {
                    value: Value::Flag(true),
                },
            );
            provider.resolved = Ok(input.clone());
        }
        let original = input.clone();
        let mut action = prepared(input);
        let mut gate = Gate::new();
        let mut control = Control::new();
        assert_eq!(
            action.dispatch(&mut provider, &mut gate, &mut control),
            Ok(DeliveryStatus::Confirmed)
        );
        assert_eq!(action.step().outcome, Outcome::PendingVerification);
        assert_eq!(
            action.verify(&mut provider, &mut control),
            Ok(CheckStatus::Pass)
        );
        assert_eq!(
            action.dispatch(&mut provider, &mut gate, &mut control),
            Err(Error::StaleTicket)
        );
        assert_eq!(
            (
                provider.resolves,
                provider.deliveries,
                provider.observations,
                gate.calls
            ),
            (1, 1, 1, 1)
        );
        assert_eq!(provider.nonces, [17]);
        let result = validate(action.finish().unwrap());
        assert_eq!(result.before, original.snapshot);
        let step = &result.transition.steps[0];
        assert_eq!(step.delivery, DeliveryStatus::Confirmed);
        assert_eq!(step.outcome, Outcome::Succeeded);
        assert_eq!(step.after_snapshot, Some(Id("S11".into())));
        assert_eq!(step.verification_observation, Some(Id("O11".into())));
        assert_eq!(result.transition.completed_steps, [Id("set-step".into())]);
        assert!(result.transition.stopped_at.is_none());
    }
    assert!(matches!(
        DeliveryPermit::from_parent_nonce(0),
        Err(Error::InvalidFrame)
    ));
}

#[test]
fn preparation_refuses_readonly_stale_ambiguous_disabled_and_unsupported_intents() {
    let doc = Document::from_json(
        include_bytes!("../../../fixtures/golden/G01-READONLY.json"),
        65536,
    )
    .unwrap();
    let Artifact::ActionResult(readonly) = doc.artifact else {
        panic!("readonly")
    };
    let readonly = ActionCase {
        snapshot: readonly.snapshot,
        action: readonly.action,
    };
    assert!(matches!(
        SetCheckedExecution::prepare(readonly, Id("t".into()), Id("s".into()), reading(0), 100),
        Err(ValidationError::UnknownMeasurement)
    ));
    for (case, expected) in [
        (
            {
                let mut c = before();
                c.action.backend_ref.target.generation = Id("stale".into());
                c
            },
            ValidationError::StaleTarget,
        ),
        (
            {
                let mut c = before();
                c.action.unique_match = false;
                c
            },
            ValidationError::AmbiguousTarget,
        ),
        (
            {
                let mut c = before();
                let Property::Requested { state, .. } = c.snapshot.nodes[0]
                    .properties
                    .iter_mut()
                    .find(|p| p.field() == Field::Enabled)
                    .unwrap()
                else {
                    panic!("enabled")
                };
                *state = Availability::Known {
                    value: Value::Flag(false),
                };
                c
            },
            ValidationError::UnknownMeasurement,
        ),
        (
            {
                let mut c = before();
                c.action.resolution.writable = Availability::Known {
                    value: Value::Flag(false),
                };
                c
            },
            ValidationError::UnknownMeasurement,
        ),
        (
            {
                let mut c = before();
                c.action.resolution.available_intents.clear();
                c
            },
            ValidationError::UnknownMeasurement,
        ),
        (
            {
                let mut c = before();
                c.action.intent = Intent::Focus {};
                c.action
                    .resolution
                    .available_intents
                    .push(Id("focus".into()));
                c
            },
            ValidationError::InvalidDocument,
        ),
    ] {
        assert!(
            matches!(SetCheckedExecution::prepare(case,Id("t".into()),Id("s".into()),reading(0),100),Err(e) if e==expected)
        );
    }
}

#[test]
fn fresh_resolver_refusals_and_permission_denial_precede_effect_gate_or_delivery() {
    for changed in 0..6 {
        let mut provider = Provider::new();
        let fresh = provider.resolved.as_mut().unwrap();
        match changed {
            0 => fresh.action.unique_match = false,
            1 => {
                let mut other = fresh.snapshot.nodes[0].clone();
                other.key.key = Id("same-label-other-object".into());
                fresh.action.backend_ref.key = other.key.clone();
                fresh.snapshot.nodes.push(other);
            }
            2 => {
                fresh.action.resolution.writable = Availability::Known {
                    value: Value::Flag(false),
                }
            }
            3 => {
                provider.resolved = Err(issue(ErrorCode::StaleTarget));
            }
            4 => {
                let Property::Requested { state, .. } = fresh.snapshot.nodes[0]
                    .properties
                    .iter_mut()
                    .find(|p| p.field() == Field::Enabled)
                    .unwrap()
                else {
                    panic!("enabled")
                };
                *state = Availability::Known {
                    value: Value::Flag(false),
                };
            }
            _ => {
                fresh.action.resolution.value_allowed = Availability::Known {
                    value: Value::Flag(false),
                }
            }
        }
        let mut action = prepared(before());
        let mut gate = Gate::new();
        let mut control = Control::new();
        assert_eq!(
            action.dispatch(&mut provider, &mut gate, &mut control),
            Ok(DeliveryStatus::NotDispatched)
        );
        assert_eq!((gate.calls, provider.deliveries), (0, 0));
        assert_eq!(action.step().outcome, Outcome::Failed);
        assert!(action.issue().is_some());
        let result = validate(action.finish().unwrap());
        assert!(result.transition.completed_steps.is_empty());
        assert_eq!(result.transition.stopped_at, Some(Id("set-step".into())));
    }
    let mut action = prepared(before());
    let mut provider = Provider::new();
    let mut gate = Gate::new();
    gate.denial = Some((ErrorCode::PermissionRequired, false));
    assert_eq!(
        action.dispatch(&mut provider, &mut gate, &mut Control::new()),
        Ok(DeliveryStatus::NotDispatched)
    );
    assert_eq!(provider.deliveries, 0);
    assert_eq!(action.issue().unwrap().code, ErrorCode::PermissionRequired);
    assert_eq!(
        validate(action.finish().unwrap()).transition.steps[0].delivery,
        DeliveryStatus::NotDispatched
    );
}

#[test]
fn accepted_unknown_mismatch_and_changed_binding_verification_never_succeed() {
    for mode in 0..6 {
        let mut action = prepared(before());
        let mut provider = Provider::new();
        let mut gate = Gate::new();
        let mut control = Control::new();
        match mode {
            0 => provider.delivered = DeliveryStatus::Accepted,
            1 => provider.observed = Err(issue(ErrorCode::Timeout)),
            2 => checked(
                provider.observed.as_mut().unwrap(),
                Availability::Unknown {
                    reason: Id("not_available".into()),
                },
            ),
            3 => checked(
                provider.observed.as_mut().unwrap(),
                Availability::Known {
                    value: Value::Flag(false),
                },
            ),
            4 => {
                provider
                    .observed
                    .as_mut()
                    .unwrap()
                    .context
                    .target
                    .generation = Id("changed".into())
            }
            _ => {
                for o in &mut provider.observed.as_mut().unwrap().observations {
                    o.freshness = Freshness::Unverified;
                    o.freshness_basis = FreshnessBasis::Unverified;
                    o.last_verified = None;
                }
            }
        }
        action
            .dispatch(&mut provider, &mut gate, &mut control)
            .unwrap();
        assert_eq!(
            action.verify(&mut provider, &mut control),
            Ok(if mode == 3 {
                CheckStatus::Fail
            } else {
                CheckStatus::Unknown
            })
        );
        let result = validate(action.finish().unwrap());
        assert_ne!(result.transition.steps[0].outcome, Outcome::Succeeded);
        assert_eq!(result.transition.completed_steps, [Id("set-step".into())]);
        assert_eq!(result.transition.stopped_at, Some(Id("set-step".into())));
        if mode == 4 {
            assert!(
                result.after.is_none(),
                "wrong target data cannot enter result"
            );
        }
    }
}

#[test]
fn cancel_timeout_and_gate_loss_keep_before_after_possible_effect_distinct() {
    for (cancel_at, deliveries, delivery, outcome) in [
        (1, 0, DeliveryStatus::NotDispatched, Outcome::Interrupted),
        (3, 0, DeliveryStatus::Unknown, Outcome::ActionOutcomeUnknown),
        (
            4,
            1,
            DeliveryStatus::Confirmed,
            Outcome::ActionOutcomeUnknown,
        ),
    ] {
        let mut action = prepared(before());
        let mut provider = Provider::new();
        let mut gate = Gate::new();
        let mut control = Control::new();
        control.cancellation = Some(cancel_at);
        assert_eq!(
            action.dispatch(&mut provider, &mut gate, &mut control),
            Err(Error::Detached)
        );
        assert_eq!(provider.deliveries, deliveries);
        assert_eq!(action.step().delivery, delivery);
        assert_eq!(action.step().outcome, outcome);
        assert_eq!(
            action.dispatch(&mut provider, &mut gate, &mut control),
            Err(Error::StaleTicket)
        );
        let result = validate(action.finish().unwrap());
        assert_eq!(
            result.transition.completed_steps.len(),
            usize::from(deliveries > 0)
        );
    }
    for times in [[100, 101, 102], [1, 2, 100]] {
        let mut control = Control::new();
        control.times = times.into();
        let mut action = prepared(before());
        let mut provider = Provider::new();
        let mut gate = Gate::new();
        assert_eq!(
            action.dispatch(&mut provider, &mut gate, &mut control),
            Err(Error::DeadlineExpired)
        );
        assert_eq!(
            action.step().delivery,
            if times[0] == 100 {
                DeliveryStatus::NotDispatched
            } else {
                DeliveryStatus::Unknown
            }
        );
        validate(action.finish().unwrap());
    }
    let mut action = prepared(before());
    let mut provider = Provider::new();
    let mut gate = Gate::new();
    gate.denial = Some((ErrorCode::ActionOutcomeUnknown, true));
    assert_eq!(
        action.dispatch(&mut provider, &mut gate, &mut Control::new()),
        Ok(DeliveryStatus::Unknown)
    );
    assert_eq!(provider.deliveries, 0);
    assert_eq!(action.step().outcome, Outcome::ActionOutcomeUnknown);
    validate(action.finish().unwrap());
}

#[test]
fn unknown_delivery_is_single_use_and_late_verification_cannot_resume() {
    let mut action = prepared(before());
    let mut provider = Provider::new();
    provider.delivered = DeliveryStatus::Unknown;
    let mut gate = Gate::new();
    let mut control = Control::new();
    assert_eq!(
        action.dispatch(&mut provider, &mut gate, &mut control),
        Ok(DeliveryStatus::Unknown)
    );
    assert_eq!(
        action.verify(&mut provider, &mut control),
        Err(Error::StaleTicket)
    );
    assert_eq!(provider.observations, 0);
    assert_eq!(provider.deliveries, 1);
    assert_eq!(gate.calls, 1);
    let result = validate(action.finish().unwrap());
    assert!(result.transition.completed_steps.is_empty());
    assert_eq!(
        result.transition.steps[0].outcome,
        Outcome::ActionOutcomeUnknown
    );
    let mut action = prepared(before());
    assert_eq!(action.cancel(&reading(1)), Ok(()));
    assert_eq!(
        validate(action.finish().unwrap()).transition.steps[0].delivery,
        DeliveryStatus::NotDispatched
    );
}

#[test]
fn pending_finish_and_invalid_clock_cannot_manufacture_success() {
    assert!(matches!(
        prepared(before()).finish(),
        Err(Error::Incomplete)
    ));
    let mut action = prepared(before());
    let mut provider = Provider::new();
    let mut gate = Gate::new();
    let mut control = Control::new();
    control.times = [2, 1].into();
    assert_eq!(
        action.dispatch(&mut provider, &mut gate, &mut control),
        Err(Error::InvalidClock)
    );
    assert_eq!(gate.calls, 0);
    assert_eq!(provider.deliveries, 0);
    assert_eq!(
        validate(action.finish().unwrap()).transition.steps[0].delivery,
        DeliveryStatus::NotDispatched
    );
    let mut action = prepared(before());
    let mut control = Control::new();
    action
        .dispatch(&mut provider, &mut gate, &mut control)
        .unwrap();
    assert_eq!(
        action.cancel(&ClockReading {
            domain: Id("another-clock".into()),
            milliseconds: 10
        }),
        Err(Error::InvalidClock)
    );
    let result = validate(action.finish().unwrap());
    assert_eq!(
        result.transition.steps[0].delivery,
        DeliveryStatus::Confirmed
    );
    assert_eq!(
        result.transition.steps[0].outcome,
        Outcome::ActionOutcomeUnknown
    );
}
