# Observed document export input

- Node type: leaf; domain: `uib.cli.export`; contract: `UIB.CLI-EXPORT@1`.
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

Exactly one input form: existing --brief FILE or --snapshot FILE plus --metadata
FILE. Mixed forms reject2. --metadata without --snapshot or --snapshot without
--metadata returns export_metadata_required/2. Direct snapshot form is document
(default, explain alias); other modes reject2 and retain their existing --brief
entry. FILE is an explicit local regular file, never implicit stored-ID resolution.
Snapshot loader reuses existing strict core0.1 Document validation: Snapshot or
ChannelResponse.result=Observed only. Failed response/wrong artifact/multiple JSON
records reject invalid_input/2. One selected observed response line is one input;
multiple channels are not guessed, merged or silently discarded.

Metadata JSON is an export-owned object containing exactly:
metadata (existing full Metadata), state, scope, environment, safe_source_reference
(strings), not_depicted (string array), public_text_fields (existing Field array).
All fields are required; no geometry, source_kind, coverage, components or source
Snapshot may be supplied through this file. Reuse existing Metadata validation:
document_id, revision, title, audience, language, date, owner, retention,
specification_refs, approval {status,named_record}, page_format, output_size.
No implicit date, audience, approval or measured environment is invented. Caller
must use explicit unknown for unavailable state/environment. A fixed explanatory
note identifies state/environment/scope as caller annotations, not new observations.
The compiler's public-field allowlist/redaction rules apply unchanged; [] exports
no unreviewed public text. Value fields and pixels are never automatically included.

Adapter assembles one observed ViewInput with id="observed", title=metadata.title,
the supplied annotation fields and the unchanged canonical Snapshot. Purpose is
document; details/comparisons/transitions empty. Existing compiler produces all
six files, complete declared source scope, density-based detail sheets, factual
GeometryQuery dimensions, source aliases/evidence and full self-contained A+B text.
Source IDs never become meaningful labels; source availability/coverage/freshness,
frame kinds/Space/units/origins and unknowns remain unchanged. No new collection,
reference IO, geometry inference, precision conversion, approval or image check.

## UIB.CLI-EXPORT.BOUNDS

Both explicit files share one positive max-input-bytes read budget; the compiler's
serialized assembled DrawingBrief must also fit that same caller cap. Node/view/
density limits, complete package plus stdout output cap and new-directory-only
writer are reused unchanged. Prepublication failures create no output directory;
no partial stdout. Physical write errors remain IO1, existing destination2.
Metadata parse/missing-field errors are bounded export_metadata_required/2 or
export_invalid_input/2; privacy/status failures retain existing export codes.
Compact/JSON result_version0.1.0, six-file/package version0.1.0 and exits0/1/2/5
retain the existing contract. Success means package written, not live completeness
or generated/checked image. Independent privacy/input review and affected binary
proof remain acceptance evidence, not registration itself.

`G09-EXPORT-INPUT-001`: compatible additive explicit observation input; no changes
to core0.1, analysis0.2, observe/inspect/diff/actions, compiler numerical semantics,
proposal modes, root dependencies, collectors or product intent.
