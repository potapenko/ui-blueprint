"""Fixed stored Group? qualification; no expression evaluation or whole-value dump.

Call only at a separately qualified source site with the pinned Swift type layout.
The five-state fixture establishes that nil cannot be decoded from byte zero:
the actual Optional discriminator must first report nil/some without formatters.
"""
import re

GROUPS = ("toolbar", "tabs", "modifiers", "query")


def parse_optional(text, expected_type):
    pattern = r"\(Swift\.Optional<" + re.escape(expected_type) + r">\)\s+description\.group\s*=\s*(nil|some)\s*\{\s*\}"
    match = re.fullmatch(pattern, text.strip())
    if not match:
        raise ValueError("group_optional_unqualified")
    return match[1]


def decode_group(optional_case, raw):
    if optional_case == "nil" and raw is None:
        return {"availability": "known", "group": None}
    if optional_case == "some" and type(raw) is bytes and len(raw) == 1 and raw[0] < len(GROUPS):
        return {"availability": "known", "group": GROUPS[raw[0]]}
    raise ValueError("group_identity_unqualified")


def read_group(frame, process, command, lldb, fixture=False):
    expected = "S14GroupControl.Annotation.Group" if fixture else "PlayphrasemeApp.PlayphrasemeGuestTourAnnotation.Group"
    # Static optional case does not run descriptions, synthetic providers or getters.
    optional_case = parse_optional(command("frame variable -d no-dynamic-values -R -S false -- description.group"), expected)
    if optional_case == "nil":
        return decode_group("nil", None)
    description = frame.FindVariable("description", lldb.eNoDynamicValues).GetNonSyntheticValue()
    group = description.GetChildMemberWithName("group", lldb.eNoDynamicValues).GetNonSyntheticValue()
    if not group.IsValid() or not group.GetError().Success() or group.GetByteSize() != 56:
        raise ValueError("group_storage_unqualified")
    address = group.GetLoadAddress()
    if address == lldb.LLDB_INVALID_ADDRESS:
        raise ValueError("group_address_unqualified")
    error = lldb.SBError()
    raw = process.ReadMemory(address, 1, error)
    if not error.Success():
        raise ValueError("group_read_failed")
    return decode_group(optional_case, raw)
