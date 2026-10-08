//! Canonical worker-only operations. This module is never linked into the
//! supervisor path; all decode/validation/analysis/cache work runs under its guard.
#[path = "worker_observation.rs"]
mod observation;
pub(crate) use observation::ObservationRun;

use crate::{quota_allocator as guard, worker_tape::Tape};
use std::io::Write;
use uiblueprint_engine::{
    self as engine,
    cache::{
        Allowance, CacheStore, Clock, Incoming, QuotaLedger, ReadPolicy, SessionHandle,
        SessionIdentity, SnapshotKey, StoreLimits,
    },
};
use uiblueprint_host::{HostError, HostLimits, OperationClass, authority::TargetLease};
use uiblueprint_plugin_api::{ClockReading, Limits as ObservationLimits, ObservationSession};
use uiblueprint_schema::{SchemaVersion, analysis::*, model::*};

pub struct CanonicalSession<'a> {
    target: TargetLease,
    session_id: Id,
    clock: Id,
    observations: ObservationSession,
    store: CacheStore<'a>,
    cache_session: SessionHandle<'a>,
    limits: HostLimits,
}
impl<'a> CanonicalSession<'a> {
    pub fn attach(
        bytes: &[u8],
        target: TargetLease,
        clock: Id,
        now: u64,
        ledger: &'a QuotaLedger,
        limits: HostLimits,
    ) -> Result<Self, HostError> {
        guard::phase(guard::Phase::Decode);
        let doc =
            Document::from_json(bytes, limits.input_bytes).map_err(|_| HostError::InvalidInput)?;
        let Artifact::Session(descriptor) = doc.artifact else {
            return Err(HostError::InvalidInput);
        };
        if !target.matches(&descriptor.target) {
            return Err(HostError::PermissionDenied);
        }
        guard::phase(guard::Phase::Admission);
        let grant = ledger
            .reserve(Allowance {
                bytes: limits
                    .retained_per_worker
                    .checked_sub(QuotaLedger::backing_bytes())
                    .ok_or(HostError::ResourceLimit)?,
                session_slots: 1,
            })
            .map_err(|_| HostError::ResourceLimit)?;
        let mut store = CacheStore::new(
            grant,
            StoreLimits {
                session_slots: 1,
                snapshot_slots_per_session: 16,
                revisions_per_family: 2,
                entry_bytes: 8 * 1_048_576,
                session_bytes: 16 * 1_048_576,
                retention_ms: 300_000,
            },
            clock.clone(),
            now,
        )
        .map_err(|_| HostError::ResourceLimit)?;
        let cache_session = store
            .open_session(SessionIdentity {
                session_id: descriptor.session_id.clone(),
                target: descriptor.target.clone(),
                plugin: descriptor.plugin.clone(),
            })
            .map_err(|_| HostError::ResourceLimit)?;
        let session_id = descriptor.session_id.clone();
        let observations = ObservationSession::attach(
            *descriptor,
            clock.clone(),
            ObservationLimits {
                max_frame_bytes: limits.output_bytes,
                max_in_flight: 1,
                max_pending_encoded_bytes: limits
                    .input_bytes
                    .checked_add(limits.request_output_bytes)
                    .ok_or(HostError::Overflow)?,
            },
        )
        .map_err(|_| HostError::InvalidInput)?;
        Ok(Self {
            target,
            session_id,
            clock,
            observations,
            store,
            cache_session,
            limits,
        })
    }
    fn target(&self, context: &Context) -> Result<(), HostError> {
        if !self.target.matches(&context.target) {
            Err(HostError::PermissionDenied)
        } else {
            Ok(())
        }
    }
    /// Call after the active ObservationRun borrow ends. Existing recorded bytes,
    /// evidence and expiry remain intact; the adapter owns pending-signal ACK.
    #[cfg(feature = "web")]
    pub(crate) fn invalidate_retained_session(&mut self) -> Result<usize, HostError> {
        self.store
            .invalidate_session(self.cache_session)
            .map_err(|_| HostError::InvalidState)
    }
    /// Tape(ActionCase, Act Request, explicit Expectation); the legacy SetChecked
    /// pair remains valid. Typed decoding
    /// stays inside the admitted worker. The provider separately enforces its
    /// trusted attachment's plugin/surface/scope before fresh acquisition.
    pub(crate) fn action_input(
        &self,
        bytes: &[u8],
    ) -> Result<(ActionCase, Limits, TargetLease, Id, Option<Expectation>), HostError> {
        if !self.target.permits(OperationClass::Mutation) {
            return Err(HostError::PermissionDenied);
        }
        if bytes.len() > self.limits.input_bytes {
            return Err(HostError::ResourceLimit);
        }
        guard::phase(guard::Phase::Decode);
        let tape = Tape::decode(bytes)?;
        if !matches!(tape.count(), 2 | 3) {
            return Err(HostError::InvalidInput);
        }
        let source = Document::from_json(tape.get(0)?, self.limits.input_bytes)
            .map_err(|_| HostError::InvalidInput)?;
        let request = Document::from_json(tape.get(1)?, self.limits.input_bytes)
            .map_err(|_| HostError::InvalidInput)?;
        let Artifact::Action(case) = source.artifact else {
            return Err(HostError::InvalidInput);
        };
        let Artifact::Request(request) = request.artifact else {
            return Err(HostError::InvalidInput);
        };
        let Operation::Act { action } = request.operation else {
            return Err(HostError::PermissionDenied);
        };
        self.target(&case.snapshot.context)?;
        if request.clock_domain != self.clock
            || request.context.session_id != self.session_id
            || !uiblueprint_schema::validation::contexts_compatible(
                &request.context,
                &case.snapshot.context,
            )
            || action != case.action
        {
            return Err(HostError::InvalidInput);
        }
        let expected = self.action_expectation(&tape, &case.snapshot, &case.action)?;
        Ok((
            *case,
            request.limits,
            self.target,
            self.clock.clone(),
            expected,
        ))
    }
    /// Read-only preparation has no effect authority. Only actual provider
    /// acquisition may replace the input's unknown Resolution facts.
    pub(crate) fn prepare_input(
        &self,
        bytes: &[u8],
    ) -> Result<(Snapshot, Request, Id, Option<Expectation>), HostError> {
        if bytes.len() > self.limits.input_bytes {
            return Err(HostError::ResourceLimit);
        }
        guard::phase(guard::Phase::Decode);
        let tape = Tape::decode(bytes)?;
        if !matches!(tape.count(), 2 | 3) {
            return Err(HostError::InvalidInput);
        }
        let source = Document::from_json(tape.get(0)?, self.limits.input_bytes)
            .map_err(|_| HostError::InvalidInput)?;
        let request = Document::from_json(tape.get(1)?, self.limits.input_bytes)
            .map_err(|_| HostError::InvalidInput)?;
        let snapshot = match source.artifact {
            Artifact::Snapshot(snapshot) => snapshot,
            Artifact::ChannelResponse(response) => match response.result {
                ChannelResult::Observed(snapshot) => snapshot,
                ChannelResult::Failed(_) => return Err(HostError::ResyncRequired),
            },
            _ => return Err(HostError::InvalidInput),
        };
        let Artifact::Request(request) = request.artifact else {
            return Err(HostError::InvalidInput);
        };
        let Operation::Prepare { action } = &request.operation else {
            return Err(HostError::PermissionDenied);
        };
        self.target(&snapshot.context)?;
        let reference = &action.backend_ref;
        if request.clock_domain != self.clock
            || request.context.session_id != self.session_id
            || !uiblueprint_schema::validation::contexts_compatible(
                &request.context,
                &snapshot.context,
            )
            || reference.session_id != self.session_id
            || reference.target != snapshot.context.target
            || reference.snapshot_id != snapshot.id
            || action.authorized_scope != snapshot.context.scope_id
            || !matches!(
                (&action.intent, action.modality),
                (Intent::SetChecked { .. }, InputModality::Setter)
                    | (Intent::Focus {}, InputModality::Semantic)
                    | (Intent::Type { .. }, InputModality::Keyboard)
                    | (Intent::Fill { .. }, InputModality::Setter)
                    | (Intent::Activate {}, InputModality::Semantic)
            )
            || snapshot
                .nodes
                .iter()
                .find(|node| node.key == reference.key)
                .is_none_or(|node| node.surface != reference.surface)
            || snapshot
                .observations
                .iter()
                .find(|o| o.id == reference.observation_id)
                .is_none_or(|o| o.source_namespace != reference.key.namespace)
        {
            return Err(HostError::InvalidInput);
        }
        let expected = self.action_expectation(&tape, &snapshot, action)?;
        Ok((*snapshot, *request, self.clock.clone(), expected))
    }
    fn action_expectation(
        &self,
        tape: &Tape<'_>,
        snapshot: &Snapshot,
        action: &Action,
    ) -> Result<Option<Expectation>, HostError> {
        if tape.count() == 2 {
            return if matches!(action.intent, Intent::SetChecked { .. }) {
                Ok(None)
            } else {
                Err(HostError::InvalidInput)
            };
        }
        let document = Document::from_json(tape.get(2)?, self.limits.input_bytes)
            .map_err(|_| HostError::InvalidInput)?;
        let Artifact::Expectation(expected) = document.artifact else {
            return Err(HostError::InvalidInput);
        };
        let Rule::PropertyEquals {
            field,
            expected: value,
        } = &expected.rule
        else {
            return Err(HostError::InvalidInput);
        };
        if expected.scope_id != action.authorized_scope
            || expected.targets.len() != 1
            || !snapshot.context.fields.contains(field)
            || !snapshot.nodes.iter().any(|n| n.key == expected.targets[0])
            || expected.applies_when.platform.is_some()
            || expected.applies_when.text_scale.is_some()
            || expected
                .applies_when
                .input_mode
                .is_some_and(|mode| mode != action.modality)
        {
            return Err(HostError::InvalidInput);
        }
        // Bind the first concrete Web forms conditions before SDK preparation,
        // using the same intent semantics as kernel/provider verification.
        let valid = match &action.intent {
            Intent::Focus {} => {
                *field == Field::Focused
                    && *value == Value::Flag(true)
                    && expected.targets[0] == action.backend_ref.key
            }
            Intent::Type { .. } | Intent::Fill { .. } => {
                *field == Field::Value
                    && matches!(value, Value::Text(_))
                    && expected.targets[0] == action.backend_ref.key
            }
            Intent::SetChecked { value: wanted } => {
                *field == Field::Checked
                    && *value == Value::Flag(*wanted)
                    && expected.targets[0] == action.backend_ref.key
            }
            _ => true,
        };
        if !valid {
            return Err(HostError::InvalidInput);
        }
        Ok(Some(*expected))
    }
    pub fn execute(
        &mut self,
        class: OperationClass,
        input: &[u8],
        now: fn() -> u64,
        output: &mut impl Write,
        format: u8,
        partition: u8,
    ) -> Result<u64, HostError> {
        if !self.target.permits(class) {
            return Err(HostError::PermissionDenied);
        }
        match class {
            OperationClass::Measure | OperationClass::Check => self.analyze(class, input, output),
            OperationClass::Verify => {
                guard::phase(guard::Phase::Decode);
                let doc = AnalysisDocument::from_json(input, self.limits.input_bytes)
                    .map_err(|_| HostError::InvalidInput)?;
                match &doc.artifact {
                    AnalysisArtifact::Measurement(c) => self.target(&c.snapshot.context)?,
                    AnalysisArtifact::GeometryCheck(c) => self.target(&c.snapshot.context)?,
                    _ => return Err(HostError::InvalidInput),
                }
                guard::phase(guard::Phase::Validate);
                engine::verify_analysis_result(&doc).map_err(|_| HostError::InvalidInput)?;
                guarded_encode(&doc, output)?;
                Ok(0)
            }
            OperationClass::Validate => {
                guard::phase(guard::Phase::Decode);
                if format == 0 {
                    let document = Document::from_json(input, self.limits.input_bytes)
                        .map_err(|_| HostError::InvalidInput)?;
                    match &document.artifact {
                        Artifact::Snapshot(s) => self.target(&s.context)?,
                        Artifact::Geometry(c) => self.target(&c.context)?,
                        Artifact::Expectation(_)
                        | Artifact::Finding(_)
                        | Artifact::Property(_)
                        | Artifact::Observation(_)
                        | Artifact::Error(_) => (),
                        _ => return Err(HostError::PermissionDenied),
                    }
                } else if format == 1 {
                    let document = AnalysisDocument::from_json(input, self.limits.input_bytes)
                        .map_err(|_| HostError::InvalidInput)?;
                    if !matches!(
                        document.artifact,
                        AnalysisArtifact::GeometryQuery(_) | AnalysisArtifact::EvaluationInput(_)
                    ) {
                        return Err(HostError::InvalidInput); // Imported results use Verify, never schema-only success.
                    }
                } else {
                    return Err(HostError::InvalidInput);
                }
                let _publication = guard::PublicationGuard::enter(guard::Phase::Validate);
                output
                    .write_all(input)
                    .map_err(|_| HostError::ResourceLimit)?;
                Ok(0)
            }
            OperationClass::Retain => self.retain(input, now, output, partition),
            OperationClass::Replay => self.replay(input, now, output, partition),
            _ => Err(HostError::InvalidInput),
        }
    }
    fn analyze(
        &self,
        class: OperationClass,
        input: &[u8],
        output: &mut impl Write,
    ) -> Result<u64, HostError> {
        guard::phase(guard::Phase::Decode);
        let tape = Tape::decode(input)?;
        if tape.count() != 3 {
            return Err(HostError::InvalidInput);
        }
        let source = Document::from_json(tape.get(0)?, self.limits.input_bytes)
            .map_err(|_| HostError::InvalidInput)?;
        let Artifact::Snapshot(snapshot) = source.artifact else {
            return Err(HostError::InvalidInput);
        };
        self.target(&snapshot.context)?;
        let evaluation = AnalysisDocument::from_json(tape.get(2)?, self.limits.input_bytes)
            .map_err(|_| HostError::InvalidInput)?;
        let AnalysisArtifact::EvaluationInput(evaluation) = evaluation.artifact else {
            return Err(HostError::InvalidInput);
        };
        guard::phase(guard::Phase::Validate);
        let artifact = if class == OperationClass::Measure {
            let request = AnalysisDocument::from_json(tape.get(1)?, self.limits.input_bytes)
                .map_err(|_| HostError::InvalidInput)?;
            let AnalysisArtifact::GeometryQuery(query) = request.artifact else {
                return Err(HostError::InvalidInput);
            };
            let result = engine::measure_query_bound(&snapshot, &query, &evaluation)
                .map_err(|_| HostError::InvalidInput)?;
            AnalysisArtifact::Measurement(Box::new(MeasurementCase {
                snapshot: *snapshot,
                query: *query,
                evaluation: *evaluation,
                result,
            }))
        } else {
            let request = Document::from_json(tape.get(1)?, self.limits.input_bytes)
                .map_err(|_| HostError::InvalidInput)?;
            let Artifact::Expectation(expectation) = request.artifact else {
                return Err(HostError::InvalidInput);
            };
            let result = engine::check_bound(&snapshot, &expectation, &evaluation)
                .map_err(|_| HostError::InvalidInput)?;
            AnalysisArtifact::GeometryCheck(Box::new(GeometryCheckCase {
                snapshot: *snapshot,
                expectation: *expectation,
                evaluation: *evaluation,
                measurement: result.measurement,
                finding: result.finding,
            }))
        };
        let document = AnalysisDocument {
            schema_version: AnalysisVersion::CURRENT,
            artifact,
        };
        guarded_encode(&document, output)?;
        Ok(0)
    }
    fn retain(
        &mut self,
        input: &[u8],
        now: fn() -> u64,
        output: &mut impl Write,
        partition: u8,
    ) -> Result<u64, HostError> {
        guard::phase(guard::Phase::Decode);
        let doc = Document::from_json(input, self.limits.input_bytes)
            .map_err(|_| HostError::InvalidInput)?;
        let Artifact::Snapshot(snapshot) = doc.artifact else {
            return Err(HostError::InvalidInput);
        };
        self.target(&snapshot.context)?;
        if snapshot.context.session_id != self.session_id {
            return Err(HostError::InvalidInput);
        }
        guard::phase(guard::Phase::Admission);
        let admitted = self
            .store
            .admit_full(
                self.cache_session,
                Incoming {
                    snapshot: *snapshot,
                    partition: channels(partition)?,
                },
                Clock {
                    domain: &self.clock,
                    milliseconds: now(),
                },
            )
            .map_err(|_| HostError::ResourceLimit)?;
        let stored = self
            .store
            .read(
                admitted.handle,
                ReadPolicy::Recorded,
                Clock {
                    domain: &self.clock,
                    milliseconds: now(),
                },
            )
            .map_err(|_| HostError::ResyncRequired)?;
        snapshot_encode(stored.snapshot, output)?;
        Ok(stored.snapshot.revision)
    }
    fn replay(
        &mut self,
        input: &[u8],
        now: fn() -> u64,
        output: &mut impl Write,
        partition: u8,
    ) -> Result<u64, HostError> {
        guard::phase(guard::Phase::Decode);
        let tape = Tape::decode(input)?;
        if tape.count() != 2 {
            return Err(HostError::InvalidInput);
        }
        let bytes = tape.get(0)?;
        let doc = Document::from_json(bytes, self.limits.input_bytes).map_err(|error| {
            use uiblueprint_schema::validation::ValidationError;
            // Semantic validation can fail before the artifact check below.
            // On compatibility errors only, the SAME canonical serde type/decoder
            // establishes the artifact kind; it never admits the invalid record.
            // All allocation remains under the worker guard. Successful requests
            // and other validation failures do not decode again.
            if matches!(
                error,
                ValidationError::IncompatibleContext | ValidationError::ResyncRequired
            ) && matches!(
                serde_json::from_slice::<Document>(bytes),
                Ok(Document {
                    artifact: Artifact::Delta(_),
                    ..
                })
            ) {
                HostError::ResyncRequired
            } else {
                HostError::InvalidInput
            }
        })?;
        let Artifact::Delta(case) = doc.artifact else {
            return Err(HostError::InvalidInput);
        };
        self.target(&case.base.context)?;
        // The supplied base is checked against real retained state; a missing/old
        // worker epoch cannot resurrect a lost cache base from the request body.
        let partition_values = channels(partition)?;
        let stored_base = self
            .store
            .lookup(
                self.cache_session,
                SnapshotKey {
                    context: &case.base.context,
                    partition: &partition_values,
                    snapshot_id: &case.base.id,
                    revision: case.base.revision,
                },
                ReadPolicy::Recorded,
                Clock {
                    domain: &self.clock,
                    milliseconds: now(),
                },
            )
            .map_err(|_| HostError::ResyncRequired)?;
        if stored_base.snapshot != &case.base {
            return Err(HostError::InvalidInput);
        }
        let base = stored_base.handle;
        let id: Id = serde_json::from_slice(tape.get(1)?).map_err(|_| HostError::InvalidInput)?;
        guard::phase(guard::Phase::Replay);
        let admitted = self
            .store
            .apply_delta(
                base,
                channels(partition)?,
                &case.update,
                id,
                Clock {
                    domain: &self.clock,
                    milliseconds: now(),
                },
            )
            .map_err(|_| HostError::ResyncRequired)?;
        let stored = self
            .store
            .read(
                admitted.handle,
                ReadPolicy::Recorded,
                Clock {
                    domain: &self.clock,
                    milliseconds: now(),
                },
            )
            .map_err(|_| HostError::ResyncRequired)?;
        snapshot_encode(stored.snapshot, output)?;
        Ok(stored.snapshot.revision)
    }
    /// Each canonical response is verified by the real ObservationSession and
    /// published/ACKed through the caller before receiving the next channel.
    pub fn observe(
        &mut self,
        input: &[u8],
        now: fn() -> u64,
        requested: u8,
        mut publish: impl FnMut(u8, &[u8]) -> Result<(), HostError>,
    ) -> Result<(), HostError> {
        guard::phase(guard::Phase::Decode);
        let tape = Tape::decode(input)?;
        if tape.count() < 2 {
            return Err(HostError::InvalidInput);
        }
        let request = Document::from_json(tape.get(0)?, self.limits.input_bytes)
            .map_err(|_| HostError::InvalidInput)?;
        let Artifact::Request(request) = request.artifact else {
            return Err(HostError::InvalidInput);
        };
        let Operation::Observe { channels } = &request.operation else {
            return Err(HostError::PermissionDenied);
        };
        let mask = channels.iter().fold(0_u8, |mask, channel| {
            mask | match channel {
                Channel::ExternalSemantics => 1,
                Channel::RenderedCapture => 2,
                Channel::OptInLayoutProbe => 4,
            }
        });
        if mask != requested {
            return Err(HostError::InvalidInput);
        }
        drop(request);
        let clock = ClockReading {
            domain: self.clock.clone(),
            milliseconds: now(),
        };
        let ticket = self
            .observations
            .begin(tape.get(0)?, &clock)
            .map_err(|_| HostError::InvalidInput)?;
        let result = (|| -> Result<(), HostError> {
            for index in 1..tape.count() {
                let bytes = tape.get(index)?;
                let doc = Document::from_json(bytes, self.limits.output_bytes)
                    .map_err(|_| HostError::InvalidInput)?;
                let Artifact::ChannelResponse(response) = doc.artifact else {
                    return Err(HostError::InvalidInput);
                };
                let slot = match response.channel {
                    Channel::ExternalSemantics => 0,
                    Channel::RenderedCapture => 1,
                    Channel::OptInLayoutProbe => 2,
                };
                let observed = matches!(response.result, ChannelResult::Observed(_));
                drop(response);
                let clock = ClockReading {
                    domain: self.clock.clone(),
                    milliseconds: now(),
                };
                self.observations
                    .receive(&ticket, bytes, &clock)
                    .map_err(|_| HostError::InvalidInput)?;
                if observed {
                    publish(slot, bytes)?;
                } else {
                    let _ = self.observations.cancel(&ticket);
                    return Err(HostError::WorkerFailed);
                }
            }
            let clock = ClockReading {
                domain: self.clock.clone(),
                milliseconds: now(),
            };
            self.observations
                .complete(&ticket, &clock)
                .map_err(|_| HostError::InvalidInput)?;
            Ok(())
        })();
        if result.is_err() {
            let _ = self.observations.cancel(&ticket);
        }
        result
    }
}
fn guarded_encode(value: &impl serde::Serialize, output: &mut impl Write) -> Result<(), HostError> {
    guard::phase(guard::Phase::Encode);
    let _publication = guard::PublicationGuard::enter(guard::Phase::Validate);
    serde_json::to_writer(&mut *output, value).map_err(|_| HostError::ResourceLimit)
}
fn snapshot_encode(snapshot: &Snapshot, output: &mut impl Write) -> Result<(), HostError> {
    // Only a borrowed versioned envelope is added; the retained graph is never
    // cloned. The resulting bytes use the existing core Document/Snapshot shape.
    #[derive(serde::Serialize)]
    struct Envelope<'a> {
        schema_version: SchemaVersion,
        artifact: ArtifactRef<'a>,
    }
    #[derive(serde::Serialize)]
    struct ArtifactRef<'a> {
        kind: &'static str,
        data: &'a Snapshot,
    }
    guarded_encode(
        &Envelope {
            schema_version: SchemaVersion::CURRENT,
            artifact: ArtifactRef {
                kind: "snapshot",
                data: snapshot,
            },
        },
        output,
    )
}

fn channels(mask: u8) -> Result<Vec<Channel>, HostError> {
    if mask == 0 || mask & !7 != 0 {
        return Err(HostError::InvalidInput);
    }
    Ok([
        Channel::ExternalSemantics,
        Channel::RenderedCapture,
        Channel::OptInLayoutProbe,
    ]
    .into_iter()
    .enumerate()
    .filter(|(i, _)| mask & (1 << i) != 0)
    .map(|(_, c)| c)
    .collect())
}
