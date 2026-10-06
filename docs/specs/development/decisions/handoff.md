# T01/S01 implementation handoff

- Domain: `uib.development.handoff`; accepted/released baseline: none.
- Authority source: [C01-DEC-001](README.md#meaning-and-precedence).
- Node type: leaf; contract: `UIB.C01-HANDOFF@1`; clause: `UIB.C01-HANDOFF.CONTENT`.
- Authority: Active / Stability: Evolving; package dispatch/write leases remain root-owned.
- Read when: preparing or executing T01/S01.
- Do not read when: implementing unrelated or future-stage features.
- Requires: [D01](d01-support.md), [D02](d02-boundaries.md), [D03](d03-data.md),
  [D04](d04-identity.md), [D05](d05-limits.md), [D06](d06-performance.md),
  [D07](d07-reuse.md), [DEV.RUST@2](../rust.md).
- Owner/deadline: assigned T01/S01 Integration workers; dependency proof before freeze/live use.

## T01: concrete minimal build boundary

Pin Rust1.96.0 + rustfmt/clippy, edition2024/resolver3/rust-version1.96 and host
aarch64-apple-darwin. Root manifest centralizes approved serialization versions;
generate Cargo.lock once, commit, then use `--locked`. T01 must verify actual
resolved packages/features/MSRV/notice files before reporting setup complete.

Create only `crates/schema` (`uiblueprint-schema`) initially, with concrete
schema-version identity and compatible/incompatible-version validation for the
`0.1.0` candidate, parser round-trip and explicit rejection example. This is a
small real S01-owned responsibility, not an empty placeholder graph implementation.
No engine, CLI, web/native stub crates merely to mirror a directory proposal.
No browser, native helper, daemon or optional model launched by T01 checks.

After those targets exist, scoped commands (not execution evidence in this doc):

```sh
cargo +1.96.0 check --locked -p uiblueprint-schema --all-targets
cargo +1.96.0 fmt --all -- --check
cargo +1.96.0 clippy --locked -p uiblueprint-schema --all-targets -- -D warnings
cargo +1.96.0 test --locked -p uiblueprint-schema
```

No workspace-wide expansion unless a shared change creates affected consumers.
The initial single owner makes workspace format scoped in practice; later use
affected package checks. Checkpoint only packet-owned files on current branch.

## S01: one schema, validator and interface owner

Expand the same schema crate, never fork T01 types. Add `crates/plugin-api`
(`uiblueprint-plugin-api`) only with concrete attach/capability/version/session
and request/response contracts plus lifecycle tests. Pure operation traits use
schema types; no OS/browser handles or SDK imports. Add a diagnostic binary
`uiblueprint-validate` under schema for GOLDEN01, not a second application CLI.
S01 produces machine-readable JSON Schema and semantic validator, typed errors,
bounded parsing and redaction-safe error rendering; valid/invalid fixtures per D03.

```sh
cargo +1.96.0 check --locked -p uiblueprint-schema -p uiblueprint-plugin-api --all-targets
cargo +1.96.0 clippy --locked -p uiblueprint-schema -p uiblueprint-plugin-api --all-targets -- -D warnings
cargo +1.96.0 test --locked -p uiblueprint-schema -p uiblueprint-plugin-api
cargo +1.96.0 run --locked -p uiblueprint-schema --bin uiblueprint-validate -- <golden-path>
```

Packet names exact fixture paths before execution. Validate known false/empty,
unavailable/no-value, separate selection, sourced measurements, namespaced raw
roles/IDs, generations, observations/clocks/coverage, mixed units/transforms,
relations/focus and whole-node update compatibility. GOLDEN01 action records stay
synthetic until A01/W02/M02; S01 cannot claim real delivery. Positive/negative
validator outputs and exit statuses must match independently specified examples.

## Open proof ledger and ordering

| Obligation | Owner / deadline | Exact remaining evidence |
| --- | --- | --- |
| D02-PROOF | S01 before P1 interface freeze | Same bounded wire round-trip with F01 and current F02 fixtures; channel-completion/timeout/detach/version/size negatives. Root grants narrow bridges/runtime lane if outside S01 packet |
| D05-RES | S01 before W01/M01 | Normalized memory/frame sizing, retained revisions/caps/admission, oversize and eviction/resync checks; numbers recorded before live use |
| Web acquisition | W01 before scoped observe acceptance | Selected DOM/CSSOM+addressed AX bounded by actual work; no whole DOMSnapshot then filtered success |
| Native channel isolation | M01 before isolation/capture acceptance | Reproduce/repair simultaneous continuation leak/124; preserve completed AX on capture hang; unrelated AX progress and bounded owned cleanup |
| Geometry mapping/current probe | G01/M01/P01 before affected M03–M05 | Frame metadata/transform calibration, included popup pixels, current one-shot off/on matched active/key/focus/hit/AX/geometry |
| Freshness/action proof | K02/A01/W02/M02 before P4/P5 | Lost events, stale generations, unknown effects, no duplicate submit, real input/result attribution |
| Integrated quality/performance | Q01/Q02/Q03 on fixed candidate | Mac/Web positives, frozen per-request D06, real-case agent evaluation kept separate from answer keys |

T01 and candidate S01 work are dependency-ready after root accepts C01; none of
the open later proofs requires inventing product behavior or waiting for mobile
captures. Interface freeze/live claims wait for their exact gates. `crates/engine`
appears at G01/K01 for real pure logic; `crates/cli` at L01; adapters at W01/M01.
Shared API changes remain S01 integration work, not parallel owner improvisation.
No production schema compatibility freeze until both Mac/Web P2/P5 and P3 pass.
