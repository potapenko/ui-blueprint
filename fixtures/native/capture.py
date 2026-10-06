#!/usr/bin/env python3
"""Bounded recording driver for the owned fixture, never UI input automation."""
import argparse
import json
import os
import math
import pathlib
import resource
import statistics
import subprocess
import time

parser = argparse.ArgumentParser()
parser.add_argument('helper', type=pathlib.Path)
parser.add_argument('manifest', type=pathlib.Path)
parser.add_argument('output', type=pathlib.Path)
parser.add_argument('--samples', type=int, default=1)
parser.add_argument('--cold-runs', type=int, default=1)
parser.add_argument('--request-timeout', type=float, default=3)
parser.add_argument('--capture-lock', type=pathlib.Path, help='Shared task-owned capture resource path, outside source/config trees')
args = parser.parse_args()
assert 1 <= args.samples <= 30 and 1 <= args.cold_runs <= 8
assert 0 < args.request_timeout <= 30
child_env = dict(os.environ)
if args.capture_lock:
    assert args.capture_lock.is_absolute()
    child_env['UIB_CAPTURE_LOCK_PATH'] = str(args.capture_lock)
if 'UIB_CAPTURE_LOCK_PATH' not in child_env and child_env.get('UIB_AX_ONLY') != '1':
    parser.error('explicit task-owned --capture-lock is required for capture')
args.output.mkdir(parents=True, exist_ok=False)
initial = json.loads(args.manifest.read_text())
(args.output / 'frozen-manifest.json').write_text(json.dumps(initial, indent=2))
summary = {'process_batch_wall_seconds': [], 'helper_cpu_seconds': [], 'helper_peak_rss_bytes': [],
           'warm_ax_seconds': [], 'warm_capture_seconds': [], 'warm_total_seconds': [],
           'cold_ax_seconds': [], 'cold_capture_seconds': [], 'cold_total_seconds': [],
           'json_payload_bytes': [], 'sample_count': 0}
def preserved_channels(destination):
    completed = [str(p.relative_to(args.output)) for p in sorted(destination.glob('ax-*.json'))]
    capture = []
    for file in sorted(destination.glob('external-*.json')):
        value = json.loads(file.read_text())
        capture.append(value.get('rendered_capture', {}))
    return {'completed_ax_files': completed, 'capture_outcomes': capture}

for run in range(args.cold_runs):
    destination = args.output / f'run-{run}'
    destination.mkdir()
    usage = resource.getrusage(resource.RUSAGE_CHILDREN)
    begin = time.monotonic()
    try:
        subprocess.run([str(args.helper), str(args.manifest), str(destination), str(args.samples)], check=True, timeout=args.request_timeout, env=child_env)
    except subprocess.TimeoutExpired:
        # subprocess.run has killed and reaped this owned read-only child.
        (args.output / 'timeout.json').write_text(json.dumps({'status': 'timeout', 'injection': 'driver_deadline',
            'deadline_seconds': args.request_timeout, 'elapsed_seconds': time.monotonic()-begin,
            'owned_helper_reaped': True, **preserved_channels(destination)}))
        print('F02 bounded driver timeout observed; not an OS AX hang test')
        raise SystemExit(2)
    except subprocess.CalledProcessError as error:
        status = 'timeout' if error.returncode == 124 else 'helper_failed'
        (args.output / 'error.json').write_text(json.dumps({'status': status,
            'helper_exit': error.returncode, 'elapsed_seconds': time.monotonic()-begin,
            'evidence_complete': False, 'owned_helper_reaped': True, **preserved_channels(destination)}))
        print(f'F02 helper {status}, exit {error.returncode}; observation is incomplete')
        raise SystemExit(error.returncode)
    summary['process_batch_wall_seconds'].append(time.monotonic() - begin)
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    summary['helper_cpu_seconds'].append(after.ru_utime + after.ru_stime - usage.ru_utime - usage.ru_stime)
    summary['helper_peak_rss_bytes'].append(after.ru_maxrss)
    for i in range(args.samples):
        file = destination / f'external-{i}.json'
        value = json.loads(file.read_text())
        kind = 'cold' if i == 0 else 'warm'
        ax_duration = value['external_semantics'].get('duration_seconds')
        if ax_duration is not None: summary[f'{kind}_ax_seconds'].append(ax_duration)
        summary[f'{kind}_capture_seconds'].append(value['capture_duration_seconds'])
        summary[f'{kind}_total_seconds'].append(value['total_duration_seconds'])
        summary['json_payload_bytes'].append(file.stat().st_size)
        summary['sample_count'] += 1
final = json.loads(args.manifest.read_text())
for field in ('target_generation', 'surface_generation', 'source_state', 'state', 'app_active', 'window_key'):
    if initial[field] != final[field]:
        raise RuntimeError(f'Scenario changed during observation: {field}')
(args.output / 'final-manifest.json').write_text(json.dumps(final, indent=2))
def stats(values):
    return {'count': len(values), 'p50': statistics.median(values),
            'p95_nearest_rank': sorted(values)[math.ceil(len(values)*0.95)-1], 'max': max(values)} if values else {'count': 0}
summary['statistics'] = {key: stats(value) for key, value in summary.items() if isinstance(value, list)}
summary['method'] = 'cold=first request in new helper; warm=subsequent requests in same helper, still re-reading AX/SC inventory; wall includes all samples when samples>1; ru_maxrss is cumulative child high-water in this driver'
(args.output / 'summary.json').write_text(json.dumps(summary, indent=2))
print(json.dumps({'recorded': summary['sample_count'], 'output': str(args.output)}))
