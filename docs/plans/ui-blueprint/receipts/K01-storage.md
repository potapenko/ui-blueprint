# K01 retained storage implementation receipt — in progress

Status: canonical sizing extraction source-ready; source write barrier held for
Integration's passing combined checks and coordinated saves. Cache implementation has not begun. No
retained-store, peak-memory, live or independent acceptance claim.

## Authority and current basis

Finite [K01-storage packet](../packets/K01-storage.md), root dispatch `e70ce02`,
under approved PLAN.UIB@1 and explicit parallel-work authorization. Current master;
no nested delegation, branches/worktrees, runtime or new dependencies.

Read changed registry5/decision route and complete D05@2 → D05-MEMORY@1
METRIC/LIMITS/OWNERSHIP/ADMISSION/LIFETIME/GATES, accepted engineering decision
`df998647bc73cd7d5d9cb5e1444a7b58f1dcbc71`. Reused fully read CACHE/LIFECYCLE/
EXCHANGE/C01-EVIDENCE and their complete prior K01-storage-design closure,
D03/D04@1, RUST.md, DEV.RUST@2/D01@1/D07@2. Read execution-runbook clarification
assigning in-scope review repairs to their owners; it does not open their paths
in this packet. Only D05-MEMORY-adopted choices are requirements; prior cache.md
ownership design remains a proposal where not adopted. Mode: Restore pinned norms.

Required: explicit limits within selected ceilings; one charged aggregate ledger,
move-only grants, exact owned capacities/fixed backing, borrowed reads, full keys,
staged admission, scoped eviction, monotonic expiry and detach/drop cleanup.
Working decoder/validation/replay/encoding enforcement remains a separate gate.
No fake permit is accepted as proof; no worker packaging chosen here.

## Coherent checkpoint A — canonical owned sizing

Existing exhaustive walker moved to `crates/schema/src/owned_size.rs` with only
the crate import and module description changed. The complete walker body is
unchanged, verified against the tracked old file after excluding docs/imports.
Canonical public API remains Heap/Overflow/HeapSize/owned, including exhaustive
matches/destructures and checked capacity arithmetic. No model/wire/validator edit.

Exact extraction paths:

- `crates/schema/src/owned_size.rs` (new canonical owner)
- `crates/schema/src/lib.rs` (only owned_size module export)
- `crates/schema/tests/owned_memory.rs` (canonical import)
- `crates/schema/examples/measure_owned.rs` (canonical import)
- `crates/plugin-api/examples/resource_working.rs` (canonical import)
- `tests/bridges/resources/README.md` (canonical source link)
- `tests/bridges/resources/owned_memory.rs` (removed after all imports migrated)

owned_size.rs SHA256:
`9aeaeee648b23a347ecc9a3e0c4c123d1910e4b9bd5dc3d3d0b881787efc141e`.
Six present-file hash-map digest:
`b718f0d49be93f4845c9a4a22e10c72bff53bf8bcb76cb23bb1724219638600d`.
The digest uses compact sorted JSON mapping the six paths above to their SHA256;
old diagnostic file absence is checked separately. Individual hashes returned in
chat for Integration; no raw output/log copies were persisted.

Preliminary checks performed before the shared final barrier:
`cargo check --locked -p uiblueprint-schema --all-targets -p uiblueprint-plugin-api --example resource_working`
and `cargo test --locked -p uiblueprint-schema --test owned_memory` passed (3 tests).
Own rustfmt and scoped whitespace checks passed. Those cover String/Vec reserved
capacity, Box/ZST/no-double-inline and arithmetic overflow. Shared validators were
being repaired separately, so these are not final integrated acceptance.

## Delegated final barrier evidence

Integration was the sole runner, on root-confirmed frozen Core/Integration/E02
inputs at inspection HEAD `3a98c32bd2b7afc0acca0204ec3c855b89785007`.
Its [source receipt](S01-review-repair.md#final-coordinated-barrier) reports one
passing Rust1.96.0 locked workspace check, fmt check, Clippy -D warnings and tests.
101 tests passed: schema22, plugin14, engine25, export18, CLI22; no failures/ignored.
Core did not run this suite. Exact Core extraction hashes above were included.

Identical before/after211-file input digest:
`db7792faeea43667960657a53560c90fd47aa4f947cf1b62daec896dc5d35afe`.
The source receipt specifies path-set/hash reproduction, including additions and
removals. This delegated proof is reusable only for matching saved owner inputs;
three ordered checkpoints and independent review remain separate acceptance.

## Current ownership / next step

Root requested a short source barrier before cache source/export work. Own
extraction source is frozen at the hashes above; engine cache module/export absent.
Integration completed the combined checks above. Root orders saves Integration →
Core extraction → Export, then reconciles the complete checked set. This receipt
alone changes during the barrier. Integration saved/pushed its matching checkpoint
`2828cd8eb91382235bd327e68f708c3cfbad316f`. Root granted the seven extraction paths
plus this receipt for the next save. No suite repeat. Terminal chat returns Core
SHA/push and Git release; source freeze continues through E02 save and root
checked-set reconciliation before Store work.

Checkpoint B still required: actual bounded Snapshot store and focused quota,
key/channel/generation, admission rollback, history/privacy, expiry/invalidation,
replay/resync and detach/drop tests under pinned D05. Geometry/replay/CLI/export,
Cargo and protected validator/depth paths remain outside Core's writes.
Source readiness is not packet completion; final SHA/push/checks/residual follows
actual implementation and the next granted checkpoints.
