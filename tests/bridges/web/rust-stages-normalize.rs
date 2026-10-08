// Appended to the EXISTING collector test helper file in an immutable task-temp copy.
// That file is included as a unit-test child of collector::acquire, so private
// routines/Records are reused directly. No normalizer implementation is copied.
#[test]
#[ignore = "Q02 pinned saved-input stage diagnostic only"]
fn q02_saved_normalization() {
    use std::{fs, hint::black_box};
    let base = std::path::PathBuf::from(std::env::var("UIB_Q02_STAGE_INPUTS").unwrap());
    let kind = std::env::var("UIB_Q02_STAGE_KIND").unwrap();
    let repeats: usize = std::env::var("UIB_Q02_STAGE_REPEATS")
        .unwrap()
        .parse()
        .unwrap();
    assert!(matches!(repeats, 1 | 100));
    let bytes = fs::read(base.join(format!("{kind}-canonical.json"))).unwrap();
    let expected_doc = Document::from_json(&bytes, 524288).unwrap();
    let Artifact::ChannelResponse(response) = &expected_doc.artifact else {
        panic!("response")
    };
    let ChannelResult::Observed(expected) = &response.result else {
        panic!("observed")
    };
    let input = fs::read(base.join(format!("{kind}-input.json"))).unwrap();
    let request = fs::read(base.join(format!("{kind}-request.json"))).unwrap();
    let Artifact::Request(request) = Document::from_json(&request, 65536).unwrap().artifact else {
        panic!("request")
    };
    let mut parts = expected.id.0.split(':');
    assert_eq!(parts.next(), Some("web-snapshot"));
    let stamp = (
        parts.next().unwrap().parse::<u64>().unwrap(),
        parts.next().unwrap().parse::<u64>().unwrap(),
    );
    assert!(parts.next().is_none());
    let mut caps = limits();
    caps.max_nodes = if kind == "documents" { 128 } else { 16 };
    caps.max_text_bytes = if kind == "documents" { 16384 } else { 600 };
    let mut rows = Vec::with_capacity(repeats);
    if kind == "documents" {
        use super::super::{DocumentSeed, DocumentsScope, snapshot_normalize, snapshot_wire};
        let template: snapshot_wire::Capture = serde_json::from_slice(&input).unwrap();
        let scope = DocumentsScope {
            scope_id: request.context.scope_id.clone(),
            max_visited_nodes: 128,
            documents: request
                .context
                .surfaces
                .iter()
                .map(|s| DocumentSeed {
                    surface: s.clone(),
                    document_backend_id: template
                        .documents
                        .iter()
                        .find(|d| template.strings[d.frame_id as usize] == s.id.0)
                        .unwrap()
                        .nodes
                        .backend_node_id[0],
                    sensitivity: Sensitivity::Public,
                })
                .collect(),
        };
        let counts: Vec<_> = scope
            .documents
            .iter()
            .map(|s| {
                template
                    .documents
                    .iter()
                    .find(|d| template.strings[d.frame_id as usize] == s.surface.id.0)
                    .unwrap()
                    .nodes
                    .backend_node_id
                    .len()
            })
            .collect();
        for index in 0..repeats {
            let raw: snapshot_wire::Capture = serde_json::from_slice(&input).unwrap();
            let observation = expected.observations[0].clone();
            let started = Instant::now();
            let result = snapshot_normalize::snapshot(
                black_box(raw),
                &scope,
                &request,
                &counts,
                observation,
                stamp,
                caps,
            );
            let normalize_ns = started.elapsed().as_nanos();
            let result = result.expect("actual full normalizer");
            assert!(result == **expected, "full saved-fact equality failed");
            rows.push(q02_count_format(
                index,
                normalize_ns,
                result,
                &request,
                response.dispatch_sequence,
                &expected_doc,
            ));
        }
    } else {
        let raw: Json = serde_json::from_slice(&input).unwrap();
        // An existing bounded inert peer supplies a valid owned client to the
        // Collector value. NO CDP collection/attachment runs inside this replay.
        let fixture = Fixture::new(|_, _, _| None);
        let collector = Collector {
            client: fixture.client(8192, 8192),
            binding: collector::Binding {
                session_id: request.context.session_id.clone(),
                target: request.context.target.clone(),
                surface: request.context.surfaces[0].clone(),
                cdp_session_id: None,
                clock: Clock {
                    domain: request.clock_domain.clone(),
                    origin: Instant::now(),
                },
                allowed_scopes: vec![request.context.scope_id.clone()],
                plugin: request.context.plugin.clone(),
            },
            limits: caps,
            origin: Instant::now(),
            document: raw["document_backend"].as_u64().unwrap() as u32,
            world: raw["world"].as_i64().unwrap() as i32,
            invalid: false,
            owner: stamp.0,
            sequence: stamp.1,
            loss_generation: 0,
            pending_invalidation: false,
            allowed_surfaces: request.context.surfaces.clone(),
        };
        let dom = expected
            .nodes
            .iter()
            .find(|n| n.key.namespace.0 == "web.dom")
            .unwrap();
        let scope = Scope {
            scope_id: request.context.scope_id.clone(),
            nodes: vec![NodeRef {
                sensitivity: Sensitivity::Public,
                reference: BackendRef {
                    session_id: request.context.session_id.clone(),
                    target: request.context.target.clone(),
                    surface: dom.surface.clone(),
                    key: dom.key.clone(),
                    snapshot_id: expected.id.clone(),
                    observation_id: expected.observations[0].id.clone(),
                },
            }],
        };
        for index in 0..repeats {
            let backend = raw["backend"].as_u64().unwrap() as u32;
            let ax = kind == "semantic";
            let records = super::Records {
                dom: vec![(backend, serde_json::from_value(raw["dom"].clone()).unwrap())],
                ax: if ax {
                    vec![(backend, serde_json::from_value(raw["ax"].clone()).unwrap())]
                } else {
                    vec![]
                },
                dom_start: expected.observations[0].start,
                dom_end: expected.observations[0].end,
                ax_start: expected.observations.get(1).map_or(0.0, |o| o.start),
                ax_end: expected.observations.get(1).map_or(0.0, |o| o.end),
                ax_status: if ax {
                    collector::SourceStatus::Partial
                } else {
                    collector::SourceStatus::NotRequested
                },
                ax_queries: usize::from(ax),
                selection: None,
            };
            let started = Instant::now();
            let result = collector.normalize(
                &request,
                super::super::Plan::References(&scope),
                black_box(records),
            );
            let normalize_ns = started.elapsed().as_nanos();
            let result = result.expect("actual control normalizer");
            assert!(result == **expected, "control saved-fact equality failed");
            rows.push(q02_count_format(
                index,
                normalize_ns,
                result,
                &request,
                response.dispatch_sequence,
                &expected_doc,
            ));
        }
        drop(collector);
        fixture.finish();
    }
    println!(
        "@Q02_STAGES {}",
        json!({"kind":kind,"samples":rows,"allocator":"System; test process, not worker quota instrumentation","quality":"exact saved Snapshot and Document equality"})
    );
}
fn q02_count_format(
    index: usize,
    normalize_ns: u128,
    snapshot: Snapshot,
    request: &Request,
    sequence: u64,
    expected: &Document,
) -> Json {
    use super::super::observe;
    let document = observe::document(
        request,
        sequence,
        Channel::ExternalSemantics,
        ChannelResult::Observed(Box::new(snapshot)),
    );
    assert!(
        &document == expected,
        "actual envelope differs from saved canonical input"
    );
    let started = Instant::now();
    let bytes = observe::bounded_document(
        std::hint::black_box(&document),
        request.limits.max_output_bytes,
    );
    let format_count_ns = started.elapsed().as_nanos();
    json!({"index":index,"normalization_inclusive_ns":normalize_ns,"format_count_ns":format_count_ns,"bytes":bytes.unwrap()})
}
