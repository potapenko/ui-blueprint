//! Explicit consumer of actual V02 output, never a synthetic replacement for SDK input.
use uiblueprint_engine::cache::*;
use uiblueprint_export::*;
use uiblueprint_schema::{SchemaVersion, model::*};

#[test]
#[ignore = "requires V02_RECORD_DIRECTORY and caller-owned V02_SECRET_FILE from the actual fixture run"]
fn actual_protected_snapshots_cache_history_and_export() {
    let directory = std::path::PathBuf::from(
        std::env::var_os("V02_RECORD_DIRECTORY").expect("actual run directory"),
    );
    let secret =
        std::fs::read(std::env::var_os("V02_SECRET_FILE").expect("synthetic input source"))
            .expect("test source");
    assert!(!secret.is_empty());
    let absent = |bytes: &[u8]| {
        assert!(
            !bytes.windows(secret.len()).any(|w| w == secret),
            "private value in serialized channel"
        )
    };
    let read = |name: &str| {
        let bytes = std::fs::read(directory.join(name)).expect("actual saved snapshot");
        absent(&bytes);
        match Document::from_json(&bytes, 524288)
            .expect("valid snapshot")
            .artifact
        {
            Artifact::Snapshot(s) => *s,
            _ => panic!("expected Snapshot"),
        }
    };
    let before = read("before.json");
    let after = read("after.json");
    let protected = |s: &Snapshot| {
        s.nodes.iter().any(|n| {
            n.properties.iter().any(|p| {
                matches!(
                    p,
                    Property::Requested {
                        field: Field::Value,
                        sensitivity: Sensitivity::Sensitive,
                        state: Availability::Redacted {},
                        ..
                    }
                )
            })
        })
    };
    let empty = |s: &Snapshot| {
        s.nodes.iter().any(|n| n.properties.iter().any(|p| matches!(p,
        Property::Requested { field: Field::Value, sensitivity: Sensitivity::Public, state: Availability::Known { value: Value::Text(v) }, .. } if v.is_empty())))
    };
    assert!(protected(&before) && protected(&after));
    assert!(
        empty(&before) && empty(&after),
        "known empty public input lost"
    );
    let ledger = QuotaLedger::new(LedgerLimits {
        retained_bytes: 64 * 1048576,
        session_slots: 1,
        grants: 1,
    })
    .unwrap();
    let mut cache = CacheStore::new(
        ledger
            .reserve(Allowance {
                bytes: 8 * 1048576,
                session_slots: 1,
            })
            .unwrap(),
        StoreLimits {
            session_slots: 1,
            snapshot_slots_per_session: 2,
            revisions_per_family: 2,
            entry_bytes: 1048576,
            session_bytes: 4 * 1048576,
            retention_ms: 1000,
        },
        Id("v02-clock".into()),
        0,
    )
    .unwrap();
    let clock_id = Id("v02-clock".into());
    let clock = || Clock {
        domain: &clock_id,
        milliseconds: 1,
    };
    let session = cache
        .open_session(SessionIdentity {
            session_id: before.context.session_id.clone(),
            target: before.context.target.clone(),
            plugin: before.context.plugin.clone(),
        })
        .unwrap();
    let mut handles = Vec::new();
    for snapshot in [before, after] {
        handles.push(
            cache
                .admit_full(
                    session,
                    Incoming {
                        snapshot,
                        partition: vec![Channel::ExternalSemantics],
                    },
                    clock(),
                )
                .expect("bounded cache admission")
                .handle,
        );
    }
    // Explicit temporary history, caller-selected directory, consumed by this QA run.
    // ReadPolicy::Recorded does not upgrade freshness or Observation timestamps.
    for (i, handle) in handles.into_iter().enumerate() {
        let stored = cache
            .read(handle, ReadPolicy::Recorded, clock())
            .expect("historical cache read");
        assert!(protected(stored.snapshot) && empty(stored.snapshot));
        let bytes = serde_json::to_vec(&Document {
            schema_version: SchemaVersion::CURRENT,
            artifact: Artifact::Snapshot(Box::new(stored.snapshot.clone())),
        })
        .unwrap();
        absent(&bytes);
        let history = directory.join(format!("history-{i}.json"));
        std::fs::write(&history, &bytes).unwrap();
        let bytes = std::fs::read(history).unwrap();
        absent(&bytes);
        let Artifact::Snapshot(snapshot) = Document::from_json(&bytes, 524288).unwrap().artifact
        else {
            unreachable!()
        };
        let limits = ExportLimits {
            max_input_bytes: 2_000_000,
            max_output_bytes: 4_000_000,
            max_components: 256,
            max_views: 8,
            components_per_detail: 12,
        };
        let mut brief = DrawingBrief::from_json(
            include_bytes!("../../../fixtures/export/observed-brief.json"),
            limits,
        )
        .unwrap();
        brief.views.truncate(1);
        brief.details.clear();
        brief.comparisons.clear();
        brief.transitions.clear();
        brief.views[0].source = SourceInput::Observed {
            snapshot,
            public_text_fields: vec![Field::AccessibilityName],
        };
        brief.views[0].environment = "V02 own Native synthetic fixture; actual AX".into();
        brief.views[0].state = if i == 0 {
            "before protected input"
        } else {
            "after protected input"
        }
        .into();
        brief.views[0].scope = "three explicit held Native controls; partial".into();
        brief.views[0].safe_source_reference = "V02 explicit protected-input observation".into();
        let package = compile(&brief, limits).expect("observed export");
        assert_eq!(package.files().len(), 6);
        for (name, bytes) in package.files() {
            absent(bytes);
            std::fs::write(directory.join(format!("export-{i}-{name}")), bytes).unwrap();
        }
        assert!(
            std::str::from_utf8(&package.files()["prompt.txt"])
                .unwrap()
                .contains("redacted")
        );
    }
    cache.detach(session).unwrap();
    assert_eq!(cache.usage().snapshots, 0);
}
