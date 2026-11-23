"""Finite scope negatives for the S03 fixture-only selection candidate."""
from pathlib import Path
import struct
import sys
from types import SimpleNamespace
import unittest
from unittest.mock import patch

sys.dont_write_bytecode = True
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / 'plugins/macos'))
from swiftui_debug_snapshot import Refusal
from select_parts import select_native_overlays, verify_native_overlays
import capture_fixture


class Graph:
    """Synthetic structural records only: no process, payload or debugger."""
    def __init__(self):
        self.req = {'scope': {'type': 'App.Picker', 'containing_type': 'SwiftUI.LazyVStack',
                             'native_view_type': 'App.Cursor'}, 'limits': {'max_depth': 256}}
        self.graph = 70000
        self.records = {}
        self.owners = {}
        self.words = {5000+0x60: 1000, 80008: 1}
        self.pointers = {90000+0x28: 91000, 91000+0x10: 5000,
                         self.graph+0x120: 80000, 80000: 81000, 81000: 82000}
        self.bindings = {}
        self.links = {5000: []}
        self.windows = {}
        self.c = SimpleNamespace(objc=lambda view, key: self.windows.get(view, 99))
        self.raw_seen = set()
        self.add(1000, 5000, 1, 1)
        self.add(1100, 5000, 2, 2, parent=1000)

    def add(self, element, owner, metadata, model, parent=0, child=0, sibling=0, properties=0):
        self.owners[element] = owner
        self.records[element] = (metadata, model, 0, parent, child, sibling, properties)

    def cursor(self, number, in_list=True, owner_model=1):
        sg = 6000 + 1000*number
        root = 2000 + 300*number
        witness = root+100
        prop = 100000+100*number
        native, attribute = 200000+number, 300000+number
        model = 20+number
        self.add(root, sg, 3, model, child=witness)
        self.add(witness, sg, 4, 30, parent=root, properties=prop)
        self.words.update({sg+0x60: root, prop+12: 0, prop+8: attribute, prop+20: 0})
        parent = 1100 if in_list else 1000
        if owner_model != 1:
            parent = 1200+number*40
            self.add(parent, 5000, 2, 2, parent=parent+16)
            self.add(parent+16, 5000, 1, owner_model, parent=1000)
        self.links[5000].append((parent, 400000+model))
        self.bindings[native] = (attribute, sg)
        return native, root, witness, prop

    def native_views(self, host, expected):
        return list(self.bindings)

    def represented_layout(self, view):
        return self.bindings[view]

    def page_owner(self, element):
        return self.owners[element]

    def owned_record(self, address, size):
        if size == 32:
            assert address in self.records
        return address

    def read(self, address, size):
        assert size == 32
        return struct.pack('<QIIIIII', *self.records[address])

    def u32(self, address):
        return self.words[address]

    def ptr(self, address):
        return self.pointers[address]

    def pointer(self, value):
        return value

    def cstring(self, address, maximum):
        assert address == 82000
        return 'layoutComputer'

    def host_graph(self, host):
        return 90000

    def qualified_nominal(self, metadata):
        return {1: 'App.Picker', 2: 'SwiftUI.LazyVStack',
                3: 'SwiftUI._OverlayModifier', 4: 'App.CursorArea'}[metadata]

    def rule_owner(self, model, cache):
        return (model, 'App.Picker') if model in (1, 9) else (None, None)

    def resolve(self, model, links_only):
        return ((400000+model, 0, model), None)


class SelectionTests(unittest.TestCase):
    def refuses(self, graph, code):
        with self.assertRaises(Refusal) as result:
            select_native_overlays(graph, 42)
        self.assertEqual(result.exception.code, code)

    def test_collects_both_rows_and_excludes_toggle_without_index_selection(self):
        g = Graph()
        _, first, first_witness, _ = g.cursor(0)
        _, second, second_witness, _ = g.cursor(1)
        g.cursor(2, in_list=False)
        self.assertEqual(set(select_native_overlays(g, 42)), {first, second})
        self.assertEqual(g.native_part_witnesses, {first_witness, second_witness})
        self.assertEqual(g.native_parts_selection['native_candidates'], 3)
        g.raw_seen = {first_witness, second_witness}
        verify_native_overlays(g)
        g.raw_seen.remove(second_witness)
        with self.assertRaises(Refusal):
            verify_native_overlays(g)

    def test_duplicate_native_binding_refuses(self):
        g = Graph()
        native, _, _, _ = g.cursor(0)
        g.bindings[native+1] = g.bindings[native]
        self.refuses(g, 'native_anchor_unresolved')

    def test_foreign_native_window_refuses_before_graph_walk(self):
        g = Graph()
        native, _, _, _ = g.cursor(0)
        g.windows[native] = 100
        self.refuses(g, 'native_parts_window_mismatch')

    def test_missing_layout_witness_refuses(self):
        g = Graph()
        _, _, _, prop = g.cursor(0)
        g.words[prop+8] = 999
        self.refuses(g, 'anchor_scope_binding_unresolved')

    def test_duplicate_layout_witness_refuses(self):
        g = Graph()
        _, root, witness, prop = g.cursor(0)
        other = witness+40
        data = list(g.records[witness]); data[5] = other; g.records[witness] = tuple(data)
        g.add(other, g.owners[witness], 4, 30, parent=root, properties=prop)
        self.refuses(g, 'anchor_scope_binding_unresolved')

    def test_sibling_cycle_refuses(self):
        g = Graph()
        _, _, witness, _ = g.cursor(0)
        data = list(g.records[witness]); data[5] = witness; g.records[witness] = tuple(data)
        self.refuses(g, 'native_parts_index_cycle')

    def test_nested_matching_owner_is_not_silently_chosen(self):
        g = Graph()
        g.cursor(0, owner_model=9)
        self.refuses(g, 'native_parts_owner_ambiguous')

    def test_two_sibling_picker_owners_refuse(self):
        g = Graph()
        g.cursor(0)
        _, root, _, _ = g.cursor(1)
        # A neutral graph root contains two distinct Picker body occurrences.
        # Neither is allowed to win merely because native_views listed it first.
        g.records[1000] = (4, 0, 0, 0, 0, 0, 0)
        g.add(1300, 5000, 1, 1, parent=1000)
        data = list(g.records[1100]); data[3] = 1300; g.records[1100] = tuple(data)
        g.add(1400, 5000, 1, 9, parent=1000)
        g.add(1450, 5000, 2, 2, parent=1400)
        model = g.records[root][1]
        g.links[5000] = [(1450 if attr == 400000+model else parent, attr)
                         for parent, attr in g.links[5000]]
        self.refuses(g, 'native_parts_owner_ambiguous')

    def test_incomplete_candidate_cannot_be_ignored_as_a_nonmatch(self):
        g = Graph()
        g.cursor(0)
        _, root, _, _ = g.cursor(1, in_list=False)
        model = g.records[root][1]
        g.links[5000] = [(parent, attr) for parent, attr in g.links[5000]
                         if attr != 400000+model]
        self.refuses(g, 'native_parts_ancestry_incomplete')

    def test_index_depth_is_bounded_before_ancestor_selection(self):
        g = Graph()
        _, root, witness, _ = g.cursor(0)
        extra = witness+40
        data = list(g.records[witness]); data[4] = extra; g.records[witness] = tuple(data)
        g.add(extra, g.owners[root], 4, 30, parent=witness)
        g.req['limits']['max_depth'] = 2
        self.refuses(g, 'native_parts_index_depth')

    def test_large_unselected_index_does_not_expand_record_budget(self):
        g = Graph()
        _, root, witness, _ = g.cursor(0)
        first = 1000000
        data = list(g.records[witness]); data[5] = first; g.records[witness] = tuple(data)
        for number in range(1025):
            node = first+number*32
            following = node+32 if number < 1024 else 0
            g.add(node, g.owners[root], 4, 30, parent=root, sibling=following)
        self.refuses(g, 'native_parts_record_limit')

    def test_fixture_harness_rejects_product_before_any_capture(self):
        request = {'expected_bundle': 'me.playphraseme.reference.actors03.nov23',
                   'scope': dict(capture_fixture.FIXTURE_SCOPE)}
        with patch.object(capture_fixture.tool, 'read_request', return_value=request), \
             patch.object(capture_fixture.tool, 'capture') as target:
            with self.assertRaises(Refusal) as error:
                capture_fixture.capture(None, 'unused', 'unused', 'unused')
            self.assertEqual(error.exception.code, 'actor_fixture_required')
            target.assert_not_called()


if __name__ == '__main__':
    unittest.main()
