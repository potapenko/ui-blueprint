# K01 retained storage implementation receipt — in progress

Status: canonical sizing extraction saved; bounded Snapshot store author-checked
and source-frozen for checkpoint. Retained capability is implemented; peak-memory, live and independent acceptance remain separate.

## Authority and current basis

Finite [K01-storage packet](../packets/K01-storage.md), root dispatch `e70ce02`,
under approved PLAN.UIB@1 and explicit parallel-work authorization. Current master; no nested delegation, branches/worktrees, runtime or new dependencies.

Read changed registry5/decision route and complete D05@2 → D05-MEMORY@1 METRIC/LIMITS/OWNERSHIP/ADMISSION/LIFETIME/GATES, accepted engineering decision
`df998647bc73cd7d5d9cb5e1444a7b58f1dcbc71`. Reused fully read CACHE/LIFECYCLE/
EXCHANGE/C01-EVIDENCE and their complete prior K01-storage-design closure, D03/D04@1, RUST.md, DEV.RUST@2/D01@1/D07@2. Read execution-runbook clarification
assigning in-scope review repairs to their owners; it does not open their paths in this packet. Only D05-MEMORY-adopted choices are requirements; prior cache.md
ownership design remains a proposal where not adopted. Mode: Restore pinned norms.

## Coherent checkpoint A — canonical owned sizing

Existing exhaustive walker moved to `crates/schema/src/owned_size.rs` with only the crate import and module description changed. The complete walker body is
unchanged, verified against the tracked old file after excluding docs/imports. Canonical public API remains Heap/Overflow/HeapSize/owned, including exhaustive
matches/destructures and checked capacity arithmetic. No model/wire/validator edit.

Extraction paths: schema src/owned_size.rs + lib export; schema sizing test and measure_owned example; plugin resource_working example; resource README link;
removal of the old tests/bridges/resources/owned_memory.rs after import migration.

owned_size.rs SHA256:
`9aeaeee648b23a347ecc9a3e0c4c123d1910e4b9bd5dc3d3d0b881787efc141e`.
Six present-file hash-map digest:
`b718f0d49be93f4845c9a4a22e10c72bff53bf8bcb76cb23bb1724219638600d`.
Digest: compact sorted JSON path→SHA256 for six present files; old walker absence checked separately. No raw logs/evidence copies persisted.

Preliminary sizing3 tests and affected example compilation passed; the delegated shared barrier below supersedes that moving-working-tree check boundary.

## Delegated final barrier evidence

Integration was the sole runner, on root-confirmed frozen Core/Integration/E02 inputs at inspection HEAD `3a98c32bd2b7afc0acca0204ec3c855b89785007`.
Its [source receipt](S01-review-repair.md#final-coordinated-barrier) reports one passing Rust1.96.0 locked workspace check, fmt check, Clippy -D warnings and tests.
101 tests passed: schema22, plugin14, engine25, export18, CLI22; no failures/ignored. Core did not run this suite. Exact Core extraction hashes above were included.

Identical before/after211-file input digest:
`db7792faeea43667960657a53560c90fd47aa4f947cf1b62daec896dc5d35afe`.
The source receipt specifies path-set/hash reproduction, including additions and removals. This delegated proof is reusable only for matching saved owner inputs;
three ordered checkpoints and independent review remain separate acceptance.

## Saved extraction / barrier release

Extraction committed/pushed `ec0f35c927e9b34fb06313cc0f6c9e4ecaf36f4f`. Integration `2828cd8` and E02 `9ca645a` saved the other frozen owners; root confirmed
saved211-file digest matched the delegated101-test barrier and released Core. Canonical sizing/schema stayed unchanged during the following Store implementation.

## Checkpoint B — bounded Snapshot store

Real `engine::cache` owner: fixed grant/session/revision storage, move-only ledger allowances, borrowed reads, full canonical Context/channel/ID/revision keys,
actual-capacity accounting, size/validation/eviction planning before commit, requesting-session oldest eviction, immutable duplicate age, invalidation,
same-domain expiry, controlled replay admission, detach and final Grant release. Incoming data returns unchanged on rejection; no hidden payload clone/index.
The [API/accounting documentation](../../../development/cache.md) is now factual. Constructor caps are independent ceilings; only actual fixed backing/ownership
inconsistency refuses, without requiring every ceiling to saturate together.

Preliminary test setup was corrected to move its measured input rather than a capacity-shrinking clone; the independent capacity formula stayed unchanged.
Local Clippy repair only; no shared owner/source/oracle changes.

Root authorized the final scoped barrier; Core was runner on HEAD
`751ea3669757ebe32dd10a9b8ca44c5fdfdb2255` plus this Store candidate.
Rust1.96.0 locked engine all-target check, engine fmt check, engine all-target Clippy -D warnings and engine package tests all passed. Results:2 counter unit,
13 cache integration,14 geometry,11 replay tests;40 passed,0 failed/ignored; 1 compile-fail borrower proof passed. No full101 workspace suite rerun.

Identical before/after156-input digest:
`3c215a8fb68b6859edc7014e395529391c8940c14be27783390eb4bde4350fc3`.
Map: root Cargo.toml/Cargo.lock/rust-toolchain.toml, schema Cargo.toml, all regular files below schema/src, engine, fixtures/golden and fixtures/golden-oracles.
No .cargo files exist. Digest is SHA256 of compact sorted JSON path→SHA256 map. Own7-source-file digest (engine lib, tests/cache.rs, five cache/*.rs):
`e0048d9f175b87e7be007ae13998e08c1ccbea8c7ad0356d993aedd321f38998`.
Only docs changed after hashes/checks. Source is frozen until checkpoint/review.

Proof covers actual capacity/rollback/grant release, key/channel/generation and session isolation, FIFO/duplicate age, expiry, privacy/unavailable values and
independent full/replay equality. Borrow proof blocks detach while read lives.

Remaining: recorded reads do not provide live freshness; CurrentRequired explicitly requires external revalidation. Decoder/validator/replay/encoding working memory,
caller Completions and external pixels are not bounded by retained quotas. Separate derived caches, CLI resolution and live integration remain future packets. No fake
permit or subprocess adoption, no full K01/D05/live/independent acceptance claim. Exact pending checkpoint paths: engine src/cache/{mod,types,ledger,store,admission}.rs;
engine src/lib.rs only cache export; engine tests/cache.rs; docs/development/cache.md; this receipt. Store saved/pushed `f85f06f555772ecbb610acf6e66e8f0bf1c2a642`;
saved156-input digest matched the barrier. Source/engine lib stay frozen for review.

## Exact borrower diagnostic proof

Read-only reviewer found no actionable code defect but requested the diagnostic, not merely rustdoc's any-error success. Core ran this focused proof on Rust1.96.0
(ac68faa20), exact snippet from f85f06f store.rs, stripping only rustdoc prefixes.
Negative snippet SHA256 `50a2d7f6b6e6b2f4a820ee13e5d6f5ef37ccda6e217a550c1aa51091c3d6d2d2`:
exit1, sole coded error **E0502**, cannot borrow store mutably while immutably
borrowed; read line4, conflicting detach line5, later view use line6.
Positive control only moves the last view use before detach; SHA256
`9ffe8415853c673a2d22b755feae6c19fe701eeaa4bcc796083a935275bb1dd3`; exit0, no errors.
Focused locked engine library build selected the exact rlib; rustc used edition2024,
crate-type=lib, emit=metadata, error-format=json, -A dead_code, that --extern and
target/debug/deps dependency search. An initial wrong search path gave E0463;
it was corrected and excluded from proof. No source fix or broad test repeat.
Rlib SHA256 `5bdc2f8e09e6440eb4bc29a19b80c68cfd0ad090b65891da4f9af21b6c4f8c1e`;
all156 shared input hashes unchanged and equal to the saved barrier digest above.
Independent reviewer inspected exact snippets/diagnostics/result and returned
**ACCEPT** for retained Snapshot store f85f06f; this proof gap is closed, with no
actionable code findings. Root authorized this receipt-only checkpoint and cleanup
after successful push of Core's `uib-k01-borrower-kxuzev02` OS-temp directory.
Compact identity/results above remain; no raw logs committed. Product source stays
frozen. Full K01, live and decoder/replay/encoding peak gates are not accepted here.
