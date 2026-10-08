use serde::{Deserialize, Serialize};
use uiblueprint_schema::model::*;

/// These limits are caller policy, not calibrated product defaults.
#[derive(Clone, Copy, Debug)]
pub struct ExportLimits {
    pub max_input_bytes: usize,
    pub max_output_bytes: usize,
    pub max_components: usize,
    pub max_views: usize,
    /// Readability policy: detail sheets are added in groups of this size.
    pub components_per_detail: usize,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Purpose {
    #[default]
    #[serde(alias = "explain")]
    Document,
    Propose,
    Detail,
    Flow,
    Compare,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    Observed,
    Proposed,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStatus {
    Draft,
    Accepted,
    Superseded,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Approval {
    pub status: ApprovalStatus,
    pub named_record: Option<String>,
}
/// All free text here is an explicitly reviewed public annotation, not collector text.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    pub document_id: String,
    pub revision: String,
    pub title: String,
    pub audience: String,
    pub language: String,
    pub date: String,
    pub owner: String,
    pub retention: String,
    pub specification_refs: Vec<String>,
    pub approval: Approval,
    pub page_format: String,
    pub output_size: String,
}
/// Caller-authored document annotations for one explicitly loaded observation.
/// Contains no geometry or replacement source facts. Validated in DrawingBrief.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObservedDocumentMetadata {
    pub metadata: Metadata,
    pub state: String,
    pub scope: String,
    pub environment: String,
    pub safe_source_reference: String,
    pub not_depicted: Vec<String>,
    pub public_text_fields: Vec<Field>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingBrief {
    pub metadata: Metadata,
    #[serde(default)]
    pub purpose: Purpose,
    pub views: Vec<ViewInput>,
    pub details: Vec<DetailRequest>,
    pub comparisons: Vec<ComparisonRequest>,
    pub transitions: Vec<FlowInput>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ViewInput {
    pub id: String,
    pub title: String,
    pub state: String,
    pub scope: String,
    pub environment: String,
    pub safe_source_reference: String,
    /// Explicit exclusions; supplements canonical coverage, never overrides it.
    pub not_depicted: Vec<String>,
    pub source: SourceInput,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "source_kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SourceInput {
    Observed {
        snapshot: Box<Snapshot>,
        /// Only these public text fields may pass through; value/secret content is never exported.
        public_text_fields: Vec<Field>,
    },
    Proposed {
        layout: ProposedLayout,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProposedLayout {
    pub requirements: Vec<String>,
    pub components: Vec<ProposedComponent>,
    pub dimensions: Vec<Dimension>,
    pub chains: Vec<DimensionChain>,
    pub unknowns: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProposedComponent {
    pub id: String,
    pub parent: Option<String>,
    pub role: Role,
    pub label: String,
    pub geometry: Geometry,
    pub state_and_actions: String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Edge {
    Left,
    Right,
    Top,
    Bottom,
    CenterX,
    CenterY,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DimensionAnchor {
    pub component: String,
    pub frame_kind: FrameKind,
    pub space: Space,
    pub edge: Edge,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dimension {
    pub id: String,
    pub label: String,
    pub anchors: [DimensionAnchor; 2],
    pub value: Option<f64>,
    pub units: Unit,
    pub source_kind: SourceKind,
    pub evidence: Vec<Evidence>,
    pub requirement_ref: Option<String>,
    pub unknown_reason: Option<String>,
    pub check_tolerance: Option<f64>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DimensionChain {
    pub terms: Vec<String>,
    pub total: String,
    /// Arithmetic agreement allowance; no implicit UI measurement uncertainty.
    pub arithmetic_tolerance: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DetailRequest {
    pub view: String,
    pub components: Vec<String>,
    pub title: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComparisonRequest {
    pub before: String,
    pub after: String,
    /// Explicitly labels differing requirement/observation bases; never creates identity matches.
    pub different_basis: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FlowInput {
    pub before: String,
    pub after: Option<String>,
    pub description: String,
    /// None means assumed/unverified; a frame sequence is not causal evidence.
    pub transition: Option<Transition>,
    #[serde(default)]
    pub actions: Vec<Action>,
}
#[derive(Clone, Debug, Serialize)]
pub struct Scene {
    pub guide: &'static str,
    pub views: Vec<SceneView>,
    pub flow: Vec<FlowLink>,
    pub comparisons: Vec<ComparisonRequest>,
}
#[derive(Clone, Debug, Serialize)]
pub struct SceneView {
    pub snapshot_ref: Option<Id>,
    pub snapshot_revision: Option<u64>,
    pub id: String,
    pub title: String,
    pub source_kind: SourceKind,
    pub source: String,
    pub state: String,
    pub scope: String,
    pub environment: String,
    pub coverage: Option<Coverage>,
    pub surfaces: Vec<SurfaceRecord>,
    pub observations: Vec<Observation>,
    pub not_depicted: Vec<String>,
    pub components: Vec<SceneComponent>,
    pub relations: Vec<Relation>,
    pub mappings: Vec<ComponentMapping>,
    pub focus: Option<Focus>,
    pub requirements: Vec<String>,
    pub unknowns: Vec<String>,
}
#[derive(Clone, Debug, Serialize)]
pub struct SceneComponent {
    pub id: String,
    pub surface: Option<Identity>,
    pub source_key: Option<SourceKey>,
    pub native_role: Option<Availability>,
    pub parent: Option<String>,
    pub children: Vec<String>,
    pub role: Option<Role>,
    pub label: Option<String>,
    pub properties: Vec<Property>,
    pub geometry: Option<Geometry>,
    pub declarations: Vec<SourceDeclaration>,
    pub extensions: Vec<ExtensionProperty>,
    pub state_and_actions: Option<String>,
}
#[derive(Clone, Debug, Serialize)]
pub struct ViewDimensions {
    pub view: String,
    pub dimensions: Vec<Dimension>,
    pub chains: Vec<DimensionChain>,
}
#[derive(Clone, Debug, Serialize)]
pub struct FlowLink {
    pub before: String,
    pub after: Option<String>,
    pub description: String,
    pub status: &'static str,
    pub steps: Vec<FlowStep>,
}
#[derive(Clone, Debug, Serialize)]
pub struct FlowStep {
    pub action_ref: Id,
    pub target: SourceKey,
    pub intent: String,
    pub modality: InputModality,
    pub delivery: DeliveryStatus,
    pub outcome: Outcome,
    pub verification_observation: Option<Id>,
}
#[derive(Clone, Debug, Serialize)]
pub struct Sheet {
    pub id: String,
    pub view: String,
    pub title: String,
    pub kind: &'static str,
    pub parent_view: Option<String>,
    pub components: Vec<String>,
    pub placement: &'static str,
    pub state: String,
    pub units: Vec<Unit>,
}
