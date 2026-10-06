//! Authored capacity inputs, never live/platform samples. Tree edges have no
//! shared descendants: the protected P1-R1 DAG/hang is deliberately not exercised.
use uiblueprint_schema::{SchemaVersion, model::*};

pub fn snapshot(nodes: usize, depth: usize) -> Snapshot {
    let doc = Document::from_json(
        include_bytes!("../../../fixtures/golden/ENV-SNAPSHOT-VALID.json"),
        1_048_576,
    )
    .expect("committed authored fixture");
    let Artifact::Snapshot(mut s) = doc.artifact else {
        panic!("authored snapshot");
    };
    assert!(depth > 0 && nodes >= depth);
    s.id = Id(format!("synthetic-{nodes}-depth-{depth}"));
    s.context.plugin.id = Id("synthetic-capacity".into());
    s.observations[0].source_namespace = Id("synthetic.capacity".into());
    let template = s.nodes[0].clone();
    s.nodes.clear();
    for i in 0..nodes {
        let mut node = template.clone();
        node.key = SourceKey {
            namespace: Id("synthetic.capacity".into()),
            key: Id(format!("n{i}")),
        };
        for p in &mut node.properties {
            if let Property::Requested {
                evidence,
                field,
                state,
                ..
            } = p
            {
                evidence.source_namespace = Id("synthetic.capacity".into());
                evidence.method = Id("authored_capacity_input".into());
                if *field == Field::Name {
                    *state = Availability::Known {
                        value: Value::Text(format!("Синтетический 😀 {i}")),
                    };
                }
                if *field == Field::Checked {
                    *state = match i % 4 {
                        1 => Availability::Unknown {
                            reason: Id("synthetic_unknown".into()),
                        },
                        2 => Availability::Unsupported {
                            reason: Id("synthetic_unsupported".into()),
                        },
                        3 => Availability::Redacted {},
                        _ => Availability::Known {
                            value: Value::Flag(false),
                        },
                    };
                }
            }
        }
        s.nodes.push(node);
    }
    // Chain to depth-1, then all remaining nodes as leaves at exactly depth.
    for i in 0..depth.saturating_sub(2) {
        s.nodes[i].children = vec![s.nodes[i + 1].key.clone()];
    }
    if depth > 1 {
        s.nodes[depth - 2].children = s.nodes[depth - 1..].iter().map(|n| n.key.clone()).collect();
    }
    *s
}
pub fn document(snapshot: Snapshot) -> Document {
    Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Snapshot(Box::new(snapshot)),
    }
}
pub fn sorted_json(document: &Document) -> Vec<u8> {
    serde_json::to_vec(&serde_json::to_value(document).expect("value")).expect("sorted JSON")
}
pub fn escaped_json(document: &Document) -> Vec<u8> {
    use std::fmt::Write;
    let text = serde_json::to_string(document).expect("wire");
    let mut result = String::new();
    for c in text.chars() {
        if c.is_ascii() {
            result.push(c);
        } else {
            for unit in c.encode_utf16(&mut [0; 2]) {
                write!(result, "\\u{unit:04x}").expect("String write");
            }
        }
    }
    result.into_bytes()
}

pub fn dense_text_list(items: usize) -> Document {
    let mut s = snapshot(1, 1);
    s.context.fields.push(Field::Actions);
    s.coverage.fields = s.context.fields.clone();
    s.observations[0].coverage.fields = s.context.fields.clone();
    let Property::Requested { evidence, .. } = &s.nodes[0].properties[0] else {
        panic!("authored property")
    };
    let evidence = evidence.clone();
    s.nodes[0].properties.push(Property::Requested {
        field: Field::Actions,
        sensitivity: Sensitivity::Public,
        evidence,
        state: Availability::Known {
            value: Value::TextList(vec![String::new(); items]),
        },
    });
    document(s)
}
pub fn long_text(bytes: usize) -> Document {
    let mut s = snapshot(1, 1);
    for p in &mut s.nodes[0].properties {
        if let Property::Requested {
            field: Field::Name,
            state,
            ..
        } = p
        {
            *state = Availability::Known {
                value: Value::Text("x".repeat(bytes)),
            };
        }
    }
    document(s)
}
/// Find the largest authored payload of this family within the unchanged frame
/// limit. It is synthetic schema stress, not a claim about any live application.
pub fn fit_frame(cap: usize, make: impl Fn(usize) -> Document) -> Document {
    let (mut low, mut high) = (0, cap);
    while low < high {
        let middle = low + (high - low).div_ceil(2);
        if serde_json::to_vec(&make(middle))
            .expect("synthetic JSON")
            .len()
            <= cap
        {
            low = middle;
        } else {
            high = middle - 1;
        }
    }
    make(low)
}
