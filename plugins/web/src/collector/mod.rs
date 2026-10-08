//! Addressed read-only collection. Live qualification and host authority are separate.
mod acquire;
mod action;
pub use action::{CheckboxProvider, WebActionProvider};
mod bootstrap;
mod io;
mod observe;
pub(crate) mod wire;
use crate::{cdp, transport};
use std::{
    fmt,
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};
use uiblueprint_schema::model::*;

// Fixed scalar identity namespace: independent collectors never reuse snapshot IDs.
static OWNERS: AtomicU64 = AtomicU64::new(0);
fn next_owner() -> Result<u64, Failure> {
    OWNERS
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| v.checked_add(1))
        .map(|v| v + 1)
        .map_err(|_| Failure::new(ErrorKind::Limit))
}

/// Same-process worker clock supplied by its actual clock owner.
pub struct Clock {
    pub domain: Id,
    pub origin: Instant,
}
/// Caller-established authorization; no endpoint discovery or inferred generations.
pub struct Binding {
    pub session_id: Id,
    pub target: Identity,
    pub surface: Identity,
    pub cdp_session_id: Option<Id>,
    pub clock: Clock,
    pub allowed_scopes: Vec<Id>,
    pub plugin: PluginIdentity,
}
impl fmt::Debug for Binding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CollectorBinding { redacted }")
    }
}
/// Exact source node in the bound document. No name/coordinate/locator fallback.
pub struct NodeRef {
    pub reference: BackendRef,
    pub sensitivity: Sensitivity,
}
impl NodeRef {
    fn backend_id(&self) -> Result<u32, Failure> {
        let n = self
            .reference
            .key
            .key
            .0
            .parse::<u32>()
            .map_err(|_| Failure::new(ErrorKind::InvalidInput))?;
        if n == 0 || n > i32::MAX as u32 || n.to_string() != self.reference.key.key.0 {
            return Err(Failure::new(ErrorKind::InvalidInput));
        }
        Ok(n)
    }
}
pub struct Scope {
    pub scope_id: Id,
    pub nodes: Vec<NodeRef>,
}
/// Exact, case-sensitive id in the attached root document's light DOM.
pub struct DomId {
    pub id: Id,
    pub sensitivity: Sensitivity,
}
/// Explicit search allowance; max_depth also comes from the canonical request.
pub struct InitialScope {
    pub scope_id: Id,
    pub ids: Vec<DomId>,
    pub max_visited_nodes: u32,
}
/// Actual caller-observed read-only identity. Never a BackendRef/action permit.
pub struct RootSeed {
    pub session_id: Id,
    pub target: Identity,
    pub surface: Identity,
    pub document_backend_id: u32,
    pub backend_node_id: u32,
    pub sensitivity: Sensitivity,
}
pub struct RootedScope {
    pub scope_id: Id,
    pub root: RootSeed,
    pub max_visited_nodes: u32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectionStatus {
    Missing,
    Ambiguous,
    Incomplete,
    Unsupported,
    TimedOut,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SelectionReport {
    pub visited_nodes: u32,
    pub selected_nodes: usize,
}
pub struct BootstrapReport {
    pub report: Report,
    pub references: Vec<BackendRef>,
    pub selection: SelectionReport,
}
impl fmt::Debug for BootstrapReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BootstrapReport")
            .field("report", &self.report)
            .field("references", &self.references.len())
            .field("selection", &self.selection)
            .finish()
    }
}
#[derive(Clone, Copy)]
enum Plan<'a> {
    References(&'a Scope),
    Initial(&'a InitialScope),
    Rooted(&'a RootedScope, usize),
}
impl Plan<'_> {
    fn len(self) -> usize {
        match self {
            Self::References(s) => s.nodes.len(),
            Self::Initial(s) => s.ids.len(),
            Self::Rooted(_, cap) => cap,
        }
    }
}
struct Observed {
    report: Report,
    references: Vec<BackendRef>,
    selection: Option<SelectionReport>,
}
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub max_nodes: usize,
    pub max_methods: u32,
    pub max_reply_bytes: usize,
    pub max_total_reply_bytes: usize,
    pub max_text_bytes: usize,
    pub max_handle_bytes: usize,
    pub max_ax_properties: usize,
    pub io_read_bytes: usize,
    pub io_write_bytes: usize,
    pub io_work: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    InvalidInput,
    Limit,
    StaleTarget,
    ResyncRequired,
    Malformed,
    Protocol(i32),
    Cdp(cdp::ErrorKind),
    Timeout,
    PublicationStopped,
    CleanupUnconfirmed,
    Selection {
        status: SelectionStatus,
        visited_nodes: u32,
    },
}
/// No remote group remains tracked, release was acknowledged, or release could not be confirmed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RemoteCleanup {
    NotRequired,
    Released,
    Unconfirmed,
}
/// Static acquisition refusal location; never contains remote text or data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MalformedSite {
    ResolveReply = 1,
    SelectionReply = 2,
    SelectionException = 3,
    SelectionShape = 4,
    PropertiesReply = 5,
    PropertiesShape = 6,
    SelectedDescription = 7,
    ReadReply = 8,
    ReadException = 9,
    ReadShape = 10,
    ReadData = 11,
    AxReply = 12,
    AxData = 13,
    ContinuityReply = 14,
    ContinuityException = 15,
    ContinuityShape = 16,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Failure {
    pub kind: ErrorKind,
    pub malformed_site: Option<MalformedSite>,
    pub send_progress: transport::SendProgress,
    pub remote_cleanup: RemoteCleanup,
}
impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({:?})", self.kind, self.send_progress)
    }
}
impl std::error::Error for Failure {}
impl Failure {
    fn at(mut self, site: MalformedSite) -> Self {
        if self.kind == ErrorKind::Malformed && self.malformed_site.is_none() {
            self.malformed_site = Some(site);
        }
        self
    }
    fn with_cleanup(mut self, cleanup: RemoteCleanup) -> Self {
        self.remote_cleanup = cleanup;
        self
    }
    fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            malformed_site: None,
            send_progress: transport::SendProgress::NotQueued,
            remote_cleanup: RemoteCleanup::NotRequired,
        }
    }
}
/// Return Acknowledged only after the host owns/commits this canonical channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Publication {
    Acknowledged,
    Stop,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceStatus {
    NotRequested,
    Complete,
    Partial,
    Failed(ErrorKind),
}
#[derive(Clone, Copy, Debug)]
pub struct Report {
    pub dom: SourceStatus,
    pub ax: SourceStatus,
    pub dom_nodes: usize,
    pub ax_nodes: usize,
    pub visited_dom: usize,
    pub queried_ax: usize,
    pub omitted_nodes: Option<usize>,
    pub methods: u32,
    pub reply_bytes: usize,
    pub published_channels: usize,
    pub canonical_bytes: u64,
}
fn needs_ax(fields: &[Field]) -> bool {
    fields.iter().any(|field| {
        matches!(
            field,
            Field::Role
                | Field::Name
                | Field::AccessibilityName
                | Field::Description
                | Field::Value
                | Field::Required
                | Field::Enabled
                | Field::Readonly
                | Field::Checked
                | Field::Selected
                | Field::Expanded
                | Field::Focused
                | Field::Invalid
        )
    })
}
struct Budget {
    deadline: Instant,
    methods: u32,
    reply_bytes: usize,
    max_reply_bytes: usize,
}
/// Owns exactly one CDP connection. No background collection or action API.
pub struct Collector {
    client: cdp::Client,
    binding: Binding,
    limits: Limits,
    origin: Instant,
    document: u32,
    world: i32,
    invalid: bool,
    owner: u64,
    sequence: u64,
    loss_generation: u64,
    pending_invalidation: bool,
}
impl fmt::Debug for Collector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Collector {{ invalid: {}, sequence: {} }}",
            self.invalid, self.sequence
        )
    }
}
impl Collector {
    pub fn cancellation(&self) -> Option<transport::Cancellation> {
        self.client.cancellation()
    }
    /// Events never collect data; the owner clears this only after cache apply.
    pub fn pending_invalidation(&self) -> bool {
        self.pending_invalidation
    }
    /// Call only after the retained-session owner successfully applied invalidation.
    pub fn acknowledge_invalidation(&mut self) {
        self.pending_invalidation = false;
    }
    pub fn detach(&mut self) {
        self.pending_invalidation = true;
        self.invalid = true;
        self.client.detach();
    }
    pub fn clock_domain(&self) -> &Id {
        &self.binding.clock.domain
    }
    fn time(&self) -> f64 {
        self.origin.elapsed().as_secs_f64() * 1000.0
    }
}
impl Drop for Collector {
    fn drop(&mut self) {
        self.detach();
    }
}
