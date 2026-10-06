#!/usr/bin/env python3
"""Build only the committed common host and validator in task-temp."""
import argparse
import io
import json
import pathlib
import subprocess
import tarfile

SUPPORT = '73d772e97efcf550ea4a4d3e8480b56509ebc548'
repo = pathlib.Path(__file__).resolve().parents[3]
parser = argparse.ArgumentParser()
parser.add_argument('output', type=pathlib.Path)
a = parser.parse_args()
assert a.output.is_absolute() and not a.output.resolve().is_relative_to(repo)
a.output.mkdir(parents=True, exist_ok=False)
subprocess.run(['git', 'merge-base', '--is-ancestor', SUPPORT, 'HEAD'], cwd=repo, check=True, timeout=10)
archive = subprocess.check_output(['git', 'archive', SUPPORT, 'Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml',
                                  'crates/schema', 'crates/plugin-api', 'crates/engine', 'tests/bridges/common'],
                                 cwd=repo, timeout=15)
source = a.output / 'source'
source.mkdir()
with tarfile.open(fileobj=io.BytesIO(archive)) as tar:
    tar.extractall(source, filter='data')
with (a.output / 'build.log').open('wb') as log:
    subprocess.run(['cargo', '+1.96.0', 'build', '--locked', '--offline', '-p', 'uiblueprint-plugin-api',
                    '--example', 'd02_host', '-p', 'uiblueprint-schema', '--bin', 'uiblueprint-validate',
                    '--target-dir', str(a.output / 'target')], cwd=source,
                   stdout=log, stderr=subprocess.STDOUT, check=True, timeout=180)
report = {'support_revision': SUPPORT, 'source_archive_revision': SUPPORT,
          'host': str(a.output / 'target/debug/examples/d02_host'),
          'validator': str(a.output / 'target/debug/uiblueprint-validate'), 'runtime_launched': False}
(a.output / 'build.json').write_text(json.dumps(report, indent=2)+'\n')
print(json.dumps(report))
