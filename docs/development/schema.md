# S01 Stage A — candidate schema and validator

This is a reviewable implementation slice, **not an accepted or frozen contract**.
The four P2 findings in [the requested review](../plans/ui-blueprint/receipts/S01-user-review.md)
remain awaiting user authority and are not repaired by this stage. D02 Mac/Web
bridge proof and D05 resource calibration remain separate unfinished S01 gates.
Authority: [S01 packet](../plans/ui-blueprint/packets/S01.md), C01 D03/D07 and
[GOLDEN01](../specs/acceptance/golden.md). S01 extends the T01 schema owner in place.

## Canonical owners and validation layers

- [Rust records](../../crates/schema/src/model.rs) own wire types. `SchemaVersion`
  remains the T01 exact `0.1.0` boundary. Boxed payloads change Rust storage only.
- [Generated JSON Schema](../../schemas/uiblueprint-0.1.0.schema.json) describes
  structural shape. It is generated from those Rust records, never from examples.
- [Semantic validator](../../crates/schema/src/validation.rs), with graph/outcome
  modules, checks contextual constraints. Its known P2 gaps remain explicit below.
- [Observation plugin API](plugin-interface.md) uses the same records; no adapter DTO
  fork, OS SDK, runtime collector, geometry engine or graph cache is added.

`Document { schema_version, artifact }` carries a tagged artifact. Artifacts cover
request/session/capability, properties, observations, snapshots, geometry, delta,
actions/transitions, expectations/findings, errors, channel replies and GOLDEN01.
Validation bundles (`DeltaCase`, `ActionCase`, `FindingCase`, etc.) supply context
explicitly. They are validator inputs; transport can carry the same inner records
without embedding an entire history. No ID, payload_ref, file or URL inside a
document is dereferenced by the validator.

Properties distinguish selection from availability; known false and empty text
remain values. Unknown/unsupported/redacted carry no value. Tag-only records now
use strict empty struct variants, rejecting extra members instead of discarding
them. Focus has separate known target, known none, unknown and not_requested
states. Native roles, namespaced extensions and source declarations are preserved
separately. Sensitive declarations/properties require redacted state. This checks
declared sensitivity, not universal discovery of secrets in arbitrary UI text.

Identity IDs are nonempty strings bounded to256 Unicode scalar values; UI text is
not an ID. Context retains target/surface generations, scope/fields/projection,
plugin/schema versions and environment revision. Geometry keeps frame kind,
space/unit/origin and explicit transforms; length units stay px/css_px/pt/dp.
Declared result quantities distinguish length, area and ratio separately from
geometry units. Source clocks and parent deadlines are not silently compared.

Serde is the only JSON parser. Map-only record visitors reject array-shaped
records, duplicate/unknown members and invalid IDs without adding another parser.
Serde errors are discarded at the public `Document::from_json` boundary; returned
errors retain no raw input. JSON Schema is a structural checker, not a substitute
for semantic reference/evidence checks. Semantic-negative examples may therefore
be structurally valid; the manifest labels these two expectations separately.

## Independent oracle mapping and scope of proof

[Independent oracle corpus](../../fixtures/golden-oracles/README.md), checkpoint
`d539a0ec2bac243375256652fb75da7065b27350`, remains read-only to Integration.
[Wire manifest](../../fixtures/golden/manifest.json) maps97 case IDs to126 concrete
fixtures, expanding independent omission/variant cases. Expected exits are checked
against the author's expected.json, not inferred from our validator.
[build.py](../../fixtures/golden/build.py) translates those logical facts into the
single wire model; it is fixture tooling, not an analytics engine.

GOLDEN01 supplies S10, resolved checkbox action, delivery, S11 verification,
compatible delta, finding and limited compare export record. Error outcomes are
valid records: validator exit0 does not mean the requested operation succeeded.
Geometry amounts are declared authored oracle records; S01 checks representation,
evidence and result consistency. G01 must independently compute arithmetic and
K01 must implement replay/cache. Synthetic records do not prove real delivery,
platform capture, privacy across every channel or runtime acceptance.

## CLI and reproduction

```sh
cargo +1.96.0 run --locked -p uiblueprint-schema --bin uiblueprint-validate -- --max-bytes 1048576 fixtures/golden/GOLDEN01.json
cargo +1.96.0 run --locked -p uiblueprint-schema --example generate_schema
cargo +1.96.0 test --locked -p uiblueprint-schema -p uiblueprint-plugin-api
```

The byte limit is explicit, not a production default. `-` reads stdin. The CLI
reads at most limit+1 bytes, emits one payload-free JSON result on stdout, and
does not mix diagnostics with it. Exits:0 valid record,2 invalid/bounded-input
failure,1 IO/internal failure. No implicit URL fetching or schema loading occurs.
Saved-schema equality,126 CLI exits, round-trip/parity, IO/bounds and private-error
checks are covered by focused tests. Regenerate the schema only after changing
the canonical types; its equality test detects drift.

## Protected acceptance gaps

The four unchanged review findings concern cross-space geometric pass, field/frame
kind agreement, complete delta/source equivalence and fresh resolution evidence.
They prevent treating Stage A as accepted/frozen despite the covered tests passing.
D02 actual two-platform wire proof and D05 calibrated frame/retention/admission/
memory policy are unfinished. No native proof or other runtime was launched here.
