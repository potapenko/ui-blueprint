// W06 finite quality proof, not the Q02 timing campaign. Uses its unchanged
// generic guarded-host consumer; no product collection logic lives in this file.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),os=require('node:os'),crypto=require('node:crypto');
const {prepare}=require('./fixture-host.cjs');
const {start}=require('../../../fixtures/web/server.cjs');
const {client}=require('./performance.cjs');
const hash=file=>crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex');
const known=p=>{assert(p&&p.state.availability==='known');return p.state.value.value;};
const property=(n,f)=>n.properties.find(p=>p.field===f);
const fact=(n,f)=>n.extensions.find(e=>e.name===f)?.property;
const rect=g=>{const r=g.shape.value;return[r.x,r.y,r.width,r.height];};
function compare(raw,snapshot){
  const strings=raw.strings;
  assert.equal(snapshot.nodes.length,raw.documents.reduce((n,d)=>n+d.nodes.backendNodeId.length,0));
  assert.equal(snapshot.surface_records.length,raw.documents.length);
  assert.equal(snapshot.coverage.status,'partial');assert.equal(snapshot.coverage.omitted_count,0);
  let fields=0,boxes=0;
  for(const d of raw.documents){
    const n=d.nodes,frame=strings[d.frameId],get=i=>snapshot.nodes.find(v=>v.key.namespace==='web.dom'&&v.key.key===String(n.backendNodeId[i]));
    for(let i=0;i<n.backendNodeId.length;i++){
      const node=get(i);assert(node);assert.equal(node.surface.id,frame);
      assert.equal(node.native_role.value.value,strings[n.nodeName[i]]);
      assert.equal(known(property(node,'value')),strings[n.nodeValue[i]]);
      assert.equal(known(fact(node,'nodeType')),n.nodeType[i]);fields+=3;
      assert.deepEqual(node.children.map(c=>c.key),n.parentIndex.flatMap((p,j)=>p===i?[String(n.backendNodeId[j])]:[]));
      for(let a=0;a<n.attributes[i].length;a+=2){
        assert.equal(known(fact(node,`attribute.${a/2}.name`)),strings[n.attributes[i][a]]);
        assert.equal(known(fact(node,`attribute.${a/2}.value`)),strings[n.attributes[i][a+1]]);fields+=2;
      }
      for(const name of ['shadowRootType','textValue','inputValue','pseudoType','pseudoIdentifier','currentSourceURL','originURL']){
        const at=n[name].index.indexOf(i);
        if(at>=0){assert.equal(known(fact(node,name)),strings[n[name].value[at]]);fields++;}
        else assert.equal(fact(node,name),undefined);
      }
      for(const name of ['inputChecked','optionSelected','isClickable']){assert.equal(known(fact(node,name)),n[name].index.includes(i));fields++;}
      const l=d.layout.nodeIndex.indexOf(i);
      if(l<0){assert.equal(property(node,'layout_bounds').state.availability,'unknown');continue;}
      const geometry=known(property(node,'layout_bounds'));assert.equal(geometry.coordinate_space.units,'css_px');assert.equal(geometry.coordinate_space.kind,'document');
      assert.deepEqual(rect(geometry),d.layout.bounds[l]);fields++;
      assert.equal(known(fact(node,'layout.text')),strings[d.layout.text[l]]);
      assert.equal(known(fact(node,'layout.stackingContext')),d.layout.stackingContexts.index.includes(l));fields+=2;
      for(const [suffix,name]of [['offsetRect','offsetRects'],['clientRect','clientRects'],['scrollRect','scrollRects']]){
        const p=fact(node,`layout.${suffix}`),r=d.layout[name][l];
        if(r.length)assert.deepEqual(rect(known(p)),r);else assert.equal(p.state.availability,'unknown');fields++;
      }
      let ordinal=0;
      for(let b=0;b<d.textBoxes.layoutIndex.length;b++)if(d.textBoxes.layoutIndex[b]===l){
        assert.deepEqual(rect(known(fact(node,`textBox.${ordinal}.bounds`))),d.textBoxes.bounds[b]);
        assert.equal(known(fact(node,`textBox.${ordinal}.startUtf16`)),d.textBoxes.start[b]);
        assert.equal(known(fact(node,`textBox.${ordinal}.lengthUtf16`)),d.textBoxes.length[b]);ordinal++;boxes++;fields+=3;
      }
    }
    for(const name of ['documentURL','title','baseURL','contentLanguage','encodingName','publicId','systemId']){assert.equal(known(fact(get(0),name)),strings[d[name]]);fields++;}
    for(const name of ['scrollOffsetX','scrollOffsetY','contentWidth','contentHeight']){assert.equal(known(fact(get(0),name)),d[name]);fields++;}
    for(let i=0;i<n.contentDocumentIndex.index.length;i++){
      const owner=String(n.backendNodeId[n.contentDocumentIndex.index[i]]),child=String(raw.documents[n.contentDocumentIndex.value[i]].nodes.backendNodeId[0]);
      assert(snapshot.relations.some(r=>r.kind==='owns'&&r.from.key===owner&&r.to.key===child));
    }
  }
  for(const node of snapshot.nodes)for(const p of [...node.properties,...node.extensions.map(e=>e.property)]){
    assert.equal(p.evidence.provenance,'reported');assert.equal(p.evidence.source_namespace,'web.dom');
    assert(snapshot.observations.some(o=>o.id===p.evidence.observation_id));
  }
  return {nodes:snapshot.nodes.length,documents:raw.documents.length,fields,boxes};
}
async function main(){
  assert(process.argv.includes('--run-authorized')&&process.env.UIB_WEB_LIVE_ALLOW==='1');
  const executable=process.env.UIB_W06_EXECUTABLE;assert(path.isAbsolute(executable||''));assert.equal(hash(executable),process.env.UIB_W06_EXECUTABLE_SHA256);
  const setup=prepare(),output=fs.mkdtempSync(path.join(fs.realpathSync(os.tmpdir()),'uib-w06-proof-'));
  const report={source_pin:process.env.UIB_W06_PIN,executable_sha256:hash(executable),checks:[],output,retention:'Q02 consumes canonical baseline and report; run-owned nonimages retained until that review. No images.'};
  let fixture,server,browser,context,driver;let counter=0;
  const save=(name,data)=>{const file=path.join(output,name);fs.writeFileSync(file,typeof data==='string'?data:JSON.stringify(data),{flag:'wx',mode:0o600});return file;};
  try{
    fixture=await start();server=await setup.chromium.launchServer({headless:true,timeout:8000,args:['--remote-debugging-port=0','--remote-debugging-address=127.0.0.1']});
    const profile=server.process().spawnargs.find(v=>v.startsWith('--user-data-dir=')).slice(16);
    const port=Number(fs.readFileSync(path.join(profile,'DevToolsActivePort'),'utf8').split('\n')[0]);
    browser=await setup.chromium.connect(server.wsEndpoint(),{timeout:3000});assert.equal(browser.version(),'145.0.7632.6');
    context=await browser.newContext({viewport:{width:800,height:600},deviceScaleFactor:1});
    await context.route('**/*',r=>new URL(r.request().url()).origin===fixture.url?r.continue():r.abort());
    const page=await context.newPage();await page.goto(fixture.url+'/?generation=1',{timeout:3000});await page.waitForFunction(()=>!!window.f01,{},{timeout:3000});
    const cdp=await context.newCDPSession(page),raw=await cdp.send('DOMSnapshot.captureSnapshot',{computedStyles:[],includeDOMRects:true});
    const {targetInfo}=await cdp.send('Target.getTargetInfo'),{frameTree}=await cdp.send('Page.getFrameTree');
    const frames=[frameTree,...(frameTree.childFrames||[])].map(t=>({id:t.frame.id,generation:t.frame.loaderId}));assert.equal(frames.length,2);
    const docs=frames.map(surface=>({surface,document_backend_id:raw.documents.find(d=>raw.strings[d.frameId]===surface.id).nodes.backendNodeId[0],sensitivity:'public'}));
    const target={id:targetInfo.targetId,generation:crypto.randomUUID()},plugin={id:'web',version:'0.1.0'};
    const setupConfig={endpoint:`ws://127.0.0.1:${port}/devtools/page/${target.id}`,cdp_session_id:null,surface:frames[0],
      transport:{endpoint_bytes:1024,handshake_bytes:2048,read_buffer_bytes:64,write_buffer_bytes:64,write_buffer_max:32768,frame_bytes:32768,message_bytes:32768,outbound_bytes:16384},
      cdp:{max_request_bytes:16384,max_message_bytes:32768,max_metadata_bytes:256,max_results:1,result_bytes:32768,max_events:4,event_bytes:34000},
      collector:{max_nodes:128,max_methods:100,max_reply_bytes:32768,max_total_reply_bytes:262144,max_text_bytes:16384,max_handle_bytes:256,max_ax_properties:32,io_read_bytes:65536,io_write_bytes:32768,io_work:8192}};
    let activeSurfaces;
    async function attach(surfaces=frames){
      activeSurfaces=surfaces;
      const descriptor={schema_version:'0.1.0',artifact:{kind:'session',data:{session_id:'w06',plugin,supported_versions:['0.1.0'],target,surfaces,allowed_scopes:['full','left'],capabilities:[{channel:'external_semantics',operation:'observe',status:'partial',reason:'explicit-f01'}]}}};
      driver=client(executable,save(`config-${counter++}.json`,{descriptor,provider:{backend:'web',setup:setupConfig}}));assert.equal((await driver.next()).kind,'attached');
    }
    const state=()=>page.evaluate(()=>({active:document.activeElement?.id,scroll:[scrollX,scrollY],state:window.f01.checkpoint()}));
    async function observe(name,selection={selection:'documents',documents:docs,max_visited_nodes:128},fields=['value','layout_bounds'],mutate=()=>{}){
      const ctx={schema_version:'0.1.0',session_id:'w06',target,surfaces:activeSurfaces,scope_id:selection.selection==='documents'?'full':'left',projection:'design',fields,plugin,environment_revision:'f01-800x600-dpr1'};
      const request={schema_version:'0.1.0',artifact:{kind:'request',data:{request_id:`w06-${counter++}`,clock_domain:'rebound',context:ctx,
        limits:{max_elements:128,max_depth:16,max_output_bytes:524288,deadline_ms:2000},freshness_policy:'current_required',operation:{operation:'observe',channels:['external_semantics']}}}};
      mutate(request);const before=await state();driver.send({request,selection});const sample=await driver.next();assert.equal(sample.kind,'sample');assert.deepEqual(await state(),before);
      const snapshot=sample.frames.length?JSON.parse(sample.frames[0].canonical).artifact.data.result.data:null;
      report.checks.push({name,terminal:sample.terminal,committed:sample.committed,bytes:sample.frames[0]?.canonical.length||0,domain_usage:sample.domain_usage});
      if(snapshot){assert.equal(sample.terminal,'Completed');assert.deepEqual(snapshot.context,ctx);assert(snapshot.observations.every(o=>o.freshness==='current'&&o.answer_source==='live'));}
      assert(!JSON.stringify(sample).includes('W06_PRIVATE_CANARY'));return {snapshot,sample,request};
    }
    await attach();
    save('raw-baseline.json',raw);
    const baseline=await observe('full-baseline');save('request-baseline.json',baseline.request);assert(baseline.snapshot);report.coverage=compare(raw,baseline.snapshot);assert.equal(report.coverage.nodes,97);
    save('canonical-baseline.json',baseline.sample.frames[0].canonical);
    await page.evaluate(()=>document.getElementById('left').style.width='121px');
    const changedRaw=await cdp.send('DOMSnapshot.captureSnapshot',{computedStyles:[],includeDOMRects:true});const changed=await observe('full-changed');compare(changedRaw,changed.snapshot);
    await page.evaluate(()=>document.getElementById('left').style.removeProperty('width'));
    compare(await cdp.send('DOMSnapshot.captureSnapshot',{computedStyles:[],includeDOMRects:true}),(await observe('full-restored')).snapshot);
    assert.notEqual(changed.snapshot.observations[0].id,baseline.snapshot.observations[0].id);
    for(const [name,selection,mutation]of[
      ['wrong-document',{selection:'documents',documents:docs.map((d,i)=>({...d,document_backend_id:i?d.document_backend_id:2147483647})),max_visited_nodes:128},()=>{}],
      ['wrong-loader',{selection:'documents',documents:docs.map((d,i)=>({...d,surface:i?{...d.surface,generation:'old'}:d.surface})),max_visited_nodes:128},()=>{}],
      ['missing-frame',{selection:'documents',documents:docs.slice(0,1),max_visited_nodes:128},()=>{}],
      ['node-bound',{selection:'documents',documents:docs,max_visited_nodes:96},()=>{}],
      ['depth-bound',{selection:'documents',documents:docs,max_visited_nodes:128},r=>r.artifact.data.limits.max_depth=2],
      ['output-bound',{selection:'documents',documents:docs,max_visited_nodes:128},r=>r.artifact.data.limits.max_output_bytes=4096],
      ['wrong-target',{selection:'documents',documents:docs,max_visited_nodes:128},r=>r.artifact.data.context.target={...target,generation:'foreign'}],
    ]){assert.equal((await observe(name,selection,undefined,mutation)).snapshot,null);}
    await page.evaluate(()=>{const p=document.createElement('input');p.id='w06-private';p.type='password';p.value='W06_PRIVATE_CANARY';document.body.append(p);});
    assert.equal((await observe('private-source-refusal')).snapshot,null);
    await page.evaluate(()=>document.getElementById('w06-private').remove());
    await driver.close();driver=null;
    await attach([frames[0]]);
    assert.equal((await observe('unallowed-frame')).snapshot,null);await driver.close();driver=null;
    await attach([frames[0]]);
    const semFields=['role','accessibility_name','enabled','focused','invalid'];const selected={selection:'initial',ids:[{id:'left',sensitivity:'public'}],max_visited_nodes:256};
    for(const [name,value]of [['semantic-baseline','Apply'],['semantic-changed','Changed'],['semantic-restored','Apply']]){
      await page.evaluate(v=>document.getElementById('left').setAttribute('aria-label',v),value);
      const result=await observe(name,selected,semFields);assert(result.snapshot);const ax=result.snapshot.nodes.find(n=>n.key.namespace==='web.ax');
      assert.equal(known(property(ax,'accessibility_name')),value);assert.equal(known(fact(ax,'focusable')),true);
      const dom=result.snapshot.nodes.find(n=>n.key.namespace==='web.dom');const source=await cdp.send('Accessibility.getPartialAXTree',{backendNodeId:Number(dom.key.key),fetchRelatives:false});
      assert.equal(source.nodes[0].properties.find(p=>p.name==='focusable').value.value,known(fact(ax,'focusable')));
      assert.equal(fact(ax,'focusable').evidence.source_namespace,'web.ax');
      save(`${name}.json`,result.sample.frames[0].canonical);
    }
    await driver.close();driver=null;
    report.status='passed';report.chromium=browser.version();report.node=process.version;
  }catch(e){report.failure=e.stack;throw e;}
  finally{
    try{if(driver){report.cleanup=await driver.close();driver=null;}}
    finally{try{if(context)await context.close();}finally{try{if(browser)await browser.close();}finally{try{if(server)await server.close();}finally{if(fixture)await fixture.close();save('report.json',report);console.log(JSON.stringify(report));}}}}
  }
}
if(require.main===module)main().catch(e=>{console.error(e.stack);process.exitCode=1;});
module.exports={compare};
