# CW-B8 — responsive source correction and image acceptance

2026-10-12. Root reconciliation under CORPUS-1000. Three completed source cases,
not three new families: two existing subscription-demo states and public Terms.
Current exporter5e071f5, site checkoutc23ef963; no product code changes in this batch.

## Source qualification and counts

The original own-tab viewport override reduced nativeDPR2 to1. Its three images
remain unqualified historical reference attempts, with all52original files intact
in `uib-cw-b8-V6wS4z`. Corrected sources in `uib-cw-b8-retina-N53ttw` used direct
Page.captureScreenshot PNG at nativeDPR2:640×1440,640×1440,780×1688. Native dimensions
and decoder/signature were checked; no upscaling, crop or conversion. CSSframes
match the original measurements; fresh DOM/AX/canonical exports remain attributed.
Two demo heights change476→551 and502→616 at320CSSpx with unchanged font sizes.
Terms changes800→326CSSpx column width and paragraph/heading wrapping at390CSSpx.

Three corrected-source first calls completed all three G01 sheets. No later reroll.
36layout frames,10required dimensions and6analysis requests were checked by author.
The batch adds3eligible cases, bringing corpus to150. Image totals are391primary,
47repair-validation and4unqualified reference attempts:442generated images.

Both temp roots above are under
`/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/`.
Corrected neutral-manifest SHA256
`da15380fb458b2b2eded08b61956a0dccd2236a6eee05c05eb30e9c199dff8b4`;
manifest SHA256 `c3c1f4e603ed7ec3a531f1195bdbabafada1c43daa48d6536245f44eed6ad588`.

## Independent verdicts

Same continuing critics, inspecting corrected evidence before author findings:
CV2 turn01a13175-899b-7472-9a65-d64311d2ddfd;
CT2 turn01a13175-9167-72f1-b1e4-0ebdc029efd2.
Both review actual1536×1024 outputs and agree:

| Case | Verdict | Concrete basis |
| --- | --- | --- |
| CW60 | scopedPASS | Narrow wrapping, complete demo state/content; main320×551CSSpx and button≈232×46; correct panel272/paragraph240 widths and attribution |
| CW61 | scopedPASS | Both explanatory wraps and button focus preserved; main320×616, button≈223×46CSSpx; relationship meaning remains correct |
| CW62 | FAIL | Unassigned visible intersection790.5625 printed as a dimension and repeated; long decimals and shortenedpx labels violate the selected display assignment |

CW61's authorFAIL is superseded by both independent meaning-based verdicts:
“panel layout widths” and contextual “paragraph block widths” communicate the
basis; the exact machine token `layout_bounds` is not an additional acceptance
requirement. This preserves the earlier CT3 interpretation, not a relaxed gate.

CW62's790.5625CSSpx is a true construction value, not an arithmetic error. The
literal display assignment is only326×≈3830CSSpx on a separate full-layout sketch.
The raster promotes other construction geometry into visible annotations. Its
heading ratio2 is correctly derived from76.796875÷38.3984375; full/visible frames
are not substituted. Extra text inside the requested empty sketch is another
literal deviation. See [display presentation](../../../engineering-blueprint-guide.md#hbp-human-001--клиентская-подача-2026-10-10)
and [drawing geometry](../../../specs/product/drawing-geometry.md).
No concrete literal compiler conflict was demonstrated; no automatic code repair
or generic redesign is assigned from these verdicts.

There are now3complete primary-case images accepted by both critics: CW58, CW60,
CW61. This is scoped image acceptance, not general reliability, billing/legal
validation, CAD mapping or causal proof that density alone explains the result.
Demo/viewport restored with observed1280×720/DPR2/scroll0, own tab closed, source
repos unchanged. Author archived after handoff; all images/tool originals retained.
