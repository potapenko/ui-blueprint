use super::*;
use serde::{Serialize, de::DeserializeOwned};
use uiblueprint_schema::validation;

impl Limits {
    fn validate(self) -> Result<(), Failure> {
        let caps = [
            self.max_nodes,
            self.max_methods as usize,
            self.max_reply_bytes,
            self.max_total_reply_bytes,
            self.max_text_bytes,
            self.max_handle_bytes,
            self.max_ax_properties,
            self.io_read_bytes,
            self.io_write_bytes,
            self.io_work,
        ];
        if caps.iter().any(|&n| n == 0 || n > isize::MAX as usize)
            || self.max_text_bytes < 6
            || self.max_text_bytes > u32::MAX as usize
            || self.max_reply_bytes > self.max_total_reply_bytes
        {
            return Err(Failure::new(ErrorKind::InvalidInput));
        }
        (self.max_methods as usize)
            .checked_mul(self.io_read_bytes)
            .ok_or(Failure::new(ErrorKind::Limit))?;
        self.max_nodes
            .checked_mul(2)
            .filter(|&n| n <= u32::MAX as usize)
            .ok_or(Failure::new(ErrorKind::Limit))?;
        Ok(())
    }
}
fn valid_id(id: &Id) -> bool {
    !id.0.is_empty() && id.0.chars().count() <= 256
}
impl Collector {
    /// Verifies the current target and root frame/loader, then creates an isolated
    /// read realm and enables stable AX IDs. Only explicitly owned connection setup.
    pub fn attach(
        client: cdp::Client,
        binding: Binding,
        limits: Limits,
        deadline: Instant,
    ) -> Result<Self, Failure> {
        limits.validate()?;
        let incoming = client
            .transport_limits()
            .ok_or(Failure::new(ErrorKind::Cdp(cdp::ErrorKind::Detached)))?;
        if incoming.frame_bytes > limits.max_reply_bytes
            || incoming.message_bytes > limits.max_reply_bytes
        {
            return Err(Failure::new(ErrorKind::InvalidInput));
        }
        let ids = [
            &binding.session_id,
            &binding.target.id,
            &binding.target.generation,
            &binding.surface.id,
            &binding.surface.generation,
            &binding.clock.domain,
            &binding.plugin.id,
            &binding.plugin.version,
        ];
        if ids.into_iter().any(|id| !valid_id(id))
            || binding.allowed_scopes.is_empty()
            || binding.allowed_scopes.len() > limits.max_nodes
            || binding.allowed_scopes.iter().any(|id| !valid_id(id))
            || binding
                .cdp_session_id
                .as_ref()
                .is_some_and(|id| !valid_id(id))
        {
            return Err(Failure::new(ErrorKind::InvalidInput));
        }
        let loss_generation = client.event_loss_generation();
        let origin = binding.clock.origin;
        if Instant::now().checked_duration_since(origin).is_none() {
            return Err(Failure::new(ErrorKind::InvalidInput));
        }
        let mut owner = Self {
            client,
            binding,
            limits,
            origin,
            document: 0,
            world: 0,
            invalid: false,
            owner: next_owner()?,
            sequence: 0,
            loss_generation,
            pending_invalidation: false,
        };
        let mut budget = Budget {
            deadline,
            methods: 0,
            reply_bytes: 0,
            max_reply_bytes: limits.max_total_reply_bytes,
        };
        owner.verify_target(&mut budget)?;
        let document: wire::DocumentResult = owner.send(
            "DOM.getDocument",
            &serde_json::json!({"depth":0,"pierce":false}),
            &mut budget,
        )?;
        if document.root.node_type != 9
            || (document.root.backend_node_id == 0
                || document.root.backend_node_id > i32::MAX as u32)
        {
            return Err(Failure::new(ErrorKind::Malformed));
        }
        owner.document = document.root.backend_node_id;
        let world:wire::WorldResult=owner.send("Page.createIsolatedWorld",&serde_json::json!({"frameId":owner.binding.surface.id.0,"worldName":"ui-blueprint-read-only","grantUniveralAccess":false}),&mut budget)?;
        if world.execution_context_id <= 0 {
            return Err(Failure::new(ErrorKind::Malformed));
        }
        owner.world = world.execution_context_id;
        let _: wire::Empty =
            owner.send("Accessibility.enable", &serde_json::json!({}), &mut budget)?;
        owner.verify_document(&mut budget)?;
        Ok(owner)
    }
    fn verify_target(&mut self, budget: &mut Budget) -> Result<(), Failure> {
        let target: wire::TargetResult =
            self.send("Target.getTargetInfo", &serde_json::json!({}), budget)?;
        if target.target_info.target_id != self.binding.target.id.0 {
            self.detach();
            return Err(Failure::new(ErrorKind::StaleTarget));
        }
        let frame: wire::FrameResult =
            self.send("Page.getFrameTree", &serde_json::json!({}), budget)?;
        if frame.frame_tree.frame.id != self.binding.surface.id.0
            || frame.frame_tree.frame.loader_id != self.binding.surface.generation.0
        {
            self.pending_invalidation = true;
            self.invalid = true;
            return Err(Failure::new(ErrorKind::StaleTarget));
        }
        Ok(())
    }
    pub(super) fn verify_document(&mut self, budget: &mut Budget) -> Result<(), Failure> {
        self.verify_target(budget)?;
        let doc: wire::DocumentResult = self.send(
            "DOM.getDocument",
            &serde_json::json!({"depth":0,"pierce":false}),
            budget,
        )?;
        if doc.root.node_type != 9 || doc.root.backend_node_id != self.document {
            self.pending_invalidation = true;
            self.invalid = true;
            return Err(Failure::new(ErrorKind::StaleTarget));
        }
        Ok(())
    }
    pub(super) fn check(&mut self, budget: &Budget) -> Result<(), Failure> {
        if self.invalid {
            self.pending_invalidation = true;
            self.client.detach();
            return Err(Failure::new(ErrorKind::StaleTarget));
        }
        if self.client.cancellation().is_none_or(|c| c.is_cancelled()) {
            self.detach();
            return Err(Failure::new(ErrorKind::Cdp(cdp::ErrorKind::Cancelled)));
        }
        if Instant::now() >= budget.deadline {
            self.detach();
            return Err(Failure::new(ErrorKind::Timeout));
        }
        Ok(())
    }
    pub(super) fn send<T: DeserializeOwned, P: Serialize>(
        &mut self,
        method: &str,
        params: &P,
        budget: &mut Budget,
    ) -> Result<T, Failure> {
        self.check(budget)?;
        if budget.methods >= self.limits.max_methods {
            return Err(Failure::new(ErrorKind::Limit));
        }
        let remaining = budget
            .max_reply_bytes
            .checked_sub(budget.reply_bytes)
            .ok_or(Failure::new(ErrorKind::Limit))?;
        if remaining == 0 {
            return Err(Failure::new(ErrorKind::Limit));
        }
        // Codec caps govern payload acquisition, including previously buffered bytes.
        // Wire IO/work budgets remain separate: framing/control consume those budgets.
        let incoming = self
            .client
            .transport_limits()
            .ok_or(Failure::new(ErrorKind::Cdp(cdp::ErrorKind::Detached)))?;
        let allowance = self.limits.max_reply_bytes.min(remaining);
        if incoming.frame_bytes > allowance || incoming.message_bytes > allowance {
            return Err(Failure::new(ErrorKind::Limit));
        }
        budget.methods += 1;
        let pending = self
            .client
            .prepare(
                method,
                params,
                transport::OperationLimits {
                    deadline: budget.deadline,
                    max_read_bytes: self.limits.io_read_bytes.min(remaining),
                    max_write_bytes: self.limits.io_write_bytes,
                    max_work: self.limits.io_work,
                },
            )
            .map_err(cdp_failure)?;
        let binding = pending.ticket().binding();
        if binding.target != self.binding.target
            || binding.cdp_session_id != self.binding.cdp_session_id
        {
            pending.cancel();
            self.pending_invalidation = true;
            self.invalid = true;
            return Err(Failure::new(ErrorKind::StaleTarget));
        }
        let reply = match pending.run() {
            Ok(reply) => reply,
            Err(error) => {
                // CDP closes on failed exchange and may discard queued events.
                // Lost continuity conservatively invalidates saved session data.
                self.pending_invalidation = true;
                return Err(cdp_failure(error));
            }
        };
        budget.reply_bytes = budget
            .reply_bytes
            .checked_add(reply.wire().len())
            .ok_or(Failure::new(ErrorKind::Limit))?;
        if reply.wire().len() > self.limits.max_reply_bytes
            || budget.reply_bytes > budget.max_reply_bytes
        {
            self.detach();
            return Err(Failure::new(ErrorKind::Limit));
        }
        while let Some(event) = self.client.pop_event() {
            if matches!(
                event.method(),
                "Page.frameNavigated"
                    | "DOM.documentUpdated"
                    | "DOM.childNodeRemoved"
                    | "Runtime.executionContextsCleared"
                    | "Inspector.detached"
            ) {
                self.pending_invalidation = true;
                self.invalid = true;
            } else if matches!(
                event.method(),
                "Accessibility.nodesUpdated"
                    | "Accessibility.loadComplete"
                    | "DOM.attributeModified"
                    | "DOM.attributeRemoved"
                    | "DOM.characterDataModified"
                    | "DOM.childNodeInserted"
                    | "DOM.inlineStyleInvalidated"
                    | "CSS.styleSheetChanged"
                    | "CSS.styleSheetAdded"
                    | "CSS.styleSheetRemoved"
                    | "CSS.mediaQueryResultChanged"
                    | "Page.frameResized"
            ) {
                self.pending_invalidation = true;
            }
        }
        if self.client.event_loss_generation() != self.loss_generation {
            self.detach();
            return Err(Failure::new(ErrorKind::ResyncRequired));
        }
        self.check(budget)?;
        if let cdp::ReplyKind::Error { code } = reply.kind() {
            return Err(Failure {
                kind: ErrorKind::Protocol(code),
                malformed_site: None,
                send_progress: transport::SendProgress::Flushed,
                remote_cleanup: RemoteCleanup::NotRequired,
            });
        }
        #[derive(serde::Deserialize)]
        struct Envelope<T> {
            result: T,
        }
        let parsed: Envelope<T> =
            serde_json::from_str(reply.wire()).map_err(|_| Failure::new(ErrorKind::Malformed))?;
        self.check(budget)?;
        Ok(parsed.result)
    }
    pub(super) fn validate_plan(
        &self,
        request: &Request,
        scope_id: &Id,
        node_count: usize,
    ) -> Result<(), Failure> {
        if !valid_id(&request.request_id) || !valid_id(&request.context.environment_revision) {
            return Err(Failure::new(ErrorKind::InvalidInput));
        }
        validation::validate_request(request).map_err(|_| Failure::new(ErrorKind::InvalidInput))?;
        if request.context.target != self.binding.target
            || request.context.session_id != self.binding.session_id
            || request.context.surfaces != [self.binding.surface.clone()]
            || request.context.plugin != self.binding.plugin
            || request.clock_domain != self.binding.clock.domain
            || request.context.scope_id != *scope_id
            || !self.binding.allowed_scopes.contains(scope_id)
        {
            return Err(Failure::new(ErrorKind::StaleTarget));
        }
        if !matches!(request.operation, Operation::Observe { .. })
            || node_count > self.limits.max_nodes
            || node_count
                .checked_mul(if needs_ax(&request.context.fields) {
                    2
                } else {
                    1
                })
                .is_none_or(|n| n > request.limits.max_elements as usize)
        {
            return Err(Failure::new(ErrorKind::Limit));
        }
        Ok(())
    }
    pub(super) fn validate_root(&self, scope: &RootedScope) -> Result<(), Failure> {
        let root = &scope.root;
        if root.backend_node_id == 0
            || root.backend_node_id > i32::MAX as u32
            || root.document_backend_id == 0
            || root.document_backend_id > i32::MAX as u32
        {
            return Err(Failure::new(ErrorKind::InvalidInput));
        }
        if root.session_id != self.binding.session_id
            || root.target != self.binding.target
            || root.surface != self.binding.surface
            || root.document_backend_id != self.document
        {
            return Err(Failure::new(ErrorKind::StaleTarget));
        }
        Ok(())
    }
    pub(super) fn validate_request(&self, request: &Request, scope: &Scope) -> Result<(), Failure> {
        self.validate_plan(request, &scope.scope_id, scope.nodes.len())?;
        for (i, node) in scope.nodes.iter().enumerate() {
            let backend = node.backend_id()?;
            let reference = &node.reference;
            if reference.key.namespace.0 != "web.dom"
                || reference.surface != self.binding.surface
                || reference.target != self.binding.target
                || reference.session_id != self.binding.session_id
                || !valid_id(&reference.snapshot_id)
                || !valid_id(&reference.observation_id)
                || scope.nodes[..i]
                    .iter()
                    .any(|old| old.backend_id() == Ok(backend))
            {
                return Err(Failure::new(ErrorKind::StaleTarget));
            }
        }
        Ok(())
    }
}
fn cdp_failure(f: cdp::Failure) -> Failure {
    Failure {
        kind: ErrorKind::Cdp(f.kind),
        malformed_site: None,
        send_progress: f.send_progress,
        remote_cleanup: RemoteCleanup::NotRequired,
    }
}
