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

## G10 public neighbor caller — 2026-10-08

Finite follow-through assigned to existing CLI/Export worker under G02 packet's
G10 addendum, root4373187 and original PLAN.UIB@1 P3/P6 authority. Restore explicit
connected context; concrete additive CLI representation registered before source
as CLI@9/CLI-NEIGHBORS@1, registry21, G10-NEIGHBORS-001. Existing engine meaning,
core0.1/analysis0.2, G09 export and numerical/privacy/source invariants unchanged.

Traversal: current registry20 → product branch → CLI@8 and PROJECTIONS@1 →
MODEL/IDENTITY/EXCHANGE/PRIVACY/BOUNDARIES full retained closure, RUST/DEV.RUST@2;
G10 packet/source receipt and actual accepted scope API read. Root instructions
and applicable governance reused from immediately preceding G09 with no drift.
No action/collection, generic proximity, live UI or new data-contract owner.

Source reconciliation: inspect already called relation_neighbors with all recorded
relations, showing keys in compact; JSON inspection retained only the original
Snapshot. Reused its strict Snapshot/observed-response loader and SourceKey parsing.
Remaining missing capability was explicit cap plus a structured neighbor selection
and counterpart data. New `neighbors` dispatch is a thin caller to the SAME engine,
not a duplicate graph/selection algorithm; inspect semantics/envelope preserved.

```sh
uiblueprint neighbors --snapshot observation.json \
  --ref '{"namespace":"web.dom","key":"7"}' --max-relations 1 \
  --max-input-bytes 200000 --max-output-bytes 200000 --json
```

Borrowed output preserves the full Snapshot separately from selection (explicit
cap/returned/omitted/truncated), plus ordered incoming/outgoing/self_loop relations,
full counterpart Nodes and exact source Evidence. Duplicate counterpart entries
remain one per edge. Cap0 is valid. Source partial/unknown is independent and never
upgraded; no new stable/actionable ref, inferred identity or current observation.
JSON output_version1.0.0 is a documented CLI envelope, not a normalized schema.
Byte bounds include selector input and complete output/newline; cap cannot make
oversized source output bypass the writer. Missing seed4; invalid/limits2; IO1;
valid capped/partial selection0 with truthful statuses. Errors retain no payload.

Exact12 paths: crates/cli/src/{arguments,input,main,output}.rs;
crates/cli/tests/neighbors_binary.rs; docs/specs/README.md and
product/{README,cli,cli-neighbors}.md; docs/development/{cli,scope}.md; this receipt.
No engine/schema/export/provider/fixtures/manifests/root coordination writes.

Focused verification, Rust1.96:
- `cargo check --locked -p uiblueprint-cli --bin uiblueprint`: pass.
- `cargo fmt -p uiblueprint-cli -- --check`: pass.
- `cargo clippy --locked -p uiblueprint-cli --bin uiblueprint --test neighbors_binary -- -D warnings`: pass.
- `cargo test --locked -p uiblueprint-cli --test neighbors_binary`:4 passed.
- `cargo test --locked -p uiblueprint-cli --test binary inspect`:4 passed,
  preserving actual inspect output/identity/unknown/byte/privacy behavior.
No live/whole-workspace/numerical/review wave or unrelated suite was run.

New binary cases independently assert directions/order/self-loop/repeated links,
exact relation Evidence/counterpart Nodes and unchanged Snapshot, cap0/1 separate
from partial/unknown coverage, redaction, missing exact namespace/key, sanitized
invalid input and exact aggregate input/output byte boundaries without truncation.
Test-only canonical derivatives are not live proof; their known nonimage temp
source files and empty directories are removed by the case owner.

One actual public command used the original saved F01 D05 overlay-on response,
SHA256 `11e2a77cf6460636050e90ed0f6f63bcf3412c062bb3eb917d677be035858180`.
Seed web.dom:7, cap1 returned outgoing corresponds_to → web.ax:7; one incident
relation omitted. Full source32nodes/17relations and partial/unknown_count=null
remained structurally identical after decoding; source file bytes unchanged. JSON80524B,
exit0. No output file, image or runtime collection was created. This historical
controlled fixture is not live PlayPhrase.me or complete interface acceptance.

Input identity: inspection HEAD4373187; unchanged scope.rs SHA256
`b9dfd68d682d701f9e2f4dbb7813890d32651eaf7c14be671166933024426b44`, matching
accepted53c6f74. Relevant60-file Rust map SHA256
`7028b5857ca5c095d68bc4a1ec11308a0fa9de8560f28f9e38aab7fac52b9566`:
root Cargo.toml/Cargo.lock/rust-toolchain.toml and every .rs/Cargo.toml under
crates/{schema,engine,cli}, sorted compact JSON path→file-SHA256 then SHA256.
Protected owner diffs were empty. Source is frozen checkpoint-ready; root grants
short exact-path commit+push, then terminal SHA/push/release. Remaining full
projections/spatial heuristics/live qualification/P7 are separate, not claimed.
