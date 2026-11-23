"""Two explicit windows in one acquisition; no inferred popup attachment.

This module has no process/LLDB entry point. The existing capture owner retains
admission, budget, read privacy, cleanup and terminal publication authority.
"""
import copy
import math

from swiftui_debug_snapshot import Refusal, decode, encode, native_affine, snapshot


def charge_tree_record(reader, element):
    """Called only at typed TreeElement traversal entries, before header reads."""
    records = getattr(getattr(reader, 'c', None), 'pair_records', None)
    if records is not None:
        key = reader.graph, element
        if key not in records and len(records) >= 1024:
            raise Refusal('pair_tree_limit', 2)
        records.add(key)


def charge_node(calls, maximum):
    if hasattr(calls, 'pair_nodes'):
        if calls.pair_nodes >= maximum:
            raise Refusal('tree_limit', 2)
        calls.pair_nodes += 1


def charge_native_view(calls, view):
    seen = getattr(calls, 'pair_native_views', None)
    if seen is not None:
        if view not in seen and len(seen) >= 256:
            raise Refusal('view_tree_limit', 2)
        seen.add(view)


def requests(req):
    if not isinstance(req, dict):
        raise Refusal('invalid_surface_pair', 2)
    pair = req.get('paired_surface')
    if (not isinstance(pair, dict) or set(pair) != {'window_id', 'scope'}
            or type(pair['window_id']) is not int or not 0 < pair['window_id'] <= 2147483647
            or 'window_id' not in req or pair['window_id'] == req['window_id'] or 'scope' not in req):
        raise Refusal('invalid_surface_pair', 2)
    primary = {key: value for key, value in req.items() if key != 'paired_surface'}
    secondary = dict(primary, window_id=pair['window_id'], scope=pair['scope'])
    for subrequest in (primary, secondary):
        scope = subrequest['scope']
        if (not isinstance(scope, dict)
                or {'native_view_type', 'native_match', 'leaf_types'} & scope.keys()):
            raise Refusal('invalid_surface_pair', 2)
    return primary, secondary


def collect(calls, req, collect_window, revalidate):
    """Run the existing collector twice with shared accounting, then revalidate.

    No saved data or caller-provided link proof enters this path. If a hosting
    class is unsupported the existing collector refuses rather than guessing.
    """
    selected = requests(req)
    calls.pair_records, calls.pair_native_views = set(), set()
    calls.pair_nodes = 0
    calls.pair_reads, calls.pair_private_ranges = [], []
    records = []
    for subrequest in selected:
        if hasattr(calls, 'cache_reader'):
            del calls.cache_reader
        raw, state = collect_window(calls, subrequest)
        records.append(dict(raw=raw, state=state,
                            mapping=calls.native_layout_mapping,
                            audit=copy.deepcopy(calls.cache_audit),
                            binding=calls.selected_binding))
    for subrequest, record in zip(selected, records):
        # Recheck both bindings after the second graph read. A stable second
        # window alone cannot establish stability of the first.
        if revalidate(calls, subrequest, record['binding']) != record['mapping']:
            raise Refusal('stale_window')
    calls.cache_audit = dict(paired_surfaces=[r['audit'] for r in records],
                            unique_raw_records=len(calls.pair_records),
                            native_views=len(calls.pair_native_views),
                            evaluated_swiftui_or_graph=False)
    calls.native_layout_mapping = None
    return records


def encode_pair(records, req, metadata, start, end):
    """Normalize fresh in-memory acquisition records, never saved Snapshots."""
    selected = requests(req)
    if len(records) != 2:
        raise Refusal('incomplete_surface_pair')
    documents, provenances = [], []
    for record, subrequest in zip(records, selected):
        provenance = {}
        raw = snapshot(record['raw'], subrequest, metadata, start, end,
                       provenance, record['mapping'])
        documents.append(decode(raw)['artifact']['data'])
        provenances.append(provenance)
    first, second = documents
    if len(first['nodes'])+len(second['nodes']) > req['limits']['max_nodes']:
        raise Refusal('tree_limit', 2)
    context = first['context']
    context['surfaces'].extend(second['context']['surfaces'])
    nonce = context['environment_revision']
    # These records were collected in this same stopped interval. Do not expose
    # two fictitious clock domains merely because normalization used two nonces.
    for document in documents:
        for observation in document['observations']:
            observation['clock_domain'] = 'capture-'+nonce
    first['id'] = 'debug-pair-'+nonce
    context['scope_id'] = 'selected-window-pair'
    first['coverage']['scope_id'] = context['scope_id']
    first['coverage']['unknown_count'] += second['coverage']['unknown_count']
    window_space = dict(id=nonce+'-native-window', kind='window', units='pt', origin='top_left')
    mapping_known = all(record['mapping'] is not None for record in records)
    native_observation = None
    if mapping_known:
        # native_affine performs strict finite/basis/identity validation.
        for record, subrequest in zip(records, selected):
            native_affine(record['mapping'], subrequest)
        a, b = [record['mapping']['window_frame'] for record in records]
        translation = [b[0]-a[0], (a[1]+a[3])-(b[1]+b[3])]
        if not all(math.isfinite(value) for value in translation):
            raise Refusal('invalid_native_mapping', 2)
        native_observation = copy.deepcopy(first['observations'][0])
        native_observation['id'] = 'pair-native-'+nonce
        native_observation['coverage'] = copy.deepcopy(first['coverage'])
        native_observation['consistency_reason'] = 'same-stopped-interval-native-window-bases'
    for node in second['nodes']:
        for prop in node['properties']:
            if prop['field'] != 'layout_bounds' or prop['state']['availability'] != 'known':
                continue
            geometry = prop['state']['value']['value']
            if geometry['transform']['status'] != 'known':
                continue
            transform = geometry['transform']['transform']
            transform['environment_revision'] = nonce
            # Never turn an unsupported cached transform into known geometry.
            if mapping_known and transform['to']['kind'] == 'window':
                transform['affine'][4] += translation[0]
                transform['affine'][5] += translation[1]
                if not all(math.isfinite(value) for value in transform['affine']):
                    raise Refusal('invalid_native_mapping', 2)
                transform['to'] = window_space
                transform['evidence'] = dict(transform['evidence'],
                    observation_id=native_observation['id'],
                    method=transform['evidence']['method']+'.NSWindow.frame.paired-origin')
    first['nodes'].extend(second['nodes'])
    first['observations'].extend(second['observations'])
    if native_observation is not None:
        first['observations'].append(native_observation)
    for observation in first['observations']:
        observation['coverage']['scope_id'] = context['scope_id']
    # Neither same process, shared type, source declaration nor common geometry
    # proves an attachment. There is deliberately no accepted "proof" argument.
    receipt = dict(status='partial', attachment=dict(status='unknown',
                   reason='presentation-anchor-reader-not-qualified'),
                   common_space=window_space if mapping_known else None,
                   surfaces=[dict(window_id=sub['window_id'],
                                  requested_scope=sub['scope'],
                                  surface=doc['context']['surfaces'][0],
                                  selection_provenance=provenance,
                                  native_layout_mapping=record['mapping'])
                             for sub, doc, provenance, record in
                             zip(selected, documents, provenances, records)])
    payload = encode(dict(schema_version='0.1.0', artifact=dict(kind='snapshot', data=first)),
                     req['limits']['max_output_bytes'])
    if len(payload)+len(encode(receipt)) > req['limits']['max_output_bytes']:
        raise Refusal('output_limit', 2)
    return payload, receipt
