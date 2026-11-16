"""Geometry-only normalization of the qualified SwiftUI debug getter.

Raw input belongs in transient memory only. This module never writes it or
includes source-controlled values in exceptions. Analytics remain in Rust.
"""
import json
import math
import re
import uuid

NAMESPACE = 'swiftui.debug.layout'
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
    if not isinstance(data, dict) or set(data) != required or data['request_version'] != '1.0.0':
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


def snapshot(raw, req, metadata, start, end):
    """Return canonical bytes; unqualified transforms never join node spaces."""
    source = decode(raw)
    if not isinstance(source, list) or not source:
        raise Refusal('layout_not_available')
    if not all(type(x) in (int, float) and math.isfinite(x) for x in (start, end)) or end < start:
        raise Refusal('invalid_interval', 2)
    # Correlation IDs supplied by callers may repeat in different destinations.
    # They cannot create identity continuity between two external observations.
    nonce = uuid.uuid4().hex
    surface = dict(id='native-window-'+str(req['window_id']), generation=nonce)
    evidence = dict(observation_id='debug-'+nonce, source_namespace=NAMESPACE,
                    provenance='reported', method='NSHostingView.makeViewDebugData.position-size', uncertainty=None)
    nodes = []

    def prop(field, state):
        return dict(selection='requested', field=field, sensitivity='public', evidence=evidence, state=state)

    def visit(node, depth):
        if len(nodes) >= req['limits']['max_nodes'] or depth > req['limits']['max_depth']:
            raise Refusal('tree_limit', 2)
        if not isinstance(node, dict) or not isinstance(node.get('properties'), list):
            raise Refusal('invalid_tree', 2)
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
        layout = dict(availability='unknown', reason='position-or-size-not-exposed')
        if 3 in props and 4 in props:
            x, y = _pair(props[3], '__C.CGPoint')
            width, height = _pair(props[4], '__C.CGSize', True)
            geometry = dict(frame_kind='layout_bounds',
                coordinate_space=dict(id=key['key']+'-source-local', kind='local', units='pt', origin='top_left'),
                shape=dict(shape='rect', value=dict(x=x, y=y, width=width, height=height)),
                transform=dict(status='unknown', reason='debug-transform-not-qualified'))
            layout = dict(availability='known', value=dict(type='geometry', value=geometry))
        result = dict(key=key, surface=surface, native_role=native_role,
            properties=[prop('layout_bounds', layout)]+[prop(f, dict(availability='unknown',
                reason='not-exposed-by-layout-getter')) for f in FIELDS[1:]],
            children=[], extensions=[], source_declarations=[])
        nodes.append(result)
        result['children'] = [visit(child, depth+1) for child in children]
        return key

    for root in source:
        visit(root, 1)
    unknown = sum(p['state']['availability'] != 'known' for n in nodes for p in n['properties'])
    coverage = dict(status='partial', scope_id='selected-window-debug-layout', fields=FIELDS,
                    omitted_count=None, unknown_count=unknown)
    context = dict(schema_version='0.1.0', session_id='debug-'+nonce, target=metadata['target'],
        surfaces=[surface], scope_id=coverage['scope_id'], projection='design', fields=FIELDS,
        plugin=dict(id='swiftui-debug-capture', version='1.0.0'), environment_revision=nonce)
    observation = dict(id=evidence['observation_id'], source_namespace=NAMESPACE,
        channel='opt_in_layout_probe', start=start, end=end, clock_domain='capture-'+nonce,
        time_unit='seconds', freshness_basis='live_read', answer_source='cache', freshness='unverified',
        last_verified=None, consistency='unknown', consistency_reason='debug-tree-not-presented-frame', coverage=coverage)
    value = dict(id='debug-'+nonce, revision=1, source_state=None, context=context,
        surface_records=[], observations=[observation], nodes=nodes, relations=[], components=[],
        focus=dict(keyboard=dict(status='not_requested'), accessibility=dict(status='not_requested'),
                   active_descendant=dict(status='not_requested'), text_selection=None,
                   composition_state=dict(selection='not_requested', field='value')),
        captures=[], coverage=coverage)
    return encode(dict(schema_version='0.1.0', artifact=dict(kind='snapshot', data=value)),
                  req['limits']['max_output_bytes'])
