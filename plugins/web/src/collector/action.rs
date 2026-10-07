//! Exact native-checkbox Setter port. Caller owns the real parent effect gate.
use super::{acquire::Read, *};
use crate::normalize;
use std::time::Duration;
use uiblueprint_plugin_api::{
    ClockReading,
    actions::{DeliveryPermit, SetCheckedProvider},
};
use uiblueprint_schema::{SchemaVersion, validation};

const SET_CHECKED: &str = include_str!("set-checked.js");
const STATE: &str = r#"function checkboxState(expectedDocument) {
  'use strict';
  const sameDocument = this instanceof Element && this.ownerDocument === expectedDocument && expectedDocument === document;
  const connected = sameDocument && this.isConnected;
  const autocomplete = sameDocument ? Element.prototype.getAttribute.call(this, 'autocomplete') : null;
  const sensitive = autocomplete !== null && (autocomplete.length > 128 || /(?:^|\s)(?:current-password|new-password|one-time-code|cc-number|cc-csc)(?:\s|$)/i.test(autocomplete));
  const unavailable = {connected, sameDocument, nativeCheckbox:false, writable:false, sensitive, enabled:null, checked:null, indeterminate:null};
  if (!connected || !(this instanceof HTMLInputElement)) return unavailable;
  const native = name => Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, name).get.call(this);
  const descriptor = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'checked');
  if (native('type') !== 'checkbox') return unavailable;
  return {connected, sameDocument, nativeCheckbox:true, sensitive, writable:typeof descriptor.set === 'function',
    enabled:!Element.prototype.matches.call(this, ':disabled'), checked:native('checked'), indeterminate:native('indeterminate')};
}"#;

struct Held {
    document: String,
    object: String,
    group: String,
    action: Action,
    budget: Budget,
}
/// One action attempt; use only inside the worker's admitted allocation boundary.
/// The constructor grants no mutation authority. Deliver requires Core's move-only
/// parent permit. Dropping unresolved handles closes only this owned connection.
pub struct CheckboxProvider<'a> {
    collector: &'a mut Collector,
    limits: uiblueprint_schema::model::Limits,
    held: Option<Held>,
    started: bool,
    delivered: bool,
}
impl<'a> CheckboxProvider<'a> {
    pub fn new(collector: &'a mut Collector, limits: uiblueprint_schema::model::Limits) -> Self {
        Self {
            collector,
            limits,
            held: None,
            started: false,
            delivered: false,
        }
    }
    fn request(&self, action: &Action) -> Request {
        Request {
            request_id: action.id.clone(),
            clock_domain: self.collector.binding.clock.domain.clone(),
            context: action.context.clone(),
            limits: self.limits.clone(),
            freshness_policy: FreshnessPolicy::CurrentRequired,
            operation: Operation::Observe {
                channels: vec![Channel::ExternalSemantics],
            },
        }
    }
    fn clock(&self, now: &ClockReading) -> Result<(), Failure> {
        if now.domain != self.collector.binding.clock.domain {
            return Err(Failure::new(ErrorKind::InvalidInput));
        }
        Ok(())
    }
    fn begin(
        &mut self,
        requested: &ActionCase,
        now: &ClockReading,
        remaining: u64,
    ) -> Result<ActionCase, Failure> {
        if self.started {
            return Err(Failure::new(ErrorKind::InvalidInput));
        }
        self.started = true;
        self.clock(now)?;
        let action = &requested.action;
        validation::validate_snapshot(&requested.snapshot)
            .map_err(|_| Failure::new(ErrorKind::InvalidInput))?;
        validation::validate_action(&requested.snapshot, action)
            .map_err(|_| Failure::new(ErrorKind::InvalidInput))?;
        if action.modality != InputModality::Setter
            || !matches!(action.intent, Intent::SetChecked { .. })
            || !action.context.fields.contains(&Field::Enabled)
            || !action.context.fields.contains(&Field::Checked)
        {
            return Err(Failure::new(ErrorKind::InvalidInput));
        }
        if requested
            .snapshot
            .nodes
            .iter()
            .find(|n| n.key == action.backend_ref.key)
            .is_none_or(|n| {
                n.properties.iter().any(|p| {
                    matches!(
                        p,
                        Property::Requested {
                            sensitivity: Sensitivity::Sensitive,
                            ..
                        }
                    )
                })
            })
        {
            return Err(Failure::new(ErrorKind::InvalidInput));
        }
        let node = NodeRef {
            reference: action.backend_ref.clone(),
            sensitivity: Sensitivity::Public,
        };
        let request = self.request(action);
        let scope = Scope {
            scope_id: action.authorized_scope.clone(),
            nodes: vec![node],
        };
        self.collector.validate_request(&request, &scope)?;
        let backend = scope.nodes[0].backend_id()?;
        self.collector.sequence = self
            .collector
            .sequence
            .checked_add(1)
            .ok_or(Failure::new(ErrorKind::Limit))?;
        let mut held = Held {
            document: String::new(),
            object: String::new(),
            group: format!(
                "ui-blueprint-action:{}:{}",
                self.collector.owner, self.collector.sequence
            ),
            action: action.clone(),
            budget: Budget {
                deadline: deadline(remaining.min(self.limits.deadline_ms))?,
                methods: 0,
                reply_bytes: 0,
                max_reply_bytes: usize::try_from(self.limits.max_output_bytes)
                    .map_err(|_| Failure::new(ErrorKind::Limit))?
                    .min(self.collector.limits.max_total_reply_bytes),
            },
        };
        let result = (|| {
            self.collector.verify_document(&mut held.budget)?;
            held.document =
                self.collector
                    .resolve(self.collector.document, &held.group, &mut held.budget)?;
            held.object = self
                .collector
                .resolve(backend, &held.group, &mut held.budget)?;
            let start = self.collector.time();
            let state = read_state(self.collector, &mut held)?;
            eligible(&state, true)?;
            self.collector.verify_document(&mut held.budget)?;
            self.collector.verify_nodes(
                &held.document,
                std::slice::from_ref(&held.object),
                &mut held.budget,
            )?;
            let snapshot = make_snapshot(self.collector, &request, backend, state, start)?;
            let mut fresh = action.clone();
            fresh.backend_ref.snapshot_id = snapshot.id.clone();
            fresh.backend_ref.observation_id = snapshot.observations[0].id.clone();
            fresh.resolution = Resolution {
                evidence: normalize::evidence(
                    &snapshot.observations[0],
                    "native-checkbox-setter-capability",
                ),
                writable: Availability::Known {
                    value: Value::Flag(true),
                },
                value_allowed: Availability::Known {
                    value: Value::Flag(true),
                },
                available_intents: vec![Id("set_checked".into())],
            };
            held.action = fresh.clone();
            Ok(ActionCase {
                snapshot,
                action: fresh,
            })
        })();
        if result.is_ok() {
            self.held = Some(held);
        } else {
            self.collector.detach();
        }
        result
    }
}
fn deadline(remaining: u64) -> Result<Instant, Failure> {
    if remaining == 0 {
        return Err(Failure::new(ErrorKind::Timeout));
    }
    Instant::now()
        .checked_add(Duration::from_millis(remaining))
        .ok_or(Failure::new(ErrorKind::InvalidInput))
}
fn tighten(held: &mut Held, remaining: u64) -> Result<(), Failure> {
    held.budget.deadline = held.budget.deadline.min(deadline(remaining)?);
    Ok(())
}
fn eligible(state: &wire::CheckboxState, before: bool) -> Result<(), Failure> {
    if !state.connected || !state.same_document {
        return Err(Failure::new(ErrorKind::StaleTarget));
    }
    if state.sensitive
        || !state.native_checkbox
        || state.checked.is_none()
        || state.indeterminate != Some(false)
        || (before && (!state.writable || state.enabled != Some(true)))
    {
        return Err(Failure::new(ErrorKind::InvalidInput));
    }
    Ok(())
}
fn read_state(collector: &mut Collector, held: &mut Held) -> Result<wire::CheckboxState, Failure> {
    let result: wire::CheckboxResult = collector.send(
        "Runtime.callFunctionOn",
        &Read {
            object_id: &held.object,
            function_declaration: STATE,
            return_by_value: true,
            silent: true,
            user_gesture: false,
            await_promise: false,
            throw_on_side_effect: false,
            arguments: [serde_json::json!({"objectId":held.document})],
        },
        &mut held.budget,
    )?;
    if result.exception_details.is_some() || result.result.r#type != "object" {
        return Err(Failure::new(ErrorKind::Malformed));
    }
    result
        .result
        .value
        .ok_or(Failure::new(ErrorKind::Malformed))
}
fn make_snapshot(
    collector: &mut Collector,
    request: &Request,
    backend: u32,
    state: wire::CheckboxState,
    start: f64,
) -> Result<Snapshot, Failure> {
    collector.sequence = collector
        .sequence
        .checked_add(1)
        .ok_or(Failure::new(ErrorKind::Limit))?;
    let observation = normalize::observation(
        &request.context,
        &collector.binding.clock.domain,
        "web.dom",
        (collector.owner, collector.sequence),
        [start, collector.time()],
        true,
        false,
    );
    let read = wire::DomRead {
        connected: state.connected,
        same_document: state.same_document,
        tag: Some("INPUT".into()),
        sensitive: false,
        rect: None,
        input_kind: Some("checkbox".into()),
        value: None,
        placeholder: None,
        required: None,
        enabled: state.enabled,
        readonly: None,
        checked: state.checked,
        selected: None,
        expanded: None,
        focused: None,
        invalid: None,
        controls: None,
        declared_anchor: None,
        active_descendant: None,
    };
    let node = normalize::dom(
        backend,
        &read,
        &request.context,
        &observation,
        collector.limits.max_text_bytes,
    );
    let snapshot = normalize::snapshot(
        request,
        (collector.owner, collector.sequence),
        vec![observation],
        vec![node],
        vec![],
        false,
    );
    validation::validate_snapshot(&snapshot).map_err(|_| Failure::new(ErrorKind::Malformed))?;
    let doc = Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Snapshot(Box::new(snapshot)),
    };
    super::observe::bounded_document(&doc, request.limits.max_output_bytes)?;
    let Artifact::Snapshot(snapshot) = doc.artifact else {
        unreachable!("constructed Snapshot")
    };
    Ok(*snapshot)
}
fn issue(action: &Action, error: Failure) -> Issue {
    let code = match error.kind {
        ErrorKind::StaleTarget | ErrorKind::ResyncRequired => ErrorCode::StaleTarget,
        ErrorKind::Timeout
        | ErrorKind::Cdp(cdp::ErrorKind::Transport(transport::ErrorKind::Timeout)) => {
            ErrorCode::Timeout
        }
        ErrorKind::Limit => ErrorCode::IncompleteScope,
        ErrorKind::InvalidInput => ErrorCode::Unsupported,
        _ => ErrorCode::Interrupted,
    };
    Issue {
        code,
        scope_id: action.authorized_scope.clone(),
        failed_step: None,
        recovery_class: Id("reobserve_exact_native_checkbox".into()),
    }
}
impl SetCheckedProvider for CheckboxProvider<'_> {
    fn resolve_exact(
        &mut self,
        requested: &ActionCase,
        now: &ClockReading,
        remaining_ms: u64,
    ) -> Result<ActionCase, Issue> {
        self.begin(requested, now, remaining_ms)
            .map_err(|e| issue(&requested.action, e))
    }
    fn deliver(
        &mut self,
        action: &Action,
        _permit: DeliveryPermit,
        remaining_ms: u64,
    ) -> DeliveryStatus {
        if self.delivered {
            return DeliveryStatus::NotDispatched;
        }
        self.delivered = true;
        let Some(mut held) = self.held.take() else {
            return DeliveryStatus::NotDispatched;
        };
        let outcome = (|| {
            if &held.action != action {
                return Err(Failure::new(ErrorKind::StaleTarget));
            }
            tighten(&mut held, remaining_ms)?;
            self.collector.verify_document(&mut held.budget)?;
            let Intent::SetChecked { value } = action.intent else {
                return Err(Failure::new(ErrorKind::InvalidInput));
            };
            self.collector.pending_invalidation = true;
            let result: wire::SetterResult = self.collector.send(
                "Runtime.callFunctionOn",
                &Read {
                    object_id: &held.object,
                    function_declaration: SET_CHECKED,
                    return_by_value: true,
                    silent: true,
                    user_gesture: false,
                    await_promise: false,
                    throw_on_side_effect: false,
                    arguments: [
                        serde_json::json!({"objectId":held.document}),
                        serde_json::json!({"value":value}),
                    ],
                },
                &mut held.budget,
            )?;
            if result.exception_details.is_some() || result.result.r#type != "object" {
                return Err(Failure::new(ErrorKind::Malformed));
            }
            Ok(
                match result
                    .result
                    .value
                    .ok_or(Failure::new(ErrorKind::Malformed))?
                    .status
                {
                    wire::SetterStatus::Applied => DeliveryStatus::Confirmed,
                    _ => DeliveryStatus::NotDispatched,
                },
            )
        })();
        self.held = Some(held);
        outcome.unwrap_or(DeliveryStatus::Unknown)
    }
    fn observe_after(
        &mut self,
        action: &Action,
        now: &ClockReading,
        remaining_ms: u64,
    ) -> Result<Snapshot, Issue> {
        let Some(mut held) = self.held.take() else {
            return Err(issue(action, Failure::new(ErrorKind::InvalidInput)));
        };
        let result = (|| {
            if !self.delivered || &held.action != action {
                return Err(Failure::new(ErrorKind::StaleTarget));
            }
            self.clock(now)?;
            tighten(&mut held, remaining_ms)?;
            self.collector.verify_document(&mut held.budget)?;
            let start = self.collector.time();
            let state = read_state(self.collector, &mut held)?;
            eligible(&state, false)?;
            self.collector.verify_document(&mut held.budget)?;
            self.collector.verify_nodes(
                &held.document,
                std::slice::from_ref(&held.object),
                &mut held.budget,
            )?;
            let request = self.request(action);
            let backend = NodeRef {
                reference: action.backend_ref.clone(),
                sensitivity: Sensitivity::Public,
            }
            .backend_id()?;
            let snapshot = make_snapshot(self.collector, &request, backend, state, start)?;
            let _: wire::Empty = self.collector.send(
                "Runtime.releaseObjectGroup",
                &serde_json::json!({"objectGroup":held.group}),
                &mut held.budget,
            )?;
            Ok(snapshot)
        })();
        if result.is_err() {
            self.collector.detach();
        }
        result.map_err(|e| issue(action, e))
    }
}
impl Drop for CheckboxProvider<'_> {
    fn drop(&mut self) {
        if self.held.is_some() {
            self.collector.detach();
        }
    }
}
