# K01 canonical retained storage implementation

- Class: shipping_product; existing Core owner, inherit model/reasoning.
- Outcome: real process-independent bounded Snapshot store for subsequent local
  session/CLI and adapters, with canonical owned-size API and quota lifecycle.
- Authority: PLAN.UIB@1 at358c757 and explicit parallel-work request; ROADMAP
  D05 delegated policy now pinned D05@2/D05-MEMORY@1 (D05-RET-002), saved checkpoint
  supplied in dispatch. Master only, no nested agents/other chats/worktrees.
- Ready: replaye4ecee7, exhaustive sizingc6a5df6/working2a5dfef, Core design0d40af7,
  policy accepted by root as engineering decision, not completed implementation.
- Economy: extract the existing single exhaustive sizing owner and implement the
  concrete store; do not build a simulator, alternative graph or host framework.

## Final Spec Basis and protected domains

Root reused/read full K01-storage-design packet closure, then read changed
spec-root registry5 → decision route → D05@2 → full D05-MEMORY@1 clauses
METRIC/LIMITS/OWNERSHIP/ADMISSION/LIFETIME/GATES, with CACHE/LIFECYCLE/EXCHANGE/
C01-EVIDENCE explicit closure already current. Read current policy and handoff
before source. Other CONTENT@1 contracts, D03/D04@1, DEV.RUST@2/D07@2 unchanged.
The design doc was a proposal; only choices adopted in D05-MEMORY are requirements.
Mode Restore those contracts; no product/wire semantic change or new dependency.

Required policy: explicit constructor limits at/below the selected ceilings,
one aggregate retained allowance ledger, move-only grants, charged fixed storage
and owned payload capacities, nonduplicated usage versus reservations, full Context
and explicit channel partition keys, borrowed reads, admission before mutation,
scoped oldest-admission eviction, resync on lost base, no false deletions, no TTL
freshness promotion, exact same-domain expiry and detach/drop cleanup. Preserve
known/false/empty/unknown/redacted values and historical Observations unchanged.
Read every D05-MEMORY clause: this paragraph is navigation, not a replacement.

The store does not establish decoder/validation/replay/encoding peak-memory bounds.
Incoming owned payload and Completion charge remain caller-owned until atomic
transfer; do not imply an opaque permit enforces actual transient allocation.
Use canonical validation and pure replay. No raw pixel/wire-history cache, process
worker, live adapter, polling, derived geometry/check cache API or CLI ID resolver
in this finite slice. These remain original goal obligations, not waived gates.

## Exact ownership

Root temporarily delegates these disjoint shared paths to Core while Integration
repairs validation.rs/validation/** and plugin src/lib/depth; do not edit those.

1. Move the existing safe exhaustive walker to crates/schema/src/owned_size.rs;
   crates/schema/src/lib.rs only its module export. Preserve the policy's
   Heap/Overflow/HeapSize/owned API and exhaustive coverage, no new field inventory.
2. Migrate crates/schema/tests/owned_memory.rs, examples/measure_owned.rs and
   crates/plugin-api/examples/resource_working.rs to the canonical API. Remove
   tests/bridges/resources/owned_memory.rs only after its own imports migrate;
   adjust tests/bridges/resources/README.md links. No unrelated diagnostic changes.
3. crates/engine/src/cache.rs or cache/**, engine lib.rs only cache export,
   crates/engine/tests/cache.rs; docs/development/cache.md and
   docs/plans/ui-blueprint/receipts/K01-storage.md.

No Cargo/lock/dependencies, schema models/wire/validators, geometry/replay source,
CLI/export, fixtures/oracles, product specs or other receipts. Required out-of-scope
dependency returns to root before mutation. Shared field support gap is not permission
to weaken accounting or copy a second validator. Nested delegation prohibited.

## Focused proof and checkpoints

Reused sizing tests plus meaningful missing boundary cases: String/Vec reserved
capacity, nested/Box/ZST and checked overflow. Store tests cover actual fixed
capacity and rollback; exact byte/count ceilings; grants across Stores/session
slots, no early quota reuse; full-key/channel/generation isolation; unchanged
rejected admission and scoped eviction; identical duplicate age versus conflicting
identity; expired/invalidated reads and monotonic regression/domain/overflow;
detach/drop frees payloads but keeps vacant backing charged while Store lives;
unknown/redacted/false/empty persistence; old/evicted base returns resync.
No fake allocation permits, self-computed oracles or simulated retained owner.

Use Rust1.96 locked scoped check/fmt/Clippy; sizing and cache tests, affected example
compilation. Integration repairs shared validators and Export repairs proposal
arithmetic in parallel; prepare tests but coordinate final check barrier with root.
Record exact shared input hashes and repeat only affected checks after their change.
Independent review of quota ownership/lifecycle follows saved implementation;
self-tests are not acceptance. No runtime/browser/model call required or claimed.

Save coherent sizing extraction before/alongside store only with root Git grant,
exact agreed paths, commit AND push; return SHA/checks/residual/resource release.
Finish this finite capability or exact dependency, not a plan. Stop before next
cache purpose or host enforcement packet. Retained-store completion is not whole
K01, full process memory enforcement, or live P4 acceptance.
