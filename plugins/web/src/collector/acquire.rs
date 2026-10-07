use super::*;
use crate::normalize;
use serde::Serialize;
use uiblueprint_schema::validation;

const READ_NODE: &str = include_str!("read-node.js");
const VERIFY_NODES: &str = include_str!("verify-nodes.js");
pub(super) struct Records {
    pub(super) dom: Vec<(u32, wire::DomRead)>,
    pub(super) ax: Vec<(u32, wire::AxNode)>,
    dom_start: f64,
    dom_end: f64,
    ax_start: f64,
    ax_end: f64,
    pub(super) ax_status: SourceStatus,
    pub(super) ax_queries: usize,
    pub(super) selection: Option<SelectionReport>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Resolve<'a> {
    backend_node_id: u32,
    execution_context_id: i32,
    object_group: &'a str,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ReadOptions<'a> {
    fields: &'a [Field],
    max_chars: usize,
    sensitive: bool,
}
#[derive(Serialize)]
#[serde(untagged)]
enum Argument<'a> {
    Options {
        value: ReadOptions<'a>,
    },
    Object {
        #[serde(rename = "objectId")]
        object_id: &'a str,
    },
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Read<'a, A> {
    pub(super) object_id: &'a str,
    pub(super) function_declaration: &'static str,
    pub(super) return_by_value: bool,
    pub(super) silent: bool,
    pub(super) user_gesture: bool,
    pub(super) await_promise: bool,
    pub(super) throw_on_side_effect: bool,
    pub(super) arguments: A,
}
impl Collector {
    pub(super) fn collect(
        &mut self,
        plan: Plan<'_>,
        request: &Request,
        budget: &mut Budget,
    ) -> Result<Records, Failure> {
        self.verify_document(budget)?;
        let group = format!("ui-blueprint-observation:{}:{}", self.owner, self.sequence);
        let start = self.time();
        let mut records = Records {
            dom: Vec::new(),
            ax: Vec::new(),
            dom_start: start,
            dom_end: start,
            ax_start: start,
            ax_end: start,
            ax_status: if plan.len() == 0 {
                SourceStatus::Complete
            } else {
                SourceStatus::Partial
            },
            ax_queries: 0,
            selection: None,
        };
        records
            .dom
            .try_reserve_exact(plan.len())
            .map_err(|_| Failure::new(ErrorKind::Limit))?;
        records
            .ax
            .try_reserve_exact(plan.len())
            .map_err(|_| Failure::new(ErrorKind::Limit))?;
        let outcome = self.collect_nodes(plan, request, &group, budget, &mut records);
        // No new protocol command after cancel/expiry or a failed connection. Dropping the
        // owned connection stops reuse; explicit remote release then remains unconfirmed.
        let cleanup = if plan.len() == 0 {
            Ok(())
        } else {
            self.send::<wire::Empty, _>(
                "Runtime.releaseObjectGroup",
                &serde_json::json!({"objectGroup":group}),
                budget,
            )
            .map(|_| ())
        };
        if let Err(error) = outcome {
            if cleanup.is_err() {
                self.detach();
            }
            return Err(error.with_cleanup(if cleanup.is_ok() {
                RemoteCleanup::Released
            } else {
                RemoteCleanup::Unconfirmed
            }));
        }
        if cleanup.is_err() {
            self.detach();
            return Err(Failure::new(ErrorKind::CleanupUnconfirmed)
                .with_cleanup(RemoteCleanup::Unconfirmed));
        }
        Ok(records)
    }
    fn collect_nodes(
        &mut self,
        plan: Plan<'_>,
        request: &Request,
        group: &str,
        budget: &mut Budget,
        records: &mut Records,
    ) -> Result<(), Failure> {
        let document = if plan.len() == 0 {
            None
        } else {
            Some(self.resolve(self.document, group, budget)?)
        };
        let need_ax = needs_ax(&request.context.fields);
        if !need_ax {
            records.ax_status = SourceStatus::NotRequested;
        }
        let mut handles = Vec::new();
        handles
            .try_reserve_exact(plan.len())
            .map_err(|_| Failure::new(ErrorKind::Limit))?;
        let selected = match plan {
            Plan::References(_) => None,
            Plan::Initial(scope) => {
                let (nodes, progress) = self.select_initial(
                    scope,
                    request,
                    document
                        .as_deref()
                        .ok_or(Failure::new(ErrorKind::Malformed))?,
                    budget,
                )?;
                records.selection = Some(progress);
                Some(nodes)
            }
        };
        for index in 0..plan.len() {
            handles.push(match plan {
                Plan::References(scope) => {
                    self.resolve(scope.nodes[index].backend_id()?, group, budget)?
                }
                Plan::Initial(_) => selected
                    .as_ref()
                    .ok_or(Failure::new(ErrorKind::Malformed))?[index]
                    .object
                    .clone(),
            });
        }
        for index in 0..plan.len() {
            let (backend, sensitivity) = match plan {
                Plan::References(scope) => (
                    scope.nodes[index].backend_id()?,
                    scope.nodes[index].sensitivity,
                ),
                Plan::Initial(_) => {
                    let node = &selected
                        .as_ref()
                        .ok_or(Failure::new(ErrorKind::Malformed))?[index];
                    (node.backend, node.sensitivity)
                }
            };
            let object = &handles[index];
            let mut arguments = Vec::new();
            arguments
                .try_reserve_exact(handles.len() + 2)
                .map_err(|_| Failure::new(ErrorKind::Limit))?;
            arguments.push(Argument::Options {
                value: ReadOptions {
                    fields: &request.context.fields,
                    max_chars: self.limits.max_text_bytes / 6,
                    sensitive: sensitivity == Sensitivity::Sensitive,
                },
            });
            arguments.push(Argument::Object {
                object_id: document
                    .as_deref()
                    .ok_or(Failure::new(ErrorKind::Malformed))?,
            });
            arguments.extend(
                handles
                    .iter()
                    .map(|handle| Argument::Object { object_id: handle }),
            );
            let params = Read {
                object_id: object,
                function_declaration: READ_NODE,
                return_by_value: true,
                silent: true,
                user_gesture: false,
                await_promise: false,
                // Chromium 145 marks getClientRects/matches as potentially effectful
                // for debugger evaluation, despite these being native read APIs.
                // Only this fixed isolated-world reader uses ordinary evaluation;
                // arguments remain data and it never invokes application callbacks.
                throw_on_side_effect: false,
                arguments,
            };
            let read: wire::ReadResult = self
                .send("Runtime.callFunctionOn", &params, budget)
                .map_err(|e| e.at(MalformedSite::ReadReply))?;
            if read.exception_details.is_some() {
                return Err(Failure::new(ErrorKind::Malformed).at(MalformedSite::ReadException));
            }
            if read.result.r#type != "object" {
                return Err(Failure::new(ErrorKind::Malformed).at(MalformedSite::ReadShape));
            }
            let mut read = read
                .result
                .value
                .ok_or(Failure::new(ErrorKind::Malformed).at(MalformedSite::ReadShape))?;
            if !read.connected || !read.same_document {
                return Err(Failure::new(ErrorKind::StaleTarget));
            }
            // Caller sensitivity cannot be downgraded by page/protocol data.
            read.sensitive |= sensitivity == Sensitivity::Sensitive;
            let sensitive = read.sensitive;
            if sensitive {
                read.value = None;
                read.placeholder = None;
                read.input_kind = None;
                read.tag = None;
                read.controls = None;
                read.declared_anchor = None;
                read.active_descendant = None;
            }
            validate_dom(&read, self.limits.max_text_bytes, plan.len())
                .map_err(|e| e.at(MalformedSite::ReadData))?;
            records.dom.push((backend, read));
            records.dom_end = self.time();
            if !need_ax {
                continue;
            }
            if sensitive {
                if !matches!(records.ax_status, SourceStatus::Failed(_)) {
                    records.ax_status = SourceStatus::Partial;
                }
                continue;
            }
            if matches!(records.ax_status, SourceStatus::Failed(_)) {
                continue;
            }
            let ax_start = self.time();
            if records.ax_queries == 0 {
                records.ax_start = ax_start;
            }
            records.ax_queries += 1;
            let result = self.send::<wire::AxResult, _>(
                "Accessibility.getPartialAXTree",
                &serde_json::json!({"backendNodeId":backend,"fetchRelatives":false}),
                budget,
            );
            records.ax_end = self.time();
            let mut result = match result {
                Ok(v) => v,
                Err(error) if matches!(error.kind, ErrorKind::Protocol(_)) => {
                    records.ax_status = SourceStatus::Failed(error.kind);
                    continue;
                }
                Err(error) => return Err(error.at(MalformedSite::AxReply)),
            };
            if result.nodes.is_empty() {
                records.ax_status = SourceStatus::Partial;
                continue;
            }
            if result.nodes.len() != 1 {
                return Err(Failure::new(ErrorKind::Limit));
            }
            let ax = result
                .nodes
                .pop()
                .ok_or(Failure::new(ErrorKind::Malformed))?;
            validate_ax(&ax, backend, &self.binding.surface, self.limits)
                .map_err(|e| e.at(MalformedSite::AxData))?;
            records.ax.push((backend, ax));
        }
        // Final target/document check and one bounded pass over the ORIGINAL handles.
        // Do this after every source read, immediately before group release/publication.
        self.verify_document(budget)?;
        if let Some(document) = document.as_deref() {
            self.verify_nodes(document, &handles, budget)?;
        }
        Ok(())
    }
    fn verify_nodes(
        &mut self,
        document: &str,
        handles: &[String],
        budget: &mut Budget,
    ) -> Result<(), Failure> {
        let mut arguments = Vec::new();
        arguments
            .try_reserve_exact(handles.len() + 1)
            .map_err(|_| Failure::new(ErrorKind::Limit))?;
        arguments.push(Argument::Object {
            object_id: document,
        });
        arguments.extend(
            handles
                .iter()
                .map(|handle| Argument::Object { object_id: handle }),
        );
        let params = Read {
            object_id: document,
            function_declaration: VERIFY_NODES,
            return_by_value: true,
            silent: true,
            user_gesture: false,
            await_promise: false,
            throw_on_side_effect: true,
            arguments,
        };
        let checked: wire::VerifyResult = self
            .send("Runtime.callFunctionOn", &params, budget)
            .map_err(|e| e.at(MalformedSite::ContinuityReply))?;
        if checked.exception_details.is_some() {
            return Err(Failure::new(ErrorKind::Malformed).at(MalformedSite::ContinuityException));
        }
        if checked.result.r#type != "object" {
            return Err(Failure::new(ErrorKind::Malformed).at(MalformedSite::ContinuityShape));
        }
        let current = checked
            .result
            .value
            .ok_or(Failure::new(ErrorKind::Malformed).at(MalformedSite::ContinuityShape))?;
        if !current.current {
            return Err(Failure::new(ErrorKind::StaleTarget));
        }
        Ok(())
    }
    fn resolve(
        &mut self,
        backend: u32,
        group: &str,
        budget: &mut Budget,
    ) -> Result<String, Failure> {
        let resolved: wire::ResolveResult = self
            .send(
                "DOM.resolveNode",
                &Resolve {
                    backend_node_id: backend,
                    execution_context_id: self.world,
                    object_group: group,
                },
                budget,
            )
            .map_err(|e| e.at(MalformedSite::ResolveReply))?;
        let remote = resolved.object;
        if remote.r#type != "object" || remote.subtype.as_deref() != Some("node") {
            return Err(Failure::new(ErrorKind::StaleTarget));
        }
        let object = remote
            .object_id
            .ok_or(Failure::new(ErrorKind::Malformed).at(MalformedSite::ResolveReply))?;
        if object.is_empty() || object.len() > self.limits.max_handle_bytes {
            return Err(Failure::new(ErrorKind::Limit));
        }
        Ok(object)
    }
    pub(super) fn normalize(
        &self,
        request: &Request,
        plan: Plan<'_>,
        records: Records,
    ) -> Result<Snapshot, Failure> {
        let empty = plan.len() == 0;
        let dom = normalize::observation(
            &request.context,
            &self.binding.clock.domain,
            "web.dom",
            (self.owner, self.sequence),
            [records.dom_start, records.dom_end],
            true,
            empty,
        );
        let ax = normalize::observation(
            &request.context,
            &self.binding.clock.domain,
            "web.ax",
            (self.owner, self.sequence),
            [records.ax_start, records.ax_end],
            !records.ax.is_empty(),
            empty,
        );
        let mut nodes = Vec::new();
        let mut relations = Vec::new();
        nodes
            .try_reserve_exact(records.dom.len() + records.ax.len())
            .map_err(|_| Failure::new(ErrorKind::Limit))?;
        relations
            .try_reserve_exact(records.ax.len())
            .map_err(|_| Failure::new(ErrorKind::Limit))?;
        for (backend, read) in &records.dom {
            nodes.push(normalize::dom(
                *backend,
                read,
                &request.context,
                &dom,
                self.limits.max_text_bytes,
            ));
        }
        for (backend, read) in &records.ax {
            let node = normalize::ax(read, &request.context, &ax, self.limits.max_text_bytes);
            relations.push(Relation {
                kind: RelationKind::CorrespondsTo,
                from: SourceKey {
                    namespace: normalize::id("web.dom"),
                    key: Id(backend.to_string()),
                },
                to: node.key.clone(),
                evidence: normalize::evidence(&ax, "cdp-backendDOMNodeId"),
            });
            nodes.push(node);
        }
        normalize::dom_relations(&records.dom, &dom, &mut relations);
        let active_descendant = normalize::active_descendant(&records.dom, &dom, &request.context);
        let mut snapshot = normalize::snapshot(
            request,
            (self.owner, self.sequence),
            if records.ax_queries > 0 {
                vec![dom, ax]
            } else {
                vec![dom]
            },
            nodes,
            relations,
            empty,
        );
        snapshot.focus.active_descendant = active_descendant;
        validation::validate_snapshot(&snapshot).map_err(|_| Failure::new(ErrorKind::Malformed))?;
        Ok(snapshot)
    }
}
fn validate_dom(read: &wire::DomRead, cap: usize, selected: usize) -> Result<(), Failure> {
    if read.controls.as_ref().is_some_and(|v| {
        v.len() > selected
            || v.iter()
                .enumerate()
                .any(|(i, n)| *n >= selected || v[..i].contains(n))
    }) || [read.declared_anchor, read.active_descendant]
        .into_iter()
        .flatten()
        .any(|i| i >= selected)
    {
        return Err(Failure::new(ErrorKind::Malformed));
    }
    if read.tag.as_ref().is_some_and(|tag| tag.len() > cap)
        || read.rect.as_ref().is_some_and(|r| {
            ![r.x, r.y, r.width, r.height].iter().all(|v| v.is_finite())
                || r.width < 0.0
                || r.height < 0.0
        })
    {
        return Err(Failure::new(ErrorKind::Malformed));
    }
    Ok(())
}
fn validate_ax(
    ax: &wire::AxNode,
    backend: u32,
    surface: &Identity,
    limits: Limits,
) -> Result<(), Failure> {
    if ax.backend_dom_node_id != Some(backend)
        || ax.frame_id.as_ref().is_some_and(|f| *f != surface.id.0)
    {
        return Err(Failure::new(ErrorKind::StaleTarget));
    }
    if ax.node_id.is_empty() || ax.node_id.chars().count() > 256 {
        return Err(Failure::new(ErrorKind::Malformed));
    }
    if let Some(properties) = &ax.properties {
        if properties.len() > limits.max_ax_properties {
            return Err(Failure::new(ErrorKind::Limit));
        }
        for (i, p) in properties.iter().enumerate() {
            if properties[..i].iter().any(|old| old.name == p.name) {
                return Err(Failure::new(ErrorKind::Malformed));
            }
        }
    }
    Ok(())
}
