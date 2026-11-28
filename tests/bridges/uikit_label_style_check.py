#!/usr/bin/env python3
"""Owned UILabel fixture acceptance; never reads/exports real application text."""
import argparse
import importlib.util
import json
from pathlib import Path
import subprocess
import time
import uuid
from unittest.mock import patch

p=argparse.ArgumentParser(description=__doc__)
for name in ('session','simulator','out','bundle'):p.add_argument('--'+name,required=True)
a=p.parse_args();repo=Path(__file__).resolve().parents[2]
spec=importlib.util.spec_from_file_location('runtime',repo/'plugins/macos/swiftui_runtime.py')
r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
root,descriptor=r.endpoint(a.session)
assert descriptor['bundle']=='me.uiblueprint.labelstylefixture'
container=Path(subprocess.check_output(['xcrun','simctl','get_app_container',a.simulator,descriptor['bundle'],'data'],text=True,timeout=20).strip())
assert root.is_relative_to(container)
oracle=container/'tmp/label-oracle.json';deadline=time.monotonic()+10
while not oracle.exists() and time.monotonic()<deadline:time.sleep(.1)
expected=json.loads(oracle.read_bytes());assert all(expected['checks'].values())
out=Path(a.out);out.mkdir(mode=0o700)
w,_=r.request(a.session,'windows');assert len(w['windows'])==1;w=w['windows'][0]
response,legacy=r.request(a.session,'capture',w['window'],w['generation'],source='uikit')
assert b'label_style_' not in legacy and 'label_style' not in response
response,raw=r.request(a.session,'capture',w['window'],w['generation'],source='uikit',label_style=True)
assert response['label_style'] is True
assert all(v not in raw for v in (b'UIB_STYLE_PRIVATE_CANARY_913',b'UIB_ATTRIBUTED_PRIVATE_731'))
doc=json.loads(raw)['artifact']['data'];labels=[]
for node in doc['nodes']:
 values={d['name'].removeprefix('label_style_'):d['state'] for d in node['source_declarations'] if d['name'].startswith('label_style_')}
 if not values:continue
 assert node['native_role']['value']['value']=='UILabel'
 assert values['observation_id']['value']['value']==response['observation_id']
 assert values['effective_paint_font']['availability']=='unknown'
 assert values['effective_paint_color']['availability']=='unknown'
 labels.append((node,values))
assert len(labels)==3
for (node,style),exp in zip(labels,expected['labels']):
 for actual,key in [('font_name','font_name'),('font_family','family'),('font_point_size','size_pt'),('font_symbolic_traits','traits')]:
  assert style[actual]['value']['value']==exp[key],(actual,style[actual],exp)
 assert style['font_units']['value']['value']=='pt'
normal,fitted,pattern=[s for n,s in labels]
assert normal['font_point_size']['value']['value']==57
assert normal['trait_interface_style']['value']['value']==2
assert normal['text_color_space']['value']['value']=='extended-srgb'
assert all(abs(normal['text_color_'+c]['value']['value']-1)<1e-6 for c in 'rgba')
assert normal['highlighted_text_color_space']['reason']=='configured-color-absent'
assert fitted['adjusts_font_size_to_fit_width']['value']['value']==1
assert fitted['minimum_scale_factor']['value']['value']==expected['labels'][1]['minimum_scale_factor']
assert fitted['highlighted_text_color_space']['availability']=='known'
assert pattern['text_color_space']['reason']=='configured-color-conversion-unsupported'
for name,data in [('snapshot.json',raw),('legacy.json',legacy)]:
 r.publish(out/name,data)
 subprocess.run([str(Path(a.bundle)/'uiblueprint-validate'),'--max-bytes',str(r.MAX_BYTES),str(out/name)],check=True,capture_output=True,timeout=30)
# Cancellation retains the same terminal behavior with opt-in metadata.
request_id=str(uuid.uuid4());r.publish(root/(request_id+'.cancel'),b'cancel\n')
with patch.object(r.uuid,'uuid4',return_value=request_id):
 try:r.request(a.session,'capture',w['window'],w['generation'],source='uikit',label_style=True);raise AssertionError('cancel accepted')
 except r.Failure as e:assert str(e)=='cancelled'
assert not list(root.glob(request_id+'*'))
try:r.request(a.session,'capture',w['window'],'stale',source='uikit',label_style=True);raise AssertionError('stale accepted')
except r.Failure as e:assert str(e)=='staleWindow'
# Deliberately malformed wire fields must not be admitted/coerced by Swift.
for bad in (False,None,1,'true'):
 request_id=str(uuid.uuid4());payload=dict(version='1.0.0',id=request_id,run=descriptor['run_id'],capability=descriptor['capability'],operation='capture',window=w['window'],windowGeneration=w['generation'],source='uikit',labelStyle=bad)
 r.publish(root/(request_id+'.request.json'),r.encoded(payload));answer=root/(request_id+'.response.json');end=time.monotonic()+5
 while not answer.exists() and time.monotonic()<end:time.sleep(.02)
 result=json.loads(answer.read_bytes());assert result['status']=='failed' and 'snapshot' not in result
 answer.unlink()
response2,raw2=r.request(a.session,'capture',w['window'],w['generation'],source='uikit',label_style=True)
assert response2['observation_id']!=response['observation_id']
_,swiftui=r.request(a.session,'capture',w['window'],w['generation'])
assert b'label_style_' not in swiftui
summary=dict(status='passed',labels=len(labels),nodes=len(doc['nodes']),observation=response['observation_id'],fresh_observation=response2['observation_id'],configured_point_sizes=[s['font_point_size']['value']['value'] for n,s in labels],public_oracle_checks=expected['checks'],privacy_canaries_absent=True,default_unchanged=True,cancellation_stale_invalid_refused=True,paint_unknown=True)
r.publish(out/'summary.json',r.encoded(summary));print(json.dumps(summary))
