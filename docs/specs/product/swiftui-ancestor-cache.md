# Bounded native ancestor structural reuse

- Node type: leaf; domain: `uib.swiftui-debug-capture.ancestor-cache`.
- Contract: `UIB.SWIFTUI-ANCESTOR-CACHE@1`; Authority: Active; Stability: Evolving.
- Authority: the user's approved S15 repair and direct continuation through real
  Practice verification, plus S13's finite shared-owner handoff on2026-11-23 to
  chat `01a2094f-9996-72d3-8230-3b1e4f1ec16d` for this module and its checks.
- Requires: [NATIVE-OWNER@1](swiftui-native-owner.md),
  [debug capture](swiftui-debug-capture.md), [PRIVACY@1](privacy.md).
- Read when: reusing structural TreeElement headers in `ancestor_owner` selection.
- Accepted/released baseline: none; source tests do not qualify product capture.

## ANCESTOR-CACHE.SCOPE

One stopped observation may reuse already validated TreeElement32 headers shared
by native witness ancestry, selected traversal and transform context. Register only
explicit TreeElement sites reached by the existing graph root/parent/child/sibling
rules. A generic32-byte owned_record is not necessarily a TreeElement and must not
be registered. Geometry, model storage, property payloads and native weak handles
are never cached by this helper. Selection and source attribution remain unchanged.

## ANCESTOR-CACHE.BOUNDS

The existing1024 raw-record ceiling applies to the union of distinct discovered
TreeElements, selected raw records and transform-context records. Admission precedes
the first new header read; exactly1024 distinct records remain admissible and1025
refuses. Per-path cycle/depth checks remain separate and must not be disabled by
reuse. Existing native view/call, byte, emitted-node and deadline limits stay intact.
No cached element can stand in for a missing, foreign or ambiguous source path.

Use the existing graph/page validation before registering a header and on later
explicit TreeElement access. Check protected model ranges before every cached read,
including slices; append reused ranges to the existing privacy audit so a later
discovered protected range still causes refusal. Do not serve across a header's
boundary. Changing the reader's graph/table/base/limit binding invalidates reuse.
The cache belongs to that reader/observation only; it is neither global nor persisted.

## ANCESTOR-CACHE.INTEGRATION

The helper owns only admission, validated header storage and cached slices. The
existing CacheReader owns traversal, metadata, privacy classification, normalization,
call lifecycle and publication. Shared glue remains S13-owned. Original modes
keep their current accounting. Source/raw geometry and canonical0.1 output do not
change. Report repeated-read savings only as diagnostics, never complete coverage.

## ANCESTOR-CACHE.ACCEPT

Check repeated ancestry plus traversal/context overlap;1024/1025 boundary;
unregistered same-size memory reads; foreign/rebound graph; protected ranges added
after insertion; cross-header reads; failed or short initial read; and independent
reader lifetime. Existing cycle, duplicate-owner, stale-native and privacy-audit
checks remain required at integration. The Practice fixture must pass without a
quota increase before an exact owner-run product qualification. A cache hit or
successful test suite is not runtime acceptance.
