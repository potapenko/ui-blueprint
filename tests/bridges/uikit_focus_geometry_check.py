#!/usr/bin/env python3
"""Qualify one owned TV focus fixture through the ordinary UIKit capture path."""
import argparse
import concurrent.futures
import importlib.util
import json
from pathlib import Path
import subprocess
import time
from types import SimpleNamespace

parser = argparse.ArgumentParser(description=__doc__)
for key in ('session', 'simulator', 'out', 'bundle'):
    parser.add_argument('--' + key, required=True)
a = parser.parse_args()
repo = Path(__file__).resolve().parents[2]
def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    value = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(value)
    return value
runtime = module('focus_runtime', repo / 'plugins/macos/swiftui_runtime.py')
simulator = module('focus_simulator', repo / 'plugins/ios/swiftui_simulator.py')
root, descriptor = runtime.endpoint(a.session)
assert descriptor['bundle'] == 'me.uiblueprint.tvfocusfixture', 'owned fixture only'
container = Path(simulator.simctl('get_app_container', a.simulator, descriptor['bundle'], 'data')).resolve()
assert root.is_relative_to(container)
deadline = time.monotonic() + 15
oracle_path = container / 'tmp/focus-oracle.json'
checks_path = container / 'tmp/matrix-checks.json'
while not checks_path.exists() and time.monotonic() < deadline:
    time.sleep(.1)
oracle = json.loads(oracle_path.read_bytes())
checks = json.loads(checks_path.read_bytes())
assert len(checks['cases']) == 16
assert all(c['passed'] for c in checks['cases']), checks['cases']
windows, _ = runtime.request(a.session, 'windows')
assert len(windows['windows']) == 1
window = windows['windows'][0]
assert window['platform'] == 'tvos-simulator'
out = Path(a.out)
pair = simulator.pair(SimpleNamespace(session=a.session, simulator=a.simulator,
    out=str(out), window=window['window'], timeout=120, source='uikit'), runtime)
raw = (out / 'snapshot.json').read_bytes()
assert not any(text in raw for text in (b'Synthetic focus row', b'Second fixture row', b'Geometry fixture'))
doc = json.loads(raw)
nodes = doc['artifact']['data']['nodes']
assert doc['artifact']['data']['coverage']['omitted_count'] == 0
focus = [v for v in oracle if 'Floating' in v['type']]
assert len(focus) >= 5 and any(v['focused'] for v in oracle)
for observed in oracle:
    # Both traversals retain native source order; verify identity/type/bounds first.
    node = nodes[observed['index']]
    assert node['native_role']['value']['value'] == observed['type'].split('<', 1)[0]
    geometry = node['properties'][0]['state']['value']['value']
    rect = geometry['shape']['value']
    assert [rect[k] for k in ('x', 'y', 'width', 'height')] == observed['bounds']
    assert geometry['transform']['status'] == 'known', observed['type']
    m = geometry['transform']['transform']['affine']
    x, y, w, h = observed['bounds']
    for (px, py), actual in zip(((0, 0), (1, 0), (0, 1), (x, y), (x+w, y+h)), observed['points']):
        expected = [m[0]*px + m[2]*py + m[4], m[1]*px + m[3]*py + m[5]]
        assert all(abs(v-e) <= 1e-7 for v,e in zip(actual, expected)), observed['type']
subprocess.run([str(Path(a.bundle)/'uiblueprint-validate'), '--max-bytes', str(runtime.MAX_BYTES),
                str(out/'snapshot.json')], check=True, timeout=30, capture_output=True)
# Rust evaluates a real focus descendant in the WINDOW space, not only its local size.
node = next(n for n in nodes if n['native_role']['value']['value'] == '_UIFloatingContentCornerRadiusAnimatingView')
g = node['properties'][0]['state']['value']['value']
space = g['transform']['transform']['to']
query = dict(id='focus-window-width', scope_id='window-1', targets=[node['key']],
    applies_when=dict(platform=None,input_mode=None,text_scale=None), operation='width',
    anchors=[dict(element=node['key'],frame_kind='layout_bounds',coordinate_space=g['coordinate_space'],fraction=0,axis='x')],
    quantity_kind='length',units='pt')
query_path = out/'query.json'
query_path.write_bytes(runtime.encoded(dict(schema_version='0.2.0',artifact=dict(kind='geometry_query',data=query))))
evaluation = dict(snapshot_id=doc['artifact']['data']['id'],revision=doc['artifact']['data']['revision'],
    context=doc['artifact']['data']['context'],result_space=space,transforms=[g['transform']['transform']],conditions=None)
evaluation_path = out/'evaluation.json'
evaluation_path.write_bytes(runtime.encoded(dict(schema_version='0.2.0',artifact=dict(kind='evaluation_input',data=evaluation))))
answer = subprocess.run([str(Path(a.bundle)/'uiblueprint'),'measure','--snapshot',str(out/'snapshot.json'),
    '--query',str(query_path),'--evaluation',str(evaluation_path),'--space',space['id'],'--max-input-bytes',str(runtime.MAX_BYTES),
    '--max-output-bytes','1048576','--json','--result-only'],check=True,capture_output=True,timeout=30)
measured = json.loads(answer.stdout)
assert measured['result']['status'] == 'known'
(out/'measurement.json').write_bytes(answer.stdout)
try:
    runtime.request(a.session,'capture',window['window'],'stale',source='uikit')
    raise AssertionError('stale generation accepted')
except runtime.Failure as failure:
    assert str(failure) == 'staleWindow'
def fresh(_):
    response, raw = runtime.request(a.session,'capture',window['window'],window['generation'],source='uikit')
    return response['observation_id']
with concurrent.futures.ThreadPoolExecutor(max_workers=2) as pool:
    observations = list(pool.map(fresh,range(2)))
assert len(set([pair['observation_id'],*observations])) == 3
# Default SwiftUI source remains usable on the same fixture.
response, raw = runtime.request(a.session,'capture',window['window'],window['generation'])
assert json.loads(raw)['artifact']['data']['observations'][0]['source_namespace'] == 'swiftui.runtime.layout'
(out/'legacy-swiftui.json').write_bytes(raw)
subprocess.run([str(Path(a.bundle)/'uiblueprint-validate'),'--max-bytes',str(runtime.MAX_BYTES),str(out/'legacy-swiftui.json')],
               check=True,capture_output=True,timeout=30)
summary = dict(status='passed',nodes=len(nodes),focus_nodes=len(focus),matrix_cases=checks['cases'],
    model_window_width_pt=measured['result']['measurement']['value']['value']['amount'],
    fresh_observations=observations,legacy_swiftui_nodes=response['nodes'],stale_refused=True,canary_absent=True)
for name, value in [('public-oracle.json',oracle),('matrix-checks.json',checks),('summary.json',summary)]:
    (out/name).write_bytes(runtime.encoded(value))
print(json.dumps({k:summary[k] for k in ('status','nodes','focus_nodes','model_window_width_pt','legacy_swiftui_nodes')}))
