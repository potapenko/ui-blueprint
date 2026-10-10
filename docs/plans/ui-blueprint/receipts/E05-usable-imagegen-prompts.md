# E05 — usable literal real-form ImageGen prompts

Date: 2026-10-10. Finite Restore under the [E05 packet](../packets/E05-usable-imagegen-prompts.md),
approved PLAN.UIB@1 P6 and the explicit real-form → prompt → ImageGen → comparison
request. One chat; no agents, new chats, goals, branches or worktrees. Current
master only. Independent source/privacy acceptance remains root's next consumer;
this receipt reports author verification, not independent acceptance or P6/P7 release.

## Authority, traversal and exact boundary

Read active AGENTS and implementation/product-truth/QA routes, including product
core, routing, change, evidence, delivery and finite-worker lifecycle. Traversal:
registry33 → product branch → EXPORT@1 → full DRAWING-PACKAGE/STYLE/GEOMETRY/
PROMPT-A/PROMPT-B/REVIEW@1 and EXAMPLE@1 → CLI@16, CLI-EXPORT@2 INPUT/BOUNDS,
ANALYSIS@2/TYPES/VALIDATION@1 → EXCHANGE@2, GEOMETRY/PROJECTIONS/MODEL/IDENTITY/
BOUNDARIES/PRIVACY@1. Engineering: RUST, DEV.RUST@2, D01@1/D07@5, their
ROADMAP/RUST-BOUNDARIES/REUSE/evidence/decision-authority closure. Supporting inputs:
[E04 receipt](E04-real-form-imagegen.md), [export recipe](../../../development/export.md),
current exporter/template/tests and E04's retained original inputs. Older dependency
link labels resolve to current additive revisions. Native-only concurrent spec
registration does not change this selected basis. No export specification delta.

Contract: self-contained complete declared-scope facts, precise numbers, independent
source/validation/approval, safe source aliases/public allowlist, correct sheet plan.
Observed discrepancy: duplicated pretty component inventories and dimensions exceed
actual model input length; general sheet is included in enlarged details. Classified
as implementation defects. Author choice within Restore: factor identical fields
into explicit text tables, retain one complete inventory, use compact JSON for other
facts, refer to only actual detail sheets. No new model client, schema, configuration,
parameter, compressor service, dependency or deterministic image renderer.

Write set: `crates/export/src/{compile.rs,package.rs,prompt-template.txt}`,
`crates/export/tests/compiler.rs`, export recipe and this receipt. Protected:
engine/schema/CLI/host/collectors, machine packages, historical fixtures, Native/spec
work and unrelated `after-title-spacing.png` (never opened, edited, staged or deleted).
Compare formatting is affected, but comparison arithmetic/source semantics are not;
G12/G13 implementation and Native/live collection/timing/UI work were excluded.

## Pinned reproduction and implementation

System-temp deliverable root:
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-e05-1jq7290v`.

The baseline was built with pinned Rust1.96.0, aarch64-apple-darwin, `cargo build
--locked -p uiblueprint-cli --bin uiblueprint`, default features, task-temp target.
Actual build-start HEAD: `e89cd0f1b7dcf77cd745f0a60a7f24a1b52ede39`, tree
`6c2a04dde0e5e003ba5df1c8f9307d22c0a7db8c`.
Baseline binary SHA256:
`ebf6b81341c3fd32ac02eaaf21dcc0757b9f463ac6da002afaffff116903fdca`.
Both regenerated baseline prompts are byte-identical to E04's original failed
submissions: Director123391 characters, Settings529860. Thus E04's recorded actual
HTTP400 maximum32000 applies to these exact bytes; no redundant failing model call.
The baseline template explicitly places G01 in enlarged detail_views.

Candidate binary SHA256:
`eabd77bd923413d9b333b78999a21544d4169ce7e342ccf59b6436dd70a5f624`.
Build context advanced through unrelated commits to
`37a76fad70c1966c95be8cdf150dead91f605e08`; there are no committed changes between
these contexts in crates/Cargo manifests/lock/toolchain. `build-identity.json`
records exact candidate source hashes; the checkpoint supplies the final source
identity. `candidate.patch` preserves the reviewed diff against baseline, and the
exact candidate executable is retained for root's independent check.

Changes:

- Prompt-only tables list exact nested field paths. COMMON values apply to every
  row; COLUMNS/ROWS retain all other values in source order. No property or dimension
  is pruned. Null, false, empty strings/containers and absent paths remain distinct.
  Exact JSON spelling, rather than numeric equality, preserves -0.0 versus0.0.
  Numeric path segments denote arrays in the existing typed export records; object
  fields are named. There is no new public machine input/output contract.
- Reuse the bounded JSON writer with compact serialization for prompt facts.
  Source projection/redaction occurs before formatting; no raw Snapshot is copied.
  Add existing safe view/snapshot/revision/Surface context to the source annotation.
- State/action section points to the single complete component inventory. Dimensions
  retain every anchor, Space, unit, number, evidence, requirement, unknown and
  tolerance; derived chains are still present. Other facts and full sheet plan stay.
- Enlarged-detail instruction contains detail IDs only. Empty list explicitly means
  none; G01 remains general. It refers to the plan embedded in this prompt rather
  than assuming the model can open sheets.json.
- Placeholder substitution remains a single pass over the trusted template and
  public text still rejects template delimiters. The old whole-result `}}` search
  was removed because compact nested JSON legitimately contains adjacent braces;
  it was not a meaningful unresolved-placeholder check for compact serialization.

## Fidelity, privacy and affected checks

Same unchanged E04 `*-snapshot.json` and `*-metadata.json`, same positive CLI bounds
and densities12/64, same exact source PNGs; only executable/destination changed.
All six files are produced by the real public CLI. `verify.py`, `checks.json`,
`*-command.json`, `*-cli-receipt.json`, `*-before/`, and `*-package/` retain the proof.
No external summary/rewriter is part of submission.

| Case | Old → new characters | New UTF-8 bytes | Components / dimensions |
| --- | --- | --- | --- |
| Director | 123391 → 18653 | 21515 | 11 / 22 |
| Settings | 529860 → 29793 | 32657 | 47 / 94 |

Prompt SHA256 Director:
`1ecc57f1936701b774d4c75b64d96d37d4c82d6e8cf546f2d25d9f62c499ac3a`.
Prompt SHA256 Settings:
`c9b833e85f58dbee4ecb4595cff22efcd15ffb1e0ef35e424b7ddf62c9fd12dd`.

For each case the other five files are byte-identical to the new baseline build.
An independent table reader reconstructs every component/property/geometry/evidence
and dimension field exactly. All116 extents separately match raw browser CSSOM
values bit-for-bit, with correct source component anchors. All public text and
observed checked values match the independent raw records. Source inputs remain
unchanged; exact input hashes are in checks.json. Partial coverage, unknown
consistency and unverified freshness retain their meaning. `local_numeric_validation`
is checked; image validation remains unverified and approval draft.

Privacy: only the prior safe projection enters formatting; native Surface owners
are passed through the existing no-public-text alias/redaction function. Actual
prompts contain no machine path, endpoint/URL or unresolved placeholder. The same
inspected lossless PNG crops contain public form UI, without account/film/background
content. Values are not newly exported; Director's public draft `spiel` remains the
explicit author state annotation. Source declarations, private IDs, unapproved text,
value fields, payloads and captures remain excluded/redacted under existing rules.
This is not universal secret detection or independent privacy approval.

Passed final checks:

- `cargo test --locked -p uiblueprint-export`:33 tests (compiler18, compare8,
  proposal7). Existing actual V02 lifecycle test remains ignored because it requires
  its separate live secret fixture; no new live protected-input claim.
- `cargo test --locked -p uiblueprint-cli --test export_binary`:20 tests. Public
  document/propose/detail/compare/flow consumers, exact bounds/refusals, source
  immutability, statuses, public-field canaries and source aliases remain covered.
- `cargo clippy --locked -p uiblueprint-export --all-targets -- -D warnings` and
  `cargo fmt -p uiblueprint-export -- --check`.
- Changed local documentation links/route consistency and `git diff --check`.

New regression expectations use immutable historical component inventories and
independent literal current dimension values, not candidate output as a golden.
A separate property-order/signed-zero case exercises absent fields and exact values.
G01-only and actual D01/D02 sheet plans pass; all ten proposal components remain.
Five unchanged files retain the historical byte comparisons. Initial failures were
the expected obsolete prompt byte-equality check and two test-expectation issues
(object key order; first-use alias ordering); repaired tests pass without changing
source aliases, machine semantics or historical evidence.

## Literal ImageGen submission and visual result

System `imagegen` skill and built-in tool used. Source PNGs were inspected first.
The exact `prompt.txt` string was loaded and submitted once for each case, with the
same E04 lossless PNG as `referenced_image_paths`, no extra wrapper or manual summary.
Both calls succeeded. No follow-up regeneration was used to chase acceptance.
`imagegen-submissions.json` records prompt/reference/output hashes, paths and sizes.

Both outputs are1672×941, not the requested3840×2160. Tool-managed originals were
copied into system temp without moving/deleting/overwriting them. All originals,
temp copies and image-containing directories remain. Sources and results were shown
inline; generated temp copies are `director-generated.png` and `settings-generated.png`.

**Director: visual FAIL, unverified/draft.** All three real names/counts and the
`spiel` input are now present. The image contains all11 component rows and22 dimension
rows; visual inspection finds the22 listed values agree with the supplied inventory.
Only G01 is shown; no invented D sheet. However, a second boxed “Directors” row is
added below the real heading. The N007 options-list leader points to the first option,
and N008's option-one leader points to the second option. The component table calls
N006 “(empty)” although the draft annotation/reference shows `spiel`: known empty
visible_text is not evidence of an empty input value. Some component IDs are missing
from the actual leaders. Thus numeric values alone do not establish correct anchors,
complete layout fidelity or state fidelity. Dark filled/gradient cards and microscopic
notes deviate from the plain flat line style/readability requirement.

**Settings: visual FAIL, unverified/draft.** All7 selects retain their displayed
values/order; both switches appear off, descriptions/AI note and Close remain. No
extra D sheets, Save/provider/subscription controls appear. The image labels only
N000 width421.640625 and height617.7421875, both correct. It omits the component
inventory, all M IDs and92 of94 numeric dimension values; most N IDs are missing.
A source-scope note saying47 components does not replace that inventory. Dark/gradient
filled controls and reduced output size further prevent exact engineering acceptance.
The principal UI is recognizable; this is an illustration, not a checked blueprint.

No observed output contains a private URL, machine path or raw identity. Both retain
observed/unverified/draft and partial imported-reference attribution. This is the
same saved browser-reference export experiment as E04, not proof of canonical live
Observe, behavior/persistence, CAD accuracy, human approval or marketing efficacy.

## Delivery, remaining acceptance and resources

Literal export usability for the two required cases and G01/detail semantics are
repaired and exercised through real generation. Arbitrarily larger prompts are not
promised to fit a model: no component-removal fallback or new size policy was added.
Generated-image fidelity remains failed for the specific reasons above. Independent
source/privacy review belongs to root; self-tests do not close that acceptance gate.
No implementation dependency outside E05's authorized write set was required.

Root/user owns the retained minimal review deliverables through independent review
or explicit discard: original/candidate packages, prompts, inputs-by-existing-path,
checks/reproduction script, build identity/source diff and candidate executable.
Build intermediates and superseded non-image candidate packages are removed after
verification. Images are excluded from cleanup and never agent-deleted. No browser,
visible-app input, native timing reservation or background process was acquired;
no real quiet-CPU conflict was observed. N05 and unrelated changes remain untouched.
Checkpoint and push use the existing canonical SSH remote and fcntl lock at
`/tmp/ui-blueprint-master-git.lock`, empty index and exact six owned paths only.
Commit identifier is reported in the final chat rather than self-referenced here.
