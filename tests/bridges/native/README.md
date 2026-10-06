# S01 native common-interface proof

Owner: S01-Native. This checkpoint saves independent fixture/build preparation.
Stage A `9d2df153abd2a7d7567100e06d4260e5edda3bb3` and the own-fixture runtime
grant have since arrived; common test support remains Integration-owned.
[Bounded packet](../../../docs/plans/ui-blueprint/packets/S01-native-proof.md) and
[shared boundary](../../../docs/plans/ui-blueprint/packets/S01-bridges.md) govern.

```sh
python3 tests/bridges/native/prepare.py
```

This extracts only three source files from committed F02
`9a88b12b5855bac54bf04ba7b64d233df719ddec` into a unique task-temp, compiles both
fixture variants and the existing diagnostic helper, then records source/binary
hashes and the compiler/SDK/OS. It does not launch anything, read Rust source or
bind to a schema. No worktree, checkout, branch or index mutation occurs. An
optional `--output` must name a new absolute directory under the system task-temp
root, outside the repo; it cannot redirect build logs into a configuration tree.

Existing F02 is immutable input. Its current setup is one-shot: launch only with
a granted native lane, select exact own window A/B, establish state, then explicitly
press Snapshot. Bundle IDs are `local.uiblueprint.f02.off` and
`local.uiblueprint.f02.on`; executable is F02Fixture. SwiftUI scene identifiers
are `a`/`b`, both titles are F02 Synthetic. PID, launch time, CG window ID and
surface generation must come from that run's receipt, never this document.

The F02 observer's combined writer is **not** the completed D02 bridge: it waits
for capture before persisting AX, has a historical 45-second diagnostic watchdog,
and lacks the common framing/lifecycle boundary. Do not use it to claim preserved
AX on capture timeout or silently replace the selected D05 test budgets.

The concrete Stage A handoff, proposed integration order and open shared-owner
requirements are in [interface-proof-native.md](../../../docs/development/interface-proof-native.md).
There is deliberately no second wire schema, lifecycle protocol or fake validator
in this preparation directory. No live bridge proof has run yet.
