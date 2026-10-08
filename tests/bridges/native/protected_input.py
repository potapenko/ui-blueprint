#!/usr/bin/env python3
"""V02 own-fixture protected input; requires exclusive desktop and fresh binding.
No source values in argv/command records/output. Caller owns all inputs/results.
"""
import argparse, copy, json, os, pathlib, selectors, subprocess, tempfile, time
ROOT = pathlib.Path(__file__).resolve().parents[3]
def doc(kind, data): return {'schema_version': '0.1.0', 'artifact': {'kind': kind, 'data': data}}
def main():
    p = argparse.ArgumentParser()
    for key in ('directory', 'identity', 'helper', 'cli', 'worker', 'secret-file'):
        p.add_argument('--'+key, type=pathlib.Path, required=True)
    p.add_argument('--modality', choices=['setter'], required=True)
    p.add_argument('--scenario', choices=['input', 'source-error', 'cancel', 'reuse', 'wrong-ref'], default='input')
    a = p.parse_args()
    assert a.directory.resolve().is_relative_to(pathlib.Path(tempfile.gettempdir()).resolve())
    a.directory.mkdir(exist_ok=True)
    canary = a.secret_file.read_bytes()  # synthetic test oracle, never product input record
    def save(name, data):
        path = a.directory/name
        raw = json.dumps(data, separators=(',', ':')).encode()+b'\n'
        assert canary not in raw, 'canary in serialized channel'
        path.write_bytes(raw)
        return str(path)
    binding = json.loads(a.identity.read_text())
    binding = {k:v for k,v in binding.items() if k not in ('state', 'identity_version')}
    r = json.loads((ROOT/'fixtures/golden/ENV-REQUEST-VALID.json').read_text())['artifact']['data']
    context = r['context']
    context.update(session_id='v02-native-live', target={'id':f"f02-pid-{binding['pid']}", 'generation':binding['target_generation']}, surfaces=[{'id':f"window-{binding['window_id']}", 'generation':binding['surface_generation']}], fields=['role','accessibility_name','enabled','focused','value','actions'])
    r['limits'] = dict(max_elements=160, max_depth=9, max_output_bytes=524287, deadline_ms=1000)
    ids = ['f02.name', 'f02.secret', 'f02.secret-status']
    config = dict(binding=binding, identity_path=str(a.identity), scope_id='form-1', collection='form', form_identifiers=ids, form_session_ms=120000, acquisition_limits=json.loads((ROOT/'tests/bridges/native/acquisition/profile.json').read_text()), protected_input=dict(reference='opaque-once', action_id='protected-step', identifier='f02.secret', path=str(a.secret_file if a.scenario!='source-error' else a.directory/'missing-source'), trace=True))
    mib = 1048576
    limits = dict(workers=1,worker_bytes=64*mib,publication_reserve=mib,bootstrap_bytes=mib,parent_bytes=32*mib,input_bytes=2*mib,ingress_bytes=524288,output_bytes=524288,request_output_bytes=2*mib,completion_groups=2,control_bytes=4096,cleanup_ms=1000,retained_domain_bytes=64*mib,retained_per_worker=15*mib,main_stack_bytes=8*mib,watchdog_stack_bytes=mib)
    session = json.loads((ROOT/'fixtures/golden/ENV-SESSION-VALID.json').read_text())['artifact']['data']['session']
    session.update(session_id=context['session_id'],target=context['target'],surfaces=context['surfaces'],allowed_scopes=['form-1'])
    connection = dict(connection_version='1.0.0',target=context['target'],session=session,host_limits=limits,attach_deadline_ms=3000,provider=dict(backend='native_fixture',helper_executable=str(a.helper),configuration=json.dumps(config,separators=(',',':')),channels=1))
    process = subprocess.Popen([str(a.cli),'native-session','--connection',save('connection.json',connection),'--worker',str(a.worker),'--duration-ms','120000','--max-input-bytes',str(2*mib),'--max-output-bytes','524288'],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
    sequence = 0
    descendants = set()
    def call(op, source=None, expected=None):
        nonlocal sequence
        sequence += 1
        request = copy.deepcopy(r); request.update(request_id=f'v02-{sequence}',operation=op)
        command = {'request':save(f'request-{sequence}.json',doc('request',request))}
        if source is not None: command['source'] = save(f'source-{sequence}.json',source)
        if expected is not None: command['expectation'] = save(f'expectation-{sequence}.json',doc('expectation',expected))
        raw = json.dumps(command).encode()+b'\n'; assert canary not in raw
        with (a.directory/'commands.ndjson').open('ab') as stream:stream.write(raw)
        process.stdin.write(raw); process.stdin.flush()
        poll = selectors.DefaultSelector(); poll.register(process.stdout,selectors.EVENT_READ)
        ready = poll.select(5); poll.close(); assert ready, 'bounded reply absent'
        line = process.stdout.readline(); assert line, 'canonical reply absent'
        assert canary not in line, 'canary in stdout'
        value = json.loads(line); save(f'result-{sequence}.json',value); return value
    def observe(): return call({'operation':'observe','channels':['external_semantics']})['artifact']['data']['result']['data']
    def action(snap, index, intent, modality, result_index, field, value):
        evidence = dict(observation_id=snap['observations'][0]['id'],source_namespace='macos.ax',provenance='reported',method='caller_unresolved',uncertainty=None)
        act = dict(id='protected-step' if intent['intent']=='fill_secret' else 'focus-step',context=snap['context'],backend_ref=dict(session_id=context['session_id'],key=snap['nodes'][index]['key'],snapshot_id=snap['id'],observation_id=evidence['observation_id'],target=context['target'],surface=context['surfaces'][0]),intent=intent,modality=modality,input_space=None,required_enabled=True,authorized_scope='form-1',unique_match=True,resolution=dict(available_intents=[],writable={'availability':'unknown','reason':'prepare_required'},value_allowed={'availability':'unknown','reason':'prepare_required'},evidence=evidence))
        expected = dict(id='explicit-result',scope_id='form-1',targets=[snap['nodes'][result_index]['key']],rule={'relation':'property_equals','field':field,'expected':value},applies_when={'platform':None,'input_mode':None,'text_scale':None},expected_from='v02_fixture_public_presence')
        plan = call({'operation':'prepare','action':act},doc('snapshot',snap),expected)
        if plan['artifact']['kind']=='error':return plan
        if intent['intent']=='fill_secret' and a.scenario=='cancel':
            rows = subprocess.check_output(['ps','-axo','pid=,ppid='],text=True).splitlines()
            descendants.update(int(row.split()[0]) for row in rows if int(row.split()[1])==process.pid)
            assert len(descendants)==2, 'registered owners absent before cancel'
            process.terminate(); process.wait(timeout=4)
            end = time.monotonic()+3
            pending=set(descendants)
            while pending and time.monotonic()<end:
                for pid in list(pending):
                    try:os.kill(pid,0)
                    except ProcessLookupError:pending.remove(pid)
                time.sleep(.01)
            assert not pending, 'registered owner survived cancellation'
            return None
        return call({'operation':'act','action':plan['artifact']['data']['action']},plan,expected)
    try:
        snap = observe()
        save('before.json',doc('snapshot',snap))
        result = action(snap,1,{'intent':'focus'},'semantic',1,'focused',{'type':'flag','value':True})
        assert result['artifact']['kind']=='transition_context', 'focus refused'
        assert result['artifact']['data']['transition']['steps'][0]['outcome']=='succeeded', 'focus unverified'
        snap = observe()
        result = action(snap,1,{'intent':'fill_secret','secret_reference':'wrong-ref' if a.scenario=='wrong-ref' else 'opaque-once'},a.modality,2,'accessibility_name',{'type':'text','value':'Protected input: received'})
        if result is not None:
            save('protected-result.json',result)
            if result['artifact']['kind']=='transition_context':
                after = result['artifact']['data']['after']
                if after is not None:save('after.json',doc('snapshot',after))
            if a.scenario=='reuse':
                assert result['artifact']['data']['transition']['steps'][0]['outcome']=='succeeded', 'first delivery failed'
                again = action(observe(),1,{'intent':'fill_secret','secret_reference':'opaque-once'},a.modality,2,'accessibility_name',{'type':'text','value':'Protected input: received'})
                assert again['artifact']['kind']=='error', 'consumed reference reused'
                save('reuse-refusal.json',again)
            process.stdin.close(); code = process.wait(timeout=5)
        else:code = process.returncode
        error = process.stderr.read(); assert canary not in error, 'canary in stderr'
        (a.directory/'stderr.txt').write_bytes(error)
        # Details only from sanitized canonical records, never raw private values.
        report = dict(cancelled_owners=len(descendants), scenario=a.scenario, modality=a.modality, records=sequence, exit=code, canary_absent=True, caller_source_preserved=a.secret_file.exists())
        if result is not None:
            artifact = result['artifact']
            report['result_kind'] = artifact['kind']
            if artifact['kind']=='error':report.update(code=artifact['data']['code'],recovery=artifact['data']['recovery_class'])
            else:report['step'] = artifact['data']['transition']['steps'][0]
        if a.scenario=='input':
            assert code==0 and report['step']['delivery']=='confirmed' and report['step']['outcome']=='succeeded', 'protected input did not verify'
        elif a.scenario=='source-error':
            assert code==4 and report['step']['outcome']=='action_outcome_unknown', 'backend failure lost uncertainty'
        elif a.scenario=='wrong-ref':
            assert code==4 and report['result_kind']=='error', 'wrong secret binding accepted'
        elif a.scenario=='reuse':assert code==4, 'reuse did not stop session'
        else:assert code==-15 and len(descendants)==2, 'cancel lifecycle failed'
        save('report.json',report); print(json.dumps(report))
    finally:
        if process.poll() is None:process.terminate();process.wait(timeout=5)
if __name__ == '__main__':main()
