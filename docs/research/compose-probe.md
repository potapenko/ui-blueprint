# Compose probe source decision — KC-COMPOSE-001

Consumer: reusable source-included Desktop/macOS and Android collector, COMPOSE@1.
Read2026-10-23; no upstream code copied, no new Rust/runtime dependency.

- [popovanton0/Blueprint](https://github.com/popovanton0/Blueprint/tree/09287ace7ea4a79cb5a1a08521cd69a365f7bdac), exact09287ace: read Blueprint.kt, BlueprintId.kt and DSL Anchor/Dimension/GroupScope/MeasureUnit completely. Apache2 license and NOTICE (`Copyright 2023, Anton Popov`) read. Marker node retains LayoutCoordinates; root.localBoundingBoxOf maps registered nodes. Adopt explicit marker/public coordinate mechanism only. Reject mutable global enable switch, overwrite-on-duplicate identity, overlay, drawing arithmetic, density rounding and padding: these alter the measured UI or hide ambiguity. Own implementation tracks mount identity and bounded one-shot reads.
- [Blueprint Compose Preview](https://github.com/GusWard/Blueprint-Compose-Preview/tree/95398528a6d5b1f4fa4ce92e1a73f830608eb3fb): inspected BlueprintPreview collection/refresh/survivor portions and BlueprintLineCalculator/BlueprintItemData. Historical PROBE-SOURCES records Apache2. Reject this mechanism entirely: reflection, repeated pre-draw collection, semantic label heuristics and old nonempty survivor data conflict with explicit scoped fresh collection. No code adopted; no claim of complete source/license audit or running upstream tests.
- [Android UI1.9.4 source artifact](https://dl.google.com/dl/android/maven2/androidx/compose/ui/ui/1.9.4/ui-1.9.4-sources.jar), SHA256 `13a0746684d583d02ff4f1e48c043a26da2235dbb5ab5075015cbd84ed5f1acb`.
- [JetBrains UI1.12.0 source artifact](https://repo1.maven.org/maven2/org/jetbrains/compose/ui/ui/1.12.0/ui-1.12.0-sources.jar), SHA256 `b645ae5424b8dc6ded18015bae1fc96e329e0008db188a35f45bf8d5630c216b`.

For both pinned artifacts, read commonMain/androidx/compose/ui/layout/LayoutCoordinates.kt and node/GlobalPositionAwareModifierNode.kt fully, including Apache2 headers. Public localBoundingBoxOf with false is unclipped, transformed axis-aligned bounds; default boundsInRoot clips, so it is unsuitable for raw layout. Density belongs to the root coordinate space; no rounding/fontScale multiplication. Callback only receives finalized coordinates; export queries current attached coordinates on UI thread. Current public documentation is not used to assume new1.10 APIs exist in Android1.9.4. No private coordinator dependency or copied implementation.

Own checks: lifecycle/mount/duplicate/limits harness; Kotlin compile with pinned UI APIs; generated probe JSON→strict Rust canonical normalization→existing measure/check/diff. Actual consumer layouts, probe invariance and Android device/emulator differences remain app-owner evidence. No upstream test execution claimed.

Consumer Android UI1.9.4 repro: Scaffold marked root plus padded/scrollable Column
initially refused outside_root because parentLayoutCoordinates skips modifier
coordinators. The public parentCoordinates chain includes those coordinators;
use exact identity on that chain with128-hop cap. Focused tests keep foreign-root
and over-depth refusal. Modifier.Node.onReset changes mount identity on reuse.
Registration overflow records incomplete admission and refuses capture, without
throwing into layout callbacks or collecting additional nodes.
