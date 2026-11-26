#!/usr/bin/env python3
"""Check one state of the owned UIKitRuntimeFixture through the normal caller."""
import argparse
import importlib.util
import json
from pathlib import Path
import subprocess
import uuid
from unittest.mock import patch

p = argparse.ArgumentParser(description=__doc__)
for name in ('session', 'bundle', 'evidence', 'phase'):
    p.add_argument('--' + name, required=True)
p.add_argument('--menu', choices=('present', 'absent'), required=True)
a = p.parse_args()
repo = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('runtime', repo / 'plugins/macos/swiftui_runtime.py')
runtime = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runtime)
root, endpoint = runtime.endpoint(a.session)
assert endpoint['bundle'] == 'me.uiblueprint.uikitfixture', 'only the owned fixture is admitted'
evidence = Path(a.evidence)
assert evidence.is_dir()
windows, _ = runtime.request(a.session, 'windows')
summary = dict(phase=a.phase, windows=[], menu_nodes=0)
for window in windows['windows']:
    response, raw = runtime.request(a.session, 'capture', window['window'], window['generation'], source='uikit')
    assert not any(value in raw for value in (b'UIB_UIKIT_PRIVATE_CANARY_7d9c', b'First option', b'Second option', b'Ascending'))
    doc = json.loads(raw)['artifact']['data']
    assert doc['context']['target']['generation'] == endpoint['run_id']
    assert doc['context']['surfaces'][0]['generation'] == window['generation']
    assert doc['coverage']['status'] == 'complete'
    path = evidence / f'{a.phase}-window-{window["window"]}.json'
    runtime.publish(path, raw)
    subprocess.run([str(Path(a.bundle) / 'uiblueprint-validate'), '--max-bytes', str(runtime.MAX_BYTES), str(path)],
                   check=True, capture_output=True, timeout=30)
    names = [n['native_role']['value']['value'] for n in doc['nodes']]
    menus = names.count('_UIContextMenuView')
    summary['menu_nodes'] += menus
    summary['windows'].append(dict(window=window['window'], is_key=window['is_key'], nodes=len(names),
                                   observation=response['observation_id'], menu_nodes=menus))
    if menus:
        measured = subprocess.run(['python3', '-B', str(repo / 'plugins/macos/swiftui_runtime.py'), 'measure',
            '--source', 'uikit', '--session', a.session, '--window', str(window['window']),
            '--generation', window['generation'], '--type', '_UIContextMenuView', '--relation', 'width',
            '--bundle', a.bundle], check=True, capture_output=True, timeout=35)
        answer = json.loads(measured.stdout)
        assert len(answer['measurements']) == 1
        result = answer['measurements'][0]['result']['result']
        assert result['status'] == 'known'
        summary['menu_width_pt'] = result['measurement']['value']['value']['amount']
        scoped, scoped_raw = runtime.request(a.session, 'capture', window['window'], window['generation'],
                                             subtree_type='_UIContextMenuView', source='uikit')
        assert scoped['selection']['subtree_nodes'] > 1
        summary['scoped_nodes'] = scoped['nodes']
    try:
        runtime.request(a.session, 'capture', window['window'], 'stale-generation', source='uikit')
        raise AssertionError('stale window accepted')
    except runtime.Failure as error:
        assert str(error) == 'staleWindow'
assert (summary['menu_nodes'] > 0) == (a.menu == 'present')
if len(windows['windows']) > 1:
    try:
        runtime.request(a.session, 'capture', source='uikit')
        raise AssertionError('ambiguous window accepted')
    except runtime.Failure as error:
        assert str(error) == 'ambiguousWindow'
request_id = str(uuid.uuid4())
runtime.publish(root / (request_id + '.cancel'), b'cancel\n')
with patch.object(runtime.uuid, 'uuid4', return_value=request_id):
    try:
        runtime.request(a.session, 'capture', source='uikit')
        raise AssertionError('cancelled capture published')
    except runtime.Failure as error:
        assert str(error) == 'cancelled'
assert not list(root.glob(request_id + '*'))
# Default command still reads the SwiftUI hosting source in the main window.
main = next(w for w in windows['windows'] if w['is_key'])
legacy, raw = runtime.request(a.session, 'capture', main['window'], main['generation'])
assert json.loads(raw)['artifact']['data']['observations'][0]['source_namespace'] == 'swiftui.runtime.layout'
summary['legacy_swiftui_nodes'] = legacy['nodes']
summary.update(stale_refused=True, cancellation_no_artifact=True, canary_absent=True)
runtime.publish(evidence / (a.phase + '-summary.json'), runtime.encoded(summary))
print(json.dumps(summary))
