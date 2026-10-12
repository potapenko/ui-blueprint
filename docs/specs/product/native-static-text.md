# Explicit Native static-text content

- Node type: leaf; domain: `uib.native.static-text`; contract: `UIB.NATIVE-STATIC-TEXT@1`.
- Authority: Active / Evolving; implementation/independent acceptance separate.
- Authority source: root's explicit CA8 shipping_product Evolve packet under the user's CORPUS-1000 utility-repair mandate, following accepted CA6/CA7 source evidence.
- Read when: an ordinary native_ax Observe explicitly requests existing Field::VisibleText.
- Do not read when: old field requests, fixture/form/probe/actions, or unrelated exporter presentation.
- Requires: [NATIVE@5](native.md), [MODEL@1](model.md), [FORMS@1](forms.md), [PRIVACY@1](privacy.md), [Native acquisition@3](../development/decisions/d05-native-acquisition.md) and their explicit closure.
- Source evidence: [CA8 receipt](../../plans/ui-blueprint/receipts/corpus-native-static-text.md) pins immutable CA7 qualification; Apple NSAccessibilityStaticText.accessibilityValue semantics are guidance, not universal runtime/privacy proof.

## UIB.NATIVE-STATIC-TEXT.SELECTION

CA8-STATIC-TEXT-001 adds only explicit visible_text field support to existing
ordinary native_ax collection modes. Default/old fields trigger no new AXValue
read. Core0.1 Field::VisibleText, typed properties and source evidence are reused;
no new field/wrapper/provider/flag, public projection permission or API is created.
Private4032-byte config and all160/depth9/byte/deadline/acquisition ceilings stay.
Fixture/form/probe/action providers gain no capability through this leaf.

## UIB.NATIVE-STATIC-TEXT.ADMISSION

Before node properties, prove its native owner belongs to the admitted process;
retain existing held-window/generation/publication ownership and checks. Read the
existing bounded role/subrole/identifier classification before dispatching AXValue.
Only role exactly AXStaticText with nativeMayReadValue's complete non-protected
classification may acquire AXValue solely for visible_text. Standard editable or
other non-static roles do not cause that read. No private/parameterized API,
role/name inference, Title fallback, source-app instrumentation or masking workaround.
The existing typed batch/copy reader admits the source string before constructing
output. Reuse its one AXValue result when both Value and visible_text are selected;
charge every output occurrence without another unbounded string or acquisition.
Selecting only visible_text emits no extra raw Value property.

## UIB.NATIVE-STATIC-TEXT.RESULT

An admitted string becomes Known(Text), including a genuinely known empty string.
Its evidence identifies reported bounded_public_ax_static_text_value under the
actual Observation/source key. It is static-text CONTENT, not proof of on-screen
visibility, painted glyph range, complete clipping, semantic name or pixel mapping.
Do not infer visible_region, layout/paint bounds, complete coverage or new identity.
AccessibilityName continues to use its existing channel; AXTitle remains a separate
extension. Raw Value retains its own meaning and export exclusion.
Known protected/secure classification returns Sensitive/Redacted without AXValue.
Missing/failed role/classification returns unknown without that read. Safely
classified non-static roles return unsupported. A non-string result is unavailable,
never coerced to text. Preserve source NoValue/unsupported/errors versus known
empty, and existing budget/deadline/identity refusal distinctions. No rejected
private payload is serialized. Coverage/consistency remain independently partial/unknown.

## UIB.NATIVE-STATIC-TEXT.PUBLICATION

Collection does not make text approved for disclosure. The unchanged exporter
requires explicit caller-reviewed public_text_fields=[visible_text] to include
this field; an empty allowlist withholds it. Sensitive properties and raw Value
remain excluded, AXTitle/extension policy unchanged. No snapshot backfill or
field renaming at export; old observations retain original unknowns.
This is bounded known-source handling, not universal private-text recognition.

## Acceptance

Focused source/type/empty/non-static/editable/protected/failed-classification,
NoValue/unsupported/oversize/aggregate-output/deadline/owner/window cases must prove
correct availability and zero unsafe/new default AXValue reads. Preserve Name,
Title, Value, other modes and existing publication/lifetime controls. One fresh
source-proved public C01 long-copy observation must validate and inspect, followed
by unchanged accepted exporter outputs with and without explicit public approval.
Raw Value/AXTitle stay protected in both. Independent privacy/capability review is
required; source, author tests and registration do not supply that acceptance.
