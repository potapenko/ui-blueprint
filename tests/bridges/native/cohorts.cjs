// Q02 registered76-node Native workload; finite explicit requests, no UI setup.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),os=require('node:os'),crypto=require('node:crypto');
const {spawn,spawnSync}=require('node:child_process'),{isDeepStrictEqual}=require('node:util');
const {client}=require('../web/performance.cjs'),{sourceFidelity,sourceFidelityWithBoundary}=require('./performance.cjs');
const profile=require('./acquisition/profile.json');
const fields=['role','accessibility_name','description','value','placeholder','enabled','focused','actions','accessibility_bounds'];
const read=p=>JSON.parse(fs.readFileSync(p)),hash=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const save=(p,v)=>fs.writeFileSync(p,JSON.stringify(v)+'\n',{flag:'wx',mode:0o600});
const now=()=>Number(process.hrtime.bigint())/1e6;
function stats(samples,key){const a=samples.map(s=>s[key]).filter(Number.isFinite).sort((a,b)=>a-b);return {count:samples.length,measured:a.length,p50:a.length>0&&a.length===samples.length?(a.length%2?a[a.length>>1]:(a[a.length/2-1]+a[a.length/2])/2):null,p95:a.length>0&&a.length===samples.length?a[Math.ceil(a.length*.95)-1]:null,max:a.length?Math.max(...a):null};}
function directClient(binary,args){
 const child=spawn(binary,args,{stdio:['pipe','pipe','pipe']});let buffer='',queue=[],pending=[],exit=null,stderr=0;
 const emit=v=>pending.length?pending.shift()(v):queue.push(v);child.stdout.setEncoding('utf8');
 child.stdout.on('data',v=>{buffer+=v;assert(buffer.length<=1048576);let i;while((i=buffer.indexOf('\n'))>=0){const line=buffer.slice(0,i);buffer=buffer.slice(i+1);assert(line.startsWith('@Q02 '));emit(JSON.parse(line.slice(5)));}});
 child.stderr.on('data',v=>{stderr+=v.length;if(stderr>16384)child.kill('SIGTERM');});child.on('exit',(code,signal)=>{exit={kind:'exit',code,signal};emit(exit);});
 const next=()=>new Promise((resolve,reject)=>{if(queue.length)return resolve(queue.shift());if(exit)return reject(Error('baseline exited'));const timer=setTimeout(()=>{pending=pending.filter(f=>f!==done);if(!exit)child.kill('SIGTERM');reject(Error('baseline deadline'));},5000);const done=v=>{clearTimeout(timer);resolve(v);};pending.push(done);});
 return {child,next,send:v=>child.stdin.write(JSON.stringify(v)+'\n'),async close(){child.stdin.end();const c=await next();assert.equal(c.kind,'closed');assert.equal(c.cleanup_confirmed,true);const e=await next();assert.equal(e.kind,'exit');assert.equal(e.code,0);return c;}};
}
async function run(){
 assert(process.argv.includes('--run-authorized'));assert.equal(process.env.UIB_Q02_ALLOW,'1');
 const config=read(process.env.UIB_Q02_RUN_CONFIG),{phase,manifest:manifestPath,fixture,witness,helper,executable,baseline,output}=config;
 const method=config.quality_method||'fixed76@1';assert(['fixed76@1','N05-AX-BOUNDARY@2'].includes(method));if(method==='N05-AX-BOUNDARY@2')assert.equal(phase,'candidate');
 assert(['baseline','candidate'].includes(phase));assert(output.startsWith(fs.realpathSync(os.tmpdir())+'/'));fs.mkdirSync(output,{mode:0o700});
 for(const name of ['fixture','witness','helper','executable','baseline'])assert.equal(hash(config[name]),config.hashes[name]);
 assert.equal(config.pin,'e64154349bc93e9c7a4a91bab7698cef84b8eb7a');
 if(phase==='candidate'){assert.equal(config.baseline_reports.length,2);for(const file of config.baseline_reports){const r=read(file);assert.equal(r.phase,'baseline');assert.equal(r.cold.length,20);assert.equal(r.warm.length,100);assert.equal(r.workload,'UIB.D06-NATIVE-REQUEST@1');}}
 const manifest=read(manifestPath);assert.equal(manifest.collection_mode,'explicit_request_only');assert.equal(manifest.state.activations,1);for(const [key,value]of Object.entries({expanded:true,wide:false,name:'',secret_present:false,checked:false,applied:'none',popup:false,scroll_end:false,focus:'name'}))assert.deepEqual(manifest.state[key],value);
 const report={phase,pin:config.pin,registration:config.registration,hashes:config.hashes,manifest_sha256:hash(manifestPath),build_profile:config.build_profile||'Swift-Onone/Rust-release exploratory',baseline_reports:(config.baseline_reports||[]).map(file=>({file,sha256:hash(file)})),mode:manifest.probe_enabled?'on':'off',workload:method==='fixed76@1'?'UIB.D06-NATIVE-REQUEST@1':'UIB.D06-NATIVE-REQUEST@2',quality_method:method,cold:[],warm:[],initial:[],failures:[],closure:[],environment:{platform:os.platform(),arch:os.arch(),release:os.release(),cpus:os.cpus().length,memory:os.totalmem(),node:process.version,load_before:os.loadavg()},retention:'Q01/root review owns nonimages; all images and containing directories never agent-cleaned'};
 function witnessRead(label){const r=spawnSync(witness,[manifestPath,fixture,path.join(__dirname,'acquisition/profile.json'),'facts'],{timeout:5000,maxBuffer:1048576});assert.equal(r.status,0);const v=JSON.parse(r.stdout);save(path.join(output,label+'.json'),v);assert.equal(v.status,'validated');assert.deepEqual(v.before,v.after);assert.equal(v.before.app_active,true);assert.equal(v.before.frontmost_pid,manifest.pid);assert.deepEqual(v.before.window_frame,{X:40,Y:90,Width:550,Height:525});assert.equal(v.facts.nodes.length,76);assert.equal(v.facts.nodes[0].properties.AXMain.value,true);return v;}
 const reference=witnessRead('before');
 if(method==='N05-AX-BOUNDARY@2'){
  assert.equal(config.shipping_numeric_reports.length,2);
  report.shipping_numeric_reuse=config.shipping_numeric_reports.map(file=>{const saved=read(file);assert.equal(saved.phase,'candidate');assert.equal(saved.pin,config.pin);assert.equal(saved.cold.length,20);assert.equal(saved.warm.length,100);assert.equal(saved.workload,'UIB.D06-NATIVE-REQUEST@1');assert.deepEqual(read(path.join(path.dirname(file),'before.json')).facts.nodes,reference.facts.nodes,'shipping reference input changed');return {file,sha256:hash(file),mode:saved.mode,criterion:'numeric only; historical quality remains unchanged',reference_facts_equal:true};});
  assert.deepEqual(report.shipping_numeric_reuse.map(r=>r.mode).sort(),['off','on']);
 }
 const context={schema_version:'0.1.0',session_id:crypto.randomUUID(),target:{id:`f02-pid-${manifest.pid}`,generation:manifest.target_generation},surfaces:[{id:`window-${manifest.window_id}`,generation:manifest.surface_generation}],scope_id:`f02-window-${manifest.window_id}`,projection:'interaction',fields,plugin:{id:'macos',version:'0.1.0'},environment_revision:'f02-expanded-a'};
 const binding=Object.fromEntries(['pid','bundle_id','launch_time','window_id','window_identifier','target_generation','surface_generation'].map(k=>[k,manifest[k]]));
 const native=dir=>JSON.stringify({binding,scope_id:context.scope_id,collection:'window-ax',identity_path:manifest.identity_path,acquisition_limits:profile,acquisition_evidence:true,artifact_directory:dir,pixel_policy:'owned_synthetic_fixture'});
 const descriptor={schema_version:'0.1.0',artifact:{kind:'session',data:{session_id:context.session_id,plugin:context.plugin,supported_versions:['0.1.0'],target:context.target,surfaces:context.surfaces,allowed_scopes:[context.scope_id],capabilities:['external_semantics','rendered_capture'].map(channel=>({channel,operation:'observe',status:'partial',reason:'owned-F02-Q02'}))}}};
 const configPath=path.join(output,'host-config.json');save(configPath,{descriptor,provider:{backend:'native',helper,configuration:native(path.join(output,'unused')),channels:3}});
 let peerNumber=0,axRoot;
 function launch(){
  if(phase==='baseline')return directClient(baseline,[manifestPath,fixture,path.join(__dirname,'acquisition/profile.json')]);
  axRoot=path.join(output,`ax-peer-${peerNumber++}`);const local=path.join(output,`peer-${peerNumber}.json`);
  save(local,{descriptor,provider:{backend:'native',helper,configuration:native(axRoot),channels:3}});return client(executable,local);
 }
 const observations=new Set();
 async function one(driver,cohort,index,coldStart){
  const sampleDir=path.join(output,`${cohort}-${index}`);fs.mkdirSync(sampleDir,{mode:0o700});const images=path.join(sampleDir,'images'),start=now();let sample,row={index};
  try{
   if(phase==='baseline')driver.send({output:images});else driver.send({selection:null,native_configuration:native(images),request:{schema_version:'0.1.0',artifact:{kind:'request',data:{request_id:`q02-${cohort}-${index}`,clock_domain:'rebound',context,limits:{max_elements:160,max_depth:9,max_output_bytes:524288,deadline_ms:3000},freshness_policy:'current_required',operation:{operation:'observe',channels:['external_semantics','rendered_capture']}}}}});
   sample=await driver.next();row.outer_ms=now()-start;if(coldStart!==undefined)row.cold_ms=now()-coldStart;save(path.join(sampleDir,'sample.json'),sample);
   assert.equal(sample.kind,'sample');
   if(phase==='baseline'){
    assert.equal(sample.status,'observed');Object.assign(row,{ax_ms:sample.ax_ms,capture_ms:sample.capture_ms,request_ms:sample.request_ms,png_bytes:sample.png_bytes,image:sample.image,response_bytes:Buffer.byteLength(JSON.stringify(sample)),acquisition:sample.facts.acquisition,capture_acquisition:sample.capture_acquisition});assert.deepEqual(sample.before,reference.before);assert.deepEqual(sample.after,reference.after);assert.deepEqual(sample.facts.nodes,reference.facts.nodes);assert.deepEqual(sample.pixels,[1100,1050]);
   }else{
    assert.equal(sample.terminal,'Completed');assert.equal(sample.committed,3);assert.equal(sample.missing,0);assert.equal(sample.frames.length,2);
    const responses=sample.frames.map(f=>JSON.parse(f.canonical).artifact.data);for(const r of responses){assert.equal(r.request_id,`q02-${cohort}-${index}`);assert.equal(r.result.status,'observed');assert.deepEqual(r.result.data.context,context);}
    const ax=responses.find(r=>r.channel==='external_semantics').result.data,pixels=responses.find(r=>r.channel==='rendered_capture').result.data;
    const acquisition=read(path.join(axRoot,`observe-${sample.number}`,'ax','acquisition.json'));
    const fidelity=method==='fixed76@1'?sourceFidelity(reference.facts,ax):sourceFidelityWithBoundary(reference.facts,ax,acquisition,context);
    const stages={};for(const o of [...ax.observations,...pixels.observations]){assert.equal(o.freshness,'current');assert.equal(o.answer_source,'live');assert(!observations.has(o.id));observations.add(o.id);stages[o.channel]=(o.end-o.start)*(o.time_unit==='seconds'?1000:1);}
    assert.equal(pixels.captures.length,1);const capture=pixels.captures[0];assert.equal(capture.capture_kind,'window_isolated');assert.deepEqual(capture.capture_target,context.surfaces[0]);assert.equal(capture.pixel_width,1100);assert.equal(capture.pixel_height,1050);
    const image=path.join(images,'capture','capture.png'),metadata=read(path.join(images,'capture','capture-metadata.json'));
    Object.assign(row,{ax_ms:stages.external_semantics,capture_ms:stages.rendered_capture,capture_api_png_ms:(metadata.capture_call_end-metadata.capture_call_start)*1000,request_ms:sample.request_ms,domain_usage:sample.domain_usage,fidelity:fidelity.counts,response_bytes:sample.frames.reduce((a,f)=>a+Buffer.byteLength(f.canonical),0),png_bytes:fs.statSync(image).size,image,acquisition,source_tree_variation:fidelity.source_tree_variation||[]});
    assert.equal(fidelity.matched,true,JSON.stringify(fidelity.errors.slice(0,2)));if(method==='fixed76@1')assert.equal(ax.nodes.length,76);assert.deepEqual(ax.coverage.fields,fields);for(const node of ax.nodes)assert.deepEqual(node.properties.map(p=>p.field),fields);
   }
   const png=fs.readFileSync(row.image);assert.equal(png.readUInt32BE(16),1100);assert.equal(png.readUInt32BE(20),1050);row.png_sha256=hash(row.image);row.quality='passed';
  }catch(e){row.outer_ms??=now()-start;if(coldStart!==undefined)row.cold_ms??=now()-coldStart;row.quality='failed';row.failure=String(e.message).slice(0,700);report.failures.push({cohort,index,failure:row.failure});}
  report[cohort].push(row);fs.writeFileSync(path.join(output,'progress.json'),JSON.stringify({phase,mode:report.mode,cold:report.cold.length,warm:report.warm.length,failures:report.failures}));
 }
 try{
  for(let i=0;i<20;i++){let d;const start=now();try{d=launch();assert.equal((await d.next()).kind,'attached');await one(d,'cold',i,start);}finally{if(d)try{report.closure.push(await d.close());}catch(e){report.failures.push({cohort:'cold',index:i,closure:String(e.message)});}}}
  let d;try{d=launch();assert.equal((await d.next()).kind,'attached');await one(d,'initial',0);for(let i=0;i<100;i++)await one(d,'warm',i);}finally{if(d)report.closure.push(await d.close());}
  const after=witnessRead('after');assert.deepEqual(after.facts.nodes,reference.facts.nodes);assert.deepEqual(after.before,reference.before);assert.equal(hash(manifestPath),report.manifest_sha256);report.invariance=true;
 }catch(e){report.failure=String(e.message).slice(0,700);}
 finally{report.environment.load_after=os.loadavg();report.stats=Object.fromEntries(['cold','warm'].map(c=>[c,Object.fromEntries(['ax_ms','capture_ms','outer_ms','cold_ms','request_ms','capture_api_png_ms','response_bytes','png_bytes'].map(k=>[k,stats(report[c],k)]))]));report.gates=phase==='candidate'?{ax:Number.isFinite(report.stats.warm.ax_ms.p95)&&report.stats.warm.ax_ms.p95<=100,capture:Number.isFinite(report.stats.warm.capture_ms.p95)&&report.stats.warm.capture_ms.p95<=200,combined:Number.isFinite(report.stats.warm.outer_ms.p95)&&report.stats.warm.outer_ms.p95<=300,cold:Number.isFinite(report.stats.cold.cold_ms.p95)&&report.stats.cold.cold_ms.p95<=750,quality:report.failures.length===0&&!report.failure&&report.invariance===true&&report.cold.length===20&&report.warm.length===100}:null;if(method==='N05-AX-BOUNDARY@2'){report.diagnostic_numeric_gates=report.gates;report.gates={quality:report.gates.quality};report.timing_qualification=false;}save(path.join(output,'report.json'),report);}
 console.log(JSON.stringify({phase,mode:report.mode,counts:[report.cold.length,report.warm.length],failures:report.failures.length,failure:report.failure,invariance:report.invariance,gates:report.gates,stats:report.stats}));
}
if(require.main===module)run().catch(e=>{console.error(e.stack);process.exitCode=1;});
module.exports={stats};
