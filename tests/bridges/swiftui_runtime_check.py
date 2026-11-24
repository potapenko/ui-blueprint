#!/usr/bin/env python3
"""Focused transport/selection tests. Live tests use an owned running fixture."""
import importlib.util
import json
from pathlib import Path
import tempfile
import subprocess
import unittest

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
