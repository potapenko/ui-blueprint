# Guest Tour source-local observation

This experiment converts the fixed S14 G2 collector's successful public receipt
into the existing canonical Snapshot format. It never attaches, evaluates target
expressions or operates an application. The input remains necessary provenance;
normalization is not independent proof that a receipt came from the collector.

The source pin is unchanged PlayPhrase.me revision
`e19b20c474afe6462786ca0dd3dd5ad9ae37dbce`,
`PlayphrasemeGuestTourConnections.swift` SHA-256
`fed34661f15b526f70f07f352b0ae1cd4b5a3ded1e2ce6dcc0258f583bab884a`.
At line 29, column 28, `rect` is `geometry[anchor]`; `target` is the target's
global rectangle translated by this GeometryReader's global origin; `start`
is the target midpoint. Only the first two are published as layout frames.
The original source case name identifies their connection. No groups, window
transform, viewport, connector path or other card frames are inferred.

```sh
python3 -B experiments/native/guest-tour-layout/normalize.py PUBLIC_CARD_JSON > SNAPSHOT_JSON
```

Run the matching installed `uiblueprint-validate` on that output, then use
ordinary `inspect` and `measure` with its exact keys and coordinate space.
The format stays core 0.1.0. Coverage is partial, presentation consistency and
freshness remain unverified, and element/surface identities are observation-local.
Nanoseconds are converted to canonical milliseconds; original integer timestamps
are preserved as source metadata. Never merge separate hits into one observation.

The focused synthetic check takes the matching distribution directory and an
existing task-owned temporary output directory:

```sh
python3 -B experiments/native/guest-tour-layout/check.py UIB_BUNDLE TASK_OUTPUT
```

It exercises malformed/foreign/incomplete receipts, canonical validation,
design inspection and a 77.5 pt width through the real Rust CLI. It acquires no
application data. This passed against installed 0.6.1 with 13 refusal cases.

## Scope and basis

Execution authority: the operator's November 23 S14 fix assignment, relayed in
chat `01a20922-fc3b-7d30-935a-652c6db5f3dc`; original consumer
`01a20845-8b81-7e81-8c46-9f90b9f8f960`. This is a bounded supporting artifact,
not a new acquisition permission or release admission. The existing collector
remains owned by `owner_window_resize` until handoff.

Registry 90 → BOUNDARIES@1 / MODEL@1 / IDENTITY@1 / GEOMETRY@2 / PRIVACY@1 /
EXCHANGE@2 / LIFECYCLE@1 / CLI@21; analysis uses ANALYSIS@2 and its TYPES@1 /
VALIDATION@1. The external debug-capture@14 route does not admit a new Guest Tour
target; the separate reviewed one-attempt owner packet governs acquisition.
Observed implementation establishes the source-local relationship above. This
adapter proposes only its lossless representation using the existing schema.

## Actual first observation and remaining work

The owner completed the one admitted G2 run on November 23: one hardware hit,
normal detach, the same PID/birth/CG window, responsive AX/native capture, then
LLDB exit 0. `context.card` measured 228 × 95 pt. The Rust engine measured its
horizontal gap to the source target as 23.935294000000113 pt. Canonical validate,
inspect and four measurements passed on the actual receipt.

This was an intermediate zoom state: before 1920 × 1050, collector-after
1910 × 1046, subsequently settled 900 × 572 pt. It does not measure the settled
narrow image. The one-attempt package is consumed and must not be reused.
All 20 card IDs remained present according to the owner's independent postcheck.

Evidence retained outside Git: `/Volumes/Develop-2TB/Develop/tmp/` contains
`uib-s14-normalize-2fzuh8dg/real-context-snapshot.json` and corresponding receipt,
queries and measurements. The consumer's `playphraseme-guest-tour-01a20845-sy9r4vhv/`
contains `evidence/source-g2-probe-receipt.json`, transcript and before/after native
captures. Original acquisition is `uib-s14-g2.DtXJ4G7P/public-card.json`.

The investigated next candidate avoids the first frame of an animated resize.
Its proposed bounded method was: establish a settled window first, arm the fixed sites,
then let the owner open Tour through its normal control. Collect only the fixed
public card/target frames, map viewport/origin, connections origin and card/group
IDs at their existing source sites. Preserve separate observation intervals;
require the expected card set, matching stationary-window identity and a normal
detach. Missing fields or incomplete sets do not become a complete map.
Qualify that change on an owned console fixture and obtain non-author review
before a new consumer run. A new packet replaces, rather than resets, the consumed
one. Full-map, stable wide/narrow and composition acceptance remain open.

Runtime evidence belongs in the task-owned external area, not in this source
directory. No released SDK or shared CLI behavior changes.

## Sequential acquisition qualification

`fixture.swift` and `sequence_fixture.py` are fixture-only. The Python file must
be loaded in an owned interactive LLDB, with `__file__` set to its source path,
`S14_SEQUENCE_RUNTIME` naming the prepared internal `uib-s14-map-*` directory,
and `S14_SEQUENCE_MODE=map` or `groups`. That directory contains the signed
owned `Control`, the pinned deny-write `debugserver`, `tmp/`, and
`fixture-pins.json` with control/source SHA-256 and Mach-O UUID. It cannot select
a product PID. Compile with `swiftc -g -Onone -module-name S14MapControl`, using
an external module cache and the existing console-fixture debug entitlement.

The map pass replaces one owned hardware breakpoint at a time: map locals →
connections origin → repeated card locals. On the same fixture process it then
changes the ordinary input from baseline to expanded. Actual qualification
observed viewport widths 600 → 800, card widths 100 → 160, and three → two cards,
followed by normal detach, `recovered` and exit 0. All literal frame values were
checked independently. The groups pass observed the same three → two transition.
Evidence: `uib-s14-normalize-2fzuh8dg/fixture-{map,groups}-two-phases.json`.
Current internal runtime: `/private/tmp/uib-s14-map-t125icnf`; no live session
remains after those tests.

Earlier qualification exposed two concrete constraints. After attachment macOS
reparents the inferior to debugserver; `Popen.poll()` can cache a false exit on
ECHILD. Live identity now uses the actual image, and the child is waited only
after detach. Also, installing a third simultaneous user breakpoint was refused.
The sequential method keeps one owned point; LLDB's own additional hardware-point
warnings are retained, never replaced with software breakpoints. Failed fixture
attempts were explicitly recovered in their original PTYs before exiting.

The fixture's raw enum case prints `group {}` without the associated group ID.
A bounded one-byte fixture read distinguishes its cases, but this is not yet a
qualified decoder of the app's group data. In the actual app's row child closure,
DWARF exposes `card` and `self` but does not capture `attachment`; do not reuse the
fixture selector there. Actual group extraction remains an explicit dependency.

Offline G2 sites resolve uniquely in module UUID 3456EB33-6C0E-393F-8486-10961C55A994:
Map.swift 23:13 → 0x116de4c (`origin`, `viewport`, `scale`), Connections.swift
25:13 → 0x1169a0c (`origin`), Connections.swift 29:28 → 0x116a4d0 (card pair).
Full filenames retain the PlayphrasemeGuestTour prefix. These are file addresses,
not live load addresses or product admission. The next implementation is a fixed
stationary-window profile using these sites and the existing G2 lifecycle guards;
then non-author review and one batched owner run. No consumed packet is reset.
This former full-map continuation is superseded by the consumer direction below.

## Current handoff: component-level requests

The actual human message `01a20941-e1d1-79d0-b8fe-5b32e49d40a8` in consumer chat
`01a20845-8b81-7e81-8c46-9f90b9f8f960` was read directly. It requires transferring
Guest Tour logic first, then checking individual tooltips; full-screen pixel
matching and complete map acquisition must not block that work. The consumer
has explicitly withdrawn new live sessions on its PID under the old full-map
objective and is preparing a read-only implementation plan.

Consequently, do not prepare or launch the former full-map owner run. Preserve
the actual context receipt and canonical analysis as a usable starting point,
with their intermediate-state limitations. The fixture-only sequential work is
saved supporting research, not a released complete S14 fix. The next acquisition
depends on the consumer naming the selected component, state and missing fields
from its new plan. No new user permission loop is needed for already-authorized
work; the missing dependency is the revised concrete component request.
