//! Native provider uses the parent's registered resident helper and existing kernel.
use crate::{worker_io::WorkerIo, worker_main, worker_ops::CanonicalSession};
use uiblueprint_host::*;
use uiblueprint_plugin_api::{ClockReading, actions::*};
use uiblueprint_schema::model::*;

struct Provider<'a> {
    io: &'a WorkerIo,
    control: Control,
    bytes: Vec<u8>,
}
impl Provider<'_> {
    fn exchange(&mut self, phase: u8, nonce: u64) -> Result<&[u8], HostError> {
        self.io.write_control(Control {
            kind: ControlKind::HelperRequest,
            class: self.control.class,
            slot: 0,
            flags: phase,
            correlation: self.control.correlation,
            length: 0,
            value: self.control.correlation.operation,
            auxiliary: nonce,
        })?;
        let reply = self.io.control()?;
        if reply.kind != ControlKind::HelperReply
            || reply.class != self.control.class
            || reply.correlation != self.control.correlation
            || reply.slot != 0
            || reply.flags != 0
            || reply.value == 0
            || reply.auxiliary != self.control.correlation.operation
            || reply.length == 0
            || reply.length > self.bytes.len() as u64
        {
            return Err(HostError::InvalidControl);
        }
        let size = reply.length as usize;
        self.io.read(&mut self.bytes[..size])?;
        Ok(&self.bytes[..size])
    }
    fn document(&mut self, phase: u8) -> Result<Document, HostError> {
        let bytes = self.exchange(phase, 0)?;
        Document::from_json(bytes, bytes.len()).map_err(|_| HostError::InvalidInput)
    }
}
fn issue(action: &Action) -> Issue {
    Issue {
        code: ErrorCode::StaleTarget,
        scope_id: action.authorized_scope.clone(),
        failed_step: Some(action.id.clone()),
        recovery_class: Id("new_native_observation".into()),
    }
}
impl ActionProvider for Provider<'_> {
    fn resolve_exact(
        &mut self,
        case: &ActionCase,
        _: &Expectation,
        _: &ClockReading,
        _: u64,
    ) -> Result<ActionCase, Issue> {
        match self.document(1).map(|d| d.artifact) {
            Ok(Artifact::Action(fresh)) => Ok(*fresh),
            Ok(Artifact::Error(issue)) => Err(*issue),
            _ => Err(issue(&case.action)),
        }
    }
    fn deliver(&mut self, _: &Action, permit: DeliveryPermit, _: u64) -> DeliveryStatus {
        match self.exchange(2, permit.nonce()) {
            Ok(b"{\"delivery\":\"confirmed\"}") => DeliveryStatus::Confirmed,
            Ok(b"{\"delivery\":\"accepted\"}") => DeliveryStatus::Accepted,
            _ => DeliveryStatus::Unknown,
        }
    }
    fn observe_after(
        &mut self,
        action: &Action,
        _: &Expectation,
        _: &ClockReading,
        _: u64,
    ) -> Result<Snapshot, Issue> {
        match self.document(3).map(|d| d.artifact) {
            Ok(Artifact::Snapshot(snapshot)) => Ok(*snapshot),
            Ok(Artifact::Error(issue)) => Err(*issue),
            _ => Err(issue(action)),
        }
    }
}
pub(super) fn run(
    session: &mut CanonicalSession<'_>,
    io: &WorkerIo,
    publication: &mut [u8],
    control: Control,
    input: &[u8],
) -> Result<u64, HostError> {
    let mut provider = Provider {
        io,
        control,
        bytes: vec![0; publication.len()],
    };
    if control.class == OperationClass::Prepare {
        let (snapshot, request, _, _) = session.prepare_input(input)?;
        worker_main::admit_action(io, control, request.limits.deadline_ms)?;
        let Operation::Prepare { action } = request.operation else {
            return Err(HostError::InvalidInput);
        };
        match provider.document(1)?.artifact {
            Artifact::Action(case) => {
                let fresh = &case.action;
                if fresh.id != action.id
                    || fresh.intent != action.intent
                    || fresh.modality != action.modality
                    || fresh.backend_ref.key != action.backend_ref.key
                    || fresh.backend_ref.surface != action.backend_ref.surface
                    || !uiblueprint_schema::validation::contexts_compatible(
                        &snapshot.context,
                        &case.snapshot.context,
                    )
                {
                    return Err(HostError::InvalidInput);
                }
                crate::worker_action::publish_prepared(
                    io,
                    publication,
                    control,
                    *case,
                    request.limits.max_output_bytes as usize,
                )
            }
            Artifact::Error(issue) => crate::worker_action::publish_issue(
                io,
                publication,
                control,
                *issue,
                HostError::ActionRefused,
                request.limits.max_output_bytes as usize,
            ),
            _ => Err(HostError::InvalidInput),
        }
    } else {
        let (case, limits, target, clock, expected) = session.action_input(input)?;
        let deadline = worker_main::admit_action(io, control, limits.deadline_ms)?;
        crate::worker_action::ActionOperation {
            io,
            publication,
            control,
            target,
            clock,
            origin: worker_main::clock_origin(),
            deadline,
        }
        .execute(case, limits, &mut provider, expected)
    }
}
