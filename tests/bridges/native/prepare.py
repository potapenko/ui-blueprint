#!/usr/bin/env python3
"""Compile immutable F02 input only; never launch UI or use uncommitted Rust."""
import argparse
import hashlib
import json
import pathlib
import subprocess
import tempfile

FIXTURE_REVISION = '9a88b12b5855bac54bf04ba7b64d233df719ddec'
INPUTS = ('Fixture.swift', 'Observe.swift', 'script/build.sh')
repo = pathlib.Path(__file__).resolve().parents[3]
parser = argparse.ArgumentParser()
parser.add_argument('--output', type=pathlib.Path, help='New absolute task-temp directory outside the repository')
args = parser.parse_args()
if args.output:
    output = args.output
    temp_root = pathlib.Path(tempfile.gettempdir()).resolve()
    if (not output.is_absolute() or output.resolve().is_relative_to(repo)
            or not output.resolve().is_relative_to(temp_root) or output.resolve() == temp_root):
        parser.error('output must be a new directory under the system task-temp root, outside the repository')
    output.mkdir(parents=True, exist_ok=False)
else:
    output = pathlib.Path(tempfile.mkdtemp(prefix='uib-s01-native-'))

def run(command, timeout=15):
    return subprocess.check_output(command, cwd=repo, timeout=timeout)

revision = run(['git', 'rev-parse', f'{FIXTURE_REVISION}^{{commit}}']).decode().strip()
assert revision == FIXTURE_REVISION
sources = output / 'source'
hashes = {}
for relative in INPUTS:
    source = run(['git', 'show', f'{revision}:fixtures/native/{relative}'])
    path = sources / relative
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(source)
    hashes[f'fixtures/native/{relative}'] = hashlib.sha256(source).hexdigest()
log = output / 'compile.log'
with log.open('wb') as stream:
    completed = subprocess.run(['bash', str(sources / 'script/build.sh'), str(output / 'build')],
                               cwd=repo, stdout=stream, stderr=subprocess.STDOUT, timeout=120)
report = {'phase': 'fixture_build_preparation_only', 'fixture_revision': revision,
          'source_sha256': hashes, 'swift': run(['xcrun', 'swift', '--version']).decode().strip(),
          'sdk_version': run(['xcrun', '--show-sdk-version']).decode().strip(),
          'os': run(['sw_vers']).decode().strip(), 'compile_exit': completed.returncode,
          'output': str(output), 'runtime_launched': False, 'rust_baseline_consumed': False,
          'stage_a_revision': None, 'common_interface_proof': 'waiting_for_committed_stage_a_and_runtime_grant'}
if completed.returncode == 0:
    products = ['F02-off.app/Contents/MacOS/F02Fixture', 'F02-on.app/Contents/MacOS/F02Fixture', 'f02-observe']
    report['binary_sha256'] = {name: hashlib.sha256((output / 'build' / name).read_bytes()).hexdigest()
                              for name in products}
(output / 'preparation.json').write_text(json.dumps(report, indent=2) + '\n')
print(json.dumps({'compile_exit': completed.returncode, 'receipt': str(output / 'preparation.json'),
                  'runtime_launched': False, 'stage_a_revision': None}))
raise SystemExit(completed.returncode)
