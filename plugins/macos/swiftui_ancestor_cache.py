"""Observation-local reuse of explicitly identified TreeElement32 headers.

The caller owns structural type/path identification. Never wrap generic
owned_record(size=32): attribute/model storage can have the same size.
"""
from bisect import bisect_right, insort

from swiftui_debug_snapshot import Refusal


def witness_path_union(root, paths, max_depth):
    """Merge already source-validated root-to-witness paths, never discover one.

    paths maps unique native layout attributes to their recorded parent->child
    chains. The selector owns window/graph/source-owner proof before this call.
    Returned child order is unspecified and must not become semantic ordering.
    """
    if (type(root) is not int or root <= 0 or not isinstance(paths, dict)
            or not 1 <= len(paths) <= 128 or type(max_depth) is not int
            or not 1 <= max_depth <= 256):
        raise Refusal('ancestor_witness_paths', 2)
    children, parents, endpoints, union = {}, {}, set(), set()
    for attribute, path in paths.items():
        if type(attribute) is not int or attribute <= 0 or not isinstance(path, dict):
            raise Refusal('ancestor_witness_paths', 2)
        current, seen = root, set()
        while current is not None:
            if (type(current) is not int or current <= 0 or current in seen
                    or current not in path or len(seen) >= max_depth):
                raise Refusal('ancestor_witness_path_incomplete')
            seen.add(current);union.add(current)
            if len(union) > 1024:
                raise Refusal('ancestor_cache_record_limit', 2)
            child = path[current]
            if child is None:
                if current in endpoints or children.get(current):
                    raise Refusal('ancestor_witness_ambiguous')
                endpoints.add(current)
            else:
                if type(child) is not int or child <= 0:
                    raise Refusal('ancestor_witness_paths', 2)
                if current in endpoints or (child in parents and parents[child] != current):
                    raise Refusal('ancestor_witness_ambiguous')
                parents[child] = current
                children.setdefault(current, set()).add(child)
            current = child
    return {element: tuple(values) for element, values in children.items()}


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
