"""Privacy and union-accounting checks for the fixture structural cache."""
from pathlib import Path
import sys
from types import SimpleNamespace
import unittest
from unittest.mock import Mock

sys.dont_write_bytecode = True
sys.path.insert(0, str(Path(__file__).resolve().parents[3]/'plugins/macos'))
from swiftui_debug_snapshot import Refusal
import record_reuse


def reader():
    return SimpleNamespace(base=10000, native_parts_record_cache={
        1024: (99, (1, 2, 3, 4, 5, 6, 7)),
        1056: (99, (8, 9, 10, 11, 12, 13, 14)),
    }, raw_seen=set(), private_ranges=[], reads=[])


class ReuseChecks(unittest.TestCase):
    def test_reuse_returns_only_already_validated_header_bytes(self):
        r = reader(); record_reuse.prepare(r)
        original = Mock(side_effect=AssertionError('unexpected target read'))
        self.assertEqual(record_reuse.read(r, 11024+8, 4, original), (2).to_bytes(4, 'little'))
        self.assertEqual(r.native_parts_reused_reads, 1)
        self.assertEqual(r.reads, [(11032, 11036)])

    def test_cross_header_or_outside_read_stays_with_normal_reader(self):
        r = reader(); record_reuse.prepare(r)
        original = Mock(return_value=b'normal')
        for address, size in ((11024+31, 2), (11023, 4), (11088, 4), (11024, 0)):
            self.assertEqual(record_reuse.read(r, address, size, original), b'normal')
        self.assertEqual(original.call_count, 4)
        self.assertEqual(r.native_parts_reused_reads, 0)

    def test_later_model_exclusion_also_applies_to_cached_bytes(self):
        r = reader(); record_reuse.prepare(r)
        r.private_ranges.append((11032, 11040))
        original = Mock()
        with self.assertRaisesRegex(Refusal, 'cache_private_range'):
            record_reuse.read(r, 11032, 4, original)
        original.assert_not_called()

    def test_unique_record_ceiling_is_not_an_expanded_additive_budget(self):
        r = reader(); record_reuse.prepare(r)
        r.native_parts_indexed = set(range(1024))
        r.raw_seen = set(range(100))
        r.transform_context_seen = {900}
        self.assertEqual(record_reuse.check_budget(r, 10), 1024)
        with self.assertRaisesRegex(Refusal, 'native_parts_record_limit'):
            record_reuse.check_budget(r, 1024)

    def test_overlapping_header_bindings_refuse(self):
        r = reader()
        r.native_parts_record_cache[1040] = (99, (0, 0, 0, 0, 0, 0, 0))
        with self.assertRaisesRegex(Refusal, 'native_parts_record_overlap'):
            record_reuse.prepare(r)


if __name__ == '__main__':
    unittest.main()
