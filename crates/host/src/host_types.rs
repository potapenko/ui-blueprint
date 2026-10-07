use crate::{
    authority::{FixedId, TargetLease},
    domain::{SessionHandle, SessionReservation},
    publication::CommittedFrames,
    *,
};
use std::{
    io::{self, Write},
    time::Instant,
};

/// Caller fills already charged bytes; no encoded Vec is copied into the parent.
pub struct InputLease<'a> {
    pub(crate) bytes: ByteLease<'a>,
    pub(crate) session: SessionHandle<'a>,
}
impl InputLease<'_> {
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        self.bytes.as_mut_slice()
    }
    pub fn bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}
pub struct AttachInput<'a> {
    pub(crate) input: InputLease<'a>,
    pub(crate) reservation: SessionReservation<'a>,
    pub(crate) target: TargetLease,
}
impl AttachInput<'_> {
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        self.input.bytes_mut()
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OperationHandle<'a> {
    pub session: SessionHandle<'a>,
    pub sequence: u64,
}
#[derive(Clone, Copy, Debug)]
pub struct OutputRequest {
    pub channels: u8,
    pub frame_bytes: usize,
    pub total_bytes: usize,
    pub input_format: u8,
    pub retained_partition: u8,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Terminal {
    Completed,
    Failed(HostError),
    Cancelled,
    TimedOut,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EffectReceipt {
    NotDispatched,
    Possible { nonce: u64 },
    Confirmed { nonce: u64 },
}
pub struct HostCompletion<'a> {
    pub operation: OperationHandle<'a>,
    pub class: OperationClass,
    pub terminal: Terminal,
    pub effect: EffectReceipt,
    pub(crate) diagnostic: Option<crate::diagnostic::DiagnosticRecord>,
    pub(crate) frames: Option<CommittedFrames<'a>>,
}
impl HostCompletion<'_> {
    /// Private bounded producer failure metadata; absent on success or when no
    /// diagnostic terminal arrived (for example parent timeout or worker death).
    pub fn diagnostic(&self) -> Option<crate::diagnostic::DiagnosticRecord> {
        self.diagnostic
    }
    pub fn committed(&self) -> u8 {
        self.frames.as_ref().map_or(0, |f| f.committed)
    }
    pub fn missing(&self) -> u8 {
        self.frames.as_ref().map_or(0, |f| f.missing)
    }
    /// ACKed live channels whose validated canonical result was Failed or whose
    /// Snapshot declared incomplete coverage. No parent payload parsing occurs.
    pub fn incomplete_channels(&self) -> u8 {
        self.frames.as_ref().map_or(0, |f| f.incomplete)
    }
    /// Producer outcome bound to a complete matching ACK; absent for legacy or
    /// uncommitted bytes. Verified metadata alone does not prove terminal/cleanup.
    pub fn action_status(&self) -> Option<crate::publication::ActionPublicationStatus> {
        self.frames.as_ref().and_then(|f| f.action_status)
    }
    pub fn bytes(&self, slot: usize) -> Option<&[u8]> {
        self.frames.as_ref().and_then(|f| f.frame(slot))
    }
    pub fn effect_unknown(&self) -> bool {
        matches!(self.effect, EffectReceipt::Possible { .. })
    }
    pub fn write_channel(&self, slot: usize, output: &mut impl Write) -> io::Result<()> {
        output.write_all(self.bytes(slot).ok_or(io::ErrorKind::NotFound)?)
    }
}
// Fixed inline identity is intentional: steady-state parent events must not
// introduce a Box allocation outside the precharged control inventory.
#[allow(clippy::large_enum_variant)]
pub enum HostEvent<'a> {
    Pending,
    ShutdownComplete,
    HelperClosed {
        helper: crate::helpers::HelperHandle<'a>,
    },
    HelperCleanupPending {
        helper: crate::helpers::HelperHandle<'a>,
    },
    Attached {
        session: SessionHandle<'a>,
        clock: FixedId,
    },
    Complete(HostCompletion<'a>),
    CleanupPending {
        session: SessionHandle<'a>,
    },
    Closed {
        session: SessionHandle<'a>,
    },
}
/// Fixed-stack clock-domain construction; no parent format!/String allocation.
pub fn clock_id(epoch: u64) -> Result<FixedId, HostError> {
    struct Text {
        bytes: [u8; 64],
        len: usize,
    }
    impl std::fmt::Write for Text {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            let end = self.len.checked_add(s.len()).ok_or(std::fmt::Error)?;
            if end > self.bytes.len() {
                return Err(std::fmt::Error);
            }
            self.bytes[self.len..end].copy_from_slice(s.as_bytes());
            self.len = end;
            Ok(())
        }
    }
    use std::fmt::Write as _;
    let mut text = Text {
        bytes: [0; 64],
        len: 0,
    };
    write!(&mut text, "uib-worker-{epoch}-clock").map_err(|_| HostError::Overflow)?;
    FixedId::new(std::str::from_utf8(&text.bytes[..text.len]).map_err(|_| HostError::InvalidState)?)
}
pub(crate) fn remaining_ms(deadline: Instant, now: Instant) -> Result<u64, HostError> {
    if now >= deadline {
        return Err(HostError::DeadlineExpired);
    }
    u64::try_from(deadline.duration_since(now).as_millis()).map_err(|_| HostError::Overflow)
}
