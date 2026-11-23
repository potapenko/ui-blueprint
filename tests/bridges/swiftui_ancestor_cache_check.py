"""Independent bounded structural-cache checks; no target process or model data."""
from pathlib import Path
import sys
from types import SimpleNamespace
import unittest
from unittest.mock import Mock

sys.dont_write_bytecode = True
sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'plugins/macos'))
from swiftui_ancestor_cache import AncestorCache
from swiftui_debug_snapshot import Refusal


def reader():
    r = SimpleNamespace(req={'scope': {'native_match': 'ancestor_owner'}},
        graph=99, table=100, base=100000, limit=1000000,
        raw_seen=set(), transform_context_seen=set(), reads=[], private_ranges=[])
    r.owned_record = Mock(side_effect=lambda element, size: r.base+element)
    r.read = Mock(return_value=bytes(range(32)))
    return r


class AncestorCacheChecks(unittest.TestCase):
    def test_repeated_ancestor_and_selected_context_count_once(self):
        r=reader();c=AncestorCache(r)
        for _ in range(6):
            for element in [512, 544, 576]:c.tree_element(element)
        r.raw_seen={512, 544};r.transform_context_seen={544, 608}
        self.assertEqual(c.record_count(), 4)
        self.assertEqual(r.read.call_count, 3)
        self.assertEqual(r.owned_record.call_count, 18)
        self.assertEqual(c.cached_read(r.base+516, 8), bytes(range(4, 12)))
        self.assertEqual(r.reads, [(r.base+516, r.base+524)])

    def test_exact_limit_and_new_record_refuses_before_source_read(self):
        r=reader();c=AncestorCache(r)
        for element in range(512, 512+1024*32, 32):c.tree_element(element)
        self.assertEqual(c.record_count(), 1024)
        c.tree_element(512)
        before=r.owned_record.call_count
        with self.assertRaisesRegex(Refusal, 'ancestor_cache_record_limit'):
            c.tree_element(512+1024*32)
        self.assertEqual(r.owned_record.call_count, before)
        self.assertEqual(r.read.call_count, 1024)

    def test_selected_and_context_records_share_admission(self):
        r=reader();c=AncestorCache(r);r.raw_seen=set(range(1,1024))
        r.transform_context_seen={1024}
        c.admit(1)
        with self.assertRaisesRegex(Refusal, 'ancestor_cache_record_limit'):c.admit(1025)
        r.read.assert_not_called()

    def test_unregistered_and_cross_header_bytes_never_come_from_cache(self):
        r=reader();c=AncestorCache(r);c.tree_element(512);c.tree_element(544)
        self.assertIsNone(c.cached_read(r.base+576,32))
        self.assertIsNone(c.cached_read(r.base+536,16))
        self.assertIsNone(c.cached_read(r.base+512,0))
        self.assertEqual(c.cached_read(r.base+544,32),bytes(range(32)))

    def test_later_private_range_blocks_hit_and_explicit_reaccess(self):
        r=reader();c=AncestorCache(r);address=c.tree_element(512)
        r.private_ranges=[(address+8,address+16)]
        with self.assertRaisesRegex(Refusal,'cache_private_range'):c.cached_read(address+8,4)
        with self.assertRaisesRegex(Refusal,'cache_private_range'):c.tree_element(512)
        self.assertEqual(r.read.call_count,1)

    def test_cached_reads_remain_in_final_retrospective_privacy_audit(self):
        r=reader();c=AncestorCache(r);address=c.tree_element(512)
        c.cached_read(address+4,8);r.private_ranges=[(address+8,address+16)]
        self.assertTrue(any(a<end and b>start for a,b in r.reads for start,end in r.private_ranges))

    def test_foreign_graph_validation_precedes_hit(self):
        r=reader();c=AncestorCache(r);c.tree_element(512)
        r.owned_record.side_effect=Refusal('cache_foreign_graph')
        with self.assertRaisesRegex(Refusal,'cache_foreign_graph'):c.tree_element(512)
        self.assertEqual(r.read.call_count,1)

    def test_changed_binding_invalidates_cached_slices(self):
        for field in ['graph','table','base','limit']:
            r=reader();c=AncestorCache(r);address=c.tree_element(512)
            setattr(r,field,getattr(r,field)+1)
            with self.assertRaisesRegex(Refusal,'ancestor_cache_binding'):c.cached_read(address,4)

    def test_failed_or_short_read_does_not_install_header(self):
        for value in [b'short', RuntimeError('source read failure')]:
            r=reader();c=AncestorCache(r)
            if isinstance(value,Exception):r.read.side_effect=value
            else:r.read.return_value=value
            with self.assertRaises((Refusal,RuntimeError)):c.tree_element(512)
            self.assertEqual(c.elements,set());self.assertEqual(c.headers,{})

    def test_separate_readers_never_reuse_headers(self):
        a,b=reader(),reader();ca,cb=AncestorCache(a),AncestorCache(b)
        ca.tree_element(512);self.assertIsNone(cb.cached_read(b.base+512,4))
        cb.tree_element(512);self.assertEqual(b.read.call_count,1)

    def test_overlapping_header_and_wrong_scope_refuse(self):
        r=reader();c=AncestorCache(r);c.tree_element(512)
        with self.assertRaisesRegex(Refusal,'ancestor_cache_header_overlap'):c.tree_element(516)
        r.req['scope']['native_match']='within_owner'
        with self.assertRaisesRegex(Refusal,'ancestor_cache_scope'):AncestorCache(r)


if __name__ == '__main__':unittest.main()
