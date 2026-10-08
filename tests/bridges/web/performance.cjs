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
function check(sample,context,kind,requestId,previous) {
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
function setup(binding){return {endpoint:binding.endpoint,cdp_session_id:null,surface:binding.surface,
  transport:{endpoint_bytes:1024,handshake_bytes:2048,read_buffer_bytes:64,write_buffer_bytes:64,write_buffer_max:32768,frame_bytes:8192,message_bytes:8192,outbound_bytes:16384},
  cdp:{max_request_bytes:16384,max_message_bytes:8192,max_metadata_bytes:256,max_results:1,result_bytes:8192,max_events:4,event_bytes:34000},
  collector:{max_nodes:16,max_methods:100,max_reply_bytes:8192,max_total_reply_bytes:65536,max_text_bytes:600,max_handle_bytes:256,max_ax_properties:32,io_read_bytes:16384,io_write_bytes:32768,io_work:2048}};}
async function run(){
  assert(process.argv.includes('--run-authorized') && process.env.UIB_Q02_ALLOW==='1','explicit timed resource activation');
  const functional=process.env.UIB_Q02_FUNCTIONAL_PIN;assert(/^[a-f0-9]{40}$/.test(functional||''),'reviewed functional source pin');
  const executable=process.env.UIB_Q02_EXECUTABLE;assert(path.isAbsolute(executable||''));
  assert.equal(digest(executable),process.env.UIB_Q02_EXECUTABLE_SHA256);
  const output=process.env.UIB_Q02_OUTPUT;assert(path.isAbsolute(output||''));
  assert(path.resolve(output).startsWith(fs.realpathSync(os.tmpdir())+path.sep));fs.mkdirSync(output,{mode:0o700});
  const fixtureSetup=prepare(),report={functional_pin:functional,executable_sha256:digest(executable),samples:[],
    unavailable:['Web full-fixture cold: current rooted collector refuses iframe; 2-document/97-node baseline cannot be substituted by single-control cold.',
      'Separate API/transport/Rust normalization/format CPU, syscall counts, worker allocation high-water/cache and model tokens unavailable.'],
    environment:{node:process.version,arch:os.arch(),release:os.release(),cpus:os.cpus().length,cpu:os.cpus()[0].model,memory:os.totalmem(),load_start:os.loadavg()},
    retention:'Q02/root review consumer; remove run-owned nonimages after consumption; no screenshots created'};
  let fixture;
  async function series(kind,count,label){
    let server,browser,session,driver;const totalBegin=performance.now();
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
      session=await context.newCDPSession(page);
      const {targetInfo}=await session.send('Target.getTargetInfo');const {frameTree}=await session.send('Page.getFrameTree');
      const binding={endpoint:`ws://127.0.0.1:${port}/devtools/page/${targetInfo.targetId}`,
        target:{id:targetInfo.targetId,generation:crypto.randomUUID()},surface:{id:frameTree.frame.id,generation:frameTree.frame.loaderId}};
      const before=await page.evaluate(()=>({active:document.activeElement?.id,scroll:[scrollX,scrollY],state:window.f01.checkpoint()}));
      const contextData={schema_version:'0.1.0',session_id:'q02-'+crypto.randomUUID(),target:binding.target,surfaces:[binding.surface],scope_id:'f01-left',
        projection:'interaction',fields:kind==='geometry'?['layout_bounds']:['role','accessibility_name','enabled','focused','invalid'],
        plugin:{id:'web',version:'0.1.0'},environment_revision:'f01-800x600-dpr1'};
      const descriptor={schema_version:'0.1.0',artifact:{kind:'session',data:{session_id:contextData.session_id,plugin:contextData.plugin,supported_versions:['0.1.0'],
        target:binding.target,surfaces:[binding.surface],allowed_scopes:['f01-left'],capabilities:[{channel:'external_semantics',operation:'observe',status:'partial',reason:'bounded-f01'}]}}};
      const config=path.join(output,`${label}-${kind}-config.json`);fs.writeFileSync(config,JSON.stringify({descriptor,provider:{backend:'web',setup:setup(binding)}}),{flag:'wx',mode:0o600});
      const attachBegin=performance.now();driver=client(executable,config);assert.equal((await driver.next()).kind,'attached');
      const ids=new Set();let selection={selection:'initial',ids:[{id:'left',sensitivity:'public'}],max_visited_nodes:256};
      for(let i=0;i<count;i++){
        const requestId=`${label}-${kind}-${i}`,request={schema_version:'0.1.0',artifact:{kind:'request',data:{request_id:requestId,clock_domain:'rebound',context:contextData,
          limits:{max_elements:32,max_depth:8,max_output_bytes:65536,deadline_ms:250},freshness_policy:'current_required',operation:{operation:'observe',channels:['external_semantics']}}}};
        const record={kind,cohort:label,index:i,status:'failed'},begin=performance.now();
        try{
          driver.send({request,selection});const sample=await driver.next();record.outer_ms=performance.now()-begin;
          assert.equal(sample.kind,'sample');record.request_ms=sample.request_ms;record.domain_usage=sample.domain_usage;
          if(i===0){record.attach_first_ms=performance.now()-attachBegin;record.process_cold_ms=performance.now()-totalBegin;}
          fs.writeFileSync(path.join(output,requestId+'.json'),JSON.stringify(sample),{flag:'wx',mode:0o600});
          const checked=check(sample,contextData,kind,requestId,ids);Object.assign(record,checked,{snapshot:undefined,status:'valid_partial'});
          if(i===0){const s=checked.snapshot,n=s.nodes.find(n=>n.key.namespace==='web.dom'),o=n.properties[0].evidence;
            selection={selection:'references',nodes:[{sensitivity:'public',reference:{session_id:s.context.session_id,target:s.context.target,surface:n.surface,key:n.key,snapshot_id:s.id,observation_id:o.observation_id}}]};}
        }catch(error){record.outer_ms=performance.now()-begin;record.failure=String(error.message).slice(0,200);}
        report.samples.push(record);
        // A failure remains a sample. Do not blindly retry a broken session.
        if(record.status==='failed'){for(let j=i+1;j<count;j++)report.samples.push({kind,cohort:label,index:j,status:'not_run_after_failure'});break;}
      }
      assert.deepEqual(await page.evaluate(()=>({active:document.activeElement?.id,scroll:[scrollX,scrollY],state:window.f01.checkpoint()})),before,'read-only state invariance');
      await driver.close();driver=null;
    }finally{if(driver){try{await driver.close();}catch{report.cleanup_failure=true;}}if(session)await session.detach().catch(()=>{});if(browser)await browser.close();if(server)await server.close();}
  }
  try{
    fixture=await start();
    for(const kind of ['semantic','geometry']){
      // Cold single-control is supplemental, never the frozen whole-fixture gate.
      for(let i=0;i<20;i++)await series(kind,1,`cold-control-${i}`);
      await series(kind,101,'reused-session'); // first retained separately; next 100 are warm
    }
  }catch(error){report.failure=String(error.message).slice(0,300);}
  finally{if(fixture)await fixture.close();report.environment.load_end=os.loadavg();
    report.summary={};for(const kind of ['semantic','geometry']){
      const planned=report.samples.filter(s=>s.kind===kind&&s.cohort==='reused-session'&&s.index>0);
      const warm=planned.filter(s=>s.status!=='not_run_after_failure');
      report.summary[kind]={warm:stats(warm,'outer_ms'),not_run:100-warm.length,quality_failures:warm.filter(s=>s.status!=='valid_partial').length,threshold_ms:20};
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
