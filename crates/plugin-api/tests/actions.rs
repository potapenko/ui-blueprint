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
        expected: &Expectation,
        _: &ClockReading,
        remaining: u64,
    ) -> Result<ActionCase, Issue> {
        assert!(remaining > 0);
        assert_eq!(expected.scope_id, requested.action.authorized_scope);
        self.resolves += 1;
        self.resolved.clone()
    }
    fn deliver(&mut self, _: &Action, permit: DeliveryPermit, remaining: u64) -> DeliveryStatus {
        assert!(remaining > 0);
        self.deliveries += 1;
        self.nonces.push(permit.nonce());
        self.delivered
    }
    fn observe_after(
        &mut self,
        _: &Action,
        _: &Expectation,
        _: &ClockReading,
        remaining: u64,
    ) -> Result<Snapshot, Issue> {
        assert!(remaining > 0);
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
    fn authorize(&mut self, _: &Action, _: &ClockReading) -> Result<DeliveryPermit, GateFailure> {
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

fn property_evidence(snapshot: &Snapshot, index: usize, field: Field) -> Evidence {
    let Property::Requested { evidence, .. } = snapshot.nodes[index]
        .properties
        .iter()
        .find(|p| p.field() == field)
        .unwrap()
    else {
        panic!("requested property")
    };
    evidence.clone()
}
fn property_state(
    snapshot: &mut Snapshot,
    index: usize,
    field: Field,
    state: Availability,
    sensitivity: Sensitivity,
) {
    if !snapshot.context.fields.contains(&field) {
        snapshot.context.fields.push(field);
    }
    if !snapshot.coverage.fields.contains(&field) {
        snapshot.coverage.fields.push(field);
    }
    let observation = &mut snapshot.observations[0];
    if !observation.coverage.fields.contains(&field) {
        observation.coverage.fields.push(field);
    }
    let evidence = Evidence {
        observation_id: observation.id.clone(),
        source_namespace: observation.source_namespace.clone(),
        provenance: Provenance::Reported,
        method: Id("fake_current_property".into()),
        uncertainty: None,
    };
    snapshot.nodes[index]
        .properties
        .retain(|p| p.field() != field);
    snapshot.nodes[index].properties.push(Property::Requested {
        field,
        sensitivity,
        evidence,
        state,
    });
}
fn forms_case(intent: Intent) -> (ActionCase, Snapshot, Expectation) {
    let mut case = before();
    let mut post = after();
    let (name, modality, field, wanted) = match &intent {
        Intent::Focus {} => (
            "focus",
            InputModality::Semantic,
            Field::Focused,
            Value::Flag(true),
        ),
        Intent::Type { .. } => (
            "type",
            InputModality::Keyboard,
            Field::Value,
            Value::Text("prefixsuffix".into()),
        ),
        Intent::Fill { .. } => (
            "fill",
            InputModality::Setter,
            Field::Value,
            Value::Text("prefixsuffix".into()),
        ),
        Intent::Activate {} => (
            "activate",
            InputModality::Semantic,
            Field::Value,
            Value::Number(1.0),
        ),
        _ => unreachable!(),
    };
    case.action.intent = intent;
    case.action.modality = modality;
    case.action.resolution.available_intents = vec![Id(name.into())];
    let index = if name == "activate" {
        let mut node = case.snapshot.nodes[0].clone();
        node.key.key = Id("count".into());
        case.snapshot.nodes.push(node);
        let mut node = post.nodes[0].clone();
        node.key.key = Id("count".into());
        post.nodes.push(node);
        1
    } else {
        0
    };
    let initial = if name == "focus" {
        Value::Flag(false)
    } else if name == "type" || name == "fill" {
        Value::Text("prefix".into())
    } else {
        Value::Number(0.0)
    };
    property_state(
        &mut case.snapshot,
        index,
        field,
        Availability::Known { value: initial },
        Sensitivity::Public,
    );
    property_state(
        &mut post,
        index,
        field,
        Availability::Known {
            value: wanted.clone(),
        },
        Sensitivity::Public,
    );
    if index == 1 {
        for snapshot in [&mut case.snapshot, &mut post] {
            property_state(
                snapshot,
                0,
                field,
                Availability::Known {
                    value: Value::Number(0.0),
                },
                Sensitivity::Public,
            );
        }
    }
    case.action.context = case.snapshot.context.clone();
    let expected = Expectation {
        id: Id("explicit-state".into()),
        scope_id: case.action.authorized_scope.clone(),
        targets: vec![case.snapshot.nodes[index].key.clone()],
        rule: Rule::PropertyEquals {
            field,
            expected: wanted,
        },
        applies_when: ContextConditions {
            platform: None,
            input_mode: None,
            text_scale: None,
        },
        expected_from: Id("caller_scenario".into()),
    };
    if name == "focus" {
        let evidence = property_evidence(&post, index, field);
        post.focus.keyboard = FocusRef::Known {
            target: post.nodes[index].key.clone(),
            evidence,
        };
    }
    case.snapshot.context.fields = post.context.fields.clone();
    case.action.context = case.snapshot.context.clone();
    Document {
        schema_version: uiblueprint_schema::SchemaVersion::CURRENT,
        artifact: Artifact::Action(Box::new(case.clone())),
    }
    .validate()
    .unwrap();
    Document {
        schema_version: uiblueprint_schema::SchemaVersion::CURRENT,
        artifact: Artifact::Snapshot(Box::new(post.clone())),
    }
    .validate()
    .unwrap();
    (case, post, expected)
}
fn forms_prepared(case: ActionCase, expected: Expectation) -> ActionExecution {
    ActionExecution::prepare_action(
        case,
        expected,
        Id("form-transition".into()),
        Id("form-step".into()),
        reading(0),
        100,
    )
    .unwrap()
}
#[test]
fn focus_fill_type_activate_verify_only_explicit_fresh_public_source_state() {
    for intent in [
        Intent::Fill {
            text: "prefixsuffix".into(),
        },
        Intent::Focus {},
        Intent::Type {
            text: "suffix".into(),
        },
        Intent::Activate {},
    ] {
        for mode in 0..6 {
            let (case, mut post, expected) = forms_case(intent.clone());
            let index = usize::from(matches!(intent, Intent::Activate {}));
            let Rule::PropertyEquals {
                field,
                expected: wanted,
            } = &expected.rule
            else {
                unreachable!()
            };
            let mut check = CheckStatus::Pass;
            if mode == 1 {
                let wrong = match wanted {
                    Value::Flag(_) => Value::Flag(false),
                    Value::Text(_) => Value::Text("suffix".into()),
                    _ => Value::Number(0.0),
                };
                property_state(
                    &mut post,
                    index,
                    *field,
                    Availability::Known { value: wrong },
                    Sensitivity::Public,
                );
                if matches!(intent, Intent::Focus {}) {
                    let evidence = property_evidence(&post, index, *field);
                    post.focus.keyboard = FocusRef::None { evidence };
                }
                check = CheckStatus::Fail;
            }
            if mode == 2 {
                property_state(
                    &mut post,
                    index,
                    *field,
                    Availability::Unknown {
                        reason: Id("unavailable".into()),
                    },
                    Sensitivity::Public,
                );
                check = CheckStatus::Unknown;
            }
            if mode == 3 {
                post.nodes[index].key.key = Id("remounted-result".into());
                check = CheckStatus::Unknown;
                if matches!(intent, Intent::Focus {}) {
                    post.focus.keyboard = FocusRef::Unknown {
                        reason: Id("lost-focus".into()),
                    };
                }
            }
            if mode == 4 {
                property_state(
                    &mut post,
                    index,
                    *field,
                    Availability::Redacted {},
                    Sensitivity::Sensitive,
                );
                check = CheckStatus::Unknown;
            }
            if mode == 5 {
                post.observations[0].freshness = Freshness::Unverified;
                check = CheckStatus::Unknown;
            }
            let mut provider = Provider::new();
            provider.resolved = Ok(case.clone());
            provider.observed = Ok(post);
            let mut kernel = forms_prepared(case, expected);
            let mut gate = Gate::new();
            let mut control = Control::new();
            assert_eq!(
                kernel
                    .dispatch(&mut provider, &mut gate, &mut control)
                    .unwrap(),
                DeliveryStatus::Confirmed
            );
            assert_eq!(
                kernel.verify(&mut provider, &mut control).unwrap(),
                check,
                "{intent:?} mode{mode}"
            );
            assert_eq!((provider.deliveries, gate.calls), (1, 1));
            let report = validate(kernel.finish().unwrap());
            assert_eq!(
                report.transition.steps[0].outcome,
                match check {
                    CheckStatus::Pass => Outcome::Succeeded,
                    CheckStatus::Fail => Outcome::Failed,
                    CheckStatus::Unknown => Outcome::ActionOutcomeUnknown,
                }
            );
            assert_eq!(report.transition.completed_steps.len(), 1);
        }
    }
}
#[test]
fn focus_keyboard_axis_and_prepermit_result_binding_fail_closed() {
    for mode in 0..5 {
        let (case, mut post, expected) = forms_case(Intent::Focus {});
        if mode == 0 {
            post.focus.keyboard = FocusRef::Unknown {
                reason: Id("not-observed".into()),
            };
        }
        if mode == 1 {
            post.focus.accessibility = post.focus.keyboard.clone();
            post.focus.keyboard = FocusRef::NotRequested {};
        }
        if mode == 2 {
            post.focus.keyboard = FocusRef::None {
                evidence: property_evidence(&post, 0, Field::Focused),
            };
        }
        let mut provider = Provider::new();
        provider.resolved = Ok(case.clone());
        provider.observed = Ok(post);
        if mode == 3 {
            provider.resolved.as_mut().unwrap().snapshot.nodes[0]
                .properties
                .retain(|p| p.field() != Field::Focused);
        }
        if mode == 4 {
            property_state(
                &mut provider.resolved.as_mut().unwrap().snapshot,
                0,
                Field::Focused,
                Availability::Unsupported {
                    reason: Id("not-supported".into()),
                },
                Sensitivity::Public,
            );
        }
        let mut kernel = forms_prepared(case, expected);
        let mut gate = Gate::new();
        let mut control = Control::new();
        if mode >= 3 {
            assert_eq!(
                kernel
                    .dispatch(&mut provider, &mut gate, &mut control)
                    .unwrap(),
                DeliveryStatus::NotDispatched
            );
            assert_eq!((gate.calls, provider.deliveries), (0, 0));
        } else {
            kernel
                .dispatch(&mut provider, &mut gate, &mut control)
                .unwrap();
            assert_eq!(
                kernel.verify(&mut provider, &mut control).unwrap(),
                CheckStatus::Unknown
            );
        }
        validate(kernel.finish().unwrap());
    }
}
#[test]
fn forms_permission_cancel_and_unknown_delivery_preserve_one_use_lifecycle() {
    for mode in 0..4 {
        let (case, post, expected) = forms_case(Intent::Activate {});
        let mut provider = Provider::new();
        provider.resolved = Ok(case.clone());
        provider.observed = Ok(post);
        let mut kernel = forms_prepared(case, expected);
        let mut gate = Gate::new();
        let mut control = Control::new();
        if mode <= 1 {
            gate.denial = Some((ErrorCode::PermissionRequired, mode == 1));
        }
        if mode == 2 {
            provider.delivered = DeliveryStatus::Unknown;
        }
        if mode == 3 {
            control.cancellation = Some(4);
        }
        let _ = kernel.dispatch(&mut provider, &mut gate, &mut control);
        assert!(
            kernel
                .dispatch(&mut provider, &mut gate, &mut control)
                .is_err()
        );
        let report = validate(kernel.finish().unwrap());
        assert_eq!(
            report.transition.steps[0].outcome,
            if mode == 0 {
                Outcome::Failed
            } else {
                Outcome::ActionOutcomeUnknown
            }
        );
        assert!(provider.deliveries <= 1);
        assert_eq!(gate.calls, 1);
    }
}

#[test]
fn focus_known_other_keyboard_target_is_failure_not_accessibility_success() {
    let (case, mut post, expected) = forms_case(Intent::Focus {});
    property_state(
        &mut post,
        0,
        Field::Focused,
        Availability::Known {
            value: Value::Flag(false),
        },
        Sensitivity::Public,
    );
    let mut other = post.nodes[0].clone();
    other.key.key = Id("other-focus".into());
    post.nodes.push(other);
    property_state(
        &mut post,
        1,
        Field::Focused,
        Availability::Known {
            value: Value::Flag(true),
        },
        Sensitivity::Public,
    );
    post.focus.keyboard = FocusRef::Known {
        target: post.nodes[1].key.clone(),
        evidence: property_evidence(&post, 1, Field::Focused),
    };
    post.focus.accessibility = FocusRef::Known {
        target: post.nodes[0].key.clone(),
        evidence: property_evidence(&post, 0, Field::Focused),
    };
    let mut provider = Provider::new();
    provider.resolved = Ok(case.clone());
    provider.observed = Ok(post);
    let mut kernel = forms_prepared(case, expected);
    let mut gate = Gate::new();
    let mut control = Control::new();
    kernel
        .dispatch(&mut provider, &mut gate, &mut control)
        .unwrap();
    assert_eq!(
        kernel.verify(&mut provider, &mut control).unwrap(),
        CheckStatus::Fail
    );
    assert_eq!(
        validate(kernel.finish().unwrap()).transition.steps[0].outcome,
        Outcome::Failed
    );
}
#[test]
fn activate_result_node_is_independent_current_public_and_within_selected_scope() {
    for mode in 0..4 {
        let (mut case, mut post, expected) = forms_case(Intent::Activate {});
        if mode == 3 {
            let surface = Identity {
                id: Id("other-surface".into()),
                generation: Id("s2".into()),
            };
            case.snapshot.context.surfaces.push(surface.clone());
            case.action.context = case.snapshot.context.clone();
            post.context = case.snapshot.context.clone();
        }
        let mut fresh = case.clone();
        match mode {
            0 => {
                fresh.snapshot.nodes.remove(1);
            }
            1 => property_state(
                &mut fresh.snapshot,
                1,
                Field::Value,
                Availability::Unsupported {
                    reason: Id("no-readable-count".into()),
                },
                Sensitivity::Public,
            ),
            2 => property_state(
                &mut fresh.snapshot,
                1,
                Field::Value,
                Availability::Redacted {},
                Sensitivity::Sensitive,
            ),
            3 => fresh.snapshot.nodes[1].surface = fresh.snapshot.context.surfaces[1].clone(),
            _ => unreachable!(),
        }
        let mut provider = Provider::new();
        provider.resolved = Ok(fresh);
        provider.observed = Ok(post);
        let mut kernel = forms_prepared(case, expected);
        let mut gate = Gate::new();
        let mut control = Control::new();
        assert_eq!(
            kernel
                .dispatch(&mut provider, &mut gate, &mut control)
                .unwrap(),
            DeliveryStatus::NotDispatched
        );
        assert_eq!((gate.calls, provider.deliveries), (0, 0));
        assert_eq!(
            validate(kernel.finish().unwrap()).transition.steps[0].outcome,
            Outcome::Failed
        );
    }
}

// FillSecret reuses canonical core0.1; only a separately held public result may
// verify it. No secret material enters the kernel, even in error/cancel cases.
fn protected_case() -> (ActionCase, Snapshot, Expectation) {
    let (mut case, mut post, expected) = forms_case(Intent::Activate {});
    case.action.intent = Intent::FillSecret {
        secret_reference: Id("opaque-once".into()),
    };
    case.action.modality = InputModality::Setter;
    case.action.resolution.available_intents = vec![Id("fill".into())];
    for snapshot in [&mut case.snapshot, &mut post] {
        property_state(
            snapshot,
            0,
            Field::Value,
            Availability::Redacted {},
            Sensitivity::Sensitive,
        );
    }
    (case, post, expected)
}
#[test]
fn protected_input_requires_distinct_public_result_and_truthful_delivery() {
    for mode in 0..7 {
        let (case, post, expected) = protected_case();
        let mut provider = Provider::new();
        provider.resolved = Ok(case.clone());
        provider.observed = Ok(post);
        if mode == 1 {
            provider.delivered = DeliveryStatus::Accepted;
        }
        if mode == 2 {
            provider.delivered = DeliveryStatus::Unknown;
        }
        if mode == 6 {
            provider.observed = Err(issue(ErrorCode::Interrupted));
        }
        let mut kernel = forms_prepared(case, expected);
        let mut gate = Gate::new();
        let mut control = Control::new();
        if mode == 3 {
            kernel.cancel(&reading(1)).unwrap();
        } else {
            kernel
                .dispatch(&mut provider, &mut gate, &mut control)
                .unwrap();
            if mode == 4 {
                kernel.cancel(&reading(10)).unwrap();
            } else if mode == 5 {
                kernel.cancel(&reading(100)).unwrap();
            } else if kernel.step().outcome == Outcome::PendingVerification {
                kernel.verify(&mut provider, &mut control).unwrap();
            }
        }
        assert!(
            kernel
                .dispatch(&mut provider, &mut gate, &mut control)
                .is_err()
        );
        let outcome = kernel.step().outcome;
        if mode == 0 {
            assert_eq!(outcome, Outcome::Succeeded);
        } else {
            assert_ne!(outcome, Outcome::Succeeded);
        }
        let result = kernel.finish().unwrap();
        Document {
            schema_version: uiblueprint_schema::SchemaVersion::CURRENT,
            artifact: Artifact::TransitionContext(Box::new(result)),
        }
        .validate()
        .unwrap();
    }
    let (case, _, mut expected) = protected_case();
    expected.targets[0] = case.action.backend_ref.key.clone();
    assert!(
        ActionExecution::prepare_action(
            case,
            expected,
            Id("t".into()),
            Id("s".into()),
            reading(0),
            100
        )
        .is_err()
    );
    let (mut case, _, expected) = protected_case();
    property_state(
        &mut case.snapshot,
        1,
        Field::Value,
        Availability::Redacted {},
        Sensitivity::Sensitive,
    );
    assert!(
        ActionExecution::prepare_action(
            case,
            expected,
            Id("t".into()),
            Id("s".into()),
            reading(0),
            100
        )
        .is_err()
    );
}
