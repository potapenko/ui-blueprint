//! Fixed private failure metadata, never canonical data or a public CLI schema.
//! No remote text, endpoint, identifier or payload can enter this record.
use crate::{Control, ControlKind, HostError, OperationClass};

macro_rules! codes {
    ($name:ident { $($variant:ident = $code:literal),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum $name { $($variant = $code),+ }
        impl TryFrom<u8> for $name {
            type Error = HostError;
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value { $($code => Ok(Self::$variant),)+ _ => Err(HostError::InvalidControl) }
            }
        }
    };
}
codes!(DiagnosticStage {
    Decode = 1, Begin = 2, Permit = 3, Capacity = 4, Collect = 5,
    Encode = 6, Receive = 7, Publish = 8, Finish = 9,
});
codes!(DiagnosticCause {
    Host = 1,
    CollectorInvalidInput = 2, CollectorLimit = 3, CollectorStaleTarget = 4,
    CollectorResyncRequired = 5, CollectorMalformed = 6, CollectorProtocol = 7,
    CollectorTimeout = 8, CollectorPublicationStopped = 9, CollectorCleanupUnconfirmed = 10,
    SelectionMissing = 11, SelectionAmbiguous = 12, SelectionIncomplete = 13,
    SelectionUnsupported = 14, SelectionTimedOut = 15,
    CdpInvalidLimits = 16, CdpInvalidBinding = 17, CdpEncoding = 18, CdpEnvelope = 19,
    CdpBudget = 20, CdpDetached = 21, CdpBusy = 22, CdpCounterExhausted = 23,
    CdpWrongSession = 24, CdpUnexpectedReply = 25, CdpUncorrelatedError = 26, CdpCancelled = 27,
    TransportInvalidLimits = 28, TransportEndpointRejected = 29, TransportLoggingBoundary = 30,
    TransportConnect = 31, TransportHandshake = 32, TransportExtensions = 33,
    TransportTimeout = 34, TransportCancelled = 35, TransportByteLimit = 36,
    TransportWorkLimit = 37, TransportIo = 38, TransportProtocol = 39,
    TransportCapacity = 40, TransportBinary = 41, TransportClosed = 42,
    TransportPendingWrite = 43, TransportNoPendingWrite = 44,
});

/// Twelve inline bytes, copied through the existing 64-byte terminal control.
/// Numeric codes are private to this paired host/worker, not a wire-version promise.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiagnosticRecord {
    pub stage: DiagnosticStage,
    pub cause: DiagnosticCause,
    /// Existing RemoteCleanup: NotRequired=0, Released=1, Unconfirmed=2.
    pub remote_cleanup: u8,
    /// Existing SendProgress: NotQueued=0, Queued=1, PossiblyWritten=2, Flushed=3.
    pub send_progress: u8,
    /// Existing protocol error number, fixed HostError code from `host`, or a
    /// trusted CollectorMalformed branch code 0..=255 (zero means unspecified).
    pub code: i32,
    /// Existing Selection.visited_nodes; zero for non-selection causes.
    pub count: u32,
}
impl DiagnosticRecord {
    pub fn host(stage: DiagnosticStage, error: HostError) -> Self {
        let code = match error {
            HostError::InvalidLimits => 1,
            HostError::ResourceLimit => 2,
            HostError::AllocationFailure => 3,
            HostError::Overflow => 4,
            HostError::Busy => 5,
            HostError::InvalidInput => 6,
            HostError::InvalidState => 7,
            HostError::InvalidControl => 8,
            HostError::StaleOperation => 9,
            HostError::DeadlineExpired => 10,
            HostError::PermissionDenied => 11,
            HostError::Io => 12,
            HostError::WorkerFailed => 13,
            HostError::SystemAllocationFailure => 14,
            HostError::CleanupPending => 15,
            HostError::ResyncRequired => 16,
            // This record is Observe-only; retain its existing coarse code set.
            // Typed action terminals carry their own refusal category instead.
            HostError::ActionRefused => 6,
        };
        Self {
            stage,
            cause: DiagnosticCause::Host,
            remote_cleanup: 0,
            send_progress: 0,
            code,
            count: 0,
        }
    }
    fn validate(self) -> Result<(), HostError> {
        use DiagnosticCause::*;
        let selection = matches!(
            self.cause,
            SelectionMissing
                | SelectionAmbiguous
                | SelectionIncomplete
                | SelectionUnsupported
                | SelectionTimedOut
        );
        if self.remote_cleanup > 2
            || self.send_progress > 3
            || (!selection && self.count != 0)
            || match self.cause {
                Host => !(1..=16).contains(&self.code),
                CollectorMalformed => !(0..=255).contains(&self.code),
                CollectorProtocol | CdpUncorrelatedError => false,
                _ => self.code != 0,
            }
        {
            return Err(HostError::InvalidControl);
        }
        Ok(())
    }
    /// Only a failed live Web observe may use these otherwise-unused terminal fields.
    pub fn apply(self, terminal: &mut Control) -> Result<(), HostError> {
        self.validate()?;
        if terminal.kind != ControlKind::Terminal
            || terminal.class != OperationClass::Observe
            || terminal.slot != 0
            || terminal.flags != 0
            || terminal.length != 0
            || terminal.auxiliary != 0
            || !(1..=8).contains(&terminal.value)
        {
            return Err(HostError::InvalidControl);
        }
        terminal.flags = 1;
        terminal.length = u64::from_le_bytes([
            self.stage as u8,
            self.cause as u8,
            self.remote_cleanup,
            self.send_progress,
            0,
            0,
            0,
            0,
        ]);
        terminal.auxiliary = u64::from(self.code as u32) | (u64::from(self.count) << 32);
        Ok(())
    }
    /// Parent decodes only fixed numbers; it still never parses a canonical body.
    pub(crate) fn from_terminal(
        terminal: Control,
        live_web: bool,
    ) -> Result<Option<Self>, HostError> {
        if terminal.flags == 0 && terminal.length == 0 {
            return Ok(None);
        }
        if !live_web
            || terminal.kind != ControlKind::Terminal
            || terminal.class != OperationClass::Observe
            || terminal.slot != 0
            || terminal.flags != 1
            || !(1..=8).contains(&terminal.value)
        {
            return Err(HostError::InvalidControl);
        }
        let meta = terminal.length.to_le_bytes();
        if meta[4..].iter().any(|v| *v != 0) {
            return Err(HostError::InvalidControl);
        }
        let record = Self {
            stage: meta[0].try_into()?,
            cause: meta[1].try_into()?,
            remote_cleanup: meta[2],
            send_progress: meta[3],
            code: terminal.auxiliary as u32 as i32,
            count: (terminal.auxiliary >> 32) as u32,
        };
        record.validate()?;
        Ok(Some(record))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CONTROL_BYTES, Correlation};

    fn terminal() -> Control {
        Control {
            kind: ControlKind::Terminal,
            class: OperationClass::Observe,
            slot: 0,
            flags: 0,
            correlation: Correlation {
                session_epoch: 7,
                operation: 3,
            },
            length: 0,
            value: 5,
            auxiliary: 0,
        }
    }
    fn record(cause: DiagnosticCause, code: i32, count: u32) -> DiagnosticRecord {
        DiagnosticRecord {
            stage: DiagnosticStage::Collect,
            cause,
            remote_cleanup: 2,
            send_progress: 3,
            code,
            count,
        }
    }
    #[test]
    fn failure_preserves_signed_protocol_codes_counts_and_fixed_ownership() {
        assert_eq!(std::mem::size_of::<DiagnosticRecord>(), 12);
        assert_eq!(std::mem::size_of::<Option<DiagnosticRecord>>(), 12);
        let cases = [
            record(DiagnosticCause::CollectorProtocol, i32::MIN, 0),
            record(DiagnosticCause::CdpUncorrelatedError, i32::MAX, 0),
            record(DiagnosticCause::SelectionIncomplete, 0, u32::MAX),
            record(DiagnosticCause::CollectorMalformed, 0, 0),
            record(DiagnosticCause::CollectorMalformed, 255, 0),
            DiagnosticRecord::host(DiagnosticStage::Receive, HostError::InvalidInput),
        ];
        for expected in cases {
            let original = terminal();
            let mut control = original;
            expected.apply(&mut control).unwrap();
            let bytes = control.encode();
            assert_eq!(bytes.len(), CONTROL_BYTES);
            let decoded = Control::decode(&bytes).unwrap();
            assert_eq!(
                decoded.value, original.value,
                "terminal semantics unchanged"
            );
            assert_eq!(decoded.correlation, original.correlation);
            assert_eq!(
                DiagnosticRecord::from_terminal(decoded, true),
                Ok(Some(expected))
            );
        }
        // Existing terminals, including mutation nonce and retained handle, keep
        // their auxiliary meaning and do not manufacture failure diagnostics.
        let mut legacy = terminal();
        legacy.auxiliary = 9;
        assert_eq!(DiagnosticRecord::from_terminal(legacy, false), Ok(None));
    }
    #[test]
    fn diagnostic_rejects_wrong_operation_success_and_malformed_fixed_fields() {
        let mut valid = terminal();
        record(DiagnosticCause::CollectorMalformed, 0, 0)
            .apply(&mut valid)
            .unwrap();
        assert_eq!(
            DiagnosticRecord::from_terminal(valid, false),
            Err(HostError::InvalidControl)
        );
        for bad in [
            Control { value: 0, ..valid },
            Control {
                class: OperationClass::Mutation,
                ..valid
            },
            Control { flags: 2, ..valid },
            Control {
                length: valid.length | (1 << 32),
                ..valid
            },
            Control {
                length: (valid.length & !255) | 255,
                ..valid
            },
            Control {
                length: (valid.length & !(255 << 8)) | (255 << 8),
                ..valid
            },
            Control {
                length: valid.length | (3 << 16),
                ..valid
            },
            Control {
                length: valid.length | (4 << 24),
                ..valid
            },
            Control {
                auxiliary: 256,
                ..valid
            },
            Control {
                auxiliary: 1 << 32,
                ..valid
            },
        ] {
            assert_eq!(
                DiagnosticRecord::from_terminal(bad, true),
                Err(HostError::InvalidControl)
            );
        }
        for code in [-1, 256] {
            let mut unchanged = terminal();
            assert_eq!(
                record(DiagnosticCause::CollectorMalformed, code, 0).apply(&mut unchanged),
                Err(HostError::InvalidControl)
            );
            assert_eq!(unchanged, terminal());
        }
        let mut other_cause = terminal();
        assert_eq!(
            record(DiagnosticCause::CollectorInvalidInput, 1, 0).apply(&mut other_cause),
            Err(HostError::InvalidControl)
        );
        for mut bad in [
            Control {
                value: 0,
                ..terminal()
            },
            Control {
                class: OperationClass::Measure,
                ..terminal()
            },
        ] {
            let before = bad;
            assert_eq!(
                record(DiagnosticCause::CollectorMalformed, 0, 0).apply(&mut bad),
                Err(HostError::InvalidControl)
            );
            assert_eq!(bad, before, "rejected encoding is non-mutating");
        }
    }
}
