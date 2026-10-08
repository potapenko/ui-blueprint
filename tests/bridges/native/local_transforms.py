#!/usr/bin/env python3
"""Synthetic local mapping checks through the shipping Rust CLI; no live UI."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[3]
LOCAL = {'id': 'f02-scroll-a-local', 'kind': 'local', 'units': 'pt', 'origin': 'top_left'}
SOURCE = dict(LOCAL, id='f02-fixture-local')
ROW = {'namespace': 'macos.swiftui.probe', 'key': 'f02.scroll.a.row.0'}
VIEWPORT = dict(ROW, key='f02.scroll.a.viewport')


def invoke(command, code=0):
    result = subprocess.run([str(x) for x in command], capture_output=True, timeout=15)
    assert result.returncode == code, (result.returncode, result.stderr.decode()[:500])
    return json.loads(result.stdout)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('checks', 'validator', 'cli'):
        parser.add_argument('--' + name, type=Path, required=True)
    args = parser.parse_args()
    # This driver creates JSON only; no image capture/cleanup is performed.
    with tempfile.TemporaryDirectory(prefix='uib-local-transform-check-') as directory:
        run = Path(directory)
        report = invoke([args.checks, ROOT/'tests/bridges/native/acquisition/profile.json',
                         run, ROOT/'fixtures/native/expectations.json'])
        originals = {p: p.read_bytes() for p in run.glob('*.json')}
        for path in originals:
            invoke([args.validator, '--max-bytes', '524288', path])
        query = json.loads((ROOT/'fixtures/analysis/query-gap.json').read_text())
        q = query['artifact']['data']
        q.update(scope_id='f02.scroll.a', operation='inside', targets=[ROW, VIEWPORT], units='pt')
        for anchor, key in zip(q['anchors'], q['targets']):
            anchor.update(element=key, coordinate_space=SOURCE, fraction=0, axis='xy')
        query_path = run/'query.json'
        query_path.write_text(json.dumps(query))
        common = ['--space', LOCAL['id'], '--max-input-bytes', '2097152', '--max-output-bytes', '524288', '--json']
        measured = invoke([args.cli, 'measure', '--snapshot', run/'mapped.json', '--query', query_path, *common])
        measurement = measured['artifact']['data']['result']['measurement']
        assert measurement['value']['value']['amount'] == 6
        assert measurement['space'] == LOCAL
        assert any(e['method'] == 'swiftui_anchor_viewport_origin' and e['provenance'] == 'derived'
                   for e in measurement['evidence'])
        cases = {'mapped_move': (0, 0, 0, 0), 'mapped_scroll': (0, -70, 0, 0),
                 'mapped_resize': (0, 0, 100, 0)}
        for name, expected in cases.items():
            result = invoke([args.cli, 'diff', '--geometry', '--before', run/'mapped.json',
                             '--after', run/(name+'.json'), '--ref', json.dumps(ROW),
                             '--frame-kind', 'layout_bounds', *common])
            assert tuple(result['displacement'][key] for key in ('dx', 'dy', 'dwidth', 'dheight')) == expected
            assert result['before_geometry']['rect'] == {'x': 6, 'y': 6, 'width': 88, 'height': 18}
        # Missing/invalid mapping has no discoverable destination. The source
        # explicitly says unknown; CLI refuses the unavailable selected Space.
        # Do not invent an evaluation transform merely to make discovery succeed.
        for name in ('mapping_missing', 'mapping_stale', 'mapping_environment', 'mapping_origin', 'mapping_scale'):
            snapshot = json.loads((run/(name+'.json')).read_text())['artifact']['data']['result']['data']
            for node in snapshot['nodes']:
                transform = node['properties'][0]['state']['value']['value']['transform']
                assert transform['status'] == 'unknown' and 'transform' not in transform
            result = subprocess.run([str(args.cli), 'measure', '--snapshot', str(run/(name+'.json')),
                                     '--query', str(query_path), *common], capture_output=True, timeout=15)
            assert result.returncode == 2 and result.stdout == b'' and result.stderr == b'unknown_space\n'
        assert all(path.read_bytes() == original for path, original in originals.items())
        report.update(canonical_documents=len(originals), rust_measure_known=1, rust_diff_known=3,
                      unknown_mapping_refusals=5, source_bytes_unchanged=True)
        print(json.dumps(report))
    assert not run.exists()


if __name__ == '__main__':
    main()
