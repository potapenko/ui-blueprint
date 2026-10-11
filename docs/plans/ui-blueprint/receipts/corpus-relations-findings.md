# CD1 — отношения в сохранённом CORPUS-1000

2026-10-11. Конечный diagnostic receipt для следующих улучшений человеческих
объяснений и представительных prompts UI Blueprint. Это исследование сохранённых
источников PlayPhrase.me, не новая policy, не исправление приложений и не новые
generated cases. Новых изображений: 0. Историческая image acceptance не изменена.

## Основание и границы

Authority: конечное назначение CD1 из CORPUS-1000 прямо требует закончить анализ,
изменить только этот документ, checkpoint и push в текущий `master`. Координация
цели остаётся в исходном чате. Делегирования, новых целей и source edits здесь нет.
Навигация к данным: [wave summary](corpus-wave1-summary.md) и пять terminal handoffs
в указанных ниже корнях. Они не заменяют оригинальные inputs.

Traversal: [registry](../../../specs/README.md) → [product](../../../specs/product/README.md)
→ DESIGN-ANALYSIS@1 INPUT/TRUTH/OUTPUT/LIMITS → ANALYSIS@2 SCOPE/CLI,
ANALYSIS-TYPES@1, ANALYSIS-VALIDATION@1 BINDING/EVIDENCE/VERIFY → GEOMETRY@1;
MODEL/IDENTITY/PRIVACY/PROJECTIONS/BOUNDARIES@1, EXCHANGE@2. Для replay выбран
CLI@17 и CLI-EXPORT@2 INPUT/BOUNDS, EXPORT@2 → DRAWING-PACKAGE/STYLE/GEOMETRY/
PROMPT-A/PROMPT-B/REVIEW@2, EXAMPLE@1, DRAWING@1.2 HBP-HUMAN-001 R1–R3.
Контекст: 23 routed spec files (1525 lines) плюс выбранный HBP-раздел;
все выбранные узлы и explicit dependencies прочитаны полностью; compare-specific
G12/G13, actions, storage, live acquisition, UI, builds и source приложений исключены.
Packet начинался с registry37; прочитан текущий registry38/CLI17. Его добавление
focused popup не меняет этот closure арифметики и не приписывается frozen binary.
Глобальные implementation, product-truth core/routing/evidence/coordination/delivery и QA
прочитаны; single-chat finite execution. Это Discover/verification, без Contract Delta.

Контракт требует сохранять исходные quantities, bindings, units, frame kind и
unknown, считать через engine и не выводить намерение из повторения. Наблюдение —
исходные JSON ниже. Выбор сопоставимых семейств и новых цепочек A11/B08 — явно
редакционный выбор CD1; он не создаёт component mapping или Expectation.
Ни для одного примера не доказано авторское нормативное правило. Возможная
адаптивная сетка/округление — гипотеза, отдельно от измеренного факта.

## Двенадцать полезных примеров и контрпримеров

`aN` — нулевой индекс `metadata.design_analysis[N]`; полный query/evaluation
содержится в оригинальном metadata. W/H — width/height указанного frame; gap —
край первого → край следующего, а не расстояние между glyphs. Все равенства ниже
относятся к сохранённому наблюдению. Знак ≈ не означает разрешённый допуск.

| № | Семейство и запрос | Точный факт / полезная формула | Что сообщать человеку; от чего воздержаться |
| --- | --- | --- | --- |
| 1 | Mac C01, a0; AX табы 28–32 одного parent20 | `H1 = … = H5 = 42.5 pt`, residual0. Их W: `155.5, 93, 83, 83.5, 85 pt`. | «Пять табов одинаковой высоты; ширины различаются». Равная высота не означает равную ширину, одинаковые glyphs или требование унификации. |
| 2 | Mac C01 a2/a3; C03 a2/a3, каждый внутри своего Observation | C01: `155.5+93+83+83.5+85 + 4×5 = 520 pt`. C03: `177.5+115+105.5+107 + 3×5 = 520 pt`. Оба residual0. | «Выбранные табы вместе с промежутками заполняют ширину панели». В C03 четыре таба, в C01 пять; это две отдельные размерные цепочки, не identity-matched diff. Округлённая первая цепочка даёт 521 вместо520: не печатать её как точное равенство. |
| 3 | Mac A11-grid-narrow a0 + CD1 grid balance | `W=[205.5,206,205.5] pt`; majority205.5, support2/3, exception index1 `+0.5 pt`. Между карточками `12,12 pt`; `205.5+12+206+12+205.5 = 641 pt = W(parent56)`. | «Средняя AX-карточка шире на0.5pt; вся строка точно заполняет641pt». Уравнивание до205.5 при прежних gaps оставило бы0.5pt. Не называть эту разницу дефектом/доказанным pixel rounding; не скрывать её одинаковыми округлёнными206. |
| 4 | Web CW13 a0/a1, Year from/to и parent n28 | `87.59375 = 87.59375 CSS px`; `87.59375+4.515625+87.59375 = 179.703125 CSS px`, residual0. | «Два поля равной ширины и один измеренный промежуток полностью занимают ряд». `≈88+≈5+≈88` не является точной суммой `≈180`. Это layout, не доказательство CSS padding/token. |
| 5 | Responsive Web CW11-stable a0 и CW12 a0, разные семейства navigation | Шесть corpus rows: `H=62.7890625 CSS px`; три legal rows: `H=56 CSS px`. В каждой группе no outlier. | Показывать отдельный типовой размер каждой группы. Разные content families не дают единого «mobile row height». Ни CSS px→pt, ни перенос этих величин на native menus не обоснованы. |
| 6 | Web CW19 a0/a1, inactive genre chips одного popup | Семь `86.84375` и семь `86.8515625 CSS px`: tie, candidate absent. Шесть vertical gaps первой выбранной колонки `4.515625 CSS px`. | «Ширины образуют две равные по численности группы; gaps повторяются». Не выбирать моду при7:7 и не объединять две точные величины потому, что обе отображаются≈87. Причина разницы0.0078125 неизвестна. |
| 7 | iPad Simulator B06 a0/a1, четыре filter pop-up controls | Четыре `W=370 pt`, residual0; pattern support4, no exceptions. | «Четыре выбранных AX-контрола равны по ширине». Это одно наблюдение и одна повторно использованная выборка в equal и pattern, не восемь независимых подтверждений; border/hit shape неизвестны. |
| 8 | iPad Simulator B08 a1 + CD1 label step | Семь edge gaps `24.5 pt` между восемью AX label frames. Первый `H=20.5 pt`; `20.5+24.5=45 pt` до следующего top edge, residual0. | «Подписи следуют с шагом45pt; AX-высота подписи20.5pt». Не рисовать control/hit box высотой20.5 или45, не называть24.5 padding. Рамки раскладки не собраны. |
| 9 | Mac A06 a0 против B08; дополнительный отрицательный пример A05 a0 | Три Mac preference-menu AX heights `28 pt`. B08 label frames20.5pt относятся к другой платформе/геометрическому семейству. A05 Grammar controls `W=209.5,315.5 pt`, spread106. | Одинаковый текст «menu»/«Grammar» не делает размеры сопоставимой нормой. Сообщить локальное повторение28 и отдельно неодинаковые ширины A05; не объявлять нарушение равенства без expected_from. |
| 10 | Media: CW16 a0; B13 a1; A12-viewer a0 | Web layout `1280/720=1.7777777777777777` (16:9). iPhone AX Play `354/199=1.778894472361809` (≈1.779). Mac AX Play `1920/998=1.9238476953907815` (≈1.924). | Назвать объект, frame и величины. B13 близок к16:9, но точное равенство не установлено; это не conformance check. Mac Play — AX-плоскость, не размер круглой кнопки/glyph и не доказательство media pixel aspect. Между платформами деление/нормализация не выполнялись. |
| 11 | Web CW10-stable a0, первые два Tour card layout bounds | `180.57142639160156 / 180.5714111328125 = 1.0000000845027956`; ширины отличаются приблизительно на0.0000152587890625 CSS px. | «Почти одинаковые наблюдённые ширины; отношение≈1». Вывод «строго равны» из двух подписей≈181 неверен. Нет sourced tolerance, поэтому это не pass/fail и не запрос исправить маленькую разницу. |
| 12 | iPad Simulator B07, CD1 Width для exact SourceKey `xcode.rs1:e398` (Learn) | Engine: `unknown_property`, exit4, evidence `B07-xcode-import`; девять sidebar semantic rows не имеют AX bounds. | «Ширина Learn неизвестна». Нельзя использовать рамку sidebar/background, имя похожего элемента или pixels для заполнения пропуска и затем строить pattern. Unknown не zero и не failure требования. |

Дополнительная граница точности: B13 a0 даёт четыре action AX widths
`82.66666666666666, 82.33333333333334, 82.66666666666663, 82.33333333333331 pt`,
spread `0.3333333333333428 pt`. Это полезный контрпример ложной точной унификации;
не превращать эти AX-рамки, включающие подписи, в размеры нарисованных кругов.
C01/C03 a1 даёт `520/1920 = 0.2708333333333333` для panel/window AX widths.
Можно сказать «около27.1% ширины этого окна», но не «адаптивное правило27.1%»:
из одного размера окна зависимость от resize не восстанавливается.

### Дробная сетка: что стало известно и что осталось гипотезой

В A11 source прямо есть три соседних `clipSearch.card.0/1/2` у parent56:
`(x,y,w,h)=(304,98,205.5,115.5)`, `(521.5,98,206,115.5)`,
`(739.5,98,205.5,115.5)`; parent56 `(304,98,641,1903.5)`.
Это одна строка/Surface/Observation и общий parent; здесь сопоставимость сильнее,
чем просто сходство имён. CD1 новые queries подтверждают два12pt gaps и сумму641.

[Wave summary](corpus-wave1-summary.md) передаёт CN3 source-reading evidence:
на pin c7d8e3d adaptive GridItem minimum200, spacing12, inset16; точный алгоритм
pixel rounding там не задан. В CD1 исходники приложения не перечитывались и это
не повысилось до авторского правила. Из текущих AX-данных не следует внешний
inset16, display scale, соответствие layout/paint или причина распределения0.5pt.
Возможная remainder distribution остаётся гипотезой. Полезное объяснение сначала
показывает заполненную размерную цепочку, затем точную разницу; majority2/3 сама
по себе не является основанием переделать среднюю карточку.

## Provenance и воспроизводимость

Общий корень `T=/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T`.

| Alias | Относительный корень | Источник и ограничения |
| --- | --- | --- |
| W1 | `uib-cw-b1-gR3CYr` | DOM/CSSOM reference import; checkout c23ef963, served bundle unknown; partial, stored/unverified, non-atomic DOM/image/AX. Original `dom-layout.json`; canonical `snapshot.json`. |
| W2 | `uib-cw-b2-UhvhiK` | DOM/CSSOM reference import; original `source.json.before.nodes`, canonical `snapshot.json`; partial, unverified, non-atomic. Saved build correspondence is evidence, not fresh source acceptance. |
| M1 | `uib-CN-B1-_djmb16k` | c7d8e3d Release development app plus retained project-file change; collector ed6c60d; original `observed.json` ChannelResponse. Partial AX, separate_api_reads. |
| M3 | `uib-CN-B3-3o25e72c` | c7d8e3d Debug fixture-origin plus retained project-file change; collector526f2dc; original `observed.json`. Partial AX, separate_api_reads; no production failure inference. |
| S2 | `playphraseme-cn-b2-00wlqx9r` | Literal `raw-selected.json` → canonical `Bxx-snapshot.json`; B07 additionally `B07-xcode-source.json`. iPhone17e iOS27 and iPadPro13 M5 iPadOS27 Simulator, installed source revision unknown. |

S2 binary identities from terminal handoff: iPhone1.2(6), executable SHA256
`40bad880ef0950b81923abc2fda9d3c5e2fdc7421bb7987c3ce932b318acbfa2`;
iPad1.0(1), `5cfd34349b3ea212eb315d0e725c14f2ef8c3783916629c1f13620327673ea01`.
This is Simulator evidence, not physical devices or a current-checkout build.

Local replay used frozen `T/uib-CP1-2v302h3b/uiblueprint-4d6e0bf`, verified SHA256
`3d77ad518939c59247888a6b2c1641f97cbcf1c9c206c5191b70399751afe7e9` against the
[task registry](../task-registry.md). No rebuild or shipping calculator was added.
Historical exports remain attributed to their original pins; these fresh arithmetic
replays do not relabel old prompts/images as4d6e0bf outputs.

The table below identifies exact original source/metadata files. SHA prefixes are
navigation fingerprints of original source bytes, not replacements for source identity.
For W1 use `analysis-metadata.json`, W2/M1 `metadata.json`, M3 `metadata-v2.json`;
S2 uses `Bxx-metadata.json`. Every original evaluation binds full Snapshot ID,
revision1, Context (including generations/environment/fields), result Space,
`transforms=[]`, `conditions=null`; no restamping or cross-state matching occurred.

| Case path | Source SHA256 prefix | Observation | Frame / result Space / units |
| --- | --- | --- | --- |
| W1/CW10-stable/snapshot.json | `028bea0901aad269` | `CW10-stable-recorded-dom` | layout / cw-b1-reference-viewport / CSS px |
| W1/CW11-stable/snapshot.json | `5be5ebb0cc20517da` | `CW11-stable-recorded-dom` | same Space definition, separate Observation |
| W1/CW12/snapshot.json | `508f48f7220a9187` | `CW12-recorded-dom` | same Space definition, separate Observation |
| W2/CW13/snapshot.json | `929b464ecbc62cde` | `CW13-dom` | layout / CW13-viewport / CSS px |
| W2/CW19/snapshot.json | `096bcff64e5677c9` | `CW19-dom` | layout / CW19-viewport / CSS px |
| W2/CW16/snapshot.json | `31d83215bdd8ec78` | `CW16-dom` | layout / CW16-viewport / CSS px |
| M1/A11-grid-narrow/observed.json | `2300d1ed7977226f` | `2b539175f325499aadd492a9ccc2f6e4-external_semantics` | AX / ax-screen / pt |
| M1/A12-viewer/observed.json | `55d0106e09c7c1b0` | `ef0adc4905094e82a2d99aee29301b7d-external_semantics` | AX / ax-screen / pt |
| M1/A05-grammar/observed.json | `f44a17a8b951e29ab` | `4dfcfcd5d84e463a8054993cc8f7398e-external_semantics` | AX / ax-screen / pt |
| M1/A06-settings/observed.json | `f0272408a6ff70b1` | `be9654a4f0514ff9942d0cd9e23f622f-external_semantics` | AX / ax-screen / pt |
| M3/C01/observed.json | `fa045f34b823d35a4` | `a5d9c0bdf6bc4b61be2cb889afa310d9-external_semantics` | AX / ax-screen / pt |
| M3/C03/observed.json | `08eb673608a81bdb` | `6272fe1e890940569b85c0b8740b43c2-external_semantics` | AX / ax-screen / pt |
| S2/B06-snapshot.json | `f1a409d228e8eef2` | `B06-import` | AX / simulator-screen-points / pt |
| S2/B08-snapshot.json | `0e277b1e2a890a639` | `B08-import` | AX / simulator-screen-points / pt |
| S2/B13-snapshot.json | `1ed774c528f4ce3b` | `B13-import` | AX / simulator-screen-points / pt |
| S2/B07-snapshot.json | `323babe6f342267eb` | `B07-import`, `B07-xcode-import` | AX unknown for e398; selected simulator-screen-points / pt |

All Spaces have top_left origin; Web kind=viewport, AX kind=screen. The same
literal ax-screen or simulator-screen-points ID across files does not authorize
mixing their Target/Surface/environment bindings. Mac AX→PNG transform explicitly
unknown; S2 acquisition intervals are importer records, not exact native collection
times. No reported nonzero uncertainty in selected operand evidence does not prove
zero real-world measurement error. Preserved consistency unknown is not atomicity.

### Minimal commands and queries

A replay of any table case uses its original S/M and a **new system-temp** O:

```sh
"$BIN" imagegen-prompt --snapshot "$S" --metadata "$M" --out "$O" \
  --max-input-bytes 8388608 --max-output-bytes 33554432 \
  --max-components 500 --max-views 30 --components-per-detail 100 --json
```

Inspect `O/scene.json.views[0].design_analysis`: `facts.measurements`,
`facts.queries`, `facts.evaluation`, `facts.result`, status and summary.
This produces six text files, no ImageGen call. Existing source directories are
never an output destination. These explicit diagnostic caps are not new defaults.

For the smallest individual measure, extract an original request's operand and
binding unchanged. Run this with explicit absolute `S`, `M`, `BIN` environment
variables and zero-based `AI`, `QI` (for example C01 AI=0/QI=0):

```python
import json, os, pathlib, subprocess, tempfile
m = json.loads(pathlib.Path(os.environ['M']).read_text())
a = m['design_analysis'][int(os.environ['AI'])]
with tempfile.TemporaryDirectory(prefix='uib-CD1-query-') as temp:
    d = pathlib.Path(temp)  # only these two non-image transient files
    for name, kind, data in [
        ('q', 'geometry_query', a['operands'][int(os.environ['QI'])]),
        ('e', 'evaluation_input', a['evaluation'])]:
        (d/name).write_text(json.dumps({'schema_version':'0.2.0',
            'artifact':{'kind':kind, 'data':data}}))
    subprocess.run([os.environ['BIN'], 'measure', '--snapshot', os.environ['S'],
        '--query', str(d/'q'), '--evaluation', str(d/'e'),
        '--space', a['evaluation']['result_space']['id'],
        '--max-input-bytes', '8388608', '--max-output-bytes', '8388608', '--json'],
        check=True)
```

Selected anchor keys, in original operand order:

- C01 a0: Mac handle28–32 H/y; a2: their W/x, gaps28→29→30→31→32,
  then W parent20. C03: handle27–30; parent19. Scalar fractions0;
  gap anchors x fractions1→0. Prefix is that case's Observation ID plus `-handle-`.
- A11 a0: handles77,80,83 W/x. CD1 balance operands are those three widths,
  Gap77.right→80.left, Gap80.right→83.left, W56. Copy a0 evaluation unchanged;
  every gap uses x fractions1→0, targets both exact existing SourceKeys.
- CW13 a0/a1: n29,n30; gap n29.right→n30.left; total n28 W/x.
  CW11 H/y: dom16,19,22,25,28,31; CW12 H/y: dom16,20,24 (keys include `dom-`).
- CW19 a0: n13–n26 W/x; a1 y gaps n13→15→17→19→21→23→25, fractions1→0.
  CW16: n6 W/x,H/y. CW10: dom-6,dom-15 W/x.
- B06: `serve_sim.ax` keys0.2.2–0.2.5 W/x. B08 a1:0.3→0.5→…→0.17,
  y fractions1→0. CD1 balance copies that evaluation: H(0.3), first original gap,
  then Gap(0.3.top,0.5.top), y fractions0→0. Result20.5+24.5=45.
- B13 a1:0.5 W/x,H/y; A12: handle14 W/x,H/y. A06: handles74,76,78 H/y;
  A05: handles65,69 W/x. All use each case's existing namespace and full binding.
- B07 unknown query: `targets=[{"namespace":"xcode.rs1","key":"e398"}]`,
  operation width, one accessibility_bounds anchor x/fraction0 in existing
  simulator-screen-points/screen/top_left/pt; quantity_kind length, units pt,
  applies_when all null. Evaluation copies B07 Snapshot id/revision/context,
  selects that full Space, transforms empty, conditions null. Wrap q/e as above;
  expect exit4/unknown_property, not `check=True` success.

For new A11/B08 compositions put only the listed queries in a copied metadata's
`relation` with operation `balance`, reviewed factual caption and original
`evaluation`; original Snapshot stays byte-identical. These selections were
actually executed through imagegen-prompt4d6e0bf, not calculated only in prose.

## Проверки, полезные выводы и пределы

Executed16 original-source exports (26 existing analysis requests), all exit0;
110 constituent measure replays, all known/exit0 and exact agreement with exported
amounts. This includes reused operands; it is not110 independent observations.
Bound evaluation, full Space and contributing property evidence were checked.
Independent diagnostic rectangle arithmetic agreed with all110 operands and all
existing equal/ratio/balance compositions; it was a cross-check, not product output
or independent reviewer acceptance.46 distinct selected imported Web/Simulator
rectangles matched raw recorded x/y/w/h exactly. Mac inputs are original collector
ChannelResponses, not rewritten imports. Two extra compositions (9 operands) ran
through the product; one unknown query correctly returned4. Initial optional CW09
lookup used a directory without snapshot.json and returned io_error; it contributed
no result and was excluded, without filling missing input from images.

No original input was edited. No application launched, UI operated, image read or
captured, build run, dependency added or branch changed. Repeated C01/C02/C05 and
C03/C07 fixture-identical PNGs in the handoff are not independent confirmation;
only C01 and C03 are selected here, and neither establishes an adaptive rule.

Consumer guidance, as evidence-backed proposals within the existing contract:

1. Prefer explanations with the useful complete chain and named endpoints:
   «Пять табов одной высоты; их разные ширины и четыре5pt gaps заполняют520pt».
   A tab family is supplied by the caller; the engine does not infer author intent.
2. Preserve a meaningful fractional exception alongside rounded presentation:
   «Ширины205.5/206/205.5pt; разница0.5pt, строка641pt заполнена». The current
   text's ≈206 candidate and ≈1 deviation is valid display rounding but makes the
   mechanism hard to understand. Better explanatory selection is a proposal,
   not permission to change machine values or the current rounding contract.
3. Say «AX-подписи с шагом45pt», «отношение AX-плоскости≈1.924», «layout16:9»
   instead of one generic control-size story. Do not draw or recommend a glyph,
   painted border or hit target from a different frame kind.
4. Abstain from automatic correction for ties, unknown inputs, different states/
   spaces/families, absent expected_from and unsupported causal claims. No
   universal8px grid, golden ratio, inferred design token or statistical confidence
   follows from these selected observations. A strict majority is a disclosed
   selection heuristic; an authored Expectation would require separate authority.

Documentation-only verification: changed local links/route consistency and
`git diff --check`; no Rust build/test claim. CD1 non-image scratch outputs were
removed after summarization; original corpus inputs, packages, images and their
containing directories remain untouched. This receipt is the explicitly requested
persistent deliverable. Remaining gap is author intent/causal evidence, not arithmetic.
