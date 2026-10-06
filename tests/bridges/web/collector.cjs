// Narrow F01/B03 test glue. No lifecycle, Ticket generation, oracle or whole-tree API.
const assert = require('node:assert/strict');
const crypto = require('node:crypto');
const { performance } = require('node:perf_hooks');
const IDS = Object.freeze(['open-popup', 'portal', 'close-popup', 'left', 'cover', 'clip', 'clip-child']);
const SIZING_IDS = Object.freeze([...IDS, 'f01', 'draft', 'suggestions', 'validation', 'commit', 'applied', 'surprise', 'responsive', 'sized']);
const SCOPES = Object.freeze({ 'f01-b03-selected': IDS, 'f01-d05-form-popup': SIZING_IDS });
const FIELDS = Object.freeze(['role', 'accessibility_name', 'layout_bounds', 'hit_region', 'visible_region']);
const known = (type, value) => ({ availability: 'known', value: { type, value } });
const unknown = reason => ({ availability: 'unknown', reason });
const key = (namespace, id) => ({ namespace, key: String(id) });
function issue(code) { const e = new Error(code); e.code = code; return e; }
async function connect(context, page) {
  const cdp = await context.newCDPSession(page);
  const calls = [];
  async function send(method, params = {}, deadline = performance.now() + 3000) {
    const remaining = deadline - performance.now();
    if (remaining <= 0) throw issue('timeout');
    let timer;
    try {
      const value = await Promise.race([cdp.send(method, params), new Promise((_, reject) => {
        timer = setTimeout(() => reject(issue('timeout')), remaining);
      })]);
      calls.push(method); return value;
    } finally { clearTimeout(timer); }
  }
  await send('DOM.enable'); await send('Accessibility.enable');
  const { targetInfo } = await send('Target.getTargetInfo');
  const { frameTree } = await send('Page.getFrameTree');
  const frame = frameTree.frame;
  assert.ok(frame.id && frame.loaderId);
  return { send, calls, cdp, target: { id: targetInfo.targetId, generation: crypto.randomUUID() },
    surface: { id: frame.id, generation: frame.loaderId }, clock: `web-collector:${crypto.randomUUID()}` };
}
function observation(namespace, context, start, end, id) {
  return { id, source_namespace: namespace, channel: 'external_semantics', start, end,
    clock_domain: context.clock, time_unit: 'milliseconds', freshness_basis: 'live_read',
    consistency_reason: 'sequential-source-reads-not-atomic', answer_source: 'live', freshness: 'current',
    last_verified: end, consistency: 'unknown', coverage: context.coverage };
}
function evidence(observation, method, provenance = 'reported') {
  return { observation_id: observation.id, source_namespace: observation.source_namespace,
    provenance, method, uncertainty: null };
}
function normalize(request, binding, records, observations, sourceState) {
  const ctx = request.context;
  const domObservation = observations[0], axObservation = observations[1];
  const nodes = [], relations = [];
  const roles = { button: 'button', dialog: 'dialog', BUTTON: 'button' };
  for (const item of records) {
    for (const source of ['dom', 'ax']) {
      const obs = source === 'dom' ? domObservation : axObservation;
      const rawRole = source === 'dom' ? item.dom.tag : item.ax?.role?.value;
      const sourceKey = source === 'dom' ? key('web.dom', item.backend) : key('web.ax', item.ax?.nodeId);
      if (source === 'ax' && !item.ax) continue;
      const properties = ctx.fields.map(field => {
        let state = unknown('not-exposed-by-selected-source');
        let method = source === 'dom' ? 'selected-dom-read' : 'cdp-partial-ax-tree';
        if (field === 'role') state = known('role', roles[rawRole] || (source === 'dom' ? roles[item.dom.role] : undefined) || 'unknown');
        if (field === 'accessibility_name' && source === 'ax' && item.ax.name) state = known('text', String(item.ax.name.value));
        if (field === 'layout_bounds' && source === 'dom') {
          method = 'cssom-getBoundingClientRect';
          state = known('geometry', { frame_kind: 'layout_bounds',
            coordinate_space: { id: `viewport:${binding.surface.id}`, kind: 'viewport', units: 'css_px', origin: 'top_left' },
            shape: { shape: 'rect', value: item.dom.rect }, transform: { status: 'local_only' } });
        }
        return { selection: 'requested', field, sensitivity: 'public', evidence: evidence(obs, method), state };
      });
      nodes.push({ key: sourceKey, surface: binding.surface, native_role: rawRole === undefined ? unknown('raw-role-unavailable') : known('text',rawRole),
        properties, children: [], extensions: [], source_declarations: [
          { namespace: 'f01.dom', name: 'element-id', state: known('text',item.id), sensitivity: 'public', source: 'live-dom-id-attribute' }
        ] });
    }
    if (item.ax) relations.push({ kind: 'corresponds_to', from: key('web.dom',item.backend), to: key('web.ax',item.ax.nodeId),
      evidence: evidence(axObservation,'cdp-backendDOMNodeId') });
  }
  const portal = records.find(r=>r.id==='portal'), trigger = records.find(r=>r.id==='open-popup');
  if (portal && trigger && portal.dom.anchor === trigger.id) relations.push({kind:'anchored_to',from:key('web.dom',portal.backend),to:key('web.dom',trigger.backend),evidence:evidence(domObservation,'fixture-data-anchor-attribute')});
  return {surface_records:[{identity:binding.surface,native_owner:known('identity',binding.target),initiated_by:null,anchor:null,evidence:evidence(domObservation,'cdp-target-frame-binding')}],
    id:crypto.randomUUID(),revision:1,source_state:sourceState,context:ctx,observations,nodes,relations,components:[],
    focus:{keyboard:{status:'not_requested'},accessibility:{status:'not_requested'},active_descendant:{status:'not_requested'},text_selection:null,composition_state:{selection:'not_requested',field:'value'}},
    captures:[],coverage:domObservation.coverage};
}
async function collect(request, binding, remainingMs = request.limits.deadline_ms) {
  // Parent supplies remaining budget; its clock is never read, fabricated or compared here.
  assert.deepEqual(request.context.target, binding.target, 'exact target');
  assert.deepEqual(request.context.surfaces, [binding.surface], 'exact document surface');
  const ids = SCOPES[request.context.scope_id];
  assert.ok(ids, 'registered fixture scope');
  assert.equal(request.operation.operation, 'observe');
  assert.deepEqual(request.operation.channels, ['external_semantics']);
  assert.deepEqual([...request.context.fields].sort(), [...FIELDS].sort(), 'selected proof fields only');
  if (request.limits.max_elements < ids.length * 2 || request.limits.max_depth < 1) throw issue('incomplete_scope');
  const deadline = performance.now() + Math.min(remainingMs, request.limits.deadline_ms);
  const group = `s01-web:${crypto.randomUUID()}`;
  const frame = (await binding.send('Page.getFrameTree',{},deadline)).frameTree.frame;
  if (frame.id!==binding.surface.id || frame.loaderId!==binding.surface.generation) throw issue('stale_target');
  const records = [], missing = [];
  const domStart = performance.now();
  try {
    // Fixed ID index lookups only. No textContent, descendants, selectors or full snapshot.
    for (const id of ids) {
      const result = await binding.send('Runtime.evaluate',{expression:`document.getElementById(${JSON.stringify(id)})`,objectGroup:group},deadline);
      if (result.result.subtype==='null') {missing.push(id);continue;}
      if (result.result.subtype!=='node') throw issue('target_unresolved');
      const objectId=result.result.objectId;
      const described=await binding.send('DOM.describeNode',{objectId,depth:0},deadline);
      const read=await binding.send('Runtime.callFunctionOn',{objectId,returnByValue:true,functionDeclaration:`function(){const r=this.getBoundingClientRect();return {connected:this.isConnected,tag:this.tagName,role:this.getAttribute('role'),parent_tag:this.parentElement?.tagName,anchor:this.getAttribute('data-anchor'),rect:{x:r.x,y:r.y,width:r.width,height:r.height}};}`},deadline);
      if(!read.result.value?.connected) throw issue('stale_target');
      records.push({id,backend:described.node.backendNodeId,dom:read.result.value});
    }
    const metadata=await binding.send('Runtime.evaluate',{expression:`({state:document.documentElement.dataset.sourceState,hit:(()=>{const n=document.getElementById('left'),r=n.getBoundingClientRect(),e=document.elementFromPoint(r.x+r.width/2,r.y+r.height/2);return e?.closest('button')?.id||e?.id})()})`,returnByValue:true},deadline);
    const domEnd=performance.now(),axStart=performance.now();
    let axReturned=0;
    for(const item of records) {
      const ax=await binding.send('Accessibility.getPartialAXTree',{backendNodeId:item.backend,fetchRelatives:false},deadline);
      axReturned+=ax.nodes.length;
      if(axReturned+records.length>request.limits.max_elements) throw issue('incomplete_scope');
      item.ax=ax.nodes.find(n=>n.backendDOMNodeId===item.backend);
    }
    const axEnd=performance.now();
    const after=(await binding.send('Page.getFrameTree',{},deadline)).frameTree.frame;
    if(after.id!==frame.id||after.loaderId!==frame.loaderId)throw issue('stale_target');
    const coverage={status:'partial',scope_id:request.context.scope_id,fields:request.context.fields,omitted_count:0,unknown_count:null};
    const ctx={clock:binding.clock,coverage};
    const observations=[observation('web.dom',ctx,domStart,domEnd,crypto.randomUUID()),observation('web.ax',ctx,axStart,axEnd,crypto.randomUUID())];
    const snapshot=normalize(request,binding,records,observations,metadata.result.value.state||null);
    if(Buffer.byteLength(JSON.stringify(snapshot))>request.limits.max_output_bytes)throw issue('incomplete_scope');
    if(performance.now()>=deadline)throw issue('timeout');
    return {snapshot,diagnostic:{fixed_ids:ids,missing_ids:missing,dom_returned:records.length,ax_returned:axReturned,
      parent_tags:records.map(r=>({id:r.id,parent:r.dom.parent_tag})),anchors:records.filter(r=>r.dom.anchor).map(r=>({id:r.id,anchor:r.dom.anchor})),
      point_hit:metadata.result.value.hit,source_state:metadata.result.value.state,partial_reason:'source-specific unavailable geometry; no arbitrary visibility/occlusion'}};
  } finally {
    // Cleanup has its own finite allowance; no further acquisition after deadline.
    await binding.send('Runtime.releaseObjectGroup',{objectGroup:group},performance.now()+1000);
  }
}
module.exports={connect,collect,IDS,SIZING_IDS,FIELDS};
