use super::*;
use crate::normalize;
use serde::Serialize;
use std::{io, time::Duration};
use uiblueprint_schema::SchemaVersion;

impl Collector {
    /// The caller supplies its REAL admitted host dispatch sequence. Each callback
    /// moves one canonical Document; acknowledge only after publication/ACK. Earlier
    /// callbacks remain caller-owned if later collection, cancellation or publication fails.
    pub fn observe(
        &mut self,
        request: &Request,
        scope: &Scope,
        dispatch_sequence: u64,
        deadline: Instant,
        publish: impl FnMut(Document) -> Publication,
    ) -> Result<Report, Failure> {
        self.validate_request(request, scope)?;
        self.observe_plan(
            request,
            Plan::References(scope),
            dispatch_sequence,
            deadline,
            publish,
        )
        .map(|result| result.report)
    }
    pub(super) fn observe_plan(
        &mut self,
        request: &Request,
        plan: Plan<'_>,
        dispatch_sequence: u64,
        deadline: Instant,
        mut publish: impl FnMut(Document) -> Publication,
    ) -> Result<Observed, Failure> {
        let mut references = Vec::new();
        let mut selection = None;
        if dispatch_sequence == 0 {
            return Err(Failure::new(ErrorKind::InvalidInput));
        }
        let own_deadline = Instant::now()
            .checked_add(Duration::from_millis(request.limits.deadline_ms))
            .ok_or(Failure::new(ErrorKind::InvalidInput))?;
        let max_reply_bytes = usize::try_from(request.limits.max_output_bytes)
            .map_err(|_| Failure::new(ErrorKind::Limit))?
            .min(self.limits.max_total_reply_bytes);
        let mut budget = Budget {
            deadline: deadline.min(own_deadline),
            methods: 0,
            reply_bytes: 0,
            max_reply_bytes,
        };
        self.check(&budget)?;
        self.sequence = self
            .sequence
            .checked_add(1)
            .ok_or(Failure::new(ErrorKind::Limit))?;
        let Operation::Observe { channels } = &request.operation else {
            return Err(Failure::new(ErrorKind::InvalidInput));
        };
        let mut report = Report {
            dom: SourceStatus::NotRequested,
            ax: SourceStatus::NotRequested,
            dom_nodes: 0,
            ax_nodes: 0,
            visited_dom: 0,
            queried_ax: 0,
            omitted_nodes: if plan.len() == 0 { Some(0) } else { None },
            methods: 0,
            reply_bytes: 0,
            published_channels: 0,
            canonical_bytes: 0,
        };
        // Canonical channel order is explicit. DOM and AX are sources of ONE external channel.
        if channels.contains(&Channel::ExternalSemantics) {
            let needed = if plan.len() == 0 {
                6
            } else {
                plan.len()
                    .checked_mul(
                        if needs_ax(&request.context.fields) {
                            3
                        } else {
                            2
                        } + usize::from(matches!(plan, Plan::Rooted(_, _))),
                    )
                    .and_then(|n| {
                        n.checked_add(match plan {
                            Plan::Initial(_) => 11,
                            Plan::Rooted(_, _) => 13,
                            Plan::References(_) => 9,
                        })
                    })
                    .ok_or(Failure::new(ErrorKind::Limit))?
            };
            if needed > self.limits.max_methods as usize {
                return Err(Failure::new(ErrorKind::Limit));
            }
            let records = self.collect(plan, request, &mut budget)?;
            report.dom_nodes = records.dom.len();
            report.ax_nodes = records.ax.len();
            report.visited_dom = records.dom.len();
            report.queried_ax = records.ax_queries;
            report.dom = if plan.len() == 0 {
                SourceStatus::Complete
            } else {
                SourceStatus::Partial
            };
            report.ax = records.ax_status;
            selection = records.selection;
            let snapshot = self.normalize(request, plan, records)?;
            if matches!(plan, Plan::Initial(_) | Plan::Rooted(_, _)) {
                references = current_references(&snapshot)?;
            }
            let response = document(
                request,
                dispatch_sequence,
                Channel::ExternalSemantics,
                ChannelResult::Observed(Box::new(snapshot)),
            );
            let bytes = bounded_document(
                &response,
                request.limits.max_output_bytes - report.canonical_bytes,
            )?;
            report.canonical_bytes += bytes;
            self.check(&budget)?;
            if publish(response) != Publication::Acknowledged {
                self.detach();
                return Err(Failure::new(ErrorKind::PublicationStopped));
            }
            report.published_channels += 1;
        }
        for &channel in channels {
            if channel == Channel::ExternalSemantics {
                continue;
            }
            self.check(&budget)?;
            let response = document(
                request,
                dispatch_sequence,
                channel,
                ChannelResult::Failed(Issue {
                    code: ErrorCode::Unsupported,
                    scope_id: request.context.scope_id.clone(),
                    failed_step: None,
                    recovery_class: normalize::id("web-channel-not-implemented"),
                }),
            );
            let bytes = bounded_document(
                &response,
                request.limits.max_output_bytes - report.canonical_bytes,
            )?;
            report.canonical_bytes += bytes;
            if publish(response) != Publication::Acknowledged {
                self.detach();
                return Err(Failure::new(ErrorKind::PublicationStopped));
            }
            report.published_channels += 1;
        }
        self.check(&budget)?;
        report.methods = budget.methods;
        report.reply_bytes = budget.reply_bytes;
        Ok(Observed {
            report,
            references,
            selection,
        })
    }
}

fn document(request: &Request, sequence: u64, channel: Channel, result: ChannelResult) -> Document {
    Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::ChannelResponse(Box::new(ChannelResponse {
            request_id: request.request_id.clone(),
            session_id: request.context.session_id.clone(),
            dispatch_sequence: sequence,
            target: request.context.target.clone(),
            channel,
            result,
        })),
    }
}
pub(super) fn bounded_document(document: &Document, cap: u64) -> Result<u64, Failure> {
    struct Count {
        written: u64,
        cap: u64,
    }
    impl io::Write for Count {
        fn write(&mut self, b: &[u8]) -> io::Result<usize> {
            self.written = self
                .written
                .checked_add(b.len() as u64)
                .filter(|&n| n <= self.cap)
                .ok_or_else(|| io::Error::other("bounded canonical output"))?;
            Ok(b.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    let mut count = Count { written: 0, cap };
    document
        .serialize(&mut serde_json::Serializer::new(&mut count))
        .map_err(|_| Failure::new(ErrorKind::Limit))?;
    Ok(count.written)
}

fn current_references(snapshot: &Snapshot) -> Result<Vec<BackendRef>, Failure> {
    let dom = snapshot
        .observations
        .iter()
        .find(|o| o.source_namespace.0 == "web.dom")
        .ok_or(Failure::new(ErrorKind::Malformed))?;
    let mut refs = Vec::new();
    refs.try_reserve_exact(snapshot.nodes.len())
        .map_err(|_| Failure::new(ErrorKind::Limit))?;
    for node in &snapshot.nodes {
        if node.key.namespace.0 == "web.dom" {
            refs.push(BackendRef {
                session_id: snapshot.context.session_id.clone(),
                key: node.key.clone(),
                snapshot_id: snapshot.id.clone(),
                observation_id: dom.id.clone(),
                target: snapshot.context.target.clone(),
                surface: node.surface.clone(),
            });
        }
    }
    Ok(refs)
}
