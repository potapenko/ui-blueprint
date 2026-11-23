"""Focused transform privacy, validity and unsupported-operation checks."""
import math
from pathlib import Path
import struct
import sys
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch
sys.dont_write_bytecode = True
sys.path.insert(0, str(Path(__file__).resolve().parents[2]/'plugins/macos'))
import swiftui_cache_transform as transforms
from swiftui_debug_snapshot import Refusal


def reader(head=0, depth=1, next_head=0):
    data = {100: 0, 1000: head, 2000: 3000, 2000+16: next_head, 2000+24: depth}
    pairs = {1016: (2.5, -3.0), 1032: (-8.0, 4.0), 2032: (7.0, -11.0), 2048: (-2.0, 1.0)}
    return SimpleNamespace(
        cached_geometry=Mock(return_value=(1000, None)),
        pointer=lambda x:x, u64=lambda a:data[a], ptr=lambda a,*alignment:3000 if a==2000 else 4000,
        read=Mock(side_effect=lambda a,n:struct.pack('<dd', *pairs[a])),
        geometry_ranges=[], u32=Mock(side_effect=AssertionError('unexpected payload read')))


class TransformChecks(unittest.TestCase):
    def test_dirty_and_stale_values_never_read_transform_payload(self):
        for reason in ('cache_value_dirty', 'cache_reference_stale'):
            r=reader();r.cached_geometry.return_value=(None, reason)
            self.assertEqual(transforms.translation_inputs(r, 1, 2), (None, reason))
            r.read.assert_not_called()

    def test_empty_chain_preserves_signed_fractional_values_and_caches_per_reader(self):
        r=reader()
        first=transforms.translation_inputs(r,1,2)
        self.assertEqual(first, ({'position_adjustment':(2.5,-3.0),
                                 'pending_translation':(-8.0,4.0),'chain':()},None))
        self.assertEqual(transforms.translation_inputs(r,1,2),first)
        self.assertEqual(r.cached_geometry.call_count,1)
        self.assertEqual(r.read.call_count,2)
        other=reader();transforms.translation_inputs(other,1,2)
        self.assertEqual(other.read.call_count,2)

    def test_nonfinite_input_is_not_a_known_transform(self):
        for value in (math.nan,math.inf,-math.inf):
            r=reader();r.read.side_effect=lambda *args:struct.pack('<dd',value,0)
            with self.assertRaisesRegex(Refusal,'cache_transform_value'):
                transforms.translation_inputs(r,1,2)
            self.assertEqual(r.transform_inputs_cache,{})

    def test_unknown_chain_does_not_read_geometry_or_space_name(self):
        r=reader(head=2000)
        with patch.object(transforms,'_descriptor',return_value=(5,None)):
            self.assertEqual(transforms.translation_inputs(r,1,2),
                             (None,'cache_transform_chain_unsupported'))
        r.read.assert_not_called()

    def test_affine_element_is_not_silently_treated_as_translation(self):
        r=reader(head=2000)
        with patch.object(transforms,'_descriptor',side_effect=[(5,'Element'),(6,'AnyElement')]), \
             patch.object(transforms,'_fields'), \
             patch.object(transforms,'_argument',return_value=(7,'AffineTransformElement')):
            self.assertEqual(transforms.translation_inputs(r,1,2),
                             (None,'cache_transform_chain_unsupported'))
        r.read.assert_not_called()

    def test_corrupt_depth_and_cycle_refuse_without_partial_input_result(self):
        for depth,next_head in ((0,0),(17,0),(1,2000),(2,2000)):
            r=reader(head=2000,depth=depth,next_head=next_head)
            with patch.object(transforms,'_descriptor',side_effect=[(5,'Element'),(6,'AnyElement')]), \
                 patch.object(transforms,'_fields'), \
                 patch.object(transforms,'_argument',return_value=(7,'SizedSpaceElement')):
                with self.assertRaises(Refusal):transforms.translation_inputs(r,1,2)
            self.assertEqual(r.transform_inputs_cache,{})

    def test_typed_space_payload_is_not_read(self):
        r=reader(head=2000)
        with patch.object(transforms,'_descriptor',side_effect=[(5,'Element'),(6,'AnyElement')]), \
             patch.object(transforms,'_fields'), \
             patch.object(transforms,'_argument',return_value=(7,'SizedSpaceElement')):
            result,reason=transforms.translation_inputs(r,1,2)
        self.assertIsNone(reason)
        self.assertEqual(result['chain'],((7.0,-11.0),))
        self.assertNotIn(2048,[call.args[0] for call in r.read.call_args_list])

    def test_translation_payload_requires_exact_layout(self):
        r=reader();r.value_size=lambda _: (24,24,8)
        r.u32=lambda address:2
        with self.assertRaisesRegex(Refusal,'cache_transform_translation_layout'):
            transforms._translation(r,1,2048)
        r.read.assert_not_called()


if __name__=='__main__':unittest.main()
