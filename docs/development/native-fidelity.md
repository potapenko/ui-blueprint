# N04 — Native F02 fidelity and Q02 input handoff

Consumer: Q02's Native D06 comparability preflight; Q01 checks the affected Native
collector on its eventual functional candidate. [N04 packet](../plans/ui-blueprint/packets/N04-native-fidelity.md)
authorizes source repair, offline proof, and preparation only. No app launch,
foreground/input/capture/TCC or retained Q01-session operation is authorized here.
The [receipt](../plans/ui-blueprint/receipts/N04-native-fidelity.md) records actual checks.

## Canonical source mapping

[Native acquisition@2.PROOF](../specs/development/decisions/d05-native-acquisition.md)
and [D06@1](../specs/development/decisions/d06-performance.md) require original known
facts, limits and pixels. MODEL@1/D03@4 already provide namespaced properties; no
new core field, schema version, dependency or shared owner is needed.

| Original F02 attribute | Existing canonical owner / preserved meaning |
| --- | --- |
| AXRole | Node.native_role; normalized role independently retains unknown when unmapped |
| AXIdentifier / AXSubrole | macos.ax extensions with the exact native names |
| AXTitle | macos.ax / AXTitle extension, property.field=accessibility_name, reported AX evidence |
| AXDescription | description and existing accessibility_name properties; never replaced by Title |
| AXValue | value; known empty string/numeric zero retained, secure value unacquired/redacted |
| AXEnabled / AXFocused | enabled / focused; actual booleans, not focusability/global input-owner inference |
| AXPosition + AXSize | accessibility_bounds rect in ax-screen/screen/pt/top_left, unknown image transform |
| Reported actions / child references | actions text_list / source-node children |

Title is acquired only when accessibility_name is selected. It does not supply a
canonical Name, substitute for Description, become a locator, or establish window
identity. Fixed identity batch remains three; the original value batch remains at
most seven. A separate one-attribute Title batch follows its admission, so a local
Title refusal cannot erase the original fields. This adds one public AX batch per
node when accessibility_name is requested; its live latency cost remains unmeasured.
The same per-value/batch/aggregate ceilings apply to both batches, without retries,
splitting already-returned batches or copying/truncating rejected Title.
String/batch/aggregate/construction admission
and unavailable outcomes remain in the same owners. Failure or wrong type has no
value; known empty is preserved. No blanket attribute discovery/dump was introduced.
AXF02Unsupported was a deliberately invalid diagnostic attribute in the historical
recorder, not a known product fact or a required new collection field.

## Reproduce recorded proof (no UI)

Use a new system-temp directory. Wait for the active CPU owner to release before
compiling. All commands below run from repository root; no branch/worktree is made.

```sh
NATIVE_TASK_TMP=$(mktemp -d "${TMPDIR%/}/uib-n04.XXXXXX")
xcrun swiftc -parse-as-library -swift-version 6 -D CAPTURE_LIBRARY -D HOST_HELPER \
  -target arm64-apple-macos14.0 -module-cache-path "$NATIVE_TASK_TMP/module-cache" \
  plugins/macos/NativeAcquisition.swift plugins/macos/NativeJSON.swift \
  plugins/macos/NativeArtifacts.swift fixtures/native/Observe.swift \
  tests/bridges/native/Collector.swift tests/bridges/native/WindowAX.swift \
  plugins/macos/HostProtocol.swift \
  tests/bridges/native/acquisition/FidelityChecks.swift -o "$NATIVE_TASK_TMP/fidelity-checks"
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_TARGET_DIR="$NATIVE_TASK_TMP/target" \
  cargo +1.96.0 build --locked --offline -p uiblueprint-schema --bin uiblueprint-validate
PYTHONDONTWRITEBYTECODE=1 python3 tests/bridges/native/acquisition/fidelity_check.py \
  --checks "$NATIVE_TASK_TMP/fidelity-checks" \
  --validator "$NATIVE_TASK_TMP/target/debug/uiblueprint-validate" \
  --baseline "$F02_RECORDS/final-off-warm/run-0/external-7.json" \
  --output "$NATIVE_TASK_TMP"
```

`F02_RECORDS` is the existing root in the [F02 receipt](../plans/ui-blueprint/receipts/F02.md),
ending `F02/51b19df6-76de-4034-9949-933134c868d0`. The runner rejects a changed baseline
(SHA256 `42c2b55194136aec4560192ed5945bb3c2bf844f0417503c116219f52c7a651d`).
It substitutes only NativeAXAccess using original CF values/handles/child ordering.
Production traversal, admissions, construction and codec run unchanged; the current
Rust validator checks outputs. Original depth-first order is reconciled to actual
breadth-first output using the recorded tree, never title/rect matching.
Generated snapshots are explicitly **boundary replay**, despite production-shaped
live metadata: no new AX observation, SDK lifetime, runtime or latency evidence.
Seven cases cover original records, empty/unsupported/unknown/oversized/mistyped
title and an unselected name. All non-Title node data must match the original in
each of the five Title-only variants, including unavailable/redacted states, geometry,
actions and edges. Oversized Title must copy zero bytes, proven by equal copy counters
to empty Title while all sibling data remains equal. A missing-title counterexample
must be rejected. The original17d3475 runner lacked sibling-isolation assertions;
Q01 rejected that source after reproducing loss of otherwise known fields. This
repair extends the oracle rather than treating canonical validity as preservation.

## Reproducible distinct fixture input

The original per-call timing source was the pre-Snapshot reactive fixture:
SHA256 `d33eac888c17045d663aefaf8e6ad92742cdac9dba00b5e86296da474c0979d4` in
run-summary.json, build-final2. It is historical, not the current request model.
F02's corrected explicit-request source at `9a88b12b5855bac54bf04ba7b64d233df719ddec`
hashes `3b7f764d131719af59b04fc22b5583e63a0dc248d7e735c9ca96376a332c3c91`.

Use `53e6e6ef0d8a291c92c340fe5ed217ccbfd965cb` as the **distinct request-only
benchmark input candidate**. Its sole Fixture.swift delta from 9a88b12 adds bounded
identity-only open/close invalidation and identity_path, needed by current helpers.
No visible controls/layout were changed by that commit. Its exact source hash is
`662c92db9fbf0b131e03c022053fc68ef8ab4126d39989da48b49da39f8c238d`.
It predates current Result/Protected-input HStack and scroll-marker additions;
no controls are removed from the current fixture or defaults modified.
The source already contains Snapshot: **not byte-identical to the timing fixture**.
D06 expressly uses the current explicit-request model. Its additional button and
probe mechanism require live node/geometry reconciliation, not an assumed75 count.

```sh
PYTHONDONTWRITEBYTECODE=1 python3 tests/bridges/native/prepare_f02_fidelity.py \
  "$NATIVE_TASK_TMP/f02-input"
```

This reads only the pinned Git blob, verifies its hash, compiles off/on with Swift6,
macOS14 arm64 target, writes matching existing F02 bundle IDs and hashes both products.
It performs no launch, signing/TCC change or AX call. Q01/Q02 own later consumption
of the generated handoff.json/source/bundles in temp; remove only their nonimages
after consumption. Any subsequently created image and containing directory remains.
Do not run simultaneously with Q01's retained same-bundle application.

## Later authorized configuration and acceptance

After Q01 foreground/input resolution, functional acceptance and resource release,
Q02 may establish this distinct input in its own new temp run directory. Setup state
must be verified against both `final-off-warm/frozen-manifest.json` and
`final-on-warm/frozen-manifest.json`: A, expanded=true, wide=false, activations=1,
name="", secret_present=false, checked=false, applied="none", popup=false,
scroll_end=false, stimulus=normal; 40 rows and550×525pt. Compare sets expanded/Count1
but clears focus: the historical manifest's focus=name needs separate establishment
and verification. Snapshot publishes current bindings; stale original PID/window/
generation/identity files are never used as candidate authority.
Historical warm manifests were inactive/nonkey/nonmain in both modes. Do not treat
that as permission to bypass Q01's pending foreground prerequisite; matched input
context, including actual AX focus/geometry, needs explicit later reconciliation.

Use Q02's existing [Native driver](../../tests/bridges/native/performance.cjs), selecting
window-ax; its exact nine fields include accessibility_name AND description.
Keep profile.json unchanged, cap160/depth9/output524288/overall3000ms, current exact
binding+identity_path, owned_synthetic_fixture pixels, natural1100×1050 output.
Current helper and host must be compiled from the accepted candidate pin, separately
from this historical fixture source. No historical observer binary is substituted.
Driver environment: UIB_Q02_ALLOW=1, UIB_Q02_FUNCTIONAL_PIN, UIB_Q02_MANIFEST,
UIB_Q02_NATIVE_BASELINE, UIB_Q02_EXECUTABLE(+_SHA256), UIB_Q02_HELPER(+_SHA256),
and a new system-temp UIB_Q02_OUTPUT. These are gated future inputs, not N04 grants.

The existing driver deliberately returns requires_field_value_reconciliation even
when aggregate checks match. Compare every original known fact, unavailable state,
actions/edges and explained node change before timing. The preserved recorded
mapping above is a source proof, not that live comparison. If Snapshot's added node
or another real difference makes the frozen workload non-equivalent, return the
exact mismatch to D06's owner; do not delete it, lower quality, ignore unknowns or
silently change the thresholds. Actual SDK preservation, off/on equivalence,
1100×1050 capture, cleanup and20cold/100warm cohorts remain Q01/Q02 work.
