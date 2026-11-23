"""Bounded metadata/structural selection; never read a SwiftUI model value."""
import re
import struct

from swiftui_debug_snapshot import Refusal


def validate_options(scope):
    """Return the legacy scope for its existing validator; retain caller data."""
    if not isinstance(scope, dict):
        raise Refusal('invalid_scope', 2)
    extra = {'owner_argument_type', 'part'} & scope.keys()
    if extra and ('native_view_type' in scope or 'leaf_types' in scope):
        raise Refusal('invalid_scope', 2)
    nominal = r'[A-Za-z_][A-Za-z0-9_]*(?:\.[A-Za-z_][A-Za-z0-9_]*)+'

    def name(value):
        return isinstance(value, str) and len(value) <= 256 and re.fullmatch(nominal, value)

    if 'owner_argument_type' in scope and not name(scope['owner_argument_type']):
        raise Refusal('invalid_scope', 2)
    if 'part' in scope:
        part = scope['part']
        if (not isinstance(part, dict) or set(part) != {'type', 'containing_type'}
                or not all(name(value) for value in part.values())):
            raise Refusal('invalid_scope', 2)
    return {key: value for key, value in scope.items() if key not in extra}


def first_argument_type(reader, metadata, expected_owner):
    """Narrow ABI profile: flat generic struct, one ordinary key type parameter.

    Swift ABI Metadata.h TargetStructMetadata/TargetStructDescriptor and
    GenericContext.h define the two-word metadata prefix and trailing generic
    header. Descriptor/header checks precede the one metadata argument read.
    This is metadata only; witness tables and instantiated view values stay unread.
    """
    r = reader
    if r.u64(metadata) != 0x200 or r.qualified_nominal(metadata) != expected_owner:
        raise Refusal('source_specialization_unqualified')
    descriptor = r.ptr(metadata+8, 4)
    flags = r.u32(descriptor)
    if flags & 0xffff != 0xd1:  # struct(17), unique(0x40), generic(0x80), version0
        raise Refusal('source_specialization_unqualified')
    # Only a module-level struct; nested generic contexts have different arity.
    relative = struct.unpack('<i', r.read(descriptor+4, 4))[0]
    if not relative:
        raise Refusal('source_specialization_unqualified')
    parent = descriptor+4+(relative & ~1)
    parent = r.ptr(parent, 4) if relative & 1 else parent
    if r.u32(parent) & 31 != 0:
        raise Refusal('source_specialization_unqualified')
    section = r.c.t.ResolveLoadAddress(descriptor).GetSection()
    if (not section.IsValid() or descriptor+45 >
            section.GetLoadAddress(r.c.t)+section.GetByteSize()):
        raise Refusal('source_specialization_unqualified')
    parameters, requirements, arguments, generic_flags = struct.unpack('<HHHH', r.read(descriptor+36, 8))
    if (parameters != 1 or requirements > 16 or not 1 <= arguments <= 1+requirements
            or generic_flags != 0 or r.u8(descriptor+44) != 0x80):
        raise Refusal('source_specialization_unqualified')
    argument = r.ptr(metadata+16)
    result = r.qualified_nominal(argument)
    if result is None:
        raise Refusal('source_specialization_unqualified')
    return result


def matches_owner(reader, metadata, scope):
    expected = scope.get('owner_argument_type')
    if expected is None:
        return True
    if metadata is None:
        raise Refusal('source_specialization_unqualified')
    return first_argument_type(reader, metadata, scope['type']) == expected


def body_metadata(reader, attribute):
    """Use the same validated body/accessor metadata path as CacheReader.rule_owner."""
    r = reader
    _, descriptor, _ = r.attribute_info(attribute)
    rule = r.pointer(r.u64(descriptor))
    if not rule or r.u64(rule) != 0x200:
        raise Refusal('source_specialization_unqualified')
    descriptor = r.ptr(rule+8, 4)
    symbol = r.c.t.ResolveLoadAddress(descriptor).GetSymbol()
    if (symbol.GetStartAddress().GetLoadAddress(r.c.t) != descriptor
            or symbol.GetMangledName() not in (
                '$s7SwiftUI10StaticBody33_A4C1D658B3717A3062FEFC91A812D6EBLLVMn',
                '$s7SwiftUI11DynamicBody33_A4C1D658B3717A3062FEFC91A812D6EBLLVMn')):
        raise Refusal('source_specialization_unqualified')
    accessor = r.ptr(rule+16)
    if r.qualified_nominal(accessor) != 'SwiftUI.ViewBodyAccessor':
        raise Refusal('source_specialization_unqualified')
    return r.ptr(accessor+16)


def select_part(records, roots, part):
    """Use the already validated discovery tree, not a second target traversal.

    records maps raw element IDs to (nominal type, direct raw children).
    Only descendants of the uniquely selected owner are eligible. Node addresses
    never leave the reader; order does not affect matching.
    """
    active, visited, matches = set(), set(), set()

    def visit(element, depth):
        if element in active or element in visited:
            raise Refusal('source_part_cycle')
        if depth > 256 or len(visited) >= 1024 or element not in records:
            raise Refusal('source_part_incomplete')
        active.add(element)
        visited.add(element)
        name, children = records[element]
        found = False
        for child in children:
            found = visit(child, depth+1) or found
        if name == part['type'] and found:
            matches.add(element)
        active.remove(element)
        return found or name == part['containing_type']

    for root in roots:
        visit(root, 1)
    if len(matches) != 1:
        raise Refusal('source_part_unresolved' if not matches else 'source_part_ambiguous')
    return [matches.pop()]
