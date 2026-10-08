#!/usr/bin/env python3
"""Explicit own-F02 CLI scenario. Requires an already running, bound fixture.
No launch/input setup, permission changes, screenshot or external app access.
All generated commands/results are caller-owned system-temp files.
"""
import argparse, copy, json, os, pathlib, selectors, subprocess, tempfile, time
ROOT=pathlib.Path(__file__).resolve().parents[3]
def doc(kind,data):return {'schema_version':'0.1.0','artifact':{'kind':kind,'data':data}}
def main():
 p=argparse.ArgumentParser()
 for key in ('directory','identity','helper','cli','worker'):p.add_argument('--'+key,type=pathlib.Path,required=True)
 p.add_argument('--scenario',default='positive',choices=['positive','checked','secure','type','stale','owner','remount','parent-eof','form'])
 a=p.parse_args();assert a.directory.resolve().is_relative_to(pathlib.Path(tempfile.gettempdir()).resolve())
 a.directory.mkdir(exist_ok=True)
 binding=json.loads(a.identity.read_text());binding={k:v for k,v in binding.items() if k not in ('state','identity_version')}
 request=json.loads((ROOT/'fixtures/golden/ENV-REQUEST-VALID.json').read_text())['artifact']['data']
 context=request['context'];context['session_id']='m02-native-live';context['target']={'id':f"f02-pid-{binding['pid']}",'generation':binding['target_generation']};context['surfaces']=[{'id':f"window-{binding['window_id']}",'generation':binding['surface_generation']}]
 context['fields']=['role','accessibility_name','enabled','focused','checked','value','actions'];request['limits']={'max_elements':160,'max_depth':9,'max_output_bytes':524287,'deadline_ms':1000}
 ids=['f02.name','f02.secret','f02.enabled','f02.sample.a','f02.count','f02.complete','f02.apply','f02.result']
 config={'binding':binding,'identity_path':str(a.identity),'scope_id':'form-1','collection':'form','form_identifiers':ids,'form_session_ms':120000,'acquisition_limits':json.loads((ROOT/'tests/bridges/native/acquisition/profile.json').read_text())}
 mib=1048576
 limits=dict(workers=1,worker_bytes=64*mib,publication_reserve=mib,bootstrap_bytes=mib,parent_bytes=32*mib,input_bytes=2*mib,ingress_bytes=524288,output_bytes=524288,request_output_bytes=2*mib,completion_groups=2,control_bytes=4096,cleanup_ms=1000,retained_domain_bytes=64*mib,retained_per_worker=15*mib,main_stack_bytes=8*mib,watchdog_stack_bytes=mib)
 session=json.loads((ROOT/'fixtures/golden/ENV-SESSION-VALID.json').read_text())['artifact']['data']['session'];session.update(session_id=context['session_id'],target=context['target'],surfaces=context['surfaces'],allowed_scopes=['form-1'])
 connection=dict(connection_version='1.0.0',target=context['target'],session=session,host_limits=limits,attach_deadline_ms=3000,provider=dict(backend='native_fixture',helper_executable=str(a.helper),configuration=json.dumps(config,separators=(',',':')),channels=1))
 def save(name,data):
  path=a.directory/name;path.write_text(json.dumps(data,separators=(',',':'))+'\n');return str(path)
 path=save('connection.json',connection)
 process=subprocess.Popen([str(a.cli),'native-session','--connection',path,'--worker',str(a.worker),'--duration-ms','120000','--max-input-bytes',str(2*mib),'--max-output-bytes',str(524288)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
 sequence=0;records=[]
 def call(operation,source=None,expected=None):
  nonlocal sequence
  sequence+=1;r=copy.deepcopy(request);r['request_id']=f'm02-{sequence}';r['operation']=operation
  command={'request':save(f'request-{sequence}.json',doc('request',r))}
  if source:command['source']=save(f'source-{sequence}.json',source)
  if expected:command['expectation']=save(f'expectation-{sequence}.json',expected)
  process.stdin.write(json.dumps(command).encode()+b'\n');process.stdin.flush()
  poll=selectors.DefaultSelector();poll.register(process.stdout,selectors.EVENT_READ)
  ready=poll.select(5);poll.close()
  if not ready:raise AssertionError('no bounded reply')
  line=process.stdout.readline()
  if not line:raise AssertionError(('no canonical reply',process.poll(),process.stderr.read().decode()))
  result=json.loads(line);save(f'result-{sequence}.json',result);records.append(result);return result
 def observe():return call({'operation':'observe','channels':['external_semantics']})
 def snapshot(response):return response['artifact']['data']['result']['data']
 def state(snapshot,index,field):return next(p['state'] for p in snapshot['nodes'][index]['properties'] if p['field']==field)
 def action(snap,index,intent,field,value,result_index=None):
  observation=snap['observations'][0]['id'];key=snap['nodes'][index]['key']
  evidence={'observation_id':observation,'source_namespace':'macos.ax','provenance':'reported','method':'caller_unresolved','uncertainty':None}
  act=dict(id=f'action-{sequence}',context=snap['context'],backend_ref=dict(session_id=context['session_id'],key=key,snapshot_id=snap['id'],observation_id=observation,target=context['target'],surface=context['surfaces'][0]),intent=intent,modality='setter' if intent['intent'] in ('fill','set_checked') else 'keyboard' if intent['intent']=='type' else 'semantic',input_space=None,required_enabled=True,authorized_scope='form-1',unique_match=True,resolution=dict(available_intents=[],writable={'availability':'unknown','reason':'prepare_required'},value_allowed={'availability':'unknown','reason':'prepare_required'},evidence=evidence))
  expectation=json.loads((ROOT/'fixtures/golden/ENV-EXPECTATION-VALID.json').read_text())['artifact']['data'];expectation['targets']=[snap['nodes'][index if result_index is None else result_index]['key']];expectation['rule']={'relation':'property_equals','field':field,'expected':value}
  prepared=call({'operation':'prepare','action':act},doc('snapshot',snap),doc('expectation',expectation))
  if prepared['artifact']['kind']=='error':return prepared
  if a.scenario=='remount':
   (a.directory/'prepared.ready').write_text('ready')
   end=time.monotonic()+45
   while not (a.directory/'continue').exists():
    assert time.monotonic()<end,'bounded manual fixture transition';time.sleep(.05)
  plan=prepared['artifact']['data'];return call({'operation':'act','action':plan['action']},prepared,doc('expectation',expectation))
 try:
  first=observe();snap=snapshot(first)
  assert state(snap,1,'value')['availability']=='redacted'
  second=snapshot(observe());assert [n['key'] for n in snap['nodes']]==[n['key'] for n in second['nodes']]
  if a.scenario=='parent-eof':
   rows=subprocess.check_output(['ps','-axo','pid=,ppid='],text=True).splitlines()
   children=[int(row.split()[0]) for row in rows if int(row.split()[1])==process.pid]
   assert len(children)==2,'registered worker and resident helper must exist'
   process.kill();process.wait(timeout=3)
   end=time.monotonic()+3
   remaining=set(children)
   while remaining and time.monotonic()<end:
    for pid in list(remaining):
     try:os.kill(pid,0)
     except ProcessLookupError:remaining.remove(pid)
    time.sleep(.01)
   assert not remaining,'owned descendant survived parent EOF'
   report={'scenario':'parent-eof','owned_descendants_exited':len(children),'canonical_records':len(records),'mutation_calls':0}
   save('report.json',report);print(json.dumps(report));return
  if a.scenario=='positive':
   for index,intent,field,value,result in [(0,{'intent':'focus'},'focused',{'type':'flag','value':True},None),(0,{'intent':'fill','text':'Ada'},'value',{'type':'text','value':'Ada'},None),(3,{'intent':'activate'},'value',{'type':'text','value':'Count: 1'},4)]:
    outcome=action(second,index,intent,field,value,result);assert outcome['artifact']['kind']=='transition_context',outcome
    step=outcome['artifact']['data']['transition']['steps'][0];assert step['delivery']=='confirmed' and step['outcome']=='succeeded',step
    second=snapshot(observe())
  elif a.scenario=='form':
   steps=[(0,{'intent':'focus'},'focused',{'type':'flag','value':True},None),
          (0,{'intent':'fill','text':'Ada'},'value',{'type':'text','value':'Ada'},None),
          (5,{'intent':'activate'},'value',{'type':'text','value':'Ada Lovelace'},0),
          (2,{'intent':'activate'},'checked',{'type':'flag','value':True},2),
          (6,{'intent':'activate'},'value',{'type':'text','value':'Result: accepted-a'},7)]
   assert state(second,2,'checked')=={'availability':'known','value':{'type':'flag','value':False}}
   for index,intent,field,value,result in steps:
    outcome=action(second,index,intent,field,value,result)
    assert outcome['artifact']['kind']=='transition_context',outcome
    step=outcome['artifact']['data']['transition']['steps'][0]
    assert step['delivery']=='confirmed' and step['outcome']=='succeeded',step
    second=snapshot(observe())
  elif a.scenario=='checked':
   # No value setter means no SetChecked capability; AXPress is not a substitute.
   outcome=action(second,2,{'intent':'set_checked','value':True},'checked',{'type':'flag','value':True});assert outcome['artifact']['kind']=='error'
  elif a.scenario=='secure':
   outcome=action(second,1,{'intent':'fill','text':'SAFE_TEST_ONLY'},'value',{'type':'text','value':'SAFE_TEST_ONLY'});assert outcome['artifact']['kind']=='error'
  elif a.scenario=='stale':
   old=json.loads((a.directory.parent/'positive/result-1.json').read_text());old=snapshot(old)
   outcome=action(old,0,{'intent':'focus'},'focused',{'type':'flag','value':True});assert outcome['artifact']['data']['code']=='stale_target',outcome
  elif a.scenario=='remount':
   outcome=action(second,0,{'intent':'focus'},'focused',{'type':'flag','value':True});step=outcome['artifact']['data']['transition']['steps'][0];assert step['delivery']=='not_dispatched' and step['outcome']=='failed',step
  elif a.scenario=='owner':
   outcome=action(second,0,{'intent':'type','text':'Z'},'value',{'type':'text','value':'AdaZ'});assert outcome['artifact']['data']['code']=='interrupted'
  elif a.scenario=='type':
   outcome=action(second,0,{'intent':'focus'},'focused',{'type':'flag','value':True})
   assert outcome['artifact']['data']['transition']['steps'][0]['outcome']=='succeeded'
   second=snapshot(observe())
   outcome=action(second,0,{'intent':'type','text':'Z'},'value',{'type':'text','value':'AdaZ'});assert outcome['artifact']['data']['transition']['steps'][0]['outcome']=='action_outcome_unknown'
  process.stdin.close();code=process.wait(timeout=5)
  assert code==(0 if a.scenario in ('positive','form') else 4),(code,process.stderr.read().decode())
  assert 'M02_SYNTHETIC_CANARY' not in json.dumps(records)
  report={'scenario':a.scenario,'canonical_records':len(records),'exit':code,'fixture_pid':binding['pid'],'cleanup':'CLI exited; registered shutdown required','images':0}
  save('report.json',report);print(json.dumps(report))
 finally:
  if process.poll() is None:process.terminate();process.wait(timeout=5)
if __name__=='__main__':main()
