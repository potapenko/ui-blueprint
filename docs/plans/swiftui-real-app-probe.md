# SwiftUI real-app geometry package

Status: implementation in progress. Approved by the user on 2026-11-14 in
UI Blueprint chat `01a1dab2-2e7d-7f13-9400-8e42b0859137`; subsequent correction
selects joint work with `[pl-gpui] [mac] Подготовить онбординг Rust-проекта`
and excludes the new Rust/GPUI application entirely.

## Outcome and ownership

Obtain internal layout geometry of the real PlayPhrase.me SwiftUI recovery
component through a reusable Debug SDK, then validate and measure it with the
existing Blueprint Rust engine. F02/Canvas/standalone mockups are insufficient.
Blueprint source baseline: master `2f54c0d3ae08b5c0d048cf85b4896f9c51a8f4fd`.
Native source baseline: beta-01 `7de6680a19b3d15cf00d657f551826b95451cbdc`.

Root is the only writer in UI Blueprint. Native chat
`01a1daaf-dafe-7e63-b309-3afd2ddec599` is the only writer in playphraseme-mac
and owns its builds, exact app instance and desktop/runtime reservation.
No GPUI writes and no nested agents. Preserve unrelated after-title-spacing.png.

## Spec Basis and authorized writes

Registry67 → product → NATIVE5/EXCHANGE2/GEOMETRY2/MODEL/IDENTITY/PROJECTIONS/
LIFECYCLE/PRIVACY and native-pilots/pilots/forms/actions/cache closure; existing
CLI21/ANALYSIS2/types/validation and Compose1 examined for reuse. Current RUST.md,
DEV.RUST3 and SwiftUI Expert apply. New contract: SWIFTUI-PROBE1, Evolve under
the direct approval; old data/commands and product design are protected.

Blueprint write set: `plugins/swiftui/` concrete Swift SDK and focused tests;
`tests/bridges/` only if needed for cross-language validation; selected spec
registry/product routes; SDK and skill recipe documentation; this plan.
SDK emits canonical Snapshot directly; no Rust source/API change is required.
Native write set/approval is recorded by its owner in
`docs/plans/ui-blueprint-swiftui-pilot.md` in that repository.

## Acceptance and handoff

1. Compile/test the bounded SDK, validate its output with the current Rust schema
   and run independent numeric queries. Record the SDK source checkpoint.
2. Native owner integrates exact API into real SearchLearnRecoveryView and the
   existing local Debug AppScenario, with no production/visible-design change.
3. Native owner establishes capture, resize/content changes, on/off invariance,
   exact app/window attribution and stationary screenshot mapping.
4. Root checks the untouched observations through Rust; no manually supplied
   geometry or acceptance from AX-only data. Verify failure/limit cases.
5. Review coherent changes, show retained actual screenshots, clean only owned
   non-image outputs/processes and checkpoint both current branches.

SDK handoff: package compiles in Debug and Release;8 focused Swift tests pass.
Actual SDK-produced synthetic records validate against core0.1, and the unchanged
Rust engine returns independent8/18pt gaps. Release object has no BlueprintProbe
symbols. These checks establish SDK/codec behavior, not real-app acceptance.
Native owner's independent SDK review found retained-child disappearance could
reuse a mount ID. Root repaired both marker lifetimes through ProbeMount; the
focused retained-state renewal/rejection test passes. API/codec unchanged.
Current dependency: native integration and actual-app snapshots/on-off/pairing.
Packaging/global installation and GPUI remain outside this package.
