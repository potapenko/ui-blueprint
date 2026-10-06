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
xcrun swiftc -parse-as-library -swift-version 6 -target arm64-apple-macos14.0 \
  tests/bridges/native/Collector.swift -o /absolute/task-temp/native-collector
```

`prepare.py` extracts only committed F02 inputs and compiles off/on fixture bundles;
reuse its products while inputs are unchanged. `build_support.py` extracts the
pinned common support/workspace inputs to task-temp and builds only d02_host and
the validator, locked/offline. It never consumes mutable neighboring Rust files.
Neither build script launches UI or changes checkout/branch/index. All products
and compiler logs belong outside the repository/config/skill trees.

## Explicit own-fixture setup

Acquire the native lane through root and check current reservations. Launch only
the prepared F02 bundle with `--run-dir /absolute/evidence/fixture`. Bundle IDs are
`local.uiblueprint.f02.off` / `.on`; executable F02Fixture. Choose A or B by its
observed SwiftUI identifier, establish normal stimulus, explicitly press Snapshot.
Both window titles are F02 Synthetic; title is not identity. Use that run's actual
manifest for PID/launch time/CG window ID and surface generation, never a copied ID.

```sh
python3 tests/bridges/native/prove.py \
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
