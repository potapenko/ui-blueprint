// Recorded raw evidence is read-only. Faults below mutate isolated in-memory
// copies, never the live fixture, saved facts, or expected qualification input.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const {sourceFidelityWithBoundary}=require('./performance.cjs');
const root=process.argv[2];assert(root,'pass the retained N05 diagnostic-off-2 directory');
const read=p=>JSON.parse(fs.readFileSync(path.join(root,p))),copy=v=>structuredClone(v);
const facts=read('before.json').facts;
function recorded(index){
  const sample=read(`cold-${index}/sample.json`),snapshot=sample.frames.map(f=>JSON.parse(f.canonical).artifact.data).find(r=>r.channel==='external_semantics').result.data;
  const acquisition=read(`ax-peer-${index}/observe-1/ax/acquisition.json`);
  // Legacy rows are actual same-call evidence. This envelope is synthetic test
  // setup only; it never upgrades these old samples to the prospective method.
  acquisition.q02_boundary={method:'N05-AX-BOUNDARY@2',complete:true,truncated:false,observation_id:snapshot.observations[0].id,surface:snapshot.context.surfaces[0],rows:acquisition.q02_boundary};
  return {snapshot,acquisition,context:copy(snapshot.context)};
}
const one=recorded(14),zero=recorded(15),results=[];
function check(name,input,pass,edit=()=>{},reference=facts){
  const value=copy(input);edit(value);
  const result=sourceFidelityWithBoundary(reference,value.snapshot,value.acquisition,value.context);
  assert.equal(result.matched,pass,`${name}: ${JSON.stringify(result.errors)}`);
  results.push({name,expected:pass,matched:result.matched});return result;
}
assert.equal(check('returned child/count1',one,true).source_tree_variation.length,0);
assert.equal(check('same-call successful empty enumeration',zero,true).source_tree_variation.length,1);
check('returned child dropped before publication',zero,false,v=>{v.acquisition.q02_boundary.rows=copy(one.acquisition.q02_boundary.rows).map(r=>({...r,uptime:zero.snapshot.observations[0].start}));});
check('retained node but dropped edge',one,false,v=>{v.snapshot.nodes[28].children=[];});
check('wrong handle binding',one,false,v=>{v.acquisition.q02_boundary.rows.find(r=>r.operation==='visit').same_as_parent28_child=false;});
check('omitted known field',zero,false,v=>{v.snapshot.nodes[0].properties=v.snapshot.nodes[0].properties.filter(p=>p.field!=='focused');});
check('changed known field',zero,false,v=>{v.snapshot.nodes[0].native_role.value.value='AXButton';});
for(const status of [-25205,-25212,-25202,-25204])check(`error ${status} is not empty`,zero,false,v=>{v.acquisition.q02_boundary.rows.find(r=>r.operation==='count'&&r.node_alias===28).status=status;});
check('page failure',one,false,v=>{v.acquisition.q02_boundary.rows.find(r=>r.operation==='range').status=-25204;});
check('unvisited returned child',one,false,v=>{v.acquisition.q02_boundary.rows=v.acquisition.q02_boundary.rows.filter(r=>r.operation!=='visit');});
check('missing trace',zero,false,v=>{delete v.acquisition.q02_boundary;});
check('truncated trace',zero,false,v=>{v.acquisition.q02_boundary.truncated=true;});
check('missing trace tail',zero,false,v=>{v.acquisition.q02_boundary.rows.pop();});
check('wrong observation',zero,false,v=>{v.acquisition.q02_boundary.observation_id='unrelated';});
check('wrong target',zero,false,v=>{v.snapshot.context.target.id='another-target';});
check('wrong surface',zero,false,v=>{v.acquisition.q02_boundary.surface={id:'another-window',generation:'other'};});
check('stale boundary time',zero,false,v=>{v.acquisition.q02_boundary.rows[0].uptime=0;});
for(const name of ['refused_values','known_unread_child_entries','unknown_children_lists','remaining_queued_handles','unreturned_child_references'])check(name,zero,false,v=>{v.acquisition[name]=1;});
check('returned nodes mismatch',zero,false,v=>{v.acquisition.returned_nodes++;});
const logical=copy(facts);logical.nodes[70].properties.AXIdentifier={availability:'known',value:'required-control'};
check('logical control cannot disappear',zero,false,()=>{},logical);
check('extra missing logical control',zero,false,v=>{v.snapshot.nodes.splice(13,1);v.acquisition.returned_nodes--;v.acquisition.visited_unique_nodes--;v.acquisition.discovered_unique_handles--;});
assert.deepEqual(facts,read('before.json').facts,'raw source unchanged');
console.log(JSON.stringify({method:'N05-AX-BOUNDARY@2',cases:results.length,results}));
