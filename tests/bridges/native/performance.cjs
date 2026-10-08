// Q02 Native current-source preflight. Never launches/activates a target application.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),os=require('node:os'),crypto=require('node:crypto');
const {client}=require('../web/performance.cjs');
const profile=require('./acquisition/profile.json');
const fields=['role','accessibility_name','description','value','placeholder','enabled','focused','actions','accessibility_bounds'];
function digest(file){return crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex');}
function read(file,cap=1048576){const fd=fs.openSync(file,'r');try{const bytes=Buffer.alloc(cap+1),n=fs.readSync(fd,bytes);assert(n<=cap);return JSON.parse(bytes.subarray(0,n));}finally{fs.closeSync(fd);}}
function inventory(raw){
  const nodes=raw.external_semantics.nodes,attributes={};
  for(const node of nodes)for(const [name,p] of Object.entries(node.properties)){
    attributes[name]??={};attributes[name][p.availability]=(attributes[name][p.availability]||0)+1;
  }
  return {nodes:nodes.length,coverage:raw.external_semantics.coverage,attributes,capture:raw.rendered_capture};
}
function matrix(sample,baseline,context){
  assert.equal(sample.terminal,'Completed');assert.equal(sample.committed,3);assert.equal(sample.missing,0);
  const responses=sample.frames.map(f=>JSON.parse(f.canonical).artifact.data);
  assert.equal(responses.length,2);for(const response of responses){assert.equal(response.result.status,'observed');assert.deepEqual(response.target,context.target);assert.equal(response.session_id,context.session_id);}
  const ax=responses.find(r=>r.channel==='external_semantics').result.data;
  const pixels=responses.find(r=>r.channel==='rendered_capture').result.data;
  assert.deepEqual(ax.context,context);assert.deepEqual(pixels.context,context);
  const availability={},extensions={};
  for(const node of ax.nodes){
    assert.deepEqual(node.surface,context.surfaces[0]);
    assert.deepEqual(node.properties.map(p=>p.field),fields);
    for(const p of node.properties){availability[p.field]??={};availability[p.field][p.state.availability]=(availability[p.field][p.state.availability]||0)+1;}
    for(const e of node.extensions){extensions[e.name]??={};extensions[e.name][e.property.state.availability]=(extensions[e.name][e.property.state.availability]||0)+1;}
  }
  const root=ax.nodes[0],bounds=root.properties.find(p=>p.field==='accessibility_bounds');
  assert.equal(bounds.state.availability,'known');const window=bounds.state.value.value.shape.value;
  assert.equal(pixels.captures.length,1);const capture=pixels.captures[0];
  assert.equal(capture.capture_kind,'window_isolated');assert.deepEqual(capture.capture_target,context.surfaces[0]);
  const original=inventory(baseline),gaps=[];
  if(ax.nodes.length!==original.nodes)gaps.push(`node_count:${original.nodes}->${ax.nodes.length}`);
  if(window.width!==550||window.height!==525)gaps.push(`window:${window.width}x${window.height}`);
  if(capture.pixel_width!==1100||capture.pixel_height!==1050)gaps.push(`pixels:${capture.pixel_width}x${capture.pixel_height}`);
  if(original.attributes.AXTitle?.known&&!extensions.AXTitle?.known)gaps.push('known_baseline_AXTitle_has_no_explicit_current_source_field');
  const stages=[...ax.observations,...pixels.observations].map(o=>{
    assert.equal(o.answer_source,'live');assert.equal(o.freshness,'current');assert(['seconds','milliseconds'].includes(o.time_unit));
    return {channel:o.channel,clock_domain:o.clock_domain,ms:(o.end-o.start)*(o.time_unit==='seconds'?1000:1)};
  });
  return {comparability:gaps.length?'incompatible':'requires_field_value_reconciliation',gaps,baseline:original,current:{nodes:ax.nodes.length,coverage:ax.coverage,availability,extensions,window,
    pixels:[capture.pixel_width,capture.pixel_height],capture_kind:capture.capture_kind},stages,
    note:'Matching aggregate counts alone is not full field/value equality; each accepted original source identifier/value also needs independent reconciliation.'};
}
async function run(){
  assert(process.argv.includes('--preflight-runtime'),'this preparation implements preflight only; timed Native series require comparable workload');
  assert.equal(process.env.UIB_Q02_ALLOW,'1');assert(/^[a-f0-9]{40}$/.test(process.env.UIB_Q02_FUNCTIONAL_PIN||''));
  const manifestPath=process.env.UIB_Q02_MANIFEST,manifest=read(manifestPath),baselinePath=process.env.UIB_Q02_NATIVE_BASELINE,baseline=read(baselinePath);
  assert.equal(manifest.role,'a');assert.equal(manifest.window_identifier,'a');assert.equal(manifest.source_state.stimulus,'normal');
  assert.equal(manifest.state.expanded,true);assert.equal(manifest.state.wide,false);assert.equal(manifest.state.popup,false);
  assert.equal(manifest.state.secret_present,false);assert.equal(manifest.collection_mode,'explicit_request_only');assert(manifest.snapshot_request>0);
  assert(['local.uiblueprint.f02.off','local.uiblueprint.f02.on'].includes(manifest.bundle_id));
  const executable=process.env.UIB_Q02_EXECUTABLE,helper=process.env.UIB_Q02_HELPER;
  for(const [file,hash] of [[executable,process.env.UIB_Q02_EXECUTABLE_SHA256],[helper,process.env.UIB_Q02_HELPER_SHA256]]){assert(path.isAbsolute(file||''));assert.equal(digest(file),hash);}
  const output=process.env.UIB_Q02_OUTPUT;assert(path.resolve(output).startsWith(fs.realpathSync(os.tmpdir())+path.sep));fs.mkdirSync(output,{mode:0o700});
  const artifactDirectory=path.join(output,'images'); // Native helper creates this0700 image owner. Never delete it.
  const context={schema_version:'0.1.0',session_id:crypto.randomUUID(),target:{id:`f02-pid-${manifest.pid}`,generation:manifest.target_generation},
    surfaces:[{id:`window-${manifest.window_id}`,generation:manifest.surface_generation}],scope_id:`f02-window-${manifest.window_id}`,projection:'interaction',fields,
    plugin:{id:'macos',version:'0.1.0'},environment_revision:'f02-expanded-a'};
  const binding=Object.fromEntries(['pid','bundle_id','launch_time','window_id','window_identifier','target_generation','surface_generation'].map(k=>[k,manifest[k]]));
  const config={binding,scope_id:context.scope_id,collection:'window-ax',identity_path:manifest.identity_path,
    acquisition_limits:profile,acquisition_evidence:true,artifact_directory:artifactDirectory,pixel_policy:'owned_synthetic_fixture'};
  const descriptor={schema_version:'0.1.0',artifact:{kind:'session',data:{session_id:context.session_id,plugin:context.plugin,supported_versions:['0.1.0'],
    target:context.target,surfaces:context.surfaces,allowed_scopes:[context.scope_id],capabilities:['external_semantics','rendered_capture'].map(channel=>({channel,operation:'observe',status:'partial',reason:'owned-F02-preflight'}))}}};
  const configuration=JSON.stringify(config);assert(Buffer.byteLength(configuration)<=4032);
  const configPath=path.join(output,'config.json');fs.writeFileSync(configPath,JSON.stringify({descriptor,provider:{backend:'native',helper,configuration,channels:3}}),{flag:'wx',mode:0o600});
  const report={kind:'comparability_preflight_not_D06_timing',functional_pin:process.env.UIB_Q02_FUNCTIONAL_PIN,baseline_sha256:digest(baselinePath),
    manifest_sha256:digest(manifestPath),executable_sha256:digest(executable),helper_sha256:digest(helper),mode:manifest.probe_enabled?'on':'off',
    retention:'Q02/root owns nonimage review consumption; every image and containing directory retained without agent deletion'};
  let driver;
  try{
    driver=client(executable,configPath);assert.equal((await driver.next()).kind,'attached');
    driver.send({selection:null,native_configuration:null,request:{schema_version:'0.1.0',artifact:{kind:'request',data:{request_id:'q02-preflight',clock_domain:'rebound',context,
      limits:{max_elements:160,max_depth:9,max_output_bytes:524288,deadline_ms:3000},freshness_policy:'current_required',operation:{operation:'observe',channels:['external_semantics','rendered_capture']}}}}});
    const sample=await driver.next();fs.writeFileSync(path.join(output,'canonical.json'),JSON.stringify(sample),{flag:'wx',mode:0o600});
    report.matrix=matrix(sample,baseline,context);assert.deepEqual(read(manifestPath),manifest,'read-only fixture manifest unchanged');
  }catch(error){report.failure=String(error.message).slice(0,300);}
  finally{if(driver){try{await driver.close();report.cleanup_confirmed=true;}catch{report.cleanup_confirmed=false;}}
    fs.writeFileSync(path.join(output,'report.json'),JSON.stringify(report,null,2)+'\n',{flag:'wx',mode:0o600});}
  console.log(JSON.stringify({output,matrix:report.matrix,failure:report.failure,cleanup_confirmed:report.cleanup_confirmed}));
}
if(process.argv.includes('--baseline-only')){
  const baseline=read(process.env.UIB_Q02_NATIVE_BASELINE);const value=inventory(baseline);
  console.log(JSON.stringify({nodes:value.nodes,coverage:value.coverage,attributes:value.attributes}));
}else if(require.main===module)run().catch(error=>{console.error(error.message);process.exitCode=1;});
module.exports={inventory,matrix};
