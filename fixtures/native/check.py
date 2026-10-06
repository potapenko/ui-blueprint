#!/usr/bin/env python3
"""Check recorded F02 claims; never convert an open product gate to pass."""
import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
expected = json.loads((pathlib.Path(__file__).parent / 'expectations.json').read_text())
def manifest(case): return json.loads((root / case / 'frozen-manifest.json').read_text())
def observation(case): return json.loads((root / case / 'run-0/external-0.json').read_text())
def sample(case):
    m = manifest(case)
    matches = [n for n in observation(case)['external_semantics']['nodes']
               if n['properties']['AXIdentifier'].get('value') == f"f02.sample.{m['role']}"]
    assert len(matches) == 1
    return matches[0]
def frame(m): return next(w['frame_appkit_screen_pt'] for w in m['windows'] if w['identifier'] == m['role'])
for state in ('baseline', 'expanded', 'hit'):
    off, on = f'off-{state}', f'on-{state}'
    for field in ('AXSize', 'AXIdentifier', 'AXRole', 'AXEnabled'):
        assert sample(off)['properties'][field] == sample(on)['properties'][field]
    assert sample(off)['actions'] == sample(on)['actions']
    m = manifest(on)
    f = m['probe']['layout_bounds']
    gap = f['text']['x'] - f['icon']['x'] - f['icon']['width']
    assert gap == expected['gap_pt']['baseline' if state == 'baseline' else 'expanded']
    assert manifest(off)['probe']['layout_bounds'] == {}
assert manifest('off-hit')['state']['activations'] == manifest('on-hit')['state']['activations'] == 5
assert manifest('two-windows-a')['state']['activations'] == 5
assert manifest('two-windows-b')['state']['activations'] == 1
assert manifest('two-windows-a')['window_id'] != manifest('two-windows-b')['window_id']
assert manifest('form-accepted')['state']['applied'] == expected['accepted_result_by_window']['b']
assert manifest('form-accepted')['state']['secret'] == {'availability': 'redacted'}
assert manifest('scroll-end')['state']['applied'] == 'popup-b'
assert manifest('scroll-end')['state']['scroll_end'] is True
assert frame(manifest('resized-b'))['width'] == 650
before, after = manifest('final-before-move'), manifest('final-after-move')
assert before['window_id'] == after['window_id']
assert frame(after)['x'] - frame(before)['x'] == 40
assert frame(after)['y'] - frame(before)['y'] == -20
assert before['probe']['layout_bounds'] == after['probe']['layout_bounds']
stale = json.loads((root / 'stale-reopen-final/result.json').read_text())
assert stale['same_window_id'] and stale['old_generation'] != stale['new_generation'] and stale['exit'] == 1
assert len(observation('partial-b')['external_semantics']['nodes']) == 8
assert observation('partial-b')['external_semantics']['coverage'] == 'partial'
assert observation('slow-b')['external_semantics']['duration_seconds'] >= expected['synthetic_slow_delay_seconds']
loss = manifest('lost-event-b')['source_state']
assert loss['revision'] > loss['event_revision']
for channel in ('external_semantics', 'rendered_capture'):
    assert observation('permission-injected-b')[channel]['injection'] == 'synthetic'
    assert observation('permission-injected-b')[channel]['status'] == 'permission_required'
assert json.loads((root / 'driver-timeout/timeout.json').read_text())['owned_helper_reaped']
assert any(n['properties']['AXF02Unsupported']['availability'] == 'unsupported'
           for n in observation('final-on-cold')['external_semantics']['nodes'])
# Input canary is only scanned in persisted diagnostic JSON, not declared as
# universal redaction or a guarantee about every input-delivery transport.
canary = b'f02-synthetic-canary-6b8'
assert not any(canary in p.read_bytes() for p in root.rglob('*.json'))
for mode in ('off', 'on'):
    cold = json.loads((root / f'final-{mode}-cold/summary.json').read_text())
    warm = json.loads((root / f'final-{mode}-warm/summary.json').read_text())
    assert cold['statistics']['cold_ax_seconds']['count'] == 5
    assert warm['statistics']['warm_ax_seconds']['count'] == 19
    assert manifest(f'final-{mode}-cold')['state']['expanded'] is True
normalized = json.loads((root / 'normalized-comparison.json').read_text())
assert normalized['off']['state'] == normalized['on']['state']
assert normalized['off']['frame'] == normalized['on']['frame']
assert normalized['off']['app_active'] != normalized['on']['app_active']
concurrent = json.loads((root / 'concurrent-summary.json').read_text())
assert concurrent['a_helper_exit'] == concurrent['b_helper_exit'] == 124
request_proof = json.loads((root / 'request-validation.json').read_text())
assert request_proof['no_manifest_before_request'] and request_proof['no_publication_between_requests']
for case, expected_gap in [('request-initial', 8), ('request-updated', 18)]:
    m = manifest(case)
    assert m['collection_mode'] == 'explicit_request_only'
    frames = m['probe']['layout_bounds']
    assert set(frames) == {'icon', 'text', 'container'}
    assert frames['text']['x'] - frames['icon']['x'] - frames['icon']['width'] == expected_gap
assert manifest('request-updated')['state']['activations'] == 1
print(json.dumps({'verified_fixture_subset': 'pass', 'full_M01_M06_acceptance': 'open',
                  'pixel_comparison': 'not_comparable_active_key_context',
                  'pointer_matrix': 'six expanded-state points matched; not full hit-region proof',
                  'concurrent_capture': 'failed_with_bounded_watchdog',
                  'secure_json_canary': 'absent', 'current_explicit_request_probe': 'pass'}, indent=2))
