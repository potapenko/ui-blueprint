#!/usr/bin/env python3
"""Recorded F02 fact equality through production WindowAX; never operates UI."""
import argparse
import copy
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile

BASELINE_SHA256 = '42c2b55194136aec4560192ed5945bb3c2bf844f0417503c116219f52c7a651d'


def reconcile(raw, snapshot):
    original = raw['external_semantics']['nodes']
    # Original recorder traversed depth-first; production traverses breadth-first.
    # The replay injects exact original handles and child order, not live identities.
    children = {i: [] for i in range(len(original))}
    for i, node in enumerate(original[1:], 1):
        children[node['parent_index']].append(i)
    order = [0]
    for i in order:
        order.extend(children[i])
    assert len(order) == len(original) == len(snapshot['nodes']) == 75
    assert raw['external_semantics']['coverage'] == snapshot['coverage']['status'] == 'partial'
    known = {}
    actions = 0
    for original_index, node in zip(order, snapshot['nodes']):
        source = original[original_index]
        props = {p['field']: p['state'] for p in node['properties']}
        extensions = {e['name']: e['property']['state'] for e in node['extensions']}
        bounds = props['accessibility_bounds']['value']['value']
        assert bounds['frame_kind'] == 'accessibility_bounds'
        assert bounds['coordinate_space'] == {'id': 'ax-screen', 'kind': 'screen', 'units': 'pt', 'origin': 'top_left'}
        mapping = {'AXRole': node['native_role'], 'AXIdentifier': extensions['AXIdentifier'],
                   'AXTitle': extensions['AXTitle'], 'AXDescription': props['description'],
                   'AXValue': props['value'], 'AXEnabled': props['enabled'], 'AXFocused': props['focused']}
        for name, prop in source['properties'].items():
            if prop['availability'] != 'known':
                if name in mapping:
                    assert mapping[name]['availability'] == prop['availability'], (original_index, name)
                    assert 'value' not in mapping[name]
                continue
            known[name] = known.get(name, 0) + 1
            if name in ('AXPosition', 'AXSize'):
                shape = bounds['shape']['value']
                assert all(shape[k] == v for k, v in prop['value'].items())
            else:
                actual = mapping[name]
                assert actual['availability'] == 'known', (original_index, name)
                value = actual['value']['value']
                assert type(value) is type(prop['value']) or isinstance(value, (int, float)) and not isinstance(value, bool) and type(prop['value']) in (int, float)
                assert value == prop['value'], (original_index, name)
            if name == 'AXDescription':
                assert props['accessibility_name'] == props['description']
        if source['actions_error'] == 0:
            assert props['actions']['availability'] == 'known'
            assert props['actions']['value']['value'] == source['actions']
            actions += 1
        expected_children = [order.index(i) for i in children[original_index]]
        assert [int(c['key'].rsplit('-', 1)[1]) for c in node['children']] == expected_children
    assert known == {'AXFocused': 26, 'AXIdentifier': 61, 'AXPosition': 75, 'AXRole': 75,
                     'AXSize': 75, 'AXTitle': 1, 'AXEnabled': 65, 'AXDescription': 14, 'AXValue': 4}
    return {'known_attributes': known, 'known_fact_count': sum(known.values()), 'action_lists': actions,
            'nodes': len(order), 'edges': len(order) - 1, 'coverage': 'partial'}


def main():
    parser = argparse.ArgumentParser()
    for name in ('checks', 'validator', 'baseline', 'output'):
        parser.add_argument('--' + name, type=Path, required=True)
    args = parser.parse_args()
    assert hashlib.sha256(args.baseline.read_bytes()).hexdigest() == BASELINE_SHA256
    root = Path(__file__).resolve().parents[4]
    assert args.output.resolve().is_relative_to(Path(tempfile.gettempdir()).resolve())
    run = Path(tempfile.mkdtemp(prefix='fidelity-', dir=args.output))
    result = subprocess.run([str(args.checks), str(root / 'tests/bridges/native/acquisition/profile.json'),
                             str(args.baseline), str(run)], capture_output=True, timeout=30)
    assert result.returncode == 0 and not result.stderr, (result.returncode, result.stderr[-2000:])
    report = json.loads(result.stdout)
    for file in sorted(run.glob('*.json')):
        valid = subprocess.run([str(args.validator), '--max-bytes', '524288', str(file)], capture_output=True, timeout=5)
        assert valid.returncode == 0, (file.name, valid.stdout)
    raw = json.loads(args.baseline.read_text())
    snapshot = json.loads((run / 'recorded.json').read_text())['artifact']['data']
    report.update(reconcile(raw, snapshot))
    # A Title-only change/refusal cannot erase any independently admissible
    # sibling state, including unavailable/redacted states and non-property data.
    def without_title(node):
        return dict(node, extensions=[e for e in node['extensions']
                                     if (e['namespace'], e['name']) != ('macos.ax', 'AXTitle')])
    siblings = [without_title(node) for node in snapshot['nodes']]
    for mode in ('empty', 'unsupported', 'unknown', 'oversized', 'wrong-type'):
        changed = json.loads((run / (mode + '.json')).read_text())['artifact']['data']
        assert [without_title(node) for node in changed['nodes']] == siblings, mode + ': Title changed sibling data'
        assert changed['context'] == snapshot['context'] and changed['coverage'] == snapshot['coverage']
    negative = copy.deepcopy(snapshot)
    negative['nodes'][0]['extensions'] = [e for e in negative['nodes'][0]['extensions'] if e['name'] != 'AXTitle']
    try:
        reconcile(raw, negative)
    except (KeyError, AssertionError):
        pass
    else:
        raise AssertionError('missing title must fail')
    report.update(canonical_validated=7, baseline_sha256=BASELINE_SHA256, owned_run=str(run),
                  evidence_kind='recorded_boundary_replay_not_live_SDK_or_D06', missing_title_rejected=True,
                  title_isolation_cases=5, unaffected_nodes_per_case=len(siblings))
    (run / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report, indent=2))


if __name__ == '__main__':
    main()
