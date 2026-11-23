"""Fixture-only experiment: reuse existing leaf cuts at lazy cell boundaries.

No public request option or collector modification. Never admits a donor. The
normal bounded owner scan runs first; omitted cell interiors remain excluded.
"""
import argparse
import copy
import json
import re
from pathlib import Path
import shlex
import subprocess
import sys

sys.dont_write_bytecode = True
OWNER = "R11PreviewFixture.FeatureGateSubscribePreview"
BOUNDARY = "SwiftUI.HiddenForReuseEffect"
BUNDLES = {"me.uiblueprint.external-layout.canary.r11.preview",
           "me.uiblueprint.external-layout.canary.r11.preview.no-oracle"}


def require_fixture(req):
    if (req["expected_bundle"] not in BUNDLES or
            req.get("scope") != {"type": OWNER, "containing_type": "SwiftUI.LazyVGrid"}):
        raise ValueError("R11 fixture-only boundary experiment")


def capture(debugger, collector, request, bundle, out, backend):
    sys.path.insert(0, collector)
    import swiftui_debug_capture as tool
    import swiftui_ag_cache as cache
    from swiftui_debug_snapshot import Refusal, decode
    req = tool.read_request(Path(request)); require_fixture(req)
    original = tool.collect_fields
    identities = set()

    def collect_boundaries(calls, host, trusted_req):
        require_fixture(trusted_req)
        bounded = copy.deepcopy(trusted_req)
        bounded["scope"]["leaf_types"] = [BOUNDARY]
        reader = cache.CacheReader(calls, bounded)
        reader.anchor_next = {}  # no native witness path in this experiment
        qualified = reader.qualified_nominal
        # The framework boundary is a private nominal type. Public nominal
        # matching intentionally excludes its anonymous file context. Reuse the
        # collector's descriptor/symbol-validated private identity, not its short
        # display name, for this one fixture-qualified boundary.
        def cell_nominal(metadata):
            name = qualified(metadata)
            if name is not None or reader.nominal(metadata) != "HiddenForReuseEffect":
                return name
            reader.qualified_nominal = qualified
            try:
                private = reader.source_nominal_identity(metadata)
            finally:
                reader.qualified_nominal = cell_nominal
            if private and re.fullmatch(r"SwiftUI\.\(HiddenForReuseEffect in _[A-F0-9]{32}\)", private):
                descriptor = reader.ptr(metadata+8, 4)
                module = calls.t.ResolveLoadAddress(descriptor).GetModule().GetFileSpec().GetFilename()
                if module not in ("SwiftUI", "SwiftUICore"):
                    raise Refusal("r11_boundary_image")
                identities.add((private, module))
                return BOUNDARY
            return None
        reader.qualified_nominal = cell_nominal
        raw = reader.collect(host)
        cuts = calls.cache_audit["boundary_records"]
        if len(cuts) != 21 or any(cut["type"] != BOUNDARY for cut in cuts):
            raise Refusal("r11_cell_boundary_count")
        # Fixed typed structure, no text/position/oracle-based selection.
        document = decode(raw)
        grids = []
        def role(node):
            return next((p["attribute"]["type"] for p in node["properties"] if p["id"] == 0), None)
        def visit(node):
            if role(node) == "LazyVGrid": grids.append(node)
            for child in node["children"]: visit(child)
        for branch in document["branches"]:
            for root in branch: visit(root)
        if len(grids) != 1: raise Refusal("r11_grid_ambiguous")
        node = grids[0]
        for name in ("ResettableLazyLayoutRoot", "Tree"):
            if len(node["children"]) != 1 or role(node["children"][0]) != name:
                raise Refusal("r11_grid_structure")
            node = node["children"][0]
        if len(node["children"]) != 21 or any(role(c) != "HiddenForReuseEffect" or c["children"] for c in node["children"]):
            raise Refusal("r11_cell_structure")
        return raw

    try:
        tool.collect_fields = collect_boundaries
        tool.capture(debugger, request, bundle, out, backend)
    finally:
        tool.collect_fields = original
    manifest = json.loads((Path(out)/("manifest-"+req["request_id"]+".json")).read_text())
    if manifest["status"] == "ready":
        with (Path(out)/"boundary-profile.json").open("x") as stream:
            json.dump(dict(experiment="r11-fixture-only", scope=req["scope"],
                source_identities=sorted(identities), framework_profile=cache.PROFILE,
                excluded="cell interiors; not visibility or complete view-tree coverage"), stream, indent=2)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("collector", "request", "bundle", "out", "backend-profile"):
        parser.add_argument("--"+name, required=True, type=Path)
    args = parser.parse_args()
    sys.path.insert(0, str(args.collector))
    import swiftui_debug_capture as tool
    req = tool.read_request(args.request); require_fixture(req)
    tool.require_admitted_target(req)
    lldb = subprocess.check_output(["xcrun", "--find", "lldb"], text=True, timeout=3).strip()
    if "lldb-2103.0.34.407" not in subprocess.check_output([lldb, "--version"], text=True, timeout=3):
        raise SystemExit("unsupported debugger")
    params = (args.collector, args.request, args.bundle, args.out, args.backend_profile)
    command = "command script import "+shlex.quote(str(Path(__file__).resolve()))+"\nscript capture_boundaries.capture(lldb.debugger, "+", ".join(repr(str(p.absolute())) for p in params)+")\n"
    tool.run_lldb(lldb, command)
    result = json.loads((args.out/("manifest-"+req["request_id"]+".json")).read_text())
    print(json.dumps(result))
    raise SystemExit(result["exit_status"])
