//! R03 finite counterexample, not a production schema, cache, or wire protocol.
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Value {
    Flag(bool),
    Text(&'static str),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Requested {
    Known(Value),
    Unknown,
    Unsupported,
    Redacted,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Selection<'a> {
    Requested(&'a Requested),
    NotRequested,
}

type Fields = BTreeMap<&'static str, Requested>;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Sample {
    revision: u32,
    fields: Fields,
}

#[derive(Debug, PartialEq, Eq)]
enum Reject {
    ResyncRequired,
    IncompleteReplacement,
}

fn select<'a>(sample: &'a Sample, field: &str) -> Selection<'a> {
    match sample.fields.get(field) {
        Some(value) => Selection::Requested(value),
        None => Selection::NotRequested,
    }
}

// The fixture fixes target, surface, scope, projection and schema/plugin version.
// Only revision and field selection vary. S01/K01 must validate the full context.
fn replace(
    current: &mut Sample,
    base: u32,
    fields: &[&str],
    replacement: Fields,
) -> Result<(), Reject> {
    let current_fields: Vec<_> = current.fields.keys().copied().collect();
    if base != current.revision || fields != current_fields {
        return Err(Reject::ResyncRequired);
    }
    if replacement.keys().copied().collect::<Vec<_>>() != current_fields {
        return Err(Reject::IncompleteReplacement);
    }
    // Publish only after validating the entire (single-node) replacement.
    *current = Sample {
        revision: base + 1,
        fields: replacement,
    };
    Ok(())
}

fn known(value: Value) -> Requested {
    Requested::Known(value)
}

fn experiment() {
    // Independent expected states and the counterfactual are in README.md.
    let historical = Sample {
        revision: 7,
        fields: BTreeMap::from([
            ("enabled", known(Value::Flag(true))),
            ("label", known(Value::Text("old"))),
        ]),
    };
    let original_history = historical.clone();
    let mut current = historical.clone();
    replace(
        &mut current,
        7,
        &["enabled", "label"],
        BTreeMap::from([
            ("enabled", known(Value::Flag(false))),
            ("label", known(Value::Text(""))),
        ]),
    )
    .expect("complete same-context replacement must apply");
    assert_eq!(current.revision, 8);
    assert_eq!(current.fields["enabled"], known(Value::Flag(false)));
    assert_eq!(current.fields["label"], known(Value::Text("")));

    for status in [
        Requested::Unknown,
        Requested::Redacted,
        Requested::Unsupported,
    ] {
        let mut next = current.clone();
        replace(
            &mut next,
            8,
            &["enabled", "label"],
            BTreeMap::from([
                ("enabled", known(Value::Flag(false))),
                ("label", status.clone()),
            ]),
        )
        .expect("non-known is a complete property state");
        assert_eq!(next.fields["label"], status);
        assert_eq!(next.revision, 9);
    }

    let before_rejection = current.clone();
    assert_eq!(
        replace(&mut current, 6, &["enabled", "label"], Fields::new()),
        Err(Reject::ResyncRequired)
    );
    assert_eq!(current, before_rejection);
    assert_eq!(
        replace(
            &mut current,
            8,
            &["enabled"],
            BTreeMap::from([("enabled", known(Value::Flag(false)))]),
        ),
        Err(Reject::ResyncRequired)
    );
    assert_eq!(current, before_rejection);
    assert_eq!(
        replace(
            &mut current,
            8,
            &["enabled", "label"],
            BTreeMap::from([("enabled", known(Value::Flag(false)))]),
        ),
        Err(Reject::IncompleteReplacement)
    );
    assert_eq!(current, before_rejection);

    // A new field selection requires a full sample in its own context.
    let narrow_full = Sample {
        revision: 9,
        fields: BTreeMap::from([("enabled", known(Value::Flag(false)))]),
    };
    assert_eq!(select(&narrow_full, "label"), Selection::NotRequested);
    assert_eq!(historical, original_history);
    assert_eq!(historical.revision, 7);
    assert_eq!(historical.fields["label"], known(Value::Text("old")));

    // Falsify "merge selected values into old node and mark it fresh".
    let mut naive_fresh = historical.clone();
    naive_fresh.fields.extend(narrow_full.fields.clone());
    naive_fresh.revision = 9;
    assert_ne!(select(&naive_fresh, "label"), Selection::NotRequested);
    assert_ne!(naive_fresh, narrow_full);
}

fn main() {
    experiment();
    println!("R03 selected-update: expected states verified; naive fresh merge falsified");
}
