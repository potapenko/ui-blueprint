"""Observation-local reuse of explicitly identified TreeElement32 headers.

The caller owns structural type/path identification. Never wrap generic
owned_record(size=32): attribute/model storage can have the same size.
"""
from bisect import bisect_right, insort

from swiftui_debug_snapshot import Refusal


class AncestorCache:
    def __init__(self, reader):
        if reader.req.get('scope', {}).get('native_match') != 'ancestor_owner':
            raise Refusal('ancestor_cache_scope', 2)
        self.reader = reader
        self.binding = self._binding()
        self.elements = set()
        self.headers = {}
        self.addresses = []
        self.reused_reads = 0

    def _binding(self):
        r = self.reader
        return r.graph, r.table, r.base, r.limit

    def _current(self):
        if self._binding() != self.binding:
            raise Refusal('ancestor_cache_binding')

    def record_count(self, element=None):
        self._current()
        r = self.reader
        elements = self.elements | r.raw_seen | getattr(r, 'transform_context_seen', set())
        return len(elements if element is None else elements | {element})

    def admit(self, element=None):
        if self.record_count(element) > 1024:
            raise Refusal('ancestor_cache_record_limit', 2)

    def tree_element(self, element):
        """Validate an explicit structural record; return its source address."""
        if type(element) is not int or element <= 0:
            raise Refusal('ancestor_cache_element', 2)
        self.admit(element)
        r = self.reader
        address = r.owned_record(element, 32)
        self._current()
        self._privacy(address, 32)
        if element not in self.elements:
            index = bisect_right(self.addresses, address)
            if ((index and self.addresses[index-1]+32 > address)
                    or (index < len(self.addresses) and address+32 > self.addresses[index])):
                raise Refusal('ancestor_cache_header_overlap')
            # Do not publish a cache entry before the bounded source read succeeds.
            data = r.read(address, 32)
            if not isinstance(data, bytes) or len(data) != 32:
                raise Refusal('ancestor_cache_short_read')
            self.headers[address] = data
            insort(self.addresses, address)
            self.elements.add(element)
        elif address not in self.headers:
            raise Refusal('ancestor_cache_binding')
        return address

    def _privacy(self, address, size):
        if any(address < end and address+size > start for start, end in self.reader.private_ranges):
            raise Refusal('cache_private_range')

    def cached_read(self, address, size):
        """Return a header slice or None; unknown memory stays on the normal path."""
        self._current()
        if size <= 0:
            return None
        index = bisect_right(self.addresses, address)-1
        if index < 0:
            return None
        start = self.addresses[index]
        if address+size > start+32:
            return None
        self._privacy(address, size)
        self.reader.reads.append((address, address+size))
        self.reused_reads += 1
        return self.headers[start][address-start:address-start+size]
