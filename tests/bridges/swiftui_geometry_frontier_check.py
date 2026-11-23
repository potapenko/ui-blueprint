"""Geometry-frontier invariants; all source fields are synthetic descriptors."""
from pathlib import Path
import sys
import unittest
from unittest.mock import Mock

sys.dont_write_bytecode = True
sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'plugins/macos'))
from swiftui_geometry_frontier import GeometryFrontier
from swiftui_debug_snapshot import Refusal


class FrontierChecks(unittest.TestCase):
    def test_metadata_presence_not_known_values_sets_depth(self):
        for fields in [[('size',None,None)],[('position','dirty','stale')],
                       [('position',1,2),('size',3,4)]]:
            f=GeometryFrontier(1,256)
            self.assertEqual(f.advance(100,0,fields,lambda _:None),(1,True))

    def test_transparent_record_does_not_consume_geometry_depth(self):
        f=GeometryFrontier(2,256);admit=Mock()
        self.assertEqual(f.advance(100,0,[('transform',1,2)],admit),(0,False))
        self.assertEqual(f.advance(200,0,[('size',1,2)],admit),(1,False))
        self.assertEqual(f.advance(300,1,[('position',1,2)],admit),(2,True))
        self.assertEqual(admit.call_count,3)

    def test_siblings_share_parent_depth_not_a_global_counter(self):
        f=GeometryFrontier(2,256)
        results=[f.advance(element,1,[('size',1,2)],lambda _:None) for element in [100,200,300]]
        self.assertEqual(results,[(2,True)]*3)

    def test_native_witness_path_retains_deeper_depth_and_explicit_provenance(self):
        f=GeometryFrontier(2,256)
        depth,cut=f.advance(100,2,[('size',1,2)],lambda _:None)
        self.assertEqual((depth,cut),(3,True));f.boundary(9,depth,True)
        manifest=f.manifest()
        self.assertEqual(manifest['boundaries'],[dict(node_index=9,geometry_depth=3,native_witness_context=True)])
        self.assertEqual(manifest['coverage'],'partial')
        manifest['boundaries'].clear();self.assertEqual(len(f.boundaries),1)

    def test_cycle_and_raw_cap_refuse_without_advancing(self):
        f=GeometryFrontier(2,256);f.advance(100,0,[],lambda _:None)
        with self.assertRaisesRegex(Refusal,'geometry_frontier_cycle'):f.advance(100,0,[],lambda _:None)
        def full(_):raise Refusal('ancestor_cache_record_limit',2)
        with self.assertRaisesRegex(Refusal,'ancestor_cache_record_limit'):f.advance(200,0,[],full)
        self.assertNotIn(200,f.seen)

    def test_invalid_depth_and_geometry_path_overflow_refuse(self):
        for depth,limit in [(0,256),(True,256),(257,256),(2,1),(1,257)]:
            with self.assertRaisesRegex(Refusal,'invalid_geometry_frontier'):GeometryFrontier(depth,limit)
        with self.assertRaisesRegex(Refusal,'geometry_frontier_depth'):
            GeometryFrontier(1,2).advance(100,2,[('size',1,2)],lambda _:None)

    def test_source_descriptors_and_parent_state_are_not_rewritten(self):
        f=GeometryFrontier(1,256);fields=[('position','source-time','unknown'),('size',9,4)]
        original=list(fields);f.advance(100,0,fields,lambda _:None)
        self.assertEqual(fields,original)
        for node,depth,witness in [(-1,1,False),(256,1,False),(0,0,False),(0,1,1)]:
            with self.assertRaisesRegex(Refusal,'invalid_geometry_frontier_boundary'):f.boundary(node,depth,witness)


if __name__ == '__main__':unittest.main()
