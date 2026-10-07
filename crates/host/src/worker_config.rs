//! Private fixed-control configuration. Canonical documents never enter this codec.
use crate::{
    HostError, HostLimits, OperationClass,
    authority::{FixedId, TargetLease},
};
const WORDS: usize = 17;
pub const MAX_CONFIG_BYTES: usize = WORDS * 8 + 4 + 2 * crate::authority::ID_BYTES;
pub struct WorkerConfig {
    pub limits: HostLimits,
    pub target: TargetLease,
}
impl WorkerConfig {
    pub fn encode(&self, out: &mut [u8]) -> Result<usize, HostError> {
        let l = self.limits.validate()?;
        let ids = [
            self.target.id().as_bytes(),
            self.target.generation().as_bytes(),
        ];
        let length = WORDS * 8 + 4 + ids[0].len() + ids[1].len();
        if length > out.len()
            || length
                .checked_add(crate::CONTROL_BYTES)
                .is_none_or(|n| n > l.control_bytes)
        {
            return Err(HostError::ResourceLimit);
        }
        let words = [
            l.workers as u64,
            l.worker_bytes as u64,
            l.publication_reserve as u64,
            l.bootstrap_bytes as u64,
            l.parent_bytes as u64,
            l.input_bytes as u64,
            l.ingress_bytes as u64,
            l.output_bytes as u64,
            l.request_output_bytes as u64,
            l.completion_groups as u64,
            l.control_bytes as u64,
            l.cleanup_ms,
            l.retained_domain_bytes as u64,
            l.retained_per_worker as u64,
            l.main_stack_bytes as u64,
            l.watchdog_stack_bytes as u64,
            u64::from(self.target.permits(OperationClass::Mutation)),
        ];
        for (i, word) in words.into_iter().enumerate() {
            out[i * 8..i * 8 + 8].copy_from_slice(&word.to_le_bytes());
        }
        let mut cursor = WORDS * 8;
        for id in ids {
            out[cursor..cursor + 2].copy_from_slice(&(id.len() as u16).to_le_bytes());
            cursor += 2;
            out[cursor..cursor + id.len()].copy_from_slice(id);
            cursor += id.len();
        }
        Ok(cursor)
    }
    pub fn decode(input: &[u8]) -> Result<Self, HostError> {
        if input.len() < WORDS * 8 + 4 || input.len() > MAX_CONFIG_BYTES {
            return Err(HostError::InvalidControl);
        }
        let mut values = [0usize; WORDS];
        for (i, value) in values.iter_mut().enumerate() {
            let mut bytes = [0; 8];
            bytes.copy_from_slice(&input[i * 8..i * 8 + 8]);
            *value = usize::try_from(u64::from_le_bytes(bytes)).map_err(|_| HostError::Overflow)?;
        }
        if values[16] > 1 {
            return Err(HostError::InvalidControl);
        }
        let l = HostLimits {
            workers: values[0],
            worker_bytes: values[1],
            publication_reserve: values[2],
            bootstrap_bytes: values[3],
            parent_bytes: values[4],
            input_bytes: values[5],
            ingress_bytes: values[6],
            output_bytes: values[7],
            request_output_bytes: values[8],
            completion_groups: values[9],
            control_bytes: values[10],
            cleanup_ms: values[11] as u64,
            retained_domain_bytes: values[12],
            retained_per_worker: values[13],
            main_stack_bytes: values[14],
            watchdog_stack_bytes: values[15],
        }
        .validate()?;
        let mut cursor = WORDS * 8;
        let mut read_id = || -> Result<FixedId, HostError> {
            let length_bytes = input
                .get(cursor..cursor + 2)
                .ok_or(HostError::InvalidControl)?;
            let length = u16::from_le_bytes([length_bytes[0], length_bytes[1]]) as usize;
            cursor += 2;
            let value = input
                .get(cursor..cursor + length)
                .ok_or(HostError::InvalidControl)?;
            cursor += length;
            FixedId::new(std::str::from_utf8(value).map_err(|_| HostError::InvalidControl)?)
        };
        let id = read_id()?;
        let generation = read_id()?;
        if cursor != input.len() {
            return Err(HostError::InvalidControl);
        }
        Ok(Self {
            limits: l,
            target: TargetLease::from_parts(id, generation, values[16] != 0),
        })
    }
}
