//! Single-process synthetic allocation experiment, not production allocation policy.
#[path = "../../../tests/bridges/resources/count_alloc.rs"]
mod allocation;
#[path = "../../../tests/bridges/resources/owned_memory.rs"]
mod owned;
#[path = "../../../tests/bridges/resources/synthetic.rs"]
mod synthetic;
// This measurement selects the shared framing function; its other public entry
// points are exercised by D02 callers/common_support tests, not this executable.
#[allow(dead_code)]
#[path = "../../../tests/bridges/common/session_support.rs"]
mod common;

use std::{hint::black_box, io::BufReader, mem::size_of};
use uiblueprint_plugin_api::{ClockReading, Limits as SessionLimits, ObservationSession};
use uiblueprint_schema::{SchemaVersion, model::*};

#[global_allocator]
static ALLOCATOR: allocation::CountingSystem = allocation::CountingSystem;

fn stats(s: allocation::Stats) -> serde_json::Value {
    serde_json::json!({"baseline_live_requested":s.baseline,"retained_delta":s.retained_delta,"peak_above_baseline":s.peak_above_baseline})
}
fn allocator_self_check() -> Result<(), Box<dyn std::error::Error>> {
    let baseline = allocation::live();
    let mark = allocation::begin();
    let mut bytes = vec![0_u8; 1024];
    black_box(&bytes);
    bytes.reserve_exact(4096);
    black_box(&bytes);
    let measured = allocation::finish(mark)?;
    assert_eq!(measured.retained_delta, bytes.capacity() as i128);
    assert!(measured.peak_above_baseline >= bytes.capacity());
    drop(bytes);
    assert_eq!(allocation::live(), baseline);
    Ok(())
}
fn parser_row(
    name: &str,
    wire: &[u8],
    expected_valid: bool,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let mark = allocation::begin();
    let result = Document::from_json(wire, 4_194_304);
    let measured = allocation::finish(mark)?;
    assert_eq!(result.is_ok(), expected_valid, "synthetic case validity");
    let capacity = match &result {
        Ok(doc) => Some(owned::owned(doc).map_err(|_| "owned overflow")?),
        Err(_) => None,
    };
    if let Some((_, heap, _)) = capacity {
        assert_eq!(
            measured.retained_delta, heap.bytes as i128,
            "allocator vs exhaustive ownership"
        );
    } else {
        assert_eq!(
            measured.retained_delta, 0,
            "invalid record retains no parsed object"
        );
    }
    let encoded = if let Ok(document) = &result {
        let mark = allocation::begin();
        let output = serde_json::to_vec(document)?;
        let measured = allocation::finish(mark)?;
        Some(
            serde_json::json!({"bytes":output.len(),"capacity":output.capacity(),"allocation":stats(measured)}),
        )
    } else {
        None
    };
    let owned=capacity.map(|(inline,heap,total)|serde_json::json!({"inline":inline,"heap":heap.bytes,"total":total,"allocations":heap.allocations,"slack":heap.slack_bytes}));
    Ok(
        serde_json::json!({"case":name,"wire_bytes":wire.len(),"valid":expected_valid,"owned":owned,"parser":stats(measured),"serialization":encoded}),
    )
}
fn request(snapshot: &Snapshot, nodes: u32, depth: u32) -> Request {
    Request {
        clock_domain: Id("synthetic-parent".into()),
        request_id: Id("resource-request".into()),
        context: snapshot.context.clone(),
        limits: Limits {
            max_elements: nodes,
            max_depth: depth,
            max_output_bytes: 65_536,
            deadline_ms: 3_000,
        },
        freshness_policy: FreshnessPolicy::CurrentRequired,
        operation: Operation::Observe {
            channels: vec![Channel::ExternalSemantics],
        },
    }
}
fn session_row(
    snapshot: &Snapshot,
    nodes: u32,
    depth: u32,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let r = request(snapshot, nodes, depth);
    let request_frame = serde_json::to_vec(&Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Request(Box::new(r.clone())),
    })?;
    let descriptor = SessionDescriptor {
        allowed_scopes: vec![snapshot.context.scope_id.clone()],
        session_id: snapshot.context.session_id.clone(),
        plugin: snapshot.context.plugin.clone(),
        supported_versions: vec![SchemaVersion::CURRENT],
        target: snapshot.context.target.clone(),
        surfaces: snapshot.context.surfaces.clone(),
        capabilities: vec![Capability {
            channel: Channel::ExternalSemantics,
            operation: Id("observe".into()),
            status: CapabilityStatus::Partial,
            reason: Some(Id("synthetic".into())),
        }],
    };
    // Inputs are prepared before marking. Their retained copies in the session
    // and parser/BTree working allocations are measured as independent phases.
    let mark = allocation::begin();
    let mut session = ObservationSession::attach(
        descriptor.clone(),
        r.clock_domain.clone(),
        SessionLimits {
            max_frame_bytes: 65_536,
            max_in_flight: 1,
            max_pending_encoded_bytes: 131_072,
        },
    )?;
    let attached = allocation::finish(mark)?;
    let clock = ClockReading {
        domain: r.clock_domain.clone(),
        milliseconds: 100,
    };
    let mark = allocation::begin();
    let ticket = session.begin(&request_frame, &clock)?;
    let begun = allocation::finish(mark)?;
    let response = Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::ChannelResponse(Box::new(ChannelResponse {
            request_id: ticket.request_id.clone(),
            session_id: ticket.session_id.clone(),
            dispatch_sequence: ticket.sequence,
            target: snapshot.context.target.clone(),
            channel: Channel::ExternalSemantics,
            result: ChannelResult::Observed(Box::new(snapshot.clone())),
        })),
    };
    let response_frame = serde_json::to_vec(&response)?;
    let receive_clock = ClockReading {
        domain: r.clock_domain.clone(),
        milliseconds: 110,
    };
    let mark = allocation::begin();
    let result = session.receive(&ticket, &response_frame, &receive_clock);
    let received = allocation::finish(mark)?;
    let accepted = result.is_ok();
    if response_frame.len() > 65_536 {
        assert_eq!(result, Err(uiblueprint_plugin_api::Error::ResourceLimit));
        assert_eq!(received.retained_delta, 0);
    } else {
        result?;
    }
    let mark = allocation::begin();
    let completion = session.cancel(&ticket)?;
    let cancelled = allocation::finish(mark)?;
    assert_eq!(completion.channels.len(), usize::from(accepted));
    assert_eq!(session.pending_encoded_bytes(), 0);
    let mark = allocation::begin();
    drop(completion);
    let dropped = allocation::finish(mark)?;
    let mark = allocation::begin();
    drop(session);
    let detached = allocation::finish(mark)?;
    Ok(
        serde_json::json!({"nodes":nodes,"depth":depth,"frame_cap":65536,"pending_encoded_cap":131072,"session_inline":size_of::<ObservationSession>(),"request_bytes":request_frame.len(),"response_bytes":response_frame.len(),"accepted":accepted,"attach":stats(attached),"begin":stats(begun),"receive":stats(received),"cancel_transfers_completion":stats(cancelled),"drop_completion":stats(dropped),"drop_session":stats(detached)}),
    )
}

struct Chunked<'a> {
    bytes: &'a [u8],
    chunk: usize,
}
impl std::io::Read for Chunked<'_> {
    fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
        let n = self.bytes.len().min(out.len()).min(self.chunk);
        out[..n].copy_from_slice(&self.bytes[..n]);
        self.bytes = &self.bytes[n..];
        Ok(n)
    }
}
fn framing_rows() -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let mut rows = Vec::new();
    for cap in [65_536, 131_072, 524_288] {
        for length in [32, cap, cap + 1] {
            let mut data = vec![b'x'; length];
            *data.last_mut().ok_or("empty frame")? = b'\n';
            for chunk in [1, 31, 8192, 65_536] {
                let reader = Chunked {
                    bytes: &data,
                    chunk,
                };
                let mut reader = BufReader::new(reader);
                let buffered_capacity = reader.capacity();
                let mark = allocation::begin();
                let event = common::read_frame(&mut reader, cap);
                let measured = allocation::finish(mark)?;
                let frame_capacity = match &event {
                    common::FrameEvent::Frame(bytes) => {
                        assert!(length <= cap);
                        Some(bytes.capacity())
                    }
                    common::FrameEvent::Oversize => {
                        assert!(length > cap);
                        None
                    }
                    _ => return Err("unexpected framing outcome".into()),
                };
                rows.push(serde_json::json!({"line_bytes":length,"chunk_bytes":chunk,"frame_limit":cap,"reader_capacity":buffered_capacity,"returned_frame_capacity":frame_capacity,"allocation":stats(measured)}));
            }
        }
    }
    Ok(rows)
}

fn reuse_rows() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let s = synthetic::snapshot(32, 8);
    let r = request(&s, 32, 8);
    let frame = serde_json::to_vec(&Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Request(Box::new(r.clone())),
    })?;
    let descriptor = SessionDescriptor {
        allowed_scopes: vec![s.context.scope_id.clone()],
        session_id: s.context.session_id.clone(),
        plugin: s.context.plugin.clone(),
        supported_versions: vec![SchemaVersion::CURRENT],
        target: s.context.target.clone(),
        surfaces: s.context.surfaces.clone(),
        capabilities: vec![Capability {
            channel: Channel::ExternalSemantics,
            operation: Id("observe".into()),
            status: CapabilityStatus::Supported,
            reason: None,
        }],
    };
    let mut session = ObservationSession::attach(
        descriptor,
        r.clock_domain.clone(),
        SessionLimits {
            max_frame_bytes: 65_536,
            max_in_flight: 1,
            max_pending_encoded_bytes: 131_072,
        },
    )?;
    let mut idle_after_first = None;
    // No collector/polling: 20 explicit in-memory operations exercise ownership
    // release, with injected synthetic time (not D02 real-clock evidence).
    for i in 0..20 {
        let clock = ClockReading {
            domain: r.clock_domain.clone(),
            milliseconds: 100 + i * 20,
        };
        let ticket = session.begin(&frame, &clock)?;
        let response = Document {
            schema_version: SchemaVersion::CURRENT,
            artifact: Artifact::ChannelResponse(Box::new(ChannelResponse {
                request_id: ticket.request_id.clone(),
                session_id: ticket.session_id.clone(),
                dispatch_sequence: ticket.sequence,
                target: s.context.target.clone(),
                channel: Channel::ExternalSemantics,
                result: ChannelResult::Observed(Box::new(s.clone())),
            })),
        };
        let bytes = serde_json::to_vec(&response)?;
        session.receive(&ticket, &bytes, &clock)?;
        let completion = session.cancel(&ticket)?;
        drop(completion);
        drop(bytes);
        drop(response);
        drop(ticket);
        drop(clock);
        assert_eq!(session.pending_encoded_bytes(), 0);
        let live = allocation::live();
        if let Some(expected) = idle_after_first {
            assert_eq!(live, expected, "idle session allocation growth");
        } else {
            idle_after_first = Some(live);
        }
    }
    Ok(
        serde_json::json!({"explicit_operations":20,"idle_live_after_first":idle_after_first,"subsequent_idle_growth_bytes":0,"note":"baseline includes prepared synthetic data; zero encoded pending does not mean zero idle session heap"}),
    )
}

fn channel_accounting() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let mark = allocation::begin();
    let (tx, rx) = std::sync::mpsc::sync_channel::<common::FrameEvent>(1);
    let created = allocation::finish(mark)?;
    let mark = allocation::begin();
    tx.send(common::FrameEvent::Eof)?;
    let _ = rx.recv()?;
    let used = allocation::finish(mark)?;
    let mark = allocation::begin();
    drop(tx);
    drop(rx);
    let dropped = allocation::finish(mark)?;
    Ok(
        serde_json::json!({"capacity":1,"create":stats(created),"send_receive_no_payload":stats(used),"drop":stats(dropped),"excludes":"reader thread/runtime/OS stack and actual frame payload allocations"}),
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    allocator_self_check()?;
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if !args.is_empty() {
        use std::io::Read;
        if args.len() % 2 != 0 {
            return Err("expected --document FILE pairs".into());
        }
        let mut rows = Vec::new();
        for pair in args.chunks_exact(2) {
            if pair[0] != "--document" {
                return Err("unsupported option".into());
            }
            let mut bytes = Vec::new();
            let mark = allocation::begin();
            std::fs::File::open(&pair[1])?
                .take(4_194_305)
                .read_to_end(&mut bytes)?;
            let read = allocation::finish(mark)?;
            if bytes.len() > 4_194_304 {
                return Err("explicit diagnostic input limit exceeded".into());
            }
            let mut row = parser_row("provided-original-bytes", &bytes, true)?;
            row["input_buffer_capacity"] = bytes.capacity().into();
            row["input_read"] = stats(read);
            rows.push(row);
            let doc = Document::from_json(&bytes, 4_194_304)?;
            rows.push(parser_row(
                "provided-content-reordered-diagnostic",
                &synthetic::sorted_json(&doc),
                true,
            )?);
        }
        println!(
            "{}",
            serde_json::to_string_pretty(
                &serde_json::json!({"method":"single_process_rust_layout_allocator_v1","cohort":"provided_actual_bytes_and_explicit_reencoding","exclusions":"allocator rounding/metadata/internal realloc overlap, OS/C allocations, external pixels; not RSS or policy","samples":rows})
            )?
        );
        return Ok(());
    }
    let mut parser = Vec::new();
    let mut sessions = Vec::new();
    for (nodes, depth) in [(1, 1), (32, 8), (160, 9)] {
        let snapshot = synthetic::snapshot(nodes, depth);
        let doc = synthetic::document(snapshot.clone());
        let canonical = serde_json::to_vec(&doc)?;
        let sorted = synthetic::sorted_json(&doc);
        let escaped = synthetic::escaped_json(&doc);
        assert_eq!(
            Document::from_json(&canonical, 4_194_304)?,
            Document::from_json(&sorted, 4_194_304)?
        );
        assert_eq!(
            Document::from_json(&canonical, 4_194_304)?,
            Document::from_json(&escaped, 4_194_304)?
        );
        parser.push(parser_row(
            &format!("tree-{nodes}-depth-{depth}-canonical"),
            &canonical,
            true,
        )?);
        parser.push(parser_row(
            &format!("tree-{nodes}-depth-{depth}-sorted-members"),
            &sorted,
            true,
        )?);
        parser.push(parser_row(
            &format!("tree-{nodes}-depth-{depth}-escaped-unicode"),
            &escaped,
            true,
        )?);
        sessions.push(session_row(&snapshot, nodes as u32, depth as u32)?);
    }
    for (name, document) in [
        (
            "frame-bound-long-text",
            synthetic::fit_frame(65_536, synthetic::long_text),
        ),
        (
            "frame-bound-dense-text-list",
            synthetic::fit_frame(65_536, synthetic::dense_text_list),
        ),
    ] {
        let wire = serde_json::to_vec(&document)?;
        assert!(wire.len() <= 65_536);
        parser.push(parser_row(&format!("{name}-canonical"), &wire, true)?);
        parser.push(parser_row(
            &format!("{name}-sorted-members"),
            &synthetic::sorted_json(&document),
            true,
        )?);
    }
    for cap in [131_072, 524_288] {
        let document = synthetic::fit_frame(cap, synthetic::dense_text_list);
        parser.push(parser_row(
            &format!("dense-list-frame-{cap}-canonical"),
            &serde_json::to_vec(&document)?,
            true,
        )?);
        parser.push(parser_row(
            &format!("dense-list-frame-{cap}-sorted"),
            &synthetic::sorted_json(&document),
            true,
        )?);
    }
    // Wrong typed artifact, but valid byte-bounded JSON with content before tag.
    // It probes transient rejection storage, not the protected shared-node DAG.
    let mut malformed = String::from("{\"schema_version\":\"0.1.0\",\"artifact\":{\"data\":[");
    while malformed.len() + 24 < 65_536 {
        if !malformed.ends_with('[') {
            malformed.push(',');
        }
        malformed.push_str("[0]");
    }
    malformed.push_str("],\"kind\":\"snapshot\"}}");
    assert!(malformed.len() <= 65_536);
    parser.push(parser_row(
        "wrong-type-dense-arrays-under-64k",
        malformed.as_bytes(),
        false,
    )?);

    let dense = synthetic::fit_frame(65_000, synthetic::dense_text_list);
    let Artifact::Snapshot(dense_snapshot) = dense.artifact else {
        return Err("synthetic snapshot expected".into());
    };
    let mut dense_session = session_row(&dense_snapshot, 1, 1)?;
    dense_session["case"] = serde_json::json!("synthetic-dense-text-list-under-64k");
    sessions.push(dense_session);

    // Invalid, byte-bounded repeated properties are parser-only. No shared-child
    // DAG or protected depth_within hang is constructed or exercised.
    let mut invalid = synthetic::snapshot(1, 1);
    let duplicate = invalid.nodes[0].properties[0].clone();
    for _ in 0..128 {
        invalid.nodes[0].properties.push(duplicate.clone());
    }
    let wire = synthetic::sorted_json(&synthetic::document(invalid));
    assert!(wire.len() < 65_536);
    parser.push(parser_row(
        "invalid-duplicate-properties-sorted",
        &wire,
        false,
    )?);
    let framing = framing_rows()?;
    let reuse = reuse_rows()?;
    let channel = channel_accounting()?;
    let report = serde_json::json!({"method":"single_process_rust_layout_allocator_v1","cohort":"authored_synthetic_only","pointer_bits":usize::BITS,"parser_input_limit":4194304,"exclusions":"allocator rounding/metadata/internal realloc overlap, OS/C allocations, external payloads; baseline includes prepared inputs/report state; not RSS or a production policy","parser":parser,"sessions":sessions,"framing":framing,"session_reuse":reuse,"channel_control":channel});
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}
