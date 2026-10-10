// Q02 Native current-source preflight. Never launches/activates a target application.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),os=require('node:os'),crypto=require('node:crypto');
const {spawnSync}=require('node:child_process');
const {isDeepStrictEqual}=require('node:util');
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
function sourceFidelity(facts,snapshot){
  const errors=[],counts={known:0,unavailable:0,actions:0,edges:0};
  if(facts.nodes.length!==snapshot.nodes.length)errors.push({field:'node_count',raw:facts.nodes.length,canonical:snapshot.nodes.length});
  for(let i=0;i<Math.min(facts.nodes.length,snapshot.nodes.length);i++){
    const raw=facts.nodes[i],node=snapshot.nodes[i],props=Object.fromEntries(node.properties.map(p=>[p.field,p.state]));
    for(const p of [...node.properties,...node.extensions.map(e=>e.property)]){
      if(p.evidence.source_namespace!=='macos.ax'||p.evidence.provenance!=='reported'||!snapshot.observations.some(o=>o.id===p.evidence.observation_id&&o.source_namespace==='macos.ax'))errors.push({node:i,field:p.field,reason:'source_evidence_mismatch'});
    }
    const ext=Object.fromEntries(node.extensions.map(e=>[e.name,e.property.state]));
    const map={AXRole:node.native_role,AXSubrole:ext.AXSubrole,AXIdentifier:ext.AXIdentifier,AXTitle:ext.AXTitle,
      AXDescription:props.description,AXValue:props.value,AXEnabled:props.enabled,AXFocused:props.focused,AXPlaceholderValue:props.placeholder};
    for(const [field,actual] of Object.entries(map)){
      const expected=raw.properties[field];
      if(!expected||!actual){errors.push({node:i,field,reason:'missing'});continue;}
      if(actual.availability!==expected.availability)errors.push({node:i,field,raw:expected.availability,canonical:actual.availability});
      else if(expected.availability==='known'){
        counts.known++;if(JSON.stringify(actual.value.value)!==JSON.stringify(expected.value))errors.push({node:i,field,raw:expected.value,canonical:actual.value.value});
      }else{counts.unavailable++;if('value'in actual)errors.push({node:i,field,reason:'unavailable_has_value'});}
    }
    if(JSON.stringify(props.accessibility_name)!==JSON.stringify(props.description))errors.push({node:i,field:'Description_name_mapping'});
    const position=raw.properties.AXPosition,size=raw.properties.AXSize;
    if(position.availability==='known'&&size.availability==='known'){
      const bounds=props.accessibility_bounds;
      if(bounds.availability!=='known')errors.push({node:i,field:'bounds_unknown'});
      else{const geometry=bounds.value.value,shape=geometry.shape.value;
        if(geometry.frame_kind!=='accessibility_bounds'||!isDeepStrictEqual(geometry.coordinate_space,{id:'ax-screen',kind:'screen',units:'pt',origin:'top_left'}))errors.push({node:i,field:'bounds_units_or_frame_kind'});
        for(const [k,v]of Object.entries({...position.value,...size.value})){counts.known++;if(shape[k]!==v)errors.push({node:i,field:k,raw:v,canonical:shape[k]});}}
    }
    if(raw.actions_error===0){counts.actions++;if(props.actions.availability!=='known'||JSON.stringify(props.actions.value.value)!==JSON.stringify(raw.actions))errors.push({node:i,field:'actions'});}
    const children=node.children.map(c=>Number(c.key.match(/-(\d+)$/)?.[1]));counts.edges+=children.length;
    if(JSON.stringify(children)!==JSON.stringify(raw.children))errors.push({node:i,field:'children',raw:raw.children,canonical:children});
  }
  return {matched:errors.length===0,counts,errors,method:'bounded same-window raw/canonical breadth-first order, checked native IDs/roles/all selected source properties; comparison only, never action identity'};
}
function historicalComparison(baseline,current){
  const old=baseline.external_semantics.nodes.map(n=>({properties:n.properties}));
  function index(nodes){const found=new Map();for(const n of nodes){const p=n.properties.AXIdentifier;if(p?.availability==='known'){const id=p.value;found.set(id,found.has(id)?null:n);}}return found;}
  const a=index(old),b=index(current.nodes),differences=[],missing=[],added=[];let compared=0;
  for(const [id,node] of a){if(!node)continue;const live=b.get(id);if(!live){missing.push(id);continue;}
    for(const [name,expected]of Object.entries(node.properties)){
      if(name==='AXF02Unsupported')continue;const actual=live.properties[name];
      if(expected.availability==='known'){compared++;if(actual?.availability!=='known'||JSON.stringify(expected.value)!==JSON.stringify(actual.value))differences.push({identifier:id,field:name,expected,actual});}
    }
  }
  for(const [id,node]of b)if(node&&!a.has(id))added.push(id);
  return {baseline_nodes:old.length,current_nodes:current.nodes.length,compared_known_facts_on_unique_reported_identifiers:compared,differences,missing_identifiers:missing,added_identifiers:added,
    anonymous_nodes:'not heuristically joined across different historical/live trees; full live tree is separately compared with current canonical output'};
}
function historicalTreeComparison(baseline,current){
  const old=baseline.external_semantics.nodes,live=current.nodes;
  const children=old.map(()=>[]);old.forEach((n,i)=>{if(i)children[n.parent_index].push(i);});
  const id=n=>n.properties.AXIdentifier?.availability==='known'?n.properties.AXIdentifier.value:null;
  const oldIds=new Set(old.map(id).filter(Boolean));
  const result={paired_nodes:0,known_facts:0,unavailable_facts:0,action_lists:0,baseline_edges:old.length-1,extra_nodes:[],differences:[],unpaired:[],method:'ordered rooted tree alignment with explicit added identifiers recorded, never omitted from collection or timing; not action identity'};
  const extra=i=>{result.extra_nodes.push({index:i,identifier:id(live[i]),role:live[i].properties.AXRole});for(const child of live[i].children)extra(child);};
  function pair(a,b){
    const x=old[a],y=live[b];
    if(!y||id(x)!==id(y)||x.properties.AXRole.value!==y.properties.AXRole.value){result.unpaired.push({baseline:a,current:b,baseline_id:id(x),current_id:y?id(y):null});return;}
    result.paired_nodes++;
    for(const [name,p]of Object.entries(x.properties)){
      if(name==='AXF02Unsupported')continue;const q=y.properties[name];
      if(p.availability==='known')result.known_facts++;else result.unavailable_facts++;
      if(!q||p.availability!==q.availability||(p.availability==='known'&&!isDeepStrictEqual(p.value,q.value)))result.differences.push({baseline:a,current:b,identifier:id(x),field:name,expected:p,actual:q});
    }
    if(x.actions_error===0){result.action_lists++;if(y.actions_error!==0||!isDeepStrictEqual(x.actions,y.actions))result.differences.push({baseline:a,current:b,identifier:id(x),field:'actions',expected:x.actions,actual:y.actions});}
    let cursor=0;
    for(const child of y.children){
      const childId=id(live[child]);
      if(childId&&!oldIds.has(childId)){extra(child);continue;}
      if(cursor<children[a].length)pair(children[a][cursor++],child);else result.unpaired.push({baseline:null,current:child});
    }
    for(const child of children[a].slice(cursor))result.unpaired.push({baseline:child,current:null});
  }
  pair(0,0);result.all_original_nodes_paired=result.paired_nodes===old.length&&result.unpaired.length===0;
  result.comparability=result.all_original_nodes_paired&&result.differences.length===0&&result.extra_nodes.length===0?'equal':'not_equivalent';
  return result;
}
function invariance(before,after){
  const a=before.facts.nodes,b=after.facts.nodes,metadata=isDeepStrictEqual(before.before,before.after)&&isDeepStrictEqual(before.before,after.before)&&isDeepStrictEqual(after.before,after.after);
  let insertion=null;
  if(b.length===a.length+1){
    for(let parent=0;parent<a.length;parent++){
      if(a[parent].children.length!==0||b[parent]?.children.length!==1||!isDeepStrictEqual(a[parent].properties,b[parent].properties))continue;
      const added=b[parent].children[0],map=i=>i>=added?i+1:i;
      if(a.every((n,i)=>{const expected={...n,index:map(i),children:n.children.map(map)},actual={...b[map(i)]};if(i===parent)actual.children=actual.children.filter(x=>x!==added);return isDeepStrictEqual(expected,actual);})){insertion={parent_before:parent,added_after:added,node:b[added],all_preexisting_data_and_edges_unchanged:true};break;}
    }
  }
  return {metadata,exact_tree_and_values:isDeepStrictEqual(a,b),single_structural_insertion:insertion,
    method:'literal graph edit reconciliation only; no persistent identity or action refs inferred'};
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
  assert.equal(manifest.role,'a');assert.equal(manifest.window_identifier,'a');
  assert.equal(manifest.collection_mode,'explicit_request_only');assert(manifest.snapshot_request>0);
  assert(['local.uiblueprint.f02.off','local.uiblueprint.f02.on'].includes(manifest.bundle_id)); // exact executable/incarnation remains mandatory
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
  const witness=process.env.UIB_Q02_WITNESS,fixture=process.env.UIB_Q02_FIXTURE_EXECUTABLE;
  assert(path.isAbsolute(witness||'')&&path.isAbsolute(fixture||''));assert.equal(digest(witness),process.env.UIB_Q02_WITNESS_SHA256);
  const report={kind:'comparability_preflight_not_D06_timing',functional_pin:process.env.UIB_Q02_FUNCTIONAL_PIN,baseline_sha256:digest(baselinePath),
    manifest_sha256:digest(manifestPath),executable_sha256:digest(executable),helper_sha256:digest(helper),mode:manifest.probe_enabled?'on':'off',
    historical_manifest_state:manifest.state,witness_sha256:digest(witness),fixture_executable_sha256:digest(fixture),
    retention:'Q02/root owns nonimage review consumption; every image and containing directory retained without agent deletion'};
  function witnessRead(mode,name){
    const r=spawnSync(witness,[manifestPath,fixture,path.join(__dirname,'acquisition/profile.json'),mode],{timeout:5000,maxBuffer:1048576});
    const value=r.stdout?.length?JSON.parse(r.stdout):{status:'unavailable',code:r.error?.code||r.signal||'empty_response'};
    fs.writeFileSync(path.join(output,name+'.json'),JSON.stringify(value),{flag:'wx',mode:0o600});
    report[name]={exit:r.status,signal:r.signal,stderr_bytes:r.stderr?.length||0,status:value.status,code:value.code};
    assert.equal(r.status,0,`${name}: ${value.code||value.status}`);assert.equal(value.status,'validated');
    assert.deepEqual(value.before,value.after,`${name}: metadata changed during witness`);return value;
  }
  let driver;
  try{
    const before=witnessRead('facts','before');assert.equal(before.facts.status,'observed','raw AX unavailable before collection');
    report.fresh_identity=before.before;
    driver=client(executable,configPath);assert.equal((await driver.next()).kind,'attached');
    driver.send({selection:null,native_configuration:null,request:{schema_version:'0.1.0',artifact:{kind:'request',data:{request_id:'q02-preflight',clock_domain:'rebound',context,
      limits:{max_elements:160,max_depth:9,max_output_bytes:524288,deadline_ms:3000},freshness_policy:'current_required',operation:{operation:'observe',channels:['external_semantics','rendered_capture']}}}}});
    const sample=await driver.next();fs.writeFileSync(path.join(output,'canonical.json'),JSON.stringify(sample),{flag:'wx',mode:0o600});
    const after=witnessRead('facts','after');assert.equal(after.facts.status,'observed','raw AX unavailable after collection');
    report.invariance=invariance(before,after);
    report.invariance.retained_manifest=isDeepStrictEqual(read(manifestPath),manifest);
    const responses=sample.frames.map(f=>JSON.parse(f.canonical).artifact.data);
    report.channel_outcomes=responses.map(r=>({channel:r.channel,status:r.result.status,code:r.result.status==='failed'?r.result.data.code:null}));
    const ax=responses.find(r=>r.channel==='external_semantics');
    if(ax?.result.status==='observed')report.source_fidelity={before:sourceFidelity(before.facts,ax.result.data),after:sourceFidelity(after.facts,ax.result.data)};
    report.historical_comparison=historicalComparison(baseline,after.facts);
    report.historical_tree_comparison=historicalTreeComparison(baseline,after.facts);
    report.matrix=matrix(sample,baseline,context);
    if(report.historical_comparison.differences.length||report.historical_comparison.added_identifiers.length||report.historical_comparison.missing_identifiers.length)report.matrix.comparability='incompatible';
    if(!report.invariance.metadata||!report.invariance.exact_tree_and_values||!report.invariance.retained_manifest)report.matrix.readonly_invariance='not_fully_established';
  }catch(error){report.failure=String(error.message).slice(0,300);}
  finally{if(driver){try{report.closure=await driver.close();report.cleanup_confirmed=true;}catch{report.cleanup_confirmed=false;}}
    fs.writeFileSync(path.join(output,'report.json'),JSON.stringify(report,null,2)+'\n',{flag:'wx',mode:0o600});}
  console.log(JSON.stringify({output,matrix:report.matrix,failure:report.failure,cleanup_confirmed:report.cleanup_confirmed}));
}
if(process.argv.includes('--baseline-only')){
  const baseline=read(process.env.UIB_Q02_NATIVE_BASELINE);const value=inventory(baseline);
  console.log(JSON.stringify({nodes:value.nodes,coverage:value.coverage,attributes:value.attributes}));
}else if(require.main===module)run().catch(error=>{console.error(error.message);process.exitCode=1;});
// N05 prospective quality only. The complete reference is retained; the only
// supported structural exception is the independently investigated anonymous leaf.
// Every other difference is a failure, not an expanded allowed-count interval.
function sourceFidelityWithBoundary(facts,snapshot,acquisition,context){
  const method='N05-AX-BOUNDARY@2';
  try{
    assert.deepEqual(snapshot.context,context,'target/context mismatch');
    assert.equal(snapshot.coverage.status,'partial');
    assert.equal(snapshot.observations.length,1);
    const observation=snapshot.observations[0],proof=acquisition.q02_boundary;
    assert.equal(observation.consistency,'unknown');
    assert.equal(observation.time_unit,'seconds');assert.equal(observation.freshness,'current');assert.equal(observation.answer_source,'live');
    assert.equal(observation.source_namespace,'macos.ax');
    assert.equal(proof?.method,method,'missing method');assert.equal(proof.complete,true,'incomplete proof');assert.equal(proof.truncated,false,'truncated proof');
    assert.equal(proof.observation_id,observation.id,'wrong observation');assert.deepEqual(proof.surface,context.surfaces[0],'wrong surface');
    const rows=proof.rows;assert(Array.isArray(rows)&&rows.length>0&&rows.length<64,'missing/truncated rows');
    for(const [i,row]of rows.entries()){
      assert.equal(row.order,i);assert([28,70].includes(row.node_alias));
      assert(Number.isFinite(row.uptime)&&row.uptime>=observation.start&&row.uptime<=observation.end,'outside observation');
    }
    for(const name of ['refused_values','known_unread_child_entries','unknown_children_lists','remaining_queued_handles','unreturned_child_references','duplicate_handle_references'])assert.equal(acquisition[name],0,name);
    assert.equal(acquisition.returned_nodes,snapshot.nodes.length);assert.equal(acquisition.visited_unique_nodes,snapshot.nodes.length);assert.equal(acquisition.discovered_unique_handles,snapshot.nodes.length);
    assert.deepEqual(snapshot.coverage.fields,context.fields);
    for(const node of snapshot.nodes){
      assert.deepEqual(node.properties.map(p=>p.field),context.fields,'omitted selected field');
      const role={AXButton:'button',AXCheckBox:'checkbox',AXTextField:'textbox',AXStaticText:'text',AXGroup:'group',AXScrollArea:'scrollarea',AXSlider:'slider'}[node.native_role.value?.value];
      const state=node.properties.find(p=>p.field==='role').state;
      if(role)assert.deepEqual(state,{availability:'known',value:{type:'role',value:role}},'lost canonical role');
      else assert.equal(state.availability,'unknown');
    }
    const at=(n,op)=>rows.filter(r=>r.node_alias===n&&r.operation===op), counts=at(28,'count');
    assert.equal(counts.length,1);assert.equal(counts[0].attribute,'AXChildren');assert.equal(counts[0].status,0,'error is not empty');
    const ranges=at(28,'range'),edges=at(28,'child_alias'),visits=at(70,'visit');
    assert.equal(visits.length,1,'missing handle mapping');
    const finalCounts=at(70,'count');assert.equal(finalCounts.length,1,'missing final boundary');
    assert.equal(finalCounts[0].attribute,'AXChildren');assert.equal(finalCounts[0].status,0);assert.equal(finalCounts[0].count,0);
    assert.equal(finalCounts[0].order,rows.length-1,'incomplete trace tail');
    // Pin the investigated relation to this full source reference, never just
    // to a numeric alias or a role-based filter over the collected graph.
    const parent=facts.nodes[28],child=facts.nodes[70];
    assert.deepEqual(parent.children,[70]);assert.deepEqual(child.children,[]);
    assert.deepEqual(facts.nodes.filter(n=>n.children.includes(70)).map(n=>n.index),[28]);
    assert.equal(child.properties.AXRole.value,'AXGroup');
    for(const name of ['AXIdentifier','AXTitle','AXDescription'])assert.notEqual(child.properties[name].availability,'known','required logical control cannot disappear');
    if(counts[0].count===1){
      assert.equal(ranges.length,1);assert.deepEqual([ranges[0].status,ranges[0].offset,ranges[0].asked,ranges[0].returned],[0,0,1,1]);
      assert.equal(edges.length,1);assert.equal(edges[0].child_alias,70);assert.equal(visits[0].same_as_parent28_child,true,'wrong handle');
      assert(counts[0].order<ranges[0].order&&ranges[0].order<edges[0].order&&edges[0].order<visits[0].order);
      const result=sourceFidelity(facts,snapshot);return {...result,method,source_tree_variation:[]};
    }
    assert.equal(counts[0].count,0,'unexplained topology');assert.equal(ranges.length,0);assert.equal(edges.length,0);assert.equal(visits[0].same_as_parent28_child,false);
    // This comparison view aligns surviving aliases. It never edits raw facts,
    // stored samples, the source tree, or the requested collection scope.
    const map=i=>i>70?i-1:i;
    const available={...facts,nodes:facts.nodes.flatMap((node,i)=>i===70?[]:[{...node,index:map(i),children:node.children.filter(c=>c!==70).map(map)}])};
    const result=sourceFidelity(available,snapshot);
    return {...result,method,source_tree_variation:[{reference_parent:28,reference_child:70,source:'same_call_successful_empty_enumeration',proof_order:counts[0].order}],reference_nodes:facts.nodes.length,observed_nodes:snapshot.nodes.length};
  }catch(error){return {matched:false,counts:{},errors:[{reason:String(error.message).slice(0,300)}],method,source_tree_variation:[]};}
}
module.exports={inventory,matrix,sourceFidelity,sourceFidelityWithBoundary,historicalComparison,historicalTreeComparison,invariance};
