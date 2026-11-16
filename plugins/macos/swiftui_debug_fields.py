"""Fixture-only, target-qualified SwiftUI field reader; no value reflection."""
import math
import struct
from swiftui_debug_snapshot import Refusal, encode

GETTER = '$s7SwiftUI13NSHostingViewC14_viewDebugDataSayAA01_dF0O0G0VGyF'
IMAGES = {
    'libswiftCore.dylib': 'C9B36DA7-BB34-35AF-BC5D-91ECBCAA9ED0',
    'SwiftUI': 'D3349306-583A-30D3-AFC7-A64E41430375',
    'SwiftUICore': '641D6FF5-405D-3C0B-9F98-2D8524FC484B',
}
TYPES = {
    'data': '7SwiftUI10_ViewDebugO4DataV',
    'key': '7SwiftUI10_ViewDebugO8PropertyO', 'any': 'yp',
    'point': 'So7CGPointV', 'size': 'So6CGSizeV',
    'array': 's23_ContiguousArrayStorageCy7SwiftUI10_ViewDebugO4DataVG',
    'dictionary': 's18_DictionaryStorageCy7SwiftUI10_ViewDebugO8PropertyOypG',
}
SWIFT_SYMBOLS = {'swift_release', 'swift_getTypeByMangledNameInContext',
    'swift_reflectionMirror_recursiveCount', 'swift_reflectionMirror_recursiveChildOffset',
    '_swiftEmptyArrayStorage', '_swiftEmptyDictionarySingleton'}


class Fields:
    def __init__(self, calls, req):
        self.c, self.req = calls, req
        self.types = {}
        self.nodes = 0
        self.active_arrays = set()

    def pointer(self, value):
        return self.c.p.FixAddress(value, self.c.lldb.eAddressMaskTypeData)

    def native_pointer(self, value):
        if not value or value != self.pointer(value) or value & 7:
            raise Refusal('non_native_storage')
        return value

    def u64(self, address):
        return struct.unpack('<Q', self.c.read(address, 8))[0]

    def witness(self, metadata, size, alignment, inline=False):
        witness = self.pointer(self.u64(metadata-8))
        actual_size, stride, flags, _ = struct.unpack('<QQII', self.c.read(witness+64, 24))
        if (actual_size != size or stride != size or (flags & 0xffff)+1 != alignment
                or (inline and flags & 0x20000)):
            raise Refusal('target_value_layout_mismatch')

    def qualify(self):
        modules = {}
        for index in range(self.c.t.GetNumModules()):
            module = self.c.t.GetModuleAtIndex(index)
            name = module.GetFileSpec().GetFilename()
            if name in IMAGES:
                if name in modules:
                    raise Refusal('unsupported_image_profile')
                modules[name] = module.GetUUIDString()
        if modules != IMAGES:
            raise Refusal('unsupported_image_profile')
        for label, name in TYPES.items():
            encoded = name.encode('ascii')
            error = self.c.lldb.SBError()
            if (self.c.p.WriteMemory(self.c.memory, encoded, error) != len(encoded)
                    or error.Fail()):
                raise Refusal('type_lookup_input_failed')
            metadata = self.pointer(self.c.call('swift_getTypeByMangledNameInContext',
                self.c.memory, len(encoded), 0, 0)[0])
            if not metadata:
                raise Refusal('type_lookup_unavailable')
            self.types[label] = metadata
        for label, size, alignment in [('data', 16, 8), ('key', 1, 1), ('any', 32, 8),
                                       ('point', 16, 8), ('size', 16, 8)]:
            self.witness(self.types[label], size, alignment, label in ('point', 'size'))
        for label, offsets in [('data', [0, 8]),
                              ('dictionary', [16, 24, 32, 33, 34, 36, 40, 48, 56])]:
            if self.c.call('swift_reflectionMirror_recursiveCount', self.types[label])[0] != len(offsets):
                raise Refusal('target_field_count_mismatch')
            for index, offset in enumerate(offsets):
                if self.c.call('swift_reflectionMirror_recursiveChildOffset', self.types[label], index)[0] != offset:
                    raise Refusal('target_field_offset_mismatch')
        self.isa_mask = self.u64(self.c.symbol('objc_debug_isa_class_mask'))
        self.empty_array = self.c.symbol('_swiftEmptyArrayStorage')
        self.empty_dictionary = self.c.symbol('_swiftEmptyDictionarySingleton')

    def storage(self, address, label):
        # Check the original representation before stripping pointer bits.
        self.native_pointer(address)
        if self.pointer(self.u64(address) & self.isa_mask) != self.types[label]:
            raise Refusal('storage_metadata_mismatch')

    def nominal(self, metadata):
        metadata = self.pointer(metadata)
        kind, descriptor = struct.unpack('<QQ', self.c.read(metadata, 16))
        if kind not in (0x200, 0x201, 0x202):
            return None
        descriptor = self.pointer(descriptor)
        flags, _, name = struct.unpack('<Iii', self.c.read(descriptor, 12))
        if flags & 31 not in (16, 17, 18):
            return None
        address = descriptor+8+name
        section = self.c.t.ResolveLoadAddress(address).GetSection()
        if not section.IsValid():
            raise Refusal('type_name_not_in_image')
        limit = min(256, section.GetLoadAddress(self.c.t)+section.GetByteSize()-address)
        if limit <= 0:
            raise Refusal('type_name_limit')
        value = self.c.read(address, limit)
        end = value.find(b'\0')
        if end <= 0:
            raise Refusal('type_name_limit')
        try:
            return value[:end].decode('utf-8')
        except UnicodeError:
            raise Refusal('type_name_invalid') from None

    def properties(self, dictionary):
        self.native_pointer(dictionary)
        if dictionary == self.empty_dictionary:
            return []
        self.storage(dictionary, 'dictionary')
        # Header contains no Any values. Metadata has already established ownership.
        header = self.c.read(dictionary+16, 48)
        count, capacity = struct.unpack_from('<QQ', header)
        scale = struct.unpack_from('<b', header, 16)[0]
        if scale < 0 or scale > 5 or count > 9 or count > capacity:
            raise Refusal('dictionary_limit')
        buckets = 1 << scale
        if capacity > buckets or capacity < count:
            raise Refusal('dictionary_shape')
        keys, values = struct.unpack_from('<QQ', header, 32)
        self.native_pointer(values)
        if not keys or keys != self.pointer(keys):
            raise Refusal('non_native_storage')
        # This qualified profile tail-allocates one bitmap word, one-byte keys,
        # then 8-aligned Any values. Require exact ranges, not arbitrary pointers
        # into some other allocation. scale<=5 guarantees one bitmap word.
        if (keys != dictionary+72 or values != ((keys+buckets+7) & ~7)
                or values+buckets*32 >= 1 << 64):
            raise Refusal('dictionary_ranges')
        mask = self.u64(dictionary+64)
        occupied = [i for i in range(buckets) if mask & (1 << i)]
        if len(occupied) != count:
            raise Refusal('dictionary_shape')
        slots = {}
        for bucket in occupied:
            tag = self.c.read(keys+bucket, 1)[0]
            if tag > 8 or tag in slots:
                raise Refusal('property_shape')
            slots[tag] = values+bucket*32
        result = []
        for tag in (0, 3, 4):
            if tag not in slots:
                continue
            slot = slots[tag]
            metadata = self.pointer(self.u64(slot+24))
            if tag == 0:
                # A metatype Any contains only a type pointer. Validate its actual
                # representation before accessing the inline payload.
                kind, instance = struct.unpack('<QQ', self.c.read(metadata, 16))
                if kind != 0x304:
                    raise Refusal('type_representation_mismatch')
                self.witness(metadata, 8, 8, True)
                value = self.pointer(self.u64(slot))
                if value != self.pointer(instance):
                    raise Refusal('type_metadata_mismatch')
                name = self.nominal(value)
                if name:
                    result.append(dict(id=0, attribute=dict(type=name)))
            else:
                if metadata != self.types['point' if tag == 3 else 'size']:
                    raise Refusal('geometry_metadata_mismatch')
                pair = struct.unpack('<dd', self.c.read(slot, 16))
                if not all(math.isfinite(x) for x in pair) or (tag == 4 and min(pair) < 0):
                    raise Refusal('geometry_value')
                result.append(dict(id=tag, attribute=dict(
                    type='__C.CGPoint' if tag == 3 else '__C.CGSize', value=pair)))
        return result

    def walk(self, address, depth=1):
        self.native_pointer(address)
        # Empty arrays are accepted by exact runtime singleton identity, never by
        # first reading count from an unqualified object.
        if address == self.empty_array:
            return []
        self.storage(address, 'array')
        if address in self.active_arrays or depth > self.req['limits']['max_depth']:
            raise Refusal('tree_limit', 2)
        count, capacity = struct.unpack('<QQ', self.c.read(address+16, 16))
        if count > capacity >> 1 or self.nodes+count > self.req['limits']['max_nodes']:
            raise Refusal('tree_limit', 2)
        self.active_arrays.add(address)
        result = []
        for index in range(count):
            if self.nodes >= self.req['limits']['max_nodes']:
                raise Refusal('tree_limit', 2)
            dictionary, children = struct.unpack('<QQ', self.c.read(address+32+index*16, 16))
            self.nodes += 1
            props = self.properties(dictionary)
            result.append(dict(properties=props, children=self.walk(children, depth+1)))
        self.active_arrays.remove(address)
        return result


def collect_fields(calls, host, req):
    fields = Fields(calls, req)
    fields.qualify()
    calls.altered = True
    calls.setreg('x20', host)
    # Calls retains the returned +1 object BEFORE register restoration, so every
    # completed-call decoder/restore failure still owns exactly one cleanup.
    array = calls.call(GETTER)[0]
    return encode(fields.walk(array))
