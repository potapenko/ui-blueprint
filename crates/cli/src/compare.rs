//! Local transport decoding and command composition, never another comparison engine.
use crate::{Failure, arguments::CompareArguments, input, output};
use serde::{Deserialize, Deserializer};
use uiblueprint_engine::diff::{self, CorrespondenceError as Error};
use uiblueprint_schema::{analysis::*, model::*};

// The same map-only Serde boundary used by canonical/local operator records.
// No JSON Value conversion: duplicate fields must survive to typed rejection.
macro_rules! record {
    ($name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
        struct $name { $($field: $ty),* }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                #[derive(Deserialize)]
                #[serde(deny_unknown_fields)]
                struct Fields { $($field: $ty),* }
                struct MapOnly;
                impl<'de> serde::de::Visitor<'de> for MapOnly {
                    type Value = $name;
                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("object record") }
                    fn visit_map<A: serde::de::MapAccess<'de>>(self, a: A) -> Result<$name, A::Error> {
                        let f = Fields::deserialize(serde::de::value::MapAccessDeserializer::new(a))?;
                        Ok($name { $($field: f.$field),* })
                    }
                }
                d.deserialize_map(MapOnly)
            }
        }
    };
}
record!(Binding {
    snapshot_id: Id,
    revision: u64,
    context: Context
});
record!(SurfacePair {
    before: Identity,
    after: Identity,
    declaration: Id
});
record!(NodePair {
    id: Id,
    before: SourceKey,
    after: SourceKey,
    declaration: Id
});
record!(MeasurementPair {
    id: Id,
    before_query: GeometryQuery,
    after_query: GeometryQuery,
    before_evaluation: EvaluationInput,
    after_evaluation: EvaluationInput
});
record!(Correspondence { correspondence_version: String, before_binding: Binding, after_binding: Binding,
    surface_pairs: Vec<SurfacePair>, node_pairs: Vec<NodePair>, measurement_pairs: Vec<MeasurementPair> });
impl From<Binding> for diff::SnapshotBinding {
    fn from(v: Binding) -> Self {
        Self {
            snapshot_id: v.snapshot_id,
            revision: v.revision,
            context: v.context,
        }
    }
}
fn convert<T, U>(items: Vec<T>, f: impl Fn(T) -> U) -> Result<Vec<U>, Failure> {
    let mut result = Vec::new();
    result
        .try_reserve_exact(items.len())
        .map_err(|_| Failure::io())?;
    result.extend(items.into_iter().map(f));
    Ok(result)
}
pub(crate) fn decode(bytes: &[u8]) -> Result<diff::Correspondence, Failure> {
    let c: Correspondence = serde_json::from_slice(bytes).map_err(|_| {
        Failure::invalid(
            "invalid_correspondence; use strict correspondence1.0 objects and original bindings",
        )
    })?;
    if c.correspondence_version != "1.0.0" {
        return Err(Failure::invalid(
            "unsupported_correspondence_version; use correspondence_version 1.0.0",
        ));
    }
    Ok(diff::Correspondence {
        before_binding: c.before_binding.into(),
        after_binding: c.after_binding.into(),
        surface_pairs: convert(c.surface_pairs, |p| diff::SurfacePair {
            before: p.before,
            after: p.after,
            declaration: p.declaration,
        })?,
        node_pairs: convert(c.node_pairs, |p| diff::NodePair {
            id: p.id,
            before: p.before,
            after: p.after,
            declaration: p.declaration,
        })?,
        measurement_pairs: convert(c.measurement_pairs, |p| diff::MeasurementPair {
            id: p.id,
            before_query: p.before_query,
            after_query: p.after_query,
            before_evaluation: p.before_evaluation,
            after_evaluation: p.after_evaluation,
        })?,
    })
}
fn failure(error: Error) -> Failure {
    match error {
        Error::InvalidSnapshot => Failure::invalid("invalid_input"),
        Error::BindingMismatch => Failure::invalid(
            "correspondence_binding_mismatch; use each original snapshot_id, revision and full context; compare --diagnose explains source compatibility",
        ),
        Error::ProtectedBasis => Failure::invalid(
            "protected_basis_incompatible; run compare --diagnose on the same inputs",
        ),
        Error::InvalidPair => Failure::invalid(
            "invalid_correspondence_pair; use unique existing same-namespace endpoints within explicitly paired surfaces",
        ),
        Error::InvalidMeasurement => Failure::invalid(
            "invalid_measurement_pair; use independently bound width, height or gap queries over the declared pairs",
        ),
        Error::Capacity => Failure::io(),
        Error::NonFinite => Failure {
            code: "nonfinite_geometry",
            exit: 1,
        },
    }
}
pub(crate) fn execute(args: CompareArguments) -> Result<(Vec<u8>, u8), Failure> {
    let loaded = input::load_comparison(&args)?;
    if let Some((correspondence, bytes)) = &loaded.correspondence {
        let result = diff::compare_corresponded(
            &loaded.before,
            &loaded.after,
            correspondence,
            args.max_entries
                .ok_or(Failure::invalid("invalid_arguments"))?,
        )
        .map_err(failure)?;
        let exit = if result.omitted_entries == 0 && result.unknown_measurements == 0 {
            0
        } else {
            4
        };
        Ok((output::corresponded(&args, &result, bytes)?, exit))
    } else {
        let result = diff::diagnose_comparison(&loaded.before, &loaded.after).map_err(failure)?;
        let exit = if result.strict_diff_compatible { 0 } else { 4 };
        Ok((output::comparison_diagnostic(&args, &result)?, exit))
    }
}
