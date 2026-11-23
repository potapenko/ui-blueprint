# Native-bound SwiftUI source ancestor

- Node type: leaf; domain: `uib.swiftui-debug-capture.native-owner`.
- Contract: `UIB.SWIFTUI-NATIVE-OWNER@2`; Authority: Active; Stability: Evolving.
- Authority: approved S13 source-attributed component plan and coordinator's
  explicit shared S03 reader assignment on2026-11-23.
- Requires: [debug capture](swiftui-debug-capture.md), [IDENTITY@1](identity.md),
  [PRIVACY@1](privacy.md), [GEOMETRY@2](geometry.md).
- Acceptance: S03 prototype/negative fixtures supplied; integrated qualification
  and product admission remain separate.

## OWNER.ANCESTOR

Preserve the existing first exact nominal ancestor selection. When that path has
no matching nominal record, a body-owner fallback may use the first matching
body attribute and its outermost occurrence on the validated parent path. Do not
switch to a different attribute with the same type. Continue across intervening
wrapper owners. Require the path to reach the exact selected graph root; an
incomplete/ambiguous/foreign/cyclic path never supplies a successful fallback.

This selects a source-attributed ancestor subtree, not an arbitrary complete
body forest. An extra branch outside that ancestor remains unverified. Keep
partial coverage and never claim that one native path proves full owner coverage.
A known split branch in the S03 fixture must be included by the selected ancestor
or the capture must refuse; the earlier contiguous prototype silently omitted it
and is not accepted. Declared leaf cuts remain explicit excluded branches.

## OWNER.MULTIPLE-NATIVE

Optional `scope.native_match="ancestor_owner"` uses the existing source type and
exact native class, with optional containing_type and geometry_records selection.
Enumerate all matching native views within the
original128-view bound; independently qualify every provider/weak graph binding. Check each native view
belongs to the exact selected NSWindow before and after its weak binding; check
the host membership at both ends. A same-class view in another window refuses.
Duplicate attribute bindings, stale/foreign references or incomplete enumeration
refuse. A matching class alone never determines the source owner.

Resolve the source ancestor for each binding. Skip only a proved complete path
without the requested owner. Missing ancestry is not a negative match. Require
one unique selected ancestor across all qualifying bindings; two ancestors refuse.
After collecting that subtree, each qualifying native layout attribute must occur
exactly once. If containing_type is requested, every matched native occurrence
must have that source type on its reported ancestry.

Parent work across candidates and selected raw traversal share the original1024
record budget. No first-hit shortcut, scope broadening, text/coordinate selection,
quota increase or ignored candidate failure is permitted. Native cursor frames
remain different from the SwiftUI control's layout. No action or product admission
is created by this mode. Existing direct structural selection retains its full
owner-forest semantics; the two selection mechanisms are not interchangeable.

## OWNER.PROVENANCE

An anchored body fallback reports selection_source `native_owner_ancestor` with
one raw branch; this is explicitly different from direct body_owner forest selection.
Per-node optional extensions body_owner_type and body_owner_occurrence report the
validated StaticBody/DynamicBody ViewBodyAccessor association. Preserve private
file discriminators in flat private Swift struct names; unresolved type contexts
remain absent rather than aliasing another declaration. Occurrence tokens are
nonce-scoped, not raw attributes/pointers or cross-capture identities. These are
runtime source associations, not app-declared components, action refs, visibility
or completeness claims. Original node type and layout evidence remain unchanged.

## OWNER.SINGLE-WITNESS-CUTS

S03's source-shaped top-control fixture requires declared leaf cuts while the
same native TextField class also occurs outside the picker. Optional leaf_types
is admitted for ancestor_owner only with containing_type and geometry_records.
Resolve every candidate first. Require exactly one qualifying native witness in
the unique selected ancestor; otherwise refuse native_leaf_multiple_anchors.
Preserve that selected witness's anchor_next path, rather than the last candidate's
path. Inside a cut continue only along this path. All prior leaf-hit accounting,
ancestry/binding checks, candidate search and original budgets remain. This does
not permit multi-witness cuts or leaf_types in overlays_in_owner/within_owner.
