use super::acquire::Read;
use super::*;
use serde::Serialize;
const SELECT_IDS: &str = include_str!("select-ids.js");

pub(super) struct SelectedNode {
    pub backend: u32,
    pub object: String,
    pub sensitivity: Sensitivity,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Options<'a> {
    ids: Vec<&'a str>,
    max_visited: u32,
    max_depth: u32,
    remaining_ms: f64,
}
#[derive(Serialize)]
struct OptionsArgument<'a> {
    value: Options<'a>,
}
impl Collector {
    /// First scoped request: no prior ref/snapshot/observation is fabricated.
    /// Returned DOM refs bind the actual acknowledged canonical Snapshot/Observation.
    pub fn observe_initial(
        &mut self,
        request: &Request,
        scope: &InitialScope,
        dispatch_sequence: u64,
        deadline: Instant,
        publish: impl FnMut(Document) -> Publication,
    ) -> Result<BootstrapReport, Failure> {
        self.validate_plan(request, &scope.scope_id, scope.ids.len())?;
        if scope.ids.is_empty()
            || scope.max_visited_nodes == 0
            || !matches!(&request.operation,Operation::Observe{channels} if channels.contains(&Channel::ExternalSemantics))
        {
            return Err(Failure::new(ErrorKind::InvalidInput));
        }
        for (i, entry) in scope.ids.iter().enumerate() {
            if entry.id.0.is_empty()
                || entry.id.0.chars().count() > 256
                || scope.ids[..i].iter().any(|old| old.id == entry.id)
            {
                return Err(Failure::new(ErrorKind::InvalidInput));
            }
        }
        let result = self.observe_plan(
            request,
            Plan::Initial(scope),
            dispatch_sequence,
            deadline,
            publish,
        )?;
        Ok(BootstrapReport {
            report: result.report,
            references: result.references,
            selection: result.selection.ok_or(Failure::new(ErrorKind::Malformed))?,
        })
    }
    pub(super) fn select_initial(
        &mut self,
        scope: &InitialScope,
        request: &Request,
        document: &str,
        budget: &mut Budget,
    ) -> Result<(Vec<SelectedNode>, SelectionReport), Failure> {
        self.check(budget)?;
        let mut ids = Vec::new();
        ids.try_reserve_exact(scope.ids.len())
            .map_err(|_| Failure::new(ErrorKind::Limit))?;
        ids.extend(scope.ids.iter().map(|entry| entry.id.0.as_str()));
        let options = Options {
            ids,
            max_visited: scope.max_visited_nodes,
            max_depth: request.limits.max_depth,
            remaining_ms: budget
                .deadline
                .saturating_duration_since(Instant::now())
                .as_secs_f64()
                * 1000.0,
        };
        let selected: wire::SelectedCall = self
            .send(
                "Runtime.callFunctionOn",
                &Read {
                    object_id: document,
                    function_declaration: SELECT_IDS,
                    return_by_value: false,
                    silent: true,
                    user_gesture: false,
                    await_promise: false,
                    throw_on_side_effect: true,
                    arguments: [OptionsArgument { value: options }],
                },
                budget,
            )
            .map_err(|e| e.at(MalformedSite::SelectionReply))?;
        if selected.exception_details.is_some() {
            return Err(Failure::new(ErrorKind::Malformed).at(MalformedSite::SelectionException));
        }
        if selected.result.r#type != "object" || selected.result.subtype.is_some() {
            return Err(Failure::new(ErrorKind::Malformed).at(MalformedSite::SelectionShape));
        }
        let object = selected
            .result
            .object_id
            .ok_or(Failure::new(ErrorKind::Malformed).at(MalformedSite::SelectionShape))?;
        self.valid_handle(&object)?;
        // Inspect our flat null-prototype container only, never an application's properties.
        let properties:wire::Properties=self.send("Runtime.getProperties",&serde_json::json!({"objectId":object,"ownProperties":true,"accessorPropertiesOnly":false,"generatePreview":false}),budget).map_err(|e| e.at(MalformedSite::PropertiesReply))?;
        let (handles, progress) =
            selected_properties(properties, scope, self.limits.max_handle_bytes)
                .map_err(|e| e.at(MalformedSite::PropertiesShape))?;
        let mut nodes = Vec::new();
        nodes
            .try_reserve_exact(handles.len())
            .map_err(|_| Failure::new(ErrorKind::Limit))?;
        for (index, handle) in handles.into_iter().enumerate() {
            let description: wire::Described = self
                .send(
                    "DOM.describeNode",
                    &serde_json::json!({"objectId":handle,"depth":0,"pierce":false}),
                    budget,
                )
                .map_err(|e| e.at(MalformedSite::SelectedDescription))?;
            let backend = description.node.backend_node_id;
            if description.node.node_type != 1
                || backend == 0
                || backend > i32::MAX as u32
                || nodes.iter().any(|n: &SelectedNode| n.backend == backend)
            {
                return Err(
                    Failure::new(ErrorKind::Malformed).at(MalformedSite::SelectedDescription)
                );
            }
            nodes.push(SelectedNode {
                backend,
                object: handle,
                sensitivity: scope.ids[index].sensitivity,
            });
        }
        Ok((nodes, progress))
    }
    fn valid_handle(&self, handle: &str) -> Result<(), Failure> {
        if handle.is_empty() || handle.len() > self.limits.max_handle_bytes {
            Err(Failure::new(ErrorKind::Limit))
        } else {
            Ok(())
        }
    }
}
fn selected_properties(
    properties: wire::Properties,
    scope: &InitialScope,
    handle_cap: usize,
) -> Result<(Vec<String>, SelectionReport), Failure> {
    if properties.exception_details.is_some() || properties.result.len() > scope.ids.len() + 2 {
        return Err(Failure::new(ErrorKind::Malformed));
    }
    let mut status = None;
    let mut visited = None;
    let mut handles = Vec::new();
    handles
        .try_reserve_exact(scope.ids.len())
        .map_err(|_| Failure::new(ErrorKind::Limit))?;
    handles.resize_with(scope.ids.len(), || None);
    for property in properties.result {
        if property.get.is_some()
            || property.set.is_some()
            || property.symbol.is_some()
            || property.was_thrown == Some(true)
        {
            return Err(Failure::new(ErrorKind::Malformed));
        }
        let value = property.value.ok_or(Failure::new(ErrorKind::Malformed))?;
        match property.name.as_str() {
            "status" if status.is_none() => {
                if value.r#type != "string" {
                    return Err(Failure::new(ErrorKind::Malformed));
                }
                let Some(wire::Scalar::Text(text)) = value.value else {
                    return Err(Failure::new(ErrorKind::Malformed));
                };
                status = Some(text);
            }
            "visited" if visited.is_none() => {
                if value.r#type != "number" {
                    return Err(Failure::new(ErrorKind::Malformed));
                }
                let Some(wire::Scalar::Number(n)) = value.value else {
                    return Err(Failure::new(ErrorKind::Malformed));
                };
                if !n.is_finite()
                    || n < 0.0
                    || n.fract() != 0.0
                    || n > f64::from(scope.max_visited_nodes)
                {
                    return Err(Failure::new(ErrorKind::Malformed));
                }
                visited = Some(n as u32);
            }
            name => {
                let index = name
                    .strip_prefix("node_")
                    .and_then(|n| n.parse::<usize>().ok())
                    .filter(|&n| n < handles.len() && format!("node_{n}") == name)
                    .ok_or(Failure::new(ErrorKind::Malformed))?;
                if handles[index].is_some()
                    || value.r#type != "object"
                    || value.subtype.as_deref() != Some("node")
                {
                    return Err(Failure::new(ErrorKind::Malformed));
                }
                let handle = value.object_id.ok_or(Failure::new(ErrorKind::Malformed))?;
                if handle.is_empty() || handle.len() > handle_cap {
                    return Err(Failure::new(ErrorKind::Limit));
                }
                handles[index] = Some(handle);
            }
        }
    }
    let status = status.ok_or(Failure::new(ErrorKind::Malformed))?;
    let visited = visited.ok_or(Failure::new(ErrorKind::Malformed))?;
    let failure = match status.as_str() {
        "selected" => None,
        "missing" => Some(SelectionStatus::Missing),
        "ambiguous" => Some(SelectionStatus::Ambiguous),
        "incomplete" => Some(SelectionStatus::Incomplete),
        "unsupported" => Some(SelectionStatus::Unsupported),
        "timeout" => Some(SelectionStatus::TimedOut),
        "stale" => return Err(Failure::new(ErrorKind::StaleTarget)),
        _ => return Err(Failure::new(ErrorKind::Malformed)),
    };
    if let Some(status) = failure {
        if handles.iter().any(Option::is_some) {
            return Err(Failure::new(ErrorKind::Malformed));
        }
        return Err(Failure::new(ErrorKind::Selection {
            status,
            visited_nodes: visited,
        }));
    }
    if visited == 0 {
        return Err(Failure::new(ErrorKind::Malformed));
    }
    let handles = handles
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or(Failure::new(ErrorKind::Malformed))?;
    Ok((
        handles,
        SelectionReport {
            visited_nodes: visited,
            selected_nodes: scope.ids.len(),
        },
    ))
}
