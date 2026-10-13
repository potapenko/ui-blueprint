//! Fixed caller/validated Host facts only. No source text or graph parsing.
use serde::Serialize;
#[derive(Clone, Copy, Debug, Serialize)]
pub(crate) struct ObserveDiagnostic {
    output_version: &'static str,
    kind: &'static str,
    pub exit_code: u8,
    pub caller_phase: &'static str,
    pub caller_error: Option<&'static str>,
    operation: Option<&'static str>,
    terminal: Option<&'static str>,
    terminal_error: Option<&'static str>,
    pub requested_channels: Option<u8>,
    committed_channels: Option<u8>,
    missing_channels: Option<u8>,
    incomplete_channels: Option<u8>,
    pub worker_cleanup: &'static str,
    host_diagnostic: Option<HostDiagnostic>,
}
#[derive(Clone, Copy, Debug, Serialize)]
struct HostDiagnostic {
    stage: &'static str,
    cause: &'static str,
    code: i32,
    selection_visited_nodes: Option<u32>,
    remote_cleanup: &'static str,
    send_progress: &'static str,
}
impl ObserveDiagnostic {
    pub fn new() -> Self {
        Self {
            output_version: "1.0.0",
            kind: "observe_diagnostic",
            exit_code: 0,
            caller_phase: "preflight",
            caller_error: None,
            operation: None,
            terminal: None,
            terminal_error: None,
            requested_channels: None,
            committed_channels: None,
            missing_channels: None,
            incomplete_channels: None,
            worker_cleanup: "not_started",
            host_diagnostic: None,
        }
    }
}
#[cfg(all(target_os = "macos", any(feature = "macos", feature = "web")))]
impl ObserveDiagnostic {
    pub fn capture(&mut self, completion: &uiblueprint_host::host_types::HostCompletion<'_>) {
        use uiblueprint_host::{OperationClass, host_types::Terminal};
        let operation = match completion.class {
            OperationClass::Observe => "observe",
            OperationClass::Attach => "attach",
            _ => return,
        };
        if self.operation == Some("observe") && operation != "observe" {
            return;
        }
        self.operation = Some(operation);
        self.terminal = Some(match completion.terminal {
            Terminal::Completed => "completed",
            Terminal::Failed(_) => "failed",
            Terminal::Cancelled => "cancelled",
            Terminal::TimedOut => "timed_out",
        });
        self.terminal_error = if let Terminal::Failed(e) = completion.terminal {
            Some(host_error_name(e))
        } else {
            None
        };
        // Zero getters with no publication represent unavailable masks, not success.
        if completion.class == OperationClass::Observe
            && completion.committed() | completion.missing() != 0
        {
            self.committed_channels = Some(completion.committed());
            self.missing_channels = Some(completion.missing());
            self.incomplete_channels = Some(completion.incomplete_channels());
        }
        self.host_diagnostic = completion.diagnostic().map(HostDiagnostic::from_record);
    }
}
#[cfg(all(target_os = "macos", any(feature = "macos", feature = "web")))]
fn host_error_name(error: uiblueprint_host::HostError) -> &'static str {
    use uiblueprint_host::HostError as E;
    match error {
        E::InvalidLimits => "invalid_limits",
        E::ResourceLimit => "resource_limit",
        E::AllocationFailure => "allocation_failure",
        E::Overflow => "overflow",
        E::Busy => "busy",
        E::InvalidInput => "invalid_input",
        E::InvalidState => "invalid_state",
        E::InvalidControl => "invalid_control",
        E::StaleOperation => "stale_operation",
        E::DeadlineExpired => "deadline_expired",
        E::PermissionDenied => "permission_denied",
        E::Io => "io",
        E::WorkerFailed => "worker_failed",
        E::SystemAllocationFailure => "system_allocation_failure",
        E::CleanupPending => "cleanup_pending",
        E::ResyncRequired => "resync_required",
        E::ActionRefused => "action_refused",
    }
}
#[cfg(all(target_os = "macos", any(feature = "macos", feature = "web")))]
impl HostDiagnostic {
    fn from_record(d: uiblueprint_host::diagnostic::DiagnosticRecord) -> Self {
        use uiblueprint_host::diagnostic::{DiagnosticCause as Cause, DiagnosticStage as Stage};
        let stage = match d.stage {
            Stage::Decode => "decode",
            Stage::Begin => "begin",
            Stage::Permit => "permit",
            Stage::Capacity => "capacity",
            Stage::Collect => "collect",
            Stage::Encode => "encode",
            Stage::Receive => "receive",
            Stage::Publish => "publish",
            Stage::Finish => "finish",
        };
        let cause = match d.cause {
            Cause::Host => "host",
            Cause::CollectorInvalidInput => "collector_invalid_input",
            Cause::CollectorLimit => "collector_limit",
            Cause::CollectorStaleTarget => "collector_stale_target",
            Cause::CollectorResyncRequired => "collector_resync_required",
            Cause::CollectorMalformed => "collector_malformed",
            Cause::CollectorProtocol => "collector_protocol",
            Cause::CollectorTimeout => "collector_timeout",
            Cause::CollectorPublicationStopped => "collector_publication_stopped",
            Cause::CollectorCleanupUnconfirmed => "collector_cleanup_unconfirmed",
            Cause::SelectionMissing => "selection_missing",
            Cause::SelectionAmbiguous => "selection_ambiguous",
            Cause::SelectionIncomplete => "selection_incomplete",
            Cause::SelectionUnsupported => "selection_unsupported",
            Cause::SelectionTimedOut => "selection_timed_out",
            Cause::CdpInvalidLimits => "cdp_invalid_limits",
            Cause::CdpInvalidBinding => "cdp_invalid_binding",
            Cause::CdpEncoding => "cdp_encoding",
            Cause::CdpEnvelope => "cdp_envelope",
            Cause::CdpBudget => "cdp_budget",
            Cause::CdpDetached => "cdp_detached",
            Cause::CdpBusy => "cdp_busy",
            Cause::CdpCounterExhausted => "cdp_counter_exhausted",
            Cause::CdpWrongSession => "cdp_wrong_session",
            Cause::CdpUnexpectedReply => "cdp_unexpected_reply",
            Cause::CdpUncorrelatedError => "cdp_uncorrelated_error",
            Cause::CdpCancelled => "cdp_cancelled",
            Cause::TransportInvalidLimits => "transport_invalid_limits",
            Cause::TransportEndpointRejected => "transport_endpoint_rejected",
            Cause::TransportLoggingBoundary => "transport_logging_boundary",
            Cause::TransportConnect => "transport_connect",
            Cause::TransportHandshake => "transport_handshake",
            Cause::TransportExtensions => "transport_extensions",
            Cause::TransportTimeout => "transport_timeout",
            Cause::TransportCancelled => "transport_cancelled",
            Cause::TransportByteLimit => "transport_byte_limit",
            Cause::TransportWorkLimit => "transport_work_limit",
            Cause::TransportIo => "transport_io",
            Cause::TransportProtocol => "transport_protocol",
            Cause::TransportCapacity => "transport_capacity",
            Cause::TransportBinary => "transport_binary",
            Cause::TransportClosed => "transport_closed",
            Cause::TransportPendingWrite => "transport_pending_write",
            Cause::TransportNoPendingWrite => "transport_no_pending_write",
        };
        let selection = matches!(
            d.cause,
            Cause::SelectionMissing
                | Cause::SelectionAmbiguous
                | Cause::SelectionIncomplete
                | Cause::SelectionUnsupported
                | Cause::SelectionTimedOut
        );
        Self {
            stage,
            cause,
            code: d.code,
            selection_visited_nodes: if selection { Some(d.count) } else { None },
            remote_cleanup: match d.remote_cleanup {
                0 => "not_required",
                1 => "released",
                _ => "unconfirmed",
            },
            send_progress: match d.send_progress {
                0 => "not_queued",
                1 => "queued",
                2 => "possibly_written",
                _ => "flushed",
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unavailable_record_and_full_report_have_no_source_text() {
        let mut report = ObserveDiagnostic::new();
        report.exit_code = 4;
        report.caller_error = Some("observe_incomplete");
        let data = serde_json::to_value(report).unwrap();
        assert!(data["host_diagnostic"].is_null());
        assert!(data["terminal"].is_null());
        assert!(data["committed_channels"].is_null());
        let bytes = crate::output::observe_diagnostic(&report, 1024).unwrap();
        assert!(bytes.len() <= 1024);
        assert!(bytes.ends_with(b"\n"));
        assert!(crate::output::observe_diagnostic(&report, bytes.len() - 1).is_err());
    }
    #[cfg(all(target_os = "macos", any(feature = "macos", feature = "web")))]
    #[test]
    fn validated_host_variants_counts_and_worst_case_fit_the_fixed_minimum() {
        use uiblueprint_host::{
            Control, ControlKind, Correlation, OperationClass,
            diagnostic::{DiagnosticCause as C, DiagnosticRecord, DiagnosticStage},
        };
        for cause in [
            C::Host,
            C::SelectionIncomplete,
            C::CollectorMalformed,
            C::CollectorProtocol,
            C::CdpUncorrelatedError,
            C::TransportLoggingBoundary,
            C::CollectorCleanupUnconfirmed,
        ] {
            let code = match cause {
                C::Host => 16,
                C::CollectorMalformed => 255,
                C::CollectorProtocol | C::CdpUncorrelatedError => i32::MIN,
                _ => 0,
            };
            let record = DiagnosticRecord {
                stage: DiagnosticStage::Collect,
                cause,
                remote_cleanup: 2,
                send_progress: 3,
                code,
                count: if cause == C::SelectionIncomplete {
                    u32::MAX
                } else {
                    0
                },
            };
            let mut terminal = Control {
                kind: ControlKind::Terminal,
                class: OperationClass::Observe,
                slot: 0,
                flags: 0,
                correlation: Correlation {
                    session_epoch: 1,
                    operation: 1,
                },
                length: 0,
                value: 5,
                auxiliary: 0,
            };
            record.apply(&mut terminal).unwrap();
            let mut report = ObserveDiagnostic::new();
            report.exit_code = 4;
            report.caller_phase = "configure_helpers";
            report.caller_error = Some("unsupported_observe_platform_or_feature");
            report.operation = Some("observe");
            report.terminal = Some("failed");
            report.terminal_error = Some("system_allocation_failure");
            report.worker_cleanup = "unconfirmed";
            report.requested_channels = Some(7);
            report.committed_channels = Some(7);
            report.missing_channels = Some(7);
            report.incomplete_channels = Some(7);
            report.host_diagnostic = Some(HostDiagnostic::from_record(record));
            let bytes = crate::output::observe_diagnostic(&report, 1024).unwrap();
            assert!(bytes.len() <= 1024);
            let d: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(d["host_diagnostic"]["code"], code);
            assert_eq!(d["host_diagnostic"]["remote_cleanup"], "unconfirmed");
            assert_eq!(d["host_diagnostic"]["send_progress"], "flushed");
            assert_eq!(
                d["host_diagnostic"]["selection_visited_nodes"],
                if cause == C::SelectionIncomplete {
                    serde_json::json!(u32::MAX)
                } else {
                    serde_json::Value::Null
                }
            );
        }
    }
}
