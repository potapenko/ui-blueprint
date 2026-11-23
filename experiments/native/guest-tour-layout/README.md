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

The next candidate must avoid collecting the first frame of an animated resize.
Proposed bounded method: establish a settled window first, arm the fixed sites,
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
