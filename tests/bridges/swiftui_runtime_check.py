#!/usr/bin/env python3
"""Focused transport/selection tests. Live tests use an owned running fixture."""
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

spec = importlib.util.spec_from_file_location("swiftui_runtime", Path(__file__).parents[2] / "plugins/macos/swiftui_runtime.py")
runtime = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runtime)


class RuntimeChecks(unittest.TestCase):
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
