//! Shared strict operator connection and bounded host I/O for explicit CLI calls.
use crate::Failure;
use serde::{Deserialize, Deserializer};
use std::{
    fs,
    path::Path,
    thread,
    time::{Duration, Instant},
};
#[cfg(feature = "web")]
use uiblueprint_host::worker_tape;
use uiblueprint_host::{
    HostError, HostLimits,
    host_types::{HostCompletion, HostEvent},
    process::DarwinPlatform,
    process_api::SpawnSpec,
    supervisor::RuntimeHost,
};
use uiblueprint_schema::model::*;

// Explicit trusted configuration only. This duplicates no canonical graph;
// HostLimits itself intentionally has no public serialized configuration.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Profile {
    pub(crate) workers: usize,
    pub(crate) worker_bytes: usize,
    pub(crate) publication_reserve: usize,
    pub(crate) bootstrap_bytes: usize,
    pub(crate) parent_bytes: usize,
    pub(crate) input_bytes: usize,
    pub(crate) ingress_bytes: usize,
    pub(crate) output_bytes: usize,
    pub(crate) request_output_bytes: usize,
    pub(crate) completion_groups: usize,
    pub(crate) control_bytes: usize,
    pub(crate) cleanup_ms: u64,
    pub(crate) retained_domain_bytes: usize,
    pub(crate) retained_per_worker: usize,
    pub(crate) main_stack_bytes: usize,
    pub(crate) watchdog_stack_bytes: usize,
}
impl Profile {
    pub(crate) fn limits(self) -> Result<HostLimits, Failure> {
        HostLimits {
            workers: self.workers,
            worker_bytes: self.worker_bytes,
            publication_reserve: self.publication_reserve,
            bootstrap_bytes: self.bootstrap_bytes,
            parent_bytes: self.parent_bytes,
            input_bytes: self.input_bytes,
            ingress_bytes: self.ingress_bytes,
            output_bytes: self.output_bytes,
            request_output_bytes: self.request_output_bytes,
            completion_groups: self.completion_groups,
            control_bytes: self.control_bytes,
            cleanup_ms: self.cleanup_ms,
            retained_domain_bytes: self.retained_domain_bytes,
            retained_per_worker: self.retained_per_worker,
            main_stack_bytes: self.main_stack_bytes,
            watchdog_stack_bytes: self.watchdog_stack_bytes,
        }
        .validate()
        .map_err(host_error)
    }
}
#[derive(Deserialize)]
#[serde(tag = "backend", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum Provider {
    NativeFixture {
        helper_executable: std::path::PathBuf,
        configuration: String,
        channels: u8,
    },
    #[cfg(feature = "web")]
    Web {
        #[serde(deserialize_with = "object")]
        setup: Box<uiblueprint_host::web_config::WebSetup>,
        #[serde(deserialize_with = "object")]
        selection: uiblueprint_host::web_config::WebSelection,
    },
    #[cfg(not(feature = "web"))]
    Web {
        setup: serde::de::IgnoredAny,
        selection: serde::de::IgnoredAny,
    },
    #[serde(other)]
    Unsupported,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Connection {
    pub(crate) connection_version: String,
    pub(crate) target: Identity,
    pub(crate) session: SessionDescriptor,
    #[serde(deserialize_with = "object")]
    pub(crate) host_limits: Profile,
    pub(crate) attach_deadline_ms: u64,
    pub(crate) provider: Provider,
}
pub(crate) fn load(path: &Path, remaining: &mut usize) -> Result<Connection, Failure> {
    let bytes = crate::input::read(path, remaining)?;
    if bytes.iter().find(|c| !c.is_ascii_whitespace()) != Some(&b'{') {
        return Err(Failure::invalid("invalid_connection"));
    }
    let connection: Connection =
        serde_json::from_slice(&bytes).map_err(|_| Failure::invalid("invalid_connection"))?;
    if connection.connection_version != "1.0.0" {
        return Err(Failure::invalid("invalid_connection_version"));
    }
    Ok(connection)
}
// serde derives can accept sequence-form structs; local connection records
// must remain objects just like the existing canonical records they embed.
fn object<'de, D: Deserializer<'de>, T: Deserialize<'de>>(d: D) -> Result<T, D::Error> {
    struct Map<T>(std::marker::PhantomData<T>);
    impl<'de, T: Deserialize<'de>> serde::de::Visitor<'de> for Map<T> {
        type Value = T;
        fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("object")
        }
        fn visit_map<A: serde::de::MapAccess<'de>>(self, access: A) -> Result<T, A::Error> {
            T::deserialize(serde::de::value::MapAccessDeserializer::new(access))
        }
    }
    d.deserialize_map(Map(std::marker::PhantomData))
}
pub(crate) fn host_error(error: HostError) -> Failure {
    use HostError::*;
    match error {
        InvalidLimits | ResourceLimit | Overflow | InvalidInput => {
            Failure::invalid("observe_invalid_or_limit")
        }
        PermissionDenied | ResyncRequired | StaleOperation | Busy | DeadlineExpired => Failure {
            code: "observe_unavailable",
            exit: 4,
        },
        AllocationFailure
        | SystemAllocationFailure
        | Io
        | WorkerFailed
        | InvalidState
        | InvalidControl
        | CleanupPending => Failure {
            code: "observe_worker_or_cleanup_failure",
            exit: 1,
        },
    }
}
pub(crate) fn until(ms: u64) -> Result<Instant, Failure> {
    if ms == 0 {
        return Err(Failure::invalid("invalid_deadline"));
    }
    Instant::now()
        .checked_add(Duration::from_millis(ms))
        .ok_or(Failure::invalid("invalid_deadline"))
}
pub(crate) fn executable(path: &Path) -> Result<SpawnSpec, Failure> {
    let spec = SpawnSpec::new(path).map_err(host_error)?;
    if !fs::metadata(path).map_err(|_| Failure::io())?.is_file() {
        return Err(Failure::io());
    }
    Ok(spec)
}
pub(crate) fn next<'a>(
    host: &mut RuntimeHost<'a, DarwinPlatform>,
    end: Instant,
) -> Result<HostEvent<'a>, Failure> {
    loop {
        match host.next_event().map_err(host_error)? {
            HostEvent::Pending
            | HostEvent::HelperClosed { .. }
            | HostEvent::HelperCleanupPending { .. } => (),
            event => return Ok(event),
        }
        if Instant::now() >= end {
            return Err(host_error(HostError::DeadlineExpired));
        }
        thread::sleep(Duration::from_millis(1));
    }
}
pub(crate) fn shutdown<'a>(
    host: &mut RuntimeHost<'a, DarwinPlatform>,
    ms: u64,
    completion: &mut Option<HostCompletion<'a>>,
) -> Result<(), Failure> {
    let end = until(ms)?;
    loop {
        match host.shutdown().map_err(host_error)? {
            HostEvent::ShutdownComplete => return Ok(()),
            HostEvent::Complete(c) => *completion = Some(c),
            HostEvent::CleanupPending { .. } | HostEvent::HelperCleanupPending { .. } => {
                return Err(host_error(HostError::CleanupPending));
            }
            _ => (),
        }
        if Instant::now() >= end {
            return Err(host_error(HostError::CleanupPending));
        }
        thread::sleep(Duration::from_millis(1));
    }
}
pub(crate) fn encoded(document: &Document, limit: usize) -> Result<Vec<u8>, Failure> {
    // Uses the existing CLI bounded writer, not an unbounded payload clone.
    crate::output::trusted_input(document, limit)
}
#[cfg(feature = "web")]
pub(crate) fn tape_len(parts: &[&[u8]]) -> Result<usize, Failure> {
    parts
        .iter()
        .try_fold(8 + worker_tape::SEGMENTS * 8, |n, part| {
            n.checked_add(part.len())
        })
        .ok_or(Failure::invalid("input_limit"))
}
