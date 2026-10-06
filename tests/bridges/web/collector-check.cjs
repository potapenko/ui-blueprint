// Focused collector-only evidence with explicitly injected test request context.
// No Ticket, parent clock reading, lifecycle protocol or common-interface claim.
const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
const crypto=require('node:crypto');
const {execFileSync}=require('node:child_process');
const {withOwnedFixture}=require('./fixture-host.cjs');
const {connect,collect,FIELDS}=require('./collector.cjs');
const expected=require('../../../fixtures/web/expected.json').B03;
const checks=[],snapshots=[];
function check(name,actual,wanted){assert.deepEqual(actual,wanted,name);checks.push({name,expected:wanted,observed:actual});}
function domNode(snapshot,id){return snapshot.nodes.find(n=>n.key.namespace==='web.dom'&&n.source_declarations.some(d=>d.state.value?.value===id));}
function bounds(snapshot,id){return domNode(snapshot,id).properties.find(p=>p.field==='layout_bounds').state.value.value.shape.value;}
function rectArray(rect){return [rect.x,rect.y,rect.width,rect.height];}
async function uiState(page){return page.evaluate(()=>({active:document.activeElement.id,scroll:[scrollX,scrollY],state:window.f01.checkpoint()}));}
const watchdog=setTimeout(()=>{console.error('S01-Web collector check exceeded 25s');process.exit(1);},25000);
(async()=>{
 await withOwnedFixture(async({page,context})=>{
  await page.locator('#open-popup').click();
  await page.evaluate(()=>scrollTo(0,0));
  const binding=await connect(context,page);
  const testRequest={request_id:crypto.randomUUID(),clock_domain:'injected-test-parent-domain-no-readings',context:{
    schema_version:'0.1.0',session_id:`collector-only:${crypto.randomUUID()}`,target:binding.target,surfaces:[binding.surface],
    scope_id:'f01-b03-selected',projection:'design',fields:[...FIELDS],plugin:{id:'web-proof',version:'0.1.0'},environment_revision:'f01-800x600-dpr1'},
    limits:{max_elements:32,max_depth:8,max_output_bytes:65536,deadline_ms:250},freshness_policy:'current_required',operation:{operation:'observe',channels:['external_semantics']}};
  for(const state of ['popup-open','overlay-on','overlay-off']){
   if(state==='overlay-on')await page.evaluate(()=>window.f01.operate('overlay'));
   if(state==='overlay-off')await page.evaluate(()=>window.f01.operate('removeOverlay'));
   const before=await uiState(page);const result=await collect(testRequest,binding);
   check(`${state}:read-only UI state`,await uiState(page),before);
   check(`${state}:source state`,result.snapshot.source_state,state);
   check(`${state}:portal parent`,result.diagnostic.parent_tags.find(n=>n.id==='portal').parent,expected.portal_parent);
   check(`${state}:declared anchor`,result.diagnostic.anchors.find(n=>n.id==='portal').anchor,expected.anchor);
   const relation=result.snapshot.relations.find(r=>r.kind==='anchored_to');
   check(`${state}:normalized anchor`,[relation.from,relation.to],[domNode(result.snapshot,'portal').key,domNode(result.snapshot,'open-popup').key]);
   check(`${state}:point hit`,result.diagnostic.point_hit,state==='overlay-on'?expected.blocked_hit:expected.visible_hit);
   check(`${state}:clip bounds`,rectArray(bounds(result.snapshot,'clip-child')),expected.clip_bounds);
   const a=bounds(result.snapshot,'clip'),b=bounds(result.snapshot,'clip-child');
   check(`${state}:independent clip assertion`,Math.max(0,Math.min(a.x+a.width,b.x+b.width)-Math.max(a.x,b.x)),expected.clip_visible_width);
   check(`${state}:DOM/AX provenance`,result.snapshot.observations.map(o=>o.source_namespace),['web.dom','web.ax']);
   check(`${state}:partial retained`,result.snapshot.coverage.status,'partial');
   check(`${state}:source geometry unavailable`,result.snapshot.nodes.filter(n=>n.key.namespace==='web.ax').every(n=>n.properties.find(p=>p.field==='layout_bounds').state.availability==='unknown'),true);
   check(`${state}:bounded returned nodes`,result.snapshot.nodes.length<=testRequest.limits.max_elements,true);
   if(process.env.S01_WEB_VALIDATOR) {
    for(const document of [{schema_version:'0.1.0',artifact:{kind:'request',data:testRequest}},
      {schema_version:'0.1.0',artifact:{kind:'snapshot',data:result.snapshot}}]) {
     const resultText=execFileSync(process.env.S01_WEB_VALIDATOR,['--max-bytes','65536','-'],{input:JSON.stringify(document),timeout:5000,maxBuffer:4096,encoding:'utf8'});
     checks.push({name:`${state}:${document.artifact.kind}:Rust file validator`,expected:'exit0',observed:'exit0',machine_result:JSON.parse(resultText),evidence:'schema-only; no session lifecycle'});
    }
   }
   snapshots.push({state,request_context_kind:'injected-test-only',request:testRequest,snapshot:result.snapshot,diagnostic:result.diagnostic});
  }
  const beforeCalls=binding.calls.length;
  await assert.rejects(collect({...testRequest,context:{...testRequest.context,target:{id:'wrong-target',generation:'wrong'}}},binding));
  await assert.rejects(collect({...testRequest,context:{...testRequest.context,fields:['layout_bounds']}},binding));
  await assert.rejects(collect({...testRequest,limits:{...testRequest.limits,max_elements:1}},binding),/incomplete_scope/);
  check('injected wrong target/fields/limit performs no acquisition',binding.calls.length,beforeCalls);
  await assert.rejects(collect(testRequest,binding,0),/timeout/);
  check('expired budget performs no acquisition',binding.calls.length,beforeCalls);
  check('no whole-tree collection',binding.calls.some(x=>['DOMSnapshot.captureSnapshot','Accessibility.getFullAXTree','DOM.getDocument'].includes(x)),false);
  await binding.cdp.detach();
 });
 const files=['tests/bridges/web/collector.cjs','tests/bridges/web/collector-check.cjs','tests/bridges/web/fixture-host.cjs'];
 const hashes=Object.fromEntries(files.map(f=>[f,crypto.createHash('sha256').update(fs.readFileSync(path.resolve(__dirname,'../../..',f))).digest('hex')]));
 const report={kind:'collector-only-not-D02',schema_basis:'9d2df153abd2a7d7567100e06d4260e5edda3bb3',checks,snapshots,source_hashes:hashes};
 if(process.env.S01_WEB_OUTPUT)fs.writeFileSync(process.env.S01_WEB_OUTPUT,JSON.stringify(report,null,2)+'\n');
 console.log(JSON.stringify({status:'pass',checks:checks.length,common_boundary:'not_run',context:'injected-test-only'}));
})().catch(e=>{console.error(`S01-Web collector check failed: ${e.message}`);process.exitCode=1;}).finally(()=>clearTimeout(watchdog));
