use crate::model::{record, *};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Exact local analysis version. It does not participate in plugin negotiation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub enum AnalysisVersion {
    #[serde(rename = "0.2.0")]
    V0_2_0,
}
impl AnalysisVersion {
    pub const CURRENT: Self = Self::V0_2_0;
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V0_2_0 => "0.2.0",
        }
    }
}
impl<'de> Deserialize<'de> for AnalysisVersion {
    fn deserialize<D: serde::Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
        match String::deserialize(decoder)?.as_str() {
            "0.2.0" => Ok(Self::CURRENT),
            _ => Err(serde::de::Error::custom("unsupported analysis version")),
        }
    }
}

record!(
    /// Factual relation inputs, with no expected value/comparison/tolerance.
    GeometryQuery {
        id: Id, scope_id: Id, targets: Vec<SourceKey>,
        operation: GeometryRelation, anchors: Vec<Anchor>,
        quantity_kind: QuantityKind, units: Unit, applies_when: ContextConditions
    }
);
impl GeometryQuery {
    /// Copy only geometric query fields from an actual supplied expectation.
    /// This extraction does not validate the input or fabricate normative values.
    pub fn from_expectation(expectation: &Expectation) -> Option<Self> {
        let Rule::Geometry {
            operation,
            anchors,
            quantity_kind,
            units,
            ..
        } = &expectation.rule
        else {
            return None;
        };
        Some(Self {
            id: expectation.id.clone(),
            scope_id: expectation.scope_id.clone(),
            targets: expectation.targets.clone(),
            operation: *operation,
            anchors: anchors.clone(),
            quantity_kind: *quantity_kind,
            units: *units,
            applies_when: expectation.applies_when.clone(),
        })
    }
}
record!(ObservedConditions {
    values: ContextConditions,
    evidence: Evidence
});
record!(
    /// Owned inputs bound to one immutable source; observations are not invented here.
    EvaluationInput {
        snapshot_id: Id, revision: u64, context: Context, result_space: Space,
        transforms: Vec<Transform>, conditions: Option<ObservedConditions>
    }
);

/// Rect details are in the selected result space; insets are not padding and
/// intersection is not a claim about visual occlusion.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum MeasurementDetails {
    Scalar {},
    Insets {
        left: f64,
        top: f64,
        right: f64,
        bottom: f64,
    },
    Intersection {
        rect: Option<Rect>,
    },
    Gaps {
        values: Vec<f64>,
    },
}
record!(Measurement {
    value: Value, space: Space, details: MeasurementDetails, evidence: Vec<Evidence>
});

/// Closed unavailable reasons, retaining the current engine's string vocabulary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MeasurementUnknownReason {
    NotRequested,
    UnknownProperty,
    UnsupportedProperty,
    RedactedProperty,
    #[serde(rename = "target_unresolved")]
    MissingTarget,
    MissingTransform,
    FrameKindMismatch,
    UnsupportedShape,
    IncompleteScope,
    UnstableState,
    ApplicabilityUnknown,
    NotApplicable,
    UndefinedRatio,
}
impl MeasurementUnknownReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequested => "not_requested",
            Self::UnknownProperty => "unknown_property",
            Self::UnsupportedProperty => "unsupported_property",
            Self::RedactedProperty => "redacted_property",
            Self::MissingTarget => "target_unresolved",
            Self::MissingTransform => "missing_transform",
            Self::FrameKindMismatch => "frame_kind_mismatch",
            Self::UnsupportedShape => "unsupported_shape",
            Self::IncompleteScope => "incomplete_scope",
            Self::UnstableState => "unstable_state",
            Self::ApplicabilityUnknown => "applicability_unknown",
            Self::NotApplicable => "not_applicable",
            Self::UndefinedRatio => "undefined_ratio",
        }
    }
}
impl<'de> Deserialize<'de> for MeasurementUnknownReason {
    fn deserialize<D: serde::Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
        let text = String::deserialize(decoder)?;
        match text.as_str() {
            "not_requested" => Ok(Self::NotRequested),
            "unknown_property" => Ok(Self::UnknownProperty),
            "unsupported_property" => Ok(Self::UnsupportedProperty),
            "redacted_property" => Ok(Self::RedactedProperty),
            "target_unresolved" => Ok(Self::MissingTarget),
            "missing_transform" => Ok(Self::MissingTransform),
            "frame_kind_mismatch" => Ok(Self::FrameKindMismatch),
            "unsupported_shape" => Ok(Self::UnsupportedShape),
            "incomplete_scope" => Ok(Self::IncompleteScope),
            "unstable_state" => Ok(Self::UnstableState),
            "applicability_unknown" => Ok(Self::ApplicabilityUnknown),
            "not_applicable" => Ok(Self::NotApplicable),
            "undefined_ratio" => Ok(Self::UndefinedRatio),
            _ => Err(serde::de::Error::custom("invalid analysis unknown reason")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum MeasurementResult {
    Known {
        measurement: Measurement,
    },
    Unknown {
        reason: MeasurementUnknownReason,
        evidence: Vec<Evidence>,
    },
}
impl MeasurementResult {
    pub fn unknown_reason(&self) -> Option<MeasurementUnknownReason> {
        match self {
            Self::Known { .. } => None,
            Self::Unknown { reason, .. } => Some(*reason),
        }
    }
}
record!(MeasurementCase {
    snapshot: Snapshot,
    query: GeometryQuery,
    evaluation: EvaluationInput,
    result: MeasurementResult
});
record!(GeometryCheckCase {
    snapshot: Snapshot,
    expectation: Expectation,
    evaluation: EvaluationInput,
    measurement: MeasurementResult,
    finding: Finding
});

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(
    tag = "kind",
    content = "data",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum AnalysisArtifact {
    GeometryQuery(Box<GeometryQuery>),
    EvaluationInput(Box<EvaluationInput>),
    Measurement(Box<MeasurementCase>),
    GeometryCheck(Box<GeometryCheckCase>),
}
record!(AnalysisDocument {
    schema_version: AnalysisVersion,
    artifact: AnalysisArtifact
});
