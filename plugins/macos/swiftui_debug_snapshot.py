"""Geometry-only normalization of the qualified SwiftUI debug getter.

Raw input belongs in transient memory only. This module never writes it or
includes source-controlled values in exceptions. Analytics remain in Rust.
"""
import json
import math
import re
import uuid

NAMESPACE = 'swiftui.ag_cache.layout'
FIELDS = ['layout_bounds', 'hit_region', 'visible_region', 'paint_bounds']
MAX_BYTES = 1048576


class Refusal(Exception):
    def __init__(self, code, status=4):
        self.code, self.status = code, status
        super().__init__(code)


def _object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise Refusal('duplicate_json_key', 2)
        result[key] = value
    return result


def decode(raw):
    if not isinstance(raw, bytes) or len(raw) > MAX_BYTES:
        raise Refusal('source_byte_limit', 2)
    try:
        return json.loads(raw, object_pairs_hook=_object,
                          parse_float=lambda text: float(text) if math.isfinite(float(text)) else
                          (_ for _ in ()).throw(Refusal('nonfinite_json', 2)),
                          parse_constant=lambda _: (_ for _ in ()).throw(Refusal('nonfinite_json', 2)))
    except (ValueError, UnicodeError, RecursionError):
        raise Refusal('invalid_json', 2) from None


def encode(value, cap=MAX_BYTES):
    try:
        raw = (json.dumps(value, ensure_ascii=True, allow_nan=False,
                          separators=(',', ':')) + '\n').encode()
    except (ValueError, TypeError, RecursionError):
        raise Refusal('invalid_output', 2) from None
    if len(raw) > cap:
        raise Refusal('output_byte_limit', 2)
    return raw


def request(raw):
    data = decode(raw)
    required = {'request_version', 'request_id', 'pid', 'expected_bundle',
                'expected_launch_time', 'window_id', 'limits'}
    if not isinstance(data, dict) or set(data) not in (required, required | {'scope'}) or data['request_version'] != '1.0.0':
        raise Refusal('invalid_request', 2)
    for name in ('pid', 'window_id'):
        if type(data[name]) is not int or not 0 < data[name] <= 2147483647:
            raise Refusal('invalid_identity', 2)
    launch = data['expected_launch_time']
    if type(launch) not in (int, float) or not math.isfinite(launch) or launch <= 0:
        raise Refusal('invalid_identity', 2)
    for name, pattern in [('expected_bundle', r'[A-Za-z0-9][A-Za-z0-9.-]{0,255}'),
                          ('request_id', r'[A-Za-z0-9][A-Za-z0-9._-]{0,63}')]:
        if not isinstance(data[name], str) or not re.fullmatch(pattern, data[name]):
            raise Refusal('invalid_identity', 2)
    if 'scope' in data:
        scope = data['scope']
        scoped_native = isinstance(scope, dict) and 'native_match' in scope
        if scoped_native:
            required_keys = {'type', 'native_view_type', 'native_match'}
            if (scope['native_match'] not in ('within_owner', 'ancestor_owner', 'overlays_in_owner') or not required_keys <= set(scope)
                    or set(scope) - required_keys - {'containing_type', 'selection'}):
                raise Refusal('invalid_scope', 2)
        elif (not isinstance(scope, dict) or set(scope) not in
                ({'type'}, {'type', 'containing_type'}, {'type', 'containing_type', 'native_view_type'},
                 {'type', 'containing_type', 'native_view_type', 'selection'},
                 {'type', 'containing_type', 'native_view_type', 'selection', 'leaf_types'})):
            raise Refusal('invalid_scope', 2)
        if (scoped_native and scope['native_match'] == 'overlays_in_owner'
                and (set(scope) != {'type', 'native_view_type', 'native_match', 'containing_type', 'selection'}
                     or scope['selection'] != 'geometry_records')):
            raise Refusal('invalid_scope', 2)
        for key, value in scope.items():
            if key == 'native_match':
                continue
            if key == 'leaf_types':
                pattern = r'[A-Za-z_][A-Za-z0-9_]*(?:\.[A-Za-z_][A-Za-z0-9_]*)+'
                if (not isinstance(value, list) or not 1 <= len(value) <= 4
                        or any(not isinstance(v, str) or len(v)>256 or not re.fullmatch(pattern,v) for v in value)
                        or len(set(value)) != len(value) or scope['type'] in value):
                    raise Refusal('invalid_scope', 2)
                continue
            if key == 'selection':
                if value != 'geometry_records':
                    raise Refusal('invalid_scope', 2)
                continue
            simple = r'[A-Za-z_][A-Za-z0-9_]*'
            qualified = rf'{simple}(?:\.{simple})+'
            if key == 'native_view_type':
                # Swift demangles private nominal descriptors with the complete
                # file discriminator. Compare this string literally at lookup.
                qualified = rf'(?:{qualified}|{simple}\.\({simple} in _[A-F0-9]{{32}}\))'
            if not isinstance(value, str) or len(value) > 256 or not re.fullmatch(qualified, value):
                raise Refusal('invalid_scope', 2)
    limits = data['limits']
    ceilings = dict(max_nodes=256, max_depth=256, max_output_bytes=MAX_BYTES, deadline_ms=10000)
    if not isinstance(limits, dict) or set(limits) != set(ceilings):
        raise Refusal('invalid_limits', 2)
    if any(type(limits[k]) is not int or not 1 <= limits[k] <= v for k, v in ceilings.items()):
        raise Refusal('invalid_limits', 2)
    return data


def _pair(attribute, type_name, nonnegative=False):
    if not isinstance(attribute, dict) or attribute.get('type') != type_name:
        raise Refusal('invalid_geometry', 2)
    value = attribute.get('value')
    if not isinstance(value, list) or len(value) != 2:
        raise Refusal('invalid_geometry', 2)
    if any(type(x) not in (int, float) or not math.isfinite(x) for x in value):
        raise Refusal('invalid_geometry', 2)
    if nonnegative and any(x < 0 for x in value):
        raise Refusal('invalid_geometry', 2)
    return value


def native_affine(mapping, req):
    if mapping is None:
        return None
    expected = {'window_id', 'host_bounds', 'window_frame', 'host_is_flipped',
                'basis_window_points', 'source'}
    if (not isinstance(mapping, dict) or set(mapping) != expected
            or mapping['window_id'] != req['window_id']
            or type(mapping['window_id']) is not int or mapping['host_is_flipped'] is not True
            or mapping['source'] != 'NSView.convertPoint.to-window-base'):
        raise Refusal('invalid_native_mapping', 2)
    bounds, frame, points = mapping['host_bounds'], mapping['window_frame'], mapping['basis_window_points']
    if (not isinstance(points, (list, tuple)) or len(points) != 3
            or any(not isinstance(pair, (list, tuple)) or len(pair) != size
                   for pair, size in [(bounds, 4), (frame, 4), *((point, 2) for point in points)])
            or any(type(v) not in (int, float) or not math.isfinite(v)
                   for pair in (bounds, frame, *points) for v in pair)
            or bounds[:2] not in ([0, 0], (0, 0)) or min(*bounds[2:], *frame[2:]) <= 0):
        raise Refusal('invalid_native_mapping', 2)
    origin, x, y = points
    affine = [x[0]-origin[0], origin[1]-x[1], y[0]-origin[0], origin[1]-y[1],
              origin[0], frame[3]-origin[1]]
    determinant = affine[0]*affine[3]-affine[1]*affine[2]
    if not all(math.isfinite(v) for v in affine) or not math.isfinite(determinant) or determinant == 0:
        raise Refusal('invalid_native_mapping', 2)
    return affine


def snapshot(raw, req, metadata, start, end, selection_provenance=None, window_mapping=None):
    """Return canonical bytes; unqualified transforms never join node spaces."""
    window_affine = native_affine(window_mapping, req)
    source = decode(raw)
    selected = isinstance(source, dict)
    if selected:
        if (set(source) != {'selection_source', 'source_type', 'branches'}
                or source['selection_source'] not in ('body_owner', 'nominal_element', 'native_owner_ancestor', 'native_overlay_parts')
                or not isinstance(source['source_type'], str)
                or source['source_type'] != req.get('scope', {}).get('type')):
            raise Refusal('invalid_selection_provenance', 2)
        branches = source['branches']
        if (not isinstance(branches, list) or not 1 <= len(branches) <= 1024
                or any(not isinstance(branch, list) for branch in branches)
                or (source['selection_source'] in ('nominal_element', 'native_owner_ancestor') and len(branches) != 1)):
            raise Refusal('invalid_selection_provenance', 2)
    else:
        if not isinstance(source, list):
            raise Refusal('layout_not_available')
        branches = [source]
    if not any(branches):
        raise Refusal('layout_not_available')
    if not all(type(x) in (int, float) and math.isfinite(x) for x in (start, end)) or end < start:
        raise Refusal('invalid_interval', 2)
    # Correlation IDs supplied by callers may repeat in different destinations.
    # They cannot create identity continuity between two external observations.
    nonce = uuid.uuid4().hex
    surface = dict(id='native-window-'+str(req['window_id']), generation=nonce)
    evidence = dict(observation_id='debug-'+nonce, source_namespace=NAMESPACE,
                    provenance='reported', method='AttributeGraph.validated-cache.position-size', uncertainty=None)
    nodes = []
    child_order_unknown = False

    def prop(field, state):
        return dict(selection='requested', field=field, sensitivity='public', evidence=evidence, state=state)

    def visit(node, depth):
        nonlocal child_order_unknown
        if len(nodes) >= req['limits']['max_nodes'] or depth > req['limits']['max_depth']:
            raise Refusal('tree_limit', 2)
        if not isinstance(node, dict) or not isinstance(node.get('properties'), list):
            raise Refusal('invalid_tree', 2)
        child_order_unknown |= node.get('child_order_unknown') is True
        children = node.get('children', [])
        if not isinstance(children, list) or len(children) > req['limits']['max_nodes']:
            raise Refusal('invalid_tree', 2)
        props = {}
        for entry in node['properties']:
            if not isinstance(entry, dict) or type(entry.get('id')) is not int or entry['id'] in props:
                raise Refusal('invalid_property', 2)
            # Never copy or traverse attribute/value/environment payloads here.
            props[entry['id']] = entry.get('attribute') if entry['id'] in (0, 3, 4) else None
        key = dict(namespace=NAMESPACE, key=nonce+'-n'+str(len(nodes)))
        native_role = dict(availability='unknown', reason='source-type-unavailable')
        if 0 in props:
            attr = props[0]
            if not isinstance(attr, dict) or not isinstance(attr.get('type'), str):
                raise Refusal('invalid_type', 2)
            name = attr['type']
            if not name or len(name.encode()) > 4096 or any(ord(c) < 32 or ord(c) == 127 for c in name):
                raise Refusal('type_limit', 2)
            native_role = dict(availability='known', value=dict(type='text', value=name))
        reason = node.get('layout_unknown_reason', 'position-or-size-not-exposed')
        if reason not in ('position-or-size-not-exposed', 'cache_value_unavailable',
                          'cache_reference_stale', 'cache_value_dirty'):
            raise Refusal('invalid_cache_reason', 2)
        layout = dict(availability='unknown', reason=reason)
        if 3 in props and 4 in props:
            x, y = _pair(props[3], '__C.CGPoint')
            width, height = _pair(props[4], '__C.CGSize', True)
            geometry = dict(frame_kind='layout_bounds',
                coordinate_space=dict(id=key['key']+'-source-local', kind='local', units='pt', origin='top_left'),
                shape=dict(shape='rect', value=dict(x=x, y=y, width=width, height=height)),
                transform=dict(status='unknown', reason='debug-transform-not-qualified'))
            inputs = node.get('transform_inputs')
            if inputs is not None:
                if (not isinstance(inputs, dict)
                        or set(inputs) != {'position_adjustment', 'pending_translation', 'chain'}
                        or not isinstance(inputs['chain'], list) or len(inputs['chain']) > 80):
                    raise Refusal('invalid_transform_inputs', 2)
                pairs = [inputs['position_adjustment'], inputs['pending_translation'], *inputs['chain']]
                if any(not isinstance(pair, list) or len(pair) != 2
                       or any(type(v) not in (int, float) or not math.isfinite(v) for v in pair)
                       for pair in pairs):
                    raise Refusal('invalid_transform_inputs', 2)
                offset = [-sum(pair[axis] for pair in pairs) for axis in (0, 1)]
                if not all(math.isfinite(v) for v in offset):
                    raise Refusal('invalid_transform_inputs', 2)
                affine = [1, 0, 0, 1, *offset]
                destination = dict(id=nonce+'-hosting-root', kind='local', units='pt', origin='top_left')
                method = 'AttributeGraph.validated-translation-chain.raw-parent-path'
                if window_affine is not None:
                    a, b, c, d, tx, ty = window_affine
                    affine = [a, b, c, d, a*offset[0]+c*offset[1]+tx, b*offset[0]+d*offset[1]+ty]
                    if not all(math.isfinite(v) for v in affine):
                        raise Refusal('invalid_native_mapping', 2)
                    destination = dict(id=nonce+'-native-window', kind='window', units='pt', origin='top_left')
                    method += '.NSView.convertPoint'
                geometry['transform'] = dict(status='known', transform=dict(
                    **{'from': geometry['coordinate_space'], 'to': destination},
                    affine=affine, target=metadata['target'], surface=surface,
                    environment_revision=nonce, evidence=dict(evidence, provenance='derived', method=method)))
            layout = dict(availability='known', value=dict(type='geometry', value=geometry))
        result = dict(key=key, surface=surface, native_role=native_role,
            properties=[prop('layout_bounds', layout)]+[prop(f, dict(availability='unknown',
                reason='not-exposed-by-layout-getter')) for f in FIELDS[1:]],
            children=[], extensions=[], source_declarations=[])
        owner = node.get('body_owner')
        if owner is not None:
            if (not isinstance(owner, dict) or set(owner) != {'type', 'occurrence'}
                    or not isinstance(owner['type'], str) or not 1 <= len(owner['type']) <= 256
                    or any(ord(c) < 32 or ord(c) >= 127 for c in owner['type'])
                    or type(owner['occurrence']) is not int or not 0 <= owner['occurrence'] < req['limits']['max_nodes']):
                raise Refusal('invalid_body_owner', 2)
            owner_evidence = dict(evidence, method='AttributeGraph.validated-rule-type.ViewBodyAccessor')
            for name, text in (('body_owner_type', owner['type']),
                               ('body_owner_occurrence', nonce+'-body-'+str(owner['occurrence']))):
                result['extensions'].append(dict(namespace=NAMESPACE, name=name,
                    property=dict(selection='requested', field='description', sensitivity='public',
                                  evidence=owner_evidence, state=dict(availability='known',
                                      value=dict(type='text', value=text)))))
        nodes.append(result)
        result['children'] = [visit(child, depth+1) for child in children]
        return key

    published_branches = []
    for index, branch in enumerate(branches):
        keys = [visit(root, 1) for root in branch]
        if selected:
            published_branches.append(dict(branch_id=f'{nonce}-branch-{index}', root_keys=keys))
    unknown = sum(p['state']['availability'] != 'known' for n in nodes for p in n['properties'])
    coverage = dict(status='partial', scope_id=('selected-type:'+req['scope']['type'] if 'scope' in req
                                      else 'selected-window-debug-layout'), fields=FIELDS,
                    omitted_count=None, unknown_count=unknown)
    if req.get('scope', {}).get('selection') == 'geometry_records':
        coverage['scope_id'] += ':geometry-records-nearest-included-ancestor'
    if req.get('scope', {}).get('native_match') == 'overlays_in_owner':
        coverage['scope_id'] += ':mounted-native-overlay-parts'
    context = dict(schema_version='0.1.0', session_id='debug-'+nonce, target=metadata['target'],
        surfaces=[surface], scope_id=coverage['scope_id'], projection='design', fields=FIELDS,
        plugin=dict(id='swiftui-cache-layout', version='1.0.0'), environment_revision=nonce)
    observation = dict(id=evidence['observation_id'], source_namespace=NAMESPACE,
        channel='opt_in_layout_probe', start=start, end=end, clock_domain='capture-'+nonce,
        time_unit='seconds', freshness_basis='live_read', answer_source='cache', freshness='unverified',
        last_verified=None, consistency='unknown', consistency_reason=('cached-sibling-order-unqualified' if child_order_unknown
                            else 'cached-tree-not-presented-frame'), coverage=coverage)
    value = dict(id='debug-'+nonce, revision=1, source_state=None, context=context,
        surface_records=[], observations=[observation], nodes=nodes, relations=[], components=[],
        focus=dict(keyboard=dict(status='not_requested'), accessibility=dict(status='not_requested'),
                   active_descendant=dict(status='not_requested'), text_selection=None,
                   composition_state=dict(selection='not_requested', field='value')),
        captures=[], coverage=coverage)
    payload = encode(dict(schema_version='0.1.0', artifact=dict(kind='snapshot', data=value)),
                     req['limits']['max_output_bytes'])
    if selected and selection_provenance is not None:
        selection_provenance.update(observation_id=observation['id'], occurrence_id=nonce+'-occurrence',
                                    selection_source=source['selection_source'], source_type=source['source_type'],
                                    branches=published_branches)
    return payload
