"""LLDB entry for the owned S03 cursor fixture; not a product capture recipe.

Load a pinned collector directory on sys.path before importing this module.
The existing capture owns target identity, deadlines, recovery and publication.
"""
import inspect
import os
from pathlib import Path
import sys
import textwrap

sys.dont_write_bytecode = True
sys.path.insert(0, str(Path(__file__).resolve().parent))
import swiftui_ag_cache as cache
import swiftui_debug_capture as tool
from swiftui_debug_snapshot import Refusal, decode, encode
from select_parts import select_native_overlays, verify_native_overlays

FIXTURE_BUNDLE = 'me.uiblueprint.external-layout.canary.actor-cursor-s03'
FIXTURE_SCOPE = {
    'type': 'ActorScopeFixture.SourceActorPicker',
    'containing_type': 'SwiftUI.LazyVStack',
    'native_view_type': 'ActorScopeFixture.ActorCursorFixtureView',
    'selection': 'geometry_records',
}


def capture(debugger, request_path, bundle_path, output_path):
    old_admission = tool.require_admitted_target
    old_fields = tool.collect_fields
    old_scope = cache.CacheReader.anchored_scope
    old_verify = cache.CacheReader.verify_anchor_ancestry
    old_collect = cache.CacheReader.collect
    selection = None

    def admit(request):
        if request['expected_bundle'] != FIXTURE_BUNDLE or request.get('scope') != FIXTURE_SCOPE:
            raise Refusal('actor_fixture_required')
        return old_admission(request)

    request = tool.read_request(request_path)
    admit(request)

    # A finite experiment adapts only the existing one-anchor count. Every selected
    # witness must also occur in the collected raw tree, so one missing witness
    # cannot be hidden by duplicate matches for another native view.
    source = textwrap.dedent(inspect.getsource(old_collect))
    old_check = 'if self.anchor_matches != 1:'
    if source.count(old_check) != 1:
        raise Refusal('actor_fixture_collector_revision')
    source = source.replace(old_check, 'if self.anchor_matches != len(self.native_part_witnesses):')
    namespace = dict(cache.__dict__)
    exec(source, namespace)

    def collect_fields(calls, host, request):
        nonlocal selection
        raw = old_fields(calls, host, request)
        selection = dict(cache.reader(calls, request).native_parts_selection)
        return raw

    destination = Path(output_path)
    if (destination/'native-parts-selection.json').exists():
        raise Refusal('destination_conflict', 2)
    try:
        tool.require_admitted_target = admit
        tool.collect_fields = collect_fields
        cache.CacheReader.anchored_scope = select_native_overlays
        cache.CacheReader.verify_anchor_ancestry = verify_native_overlays
        cache.CacheReader.collect = namespace['collect']
        tool.capture(debugger, request_path, bundle_path, output_path)
    finally:
        tool.require_admitted_target = old_admission
        tool.collect_fields = old_fields
        cache.CacheReader.anchored_scope = old_scope
        cache.CacheReader.verify_anchor_ancestry = old_verify
        cache.CacheReader.collect = old_collect

    manifest = decode((destination/('manifest-'+request['request_id']+'.json')).read_bytes())
    if manifest['status'] == 'ready' and selection is not None:
        fd = os.open(destination, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
        try:
            tool.write_new(fd, 'native-parts-selection.json', encode(selection))
        finally:
            os.close(fd)
