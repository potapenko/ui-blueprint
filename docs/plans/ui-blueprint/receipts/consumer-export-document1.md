# USE-EXPORT-DOC1 — readable brief and reference handoff

2026-10-13. Finite shipping-product task from root, based on USE-MAC2 consumer
feedback under the user's utility-improvement authority. Source
`2ed8fee60243f73dd83dd21047083a66272f2336`, pushed to current master.
Owned: `crates/export/src/compile.rs`, existing `tests/compiler.rs`,
[export workflow](../../../development/export.md), this receipt. No new flags,
configuration, schema/version, files in the package, framework or dependencies.
Source apps, CLI/engine/schema/Cargo/spec tree and other owners' WIP untouched.

## Reconciled basis and observed problem

Traversal: current AGENTS/implementation/product-truth/QA →
[registry47](../../../specs/README.md) → product → EXPORT2 → PACKAGE3/STYLE2/
GEOMETRY2/PROMPT-A2/B2/REVIEW2/example1 and HBP1.2 R2/R3; CLI20/CLI-EXPORT3,
SHEET-PROMPTS1 and MODEL/BOUNDARIES/IDENTITY/PRIVACY/PROJECTIONS/GEOMETRY1,
EXCHANGE2/ANALYSIS2/TYPES1/VALIDATION1; RUST/DEV.RUST with current D01/D07.
Exporter contracts unchanged. New Observe/Web/compare features and raster experiments
excluded. Existing compare/flow brief path is deliberately unchanged.

Contract requires a human-readable brief retaining essential meaning and precise
safe machine facts, bounded output and separate statuses. Consumer's Reels brief
was410855bytes: four pretty-printed JSON blocks duplicated metadata/scene/dimensions/
sheets already emitted separately. This was a presentation defect, not bad arithmetic.
`references_count=0` correctly represented no accepted/copied image input; the missing
consumer guidance was a generation-handoff omission, not a collector failure.

## Delivered behavior

Document/propose/detail briefs now contain a document/status summary, source scope,
coverage/environment/caller state, requirements/exclusions/unknowns, geometry contexts,
public source-object table, selected anchored dimensions/chains, linked sheet plan
and review instructions. Existing exact JSON remains authoritative; safe record IDs
join it. Markdown cell text is escaped; only already-sanitized scene content is used.
Raw Value, private source identifiers and unapproved fields gain no new export path.

Reference guidance states that images are not bundled, loaded or verified. Per-sheet
entries distinguish original-image artwork, empty measurement sketches and optional
proposed appearance references. The caller attaches a reviewed same-scope/state
original only for artwork that needs it. Proposed geometry and empty sketches can
be used without one; no universal reference requirement or new refusal was added.
`package_written` remains data preparation, not image-ready/checked/accepted.
No ImageGen prompt, geometry/assignment/CP20 rule, machine semantics or output version
changed. No image/network lookup, live collection or model call added or performed.

## Reproduction and verification

Baseline: consumer core bundle under `uib-USE-MAC-COMPACT1-4ynh3gg5/core-bundle`,
consumer-reported core e3ab912d; binary checksum in handoff index. Original Reels
observed.json/metadata.json from `uib-CN-B19-bway_om4/B19-06-reels-populated` unchanged.
Same consumer limits: input1048576/output8388608, components160/views16/density30,
explicit purpose document and blue-engineering profile. Second case: existing
synthetic ProposedLayout fixture, same bounds and explicit purpose propose.

| Case | Old brief bytes | New brief bytes | Other files |
| --- | ---: | ---: | --- |
| Real Reels,50components,partial,G01/D01/D02 | 410855 | 16065 | all JSON and all prompts byte-identical |
| Proposed fixture,10components,G01/D01/D02 | 45147 | 10432 | all JSON and all prompts byte-identical |

Ten focused existing checks passed: historical machine preservation with new brief
meaning/reference assertions; public text/privacy; scope/exclusions; split ownership
and exact-byte boundary; four fragment/CP20 checks; public formula fidelity; existing
design-analysis summary. Export all-target check and Clippy -D warnings, locked CLI
build, formatting and git diff --check passed. No full unrelated suite or ImageGen.
Actual CLI output-limit refusal: exit2/output_limit, empty stdout, destination absent.
Generated Markdown links resolve; all50 Reels records and20 selected dimensions are
represented. All original input and unaffected output hashes reverified.

## Ready consumer handoff

[Artifact index](/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-USE-EXPORT-DOC1-e9094rmm/handoff-index.json)
SHA-256 `6bb84b581ef88d311b1f971fc6c9a3670962b71b91f1e5871606c7ddf8c5da21`.
[Ready CLI](/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-USE-EXPORT-DOC1-e9094rmm/uiblueprint-doc1)
SHA-256 `a2dc06e12913d17223ffa8ec3da1c1e1fa59b7ad6d52167ae3195efe41a8d11c`.
[Reels brief](/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-USE-EXPORT-DOC1-e9094rmm/candidate-reels/drawing-brief.md).
Exact original/candidate commands, hashes, checks and `consumer_retry_argv` are in
the index. Literal retry (new destination; existing directories must not be reused):

```sh
/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-USE-EXPORT-DOC1-e9094rmm/uiblueprint-doc1 imagegen-prompt --snapshot /var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-CN-B19-bway_om4/B19-06-reels-populated/observed.json --metadata /var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-CN-B19-bway_om4/B19-06-reels-populated/metadata.json --out /var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-USE-EXPORT-DOC1-e9094rmm/consumer-retry-reels --max-input-bytes 1048576 --max-output-bytes 8388608 --max-components 160 --max-views 16 --components-per-detail 30 --purpose document --profile blue-engineering --json
```

Limits: the50-row source table is still substantial; per-property evidence and
complete graph details require the linked machine files. No localization, image
readiness or raster-quality claim. Final workflow/readability acceptance belongs
to the same Mac consumer via root, not this author's checks. Existing shared bundle
and original packages untouched. Ready binary/packages/index retained for consumer;
only run-owned image-free build target removed. No task images created or deleted.
