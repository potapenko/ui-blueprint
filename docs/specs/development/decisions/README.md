# C01 development decisions

- Node type: branch; contract: `UIB.DECISIONS@1`; clause: `UIB.DECISIONS.ROUTE`.
- Authority: Active / Stability: Evolving; accepted/released implementation: none.
- Authority source: PLAN.UIB@1 user launch, ROADMAP D01–D07 delegated engineering
  choices, [C01 packet](../../../plans/ui-blueprint/packets/C01.md) and direct
  [request-driven clarification](../../../plans/ui-blueprint/receipts/product-context.md).
- Read when: T01/S01 or a named decision's dependent package.
- Do not read when: an unaffected domain already has a current selected basis.
- Requires: select the decision and explicit dependencies below, not every sibling.

## Meaning and precedence

These are implementation choices under existing Active product requirements,
not claims that prototypes satisfy those requirements. The direct user instruction
governs: explicit request → bounded collection/calculation → response; stored
snapshots support local analysis. No periodic collection between requests.
Optional events can invalidate state but cannot trigger recollection themselves.
Act/verify is a finite authorized scenario; subscribe is not required by the
primary path. No broad rewrite of the original specification is made.

`C01-DEC-001`: resolve previously open engineering choices into version-1 decision
leaves; [DEV.RUST@2](../rust.md) replaces documentation-only setup policy with
the selected pin. Existing product CONTENT@1 norms and release baseline unchanged.
Compatibility: initial, unreleased candidate. Changing a chosen contract advances
its revision; failed evidence cannot silently relax gates. Root accepts the
documentation candidate separately from the author marking work complete.

## Select a decision

| ID / contract / clause | State and deadline | Owner / read when |
| --- | --- | --- |
| [D01](d01-support.md) / `UIB.D01@1` / `UIB.D01.CONTENT` | Initial matrix chosen; runtime qualification open before P2 claims | T01 toolchain; W01/M01 capabilities |
| [D02](d02-boundaries.md) / `UIB.D02@2` / `UIB.D02.CONTENT` | Reviewed reusable worker/publication/lifecycle boundary registered; implementation proof open | Host/S01 protocol; W01/M01 adapters |
| [D03](d03-data.md) / `UIB.D03@2` / `UIB.D03.CONTENT` | Core0.1 protected; local analysis0.2 contract registered before implementation | S01/G01/L01 analysis, K01 compatibility |
| [D04](d04-identity.md) / `UIB.D04@1` / `UIB.D04.CONTENT` | Freshness policy chosen; adversarial proof before P4/P5 | W01/M01, K02, A01 |
| [D05](d05-limits.md) / `UIB.D05@3` / `UIB.D05.CONTENT` | [D05-MEMORY@2](d05-memory.md) supervised partition and [D05-WORK@1](d05-working-memory.md) actual enforcement registered; proof/calibration open | Host/K01; S01 before live use; W01/M01/P01 |
| [D06](d06-performance.md) / `UIB.D06@1` / `UIB.D06.CONTENT` | **P0 numeric gates frozen here**, candidate not evaluated | Q02; W01/M01 instrumentation |
| [D07](d07-reuse.md) / `UIB.D07@5` / `UIB.D07.CONTENT` | H01-PROCESS-001 adds exact libc host bindings; guarded Web codec/log and runtime float_roundtrip preserved | H01 host, S01/analysis, platform owner, I01 |
| [T01/S01 handoff](handoff.md) / `UIB.C01-HANDOFF@1` / `UIB.C01-HANDOFF.CONTENT` | Concrete next work and proof obligations | Assigned Integration workers |
| [Evidence](evidence.md) / `UIB.C01-EVIDENCE@1` / `UIB.C01-EVIDENCE.CONTENT` | Supporting facts/limits; never new product intent | Trace a decision to an accepted input |

## Traversal and scope receipt

AGENTS → [spec root](../../README.md) → [product branch](../../product/README.md)
C01/T01 and [acceptance branch](../../acceptance/README.md). Full CONTENT@1 closure:
BOUNDARIES, ROADMAP, RUST-BOUNDARIES, MODEL, EXCHANGE, IDENTITY, GEOMETRY,
PROJECTIONS, FORMS, ACTIONS, LIFECYCLE, CACHE, PRIVACY, NATIVE, CLI, PILOTS,
WEB-PILOTS, NATIVE-PILOTS, PERFORMANCE, GOLDEN, REUSE. Previously read full R03
leaves reused after no-diff check against C00; missing leaves read completely.
Branches: ROUTING@1, PRODUCT-ROUTES@1, ACCEPTANCE-ROUTES@1. DEV.RUST@1 and RUST.md
read before this change; DEV.RUST@2 is this packet's explicit setup-policy delta.
Twenty-one leaves resolve the selected product closure. C00 `92d2bf8`, independent
acceptance `9bedecc`; no product CONTENT revision drift or unresolved intent fork.

Mode: Reconcile existing request-driven intent and resolve authorized technical
choices. Protected: Rust analytics, model-free local operation, Mac/Web parity,
scoped collection, privacy, identity/generations, explicit unknowns and positive
acceptance gates. Excluded: mobile implementations/ML/F1–F4, detailed export,
real application changes and runtime operation. Allowed writes: this decision
subtree, spec-root route metadata, DEV.RUST and C01 receipt only.

Supporting resources: R01/R02/R03 ledgers, F01/F02 handoffs and retained samples,
product-context consultation, official Rust manifest/Cargo docs and crate metadata.
Real cases RC01–RC05 inform generic Surface/anchor/text/resize semantics only;
their collection is not UI Blueprint adapter proof and is not hardcoded in engine.
