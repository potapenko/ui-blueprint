// W04 authored B01/B04/B06 qualification through the public CLI, own headless context only.
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const os = require('node:os');
const crypto = require('node:crypto');
const http = require('node:http');
const {spawn,execFileSync} = require('node:child_process');
const {prepare} = require('./fixture-host.cjs');
const bounded = (promise, ms) => {let timer;return Promise.race([promise,new Promise((_,reject)=>{timer=setTimeout(()=>reject(Error('bounded timeout')),ms);})]).finally(()=>clearTimeout(timer));};
const property = (node,field) => node.properties.find(p=>p.field===field).state;
const rect = geometry => {const r=geometry.shape.value;return [r.x,r.y,r.width,r.height];};
function near(actual,expected){assert.equal(actual.length,expected.length);actual.forEach((n,i)=>assert(Math.abs(n-expected[i])<=0.01,`${actual} != ${expected}`));}
async function main(){
  assert(process.argv.includes('--run-authorized') && process.env.UIB_WEB_LIVE_ALLOW==='1');
  const cli=process.env.UIB_WEB_LIVE_CLI,worker=process.env.UIB_WEB_LIVE_WORKER;
  for(const file of [cli,worker]){assert(path.isAbsolute(file));await fs.access(file,require('node:fs').constants.X_OK);}
  const setup=prepare(),temp=await fs.mkdtemp(path.join(os.tmpdir(),'uib-w04-live-')),files=[];
  let server,browser,context,fixture,child,closed,profile,report;
  const checks=[];
  function workersGone(){const rows=execFileSync('ps',['-axo','command='],{encoding:'utf8',timeout:2000,maxBuffer:4*1024*1024}).split('\n');assert(!rows.some(row=>row.trim()===worker),'owned worker must be reaped');}
  async function save(name,data){const file=path.join(temp,name);await fs.writeFile(file,data,{flag:'wx',mode:0o600});files.push(file);return file;}
  let sequence=0;
  async function json(value){return save(`${sequence++}.json`,JSON.stringify(value));}
  async function call(args){
    child=spawn(cli,args,{stdio:['ignore','pipe','pipe']});let output='',diagnostics='';
    child.stdout.on('data',b=>{output+=b;if(Buffer.byteLength(output)>131072)child.kill('SIGKILL');});
    child.stderr.on('data',b=>{diagnostics+=b;if(Buffer.byteLength(diagnostics)>8192)child.kill('SIGKILL');});
    closed=new Promise((resolve,reject)=>{child.once('error',reject);child.once('close',(code,signal)=>resolve({code,signal}));});
    const exit=await bounded(closed,5000);assert.equal(exit.signal,null);workersGone();
    return {code:exit.code,output,diagnostics};
  }
  try{
    const html=await fs.readFile(path.join(setup.root,'fixtures/web/observation-pilots.html'));
    fixture=http.createServer((req,res)=>{res.writeHead(200,{'Content-Type':'text/html; charset=utf-8'});res.end(html);});
    await bounded(new Promise(resolve=>fixture.listen(0,'127.0.0.1',resolve)),2000);
    const origin=`http://127.0.0.1:${fixture.address().port}`;
    server=await setup.chromium.launchServer({headless:true,timeout:8000,args:['--remote-debugging-port=0','--remote-debugging-address=127.0.0.1']});
    profile=server.process().spawnargs.find(s=>s.startsWith('--user-data-dir=')).slice(16);
    const port=Number((await fs.readFile(path.join(profile,'DevToolsActivePort'),'utf8')).split('\n')[0]);assert(port>0);
    browser=await setup.chromium.connect(server.wsEndpoint(),{timeout:3000});assert.equal(browser.version(),'145.0.7632.6');
    context=await browser.newContext({viewport:{width:800,height:600},deviceScaleFactor:1});
    await context.route('**/*',route=>new URL(route.request().url()).origin===origin?route.continue():route.abort());
    const page=await context.newPage();page.setDefaultTimeout(2000);await page.goto(origin,{timeout:3000});
    const other=await context.newPage();await other.goto(origin,{timeout:3000});
    const cdp=await context.newCDPSession(page), otherCdp=await context.newCDPSession(other);
    async function binding(client) {
      const {targetInfo}=await client.send('Target.getTargetInfo');
      const {frameTree}=await client.send('Page.getFrameTree');
      const {root:document}=await client.send('DOM.getDocument',{depth:0,pierce:false});
      return {target:{id:targetInfo.targetId,generation:crypto.randomUUID()},
        surface:{id:frameTree.frame.id,generation:frameTree.frame.loaderId},document};
    }
    let {target,surface,document}=await binding(cdp);
    const otherBinding=await binding(otherCdp);
    const session=crypto.randomUUID();let environment='w04-800x600-font16-en';
    async function backend(id,client=cdp,doc=document){const {nodeId}=await client.send('DOM.querySelector',{nodeId:doc.nodeId,selector:`#${id}`});const {node}=await client.send('DOM.describeNode',{nodeId,depth:0,pierce:false});return node.backendNodeId;}
    async function state(){return Promise.all([page,other].map(p=>p.evaluate(()=>({active:document.activeElement?.id,scroll:[scrollX,scrollY],html:document.body.innerHTML,
      lang:document.documentElement.lang,dir:document.documentElement.dir,viewport:[innerWidth,innerHeight],
      values:[...document.querySelectorAll('input')].map(n=>n.value),
      bounds:['left','icon','label','right','responsive'].map(id=>{const r=document.getElementById(id).getBoundingClientRect();return [r.x,r.y,r.width,r.height];})}))));}
    function inputs(selection, fields=['role','accessibility_name','layout_bounds','value']){
      const scope='w04-observation',mib=1048576,plugin={id:'web',version:'0.1.0'};
      const ctx={schema_version:'0.1.0',session_id:session,target,surfaces:[surface],scope_id:scope,projection:'design',fields,plugin,environment_revision:environment};
      if(selection.selection==='rooted')selection={...selection,root:{session_id:session,target,surface,document_backend_id:document.backendNodeId,backend_node_id:selection.backend,sensitivity:'public'}};
      delete selection.backend;
      const connection={connection_version:'1.0.0',target,attach_deadline_ms:2000,
        session:{session_id:session,plugin,supported_versions:['0.1.0'],target,surfaces:[surface],allowed_scopes:[scope],capabilities:[{channel:'external_semantics',operation:'observe',status:'partial',reason:'owned-w04'}]},
        host_limits:{workers:2,worker_bytes:64*mib,publication_reserve:mib,bootstrap_bytes:mib,parent_bytes:32*mib,input_bytes:2*mib,ingress_bytes:512*1024,output_bytes:512*1024,request_output_bytes:2*mib,completion_groups:2,control_bytes:4096,cleanup_ms:1000,retained_domain_bytes:64*mib,retained_per_worker:15*mib,main_stack_bytes:8*mib,watchdog_stack_bytes:mib},
        provider:{backend:'web',selection,setup:{endpoint:`ws://127.0.0.1:${port}/devtools/page/${target.id}`,cdp_session_id:null,surface,
          transport:{endpoint_bytes:1024,handshake_bytes:2048,read_buffer_bytes:64,write_buffer_bytes:64,write_buffer_max:32768,frame_bytes:8192,message_bytes:8192,outbound_bytes:16384},
          cdp:{max_request_bytes:16384,max_message_bytes:8192,max_metadata_bytes:256,max_results:1,result_bytes:8192,max_events:4,event_bytes:34000},
          collector:{max_nodes:16,max_methods:100,max_reply_bytes:8192,max_total_reply_bytes:65536,max_text_bytes:600,max_handle_bytes:256,max_ax_properties:32,io_read_bytes:16384,io_write_bytes:32768,io_work:2048}}}};
      const request={schema_version:'0.1.0',artifact:{kind:'request',data:{request_id:crypto.randomUUID(),clock_domain:'rebound',context:ctx,limits:{max_elements:32,max_depth:8,max_output_bytes:65536,deadline_ms:250},freshness_policy:'current_required',operation:{operation:'observe',channels:['external_semantics']}}}};
      return {connection,request};
    }
    const ids=(...values)=>({selection:'initial',ids:values.map(id=>({id,sensitivity:'public'})),max_visited_nodes:256});
    let leftId=await backend('left');
    const rooted=()=>({selection:'rooted',backend:leftId,max_visited_nodes:256});
    async function observe(name,selection=rooted(),change=()=>{},attachRefusal=false){
      const input=inputs(selection);change(input);const before=await state();
      const result=await call(['observe','--connection',await json(input.connection),'--request',await json(input.request),'--worker',worker,'--max-input-bytes','131072','--max-output-bytes','65536']);
      assert.deepEqual(await state(),before,`${name}: read-only invariance`);
      assert(!result.output.includes('W04_PRIVATE_CANARY'));
      assert((attachRefusal?[4]:[2,4]).includes(result.code),`${name}: exit ${result.code} ${result.diagnostics}`);
      if(attachRefusal){assert.equal(result.output,'');assert.equal(result.diagnostics.trim(),'observe_unavailable');}
      const doc=result.output?JSON.parse(result.output):null;
      const snapshot=doc?.artifact?.data?.result?.status==='observed'?doc.artifact.data.result.data:null;
      checks.push({name,exit:result.code,observed:!!snapshot,readonly:true});
      return {snapshot,doc,result,input,file:result.output?await save(`${sequence++}.json`,result.output):null};
    }
    const keyEqual=(a,b)=>a.namespace===b.namespace&&a.key===b.key;
    const dom=(s,id)=>{const n=s.nodes.find(n=>n.key.namespace==='web.dom'&&n.key.key===String(id));assert(n);return n;};
    const ax=(s,n)=>{const r=s.relations.find(r=>r.kind==='corresponds_to'&&keyEqual(r.from,n.key));assert(r);assert.equal(r.evidence.method,'cdp-backendDOMNodeId');return s.nodes.find(n=>keyEqual(n.key,r.to));};
    const geometry=n=>property(n,'layout_bounds').value.value;
    async function inspect(observed,n,view){
      const args=['inspect','--snapshot',observed.file,'--ref',JSON.stringify(n.key),'--view',view,'--max-input-bytes','262144','--max-output-bytes','131072'];
      const result=await call([...args,'--json']);assert.equal(result.code,0);assert.deepEqual(JSON.parse(result.output).snapshot,observed.snapshot);
      const compact=await call(args);assert.equal(compact.code,0);checks.push({name:`inspect-${view}`,exit:0});return compact.output;
    }
    async function measure(observed,n,operation,amount,space=geometry(n).coordinate_space.id){
      const query={schema_version:'0.2.0',artifact:{kind:'geometry_query',data:{id:operation,scope_id:observed.snapshot.context.scope_id,targets:[n.key],operation,anchors:[{element:n.key,frame_kind:'layout_bounds',coordinate_space:geometry(n).coordinate_space,fraction:0,axis:operation==='height'?'y':'x'}],quantity_kind:'length',units:'css_px',applies_when:{platform:null,input_mode:null,text_scale:null}}}};
      const result=await call(['measure','--snapshot',observed.file,'--query',await json(query),'--space',space,'--max-input-bytes','262144','--max-output-bytes','131072','--json']);
      assert.equal(result.code,0);const data=JSON.parse(result.output).artifact.data;assert.deepEqual(data.snapshot,observed.snapshot);assert.equal(data.result.measurement.value.value.amount,amount);checks.push({name:`measure-${operation}`,amount,units:'css_px'});
    }
    async function diff(before,after,n,expected){
      const result=await call(['diff','--geometry','--before',before.file,'--after',after.file,'--ref',JSON.stringify(n.key),'--frame-kind','layout_bounds','--space',geometry(n).transform.transform.to.id,'--max-input-bytes','262144','--max-output-bytes','262144','--json']);
      assert.equal(result.code,0, result.diagnostics);const data=JSON.parse(result.output);assert.deepEqual(data.before,before.snapshot);assert.deepEqual(data.after,after.snapshot);assert.deepEqual(data.displacement,expected);checks.push({name:'geometry-diff',displacement:expected});
    }
    const initial=await observe('B06-composite');const s=initial.snapshot;assert(s);assert.equal(s.coverage.status,'partial');assert.equal(s.context.projection,'design');
    const left=dom(s,leftId),icon=dom(s,await backend('icon')),label=dom(s,await backend('label')),button=ax(s,left);
    assert.equal(property(button,'role').value.value,'button');assert.equal(property(button,'accessibility_name').value.value,'Apply');
    assert.equal(left.native_role.value.value,'BUTTON');assert.equal(icon.native_role.value.value,'SPAN');
    assert.equal(property(icon,'role').availability,'unknown');assert.equal(property(ax(s,icon),'role').availability,'unknown');
    const component=s.components[0];assert.equal(s.components.length,1);assert.equal(component.logical_component_key,'apply-control');assert.equal(component.provenance,'reported');
    assert.deepEqual(component.members.slice(0,3),[left.key,icon.key,label.key]);assert(component.members.some(k=>keyEqual(k,button.key)));
    assert.equal(left.source_declarations[0].source,component.declaration_source);
    for(const r of s.relations)assert(s.observations.some(o=>o.id===r.evidence.observation_id&&o.source_namespace===r.evidence.source_namespace));
    const design=await inspect(initial,button,'design');assert(design.includes('apply-control'));assert(design.includes('SPAN'));
    await inspect(initial,button,'interaction');
    const interaction=await observe('B06-interaction',rooted(),i=>{i.request.artifact.data.context.projection='interaction';});
    assert.equal(interaction.snapshot.components[0].logical_component_key,'apply-control');
    assert.equal(interaction.snapshot.nodes.filter(n=>n.key.namespace==='web.dom').length,3,'projection does not delete design sources');
    const references={selection:'references',nodes:[{reference:{session_id:s.context.session_id,key:left.key,snapshot_id:s.id,
      observation_id:s.observations.find(o=>o.source_namespace==='web.dom').id,target:s.context.target,surface:left.surface},sensitivity:'public'}]};
    assert((await observe('B01-current-source-ref',references)).snapshot);
    const duplicate=await observe('B01-same-labels',ids('left','right'));
    assert.equal(duplicate.snapshot.components.length,0,'missing declared parts cannot be invented');
    assert.deepEqual(duplicate.snapshot.nodes.filter(n=>n.key.namespace==='web.ax').map(n=>property(n,'accessibility_name').value.value),['Apply','Apply']);
    const otherState=await other.evaluate(()=>document.body.innerHTML);
    const otherResult=await observe('B01-other-target',ids('left','right'),i=>{
      const c=i.connection,r=i.request.artifact.data;c.target=otherBinding.target;c.session.target=otherBinding.target;c.session.surfaces=[otherBinding.surface];
      c.provider.setup.endpoint=`ws://127.0.0.1:${port}/devtools/page/${otherBinding.target.id}`;c.provider.setup.surface=otherBinding.surface;
      r.context.target=otherBinding.target;r.context.surfaces=[otherBinding.surface];
    });
    assert(otherResult.snapshot);assert.notDeepEqual(otherResult.snapshot.context.target,s.context.target);
    assert.equal((await observe('B01-wrong-target',rooted(),i=>{i.connection.provider.setup.endpoint=`ws://127.0.0.1:${port}/devtools/page/${otherBinding.target.id}`;},true)).snapshot,null);
    // B04 expectations authored from this fixture's CSS, not candidate output.
    const selection=ids('left','icon','label','value','empty','secret');
    const small=await observe('B04-text16-en',selection);const labelId=await backend('label');
    const valueId=await backend('value'),emptyId=await backend('empty'),secretId=await backend('secret');
    function values(o){assert.equal(property(dom(o.snapshot,valueId),'value').value.value,'London');assert.equal(property(dom(o.snapshot,emptyId),'value').value.value,'');assert.equal(property(dom(o.snapshot,secretId),'value').availability,'redacted');}
    values(small);await measure(small,dom(small.snapshot,labelId),'width',32);await measure(small,dom(small.snapshot,labelId),'height',16);
    await page.evaluate(()=>document.getElementById('label').style.fontSize='24px');environment='w04-800x600-font24-en';
    const large=await observe('B04-text24-en',selection);values(large);await measure(large,dom(large.snapshot,labelId),'width',48);await measure(large,dom(large.snapshot,labelId),'height',24);
    await diff(small,large,dom(small.snapshot,labelId),{dx:0,dy:-4,dwidth:16,dheight:8});
    await page.evaluate(()=>{document.documentElement.lang='fr';document.getElementById('label').textContent='Appliquer';});environment='w04-800x600-font24-fr';
    const french=await observe('B04-locale-fr',selection);values(french);assert.equal(property(ax(french.snapshot,dom(french.snapshot,leftId)),'accessibility_name').value.value,'Appliquer');
    await diff(large,french,dom(large.snapshot,labelId),{dx:0,dy:0,dwidth:0,dheight:0});
    const raw=await call(['diff','--before',large.file,'--after',french.file,'--max-input-bytes','262144','--max-output-bytes','262144','--max-entries','256','--json']);
    assert.equal(raw.code,0);assert(JSON.parse(raw.output).entries.some(e=>e.field==='accessibility_name'&&e.content_changed));checks.push({name:'locale-raw-diff',exit:0});
    // Fresh same-session saved inputs keep original clocks/context per observation.
    const responsiveId=await backend('responsive');environment='w04-800x600-font24-fr';
    const beforeResize=await observe('B04-before-resize',ids('responsive'));near(rect(geometry(dom(beforeResize.snapshot,responsiveId))),[660,120,100,30]);
    await page.setViewportSize({width:1000,height:600});environment='w04-1000x600-font24-fr';
    const resized=await observe('B04-resized',ids('responsive'));near(rect(geometry(dom(resized.snapshot,responsiveId))),[860,120,100,30]);
    await diff(beforeResize,resized,dom(beforeResize.snapshot,responsiveId),{dx:200,dy:0,dwidth:0,dheight:0});
    await page.evaluate(()=>scrollTo(0,100));environment='w04-1000x600-scroll100-font24-fr';
    const scrolled=await observe('B04-scrolled',ids('responsive'));near(rect(geometry(dom(scrolled.snapshot,responsiveId))),[860,20,100,30]);
    await diff(resized,scrolled,dom(resized.snapshot,responsiveId),{dx:0,dy:0,dwidth:0,dheight:0});
    assert.equal(geometry(dom(scrolled.snapshot,responsiveId)).transform.transform.affine[5],100);
    await page.evaluate(()=>{const old=document.getElementById('left');old.replaceWith(old.cloneNode(true));});
    assert.equal((await observe('B01-stale-remount')).snapshot,null);
    assert.equal((await observe('B01-stale-source-ref',references)).snapshot,null);
    const oldLeft=leftId;leftId=await backend('left');assert.notEqual(leftId,oldLeft);assert((await observe('B01-fresh-remount')).snapshot);
    await page.reload({timeout:3000});assert.equal((await observe('B01-stale-navigation',rooted(),()=>{},true)).snapshot,null);
    ({target,surface,document}=await binding(cdp));leftId=await backend('left');environment='w04-new-document';
    assert((await observe('B01-fresh-navigation')).snapshot);assert.equal(await other.evaluate(()=>document.body.innerHTML),otherState);
    assert.equal((await observe('bounded-refusal',{...rooted(),max_visited_nodes:1})).snapshot,null);
    for(const o of [initial,small,large,french,resized,scrolled])assert.equal(await fs.readFile(o.file,'utf8'),o.result.output);
    await cdp.detach();await otherCdp.detach();
    report={status:'passed',chromium:browser.version(),node:process.version,checks};
  }finally{
    if(child&&child.exitCode===null&&child.signalCode===null)child.kill('SIGKILL');
    if(closed)await bounded(closed,2000);
    try{if(context)await bounded(context.close(),3000);}finally{
      try{if(browser)await bounded(browser.close(),3000);}finally{
        try{if(server){await bounded(server.close(),4000);assert(server.process().exitCode!==null||server.process().signalCode!==null);}
          if(profile)await assert.rejects(fs.stat(profile),{code:'ENOENT'});workersGone();}finally{
          if(fixture)await bounded(new Promise(resolve=>fixture.close(resolve)),3000);
          for(const file of files)await fs.unlink(file);
          await fs.rmdir(temp);await assert.rejects(fs.stat(temp),{code:'ENOENT'});
        }
      }
    }
  }
  console.log(JSON.stringify({...report,cleanup:{cli_workers:'reaped',context:'closed',browser:'exited',profile:'removed',server:'closed',nonimage_temp:'removed'},images_created:0}));
}
main().catch(error=>{console.error(error.stack);process.exitCode=1;});
