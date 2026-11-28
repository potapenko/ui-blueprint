#!/usr/bin/env python3
"""One owned font fixture phase; public environment oracle and canonical capture."""
import argparse,importlib.util,json,subprocess,time,uuid
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch
p=argparse.ArgumentParser(description=__doc__)
for key in ('session','simulator','out','bundle'):p.add_argument('--'+key,required=True)
p.add_argument('--phase',type=int,choices=range(4),default=0)
p.add_argument('--probe',choices=['present','absent'],required=True)
a=p.parse_args();repo=Path(__file__).resolve().parents[2]
def module(name,path):
 s=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
r=module('runtime',repo/'plugins/macos/swiftui_runtime.py');sim=module('simulator',repo/'plugins/ios/swiftui_simulator.py')
root,d=r.endpoint(a.session);assert d['bundle']=='me.uiblueprint.fontresolutionfixture'
c=Path(sim.simctl('get_app_container',a.simulator,d['bundle'],'data')).resolve();assert root.is_relative_to(c)
(c/'tmp/phase').write_text(str(a.phase));oracle=c/'tmp/font-oracle.json';end=time.monotonic()+15
while time.monotonic()<end:
 (c/'tmp/oracle.request').touch();time.sleep(.2)
 if oracle.exists():
  o=json.loads(oracle.read_bytes())
  if o['phase']==a.phase and o['point_size']>0 and o['frame'][2]>0 and (a.phase!=1 or o['context_changes']>0):break
else:raise AssertionError('public context/layout oracle not ready')
w,_=r.request(a.session,'windows');assert len(w['windows'])==1;w=w['windows'][0]
assert w['platform']=='tvos-simulator'
out=Path(a.out)
result=sim.pair(SimpleNamespace(session=a.session,simulator=a.simulator,out=str(out),window=w['window'],timeout=120,source='uikit',label_style=False,font_resolution=True),r)
raw=(out/'snapshot.json').read_bytes();assert b'Font fixture canary 573' not in raw
nodes=json.loads(raw)['artifact']['data']['nodes'];probes=[]
for n in nodes:
 s={v['name'].removeprefix('font_resolution_'):v['state'] for v in n['source_declarations'] if v['name'].startswith('font_resolution_')}
 if s:
  assert s['role']['value']['value']=='fixture.stable-title'
  assert s['point_size']['value']['value']==o['point_size']
  assert s['postscript_name']['value']['value']==o['postscript_name']
  assert s['effective_paint_font']['availability']=='unknown'
  assert s['observation_id']['value']['value']==result['observation_id']
  probes.append(dict(key=n['key'],mount=s['mount_id']['value']['value'],context=s['context_revision']['value']['value'],point_size=s['point_size']['value']['value'],name=s['postscript_name']['value']['value']))
assert len(probes)==(1 if a.probe=='present' else 0),probes
_,plain=r.request(a.session,'capture',w['window'],w['generation'],source='uikit')
assert b'font_resolution_' not in plain
_,sw=r.request(a.session,'capture',w['window'],w['generation'])
assert b'font_resolution_' not in sw
subprocess.run([str(Path(a.bundle)/'uiblueprint-validate'),'--max-bytes',str(r.MAX_BYTES),str(out/'snapshot.json')],check=True,capture_output=True,timeout=30)
for source in ['stale']:
 try:r.request(a.session,'capture',w['window'],source,source='uikit',font_resolution=True);raise AssertionError('stale accepted')
 except r.Failure as e:assert str(e)=='staleWindow'
id=str(uuid.uuid4());r.publish(root/(id+'.cancel'),b'cancel\n')
with patch.object(r.uuid,'uuid4',return_value=id):
 try:r.request(a.session,'capture',w['window'],w['generation'],source='uikit',font_resolution=True);raise AssertionError('cancel accepted')
 except r.Failure as e:assert str(e)=='cancelled'
assert not list(root.glob(id+'*'))
summary=dict(status='passed',phase=a.phase,oracle=o,probes=probes,observation=result['observation_id'],nodes=len(nodes),default_style_absent=True,canary_absent=True,stale_cancel_refused=True)
(out/'summary.json').write_bytes(r.encoded(summary));print(json.dumps(summary))
