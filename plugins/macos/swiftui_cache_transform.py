"""Exact-profile translation inputs from retained SwiftUI transform records.

No graph evaluation, model payload, named-space value or transform callback.
The caller owns graph/cache admission and the per-observation read/privacy budget.
"""
import math
import struct

from swiftui_debug_snapshot import Refusal

_PRIVATE = '33_1CC2FE016A82CF91549A64E942CE8ED4LL'
DESCRIPTORS = {
    'AnyElement': '$s7SwiftUI10AnyElement'+_PRIVATE+'CMn',
    'Element': '$s7SwiftUI7Element'+_PRIVATE+'CMn',
    'BufferedElement': '$s7SwiftUI15BufferedElement'+_PRIVATE+'CMn',
    '_VTable': '$s7SwiftUI13ViewTransformV12UnsafeBufferV7_VTable'+_PRIVATE+'CMn',
    'SizedSpaceElement': '$s7SwiftUI17SizedSpaceElement'+_PRIVATE+'VMn',
    'CoordinateSpaceElement': '$s7SwiftUI22CoordinateSpaceElement'+_PRIVATE+'VMn',
    'TranslationElement': '$s7SwiftUI18TranslationElement'+_PRIVATE+'VMn',
    'ScrollGeometryItem': '$s7SwiftUI13ViewTransformV18ScrollGeometryItemVMn',
}
NON_GEOMETRIC = {'SizedSpaceElement', 'CoordinateSpaceElement', 'ScrollGeometryItem'}


def _descriptor(r, metadata, is_class=False):
    descriptor = r.ptr(metadata+(64 if is_class else 8), 4)
    address = r.c.t.ResolveLoadAddress(descriptor)
    symbol = address.GetSymbol()
    if (address.GetModule().GetFileSpec().GetFilename() != 'SwiftUICore'
            or symbol.GetStartAddress().GetLoadAddress(r.c.t) != descriptor):
        raise Refusal('cache_transform_descriptor')
    mangled = symbol.GetMangledName()
    name = next((name for name, expected in DESCRIPTORS.items() if mangled == expected), None)
    return descriptor, name


def _fields(r, metadata, descriptor, expected):
    if r.u32(descriptor+36) != len(expected):
        raise Refusal('cache_transform_layout')
    vector = r.u32(descriptor+40)
    if not 2 <= vector <= 64:
        raise Refusal('cache_transform_layout')
    actual = tuple(r.u64(metadata+vector*8+i*8) for i in range(len(expected)))
    if actual != expected:
        raise Refusal('cache_transform_layout')


def _argument(r, metadata, descriptor, expected_offset):
    positive, immediate = r.u32(descriptor+28), r.u32(descriptor+32)
    if positive-immediate != expected_offset:
        raise Refusal('cache_transform_generic_layout')
    argument = r.ptr(metadata+expected_offset*8)
    if r.u64(argument) != 0x200:
        raise Refusal('cache_transform_generic_layout')
    _, name = _descriptor(r, argument)
    return argument, name


def _pair(r, address):
    values = struct.unpack('<dd', r.read(address, 16))
    if not all(math.isfinite(value) for value in values):
        raise Refusal('cache_transform_value')
    r.geometry_ranges.append((address, address+16))
    return values


def _translation(r, metadata, payload):
    desc = r.ptr(metadata+8, 4)
    vector = r.u32(desc+24)
    if (r.value_size(metadata) != (16, 16, 8) or r.u32(desc+20) != 1
            or not 2 <= vector <= 8 or r.u32(metadata+vector*8) != 0):
        raise Refusal('cache_transform_translation_layout')
    return _pair(r, payload)


def _read(r, attribute, metadata):
    value, reason = r.cached_geometry(attribute, metadata, 'transform')
    if value is None:
        return None, reason
    head = r.pointer(r.u64(value))
    chain, seen, expected_depth = [], set(), None
    item_count = total_bytes = 0
    while head:
        if head & 7 or head in seen or len(seen) >= 16:
            raise Refusal('cache_transform_chain_limit')
        seen.add(head)
        meta = r.ptr(head)
        desc, name = _descriptor(r, meta, True)
        if name not in ('Element', 'BufferedElement'):
            return None, 'cache_transform_chain_unsupported'
        superclass = r.ptr(meta+8)
        superdesc, supername = _descriptor(r, superclass, True)
        if supername != 'AnyElement':
            raise Refusal('cache_transform_superclass')
        _fields(r, superclass, superdesc, (16, 24))
        _fields(r, meta, desc, (32, 48))
        depth = r.u64(head+24)
        if not 1 <= depth <= 16 or (expected_depth is not None and depth != expected_depth):
            raise Refusal('cache_transform_depth')
        expected_depth = depth-1
        if name == 'Element':
            element_meta, kind = _argument(r, meta, desc, 16)
            if kind == 'TranslationElement':
                chain.append(_translation(r, element_meta, head+48))
            elif kind not in NON_GEOMETRIC:
                return None, 'cache_transform_chain_unsupported'
        else:
            count, available = r.u32(head+60), r.u32(head+56)
            item_count += count
            if item_count > 64 or available > 8192:
                raise Refusal('cache_transform_buffer_limit')
            buffer = r.ptr(head+48) if count else 0
            used = 0
            for _ in range(count):
                vtable = r.ptr(buffer+used)
                vt_desc, vt_name = _descriptor(r, vtable, True)
                if vt_name != '_VTable':
                    return None, 'cache_transform_chain_unsupported'
                element_meta, kind = _argument(r, vtable, vt_desc, 17)
                if kind not in NON_GEOMETRIC | {'TranslationElement'}:
                    return None, 'cache_transform_chain_unsupported'
                size = r.u32(buffer+used+8)
                extent, _, alignment = r.value_size(element_meta)
                if alignment > 16 or size != (16+extent+15) & ~15:
                    raise Refusal('cache_transform_item_layout')
                used += size
                total_bytes += size
                if total_bytes > 8192:
                    raise Refusal('cache_transform_buffer_limit')
                if kind == 'TranslationElement':
                    chain.append(_translation(r, element_meta, buffer+used-size+16))
                # All other payloads, including coordinate-space names, stay unread.
        chain.append(_pair(r, head+32))
        head = r.pointer(r.u64(head+16))
        if bool(head) != bool(expected_depth):
            raise Refusal('cache_transform_depth')
    return dict(position_adjustment=_pair(r, value+16),
                pending_translation=_pair(r, value+32), chain=tuple(chain)), None


def translation_inputs(reader, attribute, metadata):
    # A request owns this reader. Repeated tree records reference the same cached
    # attribute; decoding it once avoids redundant reads within that observation.
    if not hasattr(reader, 'transform_inputs_cache'):
        reader.transform_inputs_cache = {}
    key = attribute, metadata
    if key not in reader.transform_inputs_cache:
        reader.transform_inputs_cache[key] = _read(reader, attribute, metadata)
    return reader.transform_inputs_cache[key]
