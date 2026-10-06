#!/bin/bash
set -euo pipefail
# All generated products go to an explicitly supplied task-temp directory.
base=$(cd "$(dirname "$0")/.." && pwd)
output=${1:?Usage: build.sh /absolute/task-temp}
[[ "$output" = /* ]] || exit 2
mkdir -p "$output"
for mode in off on; do
  bundle="$output/F02-$mode.app"
  mkdir -p "$bundle/Contents/MacOS"
  flags=(-D F02_DIAGNOSTIC)
  if [[ "$mode" = on ]]; then flags+=(-D PROBE); fi
  xcrun swiftc -parse-as-library -swift-version 6 -target arm64-apple-macos14.0 "${flags[@]}" "$base/Fixture.swift" -o "$bundle/Contents/MacOS/F02Fixture"
  /usr/bin/plutil -create xml1 "$bundle/Contents/Info.plist"
  for entry in "CFBundleExecutable:F02Fixture" "CFBundleIdentifier:local.uiblueprint.f02.$mode" "CFBundleName:F02Fixture" "CFBundlePackageType:APPL" "LSMinimumSystemVersion:14.0" "NSPrincipalClass:NSApplication"; do
    /usr/bin/plutil -insert "${entry%%:*}" -string "${entry#*:}" "$bundle/Contents/Info.plist"
  done
 done
xcrun swiftc -parse-as-library -swift-version 6 -target arm64-apple-macos14.0 "$base/Observe.swift" -o "$output/f02-observe"
