#!/usr/bin/env python3
"""Offline integration checks; require C04-pair-glue.patch to be applied.

No skip/fallback: an unintegrated collector must fail these tests. Run against
the assembled candidate until the shared-file owner integrates the finite patch.
"""
import copy
import json
import sys
import tempfile
import unittest
from contextlib import ExitStack
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch, Mock

sys.dont_write_bytecode = True
from swiftui_capture_pair_check import REQ, META, node, records
import swiftui_ag_cache_check as cache_checks
import swiftui_ag_cache as cache
import swiftui_debug_capture as capture
import swiftui_debug_snapshot as codec


class IntegratedPair(unittest.TestCase):
    def test_paired_request_and_source_selector_validation(self):
        req = copy.deepcopy(REQ)
        req['scope'].update(owner_argument_type='SwiftUI.LazyVGrid',
                            part=dict(type='SwiftUI.Button', containing_type='App.Arrow'))
        self.assertEqual(codec.request(json.dumps(req).encode()), req)
        for bad in (dict(req, private=1), dict(req, window_id=True),
                    dict(req, paired_surface=dict(window_id=20, scope=dict(type='App.Content', private=1))),
                    dict(req, scope=dict(type='App.Picker', part={'type': 'SwiftUI.Button'})),
                    dict(req, paired_surface={'window_id': 20})):
            with self.assertRaises(codec.Refusal): codec.request(json.dumps(bad).encode())

    def test_pair_cannot_reuse_existing_product_admission(self):
        req = copy.deepcopy(REQ); req['expected_bundle'] = 'me.example.product'
        binding = tuple(req[k] for k in ('request_id', 'pid', 'expected_bundle',
                                        'expected_launch_time', 'window_id', 'scope'))
        with patch.object(capture, 'CONSUMER_ADMISSION', binding):
            with self.assertRaisesRegex(codec.Refusal, 'product_pair_capture_withdrawn'):
                capture.require_admitted_target(req)
            single = {k: v for k, v in req.items() if k != 'paired_surface'}
            self.assertFalse(capture.require_admitted_target(single))

    def reader(self, scope):
        names = {1024: 'App.Window', 2048: 'App.Picker', 3072: 'SwiftUI.Button',
                 4096: 'App.Arrow', 5120: 'App.Picker', 6144: 'SwiftUI.Button', 7168: 'App.Arrow'}
        children = {1024: [2048, 5120], 2048: [3072], 3072: [4096], 5120: [6144], 6144: [7168]}
        r = cache_checks.CacheChecks().scoped_reader(names, children, scope)
        r.c = SimpleNamespace(pair_records=set())
        return r

    def test_specialized_owner_precedes_unique_disclosure_selection(self):
        scope = dict(type='App.Picker', owner_argument_type='SwiftUI.LazyVGrid',
                     part=dict(type='SwiftUI.Button', containing_type='App.Arrow'))
        r = self.reader(scope)
        with patch.object(cache, 'matches_owner', side_effect=lambda reader, meta, scope: meta == 2048) as match:
            self.assertEqual(r.select_scope(1024), [3072])
            self.assertEqual([c.args[1] for c in match.call_args_list], [2048, 5120])
        self.assertEqual(r.selected_source, dict(selection_source='source_part', source_type='App.Picker'))
        r.properties.assert_not_called()
        self.assertEqual(len(r.c.pair_records), 7)
        with patch.object(cache, 'matches_owner', return_value=True):
            with self.assertRaisesRegex(codec.Refusal, 'cache_scope_ambiguous'):
                self.reader(scope).select_scope(1024)

    def test_global_tree_budget_refuses_before_another_graph_header(self):
        r = self.reader(dict(type='App.Picker'))
        r.c.pair_records = {(999, index) for index in range(1024)}
        r.owned_record = Mock(side_effect=AssertionError('read beyond pair limit'))
        with self.assertRaisesRegex(codec.Refusal, 'pair_tree_limit'): r.select_scope(1024)
        r.owned_record.assert_not_called()

    def test_source_part_receipt_distinguishes_part_from_owner_forest(self):
        req = copy.deepcopy(REQ); req.pop('paired_surface')
        req['scope']['part'] = dict(type='SwiftUI.Button', containing_type='App.Arrow')
        raw = dict(selection_source='source_part', source_type='App.Picker',
                   branches=[[node([1, 2], [3, 4])]])
        provenance = {}
        codec.snapshot(json.dumps(raw).encode(), req, META, 1, 2, provenance)
        self.assertEqual(provenance['selection_source'], 'source_part')
        self.assertEqual(len(provenance['branches']), 1)

    def test_revalidation_does_not_accept_changed_window_content(self):
        calls = SimpleNamespace(objc=lambda obj, sel: 10 if sel == 'windowNumber' else 999)
        with patch.object(capture, 'native_mapping') as mapping:
            with self.assertRaisesRegex(codec.Refusal, 'stale_window'):
                capture.revalidate_window(calls, REQ, (1, 2, 3))
            mapping.assert_not_called()

    def test_capture_publishes_pair_only_after_confirmed_cleanup(self):
        error = SimpleNamespace(Fail=lambda: False, Success=lambda: True)
        frame = SimpleNamespace(GetFunctionName=lambda: 'mach_msg2_trap')
        thread = SimpleNamespace(GetQueueName=lambda: 'com.apple.main-thread', GetFrameAtIndex=lambda _: frame)
        debugger = SimpleNamespace(SetAsync=lambda _: None, HandleCommand=lambda _: None, GetListener=lambda: None)
        lldb = SimpleNamespace(SBError=lambda: error, eStateStopped=1, eStateDetached=2)
        for clean, stale in [(True, False), (False, False), (True, True)]:
            with self.subTest(clean=clean, stale=stale), tempfile.TemporaryDirectory(prefix='uib-c04-publish-') as directory:
                out = Path(directory); req = out/'request.json'; req.write_text(json.dumps(REQ))
                process = SimpleNamespace(IsValid=lambda: True, GetProcessID=lambda: 123,
                    GetThreadAtIndex=lambda _: thread, GetState=lambda: 1,
                    Detach=Mock(return_value=error))
                debugger.CreateTarget = lambda _: SimpleNamespace(AttachToProcessWithID=lambda *args: process)
                calls = SimpleNamespace(initialize=lambda: None, admit=lambda: None, count=0, close=lambda: clean)
                fake_records = records()
                def collect(c, request):
                    record = fake_records[0 if request['window_id'] == 10 else 1]
                    c.native_layout_mapping = record['mapping']
                    c.selected_binding = request['window_id']
                    c.cache_audit = {'views': 1}
                    return record['raw'], {}
                def revalidate(c, request, binding):
                    if stale: raise codec.Refusal('stale_window')
                    return fake_records[0 if binding == 10 else 1]['mapping']
                with ExitStack() as stack:
                    stack.enter_context(patch.dict(sys.modules, lldb=lldb))
                    for name, value in [('verified_helper', Mock(return_value=None)),
                                        ('metadata', Mock(return_value=META)), ('Calls', Mock(return_value=calls)),
                                        ('collect', collect), ('revalidate_window', revalidate),
                                        ('retain_debugger', Mock(return_value='external_target_exit'))]:
                        stack.enter_context(patch.object(capture, name, value))
                    capture.capture(debugger, str(req), '', str(out))
                manifest = json.loads((out/'manifest-c04-offline.json').read_bytes())
                result = out/'snapshot-c04-offline.json'
                if clean and not stale:
                    self.assertEqual(manifest['cleanup'], 'detached_restored')
                    self.assertEqual(manifest['surface_pair']['attachment']['status'], 'unknown')
                    self.assertTrue(result.is_file())
                else:
                    self.assertEqual(manifest['status'], 'failed')
                    self.assertFalse(result.exists())
                    self.assertNotIn('surface_pair', manifest)
                if not clean: process.Detach.assert_not_called()


if __name__ == '__main__':
    unittest.main()
