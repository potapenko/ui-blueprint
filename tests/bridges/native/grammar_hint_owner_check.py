"""Replay the synthetic SwiftUIGrammarHintFixture output. Pass the directory containing marked/ and panel-only/."""
import hashlib
import json
from pathlib import Path
import sys


class Unresolved(Exception):
    pass


def select(snapshot_bytes, owner):
    if owner["snapshotSHA256"] != hashlib.sha256(snapshot_bytes).hexdigest():
        raise Unresolved("different_observation")
    if owner["ownerType"] != "GrammarSearchHint" or len(owner["occurrences"]) != 1:
        raise Unresolved("owner_not_unique")
    occurrence = owner["occurrences"][0]
    if not occurrence.get("mount"):
        raise Unresolved("missing_owner_mount")

    data = json.loads(snapshot_bytes)["artifact"]["data"]
    nodes = {n["key"]["key"]: n for n in data["nodes"]}

    def declaration(node, name):
        return [d["state"]["value"]["value"] for d in node["source_declarations"]
                if d["name"] == name and d["namespace"] == "swiftui.layout"
                and d["source"] == "explicit-swiftui-probe" and d["state"]["availability"] == "known"]

    selected = [node for node in nodes.values()
                if declaration(node, "marker_id") == [occurrence["marker"]]
                and declaration(node, "mount_id") == [occurrence["mount"]]]
    if len(selected) != 1:
        raise Unresolved("mount_not_resolved")
    incoming = {}
    for node in nodes.values():
        for link in node["children"]:
            incoming.setdefault(link["key"], []).append(node)

    def check_parent(child, parent):
        if (incoming.get(child["key"]["key"]) != [parent]
                or declaration(child, "parent_marker_id") != declaration(parent, "marker_id")):
            raise Unresolved("inconsistent_parent")

    ancestor = selected[0]
    for parent_role in ("grammar-panel", "root"):
        parents = incoming.get(ancestor["key"]["key"], [])
        if len(parents) != 1 or declaration(parents[0], "marker_id") != [parent_role]:
            raise Unresolved("owner_not_bound_to_form")
        check_parent(ancestor, parents[0])
        ancestor = parents[0]
    if ancestor["key"]["key"] in incoming:
        raise Unresolved("owner_not_bound_to_form")
    branch = {}
    pending = [selected[0]]
    while pending:
        node = pending.pop()
        marker, = declaration(node, "marker_id")
        if marker in branch:
            raise Unresolved("repeated_branch_member")
        bounds, = [p["state"]["value"]["value"] for p in node["properties"] if p["field"] == "layout_bounds"]
        assert bounds["coordinate_space"] == dict(id="swiftui-root-pt", kind="local", units="pt", origin="top_left")
        branch[marker] = (node["key"]["key"], [bounds["shape"]["value"][c] for c in ("x", "y", "width", "height")])
        for child_key in node["children"]:
            child = nodes[child_key["key"]]
            check_parent(child, node)
            pending.append(child)
    return branch


def must_refuse(snapshot, owner, reason):
    try:
        select(snapshot, owner)
    except Unresolved as error:
        assert str(error) == reason
    else:
        raise AssertionError("Accepted " + reason)


def main(root):
    def inputs(mode, phase):
        base = root / mode
        return ((base / (phase + "-snapshot.json")).read_bytes(),
                json.loads((base / (phase + "-owner.json")).read_bytes()))

    for phase, selected in [("narrow", "hint"), ("wide", "neighbor")]:
        snapshot, owner = inputs("marked", phase)
        result = select(snapshot, owner)
        assert set(result) == {selected, selected + "-stack", selected + "-title", selected + "-example"}
        oracle = json.loads((root / "marked" / (phase + "-oracle.json")).read_bytes())
        assert all(bounds == oracle[name] for name, (_, bounds) in result.items())
        print(json.dumps(dict(phase=phase, branch=result), sort_keys=True))
        must_refuse(*inputs("panel-only", phase), "missing_owner_mount")
    must_refuse(*inputs("marked", "ambiguous"), "owner_not_unique")
    must_refuse(inputs("marked", "wide")[0], inputs("marked", "narrow")[1], "different_observation")
    # Deliberately add an impossible second parent in a test copy. Even a matching
    # observation digest cannot turn that edge into a unique owner relationship.
    raw, owner = inputs("marked", "narrow")
    duplicate = json.loads(raw)
    by_id = {}
    for node in duplicate["artifact"]["data"]["nodes"]:
        marker, = [d["state"]["value"]["value"] for d in node["source_declarations"] if d["name"] == "marker_id"]
        by_id[marker] = node
    by_id["neighbor-stack"]["children"].append(by_id["hint-title"]["key"])
    altered = json.dumps(duplicate).encode()
    must_refuse(altered, dict(owner, snapshotSHA256=hashlib.sha256(altered).hexdigest()), "inconsistent_parent")
    print("Verified absent mount, ambiguous owners, another observation and a second parent.")


if __name__ == "__main__":
    main(Path(sys.argv[1]))
