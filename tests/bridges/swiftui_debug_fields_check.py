"""Privacy/ABI negatives: assert forbidden bytes are never read."""
import sys
sys.dont_write_bytecode = True
import struct
from pathlib import Path
from types import SimpleNamespace
import unittest
sys.path.insert(0, str(Path(__file__).resolve().parents[2]/'plugins/macos'))
from swiftui_debug_fields import Fields
from swiftui_debug_snapshot import Refusal


class Memory:
    def __init__(self):
        self.bytes = {}
        self.reads = []
        self.p = SimpleNamespace(FixAddress=lambda address, _: address & ((1 << 56)-1))
        self.lldb = SimpleNamespace(eAddressMaskTypeData=1)
    def put(self, address, data):
        self.bytes.update({address+i: byte for i, byte in enumerate(data)})
    def read(self, address, size):
        self.reads.append((address, address+size))
        return bytes(self.bytes[address+i] for i in range(size))
    def overlap(self, start, end):
        return any(a < end and b > start for a, b in self.reads)


class Guards(unittest.TestCase):
    def setup_fields(self):
        c = Memory()
        f = Fields(c, dict(limits=dict(max_depth=256, max_nodes=256)))
        f.types = dict(array=0x2000, dictionary=0x3000, point=0x4000, size=0x5000)
        f.isa_mask = (1 << 64)-1
        f.empty_array, f.empty_dictionary = 0x6000, 0x7000
        return c, f

    def dictionary(self, tags):
        c, f = self.setup_fields()
        c.put(0x1000, struct.pack('<Q', f.types['dictionary']))
        header = bytearray(48)
        struct.pack_into('<QQ', header, 0, len(tags), 3)
        struct.pack_into('<b', header, 16, 2)
        struct.pack_into('<QQ', header, 32, 0x1048, 0x1050)
        c.put(0x1010, header)
        c.put(0x1040, struct.pack('<Q', (1 << len(tags))-1))
        c.put(0x1048, bytes(tags))
        return c, f

    def test_storage_metadata_precedes_header_or_payload(self):
        for label in ('array', 'dictionary'):
            c, f = self.setup_fields()
            c.put(0x1000, struct.pack('<Q', 0xdead))
            with self.assertRaises(Refusal):
                (f.walk if label == 'array' else f.properties)(0x1000)
            self.assertEqual(c.reads, [(0x1000, 0x1008)])
            c.reads.clear()
            with self.assertRaises(Refusal):
                (f.walk if label == 'array' else f.properties)((1 << 63) | 0x1000)
            self.assertEqual(c.reads, [])

    def test_empty_requires_exact_singleton_and_reads_nothing(self):
        c, f = self.setup_fields()
        self.assertEqual(f.walk(f.empty_array), [])
        self.assertEqual(f.properties(f.empty_dictionary), [])
        self.assertEqual(c.reads, [])

    def test_model_slot_is_never_read_and_geometry_is_exact(self):
        c, f = self.dictionary([1, 3, 4])
        # Intentionally provide NO bytes for the model's entire Any payload.
        for bucket, metadata, pair in [(1, f.types['point'], (-.25, 3.125)),
                                       (2, f.types['size'], (83.5, 16.25))]:
            slot = 0x1050+bucket*32
            c.put(slot+24, struct.pack('<Q', metadata))
            c.put(slot, struct.pack('<dd', *pair))
        props = f.properties(0x1000)
        self.assertEqual(props[0]['attribute']['value'], (-.25, 3.125))
        self.assertEqual(props[1]['attribute']['value'], (83.5, 16.25))
        self.assertFalse(c.overlap(0x1050, 0x1070))

    def test_wrong_any_metadata_refuses_before_value_read(self):
        c, f = self.dictionary([3])
        c.put(0x1068, struct.pack('<Q', 0xdead))
        with self.assertRaises(Refusal) as error:
            f.properties(0x1000)
        self.assertEqual(error.exception.code, 'geometry_metadata_mismatch')
        self.assertFalse(c.overlap(0x1050, 0x1068))

    def test_boxed_or_wrong_size_witness_is_rejected(self):
        for size, stride, flags in [(16, 16, 7 | 0x20000), (8, 8, 7), (16, 32, 7), (16, 16, 3)]:
            c, f = self.setup_fields()
            c.put(0x3ff8, struct.pack('<Q', 0x8000))
            c.put(0x8040, struct.pack('<QQII', size, stride, flags, 0))
            with self.assertRaises(Refusal):
                f.witness(0x4000, 16, 8, True)

    def test_array_limit_precedes_element_reads(self):
        c, f = self.setup_fields()
        c.put(0x1000, struct.pack('<Q', f.types['array']))
        c.put(0x1010, struct.pack('<QQ', 257, 1024))
        with self.assertRaises(Refusal): f.walk(0x1000)
        self.assertFalse(c.overlap(0x1020, 0x3000))

    def test_nonfinite_geometry_and_duplicate_keys_refuse(self):
        c, f = self.dictionary([3])
        c.put(0x1068, struct.pack('<Q', f.types['point']))
        c.put(0x1050, struct.pack('<dd', float('nan'), 1))
        with self.assertRaises(Refusal): f.properties(0x1000)
        c, f = self.dictionary([3, 3])
        with self.assertRaises(Refusal): f.properties(0x1000)
        self.assertFalse(c.overlap(0x1050, 0x1090))


if __name__ == '__main__':
    unittest.main()
