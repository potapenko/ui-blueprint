#!/usr/bin/env python3
"""Own-popup saved-pair qualification through the shipping Rust measurement CLI.

This explicit test composition preserves each source Observation and clock. It is
not a live synchronization service or a mixed-channel Observe response.
"""
import argparse
import copy
import json
from pathlib import Path
import subprocess
import tempfile


def invoke(command, code=0):
    result = subprocess.run([str(x) for x in command], capture_output=True, timeout=10)
    assert result.returncode == code, (result.returncode, result.stderr.decode()[:300])
    return json.loads(result.stdout)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('pair', 'cli', 'validator'):
        parser.add_argument('--' + name, type=Path, required=True)
    args = parser.parse_args()
    with args.pair.open('rb') as source:
        raw = source.read(1048577)
    assert len(raw) <= 1048576
    documents = [json.loads(line) for line in raw.splitlines()]
    assert len(documents) == 2
    ax, capture = [d['artifact']['data'] for d in documents]
    assert ax['channel'] == 'external_semantics' and capture['channel'] == 'rendered_capture'
    assert all(ax[key] == capture[key] for key in ('request_id','dispatch_sequence','session_id','target'))
    assert ax['result']['status'] == capture['result']['status'] == 'observed'
    original, pixels = ax['result']['data'], capture['result']['data']
    assert original['context'] == pixels['context']
    assert len(pixels['captures']) == 1 and not pixels['nodes']
    frame = pixels['captures'][0]
    mapping = frame['crop_transform']['transform']
    assert frame['crop_transform']['status'] == 'known'
    assert frame['included_surfaces'] == [mapping['surface']]
    assert mapping['surface'] == frame['capture_target']
    assert frame['surface_coverage'] == 'partial'
    snapshot = copy.deepcopy(original)
    snapshot['id'] = 'm03c-composed-' + ax['request_id']
    snapshot['observations'] += pixels['observations']
    snapshot['captures'] = pixels['captures']
    assert snapshot['nodes'] == original['nodes']
    # Both surface records are already sourced by AX; capture retains its own
    # observations/evidence. No time is translated or marked synchronized.
    popup_nodes = [n for n in snapshot['nodes'] if n['surface'] == mapping['surface']]
    button = [n for n in popup_nodes if any(p['field'] == 'accessibility_name'
        and p['state'].get('value', {}).get('value') == 'Confirm popup' for p in n['properties'])]
    assert len(button) == 1
    root, button = popup_nodes[0], button[0]
    evaluation = dict(snapshot_id=snapshot['id'], revision=snapshot['revision'],
        context=snapshot['context'], result_space=mapping['to'], transforms=[mapping], conditions=None)
    def document(kind, data, version='0.2.0'):
        return dict(schema_version=version, artifact=dict(kind=kind, data=data))
    with tempfile.TemporaryDirectory(prefix='uib-m03c-consumer-') as directory:
        run = Path(directory)
        def write(name, value):
            path = run/(name+'.json'); path.write_text(json.dumps(value)); return path
        saved = write('snapshot', document('snapshot', snapshot, '0.1.0'))
        ev = write('evaluation', document('evaluation_input', evaluation))
        invoke([args.validator, '--max-bytes', 1048576, saved])
        def measure(operation, nodes):
            query = dict(id='popup-'+operation, scope_id=snapshot['context']['scope_id'],
                targets=[n['key'] for n in nodes], operation=operation,
                anchors=[dict(element=n['key'], frame_kind='accessibility_bounds',
                    coordinate_space=mapping['from'], fraction=0, axis='xy') for n in nodes],
                quantity_kind='length', units='px',
                applies_when=dict(platform=None,input_mode=None,text_scale=None))
            q = write('query', document('geometry_query', query))
            result = invoke([args.cli, 'measure', '--snapshot', saved, '--query', q,
                '--evaluation', ev, '--space', mapping['to']['id'], '--max-input-bytes', 2097152,
                '--max-output-bytes', 1048576, '--json'])['artifact']['data']
            assert result['snapshot'] == snapshot
            assert result['result']['status'] == 'known'
            measured = result['result']['measurement']
            assert mapping['evidence'] in measured['evidence']
            return measured
        root_width = measure('width', [root]); root_height = measure('height', [root])
        assert root_width['value']['value']['amount'] == frame['pixel_width']
        assert root_height['value']['value']['amount'] == frame['pixel_height']
        width, height = measure('width', [button]), measure('height', [button])
        inside = measure('inside', [button, root])
        assert inside['value']['value']['amount'] >= 0
        # Binding is checked by the real schema/engine, not repaired by the test.
        # No matching transform must remain missing even with equal numeric units.
        query_path = run/'query.json'
        for case in ('missing', 'environment', 'generation', 'parent'):
            changed = copy.deepcopy(evaluation)
            if case == 'missing':
                changed['transforms'] = []
            elif case == 'environment':
                changed['transforms'][0]['environment_revision'] = 'different-environment'
            elif case == 'generation':
                changed['transforms'][0]['surface']['generation'] = 'retired-generation'
            else:
                changed['transforms'][0]['surface'] = frame['excluded_surfaces'][0]
            ev.write_text(json.dumps(document('evaluation_input', changed)))
            result = subprocess.run([str(args.cli), 'measure', '--snapshot', str(saved),
                '--query', str(query_path), '--evaluation', str(ev), '--space', mapping['to']['id'],
                '--max-input-bytes', '2097152', '--max-output-bytes', '1048576', '--json'],
                capture_output=True, timeout=10)
            if case in ('environment', 'generation'):
                assert result.returncode == 2 and not result.stdout, (case, result.stderr)
            else:
                assert result.returncode == 4, (case, result.stderr)
                unknown = json.loads(result.stdout)['artifact']['data']['result']
                assert unknown['status'] == 'unknown' and unknown['reason'] == 'missing_transform'
        print(json.dumps(dict(root_pixels=[frame['pixel_width'],frame['pixel_height']],
            button_pixels=[width['value']['value']['amount'],height['value']['value']['amount']],
            button_insets=inside['details'], separate_clock_domains=sorted(set(o['clock_domain'] for o in snapshot['observations'])),
            transform_evidence_consumed=True, source_records_preserved=True, binding_refusals=4)))
    assert not run.exists() and args.pair.read_bytes() == raw


if __name__ == '__main__':
    main()
