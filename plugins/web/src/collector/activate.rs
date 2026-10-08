//! Native button activation with a separately pinned public result control.
use super::*;

pub(super) struct HeldResult {
    pub(super) backend: u32,
    pub(super) object: String,
}

// HTML click() is programmatic, untrusted activation, not a pointer gesture.
// This fixed function is called only after the parent's real one-use permit.
const ACTIVATE: &str = r#"function activateButton(expectedDocument, expectedResult) {
  'use strict';
  const belongs = node => node instanceof Element && node.isConnected && node.ownerDocument === expectedDocument && expectedDocument === document;
  const privateHint = node => {
    const value = Element.prototype.getAttribute.call(node, 'autocomplete');
    return value !== null && (value.length > 128 || /(?:^|\s)(?:current-password|new-password|one-time-code|cc-number|cc-csc)(?:\s|$)/i.test(value));
  };
  if (!belongs(this) || !belongs(expectedResult) || !(this instanceof HTMLButtonElement) ||
      Element.prototype.matches.call(this, ':disabled') || privateHint(this) || privateHint(expectedResult)) return {invoked:false};
  const input = expectedResult instanceof HTMLInputElement;
  const kind = input ? Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'type').get.call(expectedResult) : null;
  if (!(expectedResult instanceof HTMLOutputElement) && !(input && ['text','search','url','tel'].includes(kind))) return {invoked:false};
  HTMLElement.prototype.click.call(this);
  return {invoked:true};
}"#;

pub(super) fn result_seed(
    snapshot: &Snapshot,
    action: &Action,
    expected: &Expectation,
    text_cap: usize,
) -> Result<NodeRef, Failure> {
    validation::validate_expectation(expected)
        .map_err(|_| Failure::new(ErrorKind::InvalidInput))?;
    let Rule::PropertyEquals {
        field: Field::Value,
        expected: Value::Text(text),
    } = &expected.rule
    else {
        return Err(Failure::new(ErrorKind::InvalidInput));
    };
    if text.len() > text_cap {
        return Err(Failure::new(ErrorKind::Limit));
    }
    if expected.scope_id != action.authorized_scope
        || expected.targets.len() != 1
        || expected.targets[0] == action.backend_ref.key
    {
        return Err(Failure::new(ErrorKind::InvalidInput));
    }
    let node = snapshot
        .nodes
        .iter()
        .find(|n| n.key == expected.targets[0])
        .ok_or(Failure::new(ErrorKind::StaleTarget))?;
    if node.key.namespace.0 != "web.dom" || node.surface != action.backend_ref.surface {
        return Err(Failure::new(ErrorKind::StaleTarget));
    }
    let Some(Property::Requested {
        sensitivity: Sensitivity::Public,
        state: Availability::Known { .. } | Availability::Unknown { .. },
        evidence,
        ..
    }) = node.properties.iter().find(|p| p.field() == Field::Value)
    else {
        return Err(Failure::new(ErrorKind::InvalidInput));
    };
    let reference = BackendRef {
        session_id: snapshot.context.session_id.clone(),
        target: snapshot.context.target.clone(),
        surface: node.surface.clone(),
        key: node.key.clone(),
        snapshot_id: snapshot.id.clone(),
        observation_id: evidence.observation_id.clone(),
    };
    validate_seed(snapshot, &reference)?;
    Ok(NodeRef {
        reference,
        sensitivity: Sensitivity::Public,
    })
}

pub(super) fn eligible_actor(read: &wire::DomRead) -> Result<(), Failure> {
    if read.sensitive || read.tag.as_deref() != Some("BUTTON") || read.enabled != Some(true) {
        return Err(Failure::new(ErrorKind::InvalidInput));
    }
    Ok(())
}

pub(super) fn read_result(
    collector: &mut Collector,
    held: &mut Held,
    action: &Action,
    before: bool,
) -> Result<wire::DomRead, Failure> {
    let result = held
        .result
        .as_ref()
        .ok_or(Failure::new(ErrorKind::InvalidInput))?;
    let read = read_dom(
        collector,
        &held.document,
        &result.object,
        &action.context.fields,
        &mut held.budget,
    )?;
    // Read eligibility is independent of enabled/readonly/editability. OUTPUT
    // uses the existing bounded native value getter; unavailable stays unknown.
    if read.sensitive
        || (before && read.value.is_none())
        || !matches!(
            (read.tag.as_deref(), read.input_kind.as_deref()),
            (Some("OUTPUT"), _) | (Some("INPUT"), Some("text" | "search" | "url" | "tel"))
        )
    {
        return Err(Failure::new(ErrorKind::InvalidInput));
    }
    Ok(read)
}

pub(super) fn verify_result(collector: &mut Collector, held: &mut Held) -> Result<(), Failure> {
    let result = held
        .result
        .as_ref()
        .ok_or(Failure::new(ErrorKind::InvalidInput))?;
    collector.verify_nodes(
        &held.document,
        std::slice::from_ref(&result.object),
        &mut held.budget,
    )
}

pub(super) fn deliver(
    collector: &mut Collector,
    held: &mut Held,
) -> Result<DeliveryStatus, Failure> {
    let result = held
        .result
        .as_ref()
        .ok_or(Failure::new(ErrorKind::InvalidInput))?;
    let response: wire::ActivationResult = collector.send(
        "Runtime.callFunctionOn",
        &Read {
            object_id: &held.object,
            function_declaration: ACTIVATE,
            return_by_value: true,
            silent: true,
            user_gesture: false,
            await_promise: false,
            throw_on_side_effect: false,
            arguments: [
                serde_json::json!({"objectId":held.document}),
                serde_json::json!({"objectId":result.object}),
            ],
        },
        &mut held.budget,
    )?;
    if response.exception_details.is_some() || response.result.r#type != "object" {
        return Err(Failure::new(ErrorKind::Malformed));
    }
    let invoked = response
        .result
        .value
        .ok_or(Failure::new(ErrorKind::Malformed))?
        .invoked;
    Ok(if invoked {
        DeliveryStatus::Confirmed
    } else {
        DeliveryStatus::NotDispatched
    })
}
