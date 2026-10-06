# S01 native common-interface proof

Finite synthetic test tooling under the [native packet](../../../docs/plans/ui-blueprint/packets/S01-native-proof.md)
and [shared handoff](../../../docs/plans/ui-blueprint/packets/S01-bridges.md).
It uses Integration's actual common host; it owns no second lifecycle or schema.

## Pinned inputs and build

- F02 fixture: `9a88b12b5855bac54bf04ba7b64d233df719ddec`.
- Stage A schema/plugin API: `9d2df153abd2a7d7567100e06d4260e5edda3bb3`.
- Common support: `73d772e97efcf550ea4a4d3e8480b56509ebc548`.

```sh
python3 tests/bridges/native/prepare.py
python3 tests/bridges/native/build_support.py /absolute/task-temp/common-build
xcrun swiftc -parse-as-library -swift-version 6 -D CAPTURE_LIBRARY -target arm64-apple-macos14.0 \
  fixtures/native/Observe.swift tests/bridges/native/Collector.swift tests/bridges/native/WindowAX.swift \
  -o /absolute/task-temp/native-collector
```

`prepare.py` extracts only committed F02 inputs and compiles off/on fixture bundles;
reuse its products while inputs are unchanged. `build_support.py` extracts the
pinned common support/workspace inputs to task-temp and builds only d02_host and
the validator, locked/offline. It never consumes mutable neighboring Rust files.
The pinned preparation helper remains historical F02 input; compile the current
fixtures/native/Observe.swift separately when testing the M01 repair. Neither build
script launches UI or changes checkout/branch/index. All products
and compiler logs belong outside the repository/config/skill trees.

## Explicit own-fixture setup

Acquire the native lane through root and check current reservations. Launch only
the prepared F02 bundle with `--run-dir /absolute/evidence/fixture`. Bundle IDs are
`local.uiblueprint.f02.off` / `.on`; executable F02Fixture. Choose A or B by its
observed SwiftUI identifier, establish normal stimulus, explicitly press Snapshot.
Both window titles are F02 Synthetic; title is not identity. Use that run's actual
manifest for PID/launch time/CG window ID and surface generation, never a copied ID.

```sh
UIB_CAPTURE_LOCK_PATH=/absolute/task-temp/capture.lock python3 tests/bridges/native/prove.py \
  --host /absolute/task-temp/common-build/target/debug/examples/d02_host \
  --validator /absolute/task-temp/common-build/target/debug/uiblueprint-validate \
  --collector /absolute/task-temp/native-collector \
  --manifest /absolute/evidence/fixture/b.json \
  --output /absolute/evidence/proof
```

Output must be new. `prove.py` creates canonical Request/Session records from the
explicit fixture setup. Descriptor-only collector mode checks public identity and
permission metadata, without requested tree/pixel acquisition. The common host
then calls actual attach/begin and returns a Ticket; only its sequence is supplied
to collection. No authored clock reading/Ticket and no JSON control-message protocol.
The host's parent Instant remains authoritative; helper observations use a separate
clock domain. Collector stdin is one bounded request; AX frame is flushed before
capture. The common host receives only canonical ChannelResponse lines.

Cases: live AX+isolated capture; real AX plus injected pending capture/expiry;
real AX plus injected cancellation; real AX plus injected detach. Fault cases
send a correctly correlated replay of that case's live AX success **after** the
terminal result and require rejection. This is injected late data, not a late OS
callback or fake captured image. Retained canonical core completions are compared
structurally with submitted documents, and live documents pass the same Rust
validator as Web. Common 3 Rust/8 synthetic negatives are reused, not duplicated.

## Bounds, cleanup and limits of proof

Test parameters: 65,536-byte frames including newline, 131,072 pending encoded
bytes, one in-flight request, eight frames, 3,000ms overall request, 1s AX wait,
2s capture wait, 1s cleanup wait and 45s whole-proof watchdog. These are literal
finite test sizes, not D05 cache/heap defaults. The collector additionally caps
input and has an 8s last-resort watchdog. The normal scope is one known sample
control located within one bounded own window (160 visited nodes/depth9), not a
full-window normalized graph. Unknown properties and transforms remain explicit.

The Python wiring has capacity-one readers and bounded line reads. It terminates
and reaps only Popen children it created, then joins readers before closing streams.
The injected pending capture is killed after its 2s budget; common expiry follows
the actual parent's 3s deadline. Cancel/detach preserve completed AX. Cleanup of
the separate test-created fixture occurs after proof, never as plugin detach.
No process-wide name kill, permission prompt, display change or other-app operation.

Capture is window_isolated, audio/cursor/children off. Metadata/call intervals are
retained separately from the canonical frame; no calibrated AX-to-pixel transform
is claimed. F02's concurrent capture continuation failure/exit124 remains an M01
gate. Probe invariance, production input/privacy/isolation and four review P2
remain outside this proof. Validator validity and lifecycle completion do not
mean product acceptance or P1 compatibility freeze.

## D05 actual window sizing input

`WindowAX.swift` adds a strict window-only AX mode to the same collector. No capture
API is called in `describe-window`/`window-ax`; old D02 cases retain their selected
sample/two-channel path. The new mode enumerates AXChildren with count + bounded
array reads. It deduplicates actual AX handles with CFEqual, emits observation-local
aliases, preserves child edges and never invents nodes to fill a ceiling. Raw
AXIdentifier/AXSubrole are runtime extensions, not code declarations. Secure value
is redacted before requesting AXValue. Unsupported/unknown and known empty/false
remain distinct. Full visual/native coverage stays partial.

After a fresh explicit Snapshot with normal stimulus and no popup:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 tests/bridges/native/sizing.py \
  --host /absolute/task-temp/common-build/target/debug/examples/d02_host \
  --validator /absolute/task-temp/common-build/target/debug/uiblueprint-validate \
  --collector /absolute/task-temp/native-collector \
  --manifest /absolute/evidence/fixture/b.json \
  --output /absolute/evidence/sample
```

The driver reuses the bounded reader/reap helpers from prove.py and the actual
common Ticket/lifecycle. Literal sizing limits are fixed before acquisition:
160 nodes/depth9, 512KiB frame/output, 1MiB pending, one request, four frames,
1,000ms parent deadline and 15s whole-driver watchdog. They are not production
memory defaults. The original D02 limits are unchanged. This larger wire allowance
is a separately declared eight-field whole-window sizing case, not a retry of a
failed smaller request. Collector AX admission remains at most900ms.

Selected core fields: role, description, value, placeholder, enabled, focused,
actions, accessibility_bounds; raw identifier/subrole metadata supports binding
and redaction. Only a small explicit raw-role mapping is applied; other roles stay
unknown while native_role is preserved. AX bounds are never promoted to layout/hit.
Acquisition metrics state actual visited/returned/discovered counts, depth, duplicate
references and known/unknown omission counts. A response smaller than160 is valid
partial evidence, not proof of the configured maximum.

`returned-wire.ndjson` preserves exact collector bytes/member order, including its
newline; use it unchanged for the memory measurer. The common retained document is
also saved and structurally compared, but its serialization may differ. No fixture,
shared schema or sizing policy is changed. No new pixels are needed or captured.

For a focused nonempty secure-field check, type a synthetic canary into the owned
fixture, press Snapshot, and pass `--check-canary-env NAME` to sizing.py. The driver
removes that one-use environment value before child launches, checks wire bytes
before persistence and host submission, checks retained data and bounded diagnostic
output in memory, and records only absence/status/byte counts. Never store the raw
canary or diagnostic text. It also preserves known empty/false versus unavailable.
Actions come from AXUIElementCopyActionNames, not role-derived setters/modalities.
This is scoped fixture evidence, not universal secret recognition.

## Capture lifecycle prerequisite

Collector.swift now consumes the same CaptureLifecycle implementation from
fixtures/native/Observe.swift, compiled with CAPTURE_LIBRARY to omit the fixture
helper main. Canonical schema/common Rust lifecycle remains unchanged. Real capture
requires a shared run-owned capture lease; the helper emits AX before waiting.
SDK authorization error-3801 maps to permission_required with explicit_permission
recovery, never an automatic retry or another backend.

The focused capture_lifecycle.py driver reuses the pinned common host/reader/reap
helpers. It performs live AX with labelled injected capture stall/failure,
actual common cancel/detach and late-frame replay rejection. It separately verifies
unrelated B AX while A holds a stalled capture lease, plus lease availability after
reap. This driver issues no real pixel requests; real pre/post-repair capture
attempts and the remaining B permission condition are recorded in the receipt.
The known SDK simultaneous-callback failure is addressed by explicit bounded
callbacks and capture-only serialization, not relabelled as parallel-capture success.
