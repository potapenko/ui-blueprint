# H01 allocation guard source review

Class verification; one fresh reviewer, inherited settings, no nested agents.
User explicitly authorized coordinated PLAN.UIB@1 implementation and reviews.
Immediate consumer: independent guard-source evidence for H01 before live use.
Economy: review the stable allocator/counter mechanism while producer wiring changes
elsewhere; later integration review reuses it unless affected evidence changes.

Read-only: no file/Git mutation, builds/tests/probes, network/browser/SDK/process
operation or external posts. Include relevant staged/unstaged/untracked drift
without modifying it. Root records the receipt. No reviewer shopping/fan-out.

Pin source to saved64cbec90de0bd1b46c8faea4efe01b2f9b3204c5. Stable allocator hash
2f1bf278b9265855ececed61ccb5b3f2e1904f17b1855adb6d659782b241e50e.
Primary files: crates/host/src/quota_allocator.rs and quota.rs. Trace only necessary
installation/configuration/lifetime consumers in main.rs, worker_main.rs,
worker_config.rs, worker_ops.rs, limits.rs and the existing Native fatal boundary.
Relevant source tests at that pin: tests/allocator.rs, support/allocator_probe.rs,
quota_counter.rs and applicable runtime guard/publication cases. Current producer
WIP, helper protocol/nonce, full parent inventory and all platform live work are
outside this finite review. Note changed installation consumers as later recheck
dependencies; do not infer that the pinned review accepts their future revisions.

Spec route: AGENTS → specs/README registry10 → development decisions/product routes
→ D05@3, D05-MEMORY@2, D05-WORK@1 all guard/profile/supervisor/proof clauses and
D02@2 failure/publication/lifetime requirements, with full explicit dependencies.
Read RUST/DEV.RUST@2 and applicable global QA/evidence governance. Exclude unrelated
analysis serialization/export/mobile/pilot execution when not required by closure.
Current norms define the requested-layout metric and exclusions; no RSS/system-wide
memory guarantee or sandbox against arbitrary native code is claimed.

Stage1 inspect source and relevant test expectations BEFORE new author host.md,
H01-host.md or H01-allocation-proof.md claims. Requirements/design registration are
normative context, not proof. Check actual GlobalAlloc installation and pre-input
configuration, prospective checked charging, alloc/zeroed, full-new-plus-old realloc,
System/null/dealloc pointer/layout/accounting invariants, concurrency/counter safety,
ordinary/publication reserve ownership/lifetime and limits, and fatal path without
allocation/recursion/unwind/format/log/lock. Inspect narrow private forwarding seams
for unchanged shipping System behavior and test-only access bounds.
Distinguish caller UB outside valid GlobalAlloc contracts from introduced defects.
No speculative hardening or wider architecture redesign.

Return initial observations first; root then supplies author evidence/revisions and
recipes for Stage2 reconciliation. Record what source establishes, what tests merely
declare, what execution is attributed and which mandatory runtime gaps remain.
No execution to fill gaps during this review. Relevant project instruction support
must be verified at its applicable file and smallest supporting range, then cited
visibly in the comment when it materially contributes beyond generic correctness.

Final normal Markdown, never JSON/structured findings. One ::code-comment for each
actionable inline defect with absolute path/short lines, severity when helpful.
Prefer no findings over speculative issues. Verdict covers only this stable guard;
full H01, semantic-phase coverage, changed producer setup and live/latency acceptance
remain separate. Same context rechecks any assigned repair.
