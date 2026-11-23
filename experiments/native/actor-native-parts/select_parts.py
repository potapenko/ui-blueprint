"""Fixture-only native-overlay selection candidate for S03.

No attach, new target-call machinery, input parsing or product admission is owned here.
The existing qualified CacheReader must bind the graph/window before calling.
The caller must perform its unchanged privacy audit and canonical normalization.
Do not install this as a consumer recipe before contract/review qualification.
"""
import struct
from swiftui_debug_snapshot import Refusal

def select_native_overlays(self, host):
    scope=self.req['scope'];records={};types={};owners={}
    def record(element):
        if element not in records:
            if not element:raise Refusal('native_parts_root_unavailable')
            if len(records)>=1024:raise Refusal('native_parts_record_limit', 2)
            owner=self.page_owner(element);p=self.owned_record(element,32)
            records[element]=(owner,struct.unpack('<QIIIIII',self.read(p,32)))
        return records[element]
    def names(element):
        if element not in types:
            _,data=record(element);meta=self.pointer(data[0])
            typ=self.qualified_nominal(meta) if meta else None
            attribute,owner=self.rule_owner(data[1],owners)
            types[element]=(typ,attribute,owner)
        return types[element]
    views=self.native_views(host,scope['native_view_type'])
    window = self.c.objc(host, 'window')
    if not window or any(self.c.objc(view, 'window') != window for view in views):
        raise Refusal('native_parts_window_mismatch')
    bindings=[self.represented_layout(view) for view in views]
    if not bindings or len({a for a,sg in bindings})!=len(bindings):raise Refusal('native_anchor_unresolved')
    wanted={a for a,sg in bindings};matches={a:[] for a in wanted};indexed=set()
    for sg in {sg for a,sg in bindings}:
        pending=[(self.u32(sg+0x60), 1)]
        while pending:
            element, depth=pending.pop()
            if depth > self.req['limits']['max_depth']:
                raise Refusal('native_parts_index_depth', 2)
            if element in indexed:raise Refusal('native_parts_index_cycle')
            indexed.add(element);owner,data=record(element)
            if owner!=sg:raise Refusal('anchor_parent_ownership')
            value=data[6];props=set()
            while value:
                if value in props or len(props)>=32:raise Refusal('cache_value_cycle')
                props.add(value);v=self.owned_record(value,24);table=self.ptr(self.graph+0x120)
                index,count=self.u32(v+12),self.u32(table+8)
                if index>=count or count>4096:raise Refusal('cache_key_limit')
                key=self.cstring(self.ptr(self.ptr(table)+index*8),64)
                if key=='layoutComputer':
                    attr=self.u32(v+8)
                    if attr in wanted:matches[attr].append(element)
                value=self.u32(v+20)
            child=data[4]
            siblings = set()
            while child:
                if child in siblings:raise Refusal('native_parts_index_cycle')
                siblings.add(child)
                if len(pending)+len(indexed)>=1024:raise Refusal('native_parts_record_limit', 2)
                pending.append((child, depth+1));child=record(child)[1][5]
    if any(len(matches[a])!=1 for a in wanted):raise Refusal('anchor_scope_binding_unresolved')
    window_root=self.u32(self.ptr(self.ptr(self.host_graph(host)+0x28)+0x10)+0x60)
    selected=[]
    for anchor,sg in bindings:
        element=matches[anchor][0];seen=set();overlay=None;source_owners=set();container=False;complete=False
        while element:
            if element in seen:raise Refusal('native_parts_parent_cycle')
            if len(seen)>=self.req['limits']['max_depth']:raise Refusal('anchor_parent_limit', 2)
            seen.add(element);owner,data=record(element);name,attribute,body=names(element)
            if overlay is None and name=='SwiftUI._OverlayModifier':overlay=element
            if overlay is not None and name==scope['containing_type']:container=True
            if body==scope['type']:source_owners.add(attribute)
            if element==window_root:complete=True;break
            if data[3]:
                parent=data[3]
                if record(parent)[0]!=owner:raise Refusal('anchor_parent_ownership')
                element=parent;continue
            if self.u32(owner+0x60)!=element:raise Refusal('anchor_subgraph_root')
            resolved,_=self.resolve(data[1],links_only=True)
            if not resolved:break
            parents={(o,p) for o,pairs in self.links.items() for p,a in pairs if a==resolved[0]}
            if len(parents)!=1:break
            owner,element=parents.pop()
            if record(element)[0]!=owner:raise Refusal('anchor_parent_ownership')
        if not complete:raise Refusal('native_parts_ancestry_incomplete')
        if container and overlay is not None:
            if len(source_owners)!=1:raise Refusal('native_parts_owner_ambiguous')
            selected.append((anchor,overlay,next(iter(source_owners)),matches[anchor][0]))
    if not selected or len({owner for a,e,owner,w in selected})!=1:raise Refusal('native_parts_owner_ambiguous')
    if len({e for a,e,owner,w in selected})!=len(selected):raise Refusal('native_parts_boundary_ambiguous')
    self.anchor_ids={a for a,e,owner,w in selected};self.anchor_matches=0
    self.native_part_witnesses={w for a,e,owner,w in selected}
    self.owner_scan_cost=self.scan_work=self.scanned_records=len(records)
    self.selected_source=None
    self.native_parts_selection={'native_candidates':len(views),'selected_parts':len(selected),'indexed_records':len(indexed),'discovery_records':len(records)}
    return [e for a,e,owner,w in selected]

def verify_native_overlays(self):
    if not self.native_part_witnesses<=self.raw_seen:raise Refusal('anchor_scope_binding_unresolved')

