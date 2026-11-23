"""Validate the S03 synthetic native-part result without inventing a mapping.

Usage: python3 -B check_capture.py CAPTURE_DIRECTORY
Checks fixture scope and lifecycle; canonical schema validation remains separate.
"""
import json
from pathlib import Path
import sys


def check(directory):
    manifests = list(directory.glob('manifest-*.json'))
    assert len(manifests) == 1, 'one terminal manifest required'
    manifest = json.loads(manifests[0].read_text())
    assert manifest['status'] == 'ready' and manifest['exit_status'] == 0
    assert manifest['cleanup'] == 'detached_restored'
    audit = manifest['cache_audit']
    assert audit['model_range_reads'] == 0 and not audit['evaluated_swiftui_or_graph']
    snapshot = json.loads((directory/manifest['snapshot']).read_text())['artifact']['data']
    assert snapshot['coverage']['status'] == 'partial'
    nodes = {json.dumps(n['key'], sort_keys=True): n for n in snapshot['nodes']}
    parent = {}
    for key, node in nodes.items():
        for child in node['children']:
            child_key = json.dumps(child, sort_keys=True)
            assert child_key in nodes and child_key not in parent
            parent[child_key] = key
    roots = set(nodes)-set(parent)
    assert len(roots) == audit['selected_roots'] and roots
    observed = set()
    known_labels = unknown_labels = 0
    for root in roots:
        queue, branch = [root], []
        while queue:
            key = queue.pop()
            assert key not in observed, 'overlapping or cyclic part branches'
            observed.add(key)
            node = nodes[key]
            branch.append(node)
            queue.extend(json.dumps(child, sort_keys=True) for child in node['children'])
        roles = [node['native_role'].get('value', {}).get('value') for node in branch]
        assert roles[0] == '_OverlayModifier', 'unexpected source boundary'
        assert roles.count('Text') == 1, 'missing or ambiguous row label'
        states = [(node, next(p['state'] for p in node['properties']
                             if p['field'] == 'layout_bounds')) for node in branch]
        known = [(node, state['value']['value']) for node, state in states
                 if state['availability'] == 'known']
        assert any(g['shape']['value']['height'] == 36 for _, g in known), 'fixture row size absent'
        assert any(g['transform']['status'] == 'known' for _, g in known), 'row root mapping absent'
        for node, geometry in known:
            assert geometry['coordinate_space']['units'] == 'pt'
            if node['native_role'].get('value', {}).get('value') != 'Text':
                continue
            assert geometry['shape']['value']['height'] == 16
            if geometry['transform']['status'] == 'known':
                known_labels += 1
            else:
                assert geometry['transform']['status'] == 'unknown'
                unknown_labels += 1
    assert observed == set(nodes), 'unaccounted nodes'
    return len(roots), known_labels, unknown_labels


if __name__ == '__main__':
    assert len(sys.argv) == 2, __doc__
    parts, known, unknown = check(Path(sys.argv[1]))
    print(f'PASS: {parts} native-backed row branches; label transforms known={known}, unknown={unknown}.')
    print('Product actor identity, complete list coverage and native-image mapping are not qualified.')
