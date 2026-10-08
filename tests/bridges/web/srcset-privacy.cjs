// W06 affected srcset privacy proof from Q01's finite reproducer. Uses its unchanged
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
const {compare}=require('./fidelity.cjs');
async function main(){
  assert(process.argv.includes('--run-authorized')&&process.env.UIB_WEB_LIVE_ALLOW==='1');
  const executable=process.env.UIB_W06_EXECUTABLE;assert(path.isAbsolute(executable||''));assert.equal(hash(executable),process.env.UIB_W06_EXECUTABLE_SHA256);
  const setup=prepare(),output=fs.mkdtempSync(path.join(fs.realpathSync(os.tmpdir()),'uib-w06-srcset-proof-'));
  const report={source_pin:process.env.UIB_W06_PIN,executable_sha256:hash(executable),driver_sha256:hash(__filename),checks:[],closures:[],output,retention:'Q02 consumes canonical baseline and report; run-owned nonimages retained until that review. No images.'};
  let fixture,server,browser,context,driver;let counter=0,diagnosticCanary=false,diagnosticTail='';
  const save=(name,data)=>{const file=path.join(output,name);fs.writeFileSync(file,typeof data==='string'?data:JSON.stringify(data),{flag:'wx',mode:0o600});return file;};
  async function closeDriver(){const closed=await driver.close();report.closures.push(closed);driver=null;assert.equal(diagnosticCanary,false,'no canary in caller diagnostics');report.diagnostic_canary=false;}
  try{
    fixture=await start();server=await setup.chromium.launchServer({headless:true,timeout:8000,args:['--remote-debugging-port=0','--remote-debugging-address=127.0.0.1']});
    const profile=server.process().spawnargs.find(v=>v.startsWith('--user-data-dir=')).slice(16);
    const port=Number(fs.readFileSync(path.join(profile,'DevToolsActivePort'),'utf8').split('\n')[0]);
    browser=await setup.chromium.connect(server.wsEndpoint(),{timeout:3000});assert.equal(browser.version(),'145.0.7632.6');
    context=await browser.newContext({viewport:{width:800,height:600},deviceScaleFactor:1});
    await context.route('**/*',r=>new URL(r.request().url()).origin===fixture.url&&!r.request().url().includes('W06_PRIVATE_CANARY')?r.continue():r.abort());
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
    let activeSurfaces,sessionId;
    async function attach(surfaces=frames){
      activeSurfaces=surfaces;
      sessionId='w06-'+crypto.randomUUID();
      const descriptor={schema_version:'0.1.0',artifact:{kind:'session',data:{session_id:sessionId,plugin,supported_versions:['0.1.0'],target,surfaces,allowed_scopes:['full','left'],capabilities:[{channel:'external_semantics',operation:'observe',status:'partial',reason:'explicit-f01'}]}}};
      driver=client(executable,save(`config-${counter++}.json`,{descriptor,provider:{backend:'web',setup:setupConfig}}));assert.equal((await driver.next()).kind,'attached');
      driver.child.stderr.on('data',bytes=>{const text=diagnosticTail+bytes.toString();diagnosticCanary ||= text.includes('W06_PRIVATE_CANARY');diagnosticTail=text.slice(-64);});
    }
    const state=()=>page.evaluate(()=>({active:document.activeElement?.id,scroll:[scrollX,scrollY],state:window.f01.checkpoint()}));
    async function observe(name,selection={selection:'documents',documents:docs,max_visited_nodes:128},fields=['value','layout_bounds'],mutate=()=>{}){
      const ctx={schema_version:'0.1.0',session_id:sessionId,target,surfaces:activeSurfaces,scope_id:selection.selection==='documents'?'full':'left',projection:'design',fields,plugin,environment_revision:'f01-800x600-dpr1'};
      const request={schema_version:'0.1.0',artifact:{kind:'request',data:{request_id:`w06-${counter++}`,clock_domain:'rebound',context:ctx,
        limits:{max_elements:128,max_depth:16,max_output_bytes:524288,deadline_ms:2000},freshness_policy:'current_required',operation:{operation:'observe',channels:['external_semantics']}}}};
      mutate(request);const before=await state();driver.send({request,selection});const sample=await driver.next();assert.equal(sample.kind,'sample');assert.deepEqual(await state(),before);
      const snapshot=sample.frames.length?JSON.parse(sample.frames[0].canonical).artifact.data.result.data:null;
      report.checks.push({name,terminal:sample.terminal,committed:sample.committed,bytes:Buffer.byteLength(sample.frames[0]?.canonical||''),domain_usage:sample.domain_usage});
      if(snapshot){assert.equal(sample.terminal,'Completed');assert.deepEqual(snapshot.context,ctx);assert(snapshot.observations.every(o=>o.freshness==='current'&&o.answer_source==='live'));}
      assert(!JSON.stringify(sample).includes('W06_PRIVATE_CANARY'));return {snapshot,sample,request};
    }
    await attach();
    const baseline=await observe('original97-baseline');report.coverage=compare(raw,baseline.snapshot);assert.equal(report.coverage.nodes,97);
    const cases=require('../../../plugins/web/tests/fixtures/collector/srcset-cases.json');
    const selected=['private-tight-relative-credential','private-tight-absolute-credential','private-third-candidate',
      'private-after-tab','private-after-data-comma','private-token-tight',
      'safe-tight-density','safe-data-base64','safe-data-internal-commas'];
    await page.evaluate(()=>{const image=document.createElement('img');image.id='w06-srcset';image.width=1;image.height=1;image.src='/public.png';document.body.append(image);});
    for(const name of selected){
      const item=cases.find(c=>c.name===name);assert(item);
      const srcset=item.value.replaceAll('SYNTHETIC_SRCSET_CANARY','W06_PRIVATE_CANARY');
      await page.evaluate(value=>document.getElementById('w06-srcset').srcset=value,srcset);
      // Wait only for the controlled currentSrc precondition, never a sampling cadence.
      const first=srcset.split(/[\t\n\f\r ]/)[0];
      await page.waitForFunction(value=>{const image=document.getElementById('w06-srcset');return image.currentSrc===new URL(value,location.href).href&&image.complete;},first,{timeout:2000});
      const result=await observe(name);
      if(item.private){
        assert.equal(result.sample.terminal,'Failed(InvalidInput)');assert.equal(result.sample.committed,0);assert.equal(result.sample.frames.length,0);
      }else{
        assert(result.snapshot);const raw=await cdp.send('DOMSnapshot.captureSnapshot',{computedStyles:[],includeDOMRects:true});compare(raw,result.snapshot);
        const fact=result.snapshot.nodes.flatMap(n=>n.extensions).find(e=>e.name.endsWith('.value')&&e.property.state.value?.value===srcset);
        assert(fact,'safe native srcset preserved verbatim');
      }
    }
    await closeDriver();
    report.status='passed';report.chromium=browser.version();report.node=process.version;
  }catch(e){report.failure=e.stack;throw e;}
  finally{
    try{if(driver)await closeDriver();}
    finally{try{if(context)await context.close();}finally{try{if(browser)await browser.close();}finally{try{if(server)await server.close();}finally{if(fixture)await fixture.close();save('report.json',report);console.log(JSON.stringify(report));}}}}
  }
}
if(require.main===module)main().catch(e=>{console.error(e.stack);process.exitCode=1;});
module.exports={compare};
