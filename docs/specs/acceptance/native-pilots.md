# Mac пилоты M01–M06

- Node type: leaf; domain: `uib.native-pilots`.
- Contract: `UIB.NATIVE-PILOTS@1`; stable clause: `UIB.NATIVE-PILOTS.CONTENT`.
- Authority: Active / Stability: Evolving; current norms; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: R02 и native fixtures/QA.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.PILOTS@1](pilots.md), [UIB.NATIVE@1](../product/native.md), [UIB.FORMS@1](../product/forms.md), [UIB.CACHE@1](../product/cache.md), [UIB.ACTIONS@1](../product/actions.md).
- Source mapping: TZ 461–473; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

<a id="uib-native-pilots-content"></a>

| Mac пилот | Обязательный результат |
| --- | --- |
| M01 | Два окна одного процесса с одинаковым title: точный выбор, move/resize, закрытие/пересоздание; ни одного cross-window действия |
| M02 | Native форма с текстом, masked secure field, checkbox и completion/IME при поддержке: фактический ввод, focus/selection, подтверждённое изменение и остановка при непредвиденной смене владельца ввода |
| M03 | Отдельный popup у края окна: допустимый system fallback, согласованные AX/capture scopes и capture_kind; закрытие инвалидирует refs |
| M04 | Scroll/clipping и применимые scale contexts: известные transforms, local diff без ложного изменения отступов; неизвестный mapping честно возвращается |
| M05 | Merged SwiftUI control: внешняя design-проекция partial; узкий opt-in layout probe показывает реальные внутренние icon/text/gap и many-to-many mapping |
| M06 | Timeout, unsupported attribute, потерянное notification и permission change: bounded partial/error, revalidation/resync, отсутствие stale action и независимость другой Target-сессии |

M01–M03 включают необходимые pixels и наблюдаемый input outcome, а не только AX-дерево. Если они недоступны, конкретный gate остаётся открытым. Для M04 отсутствие второго дисплея не блокирует обычную проверку, но cross-display квалификация остаётся отдельно не выполненной; её нельзя заявить как принятую.

M05 — обязательное исследование реализуемости главной native design-ценности. Один собственный fixture с узким измерительным probe и без него должен подтвердить неизменность геометрии/фокуса/hit/accessibility. Это не обещание полного SDK для любых приложений в первой поставке. Probe-снимок другой сборки не закрывает production evidence.
