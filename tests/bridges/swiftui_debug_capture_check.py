#!/usr/bin/env python3
"""Focused privacy/canonical/refusal checks; no product app is contacted."""
import sys
sys.dont_write_bytecode = True
import copy
import json
from pathlib import Path
import subprocess
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
           expected_bundle='me.uiblueprint.fixture', expected_launch_time=1794836979.5,
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
            result = subprocess.run([str(cli), 'inspect', '--snapshot', str(path), '--ref',
                json.dumps(dict(namespace=codec.NAMESPACE, key='own-fixture-n0')),
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
        for raw in (b'{"x":1,"x":2}', b'[NaN]', b'x'*(1048576+1)):
            with self.assertRaises(codec.Refusal) as error:
                codec.decode(raw)
            self.assertNotIn(CANARY, str(error.exception))

    def test_request_and_argument_errors_do_not_echo_private_input(self):
        req = copy.deepcopy(REQ); req['pid'] = CANARY
        with self.assertRaises(codec.Refusal) as error:
            codec.request(json.dumps(req).encode())
        self.assertEqual(str(error.exception), 'invalid_identity')
        result = subprocess.run([sys.executable, '-B', str(ROOT/'plugins/macos/swiftui_debug_capture.py'),
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
                    patch.object(capture, 'metadata', return_value=META), patch.object(capture, 'Calls', return_value=calls):
                capture.capture(debugger, str(req), '', str(out))
            result = json.loads((out/'manifest-own-fixture.json').read_bytes())
            self.assertEqual(result['cleanup'], 'owner_recovery_required')
            self.assertEqual(result['status'], 'failed')
            self.assertEqual(detached, [True])
            self.assertFalse((out/'snapshot-own-fixture.json').exists())
        # The in-flight cleanup branch performs no register writes, retry or kill.
        bare = object.__new__(capture.Calls); bare.inflight = True
        self.assertFalse(bare.close())


if __name__ == '__main__':
    unittest.main()
