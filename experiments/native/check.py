#!/usr/bin/env python3
"""Validate one recorded R02 run; not a release or complete M05 oracle."""
import hashlib
import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
records = {}
for mode in ('off', 'on'):
    for state in ('baseline', 'expanded'):
        key = f'{mode}-{state}'
        folder = root / key
        manifest = json.loads((folder / 'frozen-manifest.json').read_text())
        external = json.loads((folder / 'external.json').read_text())
        assert manifest['probe_enabled'] == (mode == 'on')
        assert manifest['expanded'] == (state == 'expanded')
        assert manifest['activations'] == (1 if state == 'expanded' else 0)
        target = external['requested_target']
        assert target['pid'] == manifest['pid']
        assert target['target_generation'] == manifest['target_generation']
        assert target['window_id'] == manifest['windows'][0]['window_id']
        ax = external['external_semantics']
        assert ax['status'] == 'observed'
        matches = [n for n in ax['nodes'] if n['properties']['AXIdentifier'].get('value') == 'r02.sample']
        assert len(matches) == 1
        sample = matches[0]
        assert sample['properties']['AXRole']['value'] == 'AXButton'
        assert 'AXPress' in sample['actions']
        assert not any(n['parent_index'] == int(sample['key'].split(':')[1]) for n in ax['nodes'])
        cap = external['rendered_capture']
        assert cap['status'] == 'observed' and cap['capture_target'] == target['window_id']
        assert cap['capture_kind'] == 'window_isolated' and not cap['captures_audio']
        records[key] = manifest, sample, cap
        if mode == 'on':
            frames = manifest['probe']['layout_bounds']
            assert set(frames) == {'icon', 'text', 'container'}
            gap = frames['text']['x'] - frames['icon']['x'] - frames['icon']['width']
            assert gap == (18 if state == 'expanded' else 8)
            assert sample['properties']['AXSize']['value'] == {
                'width': frames['container']['width'], 'height': frames['container']['height']}
        else:
            assert manifest['probe']['layout_bounds'] == {}
for state in ('baseline', 'expanded'):
    off = records[f'off-{state}'][1]
    on = records[f'on-{state}'][1]
    for field in ('AXPosition', 'AXSize', 'AXRole', 'AXIdentifier', 'AXFocused', 'AXEnabled'):
        assert off['properties'][field] == on['properties'][field], (state, field)
    assert off['actions'] == on['actions']
# This strict comparison was available for the baseline only. Expanded captures
# differ; do not normalize focus pixels away and label that full invariance.
off_png = (root / 'off-baseline/window.png').read_bytes()
on_png = (root / 'on-baseline/window.png').read_bytes()
assert off_png == on_png
print(json.dumps({'checks': 'pass', 'cases': 4,
                  'baseline_png_sha256': hashlib.sha256(off_png).hexdigest(),
                  'full_hit_region_invariance': 'not_verified',
                  'expanded_pixel_invariance': 'not_verified',
                  'pilot_M05_acceptance': 'open'}, indent=2))
