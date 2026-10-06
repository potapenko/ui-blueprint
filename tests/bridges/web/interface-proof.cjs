// Platform orchestration only: existing common Rust host owns Ticket/clock/lifecycle.
const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
const crypto=require('node:crypto');
const {spawn}=require('node:child_process');
const {performance}=require('node:perf_hooks');
const {withOwnedFixture}=require('./fixture-host.cjs');
const {connect,collect,FIELDS,SIZING_IDS}=require('./collector.cjs');
const sizing=process.argv.includes('--sizing-input');
const scopeId=sizing?'f01-d05-form-popup':'f01-b03-selected';
const frameBytes=sizing?131072:65536;
const pendingBytes=sizing?262144:131072;
const oracle=require('../../../fixtures/web/expected.json').B03;
const hostPath=process.env.S01_WEB_HOST;
const output=process.env.S01_WEB_PROOF_DIR;
assert.ok(hostPath&&path.isAbsolute(hostPath)&&output&&path.isAbsolute(output));
fs.mkdirSync(output); // Never overwrite another proof/evidence directory.
const hostChildren=new Set(),results=[],live=[];
let ownedBrowser;
const watchdog=setTimeout(()=>{
 for(const child of hostChildren)child.kill('SIGKILL');
 ownedBrowser?.close().finally(()=>process.exit(1));
 setTimeout(()=>process.exit(1),2000).unref();
},40000);
const doc=(kind,data)=>({schema_version:'0.1.0',artifact:{kind,data}});
const digest=value=>crypto.createHash('sha256').update(JSON.stringify(value)).digest('hex');
function makeInput(binding,cached=false){
 const sessionId=crypto.randomUUID();
 const session={allowed_scopes:[scopeId],session_id:sessionId,plugin:{id:'web-proof',version:'0.1.0'},supported_versions:['0.1.0'],target:binding.target,surfaces:[binding.surface],capabilities:[{channel:'external_semantics',operation:'observe',status:'partial',reason:'source-specific-geometry-unknown'}]};
 const request={request_id:crypto.randomUUID(),clock_domain:`rust-parent:${crypto.randomUUID()}`,context:{schema_version:'0.1.0',session_id:sessionId,target:binding.target,surfaces:[binding.surface],scope_id:scopeId,projection:'design',fields:[...FIELDS],plugin:session.plugin,environment_revision:'f01-800x600-dpr1'},limits:{max_elements:32,max_depth:8,max_output_bytes:frameBytes,deadline_ms:250},freshness_policy:cached?'cached_allowed':'current_required',operation:{operation:'observe',channels:['external_semantics']}};
 return {session,request};
}
function channel(request,ticket,snapshot){
 assert.equal(ticket.request_id,request.request_id);assert.equal(ticket.session_id,request.context.session_id);
 return doc('channel_response',{request_id:ticket.request_id,session_id:ticket.session_id,dispatch_sequence:ticket.sequence,target:request.context.target,channel:'external_semantics',result:{status:'observed',data:snapshot}});
}
async function startHost(name,session,request,mode='complete'){
 const dir=path.join(output,name);fs.mkdirSync(dir);
 fs.writeFileSync(path.join(dir,'session.json'),JSON.stringify(doc('session',session)));
 fs.writeFileSync(path.join(dir,'request.json'),JSON.stringify(doc('request',request)));
 const retained=path.join(dir,'retained');
 const child=spawn(hostPath,[path.join(dir,'session.json'),path.join(dir,'request.json'),String(frameBytes),String(pendingBytes),mode,'1000','8',retained],{stdio:['pipe','pipe','pipe']});
 hostChildren.add(child);const receipts=[];let buffer='',stderr='',waiter,exited=false;
 const timeout=setTimeout(()=>child.kill('SIGKILL'),4000);
 const exit=new Promise((resolve,reject)=>{
  child.once('error',reject);
  child.once('close',(code,signal)=>{exited=true;clearTimeout(timeout);hostChildren.delete(child);if(waiter){waiter.reject(new Error('host ended before receipt'));waiter=null;}resolve({code,signal});});
 });
 child.stderr.on('data',b=>{stderr+=b.toString();if(stderr.length>4096)child.kill('SIGKILL');});
 child.stdin.on('error',()=>{}); // EPIPE after terminal is inspected via exit/receipts.
 child.stdout.on('data',b=>{
  buffer+=b.toString();if(buffer.length>16384){child.kill('SIGKILL');return;}
  let i;while((i=buffer.indexOf('\n'))>=0){
   const text=buffer.slice(0,i);buffer=buffer.slice(i+1);
   try{const receipt={...JSON.parse(text),received_node_ms:performance.now()};receipts.push(receipt);if(waiter&&waiter.event===receipt.event){waiter.resolve(receipt);waiter=null;}}
   catch{child.kill('SIGKILL');}
  }
 });
 return {dir,retained,receipts,exit,
  async next(event){const old=receipts.find(r=>r.event===event);if(old)return old;if(exited)throw new Error(`host ended before ${event}`);return new Promise((resolve,reject)=>{waiter={event,resolve,reject};});},
  send(value){child.stdin.write(typeof value==='string'?value:JSON.stringify(value)+'\n');},
  end(){child.stdin.end();},stop(){child.kill('SIGKILL');},stderr:()=>stderr};
}
function assertOracle(result,state){
 const s=result.snapshot,d=result.diagnostic;
 assert.equal(d.parent_tags.find(x=>x.id==='portal').parent,oracle.portal_parent);
 assert.equal(d.anchors.find(x=>x.id==='portal').anchor,oracle.anchor);
 assert.equal(d.point_hit,state==='overlay-on'?oracle.blocked_hit:oracle.visible_hit);
 const node=s.nodes.find(n=>n.key.namespace==='web.dom'&&n.source_declarations.some(x=>x.state.value?.value==='clip-child'));
 const r=node.properties.find(p=>p.field==='layout_bounds').state.value.value.shape.value;
 assert.deepEqual([r.x,r.y,r.width,r.height],oracle.clip_bounds);
 assert.equal(s.coverage.status,'partial');
 assert.ok(s.relations.some(r=>r.kind==='anchored_to'));
}
async function retainedEqual(host,frame){
 const returnedBytes=fs.readFileSync(path.join(host.retained,'channel-0.json'));
 const retained=JSON.parse(returnedBytes);
 assert.deepEqual(retained,frame,'actual Rust Completion data preserves submitted canonical frame');
 return {returned_wire_bytes:returnedBytes.length,returned_wire_sha256:crypto.createHash('sha256').update(returnedBytes).digest('hex'),submitted_sha256:digest(frame),retained_sha256:digest(retained),nodes:retained.artifact.data.result.data.nodes.length};
}
(async()=>{
 await withOwnedFixture(async({context,page})=>{
  ownedBrowser=context.browser();await page.locator('#open-popup').click();await page.evaluate(()=>scrollTo(0,0));
  const binding=await connect(context,page);
  for(const state of (sizing?['overlay-on']:['popup-open','overlay-on','overlay-off'])){
   if(state==='overlay-on')await page.evaluate(()=>window.f01.operate('overlay'));
   if(state==='overlay-off')await page.evaluate(()=>window.f01.operate('removeOverlay'));
   const {session,request}=makeInput(binding);const host=await startHost(state,session,request);
   try{
    const ticket=await host.next('ticket');
    const result=await collect(request,binding,Math.max(0,request.limits.deadline_ms-ticket.parent_elapsed_ms));
    assert.ok(result.snapshot.observations.every(o=>o.start>=ticket.received_node_ms),'real Ticket receipt precedes requested field collection');
    assert.ok(result.snapshot.observations.every(o=>o.clock_domain!==ticket.parent_clock_domain));
    assertOracle(result,state);
    if(sizing){
     assert.deepEqual(result.diagnostic.fixed_ids,SIZING_IDS);
     assert.equal(new Set(result.snapshot.nodes.map(n=>`${n.key.namespace}:${n.key.key}`)).size,result.snapshot.nodes.length);
     assert.ok(result.snapshot.nodes.length<=32);
     assert.equal(result.diagnostic.dom_returned+result.diagnostic.missing_ids.length,SIZING_IDS.length);
     assert.ok(result.snapshot.nodes.every(n=>n.properties.length===FIELDS.length));
    }
    const frame=channel(request,ticket,result.snapshot);
    host.send(frame);const terminal=await host.next('terminal');host.end();
    const exit=await host.exit;assert.equal(exit.code,0);assert.equal(terminal.terminal,'Completed');
    assert.equal(terminal.injected_control,false);assert.deepEqual(terminal.missing_channels,[]);
    const equality=await retainedEqual(host,frame);
    results.push({case:state,source:'live',control:'normal',exit,receipts:host.receipts,equality,diagnostic:result.diagnostic,oracle:'existing F01/B03',ticket_before_source_read:true});
    live.push(result.snapshot);
   }catch(e){host.stop();await host.exit;throw e;}
  }
  // Recorded live data + explicitly injected lifecycle controls. No new UI reads.
  if(!sizing){
  for(const [mode,wanted] of [['cancel-after-first','Cancelled'],['detach-after-first','Detached'],['expire-after-first','TimedOut']]){
   const {session,request}=makeInput(binding,true);const host=await startHost(mode,session,request,mode);
   try{
    const ticket=await host.next('ticket');const snapshot=structuredClone(live.at(-1));snapshot.context=request.context;
    for(const o of snapshot.observations){o.answer_source='cache';o.freshness='stale';o.freshness_basis='unverified';}
    const frame=channel(request,ticket,snapshot);host.send(frame);
    const terminal=await host.next('terminal');assert.equal(terminal.terminal,wanted);assert.equal(terminal.injected_control,true);
    host.send(frame);const late=await host.next('late_probe');host.end();
    const exit=await host.exit;assert.equal(exit.code,0);assert.equal(late.rejected,true);
    results.push({case:mode,source:'recorded-live-with-stale-cache-label',control:'injected',late_input:'canonical recorded frame correlated with actual Ticket',exit,receipts:host.receipts,equality:await retainedEqual(host,frame)});
   }catch(e){host.stop();await host.exit;throw e;}
  }
  // Canonical Web data through real framing/version checks; source is recorded.
  for(const kind of ['wrong-response-version','malformed-frame','oversize-frame']){
   const {session,request}=makeInput(binding,true);const host=await startHost(kind,session,request);
   try{
    const ticket=await host.next('ticket');const snapshot=structuredClone(live.at(-1));snapshot.context=request.context;
    for(const o of snapshot.observations){o.answer_source='cache';o.freshness='stale';o.freshness_basis='unverified';}
    const frame=channel(request,ticket,snapshot);
    if(kind==='oversize-frame'){
     host.send('x'.repeat(65537)+'\n');const rejection=await host.next('framing_rejected');assert.equal(rejection.code,'Oversize');host.end();
     const exit=await host.exit;assert.equal(exit.code,2);results.push({case:kind,source:'synthetic oversized bytes',control:'injected',exit,receipts:host.receipts});
    }else{
     host.send(kind==='malformed-frame'?'{\n':{...frame,schema_version:'99.0.0'});
     const rejection=await host.next('frame_rejected');assert.equal(rejection.code,'InvalidFrame');
     host.send(frame);const terminal=await host.next('terminal');host.end();
     const exit=await host.exit;assert.equal(exit.code,0);assert.equal(terminal.terminal,'Completed');
     results.push({case:kind,source:'recorded-live then injected bad frame; canonical recovery',control:'injected',exit,receipts:host.receipts,equality:await retainedEqual(host,frame)});
    }
   }catch(e){host.stop();await host.exit;throw e;}
  }
  }
  assert.ok(!binding.calls.some(x=>['DOMSnapshot.captureSnapshot','Accessibility.getFullAXTree','DOM.getDocument'].includes(x)));
  await binding.cdp.detach();
 });
 ownedBrowser=null;
 const hashes={};for(const f of ['interface-proof.cjs','collector.cjs','fixture-host.cjs'])hashes[f]=crypto.createHash('sha256').update(fs.readFileSync(path.join(__dirname,f))).digest('hex');
 const report={status:'pass',kind:sizing?'D05-Web-actual-normalized-input':'D02-Web-live-common-boundary',support_commit:'73d772e97efcf550ea4a4d3e8480b56509ebc548',schema_commit:'9d2df153abd2a7d7567100e06d4260e5edda3bb3',created_utc:new Date().toISOString(),host_sha256:crypto.createHash('sha256').update(fs.readFileSync(hostPath)).digest('hex'),source_hashes:hashes,limits:{frame_bytes:frameBytes,pending_bytes:pendingBytes,max_frames:8,request_deadline_ms:250,late_grace_ms:1000,host_process_watchdog_ms:4000,whole_run_watchdog_ms:40000},results,sizing_scope:sizing?{id:scopeId,dom_ids:SIZING_IDS,source_node_ceiling:32,max_depth:8,fields:FIELDS,counts_from_result:true}:null,
  limitations:['finite test support; no production adapter or P1 freeze','source clocks distinct from Rust parent Instant','injected lifecycle controls do not prove interruption of browser/OS syscalls','five protected review findings unchanged','generic common_support 3 tests/8 synthetic cases reused, not rerun','no B02/B04, real-site or polling runs']};
 fs.writeFileSync(path.join(output,'report.json'),JSON.stringify(report,null,2)+'\n');
 console.log(JSON.stringify({status:'pass',live_cases:sizing?1:3,injected_cases:sizing?0:6,retained_equality_cases:sizing?1:8,common_support:report.support_commit}));
})().catch(e=>{console.error(`S01-Web interface proof failed: ${e.message}`);process.exitCode=1;})
.finally(async()=>{for(const child of hostChildren)child.kill('SIGKILL');clearTimeout(watchdog);});
