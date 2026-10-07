use crate::{HostError, add, mul};
use uiblueprint_engine::cache::QuotaLedger;
pub const MIB: usize = 1_048_576;
pub const MAX_WORKERS: usize = 4;
pub const MAX_GROUPS: usize = 8;
pub const CHANNELS: usize = 3;
pub const HELPERS: usize = 2;

/// All values are explicit constructor inputs; this type intentionally has no Default.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HostLimits {
    pub workers: usize,
    pub worker_bytes: usize,
    pub publication_reserve: usize,
    pub bootstrap_bytes: usize,
    pub parent_bytes: usize,
    pub input_bytes: usize,
    pub ingress_bytes: usize,
    pub output_bytes: usize,
    pub request_output_bytes: usize,
    pub completion_groups: usize,
    pub control_bytes: usize,
    pub cleanup_ms: u64,
    pub retained_domain_bytes: usize,
    pub retained_per_worker: usize,
    pub main_stack_bytes: usize,
    pub watchdog_stack_bytes: usize,
}
impl HostLimits {
    /// D05 profile validation only. Actual fixed owner inventory is additionally
    /// charged by each constructor before a child can be admitted.
    pub fn validate(self) -> Result<Self, HostError> {
        let bounds = [
            (self.workers, MAX_WORKERS),
            (self.worker_bytes, 64 * MIB),
            (self.publication_reserve, MIB),
            (self.bootstrap_bytes, MIB),
            (self.parent_bytes, 32 * MIB),
            (self.input_bytes, 2 * MIB),
            (self.ingress_bytes, 512 * 1024),
            (self.output_bytes, 512 * 1024),
            (self.request_output_bytes, 2 * MIB),
            (self.completion_groups, MAX_GROUPS),
            (self.control_bytes, 4096),
            (self.retained_domain_bytes, 64 * MIB),
            (self.main_stack_bytes, 8 * MIB),
            (self.watchdog_stack_bytes, MIB),
        ];
        if bounds.iter().any(|(value, max)| *value == 0 || value > max)
            || self.cleanup_ms == 0
            || self.cleanup_ms > 1000
            || self.publication_reserve >= self.worker_bytes
            || self.bootstrap_bytes > self.ordinary_bytes()?
            || self.ordinary_bytes()? > 63 * MIB
            || self.control_bytes < crate::CONTROL_BYTES
            || self.retained_per_worker == 0
            || self.retained_per_worker > (64 * MIB - QuotaLedger::backing_bytes()) / MAX_WORKERS
        {
            return Err(HostError::InvalidLimits);
        }
        if add(
            mul(self.workers, self.retained_per_worker)?,
            QuotaLedger::backing_bytes(),
        )? > self.retained_domain_bytes
        {
            return Err(HostError::InvalidLimits);
        }
        Ok(self)
    }
    pub fn ordinary_bytes(self) -> Result<usize, HostError> {
        self.worker_bytes
            .checked_sub(self.publication_reserve)
            .ok_or(HostError::InvalidLimits)
    }
    pub fn payload_bytes(self) -> Result<usize, HostError> {
        add(
            add(
                mul(self.workers, self.input_bytes)?,
                mul(mul(self.workers, HELPERS)?, self.ingress_bytes)?,
            )?,
            mul(mul(self.completion_groups, CHANNELS)?, self.output_bytes)?,
        )
    }
}
