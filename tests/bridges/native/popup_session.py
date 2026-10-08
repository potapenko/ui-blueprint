#!/usr/bin/env python3
"""Explicit shipping CLI popup steps on an already bound own F02 fixture.
Setup/publication and independent UI observation are external to this consumer.
No fixture launch, synthetic input backend, permission changes or inferred refs.
"""
import argparse
import copy
import json
import pathlib
import selectors
import subprocess
import tempfile
import time

ROOT = pathlib.Path(__file__).resolve().parents[3]
BINDING = ('pid', 'bundle_id', 'launch_time', 'window_id', 'window_identifier', 'target_generation', 'surface_generation')


def doc(kind, data):
    return {'schema_version': '0.1.0', 'artifact': {'kind': kind, 'data': data}}


def main():
    p = argparse.ArgumentParser(description=__doc__)
    for key in ('directory', 'manifest', 'helper', 'cli', 'worker'):
        p.add_argument('--' + key, type=pathlib.Path, required=True)
    p.add_argument('--step', choices=['open', 'resize', 'confirm'], required=True)
    p.add_argument('--reopen-barrier', action='store_true')
    a = p.parse_args()
    assert a.directory.resolve().is_relative_to(pathlib.Path(tempfile.gettempdir()).resolve())
    a.directory.mkdir(exist_ok=False)
    manifest = json.loads(a.manifest.read_text())
    parent = {k: manifest[k] for k in BINDING}
    popup = a.step == 'confirm'
    binding = {k: manifest['popup_binding'][k] for k in BINDING} if popup else parent
    request = json.loads((ROOT/'fixtures/golden/ENV-REQUEST-VALID.json').read_text())['artifact']['data']
    context = request['context']
    context.update(session_id='n03-' + a.directory.name, target={'id': f"f02-pid-{binding['pid']}", 'generation': binding['target_generation']},
                   surfaces=[{'id': f"window-{binding['window_id']}", 'generation': binding['surface_generation']}],
                   fields=['role', 'accessibility_name', 'enabled', 'focused', 'value', 'actions', 'accessibility_bounds'])
    request['limits'] = dict(max_elements=160, max_depth=9, max_output_bytes=524287, deadline_ms=1000)
    config = dict(binding=binding, identity_path=manifest['popup_binding']['identity_path'] if popup else manifest['identity_path'],
                  scope_id='form-1', collection='form', form_identifiers=['f02.popup.confirm'] if popup else ['f02.popup', 'f02.resize', 'f02.result', 'f02.name'],
                  form_session_ms=120000, acquisition_limits=json.loads((ROOT/'tests/bridges/native/acquisition/profile.json').read_text()))
    if popup:
        context['surfaces'].append({'id': f"window-{parent['window_id']}", 'generation': parent['surface_generation']})
        config.update(parent_binding=parent, parent_identity_path=manifest['identity_path'], parent_form_identifiers=['f02.popup', 'f02.result'])
    mib = 1048576
    limits = dict(workers=1, worker_bytes=64*mib, publication_reserve=mib, bootstrap_bytes=mib, parent_bytes=32*mib, input_bytes=2*mib,
                  ingress_bytes=524288, output_bytes=524288, request_output_bytes=2*mib, completion_groups=2, control_bytes=4096,
                  cleanup_ms=1000, retained_domain_bytes=64*mib, retained_per_worker=15*mib, main_stack_bytes=8*mib, watchdog_stack_bytes=mib)
    session = json.loads((ROOT/'fixtures/golden/ENV-SESSION-VALID.json').read_text())['artifact']['data']['session']
    session.update(session_id=context['session_id'], target=context['target'], surfaces=context['surfaces'], allowed_scopes=['form-1'])
    connection = dict(connection_version='1.0.0', target=context['target'], session=session, host_limits=limits, attach_deadline_ms=3000,
                      provider=dict(backend='native_fixture', helper_executable=str(a.helper), configuration=json.dumps(config, separators=(',', ':')), channels=1))

    def save(name, data):
        path = a.directory/name
        path.write_text(json.dumps(data, separators=(',', ':'))+'\n')
        return str(path)

    process = subprocess.Popen([str(a.cli), 'native-session', '--connection', save('connection.json', connection), '--worker', str(a.worker),
                                '--duration-ms', '120000', '--max-input-bytes', str(2*mib), '--max-output-bytes', '524288'],
                               stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    sequence = 0

    def call(operation, source=None, expectation=None):
        nonlocal sequence
        sequence += 1
        r = copy.deepcopy(request)
        r.update(request_id=f'n03-{sequence}', operation=operation)
        command = {'request': save(f'request-{sequence}.json', doc('request', r))}
        if source:
            command['source'] = save(f'source-{sequence}.json', source)
            command['expectation'] = save(f'expectation-{sequence}.json', expectation)
        process.stdin.write(json.dumps(command).encode()+b'\n')
        process.stdin.flush()
        with selectors.DefaultSelector() as poll:
            poll.register(process.stdout, selectors.EVENT_READ)
            assert poll.select(5), 'bounded CLI reply missing'
        line = process.stdout.readline()
        assert line, ('canonical reply missing', process.poll(), process.stderr.read().decode())
        result = json.loads(line)
        save(f'result-{sequence}.json', result)
        return result

    try:
        observed = call({'operation': 'observe', 'channels': ['external_semantics']})
        snap = observed['artifact']['data']['result']['data']
        actor_index = 1 if a.step == 'resize' else 0
        actor, result = snap['nodes'][actor_index], snap['nodes'][2]
        evidence = dict(observation_id=snap['observations'][0]['id'], source_namespace='macos.ax', provenance='reported', method='caller_unresolved', uncertainty=None)
        action = dict(id='n03-'+a.step, context=context, backend_ref=dict(session_id=context['session_id'], key=actor['key'], snapshot_id=snap['id'],
                      observation_id=snap['observations'][0]['id'], target=context['target'], surface=actor['surface']), intent={'intent': 'activate'},
                      modality='semantic', input_space=None, required_enabled=True, authorized_scope='form-1', unique_match=True,
                      resolution=dict(available_intents=[], writable={'availability': 'unknown', 'reason': 'prepare_required'},
                                      value_allowed={'availability': 'unknown', 'reason': 'prepare_required'}, evidence=evidence))
        expected = json.loads((ROOT/'fixtures/golden/ENV-EXPECTATION-VALID.json').read_text())['artifact']['data']
        expected['targets'] = [result['key']]
        # For open/resize this checks unchanged public parent state only. The
        # independently observed UI + explicit manifest prove popup/resize effects.
        expected['rule'] = dict(relation='property_equals', field='value', expected={'type': 'text', 'value':
            'Result: popup-'+parent['window_identifier'] if popup else 'Result: '+manifest['state']['applied']})
        expectation = doc('expectation', expected)
        if a.step == 'open':
            focus = copy.deepcopy(action)
            focus.update(id='n03-focus', intent={'intent': 'focus'})
            focus['backend_ref']['key'] = snap['nodes'][3]['key']
            focused = copy.deepcopy(expected)
            focused.update(targets=[snap['nodes'][3]['key']], rule=dict(relation='property_equals', field='focused', expected={'type': 'flag', 'value': True}))
            focus_expectation = doc('expectation', focused)
            focus_plan = call({'operation': 'prepare', 'action': focus}, observed, focus_expectation)
            assert focus_plan['artifact']['kind'] == 'action', focus_plan
            focus_result = call({'operation': 'act', 'action': focus_plan['artifact']['data']['action']}, focus_plan, focus_expectation)
            assert focus_result['artifact']['data']['transition']['steps'][0]['outcome'] == 'succeeded'
            observed = call({'operation': 'observe', 'channels': ['external_semantics']})
            snap = observed['artifact']['data']['result']['data']
            action['backend_ref'].update(snapshot_id=snap['id'], observation_id=snap['observations'][0]['id'])
            action['resolution']['evidence']['observation_id'] = snap['observations'][0]['id']
        prepared = call({'operation': 'prepare', 'action': action}, observed, expectation)
        assert prepared['artifact']['kind'] == 'action', prepared
        plan = prepared['artifact']['data']['action']
        outcome = call({'operation': 'act', 'action': plan}, prepared, expectation)
        assert outcome['artifact']['kind'] == 'transition_context', outcome
        step = outcome['artifact']['data']['transition']['steps'][0]
        assert step['delivery'] == 'confirmed' and step['outcome'] == 'succeeded', step
        if popup:
            after = outcome['artifact']['data']['after']
            assert [n['key'] for n in after['nodes']] == [result['key']]
            assert [s['identity'] for s in after['surface_records']] == [context['surfaces'][1]]
            if a.reopen_barrier:
                (a.directory/'reopen.ready').write_text('confirmed; waiting for explicit product reopen and independent observation')
                end = time.monotonic()+90
                while not (a.directory/'continue').exists():
                    assert time.monotonic() < end, 'reopen observation deadline'
                    time.sleep(.05)
            stale = call({'operation': 'act', 'action': plan}, prepared, expectation)
            denied = stale['artifact']['data']['transition']['steps'][0]
            assert denied['delivery'] == 'not_dispatched' and denied['outcome'] == 'failed', denied
        process.stdin.close()
        code = process.wait(timeout=5)
        assert code == (4 if popup else 0), (code, process.stderr.read().decode())
        report = dict(step=a.step, records=sequence, exit=code, delivery=step['delivery'], outcome=step['outcome'],
                      retired_ref_refused=popup, parent_result_only=popup, setup_is_external=True)
        save('report.json', report)
        print(json.dumps(report))
    finally:
        if process.poll() is None:
            process.terminate()
            process.wait(timeout=5)


if __name__ == '__main__':
    main()
