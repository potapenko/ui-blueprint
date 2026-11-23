"""Bounded native-backed overlay selection inside one admitted SwiftUI graph.

The existing CacheReader owns process/window/type admission, privacy and lifecycle.
This helper adds no attach, target launch, model predicates or geometry fallback.
"""
from bisect import bisect_right
import struct

from swiftui_debug_snapshot import Refusal


def cached_read(reader, address, size):
    """Return a validated structural-header slice, or None for the normal reader."""
    starts = getattr(reader, 'native_parts_addresses', ())
    index = bisect_right(starts, address)-1
    if size <= 0 or index < 0:
        return None
    start = starts[index]
    if address+size > start+32:
        return None
    if any(address < end and address+size > begin for begin, end in reader.private_ranges):
        raise Refusal('cache_private_range')
    reader.native_parts_reused_reads += 1
    reader.reads.append((address, address+size))
    return reader.native_parts_headers[start][address-start:address-start+size]


def record_count(reader, element=None):
    records = (getattr(reader, 'native_parts_indexed', set()) | reader.raw_seen
               | getattr(reader, 'transform_context_seen', set()))
    return len(records if element is None else records | {element})


def check_budget(reader, element=None):
    if record_count(reader, element) > 1024:
        raise Refusal('native_parts_record_limit', 2)


def represented_layout(reader, view):
    """Reuse immutable class layout; revalidate the actual provider/weak binding."""
    r = reader
    platform = r.c.objc(view, 'superview')
    cls = r.pointer(r.u64(platform) & r.isa_mask)
    descriptor = r.ptr(cls+64, 4)
    address = r.c.t.ResolveLoadAddress(descriptor)
    symbol = address.GetSymbol()
    if (address.GetModule().GetFileSpec().GetFilename() != 'SwiftUI'
            or symbol.GetStartAddress().GetLoadAddress(r.c.t) != descriptor
            or symbol.GetName() != 'nominal type descriptor for SwiftUI.AppKitPlatformViewHost'):
        raise Refusal('anchor_parent_unqualified')
    layouts = getattr(r, 'native_parts_class_layouts', None)
    if layouts is None:
        layouts = r.native_parts_class_layouts = {}
    if cls not in layouts:
        if len(layouts) >= 128:
            raise Refusal('native_parts_class_limit', 2)
        extent = r.c.call('class_getInstanceSize', cls)[0]
        offsets = []
        for key, size in (('coreRepresentedViewProvider', 8), ('coreLayoutInvalidator', 9)):
            data = key.encode('ascii')+b'\0'
            error = r.c.lldb.SBError()
            if r.c.p.WriteMemory(r.c.memory, data, error) != len(data) or error.Fail():
                raise Refusal('ivar_lookup_input')
            ivar = r.c.call('class_getInstanceVariable', cls, r.c.memory)[0]
            if not ivar:
                raise Refusal('ivar_unavailable')
            offset = r.c.call('ivar_getOffset', ivar)[0]
            if offset < 8 or offset+size > extent:
                raise Refusal('ivar_extent')
            offsets.append(offset)
        layouts[cls] = descriptor, tuple(offsets)
    saved_descriptor, (provider_offset, invalidator_offset) = layouts[cls]
    if saved_descriptor != descriptor:
        raise Refusal('native_parts_class_changed')
    if r.ptr(platform+provider_offset) != view:
        raise Refusal('anchor_provider_mismatch')
    slot = platform+invalidator_offset
    if r.u8(slot+8) != 0:
        raise Refusal('anchor_invalidator_absent')
    attribute = r.u32(slot)
    subgraph = r.page_owner(attribute)
    if (r.ptr(subgraph+0x28) != r.graph
            or r.u32(slot+4) != r.u32(subgraph+0x18) & 0x7fffffff):
        raise Refusal('expired_layout_reference')
    return attribute, subgraph


class _Selection:
    def __init__(self, reader):
        self.r = reader
        self.scope = reader.req['scope']
        self.records, self.types, self.owners = {}, {}, {}

    def record(self, element):
        if element not in self.records:
            if not element:
                raise Refusal('native_parts_root_unavailable')
            if len(self.records) >= 1024:
                raise Refusal('native_parts_record_limit', 2)
            owner = self.r.page_owner(element)
            p = self.r.owned_record(element, 32)
            self.records[element] = owner, struct.unpack('<QIIIIII', self.r.read(p, 32))
        return self.records[element]

    def names(self, element):
        if element not in self.types:
            _, data = self.record(element)
            metadata = self.r.pointer(data[0])
            name = self.r.qualified_nominal(metadata) if metadata else None
            attribute, owner = self.r.rule_owner(data[1], self.owners)
            self.types[element] = name, attribute, owner
        return self.types[element]

    def witnesses(self, bindings):
        r = self.r
        wanted = {attribute for attribute, _ in bindings}
        matches = {attribute: [] for attribute in wanted}
        indexed = set()
        for subgraph in {subgraph for _, subgraph in bindings}:
            pending = [(r.u32(subgraph+0x60), 1)]
            while pending:
                element, depth = pending.pop()
                if depth > r.req['limits']['max_depth']:
                    raise Refusal('native_parts_index_depth', 2)
                if element in indexed:
                    raise Refusal('native_parts_index_cycle')
                indexed.add(element)
                owner, data = self.record(element)
                if owner != subgraph:
                    raise Refusal('anchor_parent_ownership')
                value, properties = data[6], set()
                while value:
                    if value in properties or len(properties) >= 32:
                        raise Refusal('cache_value_cycle')
                    properties.add(value)
                    v = r.owned_record(value, 24)
                    table = r.ptr(r.graph+0x120)
                    index, count = r.u32(v+12), r.u32(table+8)
                    if index >= count or count > 4096:
                        raise Refusal('cache_key_limit')
                    key = r.cstring(r.ptr(r.ptr(table)+index*8), 64)
                    if key == 'layoutComputer':
                        attribute = r.u32(v+8)
                        if attribute in wanted:
                            matches[attribute].append(element)
                    value = r.u32(v+20)
                child, siblings = data[4], set()
                while child:
                    if child in siblings:
                        raise Refusal('native_parts_index_cycle')
                    siblings.add(child)
                    if len(pending)+len(indexed) >= 1024:
                        raise Refusal('native_parts_record_limit', 2)
                    pending.append((child, depth+1))
                    child = self.record(child)[1][5]
        if any(len(matches[attribute]) != 1 for attribute in wanted):
            raise Refusal('anchor_scope_binding_unresolved')
        return {attribute: elements[0] for attribute, elements in matches.items()}, len(indexed)

    def ancestor(self, element):
        r = self.r
        seen, overlay, source_owners, container = set(), None, set(), False
        while element:
            if element in seen:
                raise Refusal('native_parts_parent_cycle')
            if len(seen) >= r.req['limits']['max_depth']:
                raise Refusal('anchor_parent_limit', 2)
            seen.add(element)
            owner, data = self.record(element)
            name, attribute, body = self.names(element)
            if overlay is None and name == 'SwiftUI._OverlayModifier':
                overlay = element
            elif overlay is not None and name == self.scope['containing_type']:
                container = True
            if body == self.scope['type']:
                source_owners.add(attribute)
            if element == r.graph_root:
                if not container or overlay is None:
                    return None
                if len(source_owners) != 1:
                    raise Refusal('native_parts_owner_ambiguous')
                return overlay, next(iter(source_owners))
            if data[3]:
                parent = data[3]
                if self.record(parent)[0] != owner:
                    raise Refusal('anchor_parent_ownership')
                element = parent
                continue
            if r.u32(owner+0x60) != element:
                raise Refusal('anchor_subgraph_root')
            resolved, _ = r.resolve(data[1], links_only=True)
            if resolved is None:
                break
            parents = {(sg, parent) for sg, pairs in r.links.items()
                       for parent, attr in pairs if attr == resolved[0]}
            if len(parents) != 1:
                break
            parent_owner, element = parents.pop()
            if self.record(element)[0] != parent_owner:
                raise Refusal('anchor_parent_ownership')
        raise Refusal('native_parts_ancestry_incomplete')

    def select(self, host):
        r = self.r
        views = r.native_views(host, self.scope['native_view_type'])
        window = r.c.objc(host, 'window')
        if not window or any(r.c.objc(view, 'window') != window for view in views):
            raise Refusal('native_parts_window_mismatch')
        bindings = [represented_layout(r, view) for view in views]
        if not bindings or len({attribute for attribute, _ in bindings}) != len(bindings):
            raise Refusal('native_anchor_unresolved')
        matches, indexed_count = self.witnesses(bindings)
        selected = []
        for attribute, _ in bindings:
            result = self.ancestor(matches[attribute])
            if result is not None:
                overlay, owner = result
                selected.append((attribute, overlay, owner, matches[attribute]))
        if not selected or len({owner for _, _, owner, _ in selected}) != 1:
            raise Refusal('native_parts_owner_ambiguous')
        if len({element for _, element, _, _ in selected}) != len(selected):
            raise Refusal('native_parts_boundary_ambiguous')
        r.native_parts_indexed = set(self.records)
        r.native_parts_headers = {r.base+offset: struct.pack('<QIIIIII', *data)
                                  for offset, (_, data) in self.records.items()}
        r.native_parts_addresses = sorted(r.native_parts_headers)
        if any(right < left+32 for left, right in
               zip(r.native_parts_addresses, r.native_parts_addresses[1:])):
            raise Refusal('native_parts_record_overlap')
        r.native_parts_reused_reads = 0
        r.anchor_ids = {attribute for attribute, _, _, _ in selected}
        r.anchor_matches, r.anchor_counts, r.matched_anchor_nodes = 0, {}, {}
        r.native_part_witnesses = {witness for _, _, _, witness in selected}
        r.owner_scan_cost = 0
        r.scan_work = r.scanned_records = len(self.records)
        r.selected_source = dict(selection_source='native_overlay_parts', source_type=self.scope['type'])
        r.native_parts_selection = dict(native_candidates=len(views), selected_parts=len(selected),
                                       indexed_records=indexed_count, discovery_records=len(self.records))
        return [element for _, element, _, _ in selected]


def select_parts(reader, host):
    scope = reader.req['scope']
    if (set(scope) != {'native_match', 'type', 'containing_type', 'native_view_type', 'selection'}
            or scope['native_match'] != 'overlays_in_owner' or scope['selection'] != 'geometry_records'):
        raise Refusal('invalid_scope', 2)
    return _Selection(reader).select(host)


def verify_parts(reader):
    check_budget(reader)
    if (reader.anchor_counts != {attribute: 1 for attribute in reader.anchor_ids}
            or not reader.native_part_witnesses <= reader.raw_seen):
        raise Refusal('anchor_scope_binding_unresolved')
