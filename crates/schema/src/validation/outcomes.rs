use super::*;

pub fn validate_action(snapshot: &Snapshot, action: &Action) -> Result {
    let r = &action.backend_ref;
    require(
        contexts_compatible(&snapshot.context, &action.context)
            && action.authorized_scope == snapshot.context.scope_id,
        ValidationError::IncompatibleContext,
    )?;
    require(
        r.session_id == snapshot.context.session_id
            && r.snapshot_id == snapshot.id
            && r.target == snapshot.context.target,
        ValidationError::StaleTarget,
    )?;
    require(action.unique_match, ValidationError::AmbiguousTarget)?;
    let n = node(snapshot, &r.key).map_err(|_| ValidationError::StaleTarget)?;
    require(n.surface == r.surface, ValidationError::StaleTarget)?;
    let o = snapshot
        .observations
        .iter()
        .find(|o| o.id == r.observation_id)
        .ok_or(ValidationError::StaleTarget)?;
    require(
        o.freshness == Freshness::Current && o.source_namespace == n.key.namespace,
        ValidationError::StaleTarget,
    )?;
    require(
        action.required_enabled
            && property(n, Field::Enabled).and_then(Property::known) == Some(&Value::Flag(true)),
        ValidationError::UnknownMeasurement,
    )?;
    current_property(snapshot, n, Field::Enabled)?;
    if matches!(
        action.intent,
        Intent::Fill { .. }
            | Intent::FillSecret { .. }
            | Intent::Type { .. }
            | Intent::SetChecked { .. }
            | Intent::SelectOption { .. }
    ) {
        require(
            matches!(
                action.resolution.writable,
                Availability::Known {
                    value: Value::Flag(true)
                }
            ) && matches!(
                action.resolution.value_allowed,
                Availability::Known {
                    value: Value::Flag(true)
                }
            ),
            ValidationError::UnknownMeasurement,
        )?;
    }
    let resolution = evidence(&action.resolution.evidence, &snapshot.observations)?;
    require(
        action.resolution.evidence.provenance == Provenance::Reported
            && resolution.freshness == Freshness::Current
            && resolution.id == r.observation_id
            && resolution.source_namespace == n.key.namespace,
        ValidationError::StaleTarget,
    )?;
    let requested = match action.intent {
        Intent::Focus {} => "focus",
        Intent::Activate {} => "activate",
        Intent::SetChecked { .. } => "set_checked",
        Intent::Fill { .. } | Intent::FillSecret { .. } => "fill",
        Intent::Type { .. } => "type",
        Intent::SelectOption { .. } => "select_option",
        Intent::Scroll {} => "scroll",
        Intent::Press { .. } => "press",
        Intent::Submit {} => "submit",
        Intent::Dismiss {} => "dismiss",
    };
    require(
        action
            .resolution
            .available_intents
            .iter()
            .any(|id| id.0 == requested),
        ValidationError::UnknownMeasurement,
    )?;
    if matches!(action.intent, Intent::Fill { .. } | Intent::Type { .. }) {
        require(
            !matches!(
                property(n, Field::Value),
                Some(Property::Requested {
                    sensitivity: Sensitivity::Sensitive,
                    ..
                })
            ),
            ValidationError::PrivateValue,
        )?;
    }
    if action.modality == InputModality::Pointer {
        current_property(snapshot, n, Field::HitRegion)?;
        let input_space = action
            .input_space
            .as_ref()
            .ok_or(ValidationError::MissingTransform)?;
        require(
            matches!(property(n, Field::HitRegion).and_then(Property::known), Some(Value::Geometry(g)) if g.frame_kind == FrameKind::HitRegion && (&g.coordinate_space == input_space || matches!(&g.transform,TransformState::Known { transform } if &transform.to == input_space))),
            ValidationError::MissingTransform,
        )?;
    }
    Ok(())
}

// A property is bound to this node by ownership; its observation must also be
// current and cover this field/source. Resolution has no node key of its own,
// so the resolution record above must bind to the backend ref's observation.
fn current_property(snapshot: &Snapshot, n: &Node, field: Field) -> Result {
    let Some(Property::Requested { evidence: e, .. }) = property(n, field) else {
        return Err(ValidationError::UnknownMeasurement);
    };
    let observed = evidence(e, &snapshot.observations)?;
    require(
        observed.freshness == Freshness::Current
            && observed.source_namespace == n.key.namespace
            && observed.coverage.scope_id == snapshot.context.scope_id
            && observed.coverage.fields.contains(&field),
        ValidationError::StaleTarget,
    )
}

pub fn validate_transition(t: &Transition) -> Result {
    validate_context(&t.context)?;
    require(
        unique(t.steps.iter().map(|s| &s.id)) && unique(&t.completed_steps),
        ValidationError::DuplicateIdentity,
    )?;
    let mut stopped = false;
    for step in &t.steps {
        require(
            !stopped || step.delivery == DeliveryStatus::NotDispatched,
            ValidationError::InvalidOutcome,
        )?;
        if step.outcome == Outcome::Succeeded {
            require(
                step.delivery == DeliveryStatus::Confirmed
                    && step.after_snapshot.is_some()
                    && step.verification_observation.is_some(),
                ValidationError::InvalidOutcome,
            )?;
        }
        if step.delivery == DeliveryStatus::Unknown {
            require(
                step.outcome == Outcome::ActionOutcomeUnknown,
                ValidationError::InvalidOutcome,
            )?;
        }
        require(
            !t.completed_steps.contains(&step.id) || step.delivery != DeliveryStatus::NotDispatched,
            ValidationError::InvalidOutcome,
        )?;
        if matches!(
            step.outcome,
            Outcome::Failed | Outcome::Interrupted | Outcome::ActionOutcomeUnknown
        ) && t.stop_on_error
        {
            stopped = true;
        }
    }
    require(
        t.completed_steps
            .iter()
            .all(|id| t.steps.iter().any(|s| &s.id == id)),
        ValidationError::DanglingReference,
    )?;
    require(
        t.stopped_at.as_ref().is_none_or(|id| {
            t.steps.iter().any(|s| {
                &s.id == id
                    && matches!(
                        s.outcome,
                        Outcome::Failed | Outcome::Interrupted | Outcome::ActionOutcomeUnknown
                    )
            })
        }),
        ValidationError::InvalidOutcome,
    )?;
    require(
        !stopped || t.stopped_at.is_some(),
        ValidationError::InvalidOutcome,
    )
}
pub fn validate_expectation(e: &Expectation) -> Result {
    require(
        !e.targets.is_empty() && unique(&e.targets),
        ValidationError::InvalidDocument,
    )?;
    require(
        e.applies_when
            .text_scale
            .is_none_or(|s| s.is_finite() && s > 0.0),
        ValidationError::InvalidDocument,
    )?;
    match &e.rule {
        Rule::PropertyEquals { field, expected } => {
            require(field_value(*field, expected), ValidationError::ValueType)
        }
        Rule::Geometry {
            anchors,
            expected,
            tolerance,
            ..
        } => {
            require(
                !anchors.is_empty()
                    && expected.is_finite()
                    && tolerance.is_finite()
                    && *tolerance >= 0.0,
                ValidationError::InvalidGeometry,
            )?;
            for a in anchors {
                validate_anchor(a)?;
                require(
                    e.targets.contains(&a.element),
                    ValidationError::DanglingReference,
                )?;
            }
            Ok(())
        }
    }
}
pub fn validate_finding(s: &Snapshot, e: &Expectation, f: &Finding) -> Result {
    validate_snapshot(s)?;
    validate_expectation(e)?;
    require(
        e.scope_id == s.context.scope_id && f.expectation_id == e.id && f.snapshot_id == s.id,
        ValidationError::IncompatibleContext,
    )?;
    for key in &e.targets {
        node(s, key)?;
    }
    if f.status == CheckStatus::Unknown {
        return require(
            f.measured.is_none() && f.reason.is_some(),
            ValidationError::UnknownMeasurement,
        );
    }
    let observation = f
        .observation_id
        .as_ref()
        .ok_or(ValidationError::InvalidEvidence)?;
    require(
        s.observations.iter().any(|o| &o.id == observation),
        ValidationError::InvalidEvidence,
    )?;
    let measured = f
        .measured
        .as_ref()
        .ok_or(ValidationError::UnknownMeasurement)?;
    if let Rule::PropertyEquals { field, expected } = &e.rule {
        require(e.targets.len() == 1, ValidationError::InvalidDocument)?;
        let p =
            property(node(s, &e.targets[0])?, *field).ok_or(ValidationError::UnknownMeasurement)?;
        require(
            p.known() == Some(measured),
            ValidationError::UnknownMeasurement,
        )?;
        require(
            (measured == expected) == (f.status == CheckStatus::Pass),
            ValidationError::InvalidOutcome,
        )?;
    } else if let Rule::Geometry {
        anchors,
        operation,
        expected,
        comparison,
        quantity_kind,
        units,
        tolerance,
    } = &e.rule
    {
        // Validate the declared measurement/result; G01 owns deriving the amount.
        let Value::Quantity {
            amount,
            kind,
            source_units,
        } = measured
        else {
            return Err(ValidationError::ValueType);
        };
        require(
            amount.is_finite() && kind == quantity_kind && source_units == units,
            ValidationError::ValueType,
        )?;
        let matches = match comparison {
            Comparison::Equal => (amount - expected).abs() <= *tolerance,
            Comparison::AtLeast => *amount >= expected - tolerance,
            Comparison::AtMost => *amount <= expected + tolerance,
            Comparison::GreaterThan => *amount > expected + tolerance,
        };
        require(
            matches == (f.status == CheckStatus::Pass),
            ValidationError::InvalidOutcome,
        )?;
        let mut common_spaces: Option<Vec<&Space>> = None;
        for a in anchors {
            let n = node(s, &a.element)?;
            if *operation == GeometryRelation::Baseline {
                require(
                    matches!(property(n,Field::Baseline).and_then(Property::known),Some(Value::Baseline { coordinate, space }) if coordinate.is_finite() && space == &a.coordinate_space),
                    ValidationError::UnknownMeasurement,
                )?;
                common_measurement_space(&mut common_spaces, vec![&a.coordinate_space], *units)?;
                continue;
            }
            let field = geometry_field(a.frame_kind);
            let Some(Value::Geometry(g)) = property(n, field).and_then(Property::known) else {
                return Err(ValidationError::UnknownMeasurement);
            };
            require(
                g.coordinate_space == a.coordinate_space,
                ValidationError::UnknownMeasurement,
            )?;
            let mut spaces = vec![&g.coordinate_space];
            if let TransformState::Known { transform } = &g.transform
                && transform.surface == n.surface
            {
                spaces.push(&transform.to);
            }
            common_measurement_space(&mut common_spaces, spaces, *units)?;
        }
    }
    Ok(())
}

// Finding has no result-space field yet. Establish at least one actual common
// destination using only the source spaces and validated explicit transforms.
// Equal units alone cannot establish coordinate-space identity.
fn common_measurement_space<'a>(
    common: &mut Option<Vec<&'a Space>>,
    mut candidates: Vec<&'a Space>,
    units: Unit,
) -> Result {
    candidates.retain(|space| space.units == units);
    if let Some(previous) = common {
        previous.retain(|space| candidates.contains(space));
    } else {
        *common = Some(candidates);
    }
    require(
        common.as_ref().is_some_and(|spaces| !spaces.is_empty()),
        ValidationError::MissingTransform,
    )
}
pub(super) fn validate_chain(c: &GoldenChain) -> Result {
    validate_snapshot(&c.before)?;
    validate_snapshot(&c.after)?;
    validate_action(&c.before, &c.action)?;
    validate_transition(&c.delivery)?;
    validate_transition_context(&TransitionCase {
        transition: c.verification.clone(),
        before: c.before.clone(),
        after: Some(c.after.clone()),
    })?;
    require(
        c.verification
            .steps
            .iter()
            .any(|s| s.action_id == c.action.id && s.outcome == Outcome::Succeeded),
        ValidationError::InvalidOutcome,
    )?;
    validate_delta(&c.before, &c.delta)?;
    validate_finding(&c.after, &c.expectation, &c.finding)?;
    require(
        contexts_compatible(&c.before.context, &c.after.context)
            && contexts_compatible(&c.before.context, &c.delivery.context),
        ValidationError::IncompatibleContext,
    )?;
    require(
        c.after.revision == c.delta.revision
            && c.after.source_state.is_some()
            && c.after.source_state == c.delta.source_state,
        ValidationError::ResyncRequired,
    )?;
    require(
        c.delivery.steps.iter().any(|step| {
            step.action_id == c.action.id
                && step.before_snapshot == c.before.id
                && matches!(
                    step.delivery,
                    DeliveryStatus::Accepted | DeliveryStatus::Confirmed
                )
        }),
        ValidationError::InvalidOutcome,
    )?;
    graph::validate_delta_source(&c.before, &c.delta, &c.after)?;
    require(
        c.export.before == c.before.id
            && c.export.after == c.after.id
            && c.export
                .changed
                .iter()
                .all(|k| c.delta.upsert.iter().any(|n| &n.key == k)),
        ValidationError::DanglingReference,
    )?;
    // GOLDEN01 has no requested geometry or pixels; export cannot invent them.
    require(
        !c.export.include_geometry && !c.export.include_pixels,
        ValidationError::UnknownMeasurement,
    )
}

pub(super) fn validate_action_result(x: &ActionResult) -> Result {
    validate_snapshot(&x.snapshot)?;
    let expected = match x.target_state {
        TargetState::Gone => ErrorCode::StaleTarget,
        TargetState::Unresolved => ErrorCode::TargetUnresolved,
        TargetState::Current => match validate_action(&x.snapshot, &x.action) {
            Ok(()) => return Err(ValidationError::InvalidOutcome),
            Err(ValidationError::AmbiguousTarget) => ErrorCode::AmbiguousTarget,
            Err(ValidationError::UnknownMeasurement) => ErrorCode::Unsupported,
            Err(ValidationError::StaleTarget | ValidationError::IncompatibleContext) => {
                ErrorCode::StaleTarget
            }
            Err(_) => return Err(ValidationError::InvalidOutcome),
        },
    };
    require(
        !x.dispatched
            && x.issue.code == expected
            && x.issue.scope_id == x.snapshot.context.scope_id,
        ValidationError::InvalidOutcome,
    )
}
pub(super) fn validate_transition_context(x: &TransitionCase) -> Result {
    validate_transition(&x.transition)?;
    validate_snapshot(&x.before)?;
    if let Some(after) = &x.after {
        validate_snapshot(after)?;
    }
    for step in &x.transition.steps {
        require(
            step.before_snapshot == x.before.id,
            ValidationError::InvalidEvidence,
        )?;
        if step.outcome == Outcome::Succeeded {
            let after = x.after.as_ref().ok_or(ValidationError::InvalidOutcome)?;
            require(
                contexts_compatible(&x.before.context, &after.context)
                    && step.after_snapshot.as_ref() == Some(&after.id)
                    && after
                        .observations
                        .iter()
                        .any(|o| Some(&o.id) == step.verification_observation.as_ref()),
                ValidationError::InvalidOutcome,
            )?;
        }
    }
    Ok(())
}
