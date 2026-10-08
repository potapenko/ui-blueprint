# Observed document and comparison export input

- Node type: leaf; domain: `uib.cli.export`; contract: `UIB.CLI-EXPORT@2`.
- Authority: Active / Stability: Evolving; accepted/released implementation: none.
- Authority source: G09-Export dispatch under approved PLAN.UIB@1 / ROADMAP P6 delegated concrete CLI choices; EXPORT.CONTENT already requires observed document packages.
- Read when: connecting saved canonical observations to imagegen-prompt.
- Do not read when: unchanged collection/action/analysis commands suffice.
- Requires: [EXPORT@1](export.md) and full drawing closure; [EXCHANGE@1](exchange.md), [PRIVACY@1](privacy.md), [ANALYSIS@2](analysis.md).
- Precedence: [registry](../README.md); preserves CLI.CONTENT and existing --brief behavior.

## UIB.CLI-EXPORT.INPUT

```text
imagegen-prompt --snapshot FILE --metadata FILE --out NEW_DIRECTORY --max-input-bytes N --max-output-bytes N --max-components N --max-views N --components-per-detail N [--purpose document|explain] [--profile blue-engineering] [--json]
```

Document input form: existing --brief FILE or --snapshot FILE plus --metadata FILE. Mixed forms reject2. --metadata without --snapshot or
--snapshot without --metadata returns export_metadata_required/2. Direct snapshot form is document (default, explain alias); other modes
reject2 and retain their existing --brief entry. FILE is an explicit local regular file, never implicit stored-ID resolution. Snapshot
loader reuses existing strict core0.1 Document validation: Snapshot or ChannelResponse.result=Observed only. Failed response/wrong
artifact/multiple JSON records reject invalid_input/2. One selected observed response line is one input; multiple channels are not guessed,
merged or silently discarded.

Metadata JSON is an export-owned object containing exactly: metadata (existing full Metadata), state, scope, environment,
safe_source_reference (strings), not_depicted (string array), public_text_fields (existing Field array). All fields are required; no
geometry, source_kind, coverage, components or source Snapshot may be supplied through this file. Reuse existing Metadata validation:
document_id, revision, title, audience, language, date, owner, retention, specification_refs, approval {status,named_record}, page_format,
output_size. No implicit date, audience, approval or measured environment is invented. Caller must use explicit unknown for unavailable
state/environment. A fixed explanatory note identifies state/environment/scope as caller annotations, not new observations. The compiler's
public-field allowlist/redaction rules apply unchanged; [] exports no unreviewed public text. Value fields and pixels are never
automatically included.

Adapter assembles one observed ViewInput with id="observed", title=metadata.title, the supplied annotation fields and the unchanged
canonical Snapshot. Purpose is document; details/comparisons/transitions empty. Existing compiler produces all six files, complete declared
source scope, density-based detail sheets, factual GeometryQuery dimensions, source aliases/evidence and full self-contained A+B text.
Source IDs never become meaningful labels; source availability/coverage/freshness, frame kinds/Space/units/origins and unknowns remain
unchanged. No new collection, reference IO, geometry inference, precision conversion, approval or image check.

## UIB.CLI-EXPORT.BOUNDS

Both explicit files share one positive max-input-bytes read budget; the compiler's serialized assembled DrawingBrief must also fit that same
caller cap. Node/view/ density limits, complete package plus stdout output cap and new-directory-only writer are reused unchanged.
Prepublication failures create no output directory; no partial stdout. Physical write errors remain IO1, existing destination2. Metadata
parse/missing-field errors are bounded export_metadata_required/2 or export_invalid_input/2; privacy/status failures retain existing export
codes. Compact/JSON result_version0.1.0, six-file/package version0.1.0 and exits0/1/2/5 retain the existing contract. Success means package
written, not live completeness or generated/checked image. Independent privacy/input review and affected binary proof remain acceptance
evidence, not registration itself.

`G09-EXPORT-INPUT-001`: compatible additive explicit observation input; no changes to core0.1, analysis0.2, observe/inspect/diff/actions,
compiler numerical semantics, proposal modes, root dependencies, collectors or product intent.

## UIB.CLI-EXPORT.COMPARE — E03 additive representation

`imagegen-prompt --before FILE --after FILE --metadata FILE --purpose compare` uses the same required out/limits/profile/json flags. Purpose
may be omitted and then defaults to compare for this input form. Both sources use the existing Snapshot/observed ChannelResponse loader. All
three files share the input budget. Mixed brief/snapshot/pair forms, missing side, duplicate flags and other purposes reject2 before
publication. No source reference IO, capture or model call. Metadata is {metadata, before, after, different_basis, geometry_space}; each
side has title,state,scope,environment,safe_source_reference,not_depicted,public_text_fields. All annotation fields are required;
different_basis and geometry_space are optional nullable selectors. geometry_space selects an existing unambiguous full Space by ID; it supplies no geometry,
transforms or source authority. No date/state/coverage is inferred.

Both direct pairs and existing observed --brief comparisons use Rust compare_graph on original Snapshots. Source-order indices reference the
corresponding safe scene arrays; entries retain kind, field, presence, content_changed and evidence_changed. Each entry also includes
before/after public facts, applying the existing export allowlist/aliases/redaction before encoding. Withheld values remain withheld even
when the engine reports a change; flags do not authorize recovering their contents. Source facts/coverage/observations on both sides remain
complete within the existing public export projection. Missing records never mean deleted, created or empty. No heuristic matches, action
refs, freshness, chronology, causality or atomicity.

The additive comparison_results in scene/prompt/brief has before/after view IDs, status, scope, entries, omitted_entries, geometry, safely aliased before_context/after_context and limitations. Full engine scope is nodes/properties/children/metadata/relations/components/five focus axes. Snapshot envelope,
surface_records and captures are not standalone compared. An empty entry list means only no differences in that scope. No truncated package:
engine entries are capped by output bytes (each entry exceeds one byte); any omitted entry or total output overflow refuses2. Source partial
coverage remains independent. Incompatible observed contexts require different_basis or refuse2; with the note, status=incompatible_context
and no derived match. Proposal/mixed pairs retain side-by-side semantics with status=different_source_bases, never fabricated diff. Distinct
environments are allowed by G13 without relaxing CACHE/Delta.

Optional geometry_space on ComparisonRequest (default null) runs compare_geometry for exact matched nodes' recorded geometry fields in the
selected sourced Space, with independently bound evaluations and no extra transforms/conditions. Both full Space definitions must agree;
missing/ambiguous Space rejects2. Unknown frames and missing mappings retain typed reasons; displacement exists only for two known rects.
Surface mismatch retains explicit incompatible status, without guessed conversion. Without selection, only literal geometry property changes
are reported, no displacement.

Packages containing comparisons and their CLI receipts use export version0.2.0; other packages/receipts stay0.1.0 byte-compatible.
comparisons retains its view/basis fields and aliases any selected Space ID; comparison_attribution is engine_recorded_graph, partially_compared or not_compared from actual results;
no unresolved_g02. Core0.1/analysis0.2/raw diff1.0/G12/G13 stay. Six files, new destination, sanitized errors/exits0/1/2/5 and independent
statuses remain. Independent privacy/input acceptance is separate from author verification.

E03-EXPORT-COMPARE-001: Restore EXPORT/GOLDEN under approved PLAN.UIB@1 and [E03 packet](../../plans/ui-blueprint/packets/E03-observed-compare.md), delegated ROADMAP representation; CLI@14→15, CLI-EXPORT@1→2, registry27→28 before code. Requires for this clause: [G13](cli-graph-diff.md), [G12](cli-geometry-diff.md).
