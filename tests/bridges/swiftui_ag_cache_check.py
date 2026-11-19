"""Focused cache ownership/privacy/shape negatives; no target process."""
import sys
sys.dont_write_bytecode = True
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import Mock
import struct
import unittest
sys.path.insert(0, str(Path(__file__).resolve().parents[2]/'plugins/macos'))
from swiftui_ag_cache import CacheReader
from swiftui_debug_snapshot import Refusal, decode, snapshot


def reader():
    r = object.__new__(CacheReader)
    r.graph = 0x1000
    r.private_ranges, r.reads, r.geometry_ranges = [], [], []
    r.req = dict(limits=dict(max_nodes=256, max_depth=256))
    r.nodes, r.raw_seen = [], set()
    return r


class CacheChecks(unittest.TestCase):
    def test_foreign_empty_child_refuses_before_type_or_children(self):
        r = reader(); r.page_owner = lambda _: 0x2000
        r.ptr = lambda _: 0x3000
        r.read = Mock(side_effect=AssertionError('payload read'))
        r.address = Mock(side_effect=AssertionError('payload address'))
        with self.assertRaises(Refusal) as error: r.walk(512)
        self.assertEqual(error.exception.code, 'cache_foreign_graph')
        r.read.assert_not_called(); r.address.assert_not_called()

    def test_model_range_blocks_read_and_string_stops_at_terminator(self):
        r = reader(); r.private_ranges = [(104, 120)]
        memory = {100: 65, 101: 0}
        r.c = SimpleNamespace(read=lambda a,n: bytes(memory[x] for x in range(a,a+n)))
        self.assertEqual(r.cstring(100), 'A')
        self.assertEqual(r.reads, [(100,101),(101,102)])
        with self.assertRaises(Refusal): r.read(103, 2)
        self.assertEqual(r.reads, [(100,101),(101,102)])

    def geometry_reader(self, pair):
        r = reader()
        r.geometry_type = lambda *_: 16
        r.resolve = lambda _: ((512,0,512),None)
        r.attribute_info = lambda _: (100,200,0x110)
        r.ptr = lambda _: 300
        r.value_size = lambda _: (16,16,8)
        r.cached_storage = lambda *_: 400
        r.read = lambda *_: struct.pack('<dd',*pair)
        return r

    def test_negative_size_refuses_but_negative_position_is_preserved(self):
        r = self.geometry_reader((-.25, 3.125))
        with self.assertRaises(Refusal): r.geometry(512,300,'size')
        self.assertEqual(r.geometry(512,300,'position'), ((-.25,3.125),None))
        for value in (float('nan'),float('inf')):
            r = self.geometry_reader((value,1))
            with self.assertRaises(Refusal): r.geometry(512,300,'position')

    def test_dirty_cached_value_is_unknown_without_payload_read(self):
        r = self.geometry_reader((1,2)); r.cached_storage = lambda *_: None
        r.read = Mock(side_effect=AssertionError('dirty payload read'))
        self.assertEqual(r.geometry(512,300,'size'), (None,'cache_value_dirty'))
        r.read.assert_not_called()

    def test_stale_indirect_reference_is_unknown_before_geometry_payload(self):
        r = reader(); r.owned_record = lambda *_: 100
        r.u32 = lambda a: {116:0,108:1024,112:7,224:8}[a]
        r.page_owner = lambda _: 200
        self.assertEqual(r.resolve(513), (None,'cache_reference_stale'))

    def test_equal_order_uses_only_complete_chain_otherwise_unknown(self):
        r = reader(); r.unknown_order = 0
        r.page_owner = lambda _: 10; r.links = {10:[(512,77)]}
        r.subgraphs = [(1,1024,77),(1,1536,77)]
        r.owned_record = lambda a,n: a
        r.u32 = lambda a: {1048:1536,1560:0}[a]
        self.assertEqual(r.child_subgraphs(512), ([1024,1536],True))
        r.u32 = lambda _: 0
        roots,known = r.child_subgraphs(512)
        self.assertFalse(known); self.assertEqual(set(roots),{1024,1536})
        self.assertEqual(r.unknown_order,1)

    def test_generic_subclass_without_descriptor_is_not_host_proof(self):
        r=reader(); r.hosts={};r.isa_mask=(1<<64)-1;r.host_descriptor=500
        r.pointer=lambda x:x
        r.u64=lambda a:{1000:2000,2032:2,2064:0,2008:0}[a]
        r.c=SimpleNamespace(call=Mock(side_effect=AssertionError('unqualified host call')))
        self.assertIsNone(r.host_graph(1000))
        r.c.call.assert_not_called()

    def test_descriptor_can_be_four_aligned_but_heap_requires_eight(self):
        r=reader();r.pointer=lambda x:x;r.u64=lambda _:0x1004
        self.assertEqual(r.ptr(100,4),0x1004)
        with self.assertRaises(Refusal):r.ptr(100)

    def test_unresolved_model_protection_refuses_instead_of_skipping(self):
        for reason in ('cache_reference_stale','cache_value_unavailable'):
            r=reader();r.resolve=lambda *a,**kw:(None,reason)
            r.attribute_info=Mock(side_effect=AssertionError('unresolved payload'))
            with self.assertRaises(Refusal) as error:r.protect_model(513)
            self.assertEqual(error.exception.code,'cache_model_protection_incomplete')
            r.attribute_info.assert_not_called()

    def test_cached_storage_rejects_foreign_page_before_payload(self):
        r=reader();r.base=0x10000;r.limit=4096
        r.u32=lambda _:1024;r.u8=lambda _:0
        r.page_owner=lambda _:0x2000;r.ptr=lambda _:0x3000
        r.read=Mock(side_effect=AssertionError('payload read'))
        with self.assertRaises(Refusal) as error:r.cached_storage(512,0x10,16)
        self.assertEqual(error.exception.code,'cache_foreign_graph')
        r.read.assert_not_called()

    def test_range_crossing_into_foreign_page_refuses(self):
        r=reader();r.base=0x10000;r.limit=4096
        r.page_owner=lambda page:page
        r.ptr=lambda a:r.graph if a==512+0x28 else 0x9999
        with self.assertRaises(Refusal) as error:r.owned_record(1000,32)
        self.assertEqual(error.exception.code,'cache_foreign_graph')
        self.assertEqual(r.owned_record(1000,24),r.base+1000)

    def test_box_ownership_unknown_refuses_before_pointer_read(self):
        r=reader();r.u32=lambda _:1024;r.u8=lambda _:2
        r.owned_record=Mock(return_value=0x10400)
        r.ptr=Mock(side_effect=AssertionError('box pointer read'))
        with self.assertRaises(Refusal) as error:r.cached_storage(512,0x10,16)
        self.assertEqual(error.exception.code,'cache_box_ownership_unqualified')
        r.owned_record.assert_called_once_with(1024,8);r.ptr.assert_not_called()

    def test_boxed_models_are_denied_without_reading_heap_payload(self):
        r=reader();r.pointer=lambda x:x
        r.resolve=lambda *a,**kw:((512,0,512),None)
        r.attribute_info=lambda _: (0x10200,0x20000,0x110)
        r.u64=lambda a: 0x30000 if a==0x20000 else self.fail('unexpected model read')
        r.value_size=lambda _: (32,32,8)
        r.u32=lambda a:{0x2002c:32,0x10208:1024}[a]
        r.u8=lambda _:3
        r.owned_record=Mock(side_effect=lambda off,size:0x10000+off)
        slots=[]
        def pointer_slot(a):
            slots.append(a)
            return {0x10220:0x50000,0x20008:0x40000,0x10400:0x60000}[a]
        r.ptr=pointer_slot
        r.protect_model(512)
        self.assertEqual(r.private_ranges,[(0x50000,0x50020),(0x60000,0x60020)])
        self.assertEqual(slots,[0x10220,0x20008,0x10400])
        self.assertEqual(r.reads,[])
        r.c=SimpleNamespace(read=Mock(side_effect=AssertionError('private payload')))
        for address in (0x50000,0x60000):
            with self.assertRaises(Refusal):r.read(address,16)
        r.c.read.assert_not_called()

    def test_foreign_model_pointer_slot_refuses_before_pointee_resolution(self):
        r=reader();r.pointer=lambda x:x
        r.resolve=lambda *a,**kw:((512,0,512),None)
        r.attribute_info=lambda _: (0x10200,0x20000,0x110)
        r.u64=lambda _:0x30000;r.value_size=lambda _:(32,32,8)
        r.u32=lambda _:32;r.u8=lambda _:1
        def foreign(*_):raise Refusal('cache_foreign_graph')
        r.owned_record=foreign;r.ptr=Mock(side_effect=AssertionError('foreign slot read'))
        with self.assertRaises(Refusal):r.protect_model(512)
        r.ptr.assert_not_called()

    def scoped_reader(self, names, children, scope):
        r=reader();r.req['scope']=scope;r.nil_attribute=0
        r.resolve=lambda *args,**kwargs:(None,'cache_value_unavailable')
        r.pointer=lambda x:x;r.owned_record=lambda off,size:off
        r.u64=lambda p:p
        r.qualified_nominal=lambda metadata:names.get(metadata)
        following={}
        for siblings in children.values():
            for i,node in enumerate(siblings):following[node]=siblings[i+1] if i+1<len(siblings) else 0
        def u32(address):
            for node in names:
                if address==node+20:return next(iter(children.get(node,[])),0)
                if address==node+24:return following.get(node,0)
            return 0
        r.u32=u32;r.child_subgraphs=lambda _:([],True)
        r.properties=Mock(side_effect=AssertionError('outside geometry collected'))
        return r

    def test_unique_scope_with_proper_descendant_and_no_payload_reads(self):
        names={1024:'App.Root',2048:'App.Card',3072:'App.Video',4096:'App.Card'}
        r=self.scoped_reader(names,{1024:[2048,4096],2048:[3072]},
                             dict(type='App.Card',containing_type='App.Video'))
        self.assertEqual(r.select_scope(1024),[2048])
        self.assertEqual(r.scanned_records,4);r.properties.assert_not_called()

    def test_absent_ambiguous_and_self_only_scope_refuse(self):
        names={1024:'App.Root',2048:'App.Card',3072:'App.Card'}
        children={1024:[2048,3072]}
        for scope,code in [(dict(type='App.Missing'),'cache_scope_unresolved'),
                           (dict(type='App.Card'),'cache_scope_ambiguous'),
                           (dict(type='App.Card',containing_type='App.Card'),'cache_scope_unresolved')]:
            r=self.scoped_reader(names,children,scope)
            with self.assertRaises(Refusal) as error:r.select_scope(1024)
            self.assertEqual(error.exception.code,code)

    def structural_capture(self, scope):
        # Recorded structural inputs, with two outer branches of owner 101 and
        # one of owner 202, both of the same nominal body type. No live backend.
        names = {1024: 'App.Window', 2048: 'App.Empty', 3072: 'App.Transparent', 4096: 'SwiftUI.VStack',
                 5120: 'SwiftUI.Text', 6144: 'App.SelectedPart', 7168: 'App.Similar'}
        children = {1024: [2048, 3072, 7168], 3072: [4096, 6144], 4096: [5120]}
        r = self.scoped_reader(names, children, scope)
        r.req['limits']['max_output_bytes'] = 1048576
        r.req['window_id'] = 1
        r.c = SimpleNamespace()
        r.host_graph = lambda _: 10
        r.ptr = lambda _: 20
        r.u8 = lambda _: 0
        r.symbol = lambda *args: 30
        r.page_owner = lambda _: 20
        r.associations = lambda: None
        links = r.u32
        words = {20+0x60: 1024, **{n+8: n for n in names}, 2048+12: 1, 3072+12: 1}
        r.u32 = lambda address: words.get(address, links(address))
        r.rule_owner = lambda model, _: ((101, 'App.Hint') if model in (2048, 3072) else
                                         (202, 'App.Hint') if model == 7168 else (None, None))
        r.properties = lambda _: []
        r.protect_model = lambda _: None
        r.nominal = names.__getitem__
        raw = r.collect(None)
        receipt = {}
        document = decode(snapshot(raw, r.req, dict(target=dict(id='fixture', generation='one')),
                                   1, 2, receipt))['artifact']['data']
        return document, receipt

    def test_structural_selection_to_published_roots(self):
        scope = dict(type='App.Hint', containing_type='App.SelectedPart')
        doc, receipt = self.structural_capture(scope)
        nodes = doc['nodes']
        self.assertEqual([n['native_role']['value']['value'] for n in nodes],
                         ['SwiftUI.VStack', 'SwiftUI.Text', 'App.SelectedPart'])
        self.assertEqual(nodes[0]['children'], [nodes[1]['key']])
        self.assertEqual(receipt['observation_id'], doc['observations'][0]['id'])
        self.assertEqual(receipt['selection_source'], 'body_owner')
        self.assertEqual(receipt['source_type'], 'App.Hint')
        self.assertEqual([b['root_keys'] for b in receipt['branches']], [[], [nodes[0]['key'], nodes[2]['key']]])
        self.assertEqual(len({b['branch_id'] for b in receipt['branches']}), 2)
        self.assertEqual(set(receipt), {'selection_source', 'source_type', 'branches', 'occurrence_id', 'observation_id'})
        self.assertTrue(all(set(b) == {'branch_id', 'root_keys'} for b in receipt['branches']))
        doc2, other = self.structural_capture(scope)
        self.assertNotEqual(receipt['observation_id'], doc2['observations'][0]['id'])
        self.assertNotEqual(receipt['occurrence_id'], other['occurrence_id'])
        self.assertTrue({n['key']['key'] for n in nodes}.isdisjoint(n['key']['key'] for n in doc2['nodes']))
        with self.assertRaisesRegex(Refusal, 'cache_scope_ambiguous'):
            self.structural_capture(dict(type='App.Hint'))
        with self.assertRaisesRegex(Refusal, 'layout_not_available'):
            self.structural_capture(dict(type='App.Empty'))
        _, nominal = self.structural_capture(dict(type='SwiftUI.VStack'))
        self.assertEqual(nominal['selection_source'], 'nominal_element')
        self.assertEqual(len(nominal['branches']), 1)

    def test_scope_scan_does_not_use_emitted_node_quota_or_increase_raw_cap(self):
        names={1024+i*1024:'App.Other' for i in range(300)};names[2048]='App.Card'
        r=self.scoped_reader(names,{1024:list(names)[1:]},dict(type='App.Card'))
        r.req['limits']['max_nodes']=1
        self.assertEqual(r.select_scope(1024),[2048])
        self.assertEqual(r.scanned_records,300)
        names={1024+i*1024:'App.Other' for i in range(1025)};names[2048]='App.Card'
        r=self.scoped_reader(names,{1024:list(names)[1:]},dict(type='App.Card'))
        with self.assertRaises(Refusal) as error:r.select_scope(1024)
        self.assertEqual(error.exception.code,'cache_scope_scan_limit')

    def test_anchor_can_only_ascend_a_unique_owned_parent(self):
        r = reader()
        r.req['scope'] = {'native_view_type':'App.Surface'}
        r.native_layout_anchor = lambda *_: (512, 1500)
        r.u32 = lambda address: {1500+0x60:2048,2048+16:0,2048+8:4096}[address]
        r.u64 = lambda _: 0; r.pointer = lambda x:x
        r.page_owner = lambda _: 1500
        r.owned_record = lambda offset, size: offset
        r.resolve = lambda *a, **kw: ((8192,0,8192),None)
        r.links = {3000:[(4000,8192)],5000:[(6000,8192)]}
        r.select_scope = Mock(side_effect=Refusal('cache_scope_unresolved'))
        with self.assertRaises(Refusal) as error:
            r.anchored_scope(700)
        self.assertEqual(error.exception.code,'anchor_card_owner_unresolved')
        r.select_scope.assert_not_called()

    def test_geometry_selection_preserves_skipped_model_and_ancestor(self):
        r = reader()
        r.req['scope'] = {'selection':'geometry_records','containing_type':'App.Video'}
        r.anchor_matches = 0
        r.nil_attribute = 0
        r.selected_models = set()
        r.omitted_views = 0
        r.owned_record = lambda off, size: off
        r.pointer = lambda x: x
        r.u64 = lambda off: off
        words = {}
        for off, child in ((1024,2048),(2048,3072),(3072,0)):
            words.update({off+12:0,off+8:off+100,off+20:child,off+24:0})
        r.u32 = words.__getitem__
        r.properties = lambda off: [('size',1,2)] if off==3072 else []
        r.qualified_nominal = lambda _: 'App.Wrapper'
        r.child_subgraphs = lambda _: ([],True)
        r.walk(1024)
        self.assertEqual(r.selected_models,{1124,2148,3172})
        self.assertEqual(r.omitted_views,1)
        self.assertEqual([n['parent'] for n in r.nodes],[None,0])
        self.assertEqual(r.nodes[0]['children'],[1])

    def test_owner_only_selection_does_not_read_unrequested_nominal_types(self):
        r = self.scoped_reader({1024:'App.Root', 2048:'App.Wrapper'}, {1024:[2048]},
                               dict(type='App.PrivateSlot', native_match='within_owner'))
        words = r.u32
        r.u32 = lambda address: 100 if address == 2048 + 8 else words(address)
        r.rule_owner = lambda model, cache: (100, 'App.PrivateSlot') if model == 100 else (None, None)
        r.qualified_nominal = Mock(side_effect=AssertionError('unrequested nominal lookup'))
        self.assertEqual(r.select_scope(1024), [2048])
        r.qualified_nominal.assert_not_called()

    def test_owner_scan_and_selected_walk_share_the_record_limit(self):
        r = reader()
        r.owner_scan_cost = 1023
        r.raw_seen.add(1024)
        r.owned_record = Mock(side_effect=AssertionError('excess read'))
        with self.assertRaises(Refusal) as error:
            r.walk(2048)
        self.assertEqual(error.exception.code, 'cache_tree_limit')
        r.owned_record.assert_not_called()

    def test_incomplete_native_enumeration_cannot_yield_candidates(self):
        r = reader()
        r.pointer = r.u64 = lambda address: 0
        r.isa_mask = 0
        r.c = SimpleNamespace(objc=Mock(side_effect=[123, 128]))
        with self.assertRaises(Refusal) as error:
            r.native_views(700, 'App.Cursor')
        self.assertEqual(error.exception.code, 'anchor_view_limit')
        self.assertEqual(r.c.objc.call_count, 2)

    def test_stale_or_duplicate_candidate_refuses_before_reading_selected_layout(self):
        for results, expected in (([(100, 20), Refusal('expired_layout_reference')], 'expired_layout_reference'),
                                  ([(100, 20), (100, 20)], 'native_anchor_unresolved')):
            r = reader()
            r.req['scope'] = dict(type='App.Slot', native_view_type='App.Cursor', native_match='within_owner')
            r.host_graph = r.ptr = r.u32 = r.page_owner = lambda *args: 100
            r.u8 = lambda address: 0
            r.symbol = lambda *args: 200
            r.associations = Mock()
            r.scanned_records = 8
            r.select_scope = Mock(return_value=[256])
            r.native_views = Mock(return_value=[300, 400])
            r.represented_layout = Mock(side_effect=results)
            r.walk = Mock(side_effect=AssertionError('premature acquisition'))
            with self.assertRaises(Refusal) as error:
                r.collect(700)
            self.assertEqual(error.exception.code, expected)
            self.assertEqual(r.represented_layout.call_count, 2)
            r.walk.assert_not_called()

    def test_lldb_short_name_cannot_alias_private_descriptor(self):
        suffix = '_00DEFCC61D65EEABD9495FC981F4871C'
        sym = SimpleNamespace(GetName=lambda:'nominal type descriptor for App.Probe',
            GetMangledName=lambda:'$s3App5Probe33'+suffix+'LLCMn')
        self.assertTrue(CacheReader.native_descriptor_matches('App.(Probe in '+suffix+')',sym))
        self.assertFalse(CacheReader.native_descriptor_matches('App.Probe',sym))
        self.assertFalse(CacheReader.native_descriptor_matches('App.(Probe in _'+'A'*32+')',sym))

    def test_public_native_descriptor_requires_a_mangled_symbol(self):
        for missing in (None, ''):
            symbol = SimpleNamespace(GetName=lambda: 'nominal type descriptor for App.Probe',
                                     GetMangledName=lambda: missing)
            self.assertFalse(CacheReader.native_descriptor_matches('App.Probe', symbol))
        symbol = SimpleNamespace(GetName=lambda: 'nominal type descriptor for App.Probe',
                                 GetMangledName=lambda: '$s3App5ProbeCMn')
        self.assertTrue(CacheReader.native_descriptor_matches('App.Probe', symbol))

    def test_declared_leaf_stops_before_child_links(self):
        r=reader()
        r.req['scope']={'selection':'geometry_records','containing_type':'App.Video',
                        'leaf_types':['App.Actions']}
        r.nil_attribute=0; r.anchor_matches=0; r.anchor_next={}
        r.boundary_hits={};r.boundary_records=[];r.selected_models=set()
        r.owned_record=lambda element,size:element
        r.pointer=lambda x:x; r.u64=lambda _:123
        r.u32=lambda addr:{1024+12:0,1024+8:42}[addr]
        r.properties=lambda _:[]; r.qualified_nominal=lambda _:'App.Actions'
        r.child_subgraphs=Mock(side_effect=AssertionError('descendants accessed'))
        r.walk(1024)
        self.assertEqual(r.nodes[0]['children'],[])
        self.assertEqual(r.selected_models,{42})
        self.assertEqual(r.boundary_records,[{'type':'App.Actions','node_index':0}])
        r.child_subgraphs.assert_not_called()

    def test_owner_video_witness_must_be_on_anchor_ancestry(self):
        r=reader();r.req['scope']={'containing_type':'App.Video'}
        r.qualified_nominal=lambda _: 'SwiftUI.ModifiedContent'
        r.nodes=[dict(metadata=10,parent=None),
                 dict(metadata=11,parent=0,leaf_type='App.Video'),
                 dict(metadata=12,parent=0)]
        r.matched_anchor_node=2
        with self.assertRaises(Refusal) as error:r.verify_anchor_ancestry()
        self.assertEqual(error.exception.code,'anchor_descendant_unresolved')
        r.nodes[2]['parent']=1
        r.verify_anchor_ancestry()

if __name__ == '__main__': unittest.main()
