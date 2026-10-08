# Explicit Web document collection

- Node type: leaf; contract: `UIB.WEB-DOCUMENTS@1`.
- Clauses: `UIB.WEB-DOCUMENTS.SCOPE`, `.FACTS`, `.BOUNDS`.
- Authority: Active / Evolving; implementation/independent acceptance separate.
- Authority source: approved PLAN.UIB@1, ROADMAP D03 and [W06 packet](../../plans/ui-blueprint/packets/W06-web-fidelity.md).
- Read when: explicit whole-document Web observation or raw AX focusability.
- Requires: [MODEL@1](model.md), [EXCHANGE@2](exchange.md), [PROJECTIONS@1](projections.md), [PRIVACY@1](privacy.md), [D04@1](../development/decisions/d04-identity.md), [D05@4](../development/decisions/d05-limits.md), [D06@1](../development/decisions/d06-performance.md).

## UIB.WEB-DOCUMENTS.SCOPE

`W06-FIDELITY-001`, Restore, registers an additive technical representation before
code. D06 already requires full F01 two-document/97-node cold coverage and raw
single-control semantic fidelity. This leaf does not change that workload or gates.
Existing rooted/initial/references paths remain single-document and unchanged.
An explicit Documents selection names the exact ordered Surface/frame-loader and
document backend IDs, per-document caller sensitivity and finite visit allowance.
Every Surface must already belong to the trusted attached SessionDescriptor and
match the canonical Request; selection data cannot grant new Surface authority.
The supported collection is the entire explicitly authorized same-process frame
tree. Missing/extra/wrong/stale frames refuse; no title/order matching, OOPIF,
shadow-root or arbitrary-frame qualification. Verify target/root continuity and
all frame loaders before and after bounded document checks and native acquisition.
No partial graph is published if the selected document set changes.

## UIB.WEB-DOCUMENTS.FACTS

Existing core0.1 nodes, children, SurfaceRecord and ExtensionProperty own the data;
no second graph, new canonical Field or version migration. `web.ax.focusable`
is a reported boolean extension with Field::Value storage selected by Focused.
It is distinct from focused and input/action authority; absent/mistyped source is
unknown, false stays known false. Its source is the addressed AX observation.

Documents uses original CDP DOMSnapshot with empty computedStyles and DOM rects,
matching the frozen full workload. DOM nodes retain backend IDs, original names,
parent/child and frame-document relations. Value selects native scalar/text and
attribute facts in namespaced extensions; LayoutBounds selects original layout,
offset/client/scroll rects and text-box facts. Native coordinates/rect kinds stay
attributed; unsupported cross-frame transforms are explicitly unknown. Source
text offsets retain UTF-16 units. Empty/false/missing and source-native metadata
remain distinct. No geometry/name/action semantics are inferred from a DOM tag.
AX and DOM namespaces remain separate; full DOM capture does not claim full AX.

## UIB.WEB-DOCUMENTS.BOUNDS

Caller explicitly authorizes whole documents; ordinary scopes never fall back to
this path. Bounded isolated native-getter preflight counts nodes/depth and rejects
unsupported boundaries before capture. Same known document set, count, IDs and
limits are checked after capture; a changed/oversized result refuses publication.
Native browser snapshot internals remain opaque SDK work, not a browser RSS/CPU
quota. Transport reply/cumulative bytes, deadline, worker allocator and canonical
output ceilings remain unchanged. No polling, hidden reattempt or collection after
terminalization. Remote handles belong to the request and are released or reported
unconfirmed under the existing collector lifecycle.

Raw protocol buffers exist transiently in the guarded adapter only. Refuse
caller-sensitive documents and known password/private DOM subtrees at preflight;
refuse the entire channel if a private node appears in the captured tables. This
avoids alias leaks through srcdoc/inline source copies. Ordinary scoped redaction
is unchanged. No raw string
table, unclassified URL token or diagnostic payload is persisted. Public source
text remains an explicitly selected native fact. This is bounded
fixture qualification, not detection of arbitrary unknown secrets. Any inability
to preserve the requested baseline facts safely remains an explicit quality gap.
