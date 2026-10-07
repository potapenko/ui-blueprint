# G02 explicit relation neighbors — author checkpoint

Authority: [finite G02 packet](../packets/G02-scope.md), root acceptance of the
read-only source handoff under user-approved PLAN.UIB@1. Restore, no contract delta.
Traversal: AGENTS → registry11 → product branch → PROJECTIONS/GEOMETRY/MODEL/
IDENTITY/BOUNDARIES@1 plus NATIVE→EXCHANGE/LIFECYCLE/PRIVACY explicit closure,
D03@2/core-data dependencies, RUST.md/DEV.RUST@2. Existing analysis contracts were
read to protect the adjacent accepted consumer; this slice does not change them.

Exact5 paths: crates/engine/src/scope.rs, crates/engine/src/lib.rs (module export
only), crates/engine/tests/scope.rs, docs/development/scope.md and this receipt.
No host/Web/Native/schema/Cargo/lock/spec/registry writes. Immediate consumer is
stored canonical scope selection and later G02 views, independently of live collectors.

`scope::relation_neighbors(&Snapshot, &SourceKey, NeighborLimits)` returns a
borrowed NeighborView with exact source/seed, ordered incident relations and their
counterpart Nodes/direction, plus the number omitted by max_relations. Original
source coverage and all availability/provenance remain borrowed. No edge or mapping
is inferred; no graph, identity, action ref or canonical projection is manufactured.
Details and explicit ownership limits are in [scope.md](../../../development/scope.md).

Six focused tests have been added: literal LabelledBy/ErrorFor direction/order;
many-to-many cross-namespace links and estimated Evidence preservation; equal
names/boxes and child/component membership without invented links; independent
selection truncation/source partial/unknown coverage and redaction preservation;
zero cap/self-loop/repeated-edge counts; missing seed and invalid graph refusal.
Inputs are synthetic derivatives of the unchanged canonical ENV-SNAPSHOT-VALID
fixture, not live measurements. No snapshots or auxiliary run logs are persisted.

Author verification passed on Rust1.96.0, aarch64-apple-darwin, with a separate
task-temporary target outside the repository (removed after handoff acceptance).
Commands used `--locked --offline -p uiblueprint-engine` and that target directory:

- `cargo check --lib --test scope`: passed.
- `cargo test --test scope`:6 passed,0 failed.
- `cargo clippy --lib --test scope -- -D warnings`: passed.
- Scoped rustfmt, changed local links and `git diff --check`: passed.

The first test compile found a local Box<Geometry> constructor mismatch; corrected
to the existing canonical type before the above results. No schema/API change or
test relaxation. No broad suite, live run or additional agent/reviewer was used.

Self-review: implementation borrows every Node/Relation/Evidence and retains exact
source identity/coverage. The only new owner is a fallibly reserved Vec of borrowed
entries, sized from the actual incident count and explicit cap; zero and oversized
caller caps cannot require more entries than present. Validation uses the canonical
owner. Full source validation precedes seed lookup; no partial result escapes a
failure. Repeated edges and self-loops preserve source semantics. No Host/Cargo/
canonical wire/cache/analysis arithmetic behavior changes. This is author review,
not independent acceptance; none was requested for this finite slice.

SHA256 input pins:

| Path | SHA256 |
| --- | --- |
| crates/engine/src/scope.rs | b9dfd68d682d701f9e2f4dbb7813890d32651eaf7c14be671166933024426b44 |
| crates/engine/src/lib.rs | d0c8292f58b5abda0ad98bb50bd343828e81dafdca8b67b7f99b1399f1ad30f2 |
| crates/engine/tests/scope.rs | 8dccb6f3b23a56f79a26f07eb3d601c528ca53503b6008e226537cfaa55991ac |
| fixtures/golden/ENV-SNAPSHOT-VALID.json (unchanged) | 098509c30077271d2d02158b283b1e3f48b5fd88b61d787a884de988df2a33c5 |

Residual: this supplies explicit one-step relation selection on stored canonical
data only. Full interaction/design views and live W01/M01/P01 source qualification
remain open. Root switched Web to a saved source snapshot excluding G02 WIP, so
there is no outstanding Core source hold or unsaved-input dependency on this slice.
