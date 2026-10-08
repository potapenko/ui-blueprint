//! W03-R: real scoped Web collection, owned transport loss and recorded replay.
use super::*;

fn selection() -> WebSelection {
    WebSelection::Initial {
        ids: ["mutation-child", "draft", "applied"]
            .map(|id| WebId {
                id: Id(id.into()),
                sensitivity: Sensitivity::Public,
            })
            .into(),
        max_visited_nodes: 256,
    }
}
fn current<'a>(
    host: &mut RuntimeHost<'a, process::Platform>,
    a: &Attached<'a>,
    selected: WebSelection,
) -> HostCompletion<'a> {
    // Environment is the unchanged viewport/DPR profile, not a fabricated source
    // revision. Both live acquisitions keep their actual Observation/time/IDs.
    observe(
        host,
        a,
        "mutation-child",
        vec![Field::LayoutBounds, Field::Value, Field::PaintBounds],
        selected,
        "w03-state",
    )
}
fn before(f: &mut Fixture, page: &str, case: &str) {
    f.call("resync_before", json!({"page":page,"case":case}));
}
fn checked(
    f: &mut Fixture,
    page: &str,
    case: &str,
    doc: Option<&Document>,
    changed: bool,
    private: bool,
) {
    f.call(
        "resync_check",
        json!({"page":page,"case":case,"document":doc,"changed":changed,"private":private}),
    );
}
fn snapshot(doc: &Document) -> &Snapshot {
    let Artifact::ChannelResponse(c) = &doc.artifact else {
        panic!("channel")
    };
    let ChannelResult::Observed(s) = &c.result else {
        panic!("observed")
    };
    s
}
fn replay<'a>(
    host: &mut RuntimeHost<'a, process::Platform>,
    a: &Attached<'a>,
    base: &Snapshot,
    full: &Snapshot,
    wrong_scope: bool,
) -> HostCompletion<'a> {
    // Replay current selected-node records while preserving base-only metadata.
    // Full live snapshots have different Surface evidence; this is NOT a claim
    // that two live acquisitions are a common full/delta source oracle.
    let mut update = Delta {
        base_revision: base.revision,
        revision: full.revision,
        source_state: full.source_state.clone(),
        context: full.context.clone(),
        observations: full.observations.clone(),
        upsert: full.nodes.clone(),
        removed: vec![],
        relations: full.relations.clone(),
        focus: full.focus.clone(),
        coverage: full.coverage.clone(),
    };
    if wrong_scope {
        update.context.scope_id = Id("other-scope".into());
    }
    let doc = Document {
        schema_version: uiblueprint_schema::SchemaVersion::CURRENT,
        artifact: Artifact::Delta(Box::new(DeltaCase {
            base: base.clone(),
            update,
            source_snapshot: None,
        })),
    };
    let data = serde_json::to_vec(&doc).unwrap();
    let id = serde_json::to_vec(&Id("w03-recorded-composite".into())).unwrap();
    let parts = [data.as_slice(), id.as_slice()];
    let mut input = host.reserve_input(a.handle, tape_length(&parts)).unwrap();
    worker_tape::encode(&parts, input.bytes_mut()).unwrap();
    host.submit(
        a.handle,
        OperationClass::Replay,
        input,
        OutputRequest {
            channels: 1,
            frame_bytes: 65536,
            total_bytes: 65536,
            input_format: 0,
            retained_partition: 1,
        },
        deadline(),
    )
    .unwrap();
    complete(host)
}
pub(super) fn run<'a>(host: &mut RuntimeHost<'a, process::Platform>, f: &mut Fixture) {
    let fault: Binding =
        serde_json::from_value(f.call("loss_binding", json!({"page":"a"}))).unwrap();
    let a = attach(host, fault, 1);
    let b = attach(host, f.binding("b"), 2);
    before(f, "a", "initial");
    let first = current(host, &a, selection());
    let original = decoded(&first);
    checked(f, "a", "initial", Some(&original), false, false);
    let bytes = first.bytes(0).unwrap().to_vec();
    drop(retain_snapshot(host, &a, &original));
    f.call("loss_cut", json!({"page":"a"}));
    f.stimulus("parentWide");
    f.stimulus("fontLarge");
    before(f, "a", "transport-refusal");
    let failure = current(host, &a, selection());
    refused(&failure);
    checked(f, "a", "transport-refusal", None, true, false);
    drop(failure);
    before(f, "a", "history-after-loss");
    drop(retain_snapshot(host, &a, &original));
    assert_eq!(first.bytes(0), Some(bytes.as_slice()));
    checked(f, "a", "history-after-loss", None, true, false);
    before(f, "b", "independent-target");
    let independent = current(host, &b, selection());
    checked(
        f,
        "b",
        "independent-target",
        Some(&decoded(&independent)),
        false,
        false,
    );
    drop(independent);
    // Existing explicit recovery: retire dead transport/refs, establish actual
    // target/document binding, and make ONE bounded selected initial request.
    detach(host, &a);
    let recovered = attach(host, f.binding("a"), 3);
    before(f, "a", "reattached-current");
    let response = current(host, &recovered, selection());
    checked(
        f,
        "a",
        "reattached-current",
        Some(&decoded(&response)),
        true,
        false,
    );
    drop(response);
    before(f, "a", "bounded-selection-refusal");
    let mut limited = selection();
    let WebSelection::Initial {
        max_visited_nodes, ..
    } = &mut limited
    else {
        unreachable!()
    };
    *max_visited_nodes = 1;
    let response = current(host, &recovered, limited);
    assert_eq!(
        response.terminal,
        Terminal::Failed(uiblueprint_host::HostError::ResourceLimit)
    );
    assert_eq!(response.committed(), 0);
    checked(f, "a", "bounded-selection-refusal", None, true, false);
    drop(response);
    before(f, "a", "explicit-after-limit");
    let base_result = current(host, &recovered, selection());
    let base_doc = decoded(&base_result);
    checked(f, "a", "explicit-after-limit", Some(&base_doc), true, false);
    drop(base_result);
    f.stimulus("private");
    before(f, "a", "private-checkpoint");
    let full_result = current(host, &recovered, selection());
    let full_doc = decoded(&full_result);
    checked(f, "a", "private-checkpoint", Some(&full_doc), true, true);
    drop(full_result);
    let base = snapshot(&base_doc);
    let full = snapshot(&full_doc);
    assert_eq!(base.context, full.context);
    assert_eq!(base.revision + 1, full.revision);
    before(f, "a", "recorded-full-delta");
    let lost = replay(host, &recovered, base, full, false);
    f.call("resync_result",json!({"page":"a","stage":"lost-base","terminal":terminal_code(lost.terminal),"committed":lost.committed()}));
    assert_eq!(
        lost.terminal,
        Terminal::Failed(uiblueprint_host::HostError::ResyncRequired)
    );
    assert_eq!(lost.committed(), 0);
    drop(lost);
    drop(retain_snapshot(host, &recovered, &base_doc));
    let mismatch = replay(host, &recovered, base, full, true);
    f.call("resync_result",json!({"page":"a","stage":"scope-mismatch","terminal":terminal_code(mismatch.terminal),"committed":mismatch.committed()}));
    assert_eq!(
        mismatch.terminal,
        Terminal::Failed(uiblueprint_host::HostError::ResyncRequired)
    );
    assert_eq!(mismatch.committed(), 0);
    drop(mismatch);
    let delta = replay(host, &recovered, base, full, false);
    f.call("resync_result",json!({"page":"a","stage":"replay","terminal":terminal_code(delta.terminal),"committed":delta.committed()}));
    assert_eq!(delta.terminal, Terminal::Completed);
    // Independently assemble the expected recorded composite from the two input
    // records. New properties keep their actual evidence; Surface metadata and
    // its original observation remain historical. Neither input is rewritten.
    let mut expected_snapshot = full.clone();
    expected_snapshot.id = Id("w03-recorded-composite".into());
    expected_snapshot.surface_records = base.surface_records.clone();
    // Sensitive draft omits its AX read; absence is not a deletion. Keep that
    // one older public AX record and both original observations (Surface + AX).
    assert_eq!(base.nodes.len(), 6);
    assert_eq!(full.nodes.len(), 5);
    let omitted: Vec<_> = base
        .nodes
        .iter()
        .enumerate()
        .filter(|(_, old)| !full.nodes.iter().any(|new| new.key == old.key))
        .collect();
    assert_eq!(omitted.len(), 1);
    assert_eq!(omitted[0].1.key.namespace.0, "web.ax");
    expected_snapshot
        .nodes
        .insert(omitted[0].0, omitted[0].1.clone());
    expected_snapshot
        .observations
        .extend(base.observations.clone());
    let expected = serde_json::to_vec(&Document {
        schema_version: full_doc.schema_version,
        artifact: Artifact::Snapshot(Box::new(expected_snapshot)),
    })
    .unwrap();
    assert_eq!(
        delta.bytes(0),
        Some(expected.as_slice()),
        "exact current properties and historical Surface evidence"
    );
    drop(delta);
    drop(retain_snapshot(host, &recovered, &base_doc));
    // The separately ACKed full_doc is the current Surface record; this replay
    // intentionally does not replace its historical Surface evidence.
    checked(f, "a", "recorded-full-delta", None, true, true);
    assert_eq!(
        first.bytes(0),
        Some(bytes.as_slice()),
        "ACK survives actual detach"
    );
    drop(first);
}
