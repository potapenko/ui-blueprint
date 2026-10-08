// Q02 finite, caller-driven Web timing. No live side effects without --run-authorized.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const os = require('node:os');
const crypto = require('node:crypto');
const {spawn, execFileSync} = require('node:child_process');
const {prepare} = require('./fixture-host.cjs');
const {start} = require('../../../fixtures/web/server.cjs');
const oracle = require('../../../experiments/web/expected.json');
const MAX = 2 * 1048576;
function digest(file) { return crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex'); }
function stats(samples, key) {
  const values = samples.map(s => s[key]).filter(Number.isFinite).sort((a,b)=>a-b);
  // A missing duration is never a censored fast success. Keep its failed request
  // in the denominator and make the percentile explicitly unavailable.
  if (!values.length || values.length !== samples.length) return {count:samples.length, measured:values.length,p50:null,p95:null};
  const mid = Math.floor(values.length/2);
  return {count:values.length,measured:values.length,p50:values.length%2?values[mid]:(values[mid-1]+values[mid])/2,p95:values[Math.ceil(values.length*.95)-1]};
}
function known(node,field) {
  const p=node.properties.find(p=>p.field===field);
  assert(p && p.selection==='requested' && p.state.availability==='known', `required known ${field}`);
  return p.state.value;
}
function check(sample,context,kind,requestId,previous,raw) {
  assert.equal(sample.terminal,'Completed');assert.equal(sample.missing,0);assert.equal(sample.committed,1);
  assert.equal(sample.frames.length,1);
  const doc=JSON.parse(sample.frames[0].canonical); assert.equal(doc.artifact.kind,'channel_response');
  const response=doc.artifact.data;assert.equal(response.request_id,requestId);assert.equal(response.result.status,'observed');
  assert.deepEqual(response.target,context.target);assert.equal(response.session_id,context.session_id);
  const snapshot=response.result.data;assert.deepEqual(snapshot.context,context);
  assert.equal(snapshot.coverage.status,'partial'); // current adapter has explicit partial source coverage
  assert.deepEqual(snapshot.coverage.fields,context.fields);
  for(const node of snapshot.nodes)assert.deepEqual(node.properties.map(p=>p.field),context.fields,'no requested property omitted');
  for(const observation of snapshot.observations){
    assert.equal(observation.freshness,'current');assert.equal(observation.answer_source,'live');
    assert.equal(observation.time_unit,'milliseconds');
    assert(!previous.has(observation.id),'fresh observation identity');previous.add(observation.id);
  }
  if(kind==='documents'){
    const result=require('./fidelity.cjs').compare(raw,snapshot);
    assert.deepEqual(result,{nodes:97,documents:2,fields:1102,boxes:19});
    for(const id of ['left','child-button']){
      const node=snapshot.nodes.find(n=>n.extensions.some((e,j)=>e.name.endsWith('.name')&&e.property.state.value?.value==='id'&&n.extensions[j+1]?.property.state.value?.value===id));
      assert(node,'literal fixture control present');const rect=known(node,'layout_bounds').value.shape.value;
      ['x','y','width','height'].forEach((p,i)=>assert(Math.abs(rect[p]-oracle.initial[id].bounds[i])<=oracle.tolerance_css_px));
    }
    return {snapshot,bytes:Buffer.byteLength(sample.frames[0].canonical),nodes:snapshot.nodes.length,coverage:result,
      source_intervals_ms:snapshot.observations.map(o=>({namespace:o.source_namespace,ms:o.end-o.start}))};
  }
  const dom=snapshot.nodes.filter(n=>n.key.namespace==='web.dom');assert.equal(dom.length,1);
  assert.deepEqual(dom[0].surface,context.surfaces[0]);
  if(kind==='geometry'){
    const geometry=known(dom[0],'layout_bounds');assert.equal(geometry.type,'geometry');
    assert.equal(geometry.value.coordinate_space.units,'css_px');
    const rect=geometry.value.shape.value;
    ['x','y','width','height'].forEach((p,i)=>assert(Math.abs(rect[p]-oracle.initial.left.bounds[i])<=oracle.tolerance_css_px));
  }else{
    const ax=snapshot.nodes.filter(n=>n.key.namespace==='web.ax');assert.equal(ax.length,1);
    assert.equal(known(ax[0],'role').value,'button');assert.equal(known(ax[0],'accessibility_name').value,'Apply');
    assert.equal(known(ax[0],'invalid').value,false);
    const focusable=ax[0].extensions.find(e=>e.namespace==='web.ax'&&e.name==='focusable');
    assert(focusable,'required raw focusable retained');
    assert.equal(focusable.property.selection,'requested');assert.equal(focusable.property.field,'value');
    assert.deepEqual(focusable.property.state,{availability:'known',value:{type:'flag',value:true}});
    assert.equal(focusable.property.evidence.source_namespace,'web.ax');
    assert.equal(focusable.property.evidence.provenance,'reported');
    assert(snapshot.observations.some(o=>o.id===focusable.property.evidence.observation_id&&o.source_namespace==='web.ax'));
  }
  return {snapshot,bytes:Buffer.byteLength(sample.frames[0].canonical),nodes:snapshot.nodes.length,
    source_intervals_ms:snapshot.observations.map(o=>({namespace:o.source_namespace,ms:o.end-o.start}))};
}
function client(executable,config) {
  const child=spawn(executable,['--ignored','--exact','requested_series','--nocapture','--test-threads=1'],
    {env:{...process.env,UIB_Q02_ALLOW:'1',UIB_Q02_CONFIG:config},stdio:['pipe','pipe','pipe']});
  let buffer='',stderrBytes=0, pending=[],queue=[],exit=null;
  const deliver=value=>pending.length?pending.shift()(value):queue.push(value);
  child.stdout.on('data',chunk=>{
    buffer+=chunk;assert(buffer.length<=MAX,'bounded harness output');
    let pos;while((pos=buffer.indexOf('\n'))>=0){const line=buffer.slice(0,pos);buffer=buffer.slice(pos+1);
      const marker=line.indexOf('@Q02 ');if(marker>=0)deliver(JSON.parse(line.slice(marker+5)));}
  });
  child.stderr.on('data',chunk=>{stderrBytes+=chunk.length;if(stderrBytes>16384)child.kill('SIGTERM');});
  child.on('exit',(code,signal)=>{exit={code,signal};deliver({kind:'exit',...exit});});
  const next=()=>new Promise((resolve,reject)=>{
    if(queue.length)return resolve(queue.shift());if(exit)return reject(new Error('harness exited'));
    const timer=setTimeout(()=>{pending=pending.filter(fn=>fn!==done);reject(new Error('harness deadline'));},5000);
    const done=value=>{clearTimeout(timer);resolve(value);};pending.push(done);
  });
  return {child,next,send:value=>child.stdin.write(JSON.stringify(value)+'\n'),async close(){
    child.stdin.end();let closed;try{closed=await next();assert.equal(closed.kind,'closed');assert.equal(closed.cleanup_confirmed,true);assert.equal(closed.error,null);
      const result=await next();assert.equal(result.kind,'exit');assert.equal(result.code,0);
    }catch(error){if(!exit)child.kill('SIGTERM');throw error;}
    return closed;
  }};
}
function setup(binding,documents=false){const value={endpoint:binding.endpoint,cdp_session_id:null,surface:binding.surface,
  transport:{endpoint_bytes:1024,handshake_bytes:2048,read_buffer_bytes:64,write_buffer_bytes:64,write_buffer_max:32768,frame_bytes:8192,message_bytes:8192,outbound_bytes:16384},
  cdp:{max_request_bytes:16384,max_message_bytes:8192,max_metadata_bytes:256,max_results:1,result_bytes:8192,max_events:4,event_bytes:34000},
  collector:{max_nodes:16,max_methods:100,max_reply_bytes:8192,max_total_reply_bytes:65536,max_text_bytes:600,max_handle_bytes:256,max_ax_properties:32,io_read_bytes:16384,io_write_bytes:32768,io_work:2048}};
  if(documents){Object.assign(value.transport,{frame_bytes:32768,message_bytes:32768});
    Object.assign(value.cdp,{max_message_bytes:32768,result_bytes:32768});
    Object.assign(value.collector,{max_nodes:128,max_reply_bytes:32768,max_total_reply_bytes:262144,max_text_bytes:16384,io_read_bytes:65536,io_work:8192});}
  return value;
}
async function documentBindings(session,frames){
  const result=[];
  for(const surface of frames){
    // Metadata only. No DOMSnapshot/tree/layout capture before the timed response.
    const world=await session.send('Page.createIsolatedWorld',{frameId:surface.id,worldName:'q02-document-identity',grantUniveralAccess:false});
    const remote=await session.send('Runtime.evaluate',{expression:'document',contextId:world.executionContextId});
    assert(remote.result.objectId && remote.result.subtype==='node');
    try{const {node}=await session.send('DOM.describeNode',{objectId:remote.result.objectId,depth:0});
      assert.equal(node.nodeType,9);assert(node.backendNodeId>0);
      result.push({surface,document_backend_id:node.backendNodeId,sensitivity:'public'});
    }finally{await session.send('Runtime.releaseObject',{objectId:remote.result.objectId});}
  }
  return result;
}
async function run(){
  assert(process.argv.includes('--run-authorized') && process.env.UIB_Q02_ALLOW==='1','explicit timed resource activation');
  const functional=process.env.UIB_Q02_FUNCTIONAL_PIN;assert(/^[a-f0-9]{40}$/.test(functional||''),'reviewed functional source pin');
  const preflight=process.argv.includes('--preflight');
  let accepted;
  if(!preflight){
    const proof=process.env.UIB_Q02_PREFLIGHT;assert(path.isAbsolute(proof||''),'saved quality preflight required before series');
    accepted=JSON.parse(fs.readFileSync(proof));assert.equal(accepted.functional_pin,functional);
    assert(!accepted.failure&&!accepted.cleanup_failure,'successful preflight lifecycle required');
  }
  const executable=process.env.UIB_Q02_EXECUTABLE;assert(path.isAbsolute(executable||''));
  assert.equal(digest(executable),process.env.UIB_Q02_EXECUTABLE_SHA256);
  if(accepted)assert.equal(accepted.executable_sha256,digest(executable),'preflight and series use identical saved products');
  const output=process.env.UIB_Q02_OUTPUT;assert(path.isAbsolute(output||''));
  assert(path.resolve(output).startsWith(fs.realpathSync(os.tmpdir())+path.sep));fs.mkdirSync(output,{mode:0o700});
  const fixtureSetup=prepare(),report={functional_pin:functional,executable_sha256:digest(executable),mode:preflight?'quality_preflight':'timed_series',samples:[],comparability:{},
    unavailable:['Documents cohorts await separate privacy acceptance; single-control acceptance does not release that scope.',
      'Separate API/transport/Rust normalization/format CPU, syscall counts, worker allocation high-water/cache and model tokens unavailable.'],closures:[],
    environment:{node:process.version,arch:os.arch(),release:os.release(),cpus:os.cpus().length,cpu:os.cpus()[0].model,memory:os.totalmem(),load_start:os.loadavg()},
    retention:'Q02/root review consumer; remove run-owned nonimages after consumption; no screenshots created'};
  let fixture;
  async function series(kind,count,label){
    let server,browser,session,driver,raw;const totalBegin=performance.now();
    const full=kind==='documents';
    try{
      server=await fixtureSetup.chromium.launchServer({headless:true,timeout:8000,args:['--remote-debugging-port=0','--remote-debugging-address=127.0.0.1']});
      browser=await fixtureSetup.chromium.connect(server.wsEndpoint(),{timeout:3000});
      assert.equal(browser.version(),'145.0.7632.6');
      const profile=server.process().spawnargs.find(v=>v.startsWith('--user-data-dir=')).split('=').slice(1).join('=');
      const port=Number(fs.readFileSync(path.join(profile,'DevToolsActivePort'),'utf8').split('\n')[0]);
      const context=await browser.newContext({viewport:{width:800,height:600},deviceScaleFactor:1});
      await context.route('**/*',route=>new URL(route.request().url()).origin===fixture.url?route.continue():route.abort());
      const page=await context.newPage();await page.goto(fixture.url+'/?generation=1',{timeout:3000});
      await page.waitForFunction(()=>!!window.f01,{},{timeout:3000});
      const attachBegin=performance.now(); // includes all required discovery/metadata/config, not only worker startup
      session=await context.newCDPSession(page);
      const {targetInfo}=await session.send('Target.getTargetInfo');const {frameTree}=await session.send('Page.getFrameTree');
      const binding={endpoint:`ws://127.0.0.1:${port}/devtools/page/${targetInfo.targetId}`,
        target:{id:targetInfo.targetId,generation:crypto.randomUUID()},surface:{id:frameTree.frame.id,generation:frameTree.frame.loaderId}};
      const surfaces=full?[frameTree,...(frameTree.childFrames||[])].map(t=>({id:t.frame.id,generation:t.frame.loaderId})):[binding.surface];
      const documents=full?await documentBindings(session,surfaces):null;
      if(full)assert.equal(documents.length,2);
      const before=await page.evaluate(()=>({active:document.activeElement?.id,scroll:[scrollX,scrollY],state:window.f01.checkpoint(),viewport:[innerWidth,innerHeight,devicePixelRatio]}));
      assert.deepEqual(before.viewport,[800,600,1]);
      const contextData={schema_version:'0.1.0',session_id:'q02-'+crypto.randomUUID(),target:binding.target,surfaces,scope_id:full?'f01-documents':'f01-left',
        projection:full?'design':'interaction',fields:full?['value','layout_bounds']:kind==='geometry'?['layout_bounds']:['role','accessibility_name','enabled','focused','invalid'],
        plugin:{id:'web',version:'0.1.0'},environment_revision:'f01-800x600-dpr1'};
      const descriptor={schema_version:'0.1.0',artifact:{kind:'session',data:{session_id:contextData.session_id,plugin:contextData.plugin,supported_versions:['0.1.0'],
        target:binding.target,surfaces,allowed_scopes:[contextData.scope_id],capabilities:[{channel:'external_semantics',operation:'observe',status:'partial',reason:'bounded-f01'}]}}};
      const config=path.join(output,`${label}-${kind}-config.json`);fs.writeFileSync(config,JSON.stringify({descriptor,provider:{backend:'web',setup:setup(binding,full)}}),{flag:'wx',mode:0o600});
      driver=client(executable,config);assert.equal((await driver.next()).kind,'attached');
      const ids=new Set();let selection=full?{selection:'documents',documents,max_visited_nodes:128}:{selection:'initial',ids:[{id:'left',sensitivity:'public'}],max_visited_nodes:256};
      for(let i=0;i<count;i++){
        const requestId=`${label}-${kind}-${i}`,request={schema_version:'0.1.0',artifact:{kind:'request',data:{request_id:requestId,clock_domain:'rebound',context:contextData,
          limits:full?{max_elements:128,max_depth:16,max_output_bytes:524288,deadline_ms:2000}:{max_elements:32,max_depth:8,max_output_bytes:65536,deadline_ms:250},freshness_policy:'current_required',operation:{operation:'observe',channels:['external_semantics']}}}};
        const record={kind,cohort:label,index:i,status:'failed'},begin=performance.now();
        try{
          driver.send({request,selection});const sample=await driver.next();record.outer_ms=performance.now()-begin;
          assert.equal(sample.kind,'sample');record.request_ms=sample.request_ms;record.domain_usage=sample.domain_usage;
          if(i===0){record.attach_first_ms=performance.now()-attachBegin;record.process_cold_ms=performance.now()-totalBegin;}
          fs.writeFileSync(path.join(output,requestId+'.json'),JSON.stringify(sample),{flag:'wx',mode:0o600});
          if(i===0 && (full||preflight)){
            raw=await session.send('DOMSnapshot.captureSnapshot',{computedStyles:[],includeDOMRects:true});
            const shape={documents:raw.documents.length,nodes:raw.documents.reduce((n,d)=>n+d.nodes.nodeName.length,0)};
            assert.deepEqual(shape,{documents:2,nodes:97});report.fixture_shape=shape;
            fs.writeFileSync(path.join(output,requestId+'-raw-oracle.json'),JSON.stringify(raw),{flag:'wx',mode:0o600});
          }
          const checked=check(sample,contextData,kind,requestId,ids,raw);Object.assign(record,checked,{snapshot:undefined,status:'valid_partial'});
          if(preflight){
            const version=await session.send('Browser.getVersion');assert.equal(version.protocolVersion,'1.3');report.environment.protocol=version;
            if(kind==='semantic'){
              const {root}=await session.send('DOM.getDocument',{depth:0});
              const {nodeId}=await session.send('DOM.querySelector',{nodeId:root.nodeId,selector:'#left'});
              const {node}=await session.send('DOM.describeNode',{nodeId,depth:0});
              assert.equal(checked.snapshot.nodes.find(n=>n.key.namespace==='web.dom').key.key,String(node.backendNodeId));
              const ax=await session.send('Accessibility.getPartialAXTree',{backendNodeId:node.backendNodeId,fetchRelatives:false});
              assert.equal(ax.nodes[0].properties.find(p=>p.name==='focusable').value.value,true);
              fs.writeFileSync(path.join(output,'semantic-raw-ax.json'),JSON.stringify(ax),{flag:'wx',mode:0o600});
            }
          }
          if(i===0&&!full){const s=checked.snapshot,n=s.nodes.find(n=>n.key.namespace==='web.dom'),o=n.properties[0].evidence;
            selection={selection:'references',nodes:[{sensitivity:'public',reference:{session_id:s.context.session_id,target:s.context.target,surface:n.surface,key:n.key,snapshot_id:s.id,observation_id:o.observation_id}}]};}
          if(i===0 && (label==='reused-session'||preflight)){
            // Separate explicit fixture stimulus, outside every measured warm sample.
            // Prove that a reused ref reads new requested data, then restore baseline.
            const saved=await page.evaluate(()=>{const n=document.getElementById('left');return {style:n.getAttribute('style'),label:n.getAttribute('aria-label')};});
            try{
              await page.evaluate(kind=>{const n=document.getElementById('left');if(kind!=='semantic')n.style.width='121px';else n.setAttribute('aria-label','Q02 fresh control');},kind);
              const changed=structuredClone(request);changed.artifact.data.request_id=requestId+'-freshness';
              driver.send({request:changed,selection});const fresh=await driver.next();assert.equal(fresh.kind,'sample');assert.equal(fresh.terminal,'Completed');assert.equal(fresh.missing,0);
              const response=JSON.parse(fresh.frames[0].canonical).artifact.data;assert.equal(response.request_id,changed.artifact.data.request_id);
              assert.equal(response.result.status,'observed');assert.deepEqual(response.result.data.context,contextData);
              const node=response.result.data.nodes.find(n=>n.key.namespace===(kind==='semantic'?'web.ax':'web.dom'));
              if(full){
                const changedRaw=await session.send('DOMSnapshot.captureSnapshot',{computedStyles:[],includeDOMRects:true});
                require('./fidelity.cjs').compare(changedRaw,response.result.data);
                const left=response.result.data.nodes.find(n=>n.extensions.some((e,j)=>e.name.endsWith('.name')&&e.property.state.value?.value==='id'&&n.extensions[j+1]?.property.state.value?.value==='left'));
                assert.equal(known(left,'layout_bounds').value.shape.value.width,121);
              }else if(kind==='geometry')assert.equal(known(node,'layout_bounds').value.shape.value.width,121);
              else assert.equal(known(node,'accessibility_name').value,'Q02 fresh control');
              fs.writeFileSync(path.join(output,requestId+'-freshness.json'),JSON.stringify(fresh),{flag:'wx',mode:0o600});
            }finally{await page.evaluate(saved=>{const n=document.getElementById('left');for(const [key,value] of [['style',saved.style],['aria-label',saved.label]]){if(value===null)n.removeAttribute(key);else n.setAttribute(key,value);}},saved);}
            const restored=structuredClone(request);restored.artifact.data.request_id=requestId+'-restored';
            driver.send({request:restored,selection});const fresh=await driver.next();check(fresh,contextData,kind,restored.artifact.data.request_id,ids,raw);
            fs.writeFileSync(path.join(output,requestId+'-restored.json'),JSON.stringify(fresh),{flag:'wx',mode:0o600});
            record.freshness_challenge='changed_and_restored_on_same_attachment_and_ref';
          }
          if(preflight)report.comparability[kind]={comparable:true,gaps:[],raw_facts:full?checked.coverage:null};
        }catch(error){record.status='failed';record.outer_ms??=performance.now()-begin;record.failure=String(error.message).slice(0,200);}
        report.samples.push(record);
        // A failure remains a sample. Do not blindly retry a broken session.
        if(record.status==='failed'){for(let j=i+1;j<count;j++)report.samples.push({kind,cohort:label,index:j,status:'not_run_after_failure'});break;}
      }
      assert.deepEqual(await page.evaluate(()=>({active:document.activeElement?.id,scroll:[scrollX,scrollY],state:window.f01.checkpoint(),viewport:[innerWidth,innerHeight,devicePixelRatio]})),before,'read-only state invariance');
      report.closures.push({kind,cohort:label,...await driver.close()});driver=null;
    }catch(error){
      if(!report.samples.some(s=>s.kind===kind&&s.cohort===label))report.samples.push({kind,cohort:label,index:0,status:'failed',process_cold_ms:performance.now()-totalBegin,failure:`setup: ${String(error.message).slice(0,160)}`});
      throw error;
    }finally{if(driver){try{report.closures.push({kind,cohort:label,...await driver.close()});}catch{report.cleanup_failure=true;}}if(session)await session.detach().catch(()=>{});if(browser)await browser.close();if(server)await server.close();}
  }
  try{
    fixture=await start();
    // Q01 88d0920 accepts these independent scopes on9d715ee; Documents remains rejected.
    for(const kind of ['semantic','geometry']){
      if(preflight){await series(kind,1,'preflight');continue;}
      if(!accepted.comparability[kind]?.comparable){report.comparability[kind]=accepted.comparability[kind];continue;}
      assert(accepted.samples.some(s=>s.kind===kind&&s.status==='valid_partial'&&s.freshness_challenge),'preflight quality and freshness required');
      // Only Documents is the frozen whole-fixture cold gate.
      for(let i=0;i<20;i++)await series(kind,1,`${kind==='documents'?'cold-full':'cold-control'}-${i}`);
      await series(kind,101,'reused-session'); // first retained separately; next 100 are warm
    }
  }catch(error){report.failure=String(error.message).slice(0,300);}
  finally{if(fixture)await fixture.close();report.environment.load_end=os.loadavg();
    report.summary={};for(const kind of ['semantic','geometry','documents']){
      const planned=report.samples.filter(s=>s.kind===kind&&s.cohort==='reused-session'&&s.index>0);
      const warm=planned.filter(s=>s.status!=='not_run_after_failure');
      const cold=report.samples.filter(s=>s.kind===kind&&s.cohort.startsWith('cold-'));
      report.summary[kind]={warm:stats(warm,'outer_ms'),cold_total:stats(cold,'process_cold_ms'),cold_attach_first:stats(cold,'attach_first_ms'),not_run:100-warm.length,
        quality_failures:[...warm,...cold].filter(s=>s.status!=='valid_partial').length,
        gates:kind==='documents'?{attach_ms:50,process_cold_ms:500}:{warm_ms:20},
        supplemental:kind==='documents'?'warm full capture':'cold single control'};
    }
    fs.writeFileSync(path.join(output,'report.json'),JSON.stringify(report,null,2)+'\n',{flag:'wx',mode:0o600});
  }
  console.log(JSON.stringify({output,summary:report.summary,failure:report.failure,cleanup_failure:report.cleanup_failure}));
  if(report.failure||report.cleanup_failure||report.samples.some(s=>s.status!=='valid_partial'))process.exitCode=1;
}
if(require.main===module && process.argv.includes('--check')){
  assert.deepEqual(stats(Array.from({length:20},(_,i)=>({ms:i+1})),'ms'),{count:20,measured:20,p50:10.5,p95:19});
  assert.equal(stats([{ms:1},{}],'ms').p95,null);assert.equal(stats([{ms:1},{}],'ms').count,2);
  const context={target:{id:'own',generation:'1'},session_id:'session',surfaces:[{id:'surface',generation:'1'}],fields:['layout_bounds']};
  const snapshot={context,coverage:{status:'partial',fields:context.fields},observations:[{id:'fresh',freshness:'current',answer_source:'live',time_unit:'milliseconds',source_namespace:'web.dom',start:5,end:6}],
    nodes:[{key:{namespace:'web.dom',key:'left'},surface:context.surfaces[0],properties:[{field:'layout_bounds',selection:'requested',state:{availability:'known',value:{type:'geometry',value:{coordinate_space:{units:'css_px'},shape:{value:{x:40,y:60,width:120,height:40}}}}}}]}]};
  const response={request_id:'r',target:context.target,session_id:context.session_id,result:{status:'observed',data:snapshot}};
  const sample=()=>({terminal:'Completed',missing:0,committed:1,frames:[{canonical:JSON.stringify({artifact:{kind:'channel_response',data:response}})}]});
  assert.equal(check(sample(),context,'geometry','r',new Set()).source_intervals_ms[0].ms,1);
  assert.throws(()=>check(sample(),context,'geometry','wrong-request',new Set()));
  assert.throws(()=>check(sample(),context,'geometry','r',new Set(['fresh'])));
  snapshot.nodes[0].properties[0].state.availability='unknown';assert.throws(()=>check(sample(),context,'geometry','r',new Set()));
  snapshot.nodes[0].properties=[];assert.throws(()=>check(sample(),context,'geometry','r',new Set()));
  console.log('Q02 quantiles and missing-sample handling pass; no runtime launched');
}else if(require.main===module)run().catch(error=>{console.error(error.message);process.exitCode=1;});
module.exports={stats,check,client};
