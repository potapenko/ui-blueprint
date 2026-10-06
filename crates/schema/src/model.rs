//! Candidate wire records. Validation is separate from collection and analytics.
use crate::SchemaVersion;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

macro_rules! record {
    ($(#[$extra:meta])* $name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
        $(#[$extra])*
        #[derive(Clone, Debug, PartialEq, Serialize, JsonSchema)]
        #[serde(deny_unknown_fields)]
        pub struct $name { $(pub $field: $ty),* }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self,D::Error> {
                #[derive(Deserialize)]
                #[serde(deny_unknown_fields)]
                struct Fields { $($field: $ty),* }
                struct MapOnly;
                impl<'de> serde::de::Visitor<'de> for MapOnly {
                    type Value = $name;
                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("an object record") }
                    fn visit_map<A: serde::de::MapAccess<'de>>(self, access: A) -> Result<$name,A::Error> {
                        let fields = Fields::deserialize(serde::de::value::MapAccessDeserializer::new(access))?;
                        Ok($name { $($field: fields.$field),* })
                    }
                }
                d.deserialize_map(MapOnly)
            }
        }
    };
}
macro_rules! vocabulary {
    ($name:ident { $($variant:ident),* $(,)? }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, JsonSchema)]
        #[serde(rename_all = "snake_case")]
        pub enum $name { $($variant),* }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self,D::Error> {
                #[derive(Deserialize)]
                #[serde(rename_all = "snake_case")]
                enum TextValue { $($variant),* }
                let text = String::deserialize(d)?;
                let parsed = TextValue::deserialize(serde::de::value::StringDeserializer::<D::Error>::new(text))?;
                Ok(match parsed { $(TextValue::$variant => Self::$variant),* })
            }
        }
    };
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, JsonSchema)]
#[serde(transparent)]
pub struct Id(#[schemars(length(min = 1, max = 256))] pub String);
impl<'de> Deserialize<'de> for Id {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let text = String::deserialize(d)?;
        if text.is_empty() || text.chars().count() > 256 {
            return Err(serde::de::Error::custom("invalid identity length"));
        }
        Ok(Self(text))
    }
}
record!(
    #[derive(Eq, PartialOrd, Ord)]
    Identity {
        id: Id,
        generation: Id
    }
);
record!(
    #[derive(Eq, PartialOrd, Ord)]
    SourceKey {
        namespace: Id,
        key: Id
    }
);

vocabulary!(Projection {
    Interaction,
    Design
});
vocabulary!(Channel {
    ExternalSemantics,
    RenderedCapture,
    OptInLayoutProbe
});
vocabulary!(Provenance {
    Reported,
    Derived,
    Estimated
});
vocabulary!(Freshness {
    Current,
    Stale,
    Unverified
});
vocabulary!(AnswerSource { Live, Cache });
vocabulary!(Consistency {
    Stable,
    Unstable,
    Unknown
});
vocabulary!(CoverageStatus {
    Complete,
    Partial,
    Unknown
});
vocabulary!(Sensitivity { Public, Sensitive });
vocabulary!(Unit { Px, CssPx, Pt, Dp });
vocabulary!(SpaceKind {
    Screen,
    Window,
    Surface,
    Viewport,
    Document,
    Local
});
vocabulary!(Origin {
    TopLeft,
    BottomLeft
});
vocabulary!(FrameKind {
    LayoutBounds,
    AccessibilityBounds,
    HitRegion,
    VisibleRegion,
    PaintBounds
});
vocabulary!(CaptureKind {
    WindowIsolated,
    DisplayComposited
});
vocabulary!(FreshnessBasis {
    LiveRead,
    Revalidated,
    TtlOnly,
    Unverified
});
vocabulary!(TimeUnit {
    Milliseconds,
    Seconds
});
vocabulary!(TargetState {
    Current,
    Gone,
    Unresolved
});
vocabulary!(Role {
    Text,
    Heading,
    Textbox,
    Searchbox,
    Button,
    Link,
    Checkbox,
    Radio,
    Switch,
    Combobox,
    Option,
    List,
    Listitem,
    Menu,
    Menuitem,
    Tab,
    Dialog,
    Form,
    Group,
    Scrollarea,
    Slider,
    Image,
    Media,
    Custom,
    Unknown
});
vocabulary!(Field {
    Role,
    Name,
    AccessibilityName,
    VisibleText,
    Description,
    Value,
    Placeholder,
    InputKind,
    Required,
    Enabled,
    Readonly,
    Checked,
    Selected,
    Expanded,
    Focused,
    Invalid,
    Actions,
    LayoutBounds,
    AccessibilityBounds,
    HitRegion,
    VisibleRegion,
    Baseline,
    PaintBounds
});
vocabulary!(RelationKind {
    LabelledBy,
    DescribedBy,
    ErrorFor,
    Controls,
    Owns,
    MemberOf,
    AnchoredTo,
    ReturnTarget,
    Represents,
    CorrespondsTo,
    ComponentOwner,
    VisualContainer
});
vocabulary!(InputModality {
    Semantic,
    Pointer,
    Keyboard,
    Setter,
    Touch,
    Remote
});
vocabulary!(CapabilityStatus {
    Supported,
    Partial,
    Unsupported,
    PermissionRequired
});
vocabulary!(FreshnessPolicy {
    CurrentRequired,
    CachedAllowed
});
vocabulary!(Comparison {
    Equal,
    AtLeast,
    AtMost,
    GreaterThan
});
vocabulary!(QuantityKind {
    Length,
    Area,
    Ratio
});
vocabulary!(GeometryRelation {
    Gap,
    Distance,
    Aligned,
    Inside,
    Intersects,
    Overflow,
    Width,
    Height,
    Ratio,
    EqualSpacing,
    Baseline
});
vocabulary!(CheckStatus {
    Pass,
    Fail,
    Unknown
});
vocabulary!(DeliveryStatus {
    NotDispatched,
    Accepted,
    Confirmed,
    Unknown
});
vocabulary!(Outcome {
    PendingVerification,
    Succeeded,
    Failed,
    Interrupted,
    ActionOutcomeUnknown
});
vocabulary!(ErrorCode {
    Unsupported,
    PluginNotInstalled,
    IncompatibleVersion,
    PermissionRequired,
    TargetUnresolved,
    StaleTarget,
    AmbiguousTarget,
    UnstableState,
    IncompleteScope,
    MissingTransform,
    ResyncRequired,
    Timeout,
    Interrupted,
    ActionOutcomeUnknown
});

record!(PluginIdentity {
    id: Id,
    version: Id
});
record!(Context { schema_version: SchemaVersion, session_id: Id, target: Identity, surfaces: Vec<Identity>, scope_id: Id, projection: Projection, fields: Vec<Field>, plugin: PluginIdentity, environment_revision: Id });
record!(Coverage { status: CoverageStatus, scope_id: Id, fields: Vec<Field>, omitted_count: Option<u64>, unknown_count: Option<u64> });
record!(Observation { id: Id, source_namespace: Id, channel: Channel, start: f64, end: f64, clock_domain: Id, time_unit: TimeUnit, freshness_basis: FreshnessBasis, consistency_reason: Option<Id>, answer_source: AnswerSource, freshness: Freshness, last_verified: Option<f64>, consistency: Consistency, coverage: Coverage });
record!(Evidence {
    observation_id: Id,
    source_namespace: Id,
    provenance: Provenance,
    method: Id,
    uncertainty: Option<Uncertainty>
});
record!(Uncertainty { absolute: f64, units: Option<Unit>, method: Id });
record!(Space {
    id: Id,
    kind: SpaceKind,
    units: Unit,
    origin: Origin
});
record!(Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64
});
record!(Point { x: f64, y: f64 });
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(
    tag = "shape",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum Shape {
    Rect(Rect),
    Quad([Point; 4]),
    Polygon(Vec<Point>),
    Fragments(Vec<Rect>),
}
record!(Transform {
    from: Space,
    to: Space,
    affine: [f64; 6],
    target: Identity,
    surface: Identity,
    environment_revision: Id,
    evidence: Evidence
});
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum TransformState {
    LocalOnly,
    Known { transform: Transform },
    Unknown { reason: Id },
}
record!(Geometry {
    frame_kind: FrameKind,
    coordinate_space: Space,
    shape: Shape,
    transform: TransformState
});

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(
    tag = "type",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum Value {
    Flag(bool),
    Text(String),
    Number(f64),
    Baseline {
        coordinate: f64,
        space: Space,
    },
    Quantity {
        amount: f64,
        kind: QuantityKind,
        source_units: Unit,
    },
    Role(Role),
    TextList(Vec<String>),
    Identity(Identity),
    Geometry(Geometry),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "availability", rename_all = "snake_case", deny_unknown_fields)]
pub enum Availability {
    Known { value: Value },
    Unknown { reason: Id },
    Unsupported { reason: Id },
    Redacted,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "selection", rename_all = "snake_case", deny_unknown_fields)]
pub enum Property {
    NotRequested {
        field: Field,
    },
    Requested {
        field: Field,
        sensitivity: Sensitivity,
        evidence: Evidence,
        state: Availability,
    },
}
impl Property {
    pub fn field(&self) -> Field {
        match self {
            Self::NotRequested { field } | Self::Requested { field, .. } => *field,
        }
    }
    pub fn known(&self) -> Option<&Value> {
        match self {
            Self::Requested {
                state: Availability::Known { value },
                ..
            } => Some(value),
            _ => None,
        }
    }
}
record!(Relation {
    kind: RelationKind,
    from: SourceKey,
    to: SourceKey,
    evidence: Evidence
});
record!(ComponentMapping { logical_component_key: Id, members: Vec<SourceKey>, declaration_source: Id, provenance: Provenance });
record!(ExtensionProperty {
    namespace: Id,
    name: Id,
    property: Property
});
record!(SourceDeclaration {
    namespace: Id,
    name: Id,
    state: Availability,
    sensitivity: Sensitivity,
    source: Id
});
record!(Node { key: SourceKey, surface: Identity, native_role: Availability, properties: Vec<Property>, children: Vec<SourceKey>, extensions: Vec<ExtensionProperty>, source_declarations: Vec<SourceDeclaration> });
record!(TextSelection {
    anchor: u64,
    focus: u64,
    units: Id,
    evidence: Evidence
});
record!(CaptureFrame { observation_id: Id, capture_target: Identity, capture_kind: CaptureKind, pixel_width: u32, pixel_height: u32, crop_transform: TransformState, included_surfaces: Vec<Identity>, excluded_surfaces: Vec<Identity>, unresolved_surfaces: Vec<Identity>, surface_coverage: CoverageStatus, captures_audio: bool, filter_id: Id, payload_ref: Id });
record!(Focus { keyboard: FocusRef, accessibility: FocusRef, active_descendant: FocusRef, text_selection: Option<TextSelection>, composition_state: Property });
record!(SurfaceRecord { identity: Identity, native_owner: Availability, initiated_by: Option<Identity>, anchor: Option<SourceKey>, evidence: Evidence });
record!(Snapshot { surface_records: Vec<SurfaceRecord>, id: Id, revision: u64, source_state: Option<Id>, context: Context, observations: Vec<Observation>, nodes: Vec<Node>, relations: Vec<Relation>, components: Vec<ComponentMapping>, focus: Focus, captures: Vec<CaptureFrame>, coverage: Coverage });
record!(Removal {
    key: SourceKey,
    evidence: Evidence
});
record!(Delta { base_revision: u64, revision: u64, source_state: Option<Id>, context: Context, observations: Vec<Observation>, upsert: Vec<Node>, removed: Vec<Removal>, relations: Vec<Relation>, focus: Focus, coverage: Coverage });
record!(BackendRef {
    session_id: Id,
    key: SourceKey,
    snapshot_id: Id,
    observation_id: Id,
    target: Identity,
    surface: Identity
});
record!(Limits {
    max_elements: u32,
    max_depth: u32,
    max_output_bytes: u64,
    deadline_ms: u64
});
record!(Capability { channel: Channel, operation: Id, status: CapabilityStatus, reason: Option<Id> });
record!(SessionDescriptor { allowed_scopes: Vec<Id>, session_id: Id, plugin: PluginIdentity, supported_versions: Vec<SchemaVersion>, target: Identity, surfaces: Vec<Identity>, capabilities: Vec<Capability> });
record!(Anchor {
    element: SourceKey,
    frame_kind: FrameKind,
    coordinate_space: Space,
    fraction: f64,
    axis: Id
});

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "intent", rename_all = "snake_case", deny_unknown_fields)]
pub enum Intent {
    Focus,
    Activate,
    SetChecked { value: bool },
    Fill { text: String },
    Type { text: String },
    FillSecret { secret_reference: Id },
    SelectOption { option: Id },
    Scroll,
    Press { key: Id },
    Submit,
    Dismiss,
}
record!(Action {
    id: Id,
    context: Context,
    backend_ref: BackendRef,
    intent: Intent,
    modality: InputModality,
    input_space: Option<Space>,
    required_enabled: bool,
    authorized_scope: Id,
    unique_match: bool,
    resolution: Resolution
});
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
pub enum Operation {
    Observe {
        channels: Vec<Channel>,
    },
    Inspect {
        snapshot_id: Id,
    },
    Measure {
        snapshot_id: Id,
        from: Anchor,
        to: Anchor,
    },
    Diff {
        before: Id,
        after: Id,
    },
    Check {
        snapshot_id: Id,
        expectation_id: Id,
    },
    Prepare {
        action: Action,
    },
    Act {
        action: Action,
    },
    Detach,
}
record!(Request {
    clock_domain: Id,
    request_id: Id,
    context: Context,
    limits: Limits,
    freshness_policy: FreshnessPolicy,
    operation: Operation
});
record!(Step { id: Id, action_id: Id, delivery: DeliveryStatus, outcome: Outcome, before_snapshot: Id, after_snapshot: Option<Id>, verification_observation: Option<Id> });
record!(Transition { id: Id, context: Context, steps: Vec<Step>, completed_steps: Vec<Id>, stopped_at: Option<Id>, stop_on_error: bool });
record!(ContextConditions { platform: Option<Id>, input_mode: Option<InputModality>, text_scale: Option<f64> });
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "relation", rename_all = "snake_case", deny_unknown_fields)]
pub enum Rule {
    PropertyEquals {
        field: Field,
        expected: Value,
    },
    Geometry {
        operation: GeometryRelation,
        anchors: Vec<Anchor>,
        expected: f64,
        comparison: Comparison,
        quantity_kind: QuantityKind,
        units: Unit,
        tolerance: f64,
    },
}
record!(Expectation { id: Id, scope_id: Id, targets: Vec<SourceKey>, rule: Rule, applies_when: ContextConditions, expected_from: Id });
record!(Finding { id: Id, expectation_id: Id, snapshot_id: Id, observation_id: Option<Id>, status: CheckStatus, measured: Option<Value>, reason: Option<Id> });
record!(Issue { code: ErrorCode, scope_id: Id, failed_step: Option<Id>, recovery_class: Id });
record!(ExportRecord { purpose: Id, before: Id, after: Id, changed: Vec<SourceKey>, include_geometry: bool, include_pixels: bool, description: String });

// Validation bundles carry explicit context instead of implicitly loading IDs,
// files or URLs. Transport uses the same inner record types without the bundle.
record!(PropertyCase {
    property: Property,
    observation: Observation
});
record!(GeometryCase {
    context: Context,
    geometry: Geometry,
    observation: Observation
});
record!(DeltaCase {
    base: Snapshot,
    update: Delta,
    source_snapshot: Option<Snapshot>
});
record!(ActionCase {
    snapshot: Snapshot,
    action: Action
});
record!(FindingCase {
    snapshot: Snapshot,
    expectation: Expectation,
    finding: Finding
});
record!(GoldenChain {
    before: Snapshot,
    action: Action,
    delivery: Transition,
    verification: Transition,
    after: Snapshot,
    delta: Delta,
    expectation: Expectation,
    finding: Finding,
    export: ExportRecord
});
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(
    tag = "kind",
    content = "data",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum Artifact {
    Request(Request),
    Session(SessionDescriptor),
    SessionContext(SessionCase),
    ResolutionRefusal(ResolutionRefusal),
    ChannelResponse(ChannelResponse),
    Capability(Capability),
    Property(PropertyCase),
    Observation(Observation),
    Snapshot(Snapshot),
    Geometry(GeometryCase),
    Delta(DeltaCase),
    Action(ActionCase),
    Transition(Transition),
    Expectation(Expectation),
    Finding(FindingCase),
    Error(Issue),
    GoldenChain(Box<GoldenChain>),
    ActionResult(ActionResult),
    DeltaResult(DeltaResult),
    TemporalComparison(TemporalComparison),
    TransitionContext(TransitionCase),
}
record!(Document {
    schema_version: SchemaVersion,
    artifact: Artifact
});

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum FocusRef {
    NotRequested,
    Known {
        target: SourceKey,
        evidence: Evidence,
    },
    None {
        evidence: Evidence,
    },
    Unknown {
        reason: Id,
    },
}
record!(Resolution { evidence: Evidence, writable: Availability, value_allowed: Availability, available_intents: Vec<Id> });
record!(ActionResult {
    snapshot: Snapshot,
    action: Action,
    target_state: TargetState,
    issue: Issue,
    dispatched: bool
});
record!(DeltaResult { base: Option<Snapshot>, update: Delta, issue: Issue, published: bool });
record!(ClockTransform {
    from_clock: Id,
    to_clock: Id,
    scale: f64,
    offset: f64,
    source: Id
});
record!(TemporalComparison { observations: Vec<Observation>, transform: Option<ClockTransform>, overlap: Option<f64>, atomic_claim: bool });
record!(TransitionCase { transition: Transition, before: Snapshot, after: Option<Snapshot> });
record!(SessionCase { session: SessionDescriptor, request: Request, backend_ref: Option<BackendRef> });
record!(TargetCandidate {
    process_id: u32,
    window_id: u64,
    title: String,
    bounds: Rect,
    coordinate_space: Space,
    process_continuity: Availability,
    owner_binding: Availability
});
record!(ResolutionRefusal { requested: Context, candidates: Vec<TargetCandidate>, issue: Issue, dispatched: bool });
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "status", content = "data", rename_all = "snake_case", deny_unknown_fields)]
pub enum ChannelResult { Observed(Box<Snapshot>), Failed(Issue) }
record!(ChannelResponse { request_id: Id, session_id: Id, dispatch_sequence: u64, target: Identity, channel: Channel, result: ChannelResult });
