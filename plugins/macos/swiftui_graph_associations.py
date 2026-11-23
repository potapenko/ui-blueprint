"""Exact-profile addressed AG associations; no AG callbacks or whole-graph scan."""
import struct

from swiftui_debug_snapshot import Refusal

_MASK = (1 << 64)-1
_HASH_MULTIPLIER = 0x9DDFEA08EB382D69


def pointer_hash(pointer):
    """AttributeGraph find<Subgraph*> arithmetic, including its 32-bit first shift."""
    value = ((((pointer & 0xFFFFFFFF) << 3) & 0xFFFFFFFF)+8) ^ (pointer >> 32)
    value = value*_HASH_MULTIPLIER & _MASK
    value ^= (pointer >> 32) ^ (value >> 47)
    value = value*_HASH_MULTIPLIER & _MASK
    value ^= value >> 47
    return value*_HASH_MULTIPLIER & _MASK


def bucket_index(value, count):
    if not count:
        raise Refusal('cache_association_hash_shape')
    return value & (count-1) if count & (count-1) == 0 else value % count


class GraphAssociations:
    def __init__(self, reader):
        self.r = reader
        self.touched_elements = set()
        self.metadata_records = set()
        self.extents, self.link_cache = {}, {}
        self.parent_cache, self.child_cache = {}, {}
        self.attachment_cache, self.children_cache = {}, {}

    @property
    def metadata_work_count(self):
        return len(self.metadata_records)

    @property
    def work_count(self):
        return len(self.metadata_records)+len(self.touched_elements)

    def _touch(self, kind, address):
        collection = self.touched_elements if kind == 'element' else self.metadata_records
        collection.add(address if kind == 'element' else (kind, address))
        # Reader glue must also include these sets in subsequent traversal checks.
        elements = (self.touched_elements | getattr(self.r, 'raw_seen', set())
                    | getattr(self.r, 'transform_context_seen', set())
                    | getattr(self.r, 'native_parts_indexed', set()))
        if len(elements)+self.metadata_work_count > 1024:
            raise Refusal('cache_association_work_limit', 2)

    def _pointer(self, value):
        pointer = self.r.pointer(value)
        if not pointer or pointer & 7:
            raise Refusal('cache_association_pointer')
        return pointer

    def _allocation(self, pointer, minimum):
        pointer = self._pointer(pointer)
        if minimum < 0 or pointer+minimum > 1 << 64:
            raise Refusal('cache_association_extent')
        if pointer not in self.extents:
            self._touch('allocation', pointer)
            extent = self.r.c.call('malloc_size', pointer)[0]
            if not 0 < extent <= (1 << 48) or pointer+extent > 1 << 64:
                raise Refusal('cache_association_extent')
            self.extents[pointer] = extent
        if self.extents[pointer] < minimum:
            raise Refusal('cache_association_extent')
        return pointer

    def _subgraph(self, pointer, allow_invalid=False):
        pointer = self._pointer(pointer)
        self._touch('subgraph', pointer)
        if self.r.ptr(pointer+0x28) != self.r.graph:
            raise Refusal('cache_foreign_graph')
        if self.r.u8(pointer+0x68):
            if allow_invalid:
                return False
            raise Refusal('cache_invalid_subgraph')
        return True

    def _element(self, offset, expected_owner):
        self._touch('element', offset)
        p = self.r.owned_record(offset, 32)
        if self.r.page_owner(offset) != expected_owner:
            raise Refusal('cache_association_element_owner')
        return p

    def links_for(self, subgraph):
        self._subgraph(subgraph)
        if subgraph in self.link_cache:
            return self.link_cache[subgraph]
        r = self.r
        table = r.pointer(r.u64(r.graph+0x118))
        if not table:
            self.link_cache[subgraph] = ()
            return ()
        self._allocation(table, 32)
        self._touch('hash_table', table)
        bucket_count, size = r.u64(table+8), r.u64(table+24)
        if not bucket_count or not size:
            if size:
                raise Refusal('cache_association_hash_shape')
            self.link_cache[subgraph] = ()
            return ()
        if bucket_count > _MASK//8:
            raise Refusal('cache_association_extent')
        buckets = self._allocation(r.ptr(table), bucket_count*8)
        expected_hash = pointer_hash(subgraph)
        index = bucket_index(expected_hash, bucket_count)
        slot = buckets+index*8
        self._touch('bucket', slot)
        predecessor = r.pointer(r.u64(slot))
        if not predecessor:
            self.link_cache[subgraph] = ()
            return ()
        # The bucket points to the predecessor, which may be the table's embedded
        # before-begin sentinel. Only its next link belongs to this lookup.
        if predecessor != table+16:
            self._allocation(predecessor, 24)
            self._touch('hash_node', predecessor)
        node = r.pointer(r.u64(predecessor))
        visited = set()
        result = None
        while node:
            if node in visited or len(visited) >= 1024:
                raise Refusal('cache_association_hash_cycle')
            visited.add(node)
            self._allocation(node, 24)
            self._touch('hash_node', node)
            stored_hash = r.u64(node+8)
            if bucket_index(stored_hash, bucket_count) != index:
                break
            key = r.pointer(r.u64(node+16))
            if pointer_hash(key) != stored_hash:
                raise Refusal('cache_association_hash_mismatch')
            if key == subgraph:
                if result is not None:
                    raise Refusal('cache_association_duplicate_owner')
                self._allocation(node, 48)
                data, count, capacity = r.pointer(r.u64(node+24)), r.u64(node+32), r.u64(node+40)
                if count > capacity or count > 1024 or capacity > _MASK//8:
                    raise Refusal('cache_association_vector_limit', 2)
                if capacity:
                    self._allocation(data, capacity*8)
                elif data or count:
                    raise Refusal('cache_association_vector_shape')
                pairs = []
                for ordinal in range(count):
                    address = data+ordinal*8
                    self._touch('link_pair', address)
                    element, model = struct.unpack('<II', r.read(address, 8))
                    self._element(element, subgraph)
                    pair = (element, model)
                    if pair in pairs:
                        raise Refusal('cache_association_duplicate_link')
                    pairs.append(pair)
                result = tuple(pairs)
            node = r.pointer(r.u64(node))
        self.link_cache[subgraph] = () if result is None else result
        return self.link_cache[subgraph]

    def parents_of(self, subgraph):
        self._subgraph(subgraph)
        if subgraph in self.parent_cache:
            return self.parent_cache[subgraph]
        r = self.r
        raw = r.u64(subgraph+0x38)
        if not raw:
            parents = []
        elif not raw & 1:
            parents = [self._pointer(raw)]
        else:
            obj = self._allocation(raw & ~1, 48)
            data, count = r.pointer(r.u64(obj+0x20)), r.u64(obj+0x28)
            if not 1 <= count <= 1024:
                raise Refusal('cache_association_parent_limit', 2)
            if data:
                self._allocation(data, count*8)
            else:
                if count > 4:
                    raise Refusal('cache_association_inline_shape')
                data = obj
            parents = []
            for ordinal in range(count):
                self._touch('parent_slot', data+ordinal*8)
                parents.append(self._pointer(r.u64(data+ordinal*8)))
        if len(set(parents)) != len(parents) or subgraph in parents:
            raise Refusal('cache_association_parent_cycle')
        for parent in parents:
            self._subgraph(parent)
        self.parent_cache[subgraph] = tuple(parents)
        return self.parent_cache[subgraph]

    def direct_children(self, subgraph):
        self._subgraph(subgraph)
        if subgraph in self.child_cache:
            return self.child_cache[subgraph]
        r = self.r
        data, count, capacity = r.pointer(r.u64(subgraph+0x40)), r.u32(subgraph+0x48), r.u32(subgraph+0x4c)
        if count > capacity or count > 1024:
            raise Refusal('cache_association_child_limit', 2)
        if capacity:
            self._allocation(data, capacity*8)
        elif data or count:
            raise Refusal('cache_association_vector_shape')
        children = []
        for ordinal in range(count):
            self._touch('child_slot', data+ordinal*8)
            child = self._pointer(r.u64(data+ordinal*8) & ~3)
            if child == subgraph or child in children:
                raise Refusal('cache_association_child_cycle')
            if self._subgraph(child, allow_invalid=True):
                if subgraph not in self.parents_of(child):
                    raise Refusal('cache_association_topology_mismatch')
                children.append(child)
        self.child_cache[subgraph] = tuple(children)
        return self.child_cache[subgraph]

    def parent_attachments(self, element):
        """Dependency parents do not prove recorded UI-tree parent attachment."""
        if element == getattr(self.r, 'graph_root', None):
            owner = self.r.page_owner(element)
            self._subgraph(owner)
            self._element(element, owner)
            return ()
        raise Refusal('cache_recorded_parent_unqualified')

    def _dependency_parent_candidates(self, element):
        if element in self.attachment_cache:
            return self.attachment_cache[element]
        r = self.r
        owner = r.page_owner(element)
        self._subgraph(owner)
        p = self._element(element, owner)
        if r.u32(owner+0x60) != element:
            raise Refusal('cache_association_not_subgraph_root')
        parents = self.parents_of(owner)
        if not parents:
            self.attachment_cache[element] = ()
            return ()
        resolved, _ = r.resolve(r.u32(p+8), links_only=True)
        if resolved is None:
            raise Refusal('cache_association_model_unavailable')
        matches = tuple((parent, node) for parent in parents
                        for node, model in self.links_for(parent) if model == resolved[0])
        self.attachment_cache[element] = matches
        return matches

    def children(self, element):
        """Only absence of link pairs is qualified; nonempty topology refuses."""
        owner = self.r.page_owner(element)
        self._element(element, owner)
        if not any(node == element for node, _ in self.links_for(owner)):
            return [], True
        raise Refusal('cache_recorded_children_unqualified')

    def _dependency_child_candidates(self, element):
        if element in self.children_cache:
            return self.children_cache[element]
        r = self.r
        owner = r.page_owner(element)
        self._element(element, owner)
        models = {model for node, model in self.links_for(owner) if node == element}
        selected = []
        if models:
            for child in self.direct_children(owner):
                root = r.u32(child+0x60)
                if not root:
                    continue
                p = self._element(root, child)
                resolved, _ = r.resolve(r.u32(p+8), links_only=True)
                if resolved is None:
                    raise Refusal('cache_association_model_unavailable')
                if resolved[0] in models:
                    selected.append((r.u32(child+0x58), root))
        roots = {root for _, root in selected}
        if len(roots) != len(selected):
            raise Refusal('cache_duplicate_subgraph')
        known = True
        if len({order for order, _ in selected}) != len(selected):
            following = {root: r.u32(self._element(root, r.page_owner(root))+24) for root in roots}
            heads, chain = roots-set(following.values()), []
            if len(heads) == 1:
                cursor = heads.pop()
                while cursor in roots and cursor not in chain:
                    chain.append(cursor)
                    cursor = following[cursor]
            if len(chain) == len(roots):
                self.children_cache[element] = (chain, True)
                return self.children_cache[element]
            r.unknown_order += 1
            known = False
        self.children_cache[element] = ([root for _, root in sorted(selected)], known)
        return self.children_cache[element]
