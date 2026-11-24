#!/usr/bin/env python3
"""Focused transport/selection tests. Live tests use an owned running fixture."""
import importlib.util
import json
import hashlib
from pathlib import Path
import tempfile
import subprocess
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location("swiftui_runtime", Path(__file__).parents[2] / "plugins/macos/swiftui_runtime.py")
runtime = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runtime)


class RuntimeChecks(unittest.TestCase):
    def test_source_identity_tracks_dirty_and_untracked_input_bytes(self):
        with tempfile.TemporaryDirectory() as root:
            def git(*args):
                subprocess.run(["git", *args], cwd=root, check=True, capture_output=True)
            git("init")
            path = Path(root) / "App.swift"
            path.write_text("original")
            git("add", "App.swift")
            git("-c", "user.name=Fixture", "-c", "user.email=fixture@example.invalid", "commit", "-m", "fixture")
            clean = runtime.source_identity(root)
            self.assertFalse(clean["dirty"])
            path.write_text("changed")
            dirty = runtime.source_identity(root)
            self.assertEqual(clean["revision"], dirty["revision"])
            self.assertNotEqual(clean["git_visible_inputs_sha256"], dirty["git_visible_inputs_sha256"])
            generated = Path(root) / "NewView.swift"
            generated.write_text("one")
            untracked = runtime.source_identity(root)
            generated.write_text("two")
            self.assertNotEqual(untracked, runtime.source_identity(root))

    def test_launch_recipe_rejects_malformed_arguments_and_reserved_environment(self):
        with tempfile.TemporaryDirectory() as root:
            path = Path(root) / "recipe.json"
            recipe = dict(recipe_version="1.0.0", project_root=".", project="App.xcodeproj",
                          scheme="App", product="App.app", bundle="me.example.app")
            for extra in (dict(arguments="--wrong"), dict(environment={"SWIFTUI_VIEW_DEBUG": "287"})):
                path.write_text(json.dumps(dict(recipe, **extra)))
                with self.assertRaises(runtime.Failure):
                    runtime.project_recipe(path)

    def test_duplicate_json_refuses(self):
        with self.assertRaises(runtime.Failure):
            json.loads('{"id":"a","id":"b"}', object_pairs_hook=runtime.unique_object)

    def test_publication_preserves_existing_output(self):
        with tempfile.TemporaryDirectory() as root:
            path = Path(root) / "value.json"
            path.write_bytes(b"original")
            with self.assertRaises(FileExistsError):
                runtime.publish(path, b"replacement")
            self.assertEqual(path.read_bytes(), b"original")
            self.assertEqual(list(Path(root).iterdir()), [path])

    def test_endpoint_rejects_nonprivate_directory(self):
        with tempfile.TemporaryDirectory() as root:
            Path(root).chmod(0o755)
            with self.assertRaisesRegex(runtime.Failure, "endpoint_not_private"):
                runtime.endpoint(Path(root) / "session.json")

    def test_scoped_request_requires_capability_before_publication(self):
        for descriptor in ({}, {"capabilities": []}):
            with patch.object(runtime, "endpoint", return_value=(Path("/unused"), descriptor)), \
                 patch.object(runtime, "publish") as publish:
                with self.assertRaisesRegex(runtime.Failure, "subtree_selection_unsupported_by_endpoint"):
                    runtime.request("unused", "capture", subtree_type="Fixture.Dialog")
                publish.assert_not_called()

    def test_scoped_request_rejects_invalid_selector_before_publication(self):
        for name in ("", "bad\nname", "é" * 513):
            with patch.object(runtime, "endpoint", return_value=(Path("/unused"), {"capabilities": ["capture-subtree-type"]})), \
                 patch.object(runtime, "publish") as publish:
                with self.assertRaisesRegex(runtime.Failure, "invalid_subtree_type"):
                    runtime.request("unused", "capture", subtree_type=name)
                publish.assert_not_called()

    def test_request_shape_stays_legacy_without_opt_in(self):
        for selector in (None, "Fixture.Dialog"):
            with tempfile.TemporaryDirectory() as root:
                root = Path(root)
                descriptor = dict(run_id="run", capability="private", capabilities=["capture-subtree-type"])
                published = []
                def respond(path, raw):
                    payload = json.loads(raw)
                    published.append(payload)
                    (root / (payload["id"] + ".response.json")).write_text(json.dumps(dict(
                        protocol="1.0.0", request_id=payload["id"], run_id="run", status="failed", reason="sourceNotReady")))
                with patch.object(runtime, "endpoint", return_value=(root, descriptor)), \
                     patch.object(runtime, "publish", side_effect=respond):
                    with self.assertRaisesRegex(runtime.Failure, "sourceNotReady"):
                        runtime.request("unused", "capture", subtree_type=selector)
                expected = {"version", "id", "run", "capability", "operation", "window", "windowGeneration"}
                if selector is not None:
                    expected.add("subtreeType")
                    self.assertEqual(published[0]["subtreeType"], selector)
                self.assertEqual(set(published[0]), expected)
                self.assertEqual(list(root.iterdir()), [])

    def test_discovery_requires_capability_before_publication(self):
        with patch.object(runtime, "endpoint", return_value=(Path("/unused"), {"capabilities": ["capture-subtree-type"]})), \
             patch.object(runtime, "publish") as publish:
            with self.assertRaisesRegex(runtime.Failure, "discovery_unsupported_by_endpoint"):
                runtime.request("unused", "discover")
            publish.assert_not_called()

    def test_discovery_payload_binding_hash_and_owned_cleanup(self):
        for fault in (None, "binding", "hash"):
            with tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                descriptor = dict(run_id="run", capability="private", capabilities=["discover-types"])
                raw = b'{"discovery_version":"1.0.0"}'
                sentinel = root / "unrelated.json"
                sentinel.write_bytes(b"preserve")
                def respond(path, data):
                    request = json.loads(data)
                    self.assertEqual(request["operation"], "discover")
                    self.assertNotIn("subtreeType", request)
                    name = request["id"] + ".discovery.json"
                    if fault != "binding":
                        (root / name).write_bytes(raw)
                    response = dict(protocol="1.0.0", request_id=request["id"], run_id="run", status="ok",
                                    discovery="unrelated.json" if fault == "binding" else name,
                                    sha256="wrong" if fault == "hash" else hashlib.sha256(raw).hexdigest())
                    (root / (request["id"] + ".response.json")).write_text(json.dumps(response))
                with patch.object(runtime, "endpoint", return_value=(root, descriptor)), \
                     patch.object(runtime, "publish", side_effect=respond):
                    if fault:
                        expected = "invalid_discovery_binding" if fault == "binding" else "discovery_hash_mismatch"
                        with self.assertRaisesRegex(runtime.Failure, expected):
                            runtime.request("unused", "discover")
                    else:
                        self.assertEqual(runtime.request("unused", "discover")[1], raw)
                self.assertEqual(sentinel.read_bytes(), b"preserve")
                self.assertEqual(list(root.iterdir()), [sentinel])

    def test_large_capacity_and_result_only_bundle_admission(self):
        self.assertEqual(runtime.MAX_BYTES, 1_073_741_824)
        for kind in ("source_nodes", "native_nodes"):
            value = dict(kind=kind, limit=1048576, observed=1048577)
            self.assertEqual(runtime.source_limit_detail(value), value)
        for help_text, valid in ((b"Measure --json --result-only", True), (b"old --json", False)):
            result = subprocess.CompletedProcess([], 0, stdout=help_text, stderr=b"")
            with patch.object(runtime.subprocess, "run", return_value=result):
                if valid:
                    runtime.require_result_only("/unused", 30)
                else:
                    with self.assertRaisesRegex(runtime.Failure, "^result_only_measurement_unsupported_by_bundle$"):
                        runtime.require_result_only("/unused", 30)

    def test_source_limit_details_are_bounded_and_private(self):
        valid = dict(kind="source_nodes", limit=16384, observed=16385)
        self.assertEqual(runtime.source_limit_detail(valid), valid)
        for value in (None, {}, dict(valid, secret="PRIVATE_CANARY"),
                      dict(valid, kind="PRIVATE_CANARY"), dict(valid, kind=[]),
                      dict(valid, observed=True), dict(valid, observed=16384),
                      dict(valid, observed=1 << 64), dict(valid, limit=16385),
                      dict(kind="type_name_controls", limit=0, observed=2)):
            with self.assertRaisesRegex(runtime.Failure, "^invalid_source_limit_detail$"):
                runtime.source_limit_detail(value)
        for kind, limit, observed in (("type_name_bytes", 65536, 65537),
                                      ("source_depth", 512, 513), ("native_depth", 512, 513),
                                      ("native_nodes", 16384, 16385), ("type_name_controls", 0, 1),
                                      ("snapshot_sequence", (1 << 64) - 1, (1 << 64) - 1)):
            value = dict(kind=kind, limit=limit, observed=observed)
            self.assertEqual(runtime.source_limit_detail(value), value)

    def test_source_limit_preserves_legacy_reason_and_cleans_only_own_response(self):
        for detail in (None, dict(kind="source_nodes", limit=16384, observed=16385)):
            with tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                sentinel = root / "unrelated"
                sentinel.write_bytes(b"preserve")
                def respond(path, raw):
                    payload = json.loads(raw)
                    response = dict(protocol="1.0.0", request_id=payload["id"], run_id="run",
                                    status="failed", reason="sourceLimit")
                    if detail is not None:
                        response["source_limit"] = detail
                    (root / (payload["id"] + ".response.json")).write_text(json.dumps(response))
                with patch.object(runtime, "endpoint", return_value=(root, dict(run_id="run", capability="private"))), \
                     patch.object(runtime, "publish", side_effect=respond):
                    with self.assertRaisesRegex(runtime.Failure, "^sourceLimit$") as caught:
                        runtime.request("unused", "capture")
                    self.assertEqual(caught.exception.source_limit, detail)
                self.assertEqual(list(root.iterdir()), [sentinel])
                self.assertEqual(sentinel.read_bytes(), b"preserve")

    def test_type_selection_keeps_unknown_geometry(self):
        node = dict(key={"namespace": "swiftui.runtime.layout", "key": "n1"},
                    native_role=dict(availability="known", value=dict(type="text", value="SwiftUI.Button<SwiftUI.Text>")),
                    properties=[dict(field="layout_bounds", state=dict(availability="unknown", reason="not-exposed"))])
        doc = dict(artifact=dict(data=dict(nodes=[node])))
        selected = runtime.measured_nodes(doc, "SwiftUI.Button")
        self.assertEqual(len(selected), 1)
        self.assertEqual(selected[0][2]["availability"], "unknown")
        self.assertEqual(runtime.measured_nodes(doc, "SwiftUI.Text"), [])


if __name__ == "__main__":
    unittest.main()
