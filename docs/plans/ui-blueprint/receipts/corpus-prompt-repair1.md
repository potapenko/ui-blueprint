# CP1 — literal corpus prompt repair, 2026-10-11

Terminal result: **14/14 Native CLI exports plus CW14 Web holdout fit the observed 32,000-character
ImageGen input limit; 4 fresh generations completed; 4 reviewed FAIL, 0 checked
images.** The export-size blocker is repaired. Full image acceptance remains open;
this checkpoint must not be counted as accepted corpus images.

## Authority, traversal and plan

Finite CP1 shipping_product packet from root chat `01a11088-e608-7801-bdfb-db5c9383af9d`
explicitly ordered implementation, focused checks, actual CLI/ImageGen, commit/push
without intermediate approval. Mode Restore; single worker, no delegation or goal.
AGENTS → implementation governance → product-truth core/routing/change/evidence/
delivery/coordination → registry37 → product branch → EXPORT@2 → PACKAGE/STYLE/
GEOMETRY/PROMPT-A/PROMPT-B/REVIEW@2 and EXAMPLE@1 → DRAWING@1.2 HBP/R2/R3.
CLI@16/CLI-EXPORT@2 document input; DESIGN-ANALYSIS@1 → ANALYSIS@2/TYPES/VALIDATION@1;
MODEL/PRIVACY/GEOMETRY/PROJECTIONS/IDENTITY/BOUNDARIES@1 and EXCHANGE@2 fully read.
RUST, DEV.RUST@2 and its selected D01/D07 closure, QA and external-wait safety read.
Built-in imagegen skill used. No new contract meaning, package version or input form.
Stale @1 links resolve to current @2 leaves under registry precedence.
Excluded: collectors, source app, engine/schema/Cargo/dependencies, actions/runtime
collection, root coordination and other workers. Existing image untouched.

Contract: concise selected human presentation with exact machine truth and separate
source/frame/state/text channels. Observed: CN-B1's 80–160-node real Native sources
produced 32–57k prompts, primarily repeated object descriptions and state captions.
Implementation choice: define objects once, link details with unprinted local
ordinals, hoist shared geometry context, retain named help text and every detail
reference. These are presentation references, not source identity/component mapping.
Write set: `crates/export/src/compile.rs`, `prompt-template.txt`,
`crates/export/tests/compiler.rs`, this receipt. No CLI adapter edit was necessary.
Plan executed: compact existing formatting → focused regression/privacy checks →
freeze compiler → repeat original commands → fresh original-reference images →
review; one template correction followed by another frozen fresh cycle.

## Implementation and compatibility

Source checkpoints: `954ecde693d245399b1ae39976ffe5558257cbb8`, then final
`4d6e0bf8b921c4f2fb02b55cca3bd6c91ab1ebae` (allowed AX/disabled captions,
explicit no-photo/tonal inserts, literal approximation marks).
Compiler also passes existing output size/page format into the prompt. Known
disabled state has a textual marker; absent state is not enabled. AX dimensions
explicitly describe accessibility rectangles, never glyph/layout/paint/hit bounds.
A12 Play's 1920×998pt AX frame remains unchanged; no new bounds were authored.
All 14 actual CLI packages have byte-identical `scene.json`, `dimensions.json`,
`sheets.json` against their original CN-B1 packages. Exact numbers, provenance,
availability, partial coverage, properties and design analysis remain intact.
Six files, old inputs/versions, privacy gates and byte-limit refusals remain.

| Original case | Before chars | Final chars |
| --- | ---: | ---: |
| A01 narrow | 44606 | 27114 |
| A01 wide | 47199 | 28606 |
| A02 overflow | 28336 | 18470 |
| A03 Common Phrases | 33375 | 21023 |
| A04 Vocabulary | 32113 | 20128 |
| A05 Grammar | 32671 | 21111 |
| A06 Settings | 46819 | 23700 |
| A07 Genre | 52376 | 26063 |
| A08 Director | 56788 | 26380 |
| A09 Cast | 52345 | 26032 |
| A10 Voice | 55957 | 26264 |
| A11 grid | 33311 | 17171 |
| A11 narrow | 53836 | 27177 |
| A12 viewer | 13923 | 11216 |
| CW14 Web holdout | 35112 | 27410 |

## Verification and bounded image outcome

Final `cargo test --locked -p uiblueprint-export --test compiler -p uiblueprint-cli
--test export_binary`: 38+21 pass. Design-corpus 11 pass; export Clippy all-targets
with `-D warnings`, changed-file rustfmt and diff whitespace checks pass.
Protected live lifecycle test was explicitly ignored for missing V02 fixture
inputs; no live secret-input acceptance claimed. Existing privacy/refusal tests pass.
No workspace-wide or collector tests, no shared target/debug writes.

Each generation used the entire literal CLI prompt and original source PNG only.
Both candidates were committed before image calls. A01 first fails photo insertion,
lost approximation marks/incorrect feature height; second removes photo and improves
≈ but omits right-column content and boxes plain karaoke words. A06 first/second
retain broad Settings controls and measured 28pt relationship, but fail clear AX
frame attribution, local dimensions/state presentation and full readable text.
All four actual images are 1672×941, despite explicit 3840×2160 prompt request.
No raster is marked checked. No source-acquisition gap or incomplete popup is fixed.
No guarantee is made that arbitrary larger scopes fit 32k; existing caller limits
remain the boundary. Independent acceptance remains root-owned, not self-certified.
Root-supplied CW-B2 holdout: same frozen binary, original inputs/limits; 215 nodes,
65 omitted, partial preserved; three machine files byte-identical. No new generation; `web-holdout.json` retains exact command/hashes.

## Exact deliverable and retention

System-temp corpus root: `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-CP1-2v302h3b`.
`corpus-verified.json` records exact argv, input/reference/prompt hashes, receipts,
all lengths, source blobs and final package paths. `review.md` records all images.
Frozen final binary: `uiblueprint-4d6e0bf`; SHA256
`3d77ad518939c59247888a6b2c1641f97cbcf1c9c206c5191b70399751afe7e9`.
Original sources remain in CN-B1 root `uib-CN-B1-_djmb16k`; no source rewriting.
Selected packages/frozen binaries retained for CORPUS root/new batches. All image
copies and containing directories retained; tool-managed originals also remain.
Run-only build/intermediate non-image artifacts removed separately; no images deleted.
