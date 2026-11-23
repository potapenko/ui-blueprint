"""Offline, fixed-G2 source-local receipt -> canonical Snapshot. No acquisition."""
import json
import copy
import math
import re
import sys
from pathlib import Path

NAMESPACE = "playphraseme.guest_tour.source_layout"
SOURCE = "PlayphrasemeGuestTourConnections.swift:29"
MODULE = "3456EB33-6C0E-393F-8486-10961C55A994"
IDENTITY = {"pid": 32187, "birth": 1795448825.752163, "window_id": 1060}
BUNDLE = "me.playphraseme.reference.guesttour.01a20845.g2"
CARDS = set("language actor commonPhrases vocabulary favorites grammar statistics previous playPause next speed repeatPlayback focus context search grammarSearch clearSearch favoriteSearch randomPhrase randomWord randomFavorite practice fullscreen reels".split())


def require(condition):
    if not condition:
        raise ValueError("unqualified_source_receipt")


def numbers(value, count):
    require(isinstance(value, list) and len(value) == count)
    require(all(type(x) in (int, float) and math.isfinite(x) for x in value))
    return value


def normalize(record):
    """Preserve the two observed frames; do not infer the rest of the map."""
    require(isinstance(record, dict))
    if record.get("format") == "playphraseme.guest-tour.component-1":
        return normalize_component(record)
    if record.get("format") == "playphraseme.guest-tour.group-1":
        return normalize_group(record)
    require(set(record) == {
        "source_site", "module_uuid", "card", "space", "target_xywh", "rect_xywh",
        "start_xy", "identity_before_after", "detached_verified", "observation_id",
        "clock_domain", "time_unit", "read_start", "read_end", "units", "before",
        "after", "runtime_state",
    })
    require(record["source_site"] == SOURCE and record["module_uuid"] == MODULE)
    require(record["detached_verified"] is True and record["identity_before_after"] == IDENTITY)
    require(record["card"] in CARDS and record["units"] == "pt")
    require(record["space"] == "guest-tour GeometryReader local points")
    require(record["time_unit"] == "ns" and record["clock_domain"] == "owner-lldb-process.monotonic")
    observation_id = record["observation_id"]
    require(isinstance(observation_id, str) and re.fullmatch(r"[a-f0-9]{32}", observation_id))
    start, end = record["read_start"], record["read_end"]
    require(type(start) is int and type(end) is int and 0 <= start <= end < 2**63)
    for name in ("before", "after"):
        identity = record[name]
        require(isinstance(identity, dict) and set(identity) == {*IDENTITY, "bundle", "window_screen_pt"})
        require(all(identity[k] == v for k, v in IDENTITY.items()) and identity["bundle"] == BUNDLE)
        window = numbers(identity["window_screen_pt"], 4)
        require(window[2] > 0 and window[3] > 0)
    target = numbers(record["target_xywh"], 4)
    rect = numbers(record["rect_xywh"], 4)
    connector_start = numbers(record["start_xy"], 2)
    require(min(target[2:] + rect[2:]) >= 0)
    require(connector_start == [target[0] + target[2] / 2, target[1] + target[3] / 2])
    scope = "guest-tour-open-map"
    surface = {"id": "cg-window-1060", "generation": observation_id}
    # A separate observation never gains stable refs or a window/image transform.
    space = {"id": "guest-tour-local-" + observation_id, "kind": "local", "origin": "top_left", "units": "pt"}
    coverage = {"scope_id": scope, "fields": ["layout_bounds"], "status": "partial", "omitted_count": None, "unknown_count": None}
    evidence = {"source_namespace": NAMESPACE, "observation_id": observation_id,
                "method": "fixed-source-static-locals", "provenance": "reported", "uncertainty": None}

    def declaration(name, text):
        return {"namespace": NAMESPACE, "name": name, "source": SOURCE,
                "sensitivity": "public", "state": {"availability": "known", "value": {"type": "text", "value": text}}}

    nodes = []
    for part, frame in (("card", rect), ("target", target)):
        geometry = {"coordinate_space": space, "frame_kind": "layout_bounds",
                    "shape": {"shape": "rect", "value": dict(zip(("x", "y", "width", "height"), frame))},
                    "transform": {"status": "local_only"}}
        nodes.append({"key": {"namespace": NAMESPACE, "key": record["card"] + "." + part},
                      "surface": surface, "children": [], "extensions": [],
                      "native_role": {"availability": "unsupported", "reason": "not_collected"},
                      "source_declarations": [declaration("card_id", record["card"]), declaration("source_local", "rect" if part == "card" else "target"),
                                              declaration("module_uuid", MODULE), declaration("read_interval_ns", f"{start}:{end}")],
                      "properties": [{"field": "layout_bounds", "selection": "requested", "sensitivity": "public", "evidence": evidence,
                                      "state": {"availability": "known", "value": {"type": "geometry", "value": geometry}}}]})
    for axis, value in zip(("x", "y"), connector_start):
        nodes[0]["extensions"].append({"namespace": NAMESPACE, "name": "connector_start_" + axis + "_pt",
            "property": {"field": "value", "selection": "requested", "sensitivity": "public", "evidence": evidence,
                         "state": {"availability": "known", "value": {"type": "number", "value": value}}}})
    nodes[0]["extensions"].append({"namespace": NAMESPACE, "name": "connector_start_space",
        "property": {"field": "value", "selection": "requested", "sensitivity": "public", "evidence": evidence,
                     "state": {"availability": "known", "value": {"type": "text", "value": space["id"]}}}})
    snapshot = {
        "id": "guest-tour-" + observation_id, "revision": 1,
        "context": {"schema_version": "0.1.0", "session_id": observation_id,
                    "target": {"id": BUNDLE, "generation": "32187-1795448825.752163"},
                    "surfaces": [surface], "scope_id": scope, "projection": "design", "fields": ["layout_bounds"],
                    "plugin": {"id": NAMESPACE, "version": "0.1.0"}, "environment_revision": "local-only-" + observation_id},
        "observations": [{"id": observation_id, "source_namespace": NAMESPACE, "channel": "external_semantics",
                          "clock_domain": "owner-lldb-" + observation_id, "time_unit": "milliseconds",
                          "start": start / 1_000_000, "end": end / 1_000_000,
                          "answer_source": "live", "freshness": "unverified", "freshness_basis": "live_read", "last_verified": None,
                          "consistency": "unknown", "consistency_reason": "single-layout-execution-not-presented-frame", "coverage": coverage}],
        "nodes": nodes, "relations": [], "components": [], "surface_records": [], "captures": [], "coverage": coverage,
        "focus": {"keyboard": {"status": "unknown", "reason": "not_observed"}, "accessibility": {"status": "unknown", "reason": "not_observed"},
                  "active_descendant": {"status": "unknown", "reason": "not_observed"}, "text_selection": None,
                  "composition_state": {"field": "value", "selection": "not_requested"}},
        "source_state": None,
    }
    return {"schema_version": "0.1.0", "artifact": {"kind": "snapshot", "data": snapshot}}


def normalize_component(record):
    """Three attributed read intervals; SwiftUI global is not screen space."""
    require(set(record) == {"format", "card", "module_uuid", "source_revision", "records", "units", "time_unit",
                            "clock_domain", "detached_verified", "stationary_window", "before", "after", "fixture",
                            "consistency", "groups"})
    require(record["fixture"] is False and record["stationary_window"] is True)
    require(record["detached_verified"] is True and record["module_uuid"] == MODULE)
    require(record["source_revision"] == "e19b20c474afe6462786ca0dd3dd5ad9ae37dbce")
    require(record["units"] == "pt" and record["time_unit"] == "ns" and record["groups"] == "not_collected")
    require(record["consistency"] == "sequential-source-reads-not-presented-frame")
    require(isinstance(record["clock_domain"], str) and re.fullmatch(r"owner-lldb-[a-f0-9]{32}", record["clock_domain"]))
    require(record["before"] == record["after"])
    require(isinstance(record["records"], list) and len(record["records"]) == 3)
    rows = record["records"]
    require([r["kind"] for r in rows] == ["map", "origin", "card"])
    prior = -1
    ids = set()
    for row in rows:
        require(set(row) == {"kind", "observation_id", "read_start", "read_end", "values"})
        require(isinstance(row["observation_id"], str) and re.fullmatch(r"[a-f0-9]{32}", row["observation_id"]))
        require(row["observation_id"] not in ids)
        ids.add(row["observation_id"])
        require(type(row["read_start"]) is int and type(row["read_end"]) is int and prior <= row["read_start"] <= row["read_end"] < 2**63)
        require(row["read_start"] >= 0)
        prior = row["read_end"]
    map_values, origin_values, card_values = [r["values"] for r in rows]
    require(set(map_values) == {"origin", "viewport", "scale"} and set(origin_values) == {"origin"}
            and set(card_values) == {"target", "rect", "start"})
    map_origin = numbers(map_values["origin"], 2)
    connection_origin = numbers(origin_values["origin"], 2)
    viewport = numbers(map_values["viewport"], 4)
    require(min(viewport[2:]) > 0 and numbers(map_values["scale"], 1)[0] > 0)
    card_row = rows[2]
    result = normalize({"source_site": SOURCE, "module_uuid": MODULE, "card": record["card"],
                        "space": "guest-tour GeometryReader local points", "target_xywh": card_values["target"],
                        "rect_xywh": card_values["rect"], "start_xy": card_values["start"],
                        "identity_before_after": IDENTITY, "detached_verified": True,
                        "observation_id": card_row["observation_id"], "clock_domain": "owner-lldb-process.monotonic",
                        "time_unit": "ns", "read_start": card_row["read_start"], "read_end": card_row["read_end"],
                        "units": "pt", "before": record["before"], "after": record["after"], "runtime_state": "stationary-window-source-reads"})
    snapshot = result["artifact"]["data"]
    context = snapshot["context"]
    context["scope_id"] = "guest-tour-component-" + record["card"]
    snapshot["coverage"]["scope_id"] = context["scope_id"]
    template = snapshot["observations"][0]
    snapshot["observations"] = []
    for row in rows:
        observation = copy.deepcopy(template)
        observation.update(id=row["observation_id"], start=row["read_start"] / 1_000_000,
                           end=row["read_end"] / 1_000_000, clock_domain=record["clock_domain"],
                           consistency_reason="sequential-source-reads-not-presented-frame")
        observation["coverage"] = copy.deepcopy(snapshot["coverage"])
        snapshot["observations"].append(observation)
    global_space = {"id": "swiftui-global-" + card_row["observation_id"], "kind": "local", "origin": "top_left", "units": "pt"}

    def transform(geometry, origin, row):
        geometry["transform"] = {"status": "known", "transform": {
            "from": geometry["coordinate_space"], "to": global_space, "affine": [1, 0, 0, 1, *origin],
            "target": context["target"], "surface": context["surfaces"][0],
            "environment_revision": context["environment_revision"],
            "evidence": {"source_namespace": NAMESPACE, "observation_id": row["observation_id"], "method": "swiftui-global-origin-translation",
                         "provenance": "derived", "uncertainty": None}}}

    for node in snapshot["nodes"]:
        transform(node["properties"][0]["state"]["value"]["value"], connection_origin, rows[1])
    map_node = copy.deepcopy(snapshot["nodes"][0])
    map_node["key"]["key"] = "map.viewport"
    map_node["source_declarations"] = []
    map_node["extensions"] = [{"namespace": NAMESPACE, "name": "annotation_scale",
        "property": {"field": "value", "selection": "requested", "sensitivity": "public",
                     "evidence": {"source_namespace": NAMESPACE, "observation_id": rows[0]["observation_id"],
                                  "method": "map-source-static-locals", "provenance": "reported", "uncertainty": None},
                     "state": {"availability": "known", "value": {"type": "number", "value": map_values["scale"][0]}}}}]
    prop = map_node["properties"][0]
    prop["evidence"].update(observation_id=rows[0]["observation_id"], method="map-source-static-locals")
    geometry = prop["state"]["value"]["value"]
    geometry["coordinate_space"]["id"] = "guest-tour-map-local-" + rows[0]["observation_id"]
    geometry["shape"]["value"] = dict(zip(("x", "y", "width", "height"), viewport))
    transform(geometry, map_origin, rows[0])
    snapshot["nodes"].append(map_node)
    return result


def normalize_group(record):
    """Only the selected card's reported membership; no group geometry or siblings."""
    require(set(record) == {"format", "card", "module_uuid", "source_revision", "records", "time_unit", "clock_domain",
                            "detached_verified", "stationary_window", "before", "after", "fixture"})
    require(record["fixture"] is False and record["stationary_window"] is True and record["detached_verified"] is True)
    require(record["module_uuid"] == MODULE and record["source_revision"] == "e19b20c474afe6462786ca0dd3dd5ad9ae37dbce")
    require(record["card"] in CARDS and record["before"] == record["after"])
    identity = record["before"]
    require(isinstance(identity, dict) and set(identity) == {*IDENTITY, "bundle", "window_screen_pt"})
    require(all(identity[k] == v for k, v in IDENTITY.items()) and identity["bundle"] == BUNDLE)
    require(min(numbers(identity["window_screen_pt"], 4)[2:]) > 0)
    require(record["time_unit"] == "ns" and isinstance(record["clock_domain"], str)
            and re.fullmatch(r"owner-lldb-[a-f0-9]{32}", record["clock_domain"]))
    require(isinstance(record["records"], list) and len(record["records"]) == 1)
    row = record["records"][0]
    require(set(row) == {"kind", "observation_id", "read_start", "read_end", "values"} and row["kind"] == "group")
    oid = row["observation_id"]
    require(isinstance(oid, str) and re.fullmatch(r"[a-f0-9]{32}", oid))
    require(type(row["read_start"]) is int and type(row["read_end"]) is int and 0 <= row["read_start"] <= row["read_end"] < 2**63)
    require(set(row["values"]) == {"availability", "group"} and row["values"]["availability"] == "known")
    group = row["values"]["group"]
    require(group in (None, "toolbar", "tabs", "modifiers", "query"))
    surface = {"id": "cg-window-1060", "generation": oid}
    scope = "guest-tour-group-" + record["card"]
    coverage = {"scope_id": scope, "fields": ["layout_bounds"], "status": "partial", "omitted_count": None, "unknown_count": 1}
    source = "PlayphrasemeGuestTourAnnotation.swift:104"
    key = {"namespace": NAMESPACE, "key": record["card"] + ".card"}
    declarations = [{"namespace": NAMESPACE, "name": name, "source": source, "sensitivity": "public",
                     "state": {"availability": "known", "value": {"type": "text", "value": value}}}
                    for name, value in (("card_id", record["card"]), ("group_id", group if group is not None else "none"),
                                        ("observation_id", oid), ("module_uuid", MODULE))]
    snapshot = {"id": "guest-tour-group-" + oid, "revision": 1, "source_state": None,
        "context": {"schema_version": "0.1.0", "session_id": oid, "target": {"id": BUNDLE, "generation": "32187-1795448825.752163"},
                    "surfaces": [surface], "scope_id": scope, "projection": "design", "fields": ["layout_bounds"],
                    "plugin": {"id": NAMESPACE, "version": "0.1.0"}, "environment_revision": "local-only-" + oid},
        "observations": [{"id": oid, "source_namespace": NAMESPACE, "channel": "external_semantics", "clock_domain": record["clock_domain"],
                          "time_unit": "milliseconds", "start": row["read_start"] / 1_000_000, "end": row["read_end"] / 1_000_000,
                          "answer_source": "live", "freshness": "unverified", "freshness_basis": "live_read", "last_verified": None,
                          "consistency": "unknown", "consistency_reason": "source-execution-not-presented-frame", "coverage": coverage}],
        "nodes": [{"key": key, "surface": surface, "children": [], "extensions": [], "properties": [
                   {"field": "layout_bounds", "selection": "requested", "sensitivity": "public",
                    "state": {"availability": "unknown", "reason": "group-only-observation"},
                    "evidence": {"observation_id": oid, "source_namespace": NAMESPACE, "method": "source-stored-group-field",
                                 "provenance": "reported", "uncertainty": None}}],
                   "native_role": {"availability": "unsupported", "reason": "not_collected"}, "source_declarations": declarations}],
        "components": [] if group is None else [{"logical_component_key": "guest-tour." + group, "members": [key],
                                               "declaration_source": source + ":" + oid, "provenance": "reported"}],
        "relations": [], "surface_records": [], "captures": [], "coverage": coverage,
        "focus": {"keyboard": {"status": "unknown", "reason": "not_observed"}, "accessibility": {"status": "unknown", "reason": "not_observed"},
                  "active_descendant": {"status": "unknown", "reason": "not_observed"}, "text_selection": None,
                  "composition_state": {"field": "value", "selection": "not_requested"}}}
    return {"schema_version": "0.1.0", "artifact": {"kind": "snapshot", "data": snapshot}}


def unique_object(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result)
        result[key] = value
    return result


def main():
    try:
        require(len(sys.argv) == 2)
        with Path(sys.argv[1]).open("rb") as source:
            raw = source.read(16385)
        require(len(raw) <= 16384)
        record = json.loads(raw, object_pairs_hook=unique_object)
        output = json.dumps(normalize(record), allow_nan=False, separators=(",", ":"))
    except (OSError, ValueError, TypeError, KeyError, OverflowError):
        print("unqualified_source_receipt", file=sys.stderr)
        return 2
    print(output)
    return 0


if __name__ == "__main__":
    sys.exit(main())
