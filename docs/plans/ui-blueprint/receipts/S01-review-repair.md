# S01 shared review repairs — source-ready candidate

Status: coordinated final shared checks passed; checkpoint-ready, awaiting Git
grant. No independent acceptance or saved repair SHA yet.
D05 policy checkpoint df998647bc73cd7d5d9cb5e1444a7b58f1dcbc71 is already pushed.

## Authority, basis and ownership

[S01-review-repair packet](../packets/S01-review-repair.md), user clarification
recorded in execution.md36f2089, and original PLAN.UIB@1 authorize these Restore
repairs. The earlier read-only reviewer restriction did not require renewed user
permission for implementation within the approved plan. Current master only;
no nested agents/chats/worktrees, source-app operation, runtime or new dependency.

Reused the fully read AGENTS/spec registry -> product/decision/acceptance closure:
MODEL/EXCHANGE/IDENTITY/GEOMETRY/PROJECTIONS/FORMS/CACHE/ACTIONS/LIFECYCLE/
PRIVACY/BOUNDARIES/GOLDEN/ROADMAP@1 CONTENT and explicit dependencies; D03/D04@1,
saved D05@2/D05-MEMORY@1, RUST.md, DEV.RUST@2/D01@1/D07@2. No intended-behavior
change or schema cutover. Requirements come from those contracts; the four
[S01 findings](S01-user-review.md) and [P1-R1](P1-review.md) are defect evidence.
Historical reviewed line numbers were reconciled with actual current symbols.

Own writes: schema validation.rs, validation/graph.rs, validation/outcomes.rs;
schema tests/review_repairs.rs; plugin-api src/lib.rs and new src/depth.rs; this
receipt. Core exclusively owns canonical sizing extraction paths; Export owns
its two E02 repairs. No model/wire/schema-version/Cargo/fixture/oracle edits here.

## Concrete repairs

1. Geometric findings require a nonempty common coordinate-space destination,
   considering complete Space identity and explicit validated attached transforms.
   A transform must belong to the anchor node's surface to supply a destination.
   Equal units alone do not establish a common space. Baselines use their explicit
   space; this adds no arithmetic engine, fabricated evidence or result-space DTO.
2. The five geometric property fields require their matching Geometry.frame_kind.
   Geometry type alone cannot make AX bounds satisfy layout_bounds. Existing
   structural schema stays unchanged; this is the semantic validator's obligation.
3. DeltaCase compares the complete selected source-node map, revision, compatible
   Context, source_state, relations/focus/coverage and retained metadata. Added,
   removed and unchanged nodes all participate. Evidence observations must come
   unchanged from base/update, with all update observations present. Valid preserved
   historical observations remain legal. The private predicate also replaces the
   weaker duplicated GoldenChain checks; it does not publish another graph or
   depend on engine. Existing replay ownership/implementation is unchanged.
4. Resolution evidence must be reported/current and bind to the backend ref's
   Observation/source. Required enabled (and pointer hit-region) property evidence
   must be current, node-source-bound and cover the property. Separately observed
   current node properties remain valid; an unrelated current ref cannot upgrade
   stale enabled/writable/value_allowed/available_intents proof.
5. Depth uses a nonrecursive topological longest-path pass. Shared children retain
   the greatest incoming depth; every node/edge participates in bounded passes,
   O((V+E)log V) work/O(V) storage. Cycles/dangling children fail closed. Request
   limits are unchanged; no visited-only approximation or exponential-path walk.

## Local checks, not the final shared barrier

- Seven schema regression tests passed, covering independent negative mutations
  and positive neighboring cases for spaces/transforms, every frame kind, full
  delta source equivalence and current/source-bound action evidence.
- Three plugin depth tests passed: shared longest path and depth boundary;
 64-node/32-layer admitted DAG; cycle/dangling rejection. The new algorithm ran
  under a20-second owned-process-group guard; the old exponential case never ran.
- Four existing golden tests passed: independent oracle/126 wire cases, round-trip,
  structural/semantic parity, published schema equality and privacy/nonfinite checks.
- Scoped schema/plugin all-target check and Clippy -D warnings passed. The added
  wrong-surface transform neighbor subsequently passed its focused regression.

Core extraction and Export repairs were concurrently present during those local
checks. The final coordinated barrier below pins their exact frozen inputs;
all matching owner checkpoints must still be saved before integrated acceptance.
No live platform QA or hard decoder/process memory enforcement is claimed.

## Final coordinated barrier

Root confirmed source/test freeze for Integration, Core extraction and E02 repairs;
Integration was the sole final runner. Inspection HEAD3a98c32bd2b7afc0acca0204ec3c855b89785007,
with the three owners' assembled working changes. Rust1.96.0, current host.

All four authorized commands passed once against that frozen input set:

```sh
cargo +1.96.0 check --locked --workspace --all-targets
cargo +1.96.0 fmt --all -- --check
cargo +1.96.0 clippy --locked --workspace --all-targets -- -D warnings
cargo +1.96.0 test --locked --workspace
```

| Package | Tests passed / failed | Scope |
| --- | --- | --- |
| schema |22 /0 |2 validator CLI,4 golden/parity,3 canonical sizing,7 repairs,6 version |
| plugin-api |14 /0 |3 depth,3 shared IO support,8 lifecycle |
| engine |25 /0 |14 geometry,11 atomic replay |
| export |18 /0 |12 compiler,6 E02 proposal regressions |
| cli |22 /0 |11 existing binary,11 export binary including2 E02 regressions |

Total101 tests passed, zero failed/ignored; doctests passed with zero examples.
No failed check, source repair or repeated suite inside the barrier.

Full before/after input SHA256, identical across211 files:
`db7792faeea43667960657a53560c90fd47aa4f947cf1b62daec896dc5d35afe`.
Reproduction: sorted compact JSON path->file-SHA256 map, then SHA256 of that JSON.
Map includes root Cargo.toml/Cargo.lock/rust-toolchain.toml and all regular files
recursively under crates/, fixtures/golden/, fixtures/golden-oracles/,
fixtures/export/, schemas/, tests/bridges/common/, tests/bridges/resources/ and
.cargo/ if present. No target directory, raw output or runtime capture is included.
Added/removed files participate via map membership. Old diagnostic walker and
engine cache module absence were independently checked before the barrier.

Root-provided frozen owner identities all matched before running:
- Integration6 Rust paths:8e1caac8c8594c221c89a1740863ed84176c075a955acc9fd9c6b56c9849e056.
- E02 four-path set:444236409a5905687272c574217a629e731075d2439a35842326022bd0a27162.
- Core6 present extraction paths:b718f0d49be93f4845c9a4a22e10c72bff53bf8bcb76cb23bb1724219638600d;
  canonical walker9aeaeee648b23a347ecc9a3e0c4c123d1910e4b9bd5dc3d3d0b881787efc141e.
Source/test freeze remains in force for saving/review; only this receipt changed
after the final hash. Matching saved owner checkpoints reuse this proof without
repeating unchanged suites. Root retains final integrated saved-SHA reconciliation.

## Remaining acceptance and checkpoint

Root coordinates path-limited checkpoints and independent recheck of the same
five findings. Source needs no other owner
change or dependency to compile: graph::validate_delta_source is present and the
schema/plugin all-target check succeeds. Index is not reserved during development.
After Git grant, commit/push only the seven own paths and return exact SHA,
shared hashes/results and release. No repeat of unchanged checks without an input
change. Full D05 live enforcement, L01 result/context schema and later platform
acceptance remain separate; this packet repairs the scoped invalid-input behavior.
