#!/usr/bin/env python3
"""Build the pinned explicit-request F02 + identity-only revision. Never launch it."""
import argparse
import hashlib
import json
from pathlib import Path
import plistlib
import subprocess
import tempfile

REVISION = '53e6e6ef0d8a291c92c340fe5ed217ccbfd965cb'
SOURCE_SHA256 = '662c92db9fbf0b131e03c022053fc68ef8ab4126d39989da48b49da39f8c238d'
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('output', type=Path, help='new directory under system temp; retained for Q01/Q02')
args = parser.parse_args()
root = Path(__file__).resolve().parents[3]
output = args.output.resolve()
assert output.is_relative_to(Path(tempfile.gettempdir()).resolve()) and not output.exists()
source = subprocess.check_output(['git', 'show', REVISION + ':fixtures/native/Fixture.swift'], cwd=root)
assert hashlib.sha256(source).hexdigest() == SOURCE_SHA256
output.mkdir(mode=0o700)
source_path = output / 'Fixture.swift'
source_path.write_bytes(source)
report = {'fixture_revision': REVISION, 'fixture_source_sha256': SOURCE_SHA256,
          'kind': 'historical_explicit_request_fixture_with_identity_only_invalidation',
          'runtime_launched': False, 'live_comparability': 'pending', 'binaries': {},
          'retention': 'Q01/Q02 benchmark handoff; remove own nonimages after consumption; never delete images'}
for mode in ('off', 'on'):
    bundle = output / ('F02-' + mode + '.app')
    executable = bundle / 'Contents/MacOS/F02Fixture'
    executable.parent.mkdir(parents=True)
    flags = ['-D', 'PROBE'] if mode == 'on' else []
    subprocess.run(['xcrun', 'swiftc', '-parse-as-library', '-swift-version', '6',
                    '-target', 'arm64-apple-macos14.0', '-module-cache-path', str(output / 'module-cache'),
                    '-D', 'F02_DIAGNOSTIC', *flags,
                    str(source_path), '-o', str(executable)], check=True, timeout=120)
    info = {'CFBundleExecutable': 'F02Fixture', 'CFBundleIdentifier': 'local.uiblueprint.f02.' + mode,
            'CFBundleName': 'F02Fixture', 'CFBundlePackageType': 'APPL',
            'LSMinimumSystemVersion': '14.0', 'NSPrincipalClass': 'NSApplication'}
    (bundle / 'Contents/Info.plist').write_bytes(plistlib.dumps(info))
    report['binaries'][mode] = {'path': str(executable), 'sha256': hashlib.sha256(executable.read_bytes()).hexdigest()}
(output / 'handoff.json').write_text(json.dumps(report, indent=2) + '\n')
print(json.dumps(report, indent=2))
