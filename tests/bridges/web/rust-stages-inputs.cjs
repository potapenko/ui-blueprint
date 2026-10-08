// Two explicit read-only F01 inputs for offline Rust stage diagnostics, not latency cohorts.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),os=require('node:os'),crypto=require('node:crypto');
const {prepare}=require('./fixture-host.cjs'),{start}=require('../../../fixtures/web/server.cjs');
const {client,setup,check}=require('./performance.cjs');
const hash=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
async function main(){
  assert(process.argv.includes('--run-authorized'));
  const pins=JSON.parse(fs.readFileSync(process.env.UIB_Q02_STAGE_PINS));
  assert.equal(pins.functional_pin,'a7c04164df08441cfbbaa61b501aa64d29290732');
  assert.equal(hash(pins.executable),pins.executable_sha256);assert.equal(hash(pins.worker),pins.worker_sha256);
  const output=process.env.UIB_Q02_STAGE_INPUTS;
  assert(path.resolve(output).startsWith(fs.realpathSync(os.tmpdir())+path.sep));fs.mkdirSync(output,{mode:0o700});
  const source=process.env.UIB_Q02_STAGE_SOURCE;assert(path.isAbsolute(source));
  const getter=fs.readFileSync(path.join(source,'plugins/web/src/collector/read-node.js'),'utf8');
  const report={source_pin:pins.functional_pin,getter_sha256:crypto.createHash('sha256').update(getter).digest('hex'),cases:[],closures:[],purpose:'saved input acquisition only; no new latency cohort'};
  const save=(name,value)=>{const file=path.join(output,name);fs.writeFileSync(file,typeof value==='string'?value:JSON.stringify(value),{flag:'wx',mode:0o600});return file;};
  let fixture,server,browser,context,driver,cdp;
  try{
    const tools=prepare();fixture=await start();
    server=await tools.chromium.launchServer({headless:true,timeout:8000,args:['--remote-debugging-port=0','--remote-debugging-address=127.0.0.1']});
    browser=await tools.chromium.connect(server.wsEndpoint(),{timeout:3000});assert.equal(browser.version(),'145.0.7632.6');
    const profile=server.process().spawnargs.find(v=>v.startsWith('--user-data-dir=')).slice(16);
    const port=Number(fs.readFileSync(path.join(profile,'DevToolsActivePort'),'utf8').split('\n')[0]);
    context=await browser.newContext({viewport:{width:800,height:600},deviceScaleFactor:1});
    await context.route('**/*',r=>new URL(r.request().url()).origin===fixture.url?r.continue():r.abort());
    const page=await context.newPage();await page.goto(fixture.url+'/?generation=1',{timeout:3000});
    await page.waitForFunction(()=>!!window.f01,{},{timeout:3000});cdp=await context.newCDPSession(page);
    const {targetInfo}=await cdp.send('Target.getTargetInfo'),{frameTree}=await cdp.send('Page.getFrameTree');
    const binding={endpoint:`ws://127.0.0.1:${port}/devtools/page/${targetInfo.targetId}`,target:{id:targetInfo.targetId,generation:crypto.randomUUID()},surface:{id:frameTree.frame.id,generation:frameTree.frame.loaderId}};
    const before=await page.evaluate(()=>({state:window.f01.checkpoint(),active:document.activeElement?.id,scroll:[scrollX,scrollY]}));
    const {executionContextId}=await cdp.send('Page.createIsolatedWorld',{frameId:binding.surface.id,worldName:'q02-stage-inputs',grantUniveralAccess:false});
    const doc=await cdp.send('Runtime.evaluate',{expression:'document',contextId:executionContextId,objectGroup:'q02-stage-inputs'});
    const {node:documentNode}=await cdp.send('DOM.describeNode',{objectId:doc.result.objectId,depth:0});
    for(const kind of ['semantic','geometry']){
      const ctx={schema_version:'0.1.0',session_id:'q02-stages-'+crypto.randomUUID(),target:binding.target,surfaces:[binding.surface],scope_id:'f01-left',projection:'interaction',
        fields:kind==='semantic'?['role','accessibility_name','enabled','focused','invalid']:['layout_bounds'],plugin:{id:'web',version:'0.1.0'},environment_revision:'f01-800x600-dpr1'};
      const descriptor={schema_version:'0.1.0',artifact:{kind:'session',data:{session_id:ctx.session_id,plugin:ctx.plugin,supported_versions:['0.1.0'],target:ctx.target,surfaces:ctx.surfaces,allowed_scopes:[ctx.scope_id],capabilities:[{channel:'external_semantics',operation:'observe',status:'partial',reason:'saved-stage-input'}]}}};
      driver=client(pins.executable,save(kind+'-config.json',{descriptor,provider:{backend:'web',setup:setup(binding)}}));assert.equal((await driver.next()).kind,'attached');
      const request={schema_version:'0.1.0',artifact:{kind:'request',data:{request_id:'q02-stage-'+kind,clock_domain:'rebound',context:ctx,limits:{max_elements:32,max_depth:8,max_output_bytes:65536,deadline_ms:250},freshness_policy:'current_required',operation:{operation:'observe',channels:['external_semantics']}}}};
      driver.send({request,selection:{selection:'initial',ids:[{id:'left',sensitivity:'public'}],max_visited_nodes:256}});
      const sample=await driver.next(),checked=check(sample,ctx,kind,request.artifact.data.request_id,new Set());
      const dom=checked.snapshot.nodes.find(n=>n.key.namespace==='web.dom'),backend=Number(dom.key.key);
      request.artifact.data.clock_domain=checked.snapshot.observations[0].clock_domain;
      const resolved=await cdp.send('DOM.resolveNode',{backendNodeId:backend,executionContextId,objectGroup:'q02-stage-inputs'});
      const raw=await cdp.send('Runtime.callFunctionOn',{objectId:resolved.object.objectId,functionDeclaration:getter,returnByValue:true,silent:true,userGesture:false,awaitPromise:true,throwOnSideEffect:false,
        arguments:[{value:{fields:ctx.fields,maxChars:600,sensitive:false,remainingMs:250}},{objectId:doc.result.objectId},{objectId:resolved.object.objectId}]});
      assert(!raw.exceptionDetails && raw.result.value.connected && raw.result.value.sameDocument);
      const ax=kind==='semantic'?await cdp.send('Accessibility.getPartialAXTree',{backendNodeId:backend,fetchRelatives:false}):null;
      if(ax){assert.equal(ax.nodes.length,1);assert.equal(ax.nodes[0].nodeId,checked.snapshot.nodes.find(n=>n.key.namespace==='web.ax').key.key);}
      save(kind+'-request.json',request);save(kind+'-canonical.json',sample.frames[0].canonical);
      save(kind+'-input.json',{backend,document_backend:documentNode.backendNodeId,world:executionContextId,dom:raw.result.value,ax:ax?.nodes[0]??null});
      report.cases.push({kind,nodes:checked.nodes,bytes:checked.bytes});report.closures.push(await driver.close());driver=null;
    }
    assert.deepEqual(await page.evaluate(()=>({state:window.f01.checkpoint(),active:document.activeElement?.id,scroll:[scrollX,scrollY]})),before);
    await cdp.send('Runtime.releaseObjectGroup',{objectGroup:'q02-stage-inputs'});report.status='passed';
  }finally{
    let cleanupError;
    for(const close of [async()=>{if(driver)report.closures.push(await driver.close());},
      async()=>{if(cdp)await cdp.detach();},async()=>{if(context)await context.close();},
      async()=>{if(browser)await browser.close();},async()=>{if(server)await server.close();},async()=>{if(fixture)await fixture.close();}]){
      try{await close();}catch(error){cleanupError??=error;}
    }
    if(cleanupError)report.cleanup_confirmed=false;
    save('acquisition.json',report);
    if(cleanupError)throw cleanupError;
  }
  console.log(JSON.stringify({output,status:report.status,cases:report.cases}));
}
if(require.main===module)main().catch(e=>{console.error(e.message);process.exitCode=1;});
