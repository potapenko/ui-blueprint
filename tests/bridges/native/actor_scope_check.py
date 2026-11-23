"""Check synthetic Actors captures against independent public SwiftUI geometry.

Usage: python3 -B actor_scope_check.py WIDE_DIRECTORY NARROW_DIRECTORY DUPLICATE_DIRECTORY [SPLIT_DIRECTORY]
Consumes fixture-only body-owner attribution, never supplies product correspondence.
Checks known translations against public SwiftUI global frames; never qualifies
native-window/PNG mapping. Split-owner completeness is an optional negative case.
"""
import json
from pathlib import Path
import sys


PARTS = {
    'ActorHeader': 'header', 'ActorClose': 'close', 'ActorQuery': 'query',
    'ActorPossible': 'toggle', 'ActorSuggestions': 'scroll',
    'ActorSuggestionRow': 'row', 'ActorSuggestionLabel': 'label',
}


def read(path):
    return json.loads(path.read_text())


def manifest(directory):
    matches = list(directory.glob('manifest-*.json'))
    assert len(matches) == 1, 'one terminal manifest required'
    return read(matches[0])


def positive(directory):
    result = manifest(directory)
    assert result['status'] == 'ready' and result['exit_status'] == 0
    assert result['cleanup'] == 'detached_restored'
    assert result['cache_audit']['model_range_reads'] == 0
    assert not result['cache_audit']['evaluated_swiftui_or_graph']
    assert (directory/'oracle-before.json').read_bytes() == (directory/'oracle-after.json').read_bytes()
    oracle = read(directory/'oracle-after.json')
    document = read(directory/result['snapshot'])['artifact']['data']
    assert document['context']['target']['id'] == 'macos-pid-'+str(oracle['pid'])
    nodes = document['nodes']
    indices = {json.dumps(n['key'], sort_keys=True): i for i, n in enumerate(nodes)}
    parents = {}
    for i, node in enumerate(nodes):
        for child in node['children']:
            child_index = indices[json.dumps(child, sort_keys=True)]
            assert child_index not in parents
            parents[child_index] = i
    owners = {r['node_index']: r['body_owner'] for r in read(directory/'fixture-owners.json')}

    def outermost(index, name):
        seen = set()
        while index in parents:
            index = parents[index]
            assert index not in seen
            seen.add(index)
            if owners.get(index) == name:
                return False
        return True

    frames = {}
    mapped = set()
    for source, part in PARTS.items():
        name = 'ActorScopeFixture.'+source
        candidates = [i for i, owner in owners.items() if owner == name and outermost(i, name)]
        assert len(candidates) == 1, 'ambiguous or missing fixture part: '+part
        state = next(p['state'] for p in nodes[candidates[0]]['properties'] if p['field'] == 'layout_bounds')
        assert state['availability'] == 'known', part
        geometry = state['value']['value']
        assert geometry['coordinate_space']['units'] == 'pt'
        rect = geometry['shape']['value']
        frames[part] = [rect[k] for k in ('x', 'y', 'width', 'height')]
        assert frames[part][2:] == oracle['frames'][part][2:], part+' size mismatch'
        transform = geometry['transform']
        if transform['status'] == 'known':
            mapping = transform['transform']
            assert mapping['from'] == geometry['coordinate_space']
            assert mapping['target'] == document['context']['target']
            assert mapping['surface'] == nodes[candidates[0]]['surface']
            assert mapping['to']['units'] == 'pt' and mapping['to']['origin'] == 'top_left'
            affine = mapping['affine']
            assert affine[:4] == [1, 0, 0, 1], 'unsupported fixture mapping'
            converted = [rect['x']+affine[4], rect['y']+affine[5], rect['width'], rect['height']]
            assert converted == oracle['frames'][part+'-global'], part+' transformed mismatch'
            mapped.add(part)
        elif part not in ('row', 'label'):
            assert transform['status'] == 'unknown'
            assert frames[part] == oracle['frames'][part+'-global'], part+' global mismatch'
    # This checks an observed pair inside one fixture scroll subtree. It does not
    # promote the Snapshot's separate unknown transforms to a common space.
    for axis in (0, 1):
        assert frames['label'][axis]-frames['row'][axis] == (
            oracle['frames']['label-global'][axis]-oracle['frames']['row-global'][axis])
    for node in nodes:
        state = next(p['state'] for p in node['properties'] if p['field'] == 'layout_bounds')
        if state['availability'] == 'known':
            rect = state['value']['value']['shape']['value']
            assert [rect['width'], rect['height']] != [37, 19], 'outside sentinel acquired'
    return oracle['pid'], document['context']['target'], frames, mapped


def main():
    assert len(sys.argv) in (4, 5), __doc__
    wide, narrow, duplicate = map(Path, sys.argv[1:4])
    wide_pid, wide_target, wide_frames, wide_mapped = positive(wide)
    narrow_pid, narrow_target, narrow_frames, narrow_mapped = positive(narrow)
    assert wide_pid == narrow_pid and wide_target == narrow_target, 'different process generations'
    assert wide_frames['query'][2]-narrow_frames['query'][2] == 140
    result = manifest(duplicate)
    assert result['code'] == 'native_anchor_unresolved' and result['exit_status'] == 4
    assert result['snapshot'] is None and result['cleanup'] == 'detached_restored'
    assert not list(duplicate.glob('snapshot-*.json'))
    if len(sys.argv) == 5:
        split = Path(sys.argv[4])
        result = manifest(split)
        assert result['cleanup'] == 'detached_restored'
        if result['status'] == 'ready':
            expected = read(split/'oracle-after.json')['frames']['split-owner'][2:]
            nodes = read(split/result['snapshot'])['artifact']['data']['nodes']
            sizes = []
            for node in nodes:
                state = next(p['state'] for p in node['properties'] if p['field'] == 'layout_bounds')
                if state['availability'] == 'known':
                    rect = state['value']['value']['shape']['value']
                    sizes.append([rect['width'], rect['height']])
            assert expected in sizes, 'split body-owner branch silently omitted'
        else:
            assert result['snapshot'] is None and result['exit_status'] != 0
    print('PASS: seven part sizes; five global frame matches; same-process resize; duplicate refusal.')
    if wide_mapped == narrow_mapped == set(PARTS.values()):
        print('PASS: all seven root-space translations match public SwiftUI geometry in both states.')
    print('PENDING: product capture and native-window mapping; split coverage requires its separate case.')


if __name__ == '__main__':
    main()
