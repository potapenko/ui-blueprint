//! Initial version boundary for S01. This does not define the product graph.
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// A supported candidate wire version, separate from the package version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum SchemaVersion {
    #[serde(rename = "0.1.0")]
    V0_1_0,
}

impl SchemaVersion {
    pub const CURRENT: Self = Self::V0_1_0;

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V0_1_0 => "0.1.0",
        }
    }
}

impl fmt::Display for SchemaVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SchemaVersion {
    type Err = VersionError;

    /// Accepts the exact current version. Other canonical numeric triplets are
    /// incompatible; malformed tokens (including prerelease/build syntax) are
    /// invalid. This is the candidate wire grammar, not a general SemVer parser.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut parts = value.split('.');
        for _ in 0..3 {
            let part = parts.next().ok_or(VersionError::MalformedVersion)?;
            if part.is_empty()
                || !part.bytes().all(|byte| byte.is_ascii_digit())
                || (part.len() > 1 && part.starts_with('0'))
            {
                return Err(VersionError::MalformedVersion);
            }
        }
        if parts.next().is_some() {
            return Err(VersionError::MalformedVersion);
        }
        match value {
            "0.1.0" => Ok(Self::CURRENT),
            _ => Err(VersionError::IncompatibleVersion),
        }
    }
}

/// The finite T01 document. S01 extends this owner with actual envelopes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct VersionDocument {
    schema_version: SchemaVersion,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawVersionDocument {
    schema_version: String,
}

impl VersionDocument {
    pub const fn current() -> Self {
        Self {
            schema_version: SchemaVersion::CURRENT,
        }
    }

    pub const fn schema_version(&self) -> SchemaVersion {
        self.schema_version
    }

    /// Parses one complete document, checking the caller's byte bound first.
    ///
    /// # Errors
    /// Rejects oversized input, invalid JSON/document shape, malformed version
    /// tokens and incompatible versions. Errors retain no untrusted payload.
    pub fn from_json(input: &[u8], max_bytes: usize) -> Result<Self, VersionError> {
        if input.len() > max_bytes {
            return Err(VersionError::InputTooLarge);
        }
        // Serde's struct decoder can also accept sequences; the wire document
        // is an object. JSON parsing itself remains serde_json's responsibility.
        if input
            .iter()
            .copied()
            .find(|byte| !b" \t\r\n".contains(byte))
            != Some(b'{')
        {
            return Err(VersionError::InvalidDocument);
        }
        // serde errors can quote unknown keys or values; never retain them in
        // our diagnostic error, including through Debug or a source chain.
        let raw: RawVersionDocument =
            serde_json::from_slice(input).map_err(|_| VersionError::InvalidDocument)?;
        Ok(Self {
            schema_version: raw.schema_version.parse()?,
        })
    }

    /// Serializes the typed document without unrelated diagnostic output.
    ///
    /// # Errors
    /// Returns a payload-free error if the serializer cannot encode the document.
    pub fn to_json(&self) -> Result<String, VersionError> {
        serde_json::to_string(self).map_err(|_| VersionError::EncodingFailed)
    }
}

/// Stable error categories for this finite version boundary, not final CLI codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VersionError {
    InputTooLarge,
    InvalidDocument,
    MalformedVersion,
    IncompatibleVersion,
    EncodingFailed,
}

impl fmt::Display for VersionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InputTooLarge => "version document exceeds the caller's byte limit",
            Self::InvalidDocument => {
                "expected a JSON object containing only a schema_version string"
            }
            Self::MalformedVersion => "schema version must be a canonical numeric triplet",
            Self::IncompatibleVersion => "incompatible schema version; supported version is 0.1.0",
            Self::EncodingFailed => "could not encode the version document",
        })
    }
}

impl std::error::Error for VersionError {}
