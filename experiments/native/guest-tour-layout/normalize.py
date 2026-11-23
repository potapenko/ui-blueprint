"""Offline, fixed-G2 source-local receipt -> canonical Snapshot. No acquisition."""
import json
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
