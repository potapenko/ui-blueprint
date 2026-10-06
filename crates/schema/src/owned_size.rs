//! Exhaustive canonical owned-layout sizing. Not an allocator/RSS or peak cap.
//! Inline layout is counted once; heap payload uses actual container capacities.
use crate::{SchemaVersion, model::*};
use std::mem::size_of;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Heap {
    pub bytes: usize,
    pub allocations: usize,
    pub slack_bytes: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Overflow;

impl Heap {
    pub fn combine(self, other: Self) -> Result<Self, Overflow> {
        Ok(Self {
            bytes: self.bytes.checked_add(other.bytes).ok_or(Overflow)?,
            allocations: self
                .allocations
                .checked_add(other.allocations)
                .ok_or(Overflow)?,
            slack_bytes: self
                .slack_bytes
                .checked_add(other.slack_bytes)
                .ok_or(Overflow)?,
        })
    }
}
fn sum(parts: impl IntoIterator<Item = Heap>) -> Result<Heap, Overflow> {
    parts.into_iter().try_fold(Heap::default(), Heap::combine)
}
pub trait HeapSize {
    fn heap(&self) -> Result<Heap, Overflow>;
}
pub fn owned<T: HeapSize>(value: &T) -> Result<(usize, Heap, usize), Overflow> {
    let heap = value.heap()?;
    let inline = size_of::<T>();
    Ok((
        inline,
        heap,
        inline.checked_add(heap.bytes).ok_or(Overflow)?,
    ))
}
macro_rules! zero { ($($ty:ty),*) => { $(impl HeapSize for $ty { fn heap(&self)->Result<Heap,Overflow>{Ok(Heap::default())} })* }; }
zero!(bool, u8, u32, u64, usize, f64);
impl HeapSize for String {
    fn heap(&self) -> Result<Heap, Overflow> {
        Ok(Heap {
            bytes: self.capacity(),
            allocations: usize::from(self.capacity() > 0),
            slack_bytes: self.capacity() - self.len(),
        })
    }
}
impl<T: HeapSize> HeapSize for Vec<T> {
    fn heap(&self) -> Result<Heap, Overflow> {
        let bytes = self
            .capacity()
            .checked_mul(size_of::<T>())
            .ok_or(Overflow)?;
        let backing = Heap {
            bytes,
            allocations: usize::from(bytes > 0),
            slack_bytes: (self.capacity() - self.len())
                .checked_mul(size_of::<T>())
                .ok_or(Overflow)?,
        };
        self.iter()
            .try_fold(backing, |sum, value| sum.combine(value.heap()?))
    }
}
impl<T: HeapSize> HeapSize for Box<T> {
    fn heap(&self) -> Result<Heap, Overflow> {
        Heap {
            bytes: size_of::<T>(),
            allocations: usize::from(size_of::<T>() > 0),
            slack_bytes: 0,
        }
        .combine((**self).heap()?)
    }
}
impl<T: HeapSize> HeapSize for Option<T> {
    fn heap(&self) -> Result<Heap, Overflow> {
        match self {
            Some(value) => value.heap(),
            None => Ok(Heap::default()),
        }
    }
}
impl<T: HeapSize, const N: usize> HeapSize for [T; N] {
    fn heap(&self) -> Result<Heap, Overflow> {
        self.iter()
            .try_fold(Heap::default(), |sum, value| sum.combine(value.heap()?))
    }
}
impl HeapSize for Id {
    fn heap(&self) -> Result<Heap, Overflow> {
        self.0.heap()
    }
}
impl HeapSize for SchemaVersion {
    fn heap(&self) -> Result<Heap, Overflow> {
        match self {
            Self::V0_1_0 => Ok(Heap::default()),
        }
    }
}

// Exhaustive destructures/matches force a compile failure when canonical fields
// or variants change. This enumerates ownership, never defines alternate DTOs.
macro_rules! record_heap {
    ($ty:ident { $($field:ident),* }) => {
        impl HeapSize for $ty { fn heap(&self)->Result<Heap,Overflow> {
            let Self { $($field),* }=self;
            sum([$($field.heap()?),*])
        } }
    };
}
macro_rules! unit_heap {
    ($ty:ident { $($variant:ident),* }) => {
        impl HeapSize for $ty { fn heap(&self)->Result<Heap,Overflow> {
            match self { $(Self::$variant=>Ok(Heap::default())),* }
        } }
    };
}
macro_rules! payload_heap {
    ($ty:ident { $($variant:ident),* }) => {
        impl HeapSize for $ty { fn heap(&self)->Result<Heap,Overflow> {
            match self { $(Self::$variant(value)=>value.heap()),* }
        } }
    };
}

impl HeapSize for TransformState {
    fn heap(&self) -> Result<Heap, Overflow> {
        match self {
            Self::LocalOnly {} => Ok(Heap::default()),
            Self::Known { transform } => transform.heap(),
            Self::Unknown { reason } => reason.heap(),
        }
    }
}
impl HeapSize for Value {
    fn heap(&self) -> Result<Heap, Overflow> {
        match self {
            Self::Flag(v) => v.heap(),
            Self::Text(v) => v.heap(),
            Self::Number(v) => v.heap(),
            Self::Role(v) => v.heap(),
            Self::TextList(v) => v.heap(),
            Self::Identity(v) => v.heap(),
            Self::Geometry(v) => v.heap(),
            Self::Baseline { coordinate, space } => sum([coordinate.heap()?, space.heap()?]),
            Self::Quantity {
                amount,
                kind,
                source_units,
            } => sum([amount.heap()?, kind.heap()?, source_units.heap()?]),
        }
    }
}
impl HeapSize for Availability {
    fn heap(&self) -> Result<Heap, Overflow> {
        match self {
            Self::Known { value } => value.heap(),
            Self::Unknown { reason } | Self::Unsupported { reason } => reason.heap(),
            Self::Redacted {} => Ok(Heap::default()),
        }
    }
}
impl HeapSize for Property {
    fn heap(&self) -> Result<Heap, Overflow> {
        match self {
            Self::NotRequested { field } => field.heap(),
            Self::Requested {
                field,
                sensitivity,
                evidence,
                state,
            } => sum([
                field.heap()?,
                sensitivity.heap()?,
                evidence.heap()?,
                state.heap()?,
            ]),
        }
    }
}
impl HeapSize for Intent {
    fn heap(&self) -> Result<Heap, Overflow> {
        match self {
            Self::Focus {}
            | Self::Activate {}
            | Self::Scroll {}
            | Self::Submit {}
            | Self::Dismiss {} => Ok(Heap::default()),
            Self::SetChecked { value } => value.heap(),
            Self::Fill { text } | Self::Type { text } => text.heap(),
            Self::FillSecret { secret_reference } => secret_reference.heap(),
            Self::SelectOption { option } => option.heap(),
            Self::Press { key } => key.heap(),
        }
    }
}
impl HeapSize for Operation {
    fn heap(&self) -> Result<Heap, Overflow> {
        match self {
            Self::Observe { channels } => channels.heap(),
            Self::Inspect { snapshot_id } => snapshot_id.heap(),
            Self::Measure {
                snapshot_id,
                from,
                to,
            } => sum([snapshot_id.heap()?, from.heap()?, to.heap()?]),
            Self::Diff { before, after } => sum([before.heap()?, after.heap()?]),
            Self::Check {
                snapshot_id,
                expectation_id,
            } => sum([snapshot_id.heap()?, expectation_id.heap()?]),
            Self::Prepare { action } | Self::Act { action } => action.heap(),
            Self::Detach {} => Ok(Heap::default()),
        }
    }
}
impl HeapSize for Rule {
    fn heap(&self) -> Result<Heap, Overflow> {
        match self {
            Self::PropertyEquals { field, expected } => sum([field.heap()?, expected.heap()?]),
            Self::Geometry {
                operation,
                anchors,
                expected,
                comparison,
                quantity_kind,
                units,
                tolerance,
            } => sum([
                operation.heap()?,
                anchors.heap()?,
                expected.heap()?,
                comparison.heap()?,
                quantity_kind.heap()?,
                units.heap()?,
                tolerance.heap()?,
            ]),
        }
    }
}
impl HeapSize for FocusRef {
    fn heap(&self) -> Result<Heap, Overflow> {
        match self {
            Self::NotRequested {} => Ok(Heap::default()),
            Self::Known { target, evidence } => sum([target.heap()?, evidence.heap()?]),
            Self::None { evidence } => evidence.heap(),
            Self::Unknown { reason } => reason.heap(),
        }
    }
}

record_heap!(Identity { id, generation });
record_heap!(SourceKey { namespace, key });
record_heap!(PluginIdentity { id, version });
record_heap!(Context {
    schema_version,
    session_id,
    target,
    surfaces,
    scope_id,
    projection,
    fields,
    plugin,
    environment_revision
});
record_heap!(Coverage {
    status,
    scope_id,
    fields,
    omitted_count,
    unknown_count
});
record_heap!(Observation {
    id,
    source_namespace,
    channel,
    start,
    end,
    clock_domain,
    time_unit,
    freshness_basis,
    consistency_reason,
    answer_source,
    freshness,
    last_verified,
    consistency,
    coverage
});
record_heap!(Evidence {
    observation_id,
    source_namespace,
    provenance,
    method,
    uncertainty
});
record_heap!(Uncertainty {
    absolute,
    units,
    method
});
record_heap!(Space {
    id,
    kind,
    units,
    origin
});
record_heap!(Rect {
    x,
    y,
    width,
    height
});
record_heap!(Point { x, y });
record_heap!(Transform {
    from,
    to,
    affine,
    target,
    surface,
    environment_revision,
    evidence
});
record_heap!(Geometry {
    frame_kind,
    coordinate_space,
    shape,
    transform
});
record_heap!(Relation {
    kind,
    from,
    to,
    evidence
});
record_heap!(ComponentMapping {
    logical_component_key,
    members,
    declaration_source,
    provenance
});
record_heap!(ExtensionProperty {
    namespace,
    name,
    property
});
record_heap!(SourceDeclaration {
    namespace,
    name,
    state,
    sensitivity,
    source
});
record_heap!(Node {
    key,
    surface,
    native_role,
    properties,
    children,
    extensions,
    source_declarations
});
record_heap!(TextSelection {
    anchor,
    focus,
    units,
    evidence
});
record_heap!(CaptureFrame {
    observation_id,
    capture_target,
    capture_kind,
    pixel_width,
    pixel_height,
    crop_transform,
    included_surfaces,
    excluded_surfaces,
    unresolved_surfaces,
    surface_coverage,
    captures_audio,
    filter_id,
    payload_ref
});
record_heap!(Focus {
    keyboard,
    accessibility,
    active_descendant,
    text_selection,
    composition_state
});
record_heap!(SurfaceRecord {
    identity,
    native_owner,
    initiated_by,
    anchor,
    evidence
});
record_heap!(Snapshot {
    surface_records,
    id,
    revision,
    source_state,
    context,
    observations,
    nodes,
    relations,
    components,
    focus,
    captures,
    coverage
});
record_heap!(Removal { key, evidence });
record_heap!(Delta {
    base_revision,
    revision,
    source_state,
    context,
    observations,
    upsert,
    removed,
    relations,
    focus,
    coverage
});
record_heap!(BackendRef {
    session_id,
    key,
    snapshot_id,
    observation_id,
    target,
    surface
});
record_heap!(Limits {
    max_elements,
    max_depth,
    max_output_bytes,
    deadline_ms
});
record_heap!(Capability {
    channel,
    operation,
    status,
    reason
});
record_heap!(SessionDescriptor {
    allowed_scopes,
    session_id,
    plugin,
    supported_versions,
    target,
    surfaces,
    capabilities
});
record_heap!(Anchor {
    element,
    frame_kind,
    coordinate_space,
    fraction,
    axis
});
record_heap!(Action {
    id,
    context,
    backend_ref,
    intent,
    modality,
    input_space,
    required_enabled,
    authorized_scope,
    unique_match,
    resolution
});
record_heap!(Request {
    clock_domain,
    request_id,
    context,
    limits,
    freshness_policy,
    operation
});
record_heap!(Step {
    id,
    action_id,
    delivery,
    outcome,
    before_snapshot,
    after_snapshot,
    verification_observation
});
record_heap!(Transition {
    id,
    context,
    steps,
    completed_steps,
    stopped_at,
    stop_on_error
});
record_heap!(ContextConditions {
    platform,
    input_mode,
    text_scale
});
record_heap!(Expectation {
    id,
    scope_id,
    targets,
    rule,
    applies_when,
    expected_from
});
record_heap!(Finding {
    id,
    expectation_id,
    snapshot_id,
    observation_id,
    status,
    measured,
    reason
});
record_heap!(Issue {
    code,
    scope_id,
    failed_step,
    recovery_class
});
record_heap!(ExportRecord {
    purpose,
    before,
    after,
    changed,
    include_geometry,
    include_pixels,
    description
});
record_heap!(PropertyCase {
    property,
    observation
});
record_heap!(GeometryCase {
    context,
    geometry,
    observation
});
record_heap!(DeltaCase {
    base,
    update,
    source_snapshot
});
record_heap!(ActionCase { snapshot, action });
record_heap!(FindingCase {
    snapshot,
    expectation,
    finding
});
record_heap!(GoldenChain {
    before,
    action,
    delivery,
    verification,
    after,
    delta,
    expectation,
    finding,
    export
});
record_heap!(Document {
    schema_version,
    artifact
});
record_heap!(Resolution {
    evidence,
    writable,
    value_allowed,
    available_intents
});
record_heap!(ActionResult {
    snapshot,
    action,
    target_state,
    issue,
    dispatched
});
record_heap!(DeltaResult {
    base,
    update,
    issue,
    published
});
record_heap!(ClockTransform {
    from_clock,
    to_clock,
    scale,
    offset,
    source
});
record_heap!(TemporalComparison {
    observations,
    transform,
    overlap,
    atomic_claim
});
record_heap!(TransitionCase {
    transition,
    before,
    after
});
record_heap!(SessionCase {
    session,
    request,
    backend_ref
});
record_heap!(TargetCandidate {
    process_id,
    window_id,
    title,
    bounds,
    coordinate_space,
    process_continuity,
    owner_binding
});
record_heap!(ResolutionRefusal {
    requested,
    candidates,
    issue,
    dispatched
});
record_heap!(ChannelResponse {
    request_id,
    session_id,
    dispatch_sequence,
    target,
    channel,
    result
});
unit_heap!(Projection {
    Interaction,
    Design
});
unit_heap!(Channel {
    ExternalSemantics,
    RenderedCapture,
    OptInLayoutProbe
});
unit_heap!(Provenance {
    Reported,
    Derived,
    Estimated
});
unit_heap!(Freshness {
    Current,
    Stale,
    Unverified
});
unit_heap!(AnswerSource { Live, Cache });
unit_heap!(Consistency {
    Stable,
    Unstable,
    Unknown
});
unit_heap!(CoverageStatus {
    Complete,
    Partial,
    Unknown
});
unit_heap!(Sensitivity { Public, Sensitive });
unit_heap!(Unit { Px, CssPx, Pt, Dp });
unit_heap!(SpaceKind {
    Screen,
    Window,
    Surface,
    Viewport,
    Document,
    Local
});
unit_heap!(Origin {
    TopLeft,
    BottomLeft
});
unit_heap!(FrameKind {
    LayoutBounds,
    AccessibilityBounds,
    HitRegion,
    VisibleRegion,
    PaintBounds
});
unit_heap!(CaptureKind {
    WindowIsolated,
    DisplayComposited
});
unit_heap!(FreshnessBasis {
    LiveRead,
    Revalidated,
    TtlOnly,
    Unverified
});
unit_heap!(TimeUnit {
    Milliseconds,
    Seconds
});
unit_heap!(TargetState {
    Current,
    Gone,
    Unresolved
});
unit_heap!(Role {
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
unit_heap!(Field {
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
unit_heap!(RelationKind {
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
unit_heap!(InputModality {
    Semantic,
    Pointer,
    Keyboard,
    Setter,
    Touch,
    Remote
});
unit_heap!(CapabilityStatus {
    Supported,
    Partial,
    Unsupported,
    PermissionRequired
});
unit_heap!(FreshnessPolicy {
    CurrentRequired,
    CachedAllowed
});
unit_heap!(Comparison {
    Equal,
    AtLeast,
    AtMost,
    GreaterThan
});
unit_heap!(QuantityKind {
    Length,
    Area,
    Ratio
});
unit_heap!(GeometryRelation {
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
unit_heap!(CheckStatus {
    Pass,
    Fail,
    Unknown
});
unit_heap!(DeliveryStatus {
    NotDispatched,
    Accepted,
    Confirmed,
    Unknown
});
unit_heap!(Outcome {
    PendingVerification,
    Succeeded,
    Failed,
    Interrupted,
    ActionOutcomeUnknown
});
unit_heap!(ErrorCode {
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
payload_heap!(Shape {
    Rect,
    Quad,
    Polygon,
    Fragments
});
payload_heap!(Artifact {
    Request,
    Session,
    SessionContext,
    ResolutionRefusal,
    ChannelResponse,
    Capability,
    Property,
    Observation,
    Snapshot,
    Geometry,
    Delta,
    Action,
    Transition,
    Expectation,
    Finding,
    Error,
    GoldenChain,
    ActionResult,
    DeltaResult,
    TemporalComparison,
    TransitionContext
});
payload_heap!(ChannelResult { Observed, Failed });
