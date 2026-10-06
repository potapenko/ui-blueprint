#!/usr/bin/env python3
"""Focused M01 fault/isolation proof; no new real pixel attempts after denial."""
import argparse
import fcntl
import json
import os
import pathlib
import signal
import subprocess
import time
import uuid
import prove


def request_for(manifest):
    nonce = str(uuid.uuid4())
    context = {'schema_version': '0.1.0', 'session_id': f'capture-session-{nonce}',
               'target': {'id': f"f02-pid-{manifest['pid']}", 'generation': manifest['target_generation']},
               'surfaces': [{'id': f"window-{manifest['window_id']}", 'generation': manifest['surface_generation']}],
               'scope_id': f"fixture-window-{manifest['window_id']}-sample", 'projection': 'interaction',
               'fields': ['role', 'accessibility_name', 'enabled', 'accessibility_bounds'],
               'plugin': {'id': 'macos', 'version': '0.1.0'}, 'environment_revision': f'capture-environment-{nonce}'}
    return {'schema_version': '0.1.0', 'artifact': {'kind': 'request', 'data': {
        'clock_domain': f'capture-parent-{nonce}', 'request_id': f'capture-request-{nonce}', 'context': context,
        'limits': {'max_elements': 160, 'max_depth': 9, 'max_output_bytes': prove.FRAME, 'deadline_ms': 3000},
        'freshness_policy': 'current_required',
        'operation': {'operation': 'observe', 'channels': ['external_semantics', 'rendered_capture']}}}}


def bridge_case(args, mode):
    folder = args.output/mode; folder.mkdir()
    request = request_for(json.loads((args.fixtures/'a.json').read_text()))
    request_path = folder/'request.json'; request_path.write_bytes(prove.encoded(request))
    prove.validate(args.validator, request_path)
    environment = dict(os.environ, UIB_CAPTURE_LOCK_PATH=str(args.lock))
    descriptor = subprocess.run([str(args.collector), str(args.fixtures/'a.json'), str(folder), 'describe'],
                                input=prove.encoded(request), capture_output=True, timeout=2, env=environment)
    assert descriptor.returncode == 0
    session_path = folder/'session.json'; session_path.write_bytes(descriptor.stdout)
    prove.validate(args.validator, session_path)
    host_mode = {'stall': 'complete', 'failure': 'complete', 'cancel': 'cancel-after-first', 'detach': 'detach-after-first'}[mode]
    owned, readers, cleanup, events = [], [], [], []
    submitted = {}
    try:
        host = subprocess.Popen([str(args.host), str(session_path), str(request_path), str(prove.FRAME), '131072',
                                 host_mode, '1000', '8', str(folder/'retained')],
                                stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL)
        owned.append(host); hr = prove.Reader(host.stdout); readers.append(hr)
        ticket = json.loads(hr.get(2)); events.append(ticket); assert ticket['event'] == 'ticket'
        environment['UIB_CAPTURE_FAULT'] = 'failure' if mode == 'failure' else 'stall'
        collector = subprocess.Popen([str(args.collector), str(args.fixtures/'a.json'), str(folder), 'live', str(ticket['sequence'])],
                                     stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, env=environment)
        owned.append(collector); cr = prove.Reader(collector.stdout); readers.append(cr)
        collector.stdin.write(prove.encoded(request)); collector.stdin.close()
        ax = cr.get(1); ax_doc = json.loads(ax)
        assert ax_doc['artifact']['data']['channel'] == 'external_semantics'
        assert ax_doc['artifact']['data']['result']['status'] == 'observed'
        (folder/'submitted-ax.json').write_bytes(ax); prove.validate(args.validator, folder/'submitted-ax.json')
        submitted['external_semantics'] = ax_doc
        host.stdin.write(ax); host.stdin.flush()
        events.append(json.loads(hr.get(1))); assert events[-1]['event'] == 'frame_accepted'
        if mode in ('stall', 'failure'):
            # Callback deadline is 2s; allow a small bounded delivery margin while
            # the common parent's authoritative overall deadline stays 3s.
            capture = cr.get(2.2); doc = json.loads(capture)
            assert doc['artifact']['data']['result']['status'] == 'failed'
            expected = 'timeout' if mode == 'stall' else 'incomplete_scope'
            assert doc['artifact']['data']['result']['data']['code'] == expected
            (folder/'submitted-capture.json').write_bytes(capture)
            prove.validate(args.validator, folder/'submitted-capture.json')
            submitted['rendered_capture'] = doc
            host.stdin.write(capture); host.stdin.flush()
            events.append(json.loads(hr.get(1))); assert events[-1]['event'] == 'frame_accepted'
        terminal = json.loads(hr.get(1)); events.append(terminal)
        assert terminal['terminal'] == {'stall':'Completed','failure':'Completed','cancel':'Cancelled','detach':'Detached'}[mode]
        if mode in ('cancel', 'detach'):
            cleanup.append(prove.reap(collector))
            host.stdin.write(ax); host.stdin.flush()
            events.append(json.loads(hr.get(1))); assert events[-1]['event'] == 'late_probe' and events[-1]['rejected']
        host.stdin.close(); assert host.wait(timeout=1) == 0
        if mode in ('stall', 'failure'): assert collector.wait(timeout=1) == 0
        retained = [json.loads(p.read_bytes()) for p in (folder/'retained').glob('channel-*.json')]
        assert len(retained) == len(submitted)
        for doc in retained: assert doc == submitted[doc['artifact']['data']['channel']]
        result = {'condition':'live AX; injected capture fault/control; no capture API invoked',
                  'mode':mode,'retained_equals_submitted':True,'events':events}
    finally:
        for child in reversed(owned):
            if not any(x['pid'] == child.pid for x in cleanup): cleanup.append(prove.reap(child))
        for reader in reversed(readers): reader.finish()
    result['cleanup'] = cleanup; result['readers_joined'] = True
    (folder/'result.json').write_text(json.dumps(result,indent=2)+'\n')
    return result


def isolation(args):
    folder=args.output/'ax-isolation'; folder.mkdir(); (folder/'a').mkdir(); (folder/'b').mkdir()
    env=dict(os.environ,UIB_CAPTURE_LOCK_PATH=str(args.lock),UIB_CAPTURE_FAULT='stall')
    start=time.monotonic()
    a=subprocess.Popen([str(args.observe),str(args.fixtures/'a.json'),str(folder/'a'),'1'],
                       stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,env=env)
    b=None; cleanup=[]
    try:
        # Bounded wait for this request's completion/lock evidence, not UI polling.
        held=False
        while time.monotonic()-start < 1:
            if (folder/'a/ax-0.json').exists() and args.lock.exists():
                with args.lock.open('rb') as check:
                    try: fcntl.flock(check,fcntl.LOCK_EX|fcntl.LOCK_NB); fcntl.flock(check,fcntl.LOCK_UN)
                    except BlockingIOError: held=True
                if held:break
            time.sleep(.005)
        assert held, 'capture stall did not acquire its scoped resource'
        env=dict(os.environ,UIB_CAPTURE_LOCK_PATH=str(args.lock),UIB_AX_ONLY='1')
        b_start=time.monotonic()
        b=subprocess.Popen([str(args.observe),str(args.fixtures/'b.json'),str(folder/'b'),'1'],
                           stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,env=env)
        assert b.wait(timeout=1)==0
        b_ms=round((time.monotonic()-b_start)*1000)
        assert a.poll() is None, 'stalled capture finished before the independent AX proof'
        assert a.wait(timeout=max(.01,3-(time.monotonic()-start)))==2
        for role in ('a','b'):
            data=json.loads((folder/role/'ax-0.json').read_text())
            assert data['external_semantics']['status']=='observed' and data['external_semantics']['nodes']
        cap=json.loads((folder/'a/external-0.json').read_text())['rendered_capture']
        assert cap['code']=='timeout' and cap['failed_step']=='injected_capture_stall'
        result={'injected_capture_stall_a':True,'live_unrelated_ax_b':True,'b_ax_ms':b_ms,
                'b_completed_while_a_capture_lease_held':True,'a_ax_preserved':True,'capture_a':cap}
    finally:
        for child in (b,a):
            if child is not None:cleanup.append(prove.reap(child))
    with args.lock.open('rb') as check:
        fcntl.flock(check,fcntl.LOCK_EX|fcntl.LOCK_NB);fcntl.flock(check,fcntl.LOCK_UN)
    result['lease_released_after_reap']=True;result['cleanup']=cleanup
    (folder/'result.json').write_text(json.dumps(result,indent=2)+'\n')
    return result


def main():
    parser=argparse.ArgumentParser()
    for name in ('observe','collector','host','validator','fixtures','lock','output'):
        parser.add_argument('--'+name,type=pathlib.Path,required=True)
    args=parser.parse_args();args.output.mkdir(parents=True,exist_ok=False)
    tests=json.loads(subprocess.check_output([str(args.observe),'--gate-checks'],timeout=3))
    assert all(x.get('outcome_matches',x.get('permission_required',False)) for x in tests)
    assert all(x.get('late_rejected',True) for x in tests)
    (args.output/'gate-checks.json').write_text(json.dumps(tests,indent=2)+'\n')
    isolation_result=isolation(args)
    cases=[bridge_case(args,mode) for mode in ('stall','failure','cancel','detach')]
    result={'support_revision':'73d772e97efcf550ea4a4d3e8480b56509ebc548',
            'gate_checks':'passed; injected terminal/late replies',
            'isolation':isolation_result,'common_cases':[{'mode':x['mode'],'retained_equals_submitted':True,'readers_joined':True} for x in cases],
            'real_pixel_calls_in_this_driver':0,'live_parallel_capture_claim':False,
            'remaining':'B real capture permission_required; no retry/backend/permission change'}
    (args.output/'result.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))


if __name__=='__main__':
    def timeout(_s,_f):raise TimeoutError('M01 whole proof watchdog')
    signal.signal(signal.SIGALRM,timeout);signal.alarm(30)
    try:main()
    finally:signal.alarm(0)
