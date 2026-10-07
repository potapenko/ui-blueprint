//! Fixed trusted authority metadata, separate from untrusted canonical bodies.
use crate::{HostError, OperationClass};
use uiblueprint_schema::model::Identity;
pub const ID_BYTES: usize = 1024;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FixedId {
    bytes: [u8; ID_BYTES],
    len: u16,
}
impl FixedId {
    pub fn new(value: &str) -> Result<Self, HostError> {
        if value.is_empty() || value.chars().count() > 256 || value.len() > ID_BYTES {
            return Err(HostError::InvalidInput);
        }
        let mut result = Self {
            bytes: [0; ID_BYTES],
            len: value.len() as u16,
        };
        result.bytes[..value.len()].copy_from_slice(value.as_bytes());
        Ok(result)
    }
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes[..self.len as usize])
            .expect("FixedId copies validated UTF-8")
    }
    pub fn len(&self) -> usize {
        self.len as usize
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
impl std::fmt::Debug for FixedId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FixedId")
            .field("bytes", &self.len)
            .finish_non_exhaustive()
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TargetLease {
    id: FixedId,
    generation: FixedId,
    mutation: bool,
}
impl TargetLease {
    /// Caller must establish actual authority independently; never construct this
    /// from UI text or inferred capabilities. Descriptor/request bodies cannot
    /// broaden its target/generation or mutation class.
    pub fn authorized(target: &Identity, mutation: bool) -> Result<Self, HostError> {
        Ok(Self {
            id: FixedId::new(&target.id.0)?,
            generation: FixedId::new(&target.generation.0)?,
            mutation,
        })
    }
    pub fn permits(&self, class: OperationClass) -> bool {
        class != OperationClass::Mutation || self.mutation
    }
    pub fn matches(&self, target: &Identity) -> bool {
        self.id.as_str() == target.id.0 && self.generation.as_str() == target.generation.0
    }
    pub fn id(&self) -> &str {
        self.id.as_str()
    }
    pub fn generation(&self) -> &str {
        self.generation.as_str()
    }
    pub(crate) fn from_parts(id: FixedId, generation: FixedId, mutation: bool) -> Self {
        Self {
            id,
            generation,
            mutation,
        }
    }
}
