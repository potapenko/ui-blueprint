"""Reuse validated TreeElement headers only within one stopped observation.

This cache never includes model payloads or persists target addresses. The normal
reader owns graph/type validation and the actual source-read byte limit. Every
cache hit rechecks protected ranges; selected/raw/context record accounting uses
the same unique-record ceiling as discovery.
"""
from bisect import bisect_right
import struct

from swiftui_debug_snapshot import Refusal


def prepare(reader):
    records = reader.native_parts_record_cache
    if len(records) > 1024:
        raise Refusal('native_parts_record_limit', 2)
    reader.native_parts_indexed = set(records)
    reader.native_parts_headers = {
        reader.base+offset: struct.pack('<QIIIIII', *header)
        for offset, (_, header) in records.items()
    }
    reader.native_parts_addresses = sorted(reader.native_parts_headers)
    if any(right < left+32 for left, right in
           zip(reader.native_parts_addresses, reader.native_parts_addresses[1:])):
        raise Refusal('native_parts_record_overlap')
    reader.native_parts_reused_reads = 0
    # The caller must enforce check_budget at each walk/context visit. Discovery
    # must not also be charged by the legacy additive owner_scan_cost mechanism.
    reader.owner_scan_cost = 0


def check_budget(reader, element=None):
    records = (reader.native_parts_indexed | reader.raw_seen
               | getattr(reader, 'transform_context_seen', set()))
    if element is not None:
        records = records | {element}
    if len(records) > 1024:
        raise Refusal('native_parts_record_limit', 2)
    return len(records)


def read(reader, address, size, original):
    starts = getattr(reader, 'native_parts_addresses', ())
    index = bisect_right(starts, address)-1
    if size > 0 and index >= 0:
        start = starts[index]
        if start <= address and address+size <= start+32:
            if any(address < end and address+size > begin
                   for begin, end in reader.private_ranges):
                raise Refusal('cache_private_range')
            reader.native_parts_reused_reads += 1
            reader.reads.append((address, address+size))
            return reader.native_parts_headers[start][address-start:address-start+size]
    return original(reader, address, size)
