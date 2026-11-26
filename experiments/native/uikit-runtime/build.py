#!/usr/bin/env python3
"""Compile the owned diagnostic fixture; explicit external build and internal app roots."""
import argparse
import os
from pathlib import Path
import plistlib
import subprocess

p = argparse.ArgumentParser()
p.add_argument('--build-root', required=True)
p.add_argument('--app-root', required=True)
a = p.parse_args()
repo = Path(__file__).resolve().parents[3]
build, run = Path(a.build_root).resolve(), Path(a.app_root).resolve()
assert build.is_dir() and run.is_dir()
assert run.stat().st_dev == Path('/private/tmp').stat().st_dev
sdk = subprocess.check_output(['xcrun', '--sdk', 'iphonesimulator', '--show-sdk-path'], text=True).strip()
env = dict(os.environ, TMPDIR=str(build), TMP=str(build), TEMP=str(build))
module = repo / 'plugins/swiftui-runtime/Sources/RuntimeUpdateScope'
subprocess.run(['xcrun', '--sdk', 'iphonesimulator', 'clang', '-target', 'arm64-apple-ios17.0-simulator', '-isysroot', sdk,
    '-DUIBLUEPRINT_RUNTIME_DEBUG', '-I', str(module / 'include'), '-c', str(module / 'RuntimeUpdateScope.c'),
    '-o', str(build / 'scope.o')], check=True, env=env)
app = run / 'UIKitRuntimeFixture.app'
app.mkdir(exist_ok=True)
sources = sorted((repo / 'plugins/swiftui-runtime/Sources/UIBlueprintSwiftUIRuntime').glob('*.swift'))
subprocess.run(['xcrun', 'swiftc', '-target', 'arm64-apple-ios17.0-simulator', '-sdk', sdk,
    '-D', 'DEBUG', '-parse-as-library', '-module-name', 'UIKitRuntimeFixture',
    '-module-cache-path', str(build / 'modules'), '-I', str(module / 'include'),
    *map(str, sources), str(Path(__file__).with_name('Fixture.swift')), str(build / 'scope.o'),
    '-o', str(app / 'UIKitRuntimeFixture')], check=True, env=env)
(app / 'Info.plist').write_bytes(plistlib.dumps(dict(CFBundleIdentifier='me.uiblueprint.uikitfixture',
    CFBundleName='UIKitRuntimeFixture', CFBundleExecutable='UIKitRuntimeFixture', CFBundlePackageType='APPL',
    CFBundleVersion='1', CFBundleShortVersionString='1.0', MinimumOSVersion='17.0',
    UIDeviceFamily=[1], UILaunchScreen={}, UIApplicationSceneManifest={'UIApplicationSupportsMultipleScenes':False})))
subprocess.run(['codesign', '--force', '--sign', '-', str(app)], check=True, env=env)
print(app)
