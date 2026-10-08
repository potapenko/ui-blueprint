//! Parent-authorized Native executable/configuration. This is opaque trusted
//! setup, not UI data or a second graph schema. The Native owner validates actual
//! PID/incarnation/Surface/scope/artifact binding before constructing it.
use crate::{CONTROL_BYTES, HostError, process_api::SpawnSpec};
pub const NATIVE_CONFIG_BYTES: usize = 4096 - CONTROL_BYTES;
pub struct NativeHelperBinding {
    pub(crate) executable: SpawnSpec,
    configuration: [u8; NATIVE_CONFIG_BYTES],
    length: usize,
    pub(crate) channels: u8,
    pub(crate) residency: Option<std::time::Instant>,
}
impl NativeHelperBinding {
    /// Existing Native AX/capture/probe channels are selectable. The caller owns
    /// actual authority and the configured output destination; helper/UI payloads
    /// cannot construct or replace this parent-owned binding.
    pub fn authorized(
        executable: SpawnSpec,
        channels: u8,
        configuration: &[u8],
    ) -> Result<Self, HostError> {
        if channels == 0
            || channels & !7 != 0
            || configuration.is_empty()
            || configuration.len() > NATIVE_CONFIG_BYTES
        {
            return Err(HostError::InvalidInput);
        }
        let mut result = Self {
            executable,
            configuration: [0; NATIVE_CONFIG_BYTES],
            length: configuration.len(),
            channels,
            residency: None,
        };
        result.configuration[..configuration.len()].copy_from_slice(configuration);
        Ok(result)
    }
    /// Explicit bounded form session; no helper survives this deadline or detach.
    pub fn form_session(
        executable: SpawnSpec,
        configuration: &[u8],
        duration_ms: u64,
    ) -> Result<Self, HostError> {
        if duration_ms == 0 || duration_ms > 300_000 {
            return Err(HostError::InvalidInput);
        }
        let mut binding = Self::authorized(executable, 1, configuration)?;
        binding.residency = Some(
            std::time::Instant::now()
                .checked_add(std::time::Duration::from_millis(duration_ms))
                .ok_or(HostError::Overflow)?,
        );
        Ok(binding)
    }
    pub(crate) fn bytes(&self) -> &[u8] {
        &self.configuration[..self.length]
    }
}
