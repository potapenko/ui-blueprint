#[path = "../../../tests/bridges/resources/owned_memory.rs"]
mod memory;
use memory::{Heap, HeapSize};
use std::{fs, mem::size_of, path::PathBuf};
use uiblueprint_schema::{SchemaVersion, model::*};

fn snapshot() -> Snapshot {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/golden/ENV-SNAPSHOT-VALID.json");
    let d =
        Document::from_json(&fs::read(path).expect("fixture"), 1_048_576).expect("valid snapshot");
    let Artifact::Snapshot(s) = d.artifact else {
        panic!("snapshot")
    };
    *s
}

#[test]
fn unused_owned_capacity_changes_memory_without_changing_wire_or_node_count() {
    let mut s = snapshot();
    let wire = serde_json::to_vec(&s).expect("wire");
    let before = memory::owned(&s).expect("size").2;
    let old_capacity = s.nodes.capacity();
    s.nodes.reserve_exact(16);
    let capacity_increase = s.nodes.capacity() - old_capacity;
    assert_eq!(s.nodes.len(), 1);
    assert_eq!(serde_json::to_vec(&s).expect("same data"), wire);
    assert_eq!(
        memory::owned(&s).expect("larger allocation").2 - before,
        capacity_increase * size_of::<Node>()
    );

    let before = memory::owned(&s).expect("size").2;
    let name = s.nodes[0]
        .properties
        .iter_mut()
        .find(|p| p.field() == Field::Name)
        .expect("name");
    let Property::Requested {
        state: Availability::Known {
            value: Value::Text(text),
        },
        ..
    } = name
    else {
        panic!("text")
    };
    let capacity = text.capacity();
    text.reserve_exact(1024);
    let increase = text.capacity() - capacity;
    assert_eq!(serde_json::to_vec(&s).expect("same serialized data"), wire);
    assert_eq!(
        memory::owned(&s).expect("string allocation counted").2 - before,
        increase
    );
}

#[test]
fn zero_sized_version_vectors_and_nested_boxes_are_counted_without_double_inline() {
    let versions = vec![SchemaVersion::CURRENT; 10];
    assert_eq!(versions.capacity(), usize::MAX);
    assert_eq!(
        versions.heap().expect("ZST capacity is not an allocation"),
        Heap::default()
    );
    let s = snapshot();
    let plain = memory::owned(&s).expect("plain").2;
    let boxed = Box::new(s);
    let measured = memory::owned(&boxed).expect("boxed");
    assert_eq!(measured.2, plain + size_of::<Box<Snapshot>>());
    assert_eq!(
        measured.1.allocations,
        boxed.as_ref().heap().expect("payload").allocations + 1
    );
}

#[test]
fn size_arithmetic_fails_instead_of_wrapping_a_budget_charge() {
    let maximum = Heap {
        bytes: usize::MAX,
        allocations: 0,
        slack_bytes: 0,
    };
    assert!(
        maximum
            .combine(Heap {
                bytes: 1,
                ..Heap::default()
            })
            .is_err()
    );
    assert!(
        Heap {
            allocations: usize::MAX,
            ..Heap::default()
        }
        .combine(Heap {
            allocations: 1,
            ..Heap::default()
        })
        .is_err()
    );
    assert!(
        Heap {
            slack_bytes: usize::MAX,
            ..Heap::default()
        }
        .combine(Heap {
            slack_bytes: 1,
            ..Heap::default()
        })
        .is_err()
    );
}
