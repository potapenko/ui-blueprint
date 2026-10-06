# K01-replay candidate receipt

Status: first checked pure replay candidate ready for root Git grant.
This is not K01 storage, S01 acceptance, D05 policy or a review-repair claim.

## Authority and traversal

Finite [packet](../packets/K01-replay.md), dispatched from root after `92aab9d`.
Original PLAN.UIB@1/launch and parallel authorization remain in the
[registry](../task-registry.md). The approved plan permits early GOLDEN01 cache
work; this packet explicitly bounds it to pure replay. Current master only,
no nested delegation, runtime, new dependencies/crates or shared manifest writes.

AGENTS → spec registry → product/decision/acceptance routes. Reused the full
current G01/L01 CACHE@1, EXCHANGE@1, PROJECTIONS@1, LIFECYCLE@1,
MODEL/IDENTITY/BOUNDARIES/PRIVACY@1, D03/D05@1, GOLDEN@1 and explicit closure;
RUST.md, DEV.RUST@2, D01@1/D07@2 and their explicit dependencies.
Read missing D04@1 → NATIVE@1 and its already-read EXCHANGE/GEOMETRY/
PROJECTIONS/LIFECYCLE dependencies. Read complete updated D05@1: added sizing
status does not change CONTENT policy; numeric production caps remain open.
All selected CONTENT clauses remain Active/Evolving, not released acceptance.
Excluded: storage/eviction policy, runtime/input, exports and projection heuristics.

Mode: implement existing atomic-delta norms, no spec delta. Supporting evidence:
R03 `26a19fa` AccessKit update ledger/U1/U2, independent oracle `d539a0e`,
canonical model/shared graph validator, selected full/delta fixture records.
No upstream source copied. Source/model/validator paths still match Stage A
`9d2df15`; no protected-owner repair. Existing canonical validator remains the
validation authority, with engine-owned immutable apply/publication semantics.

## Delivered capability and boundary

Public replay consumes canonical base+delta and caller-provided new Snapshot ID,
returns a complete validated Snapshot or typed resync/error, and leaves base
unchanged. It performs whole selected-node replacement, explicit justified
removals, atomic children/relations/focus and deterministic node ordering.
Referenced historical Observations retain original provenance/intervals; old
values cannot fill unavailable replacements. Conflicting reuse of Observation
identity refuses replay. No implicit deletion from partial/narrow/absent data.

Delta cannot replace components/surface records/captures. They remain historical
base records; dangling retained references require a full Snapshot/resync, not
invented metadata cleanup. This is an explicit current representation boundary.
No blocker for the supported pure replay operation; a richer delta record belongs
to Integration if a future consumer requires metadata mutation through deltas.
See [replay API documentation](../../../development/replay.md).

Exact writes: `crates/engine/src/replay.rs`, `crates/engine/src/lib.rs` only the
`pub mod replay;` export, `crates/engine/tests/replay.rs`,
`docs/development/replay.md`, this receipt. Arithmetic/resolve/G01 tests, CLI,
schema/plugin code, fixtures/oracles, root Cargo/lock remain untouched.

## Verification

Rust1.96.0; scoped working-tree candidate against saved shared inputs; HEAD at
verification `51c780959907dc85f6e2c5998850c25964ce7296`:

- `cargo check --locked -p uiblueprint-engine --all-targets`: pass.
- `cargo test --locked -p uiblueprint-engine --test replay`: 11 passed, 0 failed.
- `cargo clippy --locked -p uiblueprint-engine --all-targets -- -D warnings`: pass.
- `cargo fmt -p uiblueprint-engine -- --check`: pass.
- Changed local links and scoped `git diff --check`: pass.

Initial compile revealed canonical FocusRef::Unknown; added its explicit
no-evidence branch. No shared change. Independent exact full/delta equality,
R03 reparenting, known→unknown/redacted/unsupported/false/empty, context and
revision resync, incomplete/removal/reference rejection, unchanged base,
new-node order, explicit empty scope and historical capture evidence passed.
No live equivalence or performance claim. Unchanged geometry logic suites were
not rerun; compilation/Clippy covered the public module export.

Five protected review findings remain awaiting repair authority. D05 must define
real retention/memory/queue/eviction policy before a storage owner is implemented.
Root granted exactly the five own paths listed above for checkpoint+push.
The commit containing this receipt saves only the pure replay candidate;
SHA/push/index-clean confirmation and Git release return in the terminal chat
receipt for root. Base immutability/evidence tests do not close storage, eviction,
D05 or the five protected findings. Replay edits stop after push until handoff.
