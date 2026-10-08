# Independent source review: export, distribution, popup geometry

Свежий read-only чат01a11b7c-f321-70e2-b0ca-91f161b5d340. Initial turn
01a11b7c-f5e9-76c1-a003-752dc1225903 completed до author receipts/finals/verdicts.
Scope:3075239 distribution,c97c513 E03,b43d0df B03,5fd4b6a M03 vs individual
parents и нужные saved callers. Текущие Native form/W04/WIP исключены.
Build/tests/runtime не выполнялись, файлы/index не менялись. Для Native API
reviewer сверил локальные public SDK headers.

## Initial observations

Actionable introduced findings не обнаружены. Проверены:
- Distribution: modules/features, matching binaries из одного pin, notices,
  fixed inventory, exclusive publication, verify/remove/foreign-file preservation.
- E03: существующие engine comparators, immutable inputs, allowlist/aliases,
  declared scope и separate content/evidence/unknown/partial, Space binding,
  input/output bounds и legacy exports.
- B03: exact single-point identity, clipping отдельно от visibility/occlusion,
  document/Surface attribution, final connectivity и bounded observer lifetime;
  unconfirmed remote cleanup после transport loss не скрывается.
- M03: public window/filter/scale metadata, Target/Surface/environment binding,
  changed metadata refusal, separate clocks/partial, PNG dimensions только
  проверяют mapping; retained staging/final images не удаляются.

## Reconciliation pending

После initial observations тому же reviewer переданы четыре соответствующих
receipts. Особые limits: I01 tested product82342f0; E03 old historical baseline
failure на исходномe4bc256; B03 shared-CLI-WIP runtime не final integrated pin;
M03 cold1062.46ms incomplete при1s, D06 не passed, финальный defensive guard
проверен focused source tests, standalone composition consumer не shipping join.
Final verdict ещё не получен. Review относится к source/safety scope, не full
P7/pilots/performance. Author runtime не становится independently rerun.
