//! Exact held-node actions; Type uses the explicitly revalidated focused widget.
//! Caller owns the real parent effect gate and shared focus/input lane.
use super::{acquire::Read, *};
use crate::normalize;
use std::time::Duration;
use uiblueprint_plugin_api::{
    ClockReading,
    actions::{ActionProvider, DeliveryPermit},
};
use uiblueprint_schema::{SchemaVersion, validation};
#[path = "activate.rs"]
mod activate;

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
    action: Option<Action>,
    expected: Option<Expectation>,
    result: Option<activate::HeldResult>,
    budget: Budget,
}
/// One action attempt; use only inside the worker's admitted allocation boundary.
/// The constructor grants no mutation authority. Deliver requires Core's move-only
/// parent permit. Dropping unresolved handles closes only this owned connection.
pub struct WebActionProvider<'a> {
    collector: &'a mut Collector,
    limits: uiblueprint_schema::model::Limits,
    held: Option<Held>,
    started: bool,
    delivered: bool,
}
/// Compatibility name for the existing native-checkbox consumer.
pub type CheckboxProvider<'a> = WebActionProvider<'a>;

impl<'a> WebActionProvider<'a> {
    pub fn new(collector: &'a mut Collector, limits: uiblueprint_schema::model::Limits) -> Self {
        Self {
            collector,
            limits,
            held: None,
            started: false,
            delivered: false,
        }
    }
    fn request(&self, context: &Context, action_id: &Id) -> Request {
        Request {
            request_id: action_id.clone(),
            clock_domain: self.collector.binding.clock.domain.clone(),
            context: context.clone(),
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
    fn supported(&self, action: &Action) -> Result<(), Failure> {
        let fields: &[Field] = match (&action.intent, action.modality) {
            (Intent::SetChecked { .. }, InputModality::Setter) => &[Field::Enabled, Field::Checked],
            (Intent::Activate {}, InputModality::Semantic) => {
                &[Field::Enabled, Field::Value, Field::InputKind]
            }
            (Intent::Focus {}, InputModality::Semantic) => &[
                Field::Enabled,
                Field::Focused,
                Field::Value,
                Field::InputKind,
            ],
            (Intent::Type { text }, InputModality::Keyboard) => {
                if text.len() > self.collector.limits.max_text_bytes {
                    return Err(Failure::new(ErrorKind::Limit));
                }
                &[
                    Field::Enabled,
                    Field::Focused,
                    Field::Value,
                    Field::InputKind,
                    Field::Readonly,
                ]
            }
            _ => return Err(Failure::new(ErrorKind::InvalidInput)),
        };
        if fields.iter().any(|f| !action.context.fields.contains(f)) {
            return Err(Failure::new(ErrorKind::InvalidInput));
        }
        Ok(())
    }
    /// Read-only initial preparation from actual recorded identity plus intent.
    /// Consumes this preparation owner; use a new provider for kernel dispatch,
    /// which must resolve again. No permit or setter is reachable from this method.
    pub fn prepare_exact(
        mut self,
        snapshot: &Snapshot,
        requested: &Request,
        expected: Option<&Expectation>,
        now: &ClockReading,
        remaining_ms: u64,
    ) -> Result<ActionCase, Issue> {
        let scope = snapshot.context.scope_id.clone();
        let result = (|| {
            validation::validate_request(requested)
                .map_err(|_| Failure::new(ErrorKind::InvalidInput))?;
            let Operation::Prepare { action } = &requested.operation else {
                return Err(Failure::new(ErrorKind::InvalidInput));
            };
            self.supported(action)?;
            if !action.required_enabled
                || requested.limits != self.limits
                || requested.clock_domain != now.domain
                || action.authorized_scope != snapshot.context.scope_id
                || !validation::contexts_compatible(&snapshot.context, &requested.context)
                || !validation::contexts_compatible(&snapshot.context, &action.context)
            {
                return Err(Failure::new(ErrorKind::InvalidInput));
            }
            validate_seed(snapshot, &action.backend_ref)?;
            let result = if matches!(action.intent, Intent::Activate {}) {
                Some(activate::result_seed(
                    snapshot,
                    action,
                    expected.ok_or(Failure::new(ErrorKind::InvalidInput))?,
                    self.collector.limits.max_text_bytes,
                )?)
            } else {
                None
            };
            let (fresh, mut held) = self.probe(action, result, now, remaining_ms)?;
            let prepared = (|| {
                let mut prepared_action = action.clone();
                prepared_action.backend_ref.snapshot_id = fresh.id.clone();
                prepared_action.backend_ref.observation_id = fresh.observations[0].id.clone();
                prepared_action.unique_match = true;
                prepared_action.resolution = resolution(&fresh, &action.intent);
                let document = Document {
                    schema_version: SchemaVersion::CURRENT,
                    artifact: Artifact::Action(Box::new(ActionCase {
                        snapshot: fresh,
                        action: prepared_action,
                    })),
                };
                document
                    .validate()
                    .map_err(|_| Failure::new(ErrorKind::Malformed))?;
                super::observe::bounded_document(&document, self.limits.max_output_bytes)?;
                let _: wire::Empty = self.collector.send(
                    "Runtime.releaseObjectGroup",
                    &serde_json::json!({"objectGroup":held.group}),
                    &mut held.budget,
                )?;
                let Artifact::Action(case) = document.artifact else {
                    unreachable!("constructed ActionCase")
                };
                Ok(*case)
            })();
            if prepared.is_err() {
                self.collector.detach();
            }
            prepared
        })();
        result.map_err(|error| match &requested.operation {
            Operation::Prepare { action } => issue(action, error),
            _ => issue_for(scope, error),
        })
    }
    fn begin(
        &mut self,
        requested: &ActionCase,
        expected: &Expectation,
        now: &ClockReading,
        remaining: u64,
    ) -> Result<ActionCase, Failure> {
        validate_seed(&requested.snapshot, &requested.action.backend_ref)?;
        validation::validate_action(&requested.snapshot, &requested.action)
            .map_err(|_| Failure::new(ErrorKind::InvalidInput))?;
        let action = &requested.action;
        self.supported(action)?;
        let result = if matches!(action.intent, Intent::Activate {}) {
            Some(activate::result_seed(
                &requested.snapshot,
                action,
                expected,
                self.collector.limits.max_text_bytes,
            )?)
        } else {
            expected_for(action, expected)?;
            None
        };
        let (snapshot, mut held) = self.probe(action, result, now, remaining)?;
        let mut fresh = action.clone();
        fresh.backend_ref.snapshot_id = snapshot.id.clone();
        fresh.backend_ref.observation_id = snapshot.observations[0].id.clone();
        fresh.resolution = resolution(&snapshot, &action.intent);
        held.action = Some(fresh.clone());
        held.expected = Some(expected.clone());
        self.held = Some(held);
        Ok(ActionCase {
            snapshot,
            action: fresh,
        })
    }
    fn probe(
        &mut self,
        action: &Action,
        result_node: Option<NodeRef>,
        now: &ClockReading,
        remaining: u64,
    ) -> Result<(Snapshot, Held), Failure> {
        let context = &action.context;
        let reference = &action.backend_ref;
        if self.started {
            return Err(Failure::new(ErrorKind::InvalidInput));
        }
        self.started = true;
        self.clock(now)?;
        let node = NodeRef {
            reference: reference.clone(),
            sensitivity: Sensitivity::Public,
        };
        let request = self.request(context, &action.id);
        let mut scope = Scope {
            scope_id: context.scope_id.clone(),
            nodes: vec![node],
        };
        if let Some(result) = result_node {
            scope.nodes.push(result);
        }
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
            action: None,
            expected: None,
            result: None,
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
            let read = action_read(self.collector, &mut held, action, true)?;
            let mut records = vec![(backend, read)];
            if let Some(result) = scope.nodes.get(1) {
                let backend = result.backend_id()?;
                let object = self
                    .collector
                    .resolve(backend, &held.group, &mut held.budget)?;
                held.result = Some(activate::HeldResult { backend, object });
                records.push((
                    backend,
                    activate::read_result(self.collector, &mut held, action, true)?,
                ));
                activate::verify_result(self.collector, &mut held)?;
            }
            self.collector.verify_document(&mut held.budget)?;
            self.collector.verify_nodes(
                &held.document,
                std::slice::from_ref(&held.object),
                &mut held.budget,
            )?;
            let snapshot = make_snapshot(self.collector, &request, records, start)?;
            Ok(snapshot)
        })();
        match result {
            Ok(snapshot) => Ok((snapshot, held)),
            Err(error) => {
                self.collector.detach();
                Err(error)
            }
        }
    }
}
fn expected_for(action: &Action, expected: &Expectation) -> Result<(), Failure> {
    if matches!(action.intent, Intent::SetChecked { .. }) {
        return Ok(()); // Preserve the original kernel-owned Checked predicate.
    }
    validation::validate_expectation(expected)
        .map_err(|_| Failure::new(ErrorKind::InvalidInput))?;
    let valid_rule = matches!(
        (&action.intent, &expected.rule),
        (
            Intent::Focus {},
            Rule::PropertyEquals {
                field: Field::Focused,
                expected: Value::Flag(true)
            }
        ) | (
            Intent::Type { .. },
            Rule::PropertyEquals {
                field: Field::Value,
                expected: Value::Text(_)
            }
        )
    );
    if !valid_rule
        || expected.targets.as_slice() != [action.backend_ref.key.clone()]
        || expected.scope_id != action.authorized_scope
    {
        return Err(Failure::new(ErrorKind::InvalidInput));
    }
    Ok(())
}
fn validate_seed(snapshot: &Snapshot, reference: &BackendRef) -> Result<(), Failure> {
    validation::validate_snapshot(snapshot).map_err(|_| Failure::new(ErrorKind::InvalidInput))?;
    let node = snapshot
        .nodes
        .iter()
        .find(|n| n.key == reference.key)
        .ok_or(Failure::new(ErrorKind::StaleTarget))?;
    if reference.snapshot_id != snapshot.id
        || reference.session_id != snapshot.context.session_id
        || reference.target != snapshot.context.target
        || reference.surface != node.surface
        || !snapshot.observations.iter().any(|o| {
            o.id == reference.observation_id && o.source_namespace == reference.key.namespace
        })
    {
        return Err(Failure::new(ErrorKind::StaleTarget));
    }
    if node.properties.iter().any(|p| {
        matches!(
            p,
            Property::Requested {
                sensitivity: Sensitivity::Sensitive,
                ..
            }
        )
    }) {
        return Err(Failure::new(ErrorKind::InvalidInput));
    }
    Ok(())
}
fn resolution(snapshot: &Snapshot, intent: &Intent) -> Resolution {
    let (method, name) = match intent {
        Intent::Focus {} => ("cdp.DOM.focus-native-text-control", "focus"),
        Intent::Type { .. } => ("cdp.Input.insertText-native-ImeCommitText", "type"),
        Intent::Activate {} => ("dom.HTMLElement.click-native-button-untrusted", "activate"),
        _ => ("native-checkbox-setter-capability", "set_checked"),
    };
    Resolution {
        evidence: normalize::evidence(&snapshot.observations[0], method),
        writable: Availability::Known {
            value: Value::Flag(true),
        },
        value_allowed: Availability::Known {
            value: Value::Flag(true),
        },
        available_intents: vec![Id(name.into())],
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
fn action_read(
    collector: &mut Collector,
    held: &mut Held,
    action: &Action,
    before: bool,
) -> Result<wire::DomRead, Failure> {
    if matches!(action.intent, Intent::SetChecked { .. }) {
        let state = read_state(collector, held)?;
        eligible(&state, before)?;
        return Ok(checkbox_read(state));
    }
    let read = read_dom(
        collector,
        &held.document,
        &held.object,
        &action.context.fields,
        &mut held.budget,
    )?;
    if matches!(action.intent, Intent::Activate {}) {
        activate::eligible_actor(&read)?;
        return Ok(read);
    }
    // Restrict the existing Focus/Type port to its qualified editable controls.
    if read.sensitive
        || !matches!(
            (read.tag.as_deref(), read.input_kind.as_deref()),
            (Some("INPUT"), Some("text" | "search" | "url" | "tel"))
                | (Some("TEXTAREA"), Some("textarea"))
        )
        || (before
            && (read.enabled != Some(true)
                || read
                    .value
                    .as_ref()
                    .is_none_or(|v| v.len() > collector.limits.max_text_bytes)))
        || (before && matches!(action.intent, Intent::Type { .. }) && read.readonly != Some(false))
    {
        return Err(Failure::new(ErrorKind::InvalidInput));
    }
    if matches!(action.intent, Intent::Type { .. })
        && (read.focused != Some(true) || read.document_focused != Some(true))
    {
        return Err(Failure::new(ErrorKind::StaleTarget));
    }
    Ok(read)
}
fn read_dom(
    collector: &mut Collector,
    document: &str,
    object: &str,
    fields: &[Field],
    budget: &mut Budget,
) -> Result<wire::DomRead, Failure> {
    let result: wire::ReadResult = collector.send(
        "Runtime.callFunctionOn",
        &Read {
            object_id: object,
            function_declaration: acquire::READ_NODE,
            return_by_value: true,
            silent: true,
            user_gesture: false,
            await_promise: false,
            throw_on_side_effect: false,
            arguments: [
                serde_json::json!({"value":{"fields":fields,"maxChars":collector.limits.max_text_bytes/6,"sensitive":false}}),
                serde_json::json!({"objectId":document}),
                serde_json::json!({"objectId":object}),
            ],
        },
        budget,
    )?;
    if result.exception_details.is_some() || result.result.r#type != "object" {
        return Err(Failure::new(ErrorKind::Malformed));
    }
    let read = result
        .result
        .value
        .ok_or(Failure::new(ErrorKind::Malformed))?;
    acquire::validate_dom(&read, collector.limits.max_text_bytes, 1)?;
    if !read.connected || !read.same_document {
        return Err(Failure::new(ErrorKind::StaleTarget));
    }
    Ok(read)
}
fn checkbox_read(state: wire::CheckboxState) -> wire::DomRead {
    wire::DomRead {
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
        selection: None,
        document_focused: None,
    }
}
fn make_snapshot(
    collector: &mut Collector,
    request: &Request,
    records: Vec<(u32, wire::DomRead)>,
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
    let nodes = records
        .iter()
        .map(|(backend, read)| {
            normalize::dom(
                *backend,
                read,
                &request.context,
                &observation,
                collector.limits.max_text_bytes,
            )
        })
        .collect();
    let keyboard = normalize::keyboard_focus(&records, &observation, &request.context);
    let selection = normalize::text_selection(
        &records,
        &observation,
        &request.context,
        collector.limits.max_text_bytes,
    );
    let mut snapshot = normalize::snapshot(
        request,
        (collector.owner, collector.sequence),
        vec![observation],
        nodes,
        vec![],
        false,
    );
    snapshot.focus.keyboard = keyboard;
    if let Some((keyboard, selection)) = selection {
        snapshot.focus.keyboard = keyboard;
        snapshot.focus.text_selection = Some(selection);
    }
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
    let mut issue = issue_for(action.authorized_scope.clone(), error);
    if !matches!(action.intent, Intent::SetChecked { .. }) {
        issue.recovery_class = Id("reobserve_exact_web_form".into());
    }
    issue
}
fn issue_for(scope_id: Id, error: Failure) -> Issue {
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
        scope_id,
        failed_step: None,
        recovery_class: Id("reobserve_exact_native_checkbox".into()),
    }
}
impl ActionProvider for WebActionProvider<'_> {
    fn resolve_exact(
        &mut self,
        requested: &ActionCase,
        expected: &Expectation,
        now: &ClockReading,
        remaining_ms: u64,
    ) -> Result<ActionCase, Issue> {
        self.begin(requested, expected, now, remaining_ms)
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
            if held.action.as_ref() != Some(action) {
                return Err(Failure::new(ErrorKind::StaleTarget));
            }
            tighten(&mut held, remaining_ms)?;
            self.collector.verify_document(&mut held.budget)?;
            if matches!(action.intent, Intent::Activate {}) {
                action_read(self.collector, &mut held, action, true)?;
                activate::read_result(self.collector, &mut held, action, true)?;
                activate::verify_result(self.collector, &mut held)?;
                self.collector.verify_nodes(
                    &held.document,
                    std::slice::from_ref(&held.object),
                    &mut held.budget,
                )?;
                self.collector.pending_invalidation = true;
                return activate::deliver(self.collector, &mut held);
            }
            if !matches!(action.intent, Intent::SetChecked { .. }) {
                // Native Input.insertText targets the CURRENT focused widget and
                // calls widget Focus internally. The host must hold its input lane.
                // Recheck here; never focus/repair a Type target or synthesize events.
                self.supported(action)?;
                action_read(self.collector, &mut held, action, true)?;
                self.collector.verify_nodes(
                    &held.document,
                    std::slice::from_ref(&held.object),
                    &mut held.budget,
                )?;
                self.collector.pending_invalidation = true;
                let (method, params) = match &action.intent {
                    Intent::Focus {} => ("DOM.focus", serde_json::json!({"objectId":held.object})),
                    Intent::Type { text } => ("Input.insertText", serde_json::json!({"text":text})),
                    _ => return Err(Failure::new(ErrorKind::InvalidInput)),
                };
                let _: wire::Empty = self.collector.send(method, &params, &mut held.budget)?;
                return Ok(DeliveryStatus::Confirmed);
            }
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
        expected: &Expectation,
        now: &ClockReading,
        remaining_ms: u64,
    ) -> Result<Snapshot, Issue> {
        let Some(mut held) = self.held.take() else {
            return Err(issue(action, Failure::new(ErrorKind::InvalidInput)));
        };
        let result = (|| {
            if !self.delivered
                || held.action.as_ref() != Some(action)
                || held.expected.as_ref() != Some(expected)
            {
                return Err(Failure::new(ErrorKind::StaleTarget));
            }
            self.clock(now)?;
            tighten(&mut held, remaining_ms)?;
            self.collector.verify_document(&mut held.budget)?;
            let start = self.collector.time();
            if matches!(action.intent, Intent::Activate {}) {
                let read = activate::read_result(self.collector, &mut held, action, false)?;
                self.collector.verify_document(&mut held.budget)?;
                activate::verify_result(self.collector, &mut held)?;
                let backend = held
                    .result
                    .as_ref()
                    .ok_or(Failure::new(ErrorKind::InvalidInput))?
                    .backend;
                let request = self.request(&action.context, &action.id);
                let snapshot =
                    make_snapshot(self.collector, &request, vec![(backend, read)], start)?;
                let _: wire::Empty = self.collector.send(
                    "Runtime.releaseObjectGroup",
                    &serde_json::json!({"objectGroup":held.group}),
                    &mut held.budget,
                )?;
                return Ok(snapshot);
            }
            let read = action_read(self.collector, &mut held, action, false)?;
            self.collector.verify_document(&mut held.budget)?;
            self.collector.verify_nodes(
                &held.document,
                std::slice::from_ref(&held.object),
                &mut held.budget,
            )?;
            let request = self.request(&action.context, &action.id);
            let backend = NodeRef {
                reference: action.backend_ref.clone(),
                sensitivity: Sensitivity::Public,
            }
            .backend_id()?;
            let snapshot = make_snapshot(self.collector, &request, vec![(backend, read)], start)?;
            if matches!(action.intent, Intent::Type { .. })
                && !matches!(&snapshot.focus.keyboard, FocusRef::Known { target, .. } if target == &action.backend_ref.key)
            {
                return Err(Failure::new(ErrorKind::StaleTarget));
            }
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
impl Drop for WebActionProvider<'_> {
    fn drop(&mut self) {
        if self.held.is_some() {
            self.collector.detach();
        }
    }
}
