//! Fixed private IPC control, never a public graph/completion JSON format.
use crate::HostError;
pub const CONTROL_BYTES: usize = 64;
const MAGIC: [u8; 8] = *b"UIBHST01";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OperationClass {
    Attach = 1,
    Validate = 2,
    Measure = 3,
    Check = 4,
    Verify = 5,
    Retain = 6,
    Replay = 7,
    Observe = 8,
    Mutation = 9,
}
impl TryFrom<u8> for OperationClass {
    type Error = HostError;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            1 => Self::Attach,
            2 => Self::Validate,
            3 => Self::Measure,
            4 => Self::Check,
            5 => Self::Verify,
            6 => Self::Retain,
            7 => Self::Replay,
            8 => Self::Observe,
            9 => Self::Mutation,
            _ => return Err(HostError::InvalidControl),
        })
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ControlKind {
    Configure = 1,
    Ready = 2,
    Submit = 3,
    Frame = 4,
    Commit = 5,
    Ack = 6,
    Terminal = 7,
    Cancel = 8,
    EffectReady = 9,
    EffectPermit = 10,
    Fatal = 11,
    Shutdown = 12,
}
impl TryFrom<u8> for ControlKind {
    type Error = HostError;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            1 => Self::Configure,
            2 => Self::Ready,
            3 => Self::Submit,
            4 => Self::Frame,
            5 => Self::Commit,
            6 => Self::Ack,
            7 => Self::Terminal,
            8 => Self::Cancel,
            9 => Self::EffectReady,
            10 => Self::EffectPermit,
            11 => Self::Fatal,
            12 => Self::Shutdown,
            _ => return Err(HostError::InvalidControl),
        })
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Correlation {
    pub session_epoch: u64,
    pub operation: u64,
}
/// Three numeric auxiliaries have message-specific meanings (length/duration,
/// quota/fatal reason, sequence/nonce). No string/Vec/JSON owner is hidden here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Control {
    pub kind: ControlKind,
    pub class: OperationClass,
    pub slot: u8,
    pub flags: u8,
    pub correlation: Correlation,
    pub length: u64,
    pub value: u64,
    pub auxiliary: u64,
}
impl Control {
    pub fn encode(self) -> [u8; CONTROL_BYTES] {
        let mut bytes = [0; CONTROL_BYTES];
        bytes[..8].copy_from_slice(&MAGIC);
        bytes[8] = self.kind as u8;
        bytes[9] = self.class as u8;
        bytes[10] = self.slot;
        bytes[11] = self.flags;
        for (index, value) in [
            self.correlation.session_epoch,
            self.correlation.operation,
            self.length,
            self.value,
            self.auxiliary,
        ]
        .into_iter()
        .enumerate()
        {
            bytes[16 + index * 8..24 + index * 8].copy_from_slice(&value.to_le_bytes());
        }
        bytes
    }
    pub fn decode(bytes: &[u8; CONTROL_BYTES]) -> Result<Self, HostError> {
        if bytes[..8] != MAGIC
            || bytes[12..16].iter().chain(&bytes[56..]).any(|b| *b != 0)
            || bytes[10] >= 3
        {
            return Err(HostError::InvalidControl);
        }
        let mut values = [0_u64; 5];
        for (index, value) in values.iter_mut().enumerate() {
            let mut field = [0; 8];
            field.copy_from_slice(&bytes[16 + index * 8..24 + index * 8]);
            *value = u64::from_le_bytes(field);
        }
        Ok(Self {
            kind: bytes[8].try_into()?,
            class: bytes[9].try_into()?,
            slot: bytes[10],
            flags: bytes[11],
            correlation: Correlation {
                session_epoch: values[0],
                operation: values[1],
            },
            length: values[2],
            value: values[3],
            auxiliary: values[4],
        })
    }
    /// Bodies remain opaque. Correlation/slot/length match is necessary but not a
    /// publication until the matching Commit is received by the supervisor.
    pub fn matches(self, other: Self) -> bool {
        self.class == other.class
            && self.slot == other.slot
            && self.correlation == other.correlation
            && self.length == other.length
    }
}
