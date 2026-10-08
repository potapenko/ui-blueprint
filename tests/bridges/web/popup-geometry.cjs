// B03 author-run public CLI scenario. Only the owned headless context is operated.
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const os = require('node:os');
const crypto = require('node:crypto');
const http = require('node:http');
const {spawn} = require('node:child_process');
const {prepare} = require('./fixture-host.cjs');
const bounded = (promise, ms) => {let timer;return Promise.race([promise,new Promise((_,reject)=>{timer=setTimeout(()=>reject(Error('bounded timeout')),ms);})]).finally(()=>clearTimeout(timer));};
const property = (node,field) => node.properties.find(p=>p.field===field).state;
const fact = (node,name) => node.extensions.find(e=>e.name===name)?.property.state;
const rect = geometry => {const r=geometry.shape.value;return [r.x,r.y,r.width,r.height];};
function near(actual,expected){assert.equal(actual.length,expected.length);actual.forEach((n,i)=>assert(Math.abs(n-expected[i])<=0.01,`${actual} != ${expected}`));}
async function main(){
  assert(process.argv.includes('--run-authorized') && process.env.UIB_WEB_LIVE_ALLOW==='1');
  const cli=process.env.UIB_WEB_LIVE_CLI,worker=process.env.UIB_WEB_LIVE_WORKER;
  for(const file of [cli,worker]){assert(path.isAbsolute(file));await fs.access(file,require('node:fs').constants.X_OK);}
  const setup=prepare(),temp=await fs.mkdtemp(path.join(os.tmpdir(),'uib-b03-live-')),files=[];
  let server,browser,context,fixture,child,closed;
  const checks=[];
  async function save(name,data){const file=path.join(temp,name);await fs.writeFile(file,data,{flag:'wx',mode:0o600});files.push(file);return file;}
  let sequence=0;
  async function json(value){return save(`${sequence++}.json`,JSON.stringify(value));}
  async function call(args){
    child=spawn(cli,args,{stdio:['ignore','pipe','pipe']});let output='',diagnostics='';
    child.stdout.on('data',b=>{output+=b;if(Buffer.byteLength(output)>131072)child.kill('SIGKILL');});
    child.stderr.on('data',b=>{diagnostics+=b;if(Buffer.byteLength(diagnostics)>8192)child.kill('SIGKILL');});
    closed=new Promise((resolve,reject)=>{child.once('error',reject);child.once('close',(code,signal)=>resolve({code,signal}));});
    const exit=await bounded(closed,5000);assert.equal(exit.signal,null);
    return {code:exit.code,output,diagnostics};
  }
  try{
    const html=await fs.readFile(path.join(setup.root,'fixtures/web/popup-geometry.html'));
    fixture=http.createServer((req,res)=>{res.writeHead(200,{'Content-Type':'text/html; charset=utf-8'});res.end(html);});
    await bounded(new Promise(resolve=>fixture.listen(0,'127.0.0.1',resolve)),2000);
    const origin=`http://127.0.0.1:${fixture.address().port}`;
    server=await setup.chromium.launchServer({headless:true,timeout:8000,args:['--remote-debugging-port=0','--remote-debugging-address=127.0.0.1']});
    const profile=server.process().spawnargs.find(s=>s.startsWith('--user-data-dir=')).slice(16);
    const port=Number((await fs.readFile(path.join(profile,'DevToolsActivePort'),'utf8')).split('\n')[0]);assert(port>0);
    browser=await setup.chromium.connect(server.wsEndpoint(),{timeout:3000});assert.equal(browser.version(),'145.0.7632.6');
    context=await browser.newContext({viewport:{width:800,height:600},deviceScaleFactor:1});
    await context.route('**/*',route=>new URL(route.request().url()).origin===origin?route.continue():route.abort());
    const page=await context.newPage();page.setDefaultTimeout(2000);await page.goto(origin,{timeout:3000});
    const cdp=await context.newCDPSession(page);
    const {targetInfo}=await cdp.send('Target.getTargetInfo');
    const {frameTree}=await cdp.send('Page.getFrameTree');
    const {root:document}=await cdp.send('DOM.getDocument',{depth:0,pierce:false});
    const surface={id:frameTree.frame.id,generation:frameTree.frame.loaderId};
    const target={id:targetInfo.targetId,generation:crypto.randomUUID()};
    const endpoint=`ws://127.0.0.1:${port}/devtools/page/${target.id}`;
    // Trusted fixture setup selects exact identities; these selectors never reach the product.
    async function backend(id){const {nodeId}=await cdp.send('DOM.querySelector',{nodeId:document.nodeId,selector:`#${id}`});const {node}=await cdp.send('DOM.describeNode',{nodeId,depth:0,pierce:false});return node.backendNodeId;}
    const popupId=await backend('popup'),triggerId=await backend('trigger'),frameId=await backend('boundary');
    async function state(){return page.evaluate(()=>({active:document.activeElement?.id,scroll:[scrollX,scrollY],html:document.body.innerHTML,
      bounds:['popup','clip','overlay'].map(id=>{const n=document.getElementById(id);if(!n)return null;const r=n.getBoundingClientRect();return [r.x,r.y,r.width,r.height];})}));}
    function inputs(selection, fields=['layout_bounds','hit_region','visible_region','paint_bounds']){
      const session=crypto.randomUUID(),scope='b03-popup',mib=1048576,plugin={id:'web',version:'0.1.0'};
      const ctx={schema_version:'0.1.0',session_id:session,target,surfaces:[surface],scope_id:scope,projection:'design',fields,plugin,environment_revision:'b03-800x600-dpr1'};
      if(selection.selection==='rooted')selection={...selection,root:{session_id:session,target,surface,document_backend_id:document.backendNodeId,backend_node_id:selection.backend,sensitivity:'public'}};
      delete selection.backend;
      const connection={connection_version:'1.0.0',target,attach_deadline_ms:2000,
        session:{session_id:session,plugin,supported_versions:['0.1.0'],target,surfaces:[surface],allowed_scopes:[scope],capabilities:[{channel:'external_semantics',operation:'observe',status:'partial',reason:'owned-b03'}]},
        host_limits:{workers:2,worker_bytes:64*mib,publication_reserve:mib,bootstrap_bytes:mib,parent_bytes:32*mib,input_bytes:2*mib,ingress_bytes:512*1024,output_bytes:512*1024,request_output_bytes:2*mib,completion_groups:2,control_bytes:4096,cleanup_ms:1000,retained_domain_bytes:64*mib,retained_per_worker:15*mib,main_stack_bytes:8*mib,watchdog_stack_bytes:mib},
        provider:{backend:'web',selection,setup:{endpoint,cdp_session_id:null,surface,
          transport:{endpoint_bytes:1024,handshake_bytes:2048,read_buffer_bytes:64,write_buffer_bytes:64,write_buffer_max:32768,frame_bytes:8192,message_bytes:8192,outbound_bytes:16384},
          cdp:{max_request_bytes:16384,max_message_bytes:8192,max_metadata_bytes:256,max_results:1,result_bytes:8192,max_events:4,event_bytes:34000},
          collector:{max_nodes:16,max_methods:100,max_reply_bytes:8192,max_total_reply_bytes:65536,max_text_bytes:600,max_handle_bytes:256,max_ax_properties:32,io_read_bytes:16384,io_write_bytes:32768,io_work:2048}}}};
      const request={schema_version:'0.1.0',artifact:{kind:'request',data:{request_id:crypto.randomUUID(),clock_domain:'rebound',context:ctx,limits:{max_elements:32,max_depth:8,max_output_bytes:65536,deadline_ms:250},freshness_policy:'current_required',operation:{operation:'observe',channels:['external_semantics']}}}};
      return {connection,request};
    }
    async function observe(name,selection={selection:'rooted',backend:popupId,max_visited_nodes:256},change=()=>{}){
      const input=inputs(selection);change(input);const before=await state();
      const result=await call(['observe','--connection',await json(input.connection),'--request',await json(input.request),'--worker',worker,'--max-input-bytes','131072','--max-output-bytes','65536']);
      assert.deepEqual(await state(),before,`${name}: read-only invariance`);
      assert(!result.output.includes('FRAME_PRIVATE_CANARY'));
      assert([2,4].includes(result.code),`${name}: exit ${result.code} ${result.diagnostics}`);
      const doc=result.output?JSON.parse(result.output):null;
      const snapshot=doc?.artifact?.data?.result?.status==='observed'?doc.artifact.data.result.data:null;
      checks.push({name,exit:result.code,observed:!!snapshot,readonly:true});
      return {snapshot,doc,result,input};
    }
    function expected(s,id,clip,hit){
      assert(s,'positive Observe required');assert.equal(s.coverage.status,'partial');
      const n=s.nodes.find(n=>n.key.namespace==='web.dom'&&n.key.key===String(id));assert(n);
      near(rect(property(n,'layout_bounds').value.value),[120,120,180,80]);
      near(rect(fact(n,'intersection_rect_not_occlusion').value.value),clip);
      assert.equal(fact(n,'hit_sample_matches').value.value,hit);
      assert.equal(property(n,'hit_region').availability,hit?'known':'unknown');
      if(hit)near(rect(property(n,'hit_region').value.value),[210,160,0,0]);
      for(const f of ['visible_region','paint_bounds'])assert.equal(property(n,f).availability,'unknown');
      for(const e of n.extensions)assert(s.observations.some(o=>o.id===e.property.evidence.observation_id&&o.source_namespace==='web.dom'));
      assert.deepEqual(n.surface,s.context.surfaces[0]);return n;
    }
    await page.locator('#trigger').focus();
    const opened=await observe('open');const n=expected(opened.snapshot,popupId,[120,120,180,80],true);
    const saved=await save('observed.json',opened.result.output);
    const inspected=await call(['inspect','--snapshot',saved,'--ref',JSON.stringify(n.key),'--view','design','--max-input-bytes','131072','--max-output-bytes','65536','--json']);
    assert.equal(inspected.code,0);assert.deepEqual(JSON.parse(inspected.output).snapshot,opened.snapshot);checks.push({name:'public-inspect',exit:0});
    const query={schema_version:'0.2.0',artifact:{kind:'geometry_query',data:{id:'width',scope_id:opened.snapshot.context.scope_id,targets:[n.key],operation:'width',anchors:[{element:n.key,frame_kind:'layout_bounds',coordinate_space:property(n,'layout_bounds').value.value.coordinate_space,fraction:0,axis:'x'}],quantity_kind:'length',units:'css_px',applies_when:{platform:null,input_mode:null,text_scale:null}}}};
    const measured=await call(['measure','--snapshot',saved,'--query',await json(query),'--space',surface.id,'--max-input-bytes','131072','--max-output-bytes','65536','--json']);
    assert.equal(measured.code,0);assert.equal(JSON.parse(measured.output).artifact.data.result.measurement.value.value.amount,180);checks.push({name:'public-measure-width',amount:180,units:'css_px'});
    const linked=await observe('explicit-related-scope',{selection:'initial',ids:[{id:'trigger',sensitivity:'public'},{id:'popup',sensitivity:'public'}],max_visited_nodes:256});
    assert(linked.snapshot);const rel=linked.snapshot.relations;
    assert(rel.some(r=>r.kind==='controls'&&r.from.key===String(triggerId)&&r.to.key===String(popupId)));
    assert(rel.some(r=>r.kind==='anchored_to'&&r.from.key===String(popupId)&&r.to.key===String(triggerId)&&r.evidence.method==='fixture-data-anchor-attribute'));
    await page.evaluate(()=>document.getElementById('clip').style.width='100px');
    expected((await observe('clipped')).snapshot,popupId,[120,120,80,80],false);
    await page.evaluate(()=>document.getElementById('clip').style.width='10px');
    const outside=expected((await observe('fully-clipped')).snapshot,popupId,[0,0,0,0],false);
    assert.equal(fact(outside,'is_intersecting_not_visible').value.value,false);
    assert.equal(fact(outside,'intersection_ratio_not_visibility').value.value,0);
    await page.evaluate(()=>{document.getElementById('clip').style.width='220px';document.getElementById('overlay').style.display='block';});
    expected((await observe('covered')).snapshot,popupId,[120,120,180,80],false);
    await page.evaluate(()=>{document.getElementById('overlay').style.display='none';document.getElementById('popup').style.display='none';});
    const hidden=await observe('closed');assert(hidden.snapshot);const hn=hidden.snapshot.nodes.find(n=>n.key.key===String(popupId));assert.equal(property(hn,'layout_bounds').availability,'unknown');assert.equal(property(hn,'hit_region').availability,'unknown');assert.equal(hn.extensions.length,0);
    await page.evaluate(()=>{const old=document.getElementById('popup'),fresh=old.cloneNode(true);fresh.style.display='block';old.replaceWith(fresh);});
    assert.equal((await observe('stale-remount')).snapshot,null);
    const freshId=await backend('popup');assert.notEqual(freshId,popupId);
    expected((await observe('explicit-remount',{selection:'rooted',backend:freshId,max_visited_nodes:256})).snapshot,freshId,[120,120,180,80],true);
    assert.equal((await observe('frame-refused',{selection:'rooted',backend:frameId,max_visited_nodes:256})).snapshot,null);
    assert.equal((await observe('bounded-refusal',{selection:'rooted',backend:freshId,max_visited_nodes:1})).snapshot,null);
    assert.equal((await observe('wrong-document',{selection:'rooted',backend:freshId,max_visited_nodes:256},i=>{i.connection.provider.selection.root.document_backend_id++;})).snapshot,null);
    assert.equal(await fs.readFile(saved,'utf8'),opened.result.output,'saved original bytes unchanged');
    await cdp.detach();
    console.log(JSON.stringify({status:'passed',chromium:browser.version(),node:process.version,checks}));
  }finally{
    if(child&&child.exitCode===null&&child.signalCode===null)child.kill('SIGKILL');
    if(closed)await bounded(closed,2000);
    try{if(context)await bounded(context.close(),3000);}finally{
      try{if(browser)await bounded(browser.close(),3000);}finally{
        try{if(server)await bounded(server.close(),4000);}finally{
          if(fixture)await bounded(new Promise(resolve=>fixture.close(resolve)),3000);
          for(const file of files)await fs.unlink(file);
          await fs.rmdir(temp);await assert.rejects(fs.stat(temp),{code:'ENOENT'});
        }
      }
    }
  }
}
main().catch(error=>{console.error(error.stack);process.exitCode=1;});
