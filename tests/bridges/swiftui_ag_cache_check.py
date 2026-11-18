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
from swiftui_debug_snapshot import Refusal


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

if __name__ == '__main__': unittest.main()
