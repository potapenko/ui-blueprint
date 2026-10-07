use super::{ErrorKind, Failure};
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Usage {
    pub slots: usize,
    pub reserved_bytes: usize,
}
pub(super) struct Pool {
    max_slots: usize,
    max_bytes: usize,
    used: Mutex<Usage>,
}
impl Pool {
    pub fn new(max_slots: usize, max_bytes: usize) -> Arc<Self> {
        Arc::new(Self {
            max_slots,
            max_bytes,
            used: Mutex::new(Usage::default()),
        })
    }
    pub fn usage(&self) -> Usage {
        *self.used.lock().unwrap_or_else(|p| p.into_inner())
    }
    pub fn reserve(self: &Arc<Self>, bytes: usize) -> Result<Permit, Failure> {
        let mut used = self.used.lock().unwrap_or_else(|p| p.into_inner());
        let slots = used
            .slots
            .checked_add(1)
            .ok_or(Failure::plain(ErrorKind::Budget))?;
        let total = used
            .reserved_bytes
            .checked_add(bytes)
            .ok_or(Failure::plain(ErrorKind::Budget))?;
        if slots > self.max_slots || total > self.max_bytes {
            return Err(Failure::plain(ErrorKind::Budget));
        }
        *used = Usage {
            slots,
            reserved_bytes: total,
        };
        Ok(Permit {
            pool: self.clone(),
            bytes,
        })
    }
}
// Store after payload fields so the allowance releases only after those fields drop.
pub(super) struct Permit {
    pool: Arc<Pool>,
    bytes: usize,
}
impl Drop for Permit {
    fn drop(&mut self) {
        let mut used = self.pool.used.lock().unwrap_or_else(|p| p.into_inner());
        used.slots -= 1;
        used.reserved_bytes -= self.bytes;
    }
}
