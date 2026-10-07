//! Private input segmentation for operations that require several EXISTING
//! canonical documents. No graph fields or second JSON decoder exist here.
use crate::HostError;
pub const SEGMENTS: usize = 4;
const PREFIX: usize = 8 + SEGMENTS * 8;
const MAGIC: [u8; 8] = *b"UIBARGS1";
pub struct Tape<'a> {
    segments: [&'a [u8]; SEGMENTS],
    count: usize,
}
impl<'a> Tape<'a> {
    pub fn decode(bytes: &'a [u8]) -> Result<Self, HostError> {
        if bytes.len() < PREFIX || bytes[..8] != MAGIC {
            return Err(HostError::InvalidInput);
        }
        let mut cursor = PREFIX;
        let mut segments = [&bytes[0..0]; SEGMENTS];
        let mut count = 0;
        let mut ended = false;
        for (i, segment) in segments.iter_mut().enumerate() {
            let mut length = [0; 8];
            length.copy_from_slice(&bytes[8 + i * 8..16 + i * 8]);
            let length =
                usize::try_from(u64::from_le_bytes(length)).map_err(|_| HostError::Overflow)?;
            if length == 0 {
                ended = true;
                continue;
            }
            if ended {
                return Err(HostError::InvalidInput);
            }
            let end = cursor.checked_add(length).ok_or(HostError::Overflow)?;
            *segment = bytes.get(cursor..end).ok_or(HostError::InvalidInput)?;
            cursor = end;
            count += 1;
        }
        if cursor != bytes.len() {
            return Err(HostError::InvalidInput);
        }
        Ok(Self { segments, count })
    }
    pub fn get(&self, index: usize) -> Result<&'a [u8], HostError> {
        if index >= self.count {
            return Err(HostError::InvalidInput);
        }
        Ok(self.segments[index])
    }
    pub fn count(&self) -> usize {
        self.count
    }
}
/// Caller writes directly into its reserved InputLease. Length-prefix arithmetic
/// is bounded before copying, and no input-proportional parent allocation occurs.
pub fn encode(parts: &[&[u8]], target: &mut [u8]) -> Result<usize, HostError> {
    if parts.is_empty() || parts.len() > SEGMENTS || parts.iter().any(|p| p.is_empty()) {
        return Err(HostError::InvalidInput);
    }
    let total = parts.iter().try_fold(PREFIX, |sum, p| {
        sum.checked_add(p.len()).ok_or(HostError::Overflow)
    })?;
    if total > target.len() {
        return Err(HostError::ResourceLimit);
    }
    target[..PREFIX].fill(0);
    target[..8].copy_from_slice(&MAGIC);
    let mut cursor = PREFIX;
    for (i, part) in parts.iter().enumerate() {
        target[8 + i * 8..16 + i * 8].copy_from_slice(&(part.len() as u64).to_le_bytes());
        target[cursor..cursor + part.len()].copy_from_slice(part);
        cursor += part.len();
    }
    Ok(cursor)
}
