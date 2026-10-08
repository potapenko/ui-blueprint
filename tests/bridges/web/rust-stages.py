#!/usr/bin/env python3
"""Q02 exact-owner offline Rust stage replay. Products and instrumentation stay in system temp."""
import argparse
import difflib
import hashlib
import io
import json
import math
import os
from pathlib import Path
import statistics
import subprocess
import tarfile
import tempfile
import uuid

ROOT = Path(__file__).resolve().parents[3]
PIN = 'a7c04164df08441cfbbaa61b501aa64d29290732'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def invoke(command, *, env=None, timeout=180):
    result = subprocess.run(command, cwd=ROOT, env=env, capture_output=True, timeout=timeout)
    if result.returncode:
        raise RuntimeError(f'command exited {result.returncode}: ' + result.stderr.decode(errors='replace')[-4000:])
    return result.stdout.decode()


def prepare(args):
    out = args.output.resolve()
    assert out.is_relative_to(Path(tempfile.gettempdir()).resolve())
    out.mkdir(mode=0o700)
    source = out / 'source'
    source.mkdir()
    with tarfile.open(fileobj=io.BytesIO(subprocess.check_output(['git', 'archive', PIN], cwd=ROOT))) as archive:
        archive.extractall(source, filter='data')
    inputs = out / 'inputs'
    env = dict(os.environ, UIB_Q02_STAGE_INPUTS=str(inputs), UIB_Q02_STAGE_SOURCE=str(source),
               UIB_Q02_STAGE_PINS=str(args.pins.resolve()))
    text = invoke(['node', str(ROOT/'tests/bridges/web/rust-stages-inputs.cjs'), '--run-authorized'], env=env, timeout=60)
    print(text.strip())
    sample = json.loads(args.full_sample.read_text())
    canonical = sample['frames'][0]['canonical']
    response = json.loads(canonical)['artifact']['data']
    snapshot = response['result']['data']
    request = {'schema_version': '0.1.0', 'artifact': {'kind': 'request', 'data': {
        'clock_domain': snapshot['observations'][0]['clock_domain'], 'request_id': response['request_id'],
        'context': snapshot['context'], 'limits': {'max_elements': 128, 'max_depth': 16, 'max_output_bytes': 524288, 'deadline_ms': 2000},
        'freshness_policy': 'current_required', 'operation': {'operation': 'observe', 'channels': ['external_semantics']}}}}
    (inputs/'documents-canonical.json').write_text(canonical)
    (inputs/'documents-request.json').write_text(json.dumps(request))
    (inputs/'documents-input.json').write_bytes(args.full_raw.read_bytes())
    changes = []

    def append(relative, addition):
        file = source/relative
        old = file.read_text()
        new = old + '\n' + addition + '\n'
        file.write_text(new)
        changes.extend(difflib.unified_diff(old.splitlines(True), new.splitlines(True), fromfile=relative, tofile=relative))

    append('plugins/web/src/lib.rs', '#[cfg(test)]\nextern crate self as uiblueprint_web;')
    append('plugins/web/src/collector/acquire.rs', '#[cfg(test)]\n#[path = ' + json.dumps(str(source/'plugins/web/tests/collector.rs')) + ']\nmod q02_stages;')
    append('plugins/web/tests/collector.rs', (ROOT/'tests/bridges/web/rust-stages-normalize.rs').read_text())
    append('crates/host/src/worker_main.rs', (ROOT/'tests/bridges/web/rust-stages-format.rs').read_text())
    (out/'instrumentation.diff').write_text(''.join(changes))
    manifest = {'source_pin': PIN, 'inputs': {p.name: digest(p) for p in sorted(inputs.glob('*.json'))},
                'instrumentation_sha256': digest(out/'instrumentation.diff'),
                'delta': 'test-only appended modules/helper inclusion; original production bodies unchanged',
                'retention': 'Q02/root stage review; consume then remove own nonimages, no images created'}
    (out/'prepared.json').write_text(json.dumps(manifest, indent=2)+'\n')
    print(json.dumps({'prepared': str(out), 'source_pin': PIN}))


def build(args):
    out = args.output.resolve()
    assert out.is_relative_to(Path(tempfile.gettempdir()).resolve())
    env = dict(os.environ, CARGO_TARGET_DIR=str(out/'target'))
    binaries = {}
    for label, selected in [('normalize', ['-p', 'uiblueprint-web', '--lib']),
                            ('format', ['-p', 'uiblueprint-host', '--features', 'web', '--bin', 'session-worker'])]:
        text = invoke(['cargo', '+1.96.0', 'test', '--locked', '--offline', '--release', '--no-run', '--message-format=json',
                       '--manifest-path', str(out/'source/Cargo.toml'), *selected], env=env, timeout=240)
        artifacts = [json.loads(line) for line in text.splitlines() if line.startswith('{')]
        found = [a['executable'] for a in artifacts if a.get('reason') == 'compiler-artifact' and a.get('executable') and a.get('profile', {}).get('test')]
        assert len(found) == 1, found
        binaries[label] = {'path': found[0], 'sha256': digest(Path(found[0]))}
    (out/'binaries.json').write_text(json.dumps(binaries, indent=2)+'\n')
    print(json.dumps(binaries))


def run(args):
    out = args.output.resolve()
    assert out.is_relative_to(Path(tempfile.gettempdir()).resolve())
    binaries = json.loads((out/'binaries.json').read_text())
    report_path = out / ('stage-report-' + uuid.uuid4().hex[:12] + '.json')
    report = {'source_pin': PIN, 'binaries': binaries, 'runs': [],
              'method': '20 fresh test processes with one call each; 100 repeated calls in one process, no discarded warm-up. Timers exclude input read/decode/setup and post-timer equality checks.',
              'kind': 'offline diagnostic stages, NOT replacement latency cohorts'}
    tests = {'normalize': 'collector::acquire::q02_stages::q02_saved_normalization', 'format': 'worker_main::q02_stage_format::saved_fixed_output'}
    for kind in ('semantic', 'geometry', 'documents'):
        for stage in ('normalize', 'format'):
            binary = binaries[stage]
            assert digest(Path(binary['path'])) == binary['sha256']
            for cohort, count in [('process_fresh', 1)]*20 + [('reused_process', 100)]:
                env = dict(os.environ, UIB_Q02_STAGE_INPUTS=str(out/'inputs'), UIB_Q02_STAGE_KIND=kind, UIB_Q02_STAGE_REPEATS=str(count))
                result = subprocess.run([binary['path'], '--ignored', '--exact', tests[stage], '--nocapture', '--test-threads=1'],
                                        env=env, capture_output=True, timeout=30)
                lines = result.stdout.decode(errors='replace').splitlines()
                rows = [json.loads(line.split('@Q02_STAGES ', 1)[1]) for line in lines if '@Q02_STAGES ' in line]
                entry = {'kind': kind, 'stage': stage, 'cohort': cohort, 'exit': result.returncode, 'data': rows}
                report['runs'].append(entry)
                report_path.write_text(json.dumps(report, indent=2)+'\n')
                if result.returncode or len(rows) != 1 or len(rows[0]['samples']) != count:
                    raise RuntimeError('stage run failed; retained uncensored report: ' + result.stderr.decode(errors='replace')[-2500:])
    summaries = []
    for kind in ('semantic', 'geometry', 'documents'):
        for stage, keys in [('normalize', ['normalization_inclusive_ns', 'format_count_ns']), ('format', ['format_fixed_ns'])]:
            for cohort in ('process_fresh', 'reused_process'):
                values = [s for r in report['runs'] if (r['kind'], r['stage'], r['cohort']) == (kind, stage, cohort) for s in r['data'][0]['samples']]
                for key in keys:
                    numbers = sorted(s[key]/1_000_000 for s in values)
                    summaries.append({'kind': kind, 'stage': key, 'cohort': cohort, 'n': len(numbers),
                                      'p50_ms': statistics.median(numbers), 'p95_ms': numbers[math.ceil(.95*len(numbers))-1], 'max_ms': max(numbers)})
    report['summary'] = summaries
    report_path.write_text(json.dumps(report, indent=2)+'\n')
    print(json.dumps({'report': str(report_path), 'summary': summaries}, indent=2))


parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('action', choices=['prepare', 'build', 'run'])
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--pins', type=Path)
parser.add_argument('--full-sample', type=Path)
parser.add_argument('--full-raw', type=Path)
args = parser.parse_args()
globals()[args.action](args)
