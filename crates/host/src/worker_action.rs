//! Guarded canonical single-step composition over the existing effect/output owners.
use crate::{
    worker_effect::{WorkerActionControl, WorkerEffectGate},
    worker_io::WorkerIo,
};
use std::time::Instant;
use uiblueprint_host::{authority::TargetLease, *};
use uiblueprint_plugin_api::actions::ActionControl;
use uiblueprint_schema::{SchemaVersion, model::*};

/// Existing operation/output owners, never a serialized input or new graph.
pub(super) struct ActionOperation<'a> {
    pub io: &'a mut WorkerIo,
    pub publication: &'a mut [u8],
    pub control: Control,
    pub target: TargetLease,
    pub clock: Id,
    pub origin: Instant,
    pub deadline: Instant,
}
impl ActionOperation<'_> {
    pub(super) fn execute(
        self,
        case: ActionCase,
        limits: Limits,
        provider: &mut impl uiblueprint_plugin_api::actions::SetCheckedProvider,
    ) -> Result<u64, HostError> {
        use uiblueprint_plugin_api::actions::SetCheckedExecution;
        let mut clock = WorkerActionControl::new(self.clock.clone(), self.origin);
        let start = clock.now();
        let remaining = u64::try_from(
            self.deadline
                .saturating_duration_since(Instant::now())
                .as_millis(),
        )
        .map_err(|_| HostError::Overflow)?;
        if remaining == 0 {
            return Err(HostError::DeadlineExpired);
        }
        let scope = case.action.authorized_scope.clone();
        let mut kernel = match SetCheckedExecution::prepare(
            case,
            Id(format!(
                "action-transition:{}:{}",
                self.control.correlation.session_epoch, self.control.correlation.operation
            )),
            Id(format!(
                "action-step:{}:{}",
                self.control.correlation.session_epoch, self.control.correlation.operation
            )),
            start,
            remaining,
        ) {
            Ok(kernel) => kernel,
            Err(_) => {
                return publish_refusal(
                    self.io,
                    self.publication,
                    self.control,
                    scope,
                    HostError::InvalidInput,
                    limits.max_output_bytes as usize,
                );
            }
        };
        let mut gate = WorkerEffectGate::new(
            self.io,
            self.control,
            self.target,
            self.clock,
            self.deadline,
        )?;
        let dispatched = kernel.dispatch(provider, &mut gate, &mut clock);
        let verification = if kernel.step().outcome == Outcome::PendingVerification {
            kernel.verify(provider, &mut clock).ok()
        } else {
            None
        };
        let delivery = kernel.step().delivery;
        let outcome = kernel.step().outcome;
        let issue = kernel.issue().cloned();
        let requested = gate.requested();
        let nonce = gate.nonce();
        drop(gate);
        let case = kernel.finish().map_err(|_| HostError::InvalidInput)?;
        let flags = if !requested {
            1
        } else if delivery == DeliveryStatus::Confirmed
            && outcome == Outcome::Succeeded
            && verification == Some(CheckStatus::Pass)
        {
            2
        } else if delivery == DeliveryStatus::Confirmed
            && outcome == Outcome::Failed
            && verification == Some(CheckStatus::Fail)
        {
            3
        } else {
            4
        };
        let document = Document {
            schema_version: SchemaVersion::CURRENT,
            artifact: Artifact::TransitionContext(Box::new(case)),
        };
        // Kernel already validated this canonical record. Encoding and fixed
        // control publication use the existing reserved allowance/ACK boundary.
        encode_publish(
            self.io,
            self.publication,
            self.control,
            &document,
            flags,
            limits.max_output_bytes as usize,
        )?;
        if !requested {
            return Err(issue
                .as_ref()
                .map(issue_error)
                .unwrap_or(HostError::InvalidInput));
        }
        if delivery == DeliveryStatus::Confirmed {
            return nonce.ok_or(HostError::InvalidControl);
        }
        Err(match dispatched {
            Err(uiblueprint_plugin_api::Error::DeadlineExpired) => HostError::DeadlineExpired,
            _ => HostError::WorkerFailed,
        })
    }
}
fn issue_error(issue: &Issue) -> HostError {
    match issue.code {
        ErrorCode::PermissionRequired => HostError::PermissionDenied,
        ErrorCode::StaleTarget
        | ErrorCode::TargetUnresolved
        | ErrorCode::AmbiguousTarget
        | ErrorCode::ResyncRequired => HostError::ResyncRequired,
        ErrorCode::Timeout => HostError::DeadlineExpired,
        _ => HostError::InvalidInput,
    }
}
pub(super) fn publish_refusal(
    io: &mut WorkerIo,
    publication: &mut [u8],
    control: Control,
    scope: Id,
    error: HostError,
    limit: usize,
) -> Result<u64, HostError> {
    let code = match error {
        HostError::PermissionDenied => ErrorCode::PermissionRequired,
        HostError::DeadlineExpired => ErrorCode::Timeout,
        HostError::ResyncRequired => ErrorCode::ResyncRequired,
        _ => ErrorCode::Unsupported,
    };
    let document = Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Error(Box::new(Issue {
            code,
            scope_id: scope,
            failed_step: Some(Id("action-input".into())),
            recovery_class: Id("review_action_input".into()),
        })),
    };
    document.validate().map_err(|_| HostError::InvalidInput)?;
    encode_publish(
        io,
        publication,
        control,
        &document,
        u8::from(control.class == OperationClass::Mutation),
        limit,
    )?;
    Err(error)
}
pub(super) fn publish_prepared(
    io: &mut WorkerIo,
    publication: &mut [u8],
    control: Control,
    case: ActionCase,
    limit: usize,
) -> Result<u64, HostError> {
    if control.class != OperationClass::Prepare {
        return Err(HostError::InvalidControl);
    }
    let document = Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Action(Box::new(case)),
    };
    document.validate().map_err(|_| HostError::InvalidInput)?;
    encode_publish(io, publication, control, &document, 2, limit)?;
    Ok(0)
}
fn encode_publish(
    io: &mut WorkerIo,
    publication: &mut [u8],
    control: Control,
    document: &Document,
    flags: u8,
    limit: usize,
) -> Result<(), HostError> {
    let cap = (control.auxiliary as u32 as usize)
        .min((control.auxiliary >> 32) as usize)
        .min(publication.len())
        .min(limit);
    if cap == 0 {
        return Err(HostError::ResourceLimit);
    }
    let mut output = crate::worker_main::FixedOutput {
        bytes: &mut publication[..cap],
        used: 0,
    };
    {
        let _reserve = crate::quota_allocator::PublicationGuard::enter(
            crate::quota_allocator::Phase::Validate,
        );
        serde_json::to_writer(&mut output, document).map_err(|_| HostError::ResourceLimit)?;
    }
    {
        let _reserve = crate::quota_allocator::PublicationGuard::enter(
            crate::quota_allocator::Phase::Publication,
        );
        crate::worker_main::publish(io, control, 0, &output.bytes[..output.used], flags)?;
    }
    Ok(())
}
