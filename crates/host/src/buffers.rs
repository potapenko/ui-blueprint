//! Fallibly created fixed byte owners. RefCell borrows individual slots, so a held
//! result lease never borrows the whole mutable host or blocks unrelated slots.
use crate::limits::{CHANNELS, HELPERS, MAX_GROUPS, MAX_WORKERS};
use crate::{HostError, HostLimits, add, mul};
use std::{
    cell::{RefCell, RefMut},
    mem::size_of,
};
const SLOT_COUNT: usize = MAX_WORKERS + MAX_WORKERS * HELPERS + MAX_GROUPS * CHANNELS;
const INPUT_START: usize = 0;
const INGRESS_START: usize = MAX_WORKERS;
const OUTPUT_START: usize = MAX_WORKERS + MAX_WORKERS * HELPERS;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BufferClass {
    Input { worker: usize },
    Ingress { worker: usize, helper: usize },
    Output { group: usize, channel: usize },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PoolUsage {
    pub owned_bytes: usize,
    pub slot_count: usize,
    pub leased_slots: usize,
}
pub struct ParentBuffers {
    limits: HostLimits,
    slots: [RefCell<Vec<u8>>; SLOT_COUNT],
    owned: usize,
}
impl ParentBuffers {
    /// Additional fixed roots include RuntimeHost/process/control/spawn inventory;
    /// the total actual backing is checked before any child may exist.
    pub fn new(limits: HostLimits, other_fixed_roots: usize) -> Result<Self, HostError> {
        limits.validate()?;
        let roots = add(size_of::<Self>(), other_fixed_roots)?;
        if add(roots, limits.payload_bytes()?)? > limits.parent_bytes {
            return Err(HostError::ResourceLimit);
        }
        let mut result = Self {
            limits,
            slots: std::array::from_fn(|_| RefCell::new(Vec::new())),
            owned: roots,
        };
        for worker in 0..limits.workers {
            result.allocate(INPUT_START + worker, limits.input_bytes)?;
            for helper in 0..HELPERS {
                result.allocate(
                    INGRESS_START + worker * HELPERS + helper,
                    limits.ingress_bytes,
                )?;
            }
        }
        for group in 0..limits.completion_groups {
            for channel in 0..CHANNELS {
                result.allocate(
                    OUTPUT_START + group * CHANNELS + channel,
                    limits.output_bytes,
                )?;
            }
        }
        if result.owned > limits.parent_bytes {
            return Err(HostError::ResourceLimit);
        }
        Ok(result)
    }
    fn allocate(&mut self, index: usize, bytes: usize) -> Result<(), HostError> {
        let value = self.slots[index].get_mut();
        value
            .try_reserve_exact(bytes)
            .map_err(|_| HostError::AllocationFailure)?;
        self.owned = add(self.owned, mul(value.capacity(), size_of::<u8>())?)?;
        if self.owned > self.limits.parent_bytes {
            return Err(HostError::ResourceLimit);
        }
        value.resize(bytes, 0);
        Ok(())
    }
    fn index(&self, class: BufferClass) -> Result<usize, HostError> {
        match class {
            BufferClass::Input { worker } if worker < self.limits.workers => {
                Ok(INPUT_START + worker)
            }
            BufferClass::Ingress { worker, helper }
                if worker < self.limits.workers && helper < HELPERS =>
            {
                Ok(INGRESS_START + worker * HELPERS + helper)
            }
            BufferClass::Output { group, channel }
                if group < self.limits.completion_groups && channel < CHANNELS =>
            {
                Ok(OUTPUT_START + group * CHANNELS + channel)
            }
            _ => Err(HostError::InvalidInput),
        }
    }
    pub fn reserve(&self, class: BufferClass, declared: usize) -> Result<ByteLease<'_>, HostError> {
        let mut buffer = self.slots[self.index(class)?]
            .try_borrow_mut()
            .map_err(|_| HostError::Busy)?;
        if declared > buffer.len() {
            return Err(HostError::ResourceLimit);
        }
        buffer.fill(0);
        Ok(ByteLease {
            buffer,
            length: declared,
        })
    }
    pub fn usage(&self) -> PoolUsage {
        PoolUsage {
            owned_bytes: self.owned,
            slot_count: self.limits.workers * (1 + HELPERS)
                + self.limits.completion_groups * CHANNELS,
            leased_slots: self
                .slots
                .iter()
                .filter(|slot| slot.try_borrow_mut().is_err())
                .count(),
        }
    }
    pub const fn root_bytes() -> usize {
        size_of::<Self>()
    }
}
/// Move-only slot borrow. No Vec ownership is returned and no resizing is exposed.
/// All bytes remain charged to ParentBuffers until its real destruction.
pub struct ByteLease<'a> {
    buffer: RefMut<'a, Vec<u8>>,
    length: usize,
}
impl ByteLease<'_> {
    pub fn as_slice(&self) -> &[u8] {
        &self.buffer[..self.length]
    }
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.buffer[..self.length]
    }
    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }
    pub fn len(&self) -> usize {
        self.length
    }
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
    pub fn set_len(&mut self, length: usize) -> Result<(), HostError> {
        if length > self.buffer.len() {
            return Err(HostError::ResourceLimit);
        }
        self.length = length;
        Ok(())
    }
}

impl Drop for ByteLease<'_> {
    fn drop(&mut self) {
        self.buffer.fill(0);
    }
}
