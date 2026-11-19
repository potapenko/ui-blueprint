# Воспроизведение AX-измерений Grammar и Library на другом Mac

Владелец окна PlayPhrase.me запускает отдельную программу, собранную из исходников UI Blueprint. Она возвращает `accessibility_bounds` в порядке `[x, y, width, height]`, в экранных точках AX с началом сверху слева. Положение SwiftUI layout, область мышиного клика и преобразование в пиксели остаются неизвестными. Сверка до и после измерения обозначается `revalidated_nonatomic`.

## Сборка

Нужна копия Git-репозитория, содержащая commit `f6e0418897e200fe4d2a4886e5d8febe7e8231cd`. Из каталога этой копии выполните блок в одном сеансе терминала. Он извлекает четыре файла зафиксированной ревизии во временный каталог. Для сборки нужны Xcode Command Line Tools или Xcode с macOS SDK, AppKit и ApplicationServices. Проверенный профиль — Apple Silicon, macOS 27.0.1, Apple Swift 6.4.2, macOS SDK 27.2. На новом Mac запишите локальные версии и проверьте fixture. Программе нужна уже разрешённая Accessibility-сессия. Сборка независима от установленного UI Blueprint 0.5.1.

```sh
repo=$(git rev-parse --show-toplevel)
pin=f6e0418897e200fe4d2a4886e5d8febe7e8231cd
work=$(mktemp -d "${TMPDIR:-/tmp}/uib-owner-ax.XXXXXXXX")
git -C "$repo" archive -o "$work/source.tar" "$pin" -- plugins/macos/NativeAcquisition.swift \
  tests/bridges/native/GrammarAXBounds.swift tests/bridges/native/GrammarAXFixture.swift \
  tests/bridges/native/LibraryAXFixture.swift &&
tar -xf "$work/source.tar" -C "$work" && cd "$work" &&
xcrun --sdk macosx swiftc --version && xcrun --sdk macosx --show-sdk-version &&
xcrun --sdk macosx swiftc -parse-as-library plugins/macos/NativeAcquisition.swift \
  tests/bridges/native/GrammarAXBounds.swift -o "$work/read-bounds" &&
shasum -a 256 "$work/read-bounds"
```

Сохраните хеш своей сборки с результатом прогона. Он относится к этому бинарнику и локальному toolchain.

## Проверка на собственной fixture

Следующие команды запускают отдельное тестовое окно AppKit. Две разные AX-группы содержат одинаково названные кнопки, а слот и Import используют общий AXIdentifier. Проверка сравнивает оба результата с `accessibilityFrame()`, независимо записанными самим приложением. Требуются Python 3 и доступ к графической сессии.

```sh
app="$work/LibraryFixture.app"
mkdir -p "$app/Contents/MacOS" &&
xcrun --sdk macosx swiftc -parse-as-library tests/bridges/native/LibraryAXFixture.swift \
  -o "$app/Contents/MacOS/Fixture" &&
plutil -create xml1 "$app/Contents/Info.plist" &&
plutil -insert CFBundleExecutable -string Fixture "$app/Contents/Info.plist" &&
plutil -insert CFBundlePackageType -string APPL "$app/Contents/Info.plist" &&
plutil -insert CFBundleName -string LibraryFixture "$app/Contents/Info.plist" &&
plutil -insert CFBundleIdentifier -string "me.uiblueprint.external-layout.canary.library.$(uuidgen)" "$app/Contents/Info.plist" &&
codesign -s - -f "$app" &&
open -g -n "$app" --args unique "$work/baseline.json"
```

```sh
python3 - "$work" <<'PY'
import json, os, signal, subprocess, sys, time
from pathlib import Path
d = Path(sys.argv[1]).resolve()
baseline = d / 'baseline.json'
for _ in range(100):
    if baseline.exists():
        break
    time.sleep(0.1)
s = json.loads(baseline.read_text())
argv = [str(s[k]) for k in ('pid', 'bundle', 'birth', 'cg', 'path')]
x, y, _, _ = s['expected']
try:
    for mode, key in [('library-empty-slot', 'expected'), ('library-import', 'expected_import')]:
        command = [d/'read-bounds', mode, *argv, str(x + 160), str(y + 6)]
        result = json.loads(subprocess.check_output(command))
        assert result['rect'] == s[key] and result['scope_ax_role'] == 'AXGroup'
        print(mode, result['rect'])
    assert s['expected'] != s['expected_import']
finally:
    live = subprocess.run(['ps', '-p', str(s['pid']), '-o', 'comm='], capture_output=True, text=True)
    if live.returncode == 0 and Path(live.stdout.strip()).resolve() == d/'LibraryFixture.app/Contents/MacOS/Fixture':
        os.kill(s['pid'], signal.SIGTERM)
PY
```

После проверки сохраняйте `read-bounds` до завершения замеров. Временные исходники, fixture и её результаты можно удалить после приёмки, когда ими больше не пользуются.

## Измерение в окне владельца

Владелец непосредственно перед запуском подтверждает состояние страницы, PID, bundle ID и абсолютный путь выбранной копии приложения. Для текущих launch time и CGWindowID используйте команду из [установленного native-рецепта](../../skills/ui-blueprint/references/macos.md#independent-native-png-from-the-selected-window), выбрав нужное видимое окно по собственному наблюдению.

```sh
"$UIB_BUNDLE/uiblueprint" capture-window --list-windows --pid "$APP_PID" --expected-bundle "$APP_BUNDLE_ID"
```

В `BIRTH` перенесите исходную числовую величину `process.launch_time` целиком. В `CG_ID` укажите актуальный `window_id`, в `APP_PATH` — абсолютный путь пакета .app. В `X` и `Y` укажите подтверждённую точку текущего окна в `ax-screen`, в pt сверху слева. Для первых трёх семантических измерений точка выбирает область. Для Statistics она выбирает саму кнопку. Смена процесса, окна, монитора, размера, прокрутки или состояния требует свежей привязки.

| Цель в текущем состоянии | Аргумент режима | Что подтверждается |
|---|---|---|
| Grammar Home, Present Simple | режим опустить | Единственный AXButton с AXDescription `Present Simple` внутри конкретной AXScrollArea `searchLearn.learner.grammar` |
| Favorites, проверка контейнера | `library-slot-overlay-role` | AXRole единственного предка с AXIdentifier `searchLearn.learner.overlay` на пути от точки к выбранному окну |
| Favorites, первый пустой слот | `library-empty-slot` | Единственный AXButton с AXDescription `Favorites collection 1, empty` внутри подтверждённого overlay |
| Favorites, Import | `library-import` | Единственный AXButton с AXDescription `Import` внутри того же overlay |
| Statistics, кнопка под визуально подтверждённой точкой внизу страницы | `library-statistics-witness` | Непосредственный AX hit возвращает enabled AXButton с содержащей точку рамкой, внутри AXScrollArea `searchLearn.learner.statistics`. Принадлежность к PDF засвидетельствована владельцем и отмечена `owner_visual_witness`. Имя кнопки не считывается. |

Выполните одну команду для выбранного состояния. Первая измеряет Present Simple в Grammar Home. Вторая приведена для Import. Для другого Library-измерения замените только режим в этой строке, предварительно обновив параметры цели.

```sh
"$work/read-bounds" "$APP_PID" "$APP_BUNDLE_ID" "$BIRTH" "$CG_ID" "$APP_PATH" "$X" "$Y"
"$work/read-bounds" library-import "$APP_PID" "$APP_BUNDLE_ID" "$BIRTH" "$CG_ID" "$APP_PATH" "$X" "$Y"
```

Успех возвращает одну JSON-строку. При отказе программа завершится с кодом 4, пустым stdout и причиной в stderr. Сохраните причину. Например, `ambiguous_button` означает два совпадения в области, `cg_window_frame_ambiguous` — несколько видимых окон одного процесса с одинаковой рамкой. При `ax_children_over_capacity` уникальность внутри области не установлена. Поиск ограничен 160 записями на проход с учётом цепочки предков и общим временем 10 с. У программы нет режима выбора по индексу, подстановки ближайшей кнопки или увеличения квоты.
