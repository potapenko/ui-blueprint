#!/usr/bin/env python3
"""C04 offline tests. No app, debugger, UI collection or image capture."""
import copy
import json
import struct
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import Mock

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT/'plugins/macos'))
import swiftui_debug_snapshot as codec
import swiftui_capture_pair as pair
import swiftui_source_selection as selection

REQ = dict(request_version='1.0.0', request_id='c04-offline', pid=123,
           expected_bundle='me.uiblueprint.external-layout.canary.pair', expected_launch_time=123.5,
           window_id=10, scope=dict(type='App.Picker'),
           paired_surface=dict(window_id=20, scope=dict(type='App.Content')),
           limits=dict(max_nodes=256, max_depth=256, max_output_bytes=1048576, deadline_ms=10000))
META = dict(target=dict(id='macos-pid-123', generation='123:123.5'))
SECRET = 'PRIVATE-C04-MODEL-MUST-NOT-ESCAPE'


def node(position, size, known=True):
    item = dict(properties=[dict(id=0, attribute=dict(type='SwiftUI.Button')),
                           dict(id=1, attribute=dict(value=SECRET)),
                           dict(id=3, attribute=dict(type='__C.CGPoint', value=position)),
                           dict(id=4, attribute=dict(type='__C.CGSize', value=size))], children=[])
    if known:
        item['transform_inputs'] = dict(position_adjustment=[0, 0], pending_translation=[0, 0], chain=[])
    else:
        item['transform_unknown_reason'] = 'cache_value_dirty'
    return item


def mapping(window, frame):
    height = frame[3]
    return dict(window_id=window, window_frame=frame, host_bounds=[0, 0, frame[2], height],
                host_is_flipped=True, basis_window_points=[[0, height], [1, height], [0, height-1]],
                source='NSView.convertPoint.to-window-base')


def records():
    return [dict(raw=json.dumps([node([236, 338], [36, 42])]).encode(),
                 mapping=mapping(10, [100, 200, 800, 600])),
            dict(raw=json.dumps([node([10, 20], [180, 200])]).encode(),
                 mapping=mapping(20, [400, 450, 200, 250]))]


class SourceSelection(unittest.TestCase):
    def test_options_preserve_legacy_scope_and_reject_new_native_combinations(self):
        scope = dict(type='App.Picker', owner_argument_type='SwiftUI.LazyVGrid',
                     part=dict(type='SwiftUI.Button', containing_type='App.Arrow'))
        self.assertEqual(selection.validate_options(scope), dict(type='App.Picker'))
        for bad in (dict(scope, native_view_type='App.Cursor'), dict(scope, leaf_types=[]),
                    dict(scope, owner_argument_type=SECRET+'!'),
                    dict(scope, part=dict(type='SwiftUI.Button', containing_type='App.Arrow', index=0))):
            with self.assertRaises(codec.Refusal): selection.validate_options(bad)

    def test_part_is_unique_within_selected_forest_not_first_global_match(self):
        tree = {1: ('App.Picker', [2, 3]), 2: ('SwiftUI.Button', [4]),
                3: ('SwiftUI.Button', [5]), 4: ('App.Arrow', []), 5: ('App.Close', []),
                6: ('SwiftUI.Button', [7]), 7: ('App.Arrow', [])}
        part = dict(type='SwiftUI.Button', containing_type='App.Arrow')
        self.assertEqual(selection.select_part(tree, [1], part), [2])
        with self.assertRaisesRegex(codec.Refusal, 'source_part_ambiguous'):
            selection.select_part(tree, [1, 6], part)
        with self.assertRaisesRegex(codec.Refusal, 'source_part_unresolved'):
            selection.select_part(tree, [4], part)
        tree[4] = ('App.Arrow', [1])
        with self.assertRaisesRegex(codec.Refusal, 'source_part_cycle'):
            selection.select_part(tree, [1], part)

    def metadata_reader(self, header=(1, 1, 2, 0), parameter=0x80):
        section = SimpleNamespace(IsValid=lambda: True, GetLoadAddress=lambda _: 1000,
                                  GetByteSize=lambda: 1000)
        r = SimpleNamespace(c=SimpleNamespace(t=SimpleNamespace(
            ResolveLoadAddress=lambda _: SimpleNamespace(GetSection=lambda: section))))
        r.u64 = lambda p: {100: 0x200}[p]
        r.ptr = Mock(side_effect=lambda p, *args: {108: 1000, 116: 200}[p])
        r.u32 = lambda p: {1000: 0xd1, 1100: 0}[p]
        r.read = Mock(side_effect=lambda p, n: {1004: struct.pack('<i', 96),
                                               1036: struct.pack('<HHHH', *header)}[p])
        r.u8 = lambda p: parameter if p == 1044 else self.fail('unexpected payload read')
        r.qualified_nominal = lambda p: {100: 'App.Picker', 200: 'SwiftUI.LazyVGrid'}[p]
        return r

    def test_generic_argument_reads_metadata_only_after_shape_validation(self):
        r = self.metadata_reader()
        self.assertEqual(selection.first_argument_type(r, 100, 'App.Picker'), 'SwiftUI.LazyVGrid')
        self.assertEqual(r.read.call_args_list[0].args, (1004, 4))
        self.assertEqual(r.read.call_args_list[1].args, (1036, 8))
        for header, param in [((2, 1, 3, 0), 0x80), ((1, 1, 2, 1), 0x80),
                              ((1, 17, 2, 0), 0x80), ((1, 1, 3, 0), 0x80),
                              ((1, 1, 2, 0), 0x81)]:
            r = self.metadata_reader(header, param)
            with self.assertRaisesRegex(codec.Refusal, 'source_specialization_unqualified'):
                selection.first_argument_type(r, 100, 'App.Picker')
            self.assertNotIn((116,), [c.args for c in r.ptr.call_args_list])

    def test_different_specialization_is_not_selected_and_unresolved_is_not_a_match(self):
        scope = dict(type='App.Picker', owner_argument_type='SwiftUI.VStack')
        self.assertFalse(selection.matches_owner(self.metadata_reader(), 100, scope))
        r = self.metadata_reader()
        r.qualified_nominal = lambda p: 'App.Picker' if p == 100 else None
        with self.assertRaisesRegex(codec.Refusal, 'source_specialization_unqualified'):
            selection.matches_owner(r, 100, scope)

    def test_body_accessor_path_rejects_unqualified_rule_before_argument(self):
        r = SimpleNamespace(attribute_info=lambda _: (0, 500, 0), pointer=lambda x: x,
                            u64=lambda p: 100 if p == 500 else 0x201)
        r.ptr = Mock(side_effect=AssertionError('unqualified accessor read'))
        with self.assertRaisesRegex(codec.Refusal, 'source_specialization_unqualified'):
            selection.body_metadata(r, 12)
        r.ptr.assert_not_called()


class PairedSnapshot(unittest.TestCase):
    def encode(self, rs=None, req=None):
        return pair.encode_pair(rs or records(), req or REQ, META, 1, 2)

    def test_same_interval_pair_retains_raw_rectangles_and_explicit_attachment_gap(self):
        raw, receipt = self.encode()
        self.assertNotIn(SECRET.encode(), raw)
        doc = json.loads(raw)['artifact']['data']
        self.assertEqual(len(doc['context']['surfaces']), 2)
        self.assertEqual(len(doc['nodes']), 2)
        self.assertEqual(doc['relations'], [])
        self.assertEqual(doc['components'], [])
        self.assertEqual(receipt['attachment'], dict(status='unknown', reason='presentation-anchor-reader-not-qualified'))
        a, b = [n['properties'][0]['state']['value']['value'] for n in doc['nodes']]
        self.assertEqual(b['shape']['value'], dict(x=10, y=20, width=180, height=200))
        self.assertEqual(b['transform']['transform']['affine'], [1, 0, 0, 1, 300, 100])
        self.assertEqual(a['transform']['transform']['to'], b['transform']['transform']['to'])
        self.assertEqual(len({o['clock_domain'] for o in doc['observations']}), 1)
        self.assertEqual([s['window_id'] for s in receipt['surfaces']], [10, 20])

    def test_missing_mapping_and_dirty_transform_are_not_repaired(self):
        rs = records(); rs[1]['mapping'] = None
        raw, receipt = self.encode(rs)
        self.assertIsNone(receipt['common_space'])
        data = json.loads(raw)['artifact']['data']
        self.assertEqual(data['nodes'][1]['properties'][0]['state']['value']['value']['transform']['transform']['to']['kind'], 'local')
        rs = records(); rs[1]['raw'] = json.dumps([node([10, 20], [180, 200], False)]).encode()
        raw, _ = self.encode(rs)
        data = json.loads(raw)['artifact']['data']
        self.assertEqual(data['nodes'][1]['properties'][0]['state']['value']['value']['transform']['status'], 'unknown')

    def test_pair_identity_and_aggregate_node_output_limits(self):
        for bad in (dict(REQ, paired_surface=dict(window_id=10, scope={})),
                    dict(REQ, paired_surface=dict(window_id=True, scope={})),
                    dict(REQ, scope=dict(type='App.Picker', native_view_type='App.Cursor')),
                    dict(REQ, paired_surface=dict(window_id=20, scope=None)),
                    dict(REQ, paired_surface=dict(window_id=20, scope={}, anchor_proven=True))):
            with self.assertRaises(codec.Refusal): pair.requests(bad)
        req = copy.deepcopy(REQ); req['limits']['max_nodes'] = 1
        with self.assertRaisesRegex(codec.Refusal, 'tree_limit'): self.encode(req=req)
        req = copy.deepcopy(REQ); req['limits']['max_output_bytes'] = 100
        with self.assertRaises(codec.Refusal): self.encode(req=req)
        req = copy.deepcopy(REQ)
        rs = records(); primary, secondary = pair.requests(req)
        single_sizes = [len(codec.snapshot(r['raw'], q, META, 1, 2, None, r['mapping']))
                        for r, q in zip(rs, (primary, secondary))]
        req['limits']['max_output_bytes'] = max(single_sizes)+100
        with self.assertRaises(codec.Refusal): self.encode(req=req)
        rs = records(); rs[1]['mapping']['window_id'] = 30
        with self.assertRaisesRegex(codec.Refusal, 'invalid_native_mapping'): self.encode(rs)

    def test_both_bindings_are_revalidated_after_collection(self):
        calls = SimpleNamespace()
        order = []
        def collect(c, req):
            order.append(('collect', req['window_id']))
            c.native_layout_mapping = None
            c.cache_audit = {'views': 1}
            c.selected_binding = req['window_id']
            return b'[]', {}
        def validate(c, req, binding):
            order.append(('validate', binding))
            return None
        pair.collect(calls, REQ, collect, validate)
        self.assertEqual(order, [('collect', 10), ('collect', 20), ('validate', 10), ('validate', 20)])
        with self.assertRaisesRegex(codec.Refusal, 'stale_window'):
            pair.collect(calls, REQ, collect, lambda *_: {'changed': True})

    def test_budget_is_union_of_typed_tree_records_across_graphs(self):
        calls = SimpleNamespace(pair_records=set(), pair_nodes=0, pair_native_views=set())
        reader = SimpleNamespace(c=calls, graph=1)
        for index in range(1024): pair.charge_tree_record(reader, index)
        pair.charge_tree_record(reader, 0)
        reader.graph = 2
        with self.assertRaisesRegex(codec.Refusal, 'pair_tree_limit'):
            pair.charge_tree_record(reader, 0)
        pair.charge_node(calls, 1)
        with self.assertRaisesRegex(codec.Refusal, 'tree_limit'): pair.charge_node(calls, 1)
        for index in range(256): pair.charge_native_view(calls, index)
        pair.charge_native_view(calls, 0)
        with self.assertRaisesRegex(codec.Refusal, 'view_tree_limit'):
            pair.charge_native_view(calls, 256)

    def test_installed_rust_validator_and_measure_cross_window_gap(self):
        bundle = Path('/Users/eugenepotapenko/.local/share/uiblueprint/versions/0.6.1')
        raw, receipt = self.encode()
        doc = json.loads(raw)['artifact']['data']
        space = receipt['common_space']
        with tempfile.TemporaryDirectory(prefix='uib-c04-offline-') as directory:
            root = Path(directory); source = root/'snapshot.json'; source.write_bytes(raw)
            result = subprocess.run([str(bundle/'uiblueprint-validate'), '--max-bytes', '1048576', str(source)],
                                    capture_output=True, timeout=10)
            self.assertEqual(result.returncode, 0, result.stdout+result.stderr)
            query = dict(schema_version='0.2.0', artifact=dict(kind='geometry_query', data=dict(
                id='gap', scope_id=doc['context']['scope_id'], targets=[n['key'] for n in doc['nodes']],
                operation='gap', quantity_kind='length', units='pt',
                applies_when=dict(platform=None, text_scale=None, input_mode=None),
                anchors=[dict(element=n['key'], frame_kind='layout_bounds',
                              coordinate_space=n['properties'][0]['state']['value']['value']['coordinate_space'],
                              fraction=fraction, axis='x') for n, fraction in zip(doc['nodes'], [1, 0])])))
            path = root/'query.json'; path.write_text(json.dumps(query))
            result = subprocess.run([str(bundle/'uiblueprint'), 'measure', '--snapshot', str(source),
                '--query', str(path), '--space', space['id'], '--max-input-bytes', '1048576',
                '--max-output-bytes', '1048576', '--json'], capture_output=True, timeout=10)
            self.assertEqual(result.returncode, 0, result.stdout+result.stderr)
            value = json.loads(result.stdout)['artifact']['data']['result']
            self.assertEqual(value['measurement']['value']['value']['amount'], 38)
            self.assertNotIn(SECRET.encode(), result.stdout+result.stderr)


if __name__ == '__main__':
    unittest.main()
