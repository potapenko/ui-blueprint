use super::{acquire::Read, observe, snapshot_normalize, snapshot_wire, *};
use serde::Deserialize;
use std::time::Duration;
use uiblueprint_schema::validation;

/// Caller-observed document identity, within the trusted attached Surface set.
pub struct DocumentSeed {
    pub surface: Identity,
    pub document_backend_id: u32,
    pub sensitivity: Sensitivity,
}
/// Explicit whole-document workload. Never a fallback from a smaller scope.
pub struct DocumentsScope {
    pub scope_id: Id,
    pub documents: Vec<DocumentSeed>,
    pub max_visited_nodes: u32,
}
#[derive(Deserialize)]
struct Check {
    status: String,
    count: usize,
}
#[derive(Deserialize)]
struct CheckedValue {
    value: Check,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Checked {
    result: CheckedValue,
    exception_details: Option<serde::de::IgnoredAny>,
}

impl Collector {
    /// Additional Surfaces come only from the trusted attached descriptor. This
    /// does not collect them; every Documents request must select and revalidate.
    pub fn attach_with_surfaces(
        client: cdp::Client,
        binding: Binding,
        surfaces: Vec<Identity>,
        limits: Limits,
        deadline: Instant,
    ) -> Result<Self, Failure> {
        if surfaces.is_empty()
            || surfaces.len() > limits.max_nodes
            || !surfaces.contains(&binding.surface)
            || surfaces.iter().enumerate().any(|(i, s)| {
                [&s.id, &s.generation]
                    .iter()
                    .any(|v| v.0.is_empty() || v.0.chars().count() > 256)
                    || surfaces[..i].iter().any(|old| old.id == s.id)
            })
        {
            return Err(Failure::new(ErrorKind::InvalidInput));
        }
        let mut collector = Self::attach(client, binding, limits, deadline)?;
        collector.allowed_surfaces = surfaces;
        Ok(collector)
    }

    pub fn observe_documents(
        &mut self,
        request: &Request,
        scope: &DocumentsScope,
        dispatch_sequence: u64,
        deadline: Instant,
        mut publish: impl FnMut(Document) -> Publication,
    ) -> Result<Report, Failure> {
        validation::validate_request(request).map_err(|_| Failure::new(ErrorKind::InvalidInput))?;
        let c = &request.context;
        if c.target != self.binding.target
            || c.session_id != self.binding.session_id
            || c.plugin != self.binding.plugin
            || request.clock_domain != self.binding.clock.domain
            || c.scope_id != scope.scope_id
            || !self.binding.allowed_scopes.contains(&c.scope_id)
            || c.surfaces.len() != scope.documents.len()
            || c.surfaces != self.allowed_surfaces
            || scope
                .documents
                .iter()
                .zip(&c.surfaces)
                .any(|(d, s)| &d.surface != s)
            || scope.documents.iter().any(|d| {
                d.surface == self.binding.surface && d.document_backend_id != self.document
            })
        {
            return Err(Failure::new(ErrorKind::StaleTarget));
        }
        if dispatch_sequence == 0
            || scope.max_visited_nodes == 0
            || scope.max_visited_nodes as usize > self.limits.max_nodes
            || scope.max_visited_nodes > request.limits.max_elements
            || scope
                .documents
                .iter()
                .any(|d| d.document_backend_id == 0 || d.document_backend_id > i32::MAX as u32)
            || !matches!(&request.operation, Operation::Observe{channels} if channels == &[Channel::ExternalSemantics])
            || c.fields
                .iter()
                .any(|f| !matches!(f, Field::Value | Field::LayoutBounds))
        {
            return Err(Failure::new(ErrorKind::InvalidInput));
        }
        let own = Instant::now()
            .checked_add(Duration::from_millis(request.limits.deadline_ms))
            .ok_or(Failure::new(ErrorKind::InvalidInput))?;
        let mut budget = Budget {
            deadline: deadline.min(own),
            methods: 0,
            reply_bytes: 0,
            max_reply_bytes: self.limits.max_total_reply_bytes,
        };
        self.check(&budget)?;
        self.sequence = self
            .sequence
            .checked_add(1)
            .ok_or(Failure::new(ErrorKind::Limit))?;
        let group = format!("ui-blueprint-documents:{}:{}", self.owner, self.sequence);
        let start = self.time();
        let result = (|| {
            self.verify_document(&mut budget)?;
            self.verify_frames(scope, &mut budget)?;
            let before = self.check_documents(scope, request, &group, &mut budget)?;
            let capture: snapshot_wire::Capture = self.send(
                "DOMSnapshot.captureSnapshot",
                &serde_json::json!({"computedStyles":[],"includeDOMRects":true}),
                &mut budget,
            )?;
            self.verify_frames(scope, &mut budget)?;
            let after = self.check_documents(scope, request, &group, &mut budget)?;
            self.verify_document(&mut budget)?;
            if before != after {
                return Err(Failure::new(ErrorKind::ResyncRequired));
            }
            snapshot_normalize::snapshot(
                capture,
                scope,
                request,
                &before,
                crate::normalize::observation(
                    &request.context,
                    &self.binding.clock.domain,
                    "web.dom",
                    (self.owner, self.sequence),
                    [start, self.time()],
                    true,
                    false,
                ),
                (self.owner, self.sequence),
                self.limits,
            )
        })();
        let cleanup = self.send::<wire::Empty, _>(
            "Runtime.releaseObjectGroup",
            &serde_json::json!({"objectGroup":group}),
            &mut budget,
        );
        if cleanup.is_err() {
            self.detach();
            return Err(result
                .err()
                .unwrap_or(Failure::new(ErrorKind::CleanupUnconfirmed))
                .with_cleanup(RemoteCleanup::Unconfirmed));
        }
        let snapshot = result.map_err(|e| e.with_cleanup(RemoteCleanup::Released))?;
        let count = snapshot.nodes.len();
        let response = observe::document(
            request,
            dispatch_sequence,
            Channel::ExternalSemantics,
            ChannelResult::Observed(Box::new(snapshot)),
        );
        let bytes = observe::bounded_document(&response, request.limits.max_output_bytes)?;
        self.check(&budget)?;
        if publish(response) != Publication::Acknowledged {
            self.detach();
            return Err(Failure::new(ErrorKind::PublicationStopped));
        }
        Ok(Report {
            dom: SourceStatus::Partial,
            ax: SourceStatus::NotRequested,
            dom_nodes: count,
            ax_nodes: 0,
            visited_dom: count,
            queried_ax: 0,
            omitted_nodes: Some(0),
            methods: budget.methods,
            reply_bytes: budget.reply_bytes,
            published_channels: 1,
            canonical_bytes: bytes,
        })
    }

    fn verify_frames(
        &mut self,
        scope: &DocumentsScope,
        budget: &mut Budget,
    ) -> Result<(), Failure> {
        let tree: wire::FrameResult =
            self.send("Page.getFrameTree", &serde_json::json!({}), budget)?;
        let mut pending = vec![&tree.frame_tree];
        let mut seen = Vec::new();
        while let Some(frame) = pending.pop() {
            if seen.len() >= scope.documents.len() || seen.contains(&frame.frame.id) {
                return Err(Failure::new(ErrorKind::StaleTarget));
            }
            let expected = scope
                .documents
                .iter()
                .find(|d| d.surface.id.0 == frame.frame.id)
                .ok_or(Failure::new(ErrorKind::StaleTarget))?;
            if expected.surface.generation.0 != frame.frame.loader_id {
                return Err(Failure::new(ErrorKind::StaleTarget));
            }
            seen.push(frame.frame.id.clone());
            if let Some(children) = &frame.child_frames {
                pending.extend(children);
            }
        }
        if seen.len() != scope.documents.len() {
            return Err(Failure::new(ErrorKind::StaleTarget));
        }
        Ok(())
    }

    fn check_documents(
        &mut self,
        scope: &DocumentsScope,
        request: &Request,
        group: &str,
        budget: &mut Budget,
    ) -> Result<Vec<usize>, Failure> {
        let mut counts = Vec::new();
        let mut total = 0;
        for seed in &scope.documents {
            let world:wire::WorldResult=self.send("Page.createIsolatedWorld",
                &serde_json::json!({"frameId":seed.surface.id.0,"worldName":"ui-blueprint-read-only","grantUniveralAccess":false}),budget)?;
            // Resolve in the exact frame's realm, then the fixed function checks
            // object===document. A backend ID from another frame cannot qualify.
            let object: wire::ResolveResult = self.send(
                "DOM.resolveNode",
                &serde_json::json!({"backendNodeId":seed.document_backend_id,
                    "executionContextId":world.execution_context_id,"objectGroup":group}),
                budget,
            )?;
            let handle = object
                .object
                .object_id
                .filter(|h| !h.is_empty() && h.len() <= self.limits.max_handle_bytes)
                .ok_or(Failure::new(ErrorKind::StaleTarget))?;
            let remaining = scope.max_visited_nodes as usize - total;
            let result:Checked=self.send("Runtime.callFunctionOn", &Read {
                object_id:&handle,function_declaration:include_str!("document-check.js"),
                return_by_value:true,silent:true,user_gesture:false,await_promise:false,throw_on_side_effect:false,
                arguments:[serde_json::json!({"value":{"maxNodes":remaining,
                    "maxAttributes":self.limits.max_ax_properties,"maxChars":self.limits.max_text_bytes / 6,
                    "sensitive":seed.sensitivity==Sensitivity::Sensitive,
                    "maxDepth":request.limits.max_depth,"remainingMs":budget.deadline.saturating_duration_since(Instant::now()).as_secs_f64()*1000.0}})],
            },budget)?;
            if result.exception_details.is_some() {
                return Err(Failure::new(ErrorKind::Malformed));
            }
            match result.result.value.status.as_str() {
                "current" => {}
                "private" => return Err(Failure::new(ErrorKind::InvalidInput)),
                "timeout" => return Err(Failure::new(ErrorKind::Timeout)),
                "limit" => return Err(Failure::new(ErrorKind::Limit)),
                "unsupported" => {
                    return Err(Failure::new(ErrorKind::Selection {
                        status: SelectionStatus::Unsupported,
                        visited_nodes: total as u32,
                    }));
                }
                _ => return Err(Failure::new(ErrorKind::StaleTarget)),
            }
            let count = result.result.value.count;
            if count == 0 || count > remaining {
                return Err(Failure::new(ErrorKind::Limit));
            }
            total += count;
            counts.push(count);
        }
        Ok(counts)
    }
}
