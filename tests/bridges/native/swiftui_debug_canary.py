"""LLDB-only live canary check on the explicitly owned synthetic app.

Invoke capture(debugger, request_path, bundle_path, output_path). All debugger
streams must be discarded, exactly as in the candidate's ordinary CLI caller.
The input is never saved; the receipt contains booleans and fixed codes only.
"""
import sys
sys.dont_write_bytecode = True
import json
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[3]/'plugins/macos'))
import swiftui_debug_capture as tool


def capture(debugger, request_path, bundle_path, output_path):
    req = tool.read_request(request_path)
    if not req['expected_bundle'].startswith('me.uiblueprint.external-layout.canary.'):
        raise RuntimeError('synthetic_fixture_required')
    canary = b'UIB_PRIVATE_MODEL_CANARY_86c2b1'
    observed = []
    original = tool.snapshot

    def checked(raw, *args):
        observed.append(canary in raw)
        result = original(raw, *args)
        if canary in result:
            raise RuntimeError('canary_leak')
        return result

    tool.snapshot = checked
    try:
        tool.capture(debugger, request_path, bundle_path, output_path)
    finally:
        tool.snapshot = original
    output = Path(output_path)
    manifest_bytes = (output/('manifest-'+req['request_id']+'.json')).read_bytes()
    manifest = json.loads(manifest_bytes)
    if observed != [True] or manifest['status'] != 'ready' or canary in manifest_bytes:
        raise RuntimeError('canary_verification_failed')
    snapshot = (output/manifest['snapshot']).read_bytes()
    if canary in snapshot:
        raise RuntimeError('canary_leak')
    receipt = dict(model_canary_observed_in_transient_input=True,
                   snapshot_canary_absent=True, manifest_canary_absent=True,
                   cleanup=manifest['cleanup'])
    with (output/'canary-check.json').open('x') as stream:
        json.dump(receipt, stream)
