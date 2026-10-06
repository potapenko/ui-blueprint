# Будущие inference extensions и mobile sources

- Node type: leaf; domain: `uib.extensions`.
- Contract: `UIB.EXTENSIONS@1`; stable clause: `UIB.EXTENSIONS.CONTENT`.
- Authority: Active / Stability: Evolving; future, вне P0–P7; accepted/released baseline: none.
- Authority source: UIB.TZ@1.4 / UIB.DRAWING@1.1, user confirmation 2026-10-06; C00 faithful routing only.
- Read when: только отдельно открытые extension/mobile исследования.
- Do not read when: задача не затрагивает этот домен; reference/future узлы не являются общим preload.
- Requires: [UIB.REUSE@1](reuse.md).
- Source mapping: TZ 417–422; TZ 956–963; TZ 968–971; TZ 985–996; [inverse map](../reference/source-map.md); source links are provenance, not requires.
- Precedence: [registry](../README.md); исходные Active нормы при расхождении сохраняют силу; здесь нет новых решений.

Условия источников сохранены как исторический каталог; этот узел не открывает ML/mobile scope P0–P7.

<a id="uib-extensions-content"></a>

Jev от TypeSafe AI — отдельное расширение, выключенное по умолчанию. Пользователь подключает его со своими учётными данными и условиями оплаты. Без него работают наблюдение, вся формальная аналитика, diff, CLI и обычное исполнение.

Jev получает минимальный разрешённый очищенный фрагмент и возвращает inference с происхождением и доступной оценкой уверенности. Он не меняет observed facts и не разрешает действие по вероятному совпадению. При отсутствии ключа, сети или сервиса основной путь продолжает работать; неоднозначность возвращается агенту. Автоматического платного fallback нет.

Другие классификаторы и visual parsers могут появляться через тот же интерфейс extensions. Их наличие в списке аналогов не означает включения в MVP.

### Jev и официальные SDK

[Документация Jev](https://docs.typesafe.ai/introduction), [примитивы вопросов](https://docs.typesafe.ai/primitives). Это опциональный внешний inference, не open-source замена model-free engine. Публичные weights/реализация модели в этом исследовании не подтверждены.

[Официальный Python SDK](https://github.com/typesafe-ai/typesafe-sdk-python), commit `f078f1e208a0d885154dc758344ae4fce77ac168`, [LICENSE с MIT-текстом](https://github.com/typesafe-ai/typesafe-sdk-python/blob/f078f1e208a0d885154dc758344ae4fce77ac168/LICENSE). Читать [question_types.py](https://github.com/typesafe-ai/typesafe-sdk-python/blob/f078f1e208a0d885154dc758344ae4fce77ac168/src/typesafe_sdk/_core/question_types.py), response_types.py, errors.py и retry.py в том же пакете. [Официальный JS SDK](https://github.com/typesafe-ai/typesafe-sdk-js) — дополнительный reference; его commit здесь не закреплён и реализация не проверена.

Взять форму typed request/result, раздельные errors/timeouts и явный retry budget для будущего Rust-клиента. Не переносить авто-retry в GUI actions и не превращать вероятность в разрешение нажать. SDK-лицензия не описывает условия использования сервиса/модели. Fixture: отсутствующий ключ, timeout, unsupported response и отсутствие платного fallback при выключенном плагине.

[Android Semantics](https://developer.android.com/develop/ui/compose/accessibility/semantics), [UI Automator](https://developer.android.com/training/testing/other-components/ui-automator) и [AccessibilityNodeInfo](https://developer.android.com/reference/android/view/accessibility/AccessibilityNodeInfo) — обязательные первичные чтения перед Android-плагином. Точки API: UiDevice/UiObject2, bounds/actions и merged/unmerged semantics. Исходная ревизия AndroidX и лицензия конкретного переносимого файла должны быть закреплены в фазе Android; в этом исследовании проверена документация, а не Android runtime или полный исходник UI Automator.

Для ImageGen и Vision остаются [документация генерации](https://developers.openai.com/api/docs/guides/image-generation) и [документация изображений](https://developers.openai.com/api/docs/guides/images-vision). Это ограничения внешнего канала, а не библиотека, которую нужно портировать. Локальная команда подготовки промпта не зависит от оплаченного API.

### Пиксельные модели и исследовательские прототипы

Эти проекты сохраняются в ТЗ, чтобы не потерять идеи. Они не входят в model-free путь и не становятся обязательными зависимостями или задачей обучения модели.

| Источник и код для чтения | Полезная идея | Ограничение |
| --- | --- | --- |
| [Screen2AX](https://github.com/MacPaw/Screen2AX): [hierarchy pipeline](https://github.com/MacPaw/Screen2AX/blob/402e7c4137e6bd76cc80558e24b195102067ae6f/hierarchy_dl/hierarchy.py), [heuristics](https://github.com/MacPaw/Screen2AX/tree/402e7c4137e6bd76cc80558e24b195102067ae6f/hierarchy_heuristics), [paper](https://arxiv.org/abs/2507.16704) | Восстановление групп и иерархии из пикселей, разделение detection/grouping и представления результата. Изучить как источник optional estimated relations и тестовых трудных случаев Mac. | Предсказанная иерархия не AX observation и не layout measurement. LICENSE начинается с MIT для кода и содержит дополнительные уведомления; README и LICENSE требуют отдельной сверки условий BLIP/YOLO/данных. Не обобщать лицензию кода на веса; не скачивать веса для базовой реализации. |
| [OmniParser](https://github.com/microsoft/OmniParser): [parser](https://github.com/microsoft/OmniParser/blob/354021201345a96178360b28733573e27269f2de/util/omniparser.py), [OCR и обработка регионов](https://github.com/microsoft/OmniParser/blob/354021201345a96178360b28733573e27269f2de/util/utils.py) | Преобразование скриншота в регионы с текстом/описанием; полезные промежуточные результаты для fallback и region selection. | Данные estimated, геометрия и семантика не становятся достоверными платформенными фактами. Корневой LICENSE на указанной ревизии — CC-BY-4.0; условия весов и зависимостей проверяются отдельно. Встроенный OCR/ML не обязателен. |
| [UI-TARS](https://github.com/bytedance/UI-TARS): [action parser](https://github.com/bytedance/UI-TARS/blob/582f3a7ea5d285ee8ed9e2e84048d1ab01453c49/codes/ui_tars/action_parser.py), [parser tests](https://github.com/bytedance/UI-TARS/blob/582f3a7ea5d285ee8ed9e2e84048d1ab01453c49/codes/tests/action_parser_test.py), [coordinate notes](https://github.com/bytedance/UI-TARS/blob/582f3a7ea5d285ee8ed9e2e84048d1ab01453c49/README_coordinates.md) | Явное преобразование координат, разбор предсказанного действия, тесты и отделение формата ответа от исполнения. | Изучить интерфейс и failure cases, не перенести генерацию/исполнение Python-кода из модельной строки в доверенный action engine. Модель не нужна для штатного пути. Root code license — Apache-2.0; веса отдельны. |
| [UI-TARS Desktop и Agent TARS](https://github.com/bytedance/UI-TARS-desktop): [browser operator](https://github.com/bytedance/UI-TARS-desktop/blob/2ff41a9e515828c5bd5b276e493d73aa0bdf4a3a/packages/ui-tars/operators/browser-operator/src/browser-operator.ts), [ADB operator](https://github.com/bytedance/UI-TARS-desktop/blob/2ff41a9e515828c5bd5b276e493d73aa0bdf4a3a/packages/ui-tars/operators/adb/src/index.ts) | Разные operator backends под общим интерфейсом, различие remote/browser/device и маршрутизация действий. | Не включать весь multimodal agent stack или remote service. Политики авторизации и freshness проектируем по этому ТЗ. Корневой LICENSE — Apache-2.0; дочерние пакеты проверяются отдельно. |
| [UI-Vision](https://arxiv.org/abs/2503.15661) и [Screen Parsing](https://arxiv.org/abs/2109.08763) | Исследовательские постановки element/layout grounding и группировки элементов; источник идей для сложных контрольных сцен и ограничений визуального inference. | Это литература/benchmark-направления, не проверенные готовые адаптеры. Лицензии конкретных данных/кода и пригодность корпуса проверять перед использованием; не обещать их метрики нашему инструменту. |
