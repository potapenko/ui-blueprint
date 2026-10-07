# Reject stale fixture window bindings after close/reopen

Class shipping_product; retained Native owner01a110ac-2da3-73d1-9bb2-273d4ff99e7a,
inherit, no nested agents, master. Existing M01/D04/IDENTITY requires current
incarnation/generation binding even when CGWindowID is reused. Actual source
inspection found live_manifest_path is only a.json from the last explicit Snapshot;
close rotates an in-memory generation but does not update that file. Its reread
cannot prove current generation. This is a concrete missing source owner, not a
new product feature or a reason to claim old freshSnapshot proofs more broadly.

Restore current NATIVE/NATIVE-PILOTS M01/IDENTITY/LIFECYCLE/EXCHANGE/PRIVACY and
full M01/D02/D04/D05 Native-acquisition closure. Root selected an identity-only
fixture receipt under existing scoped fixture-binding support, not arbitrary-app
identity. No canonical wire/version, user-facing layout or oracle change.

Implement a bounded identity-only file in the EXISTING caller-owned temporary
fixture run directory. Reuse launch/session/window key and surfaceGeneration.
Initial explicit Snapshot publishes an open binding; close first invalidates it
with rotated generation/closed state. Reopen stays invalid until a new explicit
Snapshot binds the actual current window. Do not trigger measurement publication,
collect geometry/probe on lifecycle event or add polling/timers. File contains only
fixed identity/state/version fields, no UI text or history. Write own file atomically
with existing source-owner mechanism; no new directory/archive or general registry.

Native helper gets the exact identity path from trusted operator configuration,
not UI content or arbitrary discovery. Use existing bounded no-follow reader and
public process/window/unique AX binding. Check expected incarnation/window key/
generation and open state before acquisition and before publishing that channel.
Missing/malformed/closed/mismatched or racing record fails closed with existing
canonical target/stale/incomplete meanings; do not trust input generations alone.
Preserve prior ACKed channels if later revalidation fails. Reader/file caps and
actual property/encoding limits remain, no broad all-window scan or extra quota.
Probe retains original measured time/cache-unverified; current identity check does
not turn its stored measurement into current data. Older unsupported fixtures lacking
this new identity evidence must refuse safely, not gain a silent bypass default.

Write only fixtures/native/Fixture.swift identity lifecycle/manifest publication
lines (no visible view/layout/control/expected changes); tests/bridges/native/
Collector.swift, plugins/macos/{HostProtocol,HostHelper}.swift and directly needed
existing trusted-config builder/caller in host_observe.py/native_fixture.rs;
nearest tests under existing acquisition/host_helper directories;
docs/development/native-helper.md, fixtures/native/README.md and
receipts/M01-current-identity.md. Declare actual subset. No Core/schema/Cargo/Web/
other project/new directories. Return actual dependency rather than new framework.

Tests target the true lifecycle: open current success; close invalidates despite
unchanged last Snapshot; reopen/CG ID reuse old generation refuses; fresh explicit
Snapshot/new generation succeeds; wrong session/window, absent/malformed/oversize
and changed-during-acquisition identity refuse without publishing observed data.
Reuse real reader/owner, no duplicated identity algorithm. Compile affected own
fixture/helper/caller with current-operation temp; no old broad suites or geometry
oracle edits. No new runtime in this source step. Return concrete same-title A/B,
close/reopen current CLI handoff on matching rebuilt fixture, truthful compatibility
and source hashes; commit/push scoped coherent step through short Git lease.

This adds necessary identity evidence to own debug fixture only. It does not make
arbitrary macOS applications identifiable, prove pointer mapping, change permissions,
retry B capture or replace independent M01/pixel/action/performance gates.
