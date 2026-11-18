"""Exact-profile cached geometry; never evaluate AttributeGraph or copy models."""
import math
import struct
from swiftui_debug_fields import IMAGES
from swiftui_debug_snapshot import Refusal, encode

PROFILE = {**IMAGES, 'AttributeGraph': '6B3E4A3F-D05C-3A1B-B187-18075DC4DCC0'}
GEOMETRY = {
    'CGPoint': ('$sSo7CGPointVMn', 16, (0, 8)),
    'CGSize': ('$sSo6CGSizeVMn', 16, (0, 8)),
    'ViewSize': ('$s7SwiftUI8ViewSizeVMn', 32, (0, 16)),
}


class CacheReader:
    def __init__(self, calls, req):
        self.c, self.req = calls, req
        self.symbols, self.names, self.hosts = {}, {}, {}
        self.reads, self.private_ranges, self.geometry_ranges = [], [], []
        self.graph = self.table = self.base = self.limit = None
        self.nodes, self.raw_seen = [], set()
        self.unknown_order = 0
        actual = {m.GetFileSpec().GetFilename(): m.GetUUIDString()
                  for m in calls.t.modules if m.GetFileSpec().GetFilename() in PROFILE}
        if actual != PROFILE:
            raise Refusal('cache_image_profile')
        self.isa_mask = self.u64(calls.symbol('objc_debug_isa_class_mask'))
        self.host_descriptor = self.symbol('$s7SwiftUI13NSHostingViewCMn', 'SwiftUI')
        self.graph_descriptor = self.symbol('$s7SwiftUI9ViewGraphCMn', 'SwiftUICore')

    def pointer(self, value):
        return self.c.p.FixAddress(value, self.c.lldb.eAddressMaskTypeData)

    def read(self, address, size):
        if address <= 0 or size < 0 or address+size > 1 << 64:
            raise Refusal('cache_address_range')
        if any(address < end and address+size > start for start, end in self.private_ranges):
            raise Refusal('cache_private_range')
        self.reads.append((address, address+size))
        return self.c.read(address, size)

    def u64(self, address):
        return struct.unpack('<Q', self.read(address, 8))[0]

    def u32(self, address):
        return struct.unpack('<I', self.read(address, 4))[0]

    def u16(self, address):
        return struct.unpack('<H', self.read(address, 2))[0]

    def u8(self, address):
        return self.read(address, 1)[0]

    def ptr(self, address, alignment=8):
        value = self.pointer(self.u64(address))
        if not value or value & (alignment-1):
            raise Refusal('cache_pointer_shape')
        return value

    def symbol(self, name, module):
        key = module, name
        if key not in self.symbols:
            found = {x.GetSymbol().GetStartAddress().GetLoadAddress(self.c.t)
                     for x in self.c.t.FindSymbols(name)
                     if x.GetModule().GetFileSpec().GetFilename() == module}
            found.discard(self.c.lldb.LLDB_INVALID_ADDRESS)
            if not found:
                for image in self.c.t.modules:
                    if image.GetFileSpec().GetFilename() == module:
                        found.update(x.GetStartAddress().GetLoadAddress(self.c.t)
                                     for x in image if x.GetName() == name)
            if len(found) != 1:
                raise Refusal('cache_symbol_unresolved')
            self.symbols[key] = found.pop()
        return self.symbols[key]

    def cstring(self, address, maximum=256):
        # A large convenience read could cross into an adjacent model literal.
        value = bytearray()
        for offset in range(maximum):
            byte = self.read(address+offset, 1)[0]
            if byte == 0:
                try:
                    return value.decode('utf-8')
                except UnicodeError:
                    break
            value.append(byte)
        raise Refusal('cache_metadata_string')

    def nominal(self, metadata):
        metadata = self.pointer(metadata)
        if not metadata:
            return None
        kind = self.u64(metadata)
        if kind not in (0x200, 0x201, 0x202):
            return None
        descriptor = self.ptr(metadata+8, 4)
        if descriptor not in self.names:
            flags = self.u32(descriptor)
            if flags & 31 not in (16, 17, 18):
                return None
            relative = struct.unpack('<i', self.read(descriptor+8, 4))[0]
            name_address = descriptor+8+relative
            if not self.c.t.ResolveLoadAddress(name_address).GetSection().IsValid():
                raise Refusal('cache_metadata_string')
            self.names[descriptor] = self.cstring(name_address)
        return self.names[descriptor]

    def value_size(self, metadata):
        witness = self.ptr(metadata-8)
        size, stride, flags = struct.unpack('<QQI', self.read(witness+64, 20))
        if size > 1048576 or stride < size or stride > 1048576:
            raise Refusal('cache_value_layout')
        return size, stride, (flags & 0xffff)+1

    def host_graph(self, view):
        if view in self.hosts:
            return self.hosts[view]
        original = self.pointer(self.u64(view) & self.isa_mask)
        metadata, visited = original, set()
        while metadata:
            if metadata in visited or len(visited) >= 16:
                raise Refusal('cache_class_hierarchy')
            visited.add(metadata)
            if self.u64(metadata+32) & 2:
                descriptor = self.pointer(self.u64(metadata+64))
                if descriptor == self.host_descriptor:
                    offset = self.u64(metadata+0x68)
                    if self.u64(original+0x68) != offset:
                        raise Refusal('cache_host_field_layout')
                    # Swift metadata's size does not include every resilient ObjC
                    # superclass byte. Use the public runtime's actual extent.
                    extent = self.c.call('class_getInstanceSize', original)[0]
                    if not 512 <= offset or offset+8 > extent or extent > 1048576:
                        raise Refusal('cache_host_field_layout')
                    graph_view = self.ptr(view+offset)
                    graph_meta = self.pointer(self.u64(graph_view) & self.isa_mask)
                    if self.ptr(graph_meta+64, 4) != self.graph_descriptor:
                        raise Refusal('cache_viewgraph_identity')
                    if self.c.call('class_getInstanceSize', graph_meta)[0] < 48:
                        raise Refusal('cache_viewgraph_layout')
                    self.hosts[view] = graph_view
                    return graph_view
            metadata = self.pointer(self.u64(metadata+8))
        self.hosts[view] = None
        return None

    def address(self, offset, size):
        if offset < 512 or offset+size > self.limit:
            raise Refusal('cache_offset_limit')
        return self.base+offset

    def page_owner(self, offset):
        page = offset & ~511
        self.address(page, 8)
        index = (page >> 9)-1
        words = self.u32(self.table+0x60)
        if index >> 6 >= words or words > 131072:
            raise Refusal('cache_page_unavailable')
        bitmap = self.ptr(self.table+0x58)
        if not self.u64(bitmap+(index >> 6)*8) & (1 << (index & 63)):
            raise Refusal('cache_page_unavailable')
        owner = self.ptr(self.base+page)
        if self.u8(owner+0x68):
            raise Refusal('cache_invalid_subgraph')
        return owner

    def owned_record(self, offset, size):
        # Validate every page spanned by the range, not merely the starting
        # record's page. Table bounds alone do not establish graph ownership.
        if size < 0 or size > 1048576:
            raise Refusal('cache_offset_limit')
        for page in range(offset & ~511, ((offset+max(size, 1)-1) & ~511)+512, 512):
            owner = self.page_owner(page)
            if self.ptr(owner+0x28) != self.graph:
                raise Refusal('cache_foreign_graph')
        return self.address(offset, size)

    def resolve(self, attribute, links_only=False):
        delta, visited, first = 0, set(), attribute
        while attribute & 3 == 1:
            if attribute in visited or len(visited) >= 16:
                raise Refusal('cache_indirect_cycle')
            visited.add(attribute)
            p = self.owned_record(attribute & ~3, 24)
            flags = self.u32(p+16)
            source = self.u32(p+8)
            if source < 4:
                return None, 'cache_value_unavailable'
            owner = self.page_owner(source)
            if (self.u32(owner+0x18) & 0x7fffffff) != self.u32(p+12):
                return None, 'cache_reference_stale'
            if flags & 1 and not links_only:
                return None, 'cache_value_dirty'
            delta += flags >> 3
            if delta > 1048576:
                raise Refusal('cache_value_layout')
            attribute = source
        if attribute < 4 or attribute & 3:
            return None, 'cache_value_unavailable'
        self.owned_record(attribute, 16)
        return (attribute, delta, first), None

    def attribute_info(self, attribute):
        p = self.owned_record(attribute, 16)
        flags = self.u32(p+4)
        index, count = flags >> 8, self.u32(self.graph+0x60)
        if not 0 < index < count or count > 65536:
            raise Refusal('cache_type_limit')
        descriptor = self.ptr(self.ptr(self.graph+0x58)+index*8)
        return p, descriptor, flags

    def cached_storage(self, p, flags, size):
        if flags & 0x11 != 0x10:
            return None
        offset = self.u32(p+8)
        if self.u8(p+3) & 2:
            # Box metadata is owned, but arbitrary heap pointees have no proven
            # graph allocation binding in this profile. Refuse before reading
            # the pointer or its payload rather than claim a complete audit.
            self.owned_record(offset, 8)
            raise Refusal('cache_box_ownership_unqualified')
        return self.owned_record(offset, size)

    def protect_model(self, model_id):
        resolved, _ = self.resolve(model_id, links_only=True)
        if resolved is None:
            raise Refusal('cache_model_protection_incomplete')
        attribute, _, _ = resolved
        p, descriptor, flags = self.attribute_info(attribute)
        # Cached View values and rule bodies are both excluded. Only their
        # metadata/box references are needed to establish these forbidden ranges.
        body_type = self.pointer(self.u64(descriptor))
        if body_type:
            size, _, _ = self.value_size(body_type)
            if size:
                offset = self.u32(descriptor+0x2c)
                if offset < 16 or offset+size > 1048576:
                    raise Refusal('cache_model_layout')
                body = p+offset
                if self.u8(p+3) & 1:
                    slot = self.owned_record(attribute+offset, 8)
                    body = self.ptr(slot)
                    # This address is only DENIED, never admitted for a read.
                    # Graph ownership is required for the pointer slot; no heap
                    # ownership claim is made about its forbidden pointee.
                else:
                    self.owned_record(attribute+offset, size)
                if body+size > 1 << 64:
                    raise Refusal('cache_model_layout')
                self.private_ranges.append((body, body+size))
        if flags & 0x10:
            metadata = self.ptr(descriptor+8)
            size, _, _ = self.value_size(metadata)
            # A dirty model value is still private; do not use the geometry
            # clean-cache predicate to omit its protected storage.
            if size:
                offset = self.u32(p+8)
                if self.u8(p+3) & 2:
                    value = self.ptr(self.owned_record(offset, 8))
                else:
                    value = self.owned_record(offset, size)
                if value+size > 1 << 64:
                    raise Refusal('cache_model_layout')
                self.private_ranges.append((value, value+size))

    def geometry_type(self, metadata, key):
        name = self.nominal(metadata)
        if name not in (('CGPoint',) if key == 'position' else ('CGSize', 'ViewSize')):
            raise Refusal('cache_geometry_type')
        mangled, size, offsets = GEOMETRY[name]
        descriptor = self.ptr(metadata+8, 4)
        symbol = self.c.t.ResolveLoadAddress(descriptor).GetSymbol()
        if (symbol.GetMangledName() != mangled
                or symbol.GetStartAddress().GetLoadAddress(self.c.t) != descriptor):
            raise Refusal('cache_geometry_descriptor')
        if self.value_size(metadata) != (size, size, 8) or self.u32(descriptor+20) != 2:
            raise Refusal('cache_geometry_layout')
        vector = self.u32(descriptor+24)
        if not 2 <= vector <= 8 or struct.unpack('<II', self.read(metadata+vector*8, 8)) != offsets:
            raise Refusal('cache_geometry_layout')
        return size

    def geometry(self, attribute, metadata, key):
        expected_size = self.geometry_type(metadata, key)
        resolved, reason = self.resolve(attribute)
        if resolved is None:
            return None, reason
        attribute, delta, first = resolved
        p, descriptor, flags = self.attribute_info(attribute)
        value_type = self.ptr(descriptor+8)
        if first == attribute:
            if value_type != metadata:
                raise Refusal('cache_geometry_type')
        else:
            indirect = self.owned_record(first & ~3, 24)
            packed = self.u32(indirect+4) | (self.u16(indirect+2) << 32)
            if self.u32(indirect+16) & 4:
                if packed != expected_size:
                    raise Refusal('cache_geometry_layout')
            elif self.pointer(packed) != metadata:
                raise Refusal('cache_geometry_type')
        size, _, _ = self.value_size(value_type)
        if delta+expected_size > size:
            raise Refusal('cache_value_bounds')
        value = self.cached_storage(p, flags, size)
        if value is None:
            return None, 'cache_value_dirty'
        pair = struct.unpack('<dd', self.read(value+delta, 16))
        self.geometry_ranges.append((value+delta, value+delta+16))
        if not all(math.isfinite(x) for x in pair) or (key == 'size' and min(pair) < 0):
            raise Refusal('cache_geometry_value')
        return pair, None

    def properties(self, element):
        p = self.owned_record(element, 32)
        value, visited, result = self.u32(p+28), set(), []
        while value:
            if value in visited or len(visited) >= 32:
                raise Refusal('cache_value_cycle')
            visited.add(value)
            v = self.owned_record(value, 24)
            table = self.ptr(self.graph+0x120)
            index, count = self.u32(v+12), self.u32(table+8)
            if index >= count or count > 4096:
                raise Refusal('cache_key_limit')
            key = self.cstring(self.ptr(self.ptr(table)+index*8), 64)
            if key == 'layoutComputer' and hasattr(self, 'anchor_id') and self.anchor_id == self.u32(v+8):
                self.anchor_matches += 1
            if key in ('position', 'size'):
                # No read of any unrequested TreeValue's type/value payload.
                result.append((key, self.ptr(v), self.u32(v+8)))
            value = self.u32(v+20)
        return result

    def associations(self):
        self.links, self.subgraphs = {}, []
        table = self.pointer(self.u64(self.graph+0x118))
        if table:
            entry, count = self.pointer(self.u64(table+16)), self.u64(table+24)
            if count > 1024:
                raise Refusal('cache_association_limit')
            seen = set()
            while entry:
                if entry in seen or len(seen) >= count:
                    raise Refusal('cache_association_cycle')
                seen.add(entry)
                owner, data, size = self.ptr(entry+16), self.pointer(self.u64(entry+24)), self.u64(entry+32)
                if size > 1024:
                    raise Refusal('cache_association_limit')
                self.links[owner] = [struct.unpack('<II', self.read(data+i*8, 8)) for i in range(size)]
                entry = self.pointer(self.u64(entry))
            if len(seen) != count:
                raise Refusal('cache_association_shape')
        array, count = self.pointer(self.u64(self.graph+0x128)), self.u32(self.graph+0x130)
        if count > 1024:
            raise Refusal('cache_subgraph_limit')
        for index in range(count):
            subgraph = self.ptr(array+index*8)
            if self.u8(subgraph+0x68):
                continue
            root = self.u32(subgraph+0x60)
            if not root:
                continue
            p = self.owned_record(root, 32)
            if self.page_owner(root) != subgraph:
                raise Refusal('cache_subgraph_root')
            resolved, _ = self.resolve(self.u32(p+8), links_only=True)
            if resolved:
                self.subgraphs.append((self.u32(subgraph+0x58), root, resolved[0]))

    def child_subgraphs(self, element):
        pairs = self.links.get(self.page_owner(element), [])
        models = {model for parent, model in pairs if parent == element}
        selected = [(order, root) for order, root, model in self.subgraphs if model in models]
        roots = {root for _, root in selected}
        if len(roots) != len(selected):
            raise Refusal('cache_duplicate_subgraph')
        known = True
        if len({order for order, _ in selected}) != len(selected):
            following = {root: self.u32(self.owned_record(root, 32)+24) for root in roots}
            heads, chain = roots-set(following.values()), []
            if len(heads) == 1:
                cursor = heads.pop()
                while cursor in roots and cursor not in chain:
                    chain.append(cursor)
                    cursor = following[cursor]
            if len(chain) == len(roots):
                return chain, True
            self.unknown_order += 1
            known = False
        return [root for _, root in sorted(selected, key=lambda item: item[0])], known

    def walk(self, element, parent=None, depth=1):
        p = self.owned_record(element, 32)
        if element in self.raw_seen or len(self.raw_seen) >= 1024 or depth > self.req['limits']['max_depth']:
            raise Refusal('cache_tree_limit', 2)
        self.raw_seen.add(element)
        metadata, flags, model = self.pointer(self.u64(p)), self.u32(p+12), self.u32(p+8)
        if not flags & 1 and metadata and model != self.nil_attribute:
            if len(self.nodes) >= self.req['limits']['max_nodes']:
                raise Refusal('tree_limit', 2)
            node = dict(parent=parent, metadata=metadata, model=model, fields=self.properties(element), children=[])
            index = len(self.nodes)
            self.nodes.append(node)
            if parent is not None:
                self.nodes[parent]['children'].append(index)
            parent = index
        child = self.u32(p+20)
        while child:
            self.walk(child, parent, depth+1)
            child = self.u32(self.owned_record(child, 32)+24)
        children, order_known = self.child_subgraphs(element)
        if not order_known and parent is not None:
            self.nodes[parent]['child_order_unknown'] = True
        for child in children:
            self.walk(child, parent, depth+1)

    def qualified_nominal(self, metadata):
        metadata = self.pointer(metadata)
        if not metadata or self.u64(metadata) not in (0x200, 0x201, 0x202):
            return None
        descriptor = self.ptr(metadata+8, 4)
        names, seen = [], set()
        while descriptor:
            if descriptor in seen or len(seen) >= 16:
                raise Refusal('cache_type_context')
            seen.add(descriptor)
            kind = self.u32(descriptor) & 31
            if kind not in (0, 16, 17, 18):
                return None  # anonymous/extension contexts are not source selectors
            offset = struct.unpack('<i', self.read(descriptor+8, 4))[0]
            name = self.cstring(descriptor+8+offset)
            if not name or any(not (c.isascii() and (c.isalnum() or c == '_')) for c in name):
                return None
            names.append(name)
            if kind == 0:
                return '.'.join(reversed(names))
            relative = struct.unpack('<i', self.read(descriptor+4, 4))[0]
            if not relative:
                return None
            address = descriptor+4+(relative & ~1)
            descriptor = self.ptr(address, 4) if relative & 1 else address
        return None

    def select_scope(self, root):
        scope = self.req.get('scope')
        if scope is None:
            self.scanned_records = 0
            return [root]
        seen, matches = set(), set()
        wanted_descendant = scope.get('containing_type')

        owner_names = {}
        active_owners = set()
        owner_roots = {}

        def rule_owner(model):
            resolved, _ = self.resolve(model, links_only=True)
            if resolved is None:
                return None, None
            attribute = resolved[0]
            if attribute not in owner_names:
                _, descriptor, _ = self.attribute_info(attribute)
                rule = self.pointer(self.u64(descriptor))
                name = None
                descriptor_addr = self.ptr(rule+8, 4) if rule and self.u64(rule)==0x200 else None
                symbol = self.c.t.ResolveLoadAddress(descriptor_addr).GetSymbol() if descriptor_addr else None
                if (symbol and symbol.GetStartAddress().GetLoadAddress(self.c.t) == descriptor_addr
                        and symbol.GetMangledName() in (
                            '$s7SwiftUI10StaticBody33_A4C1D658B3717A3062FEFC91A812D6EBLLVMn',
                            '$s7SwiftUI11DynamicBody33_A4C1D658B3717A3062FEFC91A812D6EBLLVMn')):
                    accessor = self.ptr(rule+16)
                    if self.qualified_nominal(accessor) == 'SwiftUI.ViewBodyAccessor':
                        name = self.qualified_nominal(self.ptr(accessor+16))
                owner_names[attribute] = name
            return attribute, owner_names[attribute]

        def scan(element, depth):
            p = self.owned_record(element, 32)
            if element in seen or len(seen) >= 1024 or depth > self.req['limits']['max_depth']:
                raise Refusal('cache_scope_scan_limit', 2)
            if hasattr(self, 'scan_work'):
                self.scan_work += 1
                if self.scan_work > 1024:
                    raise Refusal('cache_scope_scan_limit', 2)
            seen.add(element)
            metadata = self.pointer(self.u64(p))
            name = self.qualified_nominal(metadata) if metadata else None
            attribute, owner_name = rule_owner(self.u32(p+8))
            owner_root = owner_name is not None and attribute not in active_owners
            if owner_root:
                active_owners.add(attribute)
                owner_roots.setdefault(attribute, []).append(element)
            names = {name, owner_name if owner_root else None}
            descendant_found = False
            child = self.u32(p+20)
            while child:
                descendant_found |= scan(child, depth+1)
                child = self.u32(self.owned_record(child, 32)+24)
            children, _ = self.child_subgraphs(element)
            for child in children:
                descendant_found |= scan(child, depth+1)
            if owner_root:
                active_owners.remove(attribute)
            if scope['type'] in names and (wanted_descendant is None or descendant_found):
                matches.add(('owner', attribute) if owner_name == scope['type'] else ('element', element))
            return descendant_found or (wanted_descendant is not None and wanted_descendant in names)

        scan(root, 1)
        self.scanned_records = len(seen)
        if len(matches) != 1:
            raise Refusal('cache_scope_unresolved' if not matches else 'cache_scope_ambiguous')
        # Order uncertainty outside the selected subtree is not its property.
        self.unknown_order = 0
        kind, identity = matches.pop()
        return owner_roots[identity] if kind == 'owner' else [identity]

    def native_layout_anchor(self, host, expected):
        pending, visited, matches = [host], set(), []
        while pending:
            view = pending.pop()
            if view in visited or len(visited) >= 128:
                raise Refusal('anchor_view_limit')
            visited.add(view)
            cls = self.pointer(self.u64(view) & self.isa_mask)
            if self.u64(cls+32) & 2:
                descriptor = self.pointer(self.u64(cls+64))
                if descriptor:
                    symbol = self.c.t.ResolveLoadAddress(descriptor).GetSymbol()
                    if (symbol.GetStartAddress().GetLoadAddress(self.c.t) == descriptor
                            and symbol.GetName() == 'nominal type descriptor for '+expected):
                        matches.append(view)
            children = self.c.objc(view, 'subviews')
            count = self.c.objc(children, 'count')
            if len(visited)+len(pending)+count > 128:
                raise Refusal('anchor_view_limit')
            pending.extend(self.c.objc(children, 'objectAtIndex:', i) for i in range(count))
        if len(matches) != 1:
            raise Refusal('native_anchor_unresolved')
        view = matches[0]
        platform = self.c.objc(view, 'superview')
        cls = self.pointer(self.u64(platform) & self.isa_mask)
        descriptor = self.ptr(cls+64, 4)
        addr = self.c.t.ResolveLoadAddress(descriptor)
        symbol = addr.GetSymbol()
        if (addr.GetModule().GetFileSpec().GetFilename() != 'SwiftUI'
                or symbol.GetStartAddress().GetLoadAddress(self.c.t) != descriptor
                or symbol.GetName() != 'nominal type descriptor for SwiftUI.AppKitPlatformViewHost'):
            raise Refusal('anchor_parent_unqualified')
        extent = self.c.call('class_getInstanceSize', cls)[0]

        def offset(key, size):
            data = key.encode('ascii')+bytes([0])
            error = self.c.lldb.SBError()
            if self.c.p.WriteMemory(self.c.memory, data, error) != len(data) or error.Fail():
                raise Refusal('ivar_lookup_input')
            ivar = self.c.call('class_getInstanceVariable', cls, self.c.memory)[0]
            if not ivar:
                raise Refusal('ivar_unavailable')
            value = self.c.call('ivar_getOffset', ivar)[0]
            if value < 8 or value+size > extent:
                raise Refusal('ivar_extent')
            return value

        if self.ptr(platform+offset('coreRepresentedViewProvider', 8)) != view:
            raise Refusal('anchor_provider_mismatch')
        slot = platform+offset('coreLayoutInvalidator', 9)
        if self.u8(slot+8) != 0:
            raise Refusal('anchor_invalidator_absent')
        attribute = self.u32(slot)
        subgraph = self.page_owner(attribute)
        if (self.ptr(subgraph+0x28) != self.graph or
                self.u32(slot+4) != self.u32(subgraph+0x18) & 0x7fffffff):
            raise Refusal('expired_layout_reference')
        return attribute, subgraph

    def anchored_scope(self, host):
        self.anchor_id, sg = self.native_layout_anchor(host, self.req['scope']['native_view_type'])
        seen = set()
        self.scan_work = self.anchor_matches = 0
        for _ in range(12):
            root = self.u32(sg+0x60)
            if not root or root in seen or self.page_owner(root) != sg:
                raise Refusal('anchor_subgraph_root')
            seen.add(root)
            try:
                return self.select_scope(root)
            except Refusal as error:
                if error.code != 'cache_scope_unresolved':
                    raise
            model = self.u32(self.owned_record(root, 12)+8)
            resolved, _ = self.resolve(model, links_only=True)
            if not resolved:
                break
            parents = {(owner, parent) for owner, pairs in self.links.items()
                       for parent, attr in pairs if attr == resolved[0]}
            if len(parents) != 1:
                break
            sg, parent = parents.pop()
            self.owned_record(parent, 32)
            if self.page_owner(parent) != sg:
                raise Refusal('cache_subgraph_root')
        raise Refusal('anchor_card_owner_unresolved')

    def collect(self, host):
        graph_view = self.host_graph(host)
        if graph_view is None:
            raise Refusal('cache_host_unavailable')
        cf_subgraph = self.ptr(graph_view+0x28)
        subgraph = self.ptr(cf_subgraph+0x10)
        if self.u8(subgraph+0x68):
            raise Refusal('cache_invalid_subgraph')
        root, self.graph = self.u32(subgraph+0x60), self.ptr(subgraph+0x28)
        self.table = self.symbol('AG::data::_shared_table_bytes', 'AttributeGraph')
        self.base, self.limit = self.ptr(self.table), self.u32(self.table+0x18)
        if self.page_owner(root) != subgraph:
            raise Refusal('cache_subgraph_root')
        self.nil_attribute = self.u32(self.symbol('AGAttributeNil', 'AttributeGraph'))
        self.associations()
        anchored = 'native_view_type' in self.req.get('scope', {})
        selected_roots = self.anchored_scope(host) if anchored else self.select_scope(root)
        for selected_root in selected_roots:
            self.walk(selected_root)
        if anchored and self.anchor_matches != 1:
            raise Refusal('anchor_scope_binding_unresolved')
        # Establish all known selected model-value ranges before geometry reads.
        for node in self.nodes:
            self.protect_model(node['model'])
        if any(a < end and b > start for a, b in self.reads for start, end in self.private_ranges):
            raise Refusal('cache_private_range')
        normalized = []
        for node in self.nodes:
            props = []
            name = self.nominal(node['metadata'])
            if name:
                props.append(dict(id=0, attribute=dict(type=name)))
            reason = None
            for key, metadata, attribute in node['fields']:
                pair, unknown = self.geometry(attribute, metadata, key)
                if pair is None:
                    reason = unknown
                else:
                    props.append(dict(id=3 if key == 'position' else 4,
                        attribute=dict(type='__C.CGPoint' if key == 'position' else '__C.CGSize', value=pair)))
            item = dict(properties=props, children=[], child_order_unknown=node.get('child_order_unknown', False))
            if reason:
                item['layout_unknown_reason'] = reason
            normalized.append(item)
        roots = []
        for index, node in enumerate(self.nodes):
            normalized[index]['children'] = [normalized[child] for child in node['children']]
            if node['parent'] is None:
                roots.append(normalized[index])
        model_reads = sum(a < end and b > start for a, b in self.reads
                          for start, end in self.private_ranges)
        if model_reads:
            raise Refusal('cache_private_range')
        self.c.cache_audit = dict(selected_roots=len(selected_roots), scope_scan_records=getattr(self, 'scan_work', self.scanned_records), raw_records=len(self.raw_seen), views=len(self.nodes),
            model_ranges=len(self.private_ranges), model_range_reads=model_reads,
            geometry_reads=len(self.geometry_ranges), unknown_order_groups=self.unknown_order,
            evaluated_swiftui_or_graph=False)
        return encode(roots)


def reader(calls, req):
    if not hasattr(calls, 'cache_reader'):
        calls.cache_reader = CacheReader(calls, req)
    return calls.cache_reader


def is_host(calls, view, req):
    return reader(calls, req).host_graph(view) is not None


def collect_fields(calls, host, req):
    return reader(calls, req).collect(host)
