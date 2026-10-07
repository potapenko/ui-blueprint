# Restore owned-process termination under parent signal policy

Class: shipping_product; approved PLAN.UIB@1 and explicit in-scope repair authority
in execution.md. Mode Restore D02.LIFECYCLE, no product/spec/number change.
Inherit settings; current master; no nested agents/worktrees. One finite outcome:
close H01-PROCESS-R1 from [independent review](../receipts/H01-process-review.md).
Immediate consumer: safe H01 process supervision before real worker integration.

Read current D02@2/D05@3/MEMORY@2/WORK@1/D07@5 with already selected complete
H01 closure, RUST/DEV.RUST and the review. Root has read these contracts. Existing
Native/API/source7b942ab and actual SDK sigaction behavior are evidence; no repeated
general audit or new process framework/dependency. Core confirmed the missing
parent-signal guarantee, so there is no user choice to ask about.

## Required remedy and protected behavior

Reject incompatible automatic-reaping state before spawning a managed child;
inspect actual public Darwin signal semantics, including any applicable ignored
SIGCHLD case. Define/check the narrow supported parent disposition/reaping lifetime
invariant so the waitpid→kill path cannot silently act without its PID-reservation
precondition. On detected lost ownership/configuration, fail closed rather than
signal a potentially reused PID or declare cleanup complete. Document what Core
establishes and what an embedding caller must keep stable; do not pretend to make
arbitrary same-process native code a security sandbox or claim Darwin pidfd support.

Use a minimal enforceable guard/lifetime contract, not a new global signal framework.
Do not silently reset/replace parent/operator signal handlers or global settings.
Keep checked process ownership, existing stack limits/headroom/actual-size checks,
FD mapping/NOSIGNAL/fatal behavior and all unchanged tests/proof protected.
Public core0.1/analysis0.2/CLI semantics and actual input permits are untouched.

## Ownership and exact writes

Native owns only its existing process.rs/process/**, tests/process.rs,
tests/support/process_peer.rs, docs/development/host-process.md and
docs/plans/ui-blueprint/receipts/H01-process.md. It implements the OS boundary
validation/lost-state handling and focused disposable-peer regressions.

Core owns the actual HostDomain/RuntimeHost lifetime integration in its existing
H01 domain/authority/host-types/supervisor/worker files, docs/development/host.md
and receipts/H01-host.md. It must establish/document supported reaping ownership,
propagate failure/quarantine and retain grants until actual cleanup. Core may not
edit Native source or claim that a documentation-only assertion fixes the boundary.
If another shared API/file is necessary, return exact signatures/paths before edits;
process_api/lib/Cargo remain Core-owned, with a root-approved short handoff.
Each owner returns its actual behavior and interface assumption promptly so they
do not wait on each other's unimplemented guarantee. No broad adjacent refactor.

## Coordinated API amendment

Core's concrete consumer handoff requests
`ProcessPlatform::validate_parent_reaping() -> Result<(), HostError>` in
`crates/host/src/process_api.rs`. Root authorizes this narrow required trait method
and its contract documentation: Core owns declaration and actual HostDomain/
RuntimeHost use; Native implements it by forwarding to its existing OS predicate.
Core publishes the declaration first, then Native supplies the real implementation;
no default success or duplicated OS policy implementation. Native's current checks
remain necessary at the OS boundary; this method supplies the actual host's
precondition and subsequent validation without changing global handlers.

Basis: D02.LIFECYCLE requires ownership and truthful quarantine; the independent
R1 finding and Core's absent host guarantee establish an implementation defect.
This exact signature is Core's engineering proposal selected to connect existing
owners, not new product intent or a claim of atomic protection against arbitrary
same-process native code. Preserve the continuously stable supported SIGCHLD and
exclusive-reaper caller invariant. Core's lease/poison/quarantine/grant retention
and already ACKed results must use the real predicate before acceptance.

The selected recovery route is specs/README registry10 → product/README and
development/decisions/README → D02@2, D05@3, MEMORY@2, WORK@1, with their full
explicit dependencies (D01/D03/D07, product CONTENT@1 and performance/pilots/golden),
RUST/DEV.RUST@2 and the existing H01 packets. No revision drift found. Analysis
serialization, export, mobile and live UI remain protected and outside this repair.
Both owners retain existing write sets; only Core's process_api.rs addition opens.
Focused affected checks follow the real declaration/provider handoff; checkpoint
and push are serialized through root. Full H01 and independent R1 acceptance remain open.

## Focused proof and checkpoint

Use disposable owned peers to set the incompatible SIGCHLD/SA_NOCLDWAIT state and
prove refusal before unsafe lifecycle, without touching the operator/test-runner
signal policy or chasing PID reuse through mass spawning. Cover relevant policy
drift/lost ownership and existing repeated-reap behavior. Keep exact resource
cleanup and safe error metadata. No UI/apps/hostile JSON or new runtime survey.

Re-run only affected checks on stable shared inputs; keep the previous FD/stack/fatal
evidence when unchanged. Record actual tested configuration, source/input hashes,
what remains a caller invariant and its real Core consumer. Saved Core7b942ab is the
baseline; don't mislabel independent source review as executed tests.

Each coherent repair is exact-path committed+pushed after root Git grant; return
SHA/checks/residual/release. Same host_process_review observes saved repairs first,
then reconciles new receipts. No reviewer replacement or new user approval. Do not
mark this defect or full H01 accepted until the relevant independent gates close.
