"""Addressed graph-index checks; all memory below is synthetic structural data."""
from pathlib import Path
import struct
import sys
from types import SimpleNamespace
import unittest

sys.dont_write_bytecode = True
sys.path.insert(0, str(Path(__file__).resolve().parents[2]/'plugins/macos'))
from swiftui_debug_snapshot import Refusal
from swiftui_graph_associations import GraphAssociations, pointer_hash, bucket_index


class Memory:
    def __init__(self):
        self.graph, self.base = 0x10000, 0x100000
        self.bytes, self.extents, self.owners, self.resolutions = {}, {}, {}, {}
        self.read_log, self.call_log = [], []
        self.raw_seen, self.transform_context_seen = set(), set()
        self.unknown_order = 0
        self.c = SimpleNamespace(call=self.call)
        self.put64(self.graph+0x118, 0)

    def put(self, address, data):
        self.bytes.update((address+i, value) for i, value in enumerate(data))

    def put64(self, address, value): self.put(address, struct.pack('<Q', value))
    def put32(self, address, value): self.put(address, struct.pack('<I', value))

    def read(self, address, size):
        self.read_log.append((address, size))
        if sum(n for _, n in self.read_log) > 2*1048576:
            raise Refusal('read_byte_limit', 2)
        return bytes(self.bytes[address+i] for i in range(size))

    def u64(self, address): return struct.unpack('<Q', self.read(address, 8))[0]
    def u32(self, address): return struct.unpack('<I', self.read(address, 4))[0]
    def u8(self, address): return self.read(address, 1)[0]
    def ptr(self, address, alignment=8):
        value = self.u64(address)
        assert value and value % alignment == 0
        return value
    def pointer(self, value): return value

    def call(self, name, address):
        assert name == 'malloc_size'
        self.call_log.append(address)
        return (self.extents.get(address, 0),)

    def subgraph(self, address, parent=0, root=0, order=0, graph=None, invalid=0):
        self.put(address, bytes(112))
        self.put64(address+0x28, self.graph if graph is None else graph)
        self.put64(address+0x38, parent)
        self.put32(address+0x60, root)
        self.put32(address+0x58, order)
        self.put(address+0x68, bytes([invalid]))

    def element(self, offset, owner, model=0, sibling=0):
        self.owners[offset] = owner
        self.put(self.base+offset, bytes(32))
        self.put32(self.base+offset+8, model)
        self.put32(self.base+offset+24, sibling)

    def owned_record(self, offset, size):
        assert offset in self.owners and size == 32
        return self.base+offset

    def page_owner(self, offset): return self.owners[offset]
    def resolve(self, model, links_only):
        assert links_only
        value = self.resolutions.get(model)
        return ((value, 0, model), None) if value is not None else (None, 'unavailable')

    def table(self, owner, pairs, count=4096, size=3090):
        table, buckets, node, data = 0x40000, 0x50000, 0x80000, 0x90000
        self.put64(self.graph+0x118, table)
        self.extents.update({table: 48, buckets: count*8, node: 48, data: len(pairs)*8})
        self.put64(table, buckets)
        self.put64(table+8, count)
        self.put64(table+16, node)
        self.put64(table+24, size)
        index = bucket_index(pointer_hash(owner), count)
        self.put64(buckets+index*8, table+16)
        self.put64(node, 0)
        self.put64(node+8, pointer_hash(owner))
        self.put64(node+16, owner)
        self.put64(node+24, data if pairs else 0)
        self.put64(node+32, len(pairs))
        self.put64(node+40, len(pairs))
        for i, pair in enumerate(pairs):
            self.put(data+i*8, struct.pack('<II', *pair))
        return table, buckets, node, data


class AssociationChecks(unittest.TestCase):
    def setup_links(self, bucket_count=4096):
        r = Memory(); sg = 0x20000
        r.subgraph(sg, root=512); r.element(512, sg)
        values = r.table(sg, [(512, 77)], count=bucket_count)
        return r, GraphAssociations(r), sg, values

    def test_exact_pointer_hash_and_bucket_reductions(self):
        # Vectors are also checked against the host libc++ std::hash<void*> probe.
        for pointer, expected in [(0, 0xd7c06285b9de677a), (8, 0xe3f7f1472c11adbe),
            (0x100000000, 0xad8f2694fcd754c8), (0x1234567890, 0x54a20553cae01a10)]:
            self.assertEqual(pointer_hash(pointer), expected)
        self.assertEqual(bucket_index(23, 8), 7)
        self.assertEqual(bucket_index(23, 7), 2)

    def test_large_global_index_reads_only_requested_bucket_and_entry(self):
        r, a, sg, (_, buckets, _, _) = self.setup_links()
        self.assertEqual(a.links_for(sg), ((512, 77),))
        bucket_reads = [(p, n) for p, n in r.read_log if buckets <= p < buckets+4096*8]
        self.assertEqual(bucket_reads, [(buckets+bucket_index(pointer_hash(sg), 4096)*8, 8)])
        self.assertLess(a.work_count, 12)

    def test_nonpower_bucket_lookup(self):
        r, a, sg, _ = self.setup_links(4093)
        self.assertEqual(a.links_for(sg), ((512, 77),))

    def test_collision_chain_skips_other_owner_without_vector_payload(self):
        r, a, sg, (table, _, target, _) = self.setup_links(1)
        other = 0xa0000
        r.extents[other] = 24
        r.put64(table+16, other)
        r.put64(other, target)
        r.put64(other+8, pointer_hash(0x30000))
        r.put64(other+16, 0x30000)
        self.assertEqual(a.links_for(sg), ((512, 77),))
        self.assertFalse(any(other+24 <= p < other+48 for p, n in r.read_log))

    def test_hash_cycle_and_duplicate_owner_refuse(self):
        r, a, sg, (_, _, node, _) = self.setup_links()
        r.put64(node, node)
        with self.assertRaisesRegex(Refusal, 'cache_association_hash_cycle'):
            a.links_for(sg)
        r, a, sg, (_, _, node, _) = self.setup_links()
        other = node+64
        r.extents[other] = 48
        r.put(other, r.read(node, 48)); r.put64(node, other)
        with self.assertRaisesRegex(Refusal, 'cache_association_duplicate_owner'):
            a.links_for(sg)

    def test_allocation_precedes_bucket_or_vector_reads(self):
        for which in ('buckets', 'vector'):
            r, a, sg, (_, buckets, _, data) = self.setup_links()
            address = buckets if which == 'buckets' else data
            r.extents[address] = 4
            with self.assertRaisesRegex(Refusal, 'cache_association_extent'):
                a.links_for(sg)
            self.assertFalse(any(address <= p < address+4 for p, n in r.read_log))

    def test_wrong_owner_and_duplicate_pair_refuse(self):
        r, a, sg, _ = self.setup_links()
        r.owners[512] = 0x30000
        with self.assertRaisesRegex(Refusal, 'cache_association_element_owner'):
            a.links_for(sg)
        r = Memory(); sg = 0x20000
        r.subgraph(sg); r.element(512, sg); r.table(sg, [(512, 77), (512, 77)])
        with self.assertRaisesRegex(Refusal, 'cache_association_duplicate_link'):
            GraphAssociations(r).links_for(sg)

    def test_parent_pointer_and_inline_vector(self):
        r = Memory(); owner, parent, other, obj = 0x20000, 0x30000, 0x40000, 0xa0000
        for sg in [owner, parent, other]: r.subgraph(sg)
        r.put64(owner+0x38, parent)
        self.assertEqual(GraphAssociations(r).parents_of(owner), (parent,))
        r.extents[obj] = 48; r.put64(owner+0x38, obj | 1)
        r.put64(obj, parent); r.put64(obj+8, other); r.put64(obj+0x20, 0); r.put64(obj+0x28, 2)
        self.assertEqual(GraphAssociations(r).parents_of(owner), (parent, other))

    def test_external_parent_vector_extent_and_duplicates(self):
        r = Memory(); owner, parent, obj, data = 0x20000, 0x30000, 0xa0000, 0xb0000
        r.subgraph(owner, parent=obj | 1); r.subgraph(parent)
        r.extents.update({obj: 48, data: 16})
        r.put64(obj+0x20, data); r.put64(obj+0x28, 2)
        r.put64(data, parent); r.put64(data+8, parent)
        with self.assertRaisesRegex(Refusal, 'cache_association_parent_cycle'):
            GraphAssociations(r).parents_of(owner)
        r.extents[data] = 8
        with self.assertRaisesRegex(Refusal, 'cache_association_extent'):
            GraphAssociations(r).parents_of(owner)

    def topology(self):
        r = Memory(); parent, child, data = 0x20000, 0x30000, 0xb0000
        r.subgraph(parent, root=512); r.subgraph(child, parent=parent, root=1024, order=2)
        r.element(512, parent); r.element(1024, child, model=99); r.resolutions[99] = 77
        r.table(parent, [(512, 77)])
        r.put64(parent+0x40, data); r.put32(parent+0x48, 1); r.put32(parent+0x4c, 1)
        r.extents[data] = 8; r.put64(data, child | 3)
        return r, GraphAssociations(r), parent, child

    def test_direct_topology_matches_recorded_attachment(self):
        r, a, parent, child = self.topology()
        self.assertEqual(a._dependency_parent_candidates(1024), ((parent, 512),))
        self.assertEqual(a._dependency_child_candidates(512), ([1024], True))

    def test_nonreciprocal_child_and_foreign_graph_refuse(self):
        r, a, parent, child = self.topology(); r.put64(child+0x38, 0)
        with self.assertRaisesRegex(Refusal, 'cache_association_topology_mismatch'):
            a._dependency_child_candidates(512)
        r, a, parent, child = self.topology(); r.put64(child+0x28, 123456)
        with self.assertRaisesRegex(Refusal, 'cache_foreign_graph'):
            a._dependency_child_candidates(512)

    def test_invalid_child_is_excluded_before_model_content(self):
        r, a, parent, child = self.topology(); r.put(child+0x68, b'\1')
        self.assertEqual(a._dependency_child_candidates(512), ([], True))
        self.assertNotIn((child+0x60, 4), r.read_log)
        with self.assertRaisesRegex(Refusal, 'cache_invalid_subgraph'):
            a.links_for(child)

    def test_invalid_foreign_child_is_not_silently_excluded(self):
        r, a, parent, child = self.topology()
        r.put(child+0x68, b'\1'); r.put64(child+0x28, 123456)
        with self.assertRaisesRegex(Refusal, 'cache_foreign_graph'):
            a._dependency_child_candidates(512)
        self.assertNotIn((child+0x60, 4), r.read_log)

    def test_lookup_work_is_cumulative_and_shared_with_raw_walk(self):
        r, a, sg, _ = self.setup_links(); r.raw_seen = set(range(1024))
        with self.assertRaisesRegex(Refusal, 'cache_association_work_limit'):
            a.links_for(sg)

    def test_public_nonempty_topology_refuses_until_source_qualified(self):
        r, a, parent, child = self.topology()
        with self.assertRaisesRegex(Refusal, 'cache_recorded_children_unqualified'):
            a.children(512)
        with self.assertRaisesRegex(Refusal, 'cache_recorded_parent_unqualified'):
            a.parent_attachments(1024)
        r.graph_root = 512
        self.assertEqual(a.parent_attachments(512), ())

    def test_public_empty_child_relation_uses_exact_owner_map_absence(self):
        r, a, parent, child = self.topology()
        r.element(544, parent)
        self.assertEqual(a.children(544), ([], True))


if __name__ == '__main__':
    unittest.main()
