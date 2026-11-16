#!/usr/bin/env python3
"""Focused privacy/canonical/refusal checks; no product app is contacted."""
import sys
sys.dont_write_bytecode = True
import copy
import json
from pathlib import Path
import subprocess
import shutil
import os
import signal
import time
import tempfile
import unittest
from types import SimpleNamespace
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT/'plugins/macos'))
import swiftui_debug_snapshot as codec
import swiftui_debug_capture as capture

CANARY = 'PRIVATE_MODEL_VALUE_71cfe9'
META = dict(target=dict(id='macos-pid-123', generation='123:1794836979.5'))
REQ = dict(request_version='1.0.0', request_id='own-fixture', pid=123,
           expected_bundle='me.uiblueprint.external-layout.canary.unit', expected_launch_time=1794836979.5,
           window_id=456, limits=dict(max_nodes=256, max_depth=256,
                                     max_output_bytes=1048576, deadline_ms=10000))


def node(type_name='SwiftUI.Text', position=(16, 48), size=(80, 300), children=()):
    return dict(properties=[
        dict(id=0, attribute=dict(type=type_name)),
        dict(id=1, attribute=dict(value=CANARY, subattributes=[dict(name='token', value=CANARY)])),
        dict(id=2, attribute=dict(value=dict(items=[dict(secret=CANARY)]))),
        dict(id=3, attribute=dict(type='__C.CGPoint', value=list(position))),
        dict(id=4, attribute=dict(type='__C.CGSize', value=list(size)))], children=list(children),
        extra=dict(secret=CANARY))


class CanonicalPrivacy(unittest.TestCase):
    def encode(self, source, req=None):
        return codec.snapshot(json.dumps(source).encode(), req or REQ, META, 100.25, 100.5)

    def test_allowlist_keeps_hierarchy_and_exact_fractional_source_values(self):
        raw = self.encode([node(children=[node(position=(-0.25, 3.125), size=(83.5, 16.25))])])
        self.assertNotIn(CANARY.encode(), raw)
        self.assertNotIn(b'subattributes', raw)
        doc = json.loads(raw)['artifact']['data']
        first, second = doc['nodes']
        self.assertEqual(first['children'], [second['key']])
        geometry = second['properties'][0]['state']['value']['value']
        self.assertEqual(geometry['shape']['value'], dict(x=-0.25, y=3.125, width=83.5, height=16.25))
        self.assertEqual(geometry['transform']['status'], 'unknown')
        self.assertNotEqual(geometry['coordinate_space']['id'],
                            first['properties'][0]['state']['value']['value']['coordinate_space']['id'])
        self.assertIsNone(doc['source_state'])
        self.assertEqual(doc['coverage']['status'], 'partial')
        self.assertEqual(doc['observations'][0]['freshness'], 'unverified')

    def test_validator_and_installed_measure_read_real_canonical_shape(self):
        validator = Path('/Users/eugenepotapenko/.local/bin/uiblueprint-validate')
        self.assertTrue(validator.exists())
        with tempfile.TemporaryDirectory(prefix='uib-debug-canonical-') as directory:
            path = Path(directory)/'snapshot.json'
            path.write_bytes(self.encode([node()]))
            result = subprocess.run([str(validator), '--max-bytes', '1048576', str(path)],
                                    capture_output=True, timeout=5)
            self.assertEqual(result.returncode, 0, result.stdout)
            cli = Path('/Users/eugenepotapenko/.local/bin/uiblueprint')
            document = json.loads(path.read_bytes())['artifact']['data']
            result = subprocess.run([str(cli), 'inspect', '--snapshot', str(path), '--ref',
                json.dumps(document['nodes'][0]['key']),
                '--view', 'design', '--max-input-bytes', '1048576', '--max-output-bytes', '1048576'],
                capture_output=True, timeout=5)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertNotIn(CANARY.encode(), result.stdout+result.stderr)
            document = json.loads(path.read_bytes())['artifact']['data']
            target = document['nodes'][0]
            space = target['properties'][0]['state']['value']['value']['coordinate_space']
            query = dict(schema_version='0.2.0', artifact=dict(kind='geometry_query', data=dict(
                id='width', scope_id=document['context']['scope_id'], targets=[target['key']],
                operation='width', anchors=[dict(element=target['key'], frame_kind='layout_bounds',
                    coordinate_space=space, fraction=0, axis='x')], quantity_kind='length', units='pt',
                applies_when=dict(platform=None, text_scale=None, input_mode=None))))
            query_path = Path(directory)/'width.json'; query_path.write_text(json.dumps(query))
            measured = subprocess.run([str(cli), 'measure', '--snapshot', str(path), '--query',
                str(query_path), '--space', space['id'], '--max-input-bytes', '1048576',
                '--max-output-bytes', '1048576', '--json'], capture_output=True, timeout=5)
            self.assertEqual(measured.returncode, 0, measured.stderr)
            result = json.loads(measured.stdout)['artifact']['data']['result']
            self.assertEqual(result['status'], 'known')
            self.assertEqual(result['measurement']['value']['value']['amount'], 80)

    def test_reused_caller_id_cannot_reuse_observation_or_node_identity(self):
        first = json.loads(self.encode([node()]))['artifact']['data']
        second = json.loads(self.encode([node()]))['artifact']['data']
        self.assertNotEqual(first['id'], second['id'])
        self.assertNotEqual(first['nodes'][0]['key'], second['nodes'][0]['key'])
        self.assertNotEqual(first['context']['surfaces'], second['context']['surfaces'])
        self.assertEqual(first['context']['target'], second['context']['target'])

    def test_missing_layout_is_unknown_and_never_zero(self):
        item = node(); item['properties'] = item['properties'][:3]
        value = json.loads(self.encode([item]))['artifact']['data']['nodes'][0]['properties'][0]['state']
        self.assertEqual(value['availability'], 'unknown')
        self.assertNotIn('value', value)

    def test_malformed_geometry_limits_and_duplicates_are_fixed_errors(self):
        cases = []
        item = node(); item['properties'][4]['attribute']['value'] = [-1, 30]; cases.append([item])
        item = node(); item['properties'][3]['attribute']['value'] = [True, 30]; cases.append([item])
        item = node(); item['properties'].append(copy.deepcopy(item['properties'][0])); cases.append([item])
        item = node(); item['children'] = CANARY; cases.append([item])
        cases.append([])
        for source in cases:
            with self.subTest(source=type(source)), self.assertRaises(codec.Refusal) as error:
                self.encode(source)
            self.assertNotIn(CANARY, str(error.exception))
        for name, cap in [('max_nodes', 1), ('max_depth', 1), ('max_output_bytes', 1)]:
            req = copy.deepcopy(REQ); req['limits'][name] = cap
            with self.assertRaises(codec.Refusal):
                self.encode([node(children=[node()])], req)
        for raw in (b'{"x":1,"x":2}', b'[NaN]', b'[1e999]', b'[-1e999]', b'x'*(1048576+1)):
            with self.assertRaises(codec.Refusal) as error:
                codec.decode(raw)
            self.assertNotIn(CANARY, str(error.exception))
        ignored_overflow = json.dumps([node()]).encode().replace(json.dumps(CANARY).encode(), b'1e999')
        with self.assertRaises(codec.Refusal) as error:
            codec.snapshot(ignored_overflow, REQ, META, 1, 2)
        self.assertEqual(error.exception.code, 'nonfinite_json')

    def test_request_and_argument_errors_do_not_echo_private_input(self):
        req = copy.deepcopy(REQ); req['pid'] = CANARY
        with self.assertRaises(codec.Refusal) as error:
            codec.request(json.dumps(req).encode())
        self.assertEqual(str(error.exception), 'invalid_identity')
        result = subprocess.run([shutil.which('python3'), '-B', str(ROOT/'plugins/macos/swiftui_debug_capture.py'),
                                  '--private-'+CANARY], capture_output=True, timeout=5)
        self.assertNotIn(CANARY.encode(), result.stdout+result.stderr)
        self.assertNotEqual(result.returncode, 0)

    def test_output_conflict_and_private_directory_protect_foreign_files(self):
        with tempfile.TemporaryDirectory(prefix='uib-debug-output-') as directory:
            path = Path(directory)
            fd = capture.output_directory(path, REQ)
            try:
                capture.write_new(fd, 'snapshot-own-fixture.json', b'foreign-owned')
            finally:
                capture.os.close(fd)
            with self.assertRaises(codec.Refusal): capture.output_directory(path, REQ)
            self.assertEqual((path/'snapshot-own-fixture.json').read_bytes(), b'foreign-owned')
            (path/'snapshot-own-fixture.json').unlink()
            path.chmod(0o755)
            with self.assertRaises(codec.Refusal): capture.output_directory(path, REQ)
            path.chmod(0o700)

    def test_interrupted_call_never_claims_recovery_or_resumes_target(self):
        error = SimpleNamespace(Fail=lambda: False)
        detached = []
        frame = SimpleNamespace(GetFunctionName=lambda: 'mach_msg2_trap')
        thread = SimpleNamespace(GetQueueName=lambda: 'com.apple.main-thread', GetFrameAtIndex=lambda _: frame)
        process = SimpleNamespace(IsValid=lambda: True, GetProcessID=lambda: 123,
            GetThreadAtIndex=lambda _: thread, GetState=lambda: 1,
            Detach=lambda keep: (detached.append(keep) or error))
        target = SimpleNamespace(AttachToProcessWithID=lambda *args: process)
        debugger = SimpleNamespace(SetAsync=lambda _: None, HandleCommand=lambda _: None,
            CreateTarget=lambda _: target, GetListener=lambda: None)
        lldb = SimpleNamespace(SBError=lambda: error, eStateStopped=1, eStateDetached=2)
        def refused(): raise codec.Refusal('call_interrupted_owner_recovery')
        calls = SimpleNamespace(initialize=refused, close=lambda: False, count=1)
        with tempfile.TemporaryDirectory(prefix='uib-debug-recovery-') as directory:
            out = Path(directory); req = out/'request.json'; req.write_text(json.dumps(REQ))
            with patch.dict(sys.modules, lldb=lldb), patch.object(capture, 'verified_helper', return_value=None), \
                    patch.object(capture, 'metadata', return_value=META), patch.object(capture, 'Calls', return_value=calls), \
                    patch.object(capture, 'retain_debugger', return_value='external_target_exit') as hold:
                capture.capture(debugger, str(req), '', str(out))
            result = json.loads((out/'manifest-own-fixture.json').read_bytes())
            self.assertEqual(result['cleanup'], 'external_target_exit')
            hold.assert_called_once()
            self.assertEqual(result['status'], 'failed')
            self.assertEqual(detached, [])
            self.assertFalse((out/'snapshot-own-fixture.json').exists())
        # The in-flight cleanup branch performs no register writes, retry or kill.
        bare = object.__new__(capture.Calls); bare.inflight = True
        self.assertFalse(bare.close())

    def test_acquisition_budget_reserves_pool_cleanup_inside_total_cap(self):
        calls = object.__new__(capture.Calls)
        calls.deadline = float('inf'); calls.count = 254
        calls.owned_array = 9; calls.pool = 1; calls.memory = None; calls.inflight = calls.altered = False
        calls.pc = 100; calls.sp = 4096; calls.return_pc = 150; calls.return_sp = 3968; calls.suspended = []
        error = SimpleNamespace(Fail=lambda: False)
        frame = SimpleNamespace(GetPC=lambda: 150, GetSP=lambda: 3968,
            FindRegister=lambda _: SimpleNamespace(GetValueAsUnsigned=lambda: 0))
        calls.thread = SimpleNamespace(GetFrameAtIndex=lambda _: frame, GetStopReason=lambda: 3,
                                        GetStopReasonDataAtIndex=lambda _: 7)
        calls.bp = SimpleNamespace(GetID=lambda: 7)
        calls.t = SimpleNamespace(BreakpointDelete=lambda _: True)
        calls.p = SimpleNamespace(Continue=lambda: error, GetState=lambda: 1)
        calls.d = SimpleNamespace(SetAsync=lambda _: None)
        calls.listener = SimpleNamespace(GetNextEvent=lambda _: False, WaitForEvent=lambda *_: True)
        calls.lldb = SimpleNamespace(SBEvent=lambda: None, eStateStopped=1, eStateExited=2,
            eStateCrashed=4, eStopReasonBreakpoint=3,
            SBProcess=SimpleNamespace(EventIsProcessEvent=lambda _: True, GetStateFromEvent=lambda _: 1))
        invoked = []
        calls.symbol = lambda name: (invoked.append(name) or 200)
        calls.setreg = lambda *_: None
        calls.restore = lambda: setattr(calls, 'altered', False)
        with self.assertRaises(codec.Refusal) as refused:
            calls.call('objc_msgSend', 1, 2)
        self.assertEqual(refused.exception.code, 'call_limit')
        self.assertEqual(invoked, [])
        self.assertTrue(calls.close())
        self.assertEqual(invoked, ['swift_release', 'objc_autoreleasePoolPop'])
        self.assertIsNone(calls.owned_array)
        self.assertEqual(calls.count, 256)
        self.assertIsNone(calls.pool)

        # A fractional remaining budget must never enter WaitForEvent(1).
        calls.count = 0; calls.deadline = 10.25
        next_events = iter([False, True])
        calls.listener = SimpleNamespace(GetNextEvent=lambda _: next(next_events),
            WaitForEvent=lambda *_: self.fail('rounded fractional wait up to a second'))
        with patch.object(capture.time, 'monotonic', return_value=10):
            calls.call('objc_msgSend', 1, 2)
        self.assertEqual(calls.count, 1)
        # The same breakpoint reached from a nested call is not a return.
        frame.GetSP = lambda: 3904
        next_events = iter([False, True])
        with patch.object(capture.time, 'monotonic', return_value=10), self.assertRaises(codec.Refusal):
            calls.call('objc_msgSend', 1, 2)
        self.assertTrue(calls.inflight)

        # Returned ownership is registered before a possible restore failure.
        frame.GetSP = lambda: 3968
        frame.FindRegister = lambda _: SimpleNamespace(GetValueAsUnsigned=lambda: 0x9000)
        calls.deadline = float('inf'); calls.inflight = False
        calls.listener = SimpleNamespace(GetNextEvent=lambda _: False, WaitForEvent=lambda *_: True)
        def restore_failure(): raise codec.Refusal('register_restore_failed')
        calls.restore = restore_failure
        with self.assertRaises(codec.Refusal): calls.call(capture.GETTER)
        self.assertEqual(calls.owned_array, 0x9000)
        self.assertFalse(calls.inflight)
        # A returned release consumes ownership even if restoration then fails.
        with self.assertRaises(codec.Refusal): calls.call('swift_release', 0x9000)
        self.assertIsNone(calls.owned_array)

    def test_post_attach_identity_uses_remaining_acquisition_budget(self):
        actual = dict(metadata_version='1.0.0', status='known', target=META['target'],
            process=dict(pid=123, bundle_id=REQ['expected_bundle'], launch_time=REQ['expected_launch_time']))
        result = subprocess.CompletedProcess([], 0, json.dumps(actual).encode())
        with patch.object(capture.time, 'monotonic', return_value=10), \
                patch.object(capture.subprocess, 'run', return_value=result) as call:
            value = capture.metadata(Path('/verified/helper'), REQ, deadline=10.125)
            self.assertEqual(value, actual)
            self.assertEqual(call.call_args.kwargs['timeout'], .125)
        with patch.object(capture.time, 'monotonic', return_value=10), \
                patch.object(capture.subprocess, 'run') as call, self.assertRaises(codec.Refusal) as error:
            capture.metadata(Path('/verified/helper'), REQ, deadline=10)
        self.assertEqual(error.exception.code, 'deadline')
        call.assert_not_called()

    def test_outer_host_root_excludes_unneeded_native_descendant_walk(self):
        messages = []
        def objc(obj, name, *args):
            messages.append((obj, name))
            values = {(1, 'windows'): 2, (2, 'count'): 1, (2, 'objectAtIndex:'): 3,
                (3, 'windowNumber'): 456, (3, 'contentView'): 4,
                (4, 'respondsToSelector:'): 1, (4, 'makeViewDebugData'): 5,
                (5, 'length'): 2, (5, 'bytes'): 100,
                (3, 'isKeyWindow'): 0, (3, 'isMainWindow'): 1, (1, 'isActive'): 1}
            if (obj, name) not in values:
                self.fail('visited native descendants below selected hosting root')
            return values[obj, name]
        calls = SimpleNamespace(symbol=lambda _: 10, selector=lambda x: x, objc=objc,
            read=lambda address, _: (1).to_bytes(8, 'little') if address==10 else b'[]')
        with patch.object(capture, 'collect_fields', return_value=b'[]') as fields:
            raw, flags = capture.collect(calls, REQ)
        fields.assert_called_once_with(calls, 4, REQ)
        self.assertEqual(raw, b'[]')
        self.assertNotIn((4, 'subviews'), messages)
        self.assertEqual(flags['values'], dict(is_key_window=False, is_main_window=True, app_is_active=True))

    def test_distinct_outer_roots_refuse_without_calling_either_getter(self):
        def objc(obj, name, *args):
            if name == 'makeViewDebugData': self.fail('ambiguous root getter dispatched')
            if name == 'objectAtIndex:': return 3 if obj==2 else 7+args[0]
            values = {(1,'windows'):2, (2,'count'):1, (3,'windowNumber'):456,
                (3,'contentView'):4, (4,'respondsToSelector:'):0, (4,'subviews'):6,
                (6,'count'):2, (7,'respondsToSelector:'):1, (8,'respondsToSelector:'):1}
            if (obj,name) not in values: self.fail('descended below a hosting root')
            return values[obj,name]
        calls = SimpleNamespace(symbol=lambda _:10, selector=lambda x:x, objc=objc,
            read=lambda *_:(1).to_bytes(8,'little'))
        with self.assertRaises(codec.Refusal) as error:
            capture.collect(calls,REQ)
        self.assertEqual(error.exception.code,'hosting_surface_unresolved')

    def test_product_capture_is_withdrawn_before_debugger_start(self):
        req = copy.deepcopy(REQ); req['expected_bundle'] = 'me.playphraseme.reference.example'
        with self.assertRaises(codec.Refusal) as error:
            capture.require_fixture(req)
        self.assertEqual(error.exception.code, 'product_capture_withdrawn')

    def test_long_no_target_command_uses_private_source_file_then_removes_it(self):
        lldb = '/Applications/Xcode.app/Contents/Developer/usr/bin/lldb'
        original = subprocess.Popen
        command_files = []
        def bounded_spawn(args, **kwargs):
            path = Path(args[-1]); command_files.append(path)
            self.assertEqual(args[-2], '--source')
            self.assertEqual(path.stat().st_mode & 0o777, 0o600)
            self.assertGreater(len(path.read_bytes()), 513)
            self.assertTrue(kwargs["start_new_session"])
            return original(args, **kwargs)
        with tempfile.TemporaryDirectory(prefix='uib-no-target-input-') as directory:
            marker = Path(directory)/'executed.txt'
            commands = 'script open('+repr(str(marker))+', "w").write("executed") # '+'x'*4096+'\n'
            with patch.object(capture.subprocess, 'Popen', side_effect=bounded_spawn), \
                    patch.object(capture.signal, 'getsignal', return_value=signal.SIG_DFL), \
                    patch.object(capture.signal, 'signal', return_value=signal.SIG_DFL):
                result = capture.run_lldb(lldb, commands)
            self.assertEqual(result.returncode, 0)
            self.assertEqual(marker.read_text(), 'executed')
        self.assertTrue(command_files)
        self.assertTrue(all(not path.exists() and not path.parent.exists() for path in command_files))

    def test_launcher_repeated_wait_interrupts_do_not_kill_debugger(self):
        from unittest.mock import Mock
        before = {s: signal.getsignal(s) for s in (signal.SIGINT, signal.SIGTERM, signal.SIGHUP)}
        child = Mock()
        child.wait.side_effect = [KeyboardInterrupt(), KeyboardInterrupt(), OSError(), 0]
        with patch.object(capture.subprocess, 'Popen', return_value=child) as spawn, \
                patch.object(capture.time, 'sleep', side_effect=[KeyboardInterrupt(), None, None, None, None]), \
                patch.object(capture.signal, 'getsignal', return_value=signal.SIG_DFL), \
                patch.object(capture.signal, 'signal', return_value=signal.SIG_DFL) as handler:
            result = capture.run_lldb('/mock/lldb', 'no target')
        self.assertEqual(result.returncode, 0)
        self.assertEqual(child.wait.call_count, 4)
        child.kill.assert_not_called(); child.terminate.assert_not_called()
        child.communicate.assert_not_called()
        self.assertTrue(spawn.call_args.kwargs['start_new_session'])
        self.assertEqual(handler.call_count, 6)
        self.assertEqual(before, {s: signal.getsignal(s) for s in before})

    def test_unrestorable_native_signal_handler_refuses_before_spawn(self):
        with patch.object(capture.signal, 'getsignal', return_value=None), \
                patch.object(capture.signal, 'signal') as change, \
                patch.object(capture.subprocess, 'Popen') as spawn, self.assertRaises(codec.Refusal):
            capture.run_lldb('/mock/lldb', 'no target')
        change.assert_not_called(); spawn.assert_not_called()

    def test_live_no_target_launcher_cancellation_preserves_debugger(self):
        with tempfile.TemporaryDirectory(prefix='uib-launcher-cancel-') as directory:
            root = Path(directory); ready = root/'ready'; done = root/'done'
            commands = ('script import os,time; open('+repr(str(ready))+', "w").write(str(os.getpid())); '
                        'time.sleep(1.5); open('+repr(str(done))+', "w").write("completed")\n')
            program = ('import sys; sys.path.insert(0,'+repr(str(ROOT/'plugins/macos'))+'); '
                'import swiftui_debug_capture as c; sys.exit(c.run_lldb('
                +repr('/Applications/Xcode.app/Contents/Developer/usr/bin/lldb')+','+repr(commands)+').returncode)')
            launcher = subprocess.Popen([shutil.which('python3'), '-B', '-c', program],
                stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            deadline = time.monotonic()+5
            while not ready.exists() and time.monotonic()<deadline:
                time.sleep(.02)
            self.assertTrue(ready.exists())
            debugger_pid = int(ready.read_text())
            for signum in (signal.SIGINT, signal.SIGINT, signal.SIGTERM, signal.SIGHUP):
                os.kill(launcher.pid, signum)
                time.sleep(.03)
                self.assertIsNone(launcher.poll())
                os.kill(debugger_pid, 0)
            self.assertEqual(launcher.wait(timeout=5), 0)
            self.assertEqual(done.read_text(), 'completed')


if __name__ == '__main__':
    unittest.main()
