use super::*;
use crate::{
    model::Document,
    validation::{ValidationError, require},
};
use serde::{
    Deserialize,
    de::{IgnoredAny, MapAccess, Visitor},
};
use std::fmt;

fn semantics(document: &AnalysisDocument) -> Result<(), ValidationError> {
    match &document.artifact {
        AnalysisArtifact::GeometryQuery(value) => validate_query(value),
        AnalysisArtifact::EvaluationInput(value) => validate_evaluation_input(value),
        AnalysisArtifact::Measurement(value) => validate_measurement_case(value),
        AnalysisArtifact::GeometryCheck(value) => validate_geometry_check_case(value),
    }
}
impl AnalysisDocument {
    /// Bounded strict 0.2 decoding plus contract/evidence validation. It does not
    /// recompute geometry or resolve any referenced resource.
    pub fn from_json(input: &[u8], max_bytes: usize) -> Result<Self, ValidationError> {
        require(input.len() <= max_bytes, ValidationError::ResourceLimit)?;
        let document =
            serde_json::from_slice(input).map_err(|_| ValidationError::InvalidDocument)?;
        semantics(&document)?;
        Ok(document)
    }
    /// Validate direct typed values before serialization can erase invalid
    /// nonfinite optional metadata; then enforce the same strict wire shape.
    pub fn validate(&self) -> Result<(), ValidationError> {
        semantics(self)?;
        let bytes = serde_json::to_vec(self).map_err(|_| ValidationError::InvalidDocument)?;
        Self::from_json(&bytes, bytes.len()).map(|_| ())
    }
}
pub fn json_schema() -> Result<serde_json::Value, ValidationError> {
    serde_json::to_value(schemars::schema_for!(AnalysisDocument))
        .map_err(|_| ValidationError::InternalSchema)
}

struct Version(String);
impl<'de> Deserialize<'de> for Version {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Header;
        impl<'de> Visitor<'de> for Header {
            type Value = Version;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a versioned object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Version, A::Error> {
                let mut version = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "schema_version" {
                        if version.is_some() {
                            return Err(serde::de::Error::duplicate_field("schema_version"));
                        }
                        version = Some(map.next_value::<String>()?);
                    } else {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
                version
                    .map(Version)
                    .ok_or_else(|| serde::de::Error::missing_field("schema_version"))
            }
        }
        d.deserialize_map(Header)
    }
}
/// Exact-version file dispatch only. Plugin negotiation and the core decoder
/// remain strict 0.1; parser diagnostics never retain untrusted keys or values.
pub fn validate_input_document(input: &[u8], max_bytes: usize) -> Result<(), ValidationError> {
    require(input.len() <= max_bytes, ValidationError::ResourceLimit)?;
    let version: Version =
        serde_json::from_slice(input).map_err(|_| ValidationError::InvalidDocument)?;
    match version.0.as_str() {
        "0.1.0" => Document::from_json(input, max_bytes).map(|_| ()),
        "0.2.0" => AnalysisDocument::from_json(input, max_bytes).map(|_| ()),
        _ => Err(ValidationError::InvalidDocument),
    }
}
