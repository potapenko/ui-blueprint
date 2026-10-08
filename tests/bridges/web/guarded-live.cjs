// Preparation-only until an explicit W01-live activation. No product collection here.
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const crypto = require('node:crypto');
const { spawn, execFileSync } = require('node:child_process');
const { prepare } = require('./fixture-host.cjs');
const { start } = require('../../../fixtures/web/server.cjs');
const r01 = require('../../../experiments/web/expected.json');
const f01 = require('../../../fixtures/web/expected.json');
const CANARY = 'W01_LIVE_PRIVATE_CANARY';
const MAX_LINE = 262144;
const PREFIX = '@UIB_LIVE ';
const EVIDENCE_ROOT = require('node:fs').realpathSync(require('node:os').tmpdir());
// Authored read expectations, independent of the collector result. Stimuli below
// create these states directly; they do not qualify product input delivery.
const FORM_CASES = Object.freeze({
  'form-forward': {start:1,end:3,direction:'forward',anchor:1,focus:3,output:'',private:false},
  'form-backward': {start:1,end:3,direction:'backward',anchor:3,focus:1,output:'London',private:false},
  'form-collapsed': {start:4,end:4,direction:'none',anchor:4,focus:4,output:'London',private:false},
  'form-private': {start:1,end:3,direction:'forward',output:'London',private:true}
});
const FORM_ACTION_CASES=['form-initial','form-focus-prepare','form-focus','form-focused','form-type-prepare','form-type','form-typed','form-unfocused-prepare','form-unfocused'];
const CASES = new Set([...Object.keys(FORM_CASES),'action-observe','action-prepare','action-success','action-prepare-stale','action-stale','action-readonly-observe','action-readonly-prepare','action-readonly-refused','action-unknown-observe','action-unknown-prepare','action-unknown','b05-initial','b05-retain-initial','b05-parent','b05-history-parent','b05-font','b05-history-font','rooted-current','rooted-wrong-binding','rooted-wrong-document','rooted-stale','popup-context','left-initial','sized-before','sized-after','private','cross-target','old-remount-ref','new-remount-binding','old-navigation-ref','new-document-binding']);
for(const stage of FORM_ACTION_CASES)CASES.add(stage);
const TERMINALS = new Set(['completed','cancelled','timed_out','invalid_limits','resource_limit','allocation_failure','overflow','busy','invalid_input','invalid_state','invalid_control','stale_operation','deadline_expired','permission_denied','io','worker_failed','system_allocation_failure','cleanup_pending','resync_required','action_refused']);
const FRAME_FILES = Object.freeze({'form-forward':'form-forward.json','form-backward':'form-backward.json','form-collapsed':'form-collapsed.json','action-observe':'action-observe.json','action-prepare':'action-prepared.json','action-success':'action-transition.json','b05-initial':'b05-initial.json','b05-parent':'b05-parent.json','b05-font':'b05-font.json','rooted-current':'rooted-context.json','popup-context':'popup-context.json','left-initial':'initial-left.json','sized-before':'sized-before.json','sized-after':'sized-after.json'});
function bounded(promise, ms, code) {
  let timer; return Promise.race([promise, new Promise((_, reject) => {
    timer = setTimeout(() => reject(new Error(code)), ms);
  })]).finally(() => clearTimeout(timer));
}
async function writeExclusive(file, bytes) {
  const handle=await fs.open(file,'wx',0o600); // failure here never removes an existing file
  try {await handle.writeFile(bytes);await handle.close();}
  catch(error){await handle.close().catch(()=>{});await fs.unlink(file).catch(()=>{});throw error;}
}
async function fetchJson(url) {
  const response = await fetch(url, { signal: AbortSignal.timeout(3000) });
  assert(response.ok); let size = 0; const chunks = [];
  for await (const chunk of response.body) {
    size += chunk.length; assert(size <= MAX_LINE, 'bounded endpoint inventory'); chunks.push(chunk);
  }
  return JSON.parse(Buffer.concat(chunks).toString('utf8'));
}
function property(node, field) {
  const p = node.properties.find(p => p.field === field);
  assert(p && p.selection === 'requested' && p.state.availability === 'known');
  return p.state.value;
}
function rectangle(node) {
  const p = property(node, 'layout_bounds'); assert.equal(p.type, 'geometry');
  const g = p.value; assert.equal(g.frame_kind, 'layout_bounds');
  assert.equal(g.coordinate_space.units, 'css_px'); assert.equal(g.coordinate_space.origin, 'top_left');
  assert.equal(g.transform.status, 'local_only'); assert.equal(g.shape.shape, 'rect');
  const {x,y,width,height} = g.shape.value;
  assert([x,y,width,height].every(Number.isFinite)); return [x,y,width,height];
}
function compareRect(actual, expected) {
  assert.equal(actual.length, 4);
  actual.forEach((value, i) => assert(Math.abs(value - expected[i]) <= r01.tolerance_css_px));
}
async function uiState(page, forms = false) {
  // Private canary is compared in memory only; never serialize this checkpoint to evidence.
  return page.evaluate(forms => ({form: forms ? (() => {
    const input=document.getElementById('draft'), output=document.getElementById('applied');
    return {documentFocused:document.hasFocus(),value:input.value,start:input.selectionStart,end:input.selectionEnd,
      direction:input.selectionDirection,autocomplete:input.autocomplete,output:output.value,
      outputChildren:output.childNodes.length,outputChildType:output.firstChild?.nodeType??null};
  })() : null,active: document.activeElement?.id || '',
    scroll: [scrollX, scrollY], state: window.f01 ? window.f01.checkpoint() : {target:document.getElementById('action-target').checked,duplicate:document.getElementById('action-duplicate').checked,disabled:document.getElementById('action-disabled').checked,mixedChecked:document.getElementById('action-indeterminate').checked,mixed:document.getElementById('action-indeterminate').indeterminate,custom:document.getElementById('action-custom').getAttribute('aria-checked')}}), forms);
}
function assertFormState(state, expected) {
  assert.equal(state.active,'draft');assert.equal(state.form.documentFocused,true);
  assert.equal(state.form.value,expected.private?CANARY:'A💡B');
  assert.deepEqual([state.form.start,state.form.end,state.form.direction],
    [expected.start,expected.end,expected.direction]);
  assert.equal(state.form.autocomplete,expected.private?'one-time-code':'');
  assert.equal(state.form.output,expected.output);
  assert.equal(state.form.outputChildren,expected.output?1:0);
  assert.equal(state.form.outputChildType,expected.output?3:null);
  // Direct setup never enters the fixture's input/commit controller. In
  // particular native output London is NOT its application applied state.
  assert.deepEqual([state.state.selected,state.state.applied,state.state.valid,state.state.delivered,state.state.revision],['','',false,0,0]);
}
// Trusted own-fixture connection, using the existing web_worker_data profile.
// No UI field or saved capability grants connection authority.
function cliConnection(binding, selected) {
  const mib=1048576;
  return {connection_version:'1.0.0',target:binding.target,attach_deadline_ms:2000,
    session:{session_id:'live-cli-actions',plugin:{id:'web',version:'0.1.0'},supported_versions:['0.1.0'],
      target:binding.target,surfaces:[binding.surface],allowed_scopes:[`f01-${selected}`],
      capabilities:[{channel:'external_semantics',operation:'observe',status:'partial',reason:'bounded-live-source-under-verification'}]},
    host_limits:{workers:2,worker_bytes:64*mib,publication_reserve:mib,bootstrap_bytes:mib,parent_bytes:32*mib,input_bytes:2*mib,
      ingress_bytes:512*1024,output_bytes:512*1024,request_output_bytes:2*mib,completion_groups:2,control_bytes:4096,
      cleanup_ms:1000,retained_domain_bytes:64*mib,retained_per_worker:15*mib,main_stack_bytes:8*mib,watchdog_stack_bytes:mib},
    provider:{backend:'web',selection:{selection:'initial',ids:[{id:selected,sensitivity:'public'}],max_visited_nodes:256},
      setup:{endpoint:binding.endpoint,cdp_session_id:null,surface:binding.surface,
        transport:{endpoint_bytes:1024,handshake_bytes:2048,read_buffer_bytes:64,write_buffer_bytes:64,write_buffer_max:32768,frame_bytes:8192,message_bytes:8192,outbound_bytes:16384},
        cdp:{max_request_bytes:16384,max_message_bytes:8192,max_metadata_bytes:256,max_results:1,result_bytes:8192,max_events:4,event_bytes:34000},
        collector:{max_nodes:16,max_methods:100,max_reply_bytes:8192,max_total_reply_bytes:65536,max_text_bytes:600,max_handle_bytes:256,max_ax_properties:32,io_read_bytes:16384,io_write_bytes:32768,io_work:2048}}}};
}
function cliRequest(context, operation, stage) {
  return {schema_version:'0.1.0',artifact:{kind:'request',data:{request_id:stage,clock_domain:'caller-clock-rebound-after-attach',context,
    limits:{max_elements:32,max_depth:8,max_output_bytes:65536,deadline_ms:250},freshness_policy:'current_required',operation}}};
}
function cliPrepareRequest(observed, stage) {
  assert.equal(observed.artifact.kind,'channel_response');assert.equal(observed.artifact.data.result.status,'observed');
  const s=observed.artifact.data.result.data, nodes=s.nodes.filter(n=>n.key.namespace==='web.dom');assert.equal(nodes.length,1);
  const node=nodes[0], evidence=node.properties.find(p=>p.field==='enabled').evidence;
  const unknown={availability:'unknown',reason:'not-prepared'};
  return cliRequest(s.context,{operation:'prepare',action:{id:'live-cli-set-checked',context:s.context,
    backend_ref:{session_id:s.context.session_id,target:s.context.target,surface:node.surface,key:node.key,snapshot_id:s.id,observation_id:evidence.observation_id},
    intent:{intent:'set_checked',value:true},modality:'setter',input_space:null,required_enabled:true,authorized_scope:s.context.scope_id,
    unique_match:false,resolution:{evidence,writable:unknown,value_allowed:unknown,available_intents:[]}}},stage);
}
function ownWorkerPids(worker) {
  const pattern='^'+worker.replace(/[.*+?^${}()|[\]\\]/g,'\\$&')+'( |$)';
  try {return execFileSync('/usr/bin/pgrep',['-f',pattern],{timeout:1000,maxBuffer:4096,encoding:'utf8'}).trim().split('\n').filter(Boolean).map(s=>{assert(/^\d+$/.test(s));return Number(s);});}
  catch(error){if(error.status===1)return [];throw new Error('owned_worker_inventory_failed');}
}
async function run(evidence, report) {
  const directorCase=['director','director_semantics'].includes(report.mode);
  // Check BEFORE prepare/start/launch; ordinary node execution cannot start a live run.
  if (!process.argv.includes('--run-authorized') || process.env.UIB_WEB_LIVE_ALLOW !== '1')
    throw new Error('explicit_live_activation_required');
  const executable = process.env.UIB_WEB_LIVE_TEST;
  const worker = process.env.UIB_WEB_LIVE_WORKER;
  for (const [file, expected] of [[executable, process.env.UIB_WEB_LIVE_TEST_SHA256], [worker, process.env.UIB_WEB_LIVE_WORKER_SHA256]]) {
    assert(file && path.isAbsolute(file) && /^[a-f0-9]{64}$/.test(expected || ''), 'pinned own executable required');
    assert.equal(crypto.createHash('sha256').update(await fs.readFile(file)).digest('hex'), expected, 'compiled artifact drift');
  }
  const setup = prepare(); const launchId = crypto.randomUUID();
  const r01Oracle = 'experiments/web/expected.json';
  const pinnedOracle = execFileSync('git',['show',`${setup.fixtureCommit}:${r01Oracle}`],
    {cwd:setup.root,timeout:5000,maxBuffer:128*1024});
  assert((await fs.readFile(path.join(setup.root,r01Oracle))).equals(pinnedOracle),'R01 oracle drift');
  const sourceHashes = {};
  for (const name of [...setup.fixtureFiles,r01Oracle,'tests/bridges/web/guarded-live.cjs','crates/host/tests/web_live.rs'])
    sourceHashes[name]=crypto.createHash('sha256').update(await fs.readFile(path.join(setup.root,name))).digest('hex');
  let actionsHtml;
  if(report.mode==='actions'||report.mode==='cli_actions'){
    actionsHtml=await fs.readFile(path.join(setup.root,'fixtures/web/actions.html'));
    const pinned=execFileSync('git',['show','9bd9f662d28cdb66f8b7b096fcd77a93ecf524e4:fixtures/web/actions.html'],{cwd:setup.root,timeout:5000,maxBuffer:65536});
    assert(actionsHtml.equals(pinned));sourceHashes['fixtures/web/actions.html']=crypto.createHash('sha256').update(actionsHtml).digest('hex');
  }
  let fixture, server, browser, context, child, exited, profile;
  const checks = report.checks; const before = new Map(); let failed = false;
  Object.assign(report,{sourceHashes,binaries:{test:process.env.UIB_WEB_LIVE_TEST_SHA256,worker:process.env.UIB_WEB_LIVE_WORKER_SHA256},environment:{node:process.version,playwright:setup.runtimeVersion,platform:process.platform,arch:process.arch,os_release:require('node:os').release()},fixture:setup.fixtureCommit});
  report.phase='fixture_setup';
  const whole = setTimeout(() => { failed = true; child?.kill('SIGKILL'); if(server) void server.kill().catch(()=>{}); }, 120000);
  try {
    if(report.mode==='actions'||report.mode==='cli_actions'){
      const own=require('node:http').createServer((req,res)=>{res.setHeader('Cache-Control','no-store');if(new URL(req.url,'http://127.0.0.1').pathname==='/'){res.setHeader('Content-Type','text/html; charset=utf-8');res.end(actionsHtml);}else{res.statusCode=404;res.end('Not found');}});
      own.requestTimeout=3000;own.headersTimeout=3000;
      fixture={url:null,close:()=>new Promise(resolve=>own.close(resolve))};
      await bounded(new Promise((resolve,reject)=>{own.once('error',reject);own.listen(0,'127.0.0.1',resolve);}),3000,'actions_server_timeout');
      fixture.url=`http://127.0.0.1:${own.address().port}`;
      report.fixture='9bd9f662d28cdb66f8b7b096fcd77a93ecf524e4:actions.html';
    }else if(!directorCase) fixture = await start();
    // Keep pinned Playwright launch defaults and own its actual process/profile.
    // This is a Chromium TCP CDP endpoint in addition to the fixture driver's connection.
    server = await bounded(setup.chromium.launchServer({headless:true,timeout:8000,
      args:['--remote-debugging-port=0','--remote-debugging-address=127.0.0.1']}), 9000, 'browser_launch_timeout');
    const owned = server.process(); assert(owned && owned.pid);
    const profileArg = owned.spawnargs.find(arg => arg.startsWith('--user-data-dir='));
    assert(profileArg, 'owned launch must expose its generated profile');
    profile = profileArg.slice('--user-data-dir='.length); assert(path.isAbsolute(profile));
    assert(path.basename(profile).startsWith('playwright_chromiumdev_profile-'), 'generated owned profile only');
    let port;
    const readyEnd = Date.now() + 3000;
    while (Date.now() < readyEnd) {
      try {
        const file = path.join(profile, 'DevToolsActivePort');
        assert((await fs.stat(file)).size <= 1024);
        const text = await fs.readFile(file, 'utf8'); const line = text.split('\n')[0];
        assert(/^\d+$/.test(line)); port = Number(line); assert(port > 0 && port <= 65535); break;
      } catch (error) { if (error.code !== 'ENOENT') throw error; }
      await new Promise(resolve => setTimeout(resolve, 20)); // finite launcher readiness, no UI polling
    }
    assert(port, 'direct_CDP_endpoint_unavailable');
    const http = `http://127.0.0.1:${port}`;
    browser = await bounded(setup.chromium.connect(server.wsEndpoint(), {timeout:3000}), 3500, 'driver_connect_timeout');
    report.environment.chromium=browser.version(); assert.equal(report.environment.chromium, '145.0.7632.6');
    context = await browser.newContext({viewport:directorCase?{width:1280,height:900}:{width:800,height:600},deviceScaleFactor:1});
    if(fixture)await context.route('**/*', route => new URL(route.request().url()).origin === fixture.url ? route.continue() : route.abort());
    const pages = {};
    for (const name of directorCase?['a']:['a','b']) {
      const page = await context.newPage(); page.setDefaultTimeout(2000); page.setDefaultNavigationTimeout(3000);
      if(directorCase){
        report.site_diagnostics={page_errors:0,console_errors:0,failed_first_party_requests:0,first_party_http_errors:0};
        page.on('pageerror',()=>report.site_diagnostics.page_errors++);
        page.on('console',message=>{if(message.type()==='error')report.site_diagnostics.console_errors++;});
        page.on('requestfailed',request=>{if(new URL(request.url()).origin==='http://localhost:3000')report.site_diagnostics.failed_first_party_requests++;});
        page.on('response',response=>{if(new URL(response.url()).origin==='http://localhost:3000'&&response.status()>=400)report.site_diagnostics.first_party_http_errors++;});
        await page.goto('http://localhost:3000/#/clip-search?language=en',{timeout:15000,waitUntil:'domcontentloaded'});
        await page.locator('#clip-search-filter-director-trigger').waitFor({timeout:15000});
      }else{
        await page.goto(`${fixture.url}/?generation=1`);
        if(report.mode==='actions'||report.mode==='cli_actions')await page.locator('#action-target').waitFor();else await page.waitForFunction(() => !!window.f01);
      }
      pages[name] = page;
    }
    async function binding(page) {
      const session = await context.newCDPSession(page);
      try {
        const {targetInfo} = await bounded(session.send('Target.getTargetInfo'),3000,'target_metadata_timeout');
        const {frameTree} = await bounded(session.send('Page.getFrameTree'),3000,'frame_metadata_timeout');
        const targets = await fetchJson(`${http}/json/list`);
        const matches = targets.filter(t => t.id === targetInfo.targetId && t.type === 'page');
        assert.equal(matches.length,1); const endpoint = new URL(matches[0].webSocketDebuggerUrl);
        assert.equal(endpoint.protocol,'ws:'); assert.equal(endpoint.hostname,'127.0.0.1');
        assert.equal(Number(endpoint.port),port); assert(endpoint.pathname.endsWith(`/${targetInfo.targetId}`));
        return {endpoint:endpoint.href,target:{id:targetInfo.targetId,generation:launchId},
          surface:{id:frameTree.frame.id,generation:frameTree.frame.loaderId}};
      } finally { await session.detach(); }
    }
    async function callCli(stage,args,expectedExit,maxOutput=65536,jsonOutput=true){
      report.last_stage=stage;
      const started=performance.now();
      child=spawn(executable,args,{stdio:['ignore','pipe','pipe']});
      const chunks=[];let bytes=0,errors=0,over=false,diagnostic='';
      child.stdout.on('data',chunk=>{bytes+=chunk.length;if(bytes>maxOutput){over=true;child.kill('SIGKILL');}else chunks.push(chunk);});
      child.stderr.on('data',chunk=>{errors+=chunk.length;if(errors>65536){over=true;child.kill('SIGKILL');}else if(errors<=128)diagnostic+=chunk.toString('utf8');});
      exited=new Promise((resolve,reject)=>{child.once('error',reject);child.once('close',(code,signal)=>resolve({code,signal}));});
      const exit=await bounded(exited,5000,'cli_call_timeout');
      const stdout=Buffer.concat(chunks);const pids=ownWorkerPids(worker);
      report.outcomes.push({stage,exit:exit.code,signal:exit.signal,stdout_bytes:stdout.length,stderr_bytes:errors,elapsed_ms:performance.now()-started,owned_worker_pids_after:pids});
      // Public CLI diagnostics are fixed codes. Never retain arbitrary stderr/UI.
      if(/^(observe_invalid_or_limit|observe_unavailable|observe_incomplete|observe_worker_or_cleanup_failure)\n$/.test(diagnostic))report.outcomes.at(-1).diagnostic=diagnostic.trim();
      assert(!over);assert.equal(exit.signal,null);assert.deepEqual(pids,[]);
      assert(stdout.length>0&&stdout.at(-1)===10);
      if(jsonOutput)assert.equal(stdout.toString('utf8').split('\n').length,2,'one complete JSON line');
      const document=jsonOutput?JSON.parse(stdout.toString('utf8')):null;
      const name=`${stage}.${jsonOutput?'json':'txt'}`;await writeExclusive(path.join(evidence,name),stdout);
      report.frames.push({case:stage,file:name,bytes:stdout.length,sha256:crypto.createHash('sha256').update(stdout).digest('hex'),usage:'historical_analysis_only',live_ref_reuse:false});
      assert.equal(exit.code,expectedExit); // retain a valid canonical failure before stopping
      return {bytes:stdout,document};
    }
    if(report.mode==='first_use'){
      report.phase='developer_example';
      const page=pages.a,actual=await binding(page),other=await binding(pages.b);
      const session=await context.newCDPSession(page);let documentBackend,rootBackend;
      try{
        const {root}=await session.send('DOM.getDocument',{depth:0,pierce:false});documentBackend=root.backendNodeId;
        const found=await session.send('DOM.querySelectorAll',{nodeId:root.nodeId,selector:'#f01'});assert.equal(found.nodeIds.length,1);
        const {node}=await session.send('DOM.describeNode',{nodeId:found.nodeIds[0],depth:0,pierce:false});rootBackend=node.backendNodeId;
      }finally{await session.detach();}
      const helper=path.join(setup.root,'tests/bridges/web/geometry.cjs');
      report.sourceHashes['tests/bridges/web/geometry.cjs']=crypto.createHash('sha256').update(await fs.readFile(helper)).digest('hex');
      const args={'--cli':executable,'--worker':worker,'--endpoint':actual.endpoint,'--target-id':actual.target.id,
        '--frame-id':actual.surface.id,'--loader-id':actual.surface.generation,'--document-backend-id':String(documentBackend),'--root-backend-id':String(rootBackend)};
      async function invoke(stage,overrides,expectedExit){
        const before=await uiState(page),started=performance.now();
        child=spawn(process.execPath,[helper,...Object.entries({...args,...overrides}).flat()],{stdio:['ignore','pipe','pipe']});
        const stdout=[],stderr=[];let size=0;
        const capture=chunks=>chunk=>{size+=chunk.length;if(size>65536)child.kill('SIGKILL');else chunks.push(chunk);};
        child.stdout.on('data',capture(stdout));child.stderr.on('data',capture(stderr));
        exited=new Promise((resolve,reject)=>{child.once('error',reject);child.once('close',(code,signal)=>resolve({code,signal}));});
        const result=await bounded(exited,20000,'example_timeout');
        assert.equal(result.signal,null);assert.equal(result.code,expectedExit);assert(size<=65536);
        const bytes=Buffer.concat(stdout),diagnostic=Buffer.concat(stderr).toString('utf8');
        assert(!diagnostic||/^(target_unresolved|stale_document)\n$/.test(diagnostic));
        assert.deepEqual(await uiState(page),before);assert.deepEqual(ownWorkerPids(worker),[]);
        report.outcomes.push({stage,exit:result.code,stdout_bytes:bytes.length,diagnostic:diagnostic.trim(),elapsed_ms:performance.now()-started});
        return bytes;
      }
      const output=await invoke('example-current',{},4);
      const lines=output.toString('utf8').split('\n');
      const width=JSON.parse(lines.find(line=>line.startsWith('width: ')).slice(7));
      const height=JSON.parse(lines.find(line=>line.startsWith('height: ')).slice(8));
      assert.equal(width.status,'known');assert.equal(height.status,'known');
      assert.equal(width.measurement.value.value.amount,360); // existing authored F01 wrapper literal
      assert.equal(width.measurement.space.units,'css_px');
      await writeExclusive(path.join(evidence,'first-use.txt'),output);
      report.geometry={scope:'owned existing F01; generic caller arguments',width,height};
      await invoke('example-missing',{'--root-backend-id':'2147483647'},4);
      await invoke('example-foreign-document',{'--frame-id':other.surface.id,'--loader-id':other.surface.generation},4);
      await page.reload();await page.waitForFunction(()=>!!window.f01); // own-fixture setup only
      await invoke('example-stale-document',{},4);
      report.checks.push({case:'first-use',generic_arguments:true,read_invariance:true,known_rust_dimensions:true,missing_foreign_stale_refused:true});
      report.cli_cleanup_confirmed=true;return;
    }
    if(report.mode==='form_actions'){
      report.phase='public_focus_type';report.binaries.cli=report.binaries.test;delete report.binaries.test;
      const connection=cliConnection(await binding(pages.a),'draft');connection.session.session_id='live-cli-forms';
      const context={schema_version:'0.1.0',session_id:connection.session.session_id,target:connection.target,surfaces:connection.session.surfaces,
        scope_id:'f01-draft',projection:'interaction',fields:['enabled','focused','value','input_kind','readonly'],
        plugin:connection.session.plugin,environment_revision:'f01-form-actions'};
      const pins=[];
      async function save(name,value){const bytes=Buffer.from(JSON.stringify(value));assert(bytes.length<=131072);const file=path.join(evidence,name);await writeExclusive(file,bytes);pins.push({file,bytes});return file;}
      const config=await save('forms-connection.json',connection);
      async function invoke(stage,kind,request,source,expectation,expectedExit,oracle){
        const input=await save(`${stage}-request.json`,request);
        let args;
        if(kind==='observe')args=['observe'];
        else{
          const file=await save(`${stage}-expectation.json`,expectation);
          args=['action',kind,kind==='prepare'?'--snapshot':'--plan',path.join(evidence,`${source.stage}.json`),'--expectation',file,'--json'];
        }
        args.push('--connection',config,'--request',input,'--worker',worker,'--max-input-bytes','131072','--max-output-bytes','65536');
        await command({command:'before',payload:{page:'a',case:stage}});
        const output=await callCli(stage,args,expectedExit);
        await command({command:'form_action_check',payload:{page:'a',case:stage,kind:oracle,document:output.document,canonical:output.bytes.toString('utf8')}});
        return {...output,stage};
      }
      async function observe(stage){return invoke(stage,'observe',cliRequest(context,{operation:'observe',channels:['external_semantics']},stage),null,null,4,'observe');}
      async function prepareForm(stage,source,intent,field,expected){
        const snapshot=source.document.artifact.data.result.data,node=snapshot.nodes.find(n=>n.key.namespace==='web.dom');assert(node);
        const evidence=node.properties.find(p=>p.field==='enabled').evidence,unknown={availability:'unknown',reason:'not-prepared'};
        const action={id:stage,context:snapshot.context,backend_ref:{session_id:snapshot.context.session_id,target:snapshot.context.target,surface:node.surface,
          key:node.key,snapshot_id:snapshot.id,observation_id:evidence.observation_id},intent,modality:intent.intent==='focus'?'semantic':'keyboard',input_space:null,
          required_enabled:true,authorized_scope:snapshot.context.scope_id,unique_match:false,resolution:{evidence,writable:unknown,value_allowed:unknown,available_intents:[]}};
        const expectation={schema_version:'0.1.0',artifact:{kind:'expectation',data:{id:`${stage}-expected`,scope_id:snapshot.context.scope_id,targets:[node.key],
          rule:{relation:'property_equals',field,expected},applies_when:{platform:null,input_mode:null,text_scale:null},expected_from:'explicit-f01-form-scenario'}}};
        const result=await invoke(stage,'prepare',cliRequest(snapshot.context,{operation:'prepare',action},stage),source,expectation,0,'prepare');
        return {...result,expectation};
      }
      async function executeForm(stage,prepared,exit,oracle){const c=prepared.document.artifact.data;
        return invoke(stage,'execute',cliRequest(c.snapshot.context,{operation:'act',action:c.action},stage),prepared,prepared.expectation,exit,oracle);}
      await command({command:'stimulus',payload:{page:'a',action:'form-start'}});
      const initial=await observe('form-initial');
      const focus=await prepareForm('form-focus-prepare',initial,{intent:'focus'},'focused',{type:'flag',value:true});
      await executeForm('form-focus',focus,0,'focus');
      const focused=await observe('form-focused');
      const type=await prepareForm('form-type-prepare',focused,{intent:'type',text:'on'},'value',{type:'text',value:'Lon'});
      await executeForm('form-type',type,0,'type');
      await command({command:'stimulus',payload:{page:'a',action:'form-settle'}});
      const typed=await observe('form-typed');
      const unfocused=await prepareForm('form-unfocused-prepare',typed,{intent:'type',text:'!'},'value',{type:'text',value:'Lon!'});
      await command({command:'stimulus',payload:{page:'a',action:'form-lose-focus'}});
      await executeForm('form-unfocused',unfocused,4,'refused');
      for(const pin of pins)assert((await fs.readFile(pin.file)).equals(pin.bytes));
      for(const frame of report.frames)assert.equal(crypto.createHash('sha256').update(await fs.readFile(path.join(evidence,frame.file))).digest('hex'),frame.sha256);
      assert.deepEqual(report.outcomes.map(o=>o.stage),FORM_ACTION_CASES);assert.deepEqual(ownWorkerPids(worker),[]);
      assert.deepEqual(report.form_action_setup,['form-start','form-settle','form-lose-focus']);
      assert.equal(before.size,0);report.cli_cleanup_confirmed=true;
      checks.push({case:'forms-cli-complete',explicit_expectation_differs_from_delivered_text:true,applied_not_inferred:true,no_retry:true,browser_alive_after_workers:true});return;
    }
    if(directorCase){
      report.phase='director_geometry';report.fixture=null;
      report.binaries.cli=report.binaries.test;delete report.binaries.test;
      const page=pages.a,prefix='clip-search-filter-director';
      const selectors={wrapper:`#${prefix}-wrap`,trigger:`#${prefix}-trigger`,popup:`[data-testid="${prefix}-popover"]`,input:`#${prefix}-input`,list:`#${prefix}-options`};
      // Explicit ordinary setup only: no query input, selection or submission.
      await page.locator(selectors.trigger).click();await page.locator(selectors.input).waitFor();
      const actual=await binding(page),connection=cliConnection(actual,'director');
      connection.session.session_id='live-cli-director';connection.session.allowed_scopes=['playphraseme-director'];
      const selectedKeys=new Map();let documentBackend,rootBackend;
      const session=await context.newCDPSession(page);
      try{
        const {root}=await session.send('DOM.getDocument',{depth:0,pierce:false});documentBackend=root.backendNodeId;
        for(const [name,selector] of Object.entries(selectors)){
          const found=await session.send('DOM.querySelectorAll',{nodeId:root.nodeId,selector});assert.equal(found.nodeIds.length,1);
          const {node}=await session.send('DOM.describeNode',{nodeId:found.nodeIds[0],depth:0,pierce:false});
          selectedKeys.set(name,{namespace:'web.dom',key:String(node.backendNodeId)});if(name==='wrapper')rootBackend=node.backendNodeId;
        }
      }finally{await session.detach();}
      connection.provider.selection={selection:'rooted',max_visited_nodes:256,root:{session_id:connection.session.session_id,
        target:actual.target,surface:actual.surface,document_backend_id:documentBackend,backend_node_id:rootBackend,sensitivity:'public'}};
      const sourceContext={schema_version:'0.1.0',session_id:connection.session.session_id,target:actual.target,surfaces:[actual.surface],
        scope_id:'playphraseme-director',projection:'design',fields:report.mode==='director_semantics'
          ?['role','accessibility_name','layout_bounds','hit_region','visible_region']:['layout_bounds','hit_region','visible_region'],
        plugin:connection.session.plugin,environment_revision:'playphraseme-director-1280x900-dpr1'};
      async function save(name,value){const bytes=Buffer.from(JSON.stringify(value));assert(bytes.length<=131072);await writeExclusive(path.join(evidence,name),bytes);return path.join(evidence,name);}
      async function state(){return page.evaluate(selectors=>({url:location.href,active:document.activeElement?.id,scroll:[scrollX,scrollY],
        expanded:document.querySelector(selectors.trigger).getAttribute('aria-expanded'),input:document.querySelector(selectors.input).value,
        option_count:document.querySelector(selectors.list).querySelectorAll('[role="option"]').length,
        nodes:Object.entries(selectors).map(([name,s])=>{const r=document.querySelector(s).getBoundingClientRect();return {name,rect:[r.x,r.y,r.width,r.height]};})}),selectors);}
      const before=await state();
      const config=await save('director-connection.json',connection),request=await save('director-request.json',cliRequest(sourceContext,{operation:'observe',channels:['external_semantics']},'director-observe'));
      const output=await callCli('director-observe',['observe','--connection',config,'--request',request,'--worker',worker,'--max-input-bytes','131072','--max-output-bytes','65536'],4);
      assert.deepEqual(await state(),before);
      assert.equal(output.document.artifact.kind,'channel_response');assert.equal(output.document.artifact.data.result.status,'observed');
      const snapshot=output.document.artifact.data.result.data,file=path.join(evidence,'director-observe.json');
      const nodes=snapshot.nodes.filter(n=>n.key.namespace==='web.dom');
      const node=name=>nodes.find(n=>n.key.key===selectedKeys.get(name).key);
      report.geometry={scope:'real local PlayPhrase.me Director; empty query; opened only',url:before.url,viewport:[1280,900],dpr:1,
        coverage:snapshot.coverage,observations:snapshot.observations,available_option_count:before.option_count,bounds:[],measurements:[],
        viewport_overflow:'unknown: no observed viewport geometry node',clipping:'unknown: source visible_region unavailable'};
      for(const [name,key] of selectedKeys){
        if(node(name)){const rect=rectangle(node(name));compareRect(rect,before.nodes.find(n=>n.name===name).rect);report.geometry.bounds.push({name,key,rect,units:'css_px'});}
        else report.geometry.bounds.push({name,key,availability:'unknown',reason:'outside_returned_scope'});
      }
      report.checks.push({case:'director-observe',read_invariance:true,dom_nodes:nodes.length,ax_nodes:snapshot.nodes.length-nodes.length});
      await callCli('director-design-inspect',['inspect','--snapshot',file,'--ref',JSON.stringify(selectedKeys.get('wrapper')),'--view','design','--max-input-bytes','131072','--max-output-bytes','65536'],0,65536,false);
      const space=property(node('wrapper'),'layout_bounds').value.coordinate_space;
      const anchor=(name,fraction=0,axis='x',frame_kind='layout_bounds')=>({element:selectedKeys.get(name),frame_kind,coordinate_space:space,fraction,axis});
      const queries=[['wrapper-width','width',[anchor('wrapper')]],['wrapper-height','height',[anchor('wrapper')]],
        ['popup-width','width',[anchor('popup')]],['popup-height','height',[anchor('popup')]],
        ['trigger-popup-gap','gap',[anchor('trigger',1,'y'),anchor('popup',0,'y')]],
        ['wrapper-popup-left-offset','gap',[anchor('wrapper'),anchor('popup')]],
        ['trigger-popup-left-offset','gap',[anchor('trigger'),anchor('popup')]],
        ['input-insets','inside',[anchor('input'),anchor('popup')]],
        ['input-list-gap','gap',[anchor('input',1,'y'),anchor('list',0,'y')]],
        ['input-list-left-offset','gap',[anchor('input'),anchor('list')]],
        ['visible-popup-width','width',[anchor('popup',0,'x','visible_region')]]];
      for(const [id,operation,anchors] of queries){
        const query={schema_version:'0.2.0',artifact:{kind:'geometry_query',data:{id,scope_id:snapshot.context.scope_id,targets:anchors.map(a=>a.element),
          operation,anchors,quantity_kind:'length',units:'css_px',applies_when:{platform:null,input_mode:null,text_scale:null}}}};
        const q=await save(`director-${id}-query.json`,query);
        const unknown=id==='visible-popup-width'||anchors.some(a=>!nodes.some(n=>n.key.key===a.element.key));
        const measured=await callCli(`director-${id}`,['measure','--snapshot',file,'--query',q,'--space',space.id,'--max-input-bytes','131072','--max-output-bytes','65536','--json'],unknown?4:0);
        assert.equal(measured.document.artifact.kind,'measurement');const result=measured.document.artifact.data;
        assert.deepEqual(result.snapshot,snapshot);report.geometry.measurements.push({id,result:result.result});
      }
      assert((await fs.readFile(file)).equals(output.bytes));assert.deepEqual(await state(),before);
      assert.deepEqual(ownWorkerPids(worker),[]);report.cli_cleanup_confirmed=true;
      report.checks.push({case:'director-pipeline',engine_only_measurements:true,original_channel_bytes_preserved:true,browser_alive_after_workers:true});return;
    }
    if(report.mode==='geometry'){
      report.phase='component_geometry';report.binaries.cli=report.binaries.test;delete report.binaries.test;
      const page=pages.a;
      // Only fixture setup: expose existing content, without the product executor.
      await page.locator('#draft').fill('Lon');await page.locator('#option-london').waitFor();
      await page.locator('#open-popup').click();await page.locator('#portal').waitFor();
      const actual=await binding(page), connection=cliConnection(actual,'component');
      connection.session.session_id='live-cli-geometry';
      const ids=['f01','draft','suggestions','option-london','open-popup','portal','close-popup','responsive'];
      const selectedKeys=new Map();let rootBackend,documentBackend;
      const session=await context.newCDPSession(page);
      try{
        const {root}=await session.send('DOM.getDocument',{depth:0,pierce:false});documentBackend=root.backendNodeId;
        for(const id of ids){
          const found=await session.send('DOM.querySelectorAll',{nodeId:root.nodeId,selector:`#${id}`});assert.equal(found.nodeIds.length,1);
          const {node}=await session.send('DOM.describeNode',{nodeId:found.nodeIds[0],depth:0,pierce:false});
          selectedKeys.set(id,{namespace:'web.dom',key:String(node.backendNodeId)});if(id==='f01')rootBackend=node.backendNodeId;
        }
      }finally{await session.detach();}
      const fields=['role','accessibility_name','layout_bounds','hit_region','visible_region'];
      const sourceContext={schema_version:'0.1.0',session_id:connection.session.session_id,target:actual.target,surfaces:[actual.surface],
        scope_id:'f01-component',projection:'design',fields,plugin:connection.session.plugin,environment_revision:'f01-800x600-geometry'};
      report.geometry={scope:'owned F01 only; not real Director',bounds:[],measurements:[],viewport_overflow:'unknown: no observed viewport geometry node',clipping:'unknown unless supplied by source'};
      async function save(name,value){const bytes=Buffer.from(JSON.stringify(value));assert(bytes.length<=131072);await writeExclusive(path.join(evidence,name),bytes);return path.join(evidence,name);}
      async function state(){return {ui:await uiState(page),geometry:await page.evaluate(ids=>({viewport:[innerWidth,innerHeight],nodes:ids.map(id=>{
        const n=document.getElementById(id),r=n.getBoundingClientRect();return {id,rect:[r.x,r.y,r.width,r.height]};
      })}),ids)};}
      async function observeGeometry(stage,selection,environment){
        const c={...connection,provider:{...connection.provider,selection}};
        const request=cliRequest({...sourceContext,environment_revision:environment},{operation:'observe',channels:['external_semantics']},stage);
        const config=await save(`${stage}-connection.json`,c), input=await save(`${stage}-request.json`,request);
        const before=await state();
        const output=await callCli(stage,['observe','--connection',config,'--request',input,'--worker',worker,'--max-input-bytes','131072','--max-output-bytes','65536'],4);
        assert.deepEqual(await state(),before);
        assert.equal(output.document.artifact.kind,'channel_response');assert.equal(output.document.artifact.data.result.status,'observed');
        const snapshot=output.document.artifact.data.result.data;assert.equal(snapshot.coverage.status,'partial');
        const dom=snapshot.nodes.filter(n=>n.key.namespace==='web.dom');assert(dom.length<=16);
        for(const item of before.geometry.nodes){
          const node=dom.find(n=>n.key.key===selectedKeys.get(item.id).key);
          if(node)compareRect(rectangle(node),item.rect);
        }
        report.checks.push({case:stage,read_invariance:true,dom_nodes:dom.length,ax_nodes:snapshot.nodes.length-dom.length});
        return {snapshot,file:path.join(evidence,`${stage}.json`),bytes:output.bytes};
      }
      const rooted=await observeGeometry('geometry-rooted',{selection:'rooted',max_visited_nodes:256,root:{
        session_id:connection.session.session_id,target:actual.target,surface:actual.surface,
        document_backend_id:documentBackend,backend_node_id:rootBackend,sensitivity:'public'}},sourceContext.environment_revision);
      assert.equal(rooted.snapshot.nodes.filter(n=>n.key.namespace==='web.dom').length,10);
      assert(!rooted.snapshot.nodes.some(n=>n.key.namespace==='web.dom'&&n.key.key===selectedKeys.get('portal').key),'BODY portal is not silently in rooted subtree');
      const inspectArgs=['inspect','--snapshot',rooted.file,'--ref',JSON.stringify(selectedKeys.get('f01')),'--view','design','--max-input-bytes','131072','--max-output-bytes','65536'];
      await callCli('geometry-design-inspect',inspectArgs,0,65536,false);
      const selection={selection:'initial',ids:ids.map(id=>({id,sensitivity:'public'})),max_visited_nodes:256};
      const component=await observeGeometry('geometry-context',selection,sourceContext.environment_revision);
      const nodes=component.snapshot.nodes.filter(n=>n.key.namespace==='web.dom');assert.equal(nodes.length,ids.length);
      const node=id=>nodes.find(n=>n.key.key===selectedKeys.get(id).key);
      for(const id of ids){
        const relation=component.snapshot.relations.find(r=>r.kind==='corresponds_to'&&r.from.namespace==='web.dom'&&r.from.key===node(id).key.key);
        const ax=relation&&component.snapshot.nodes.find(n=>n.key.namespace===relation.to.namespace&&n.key.key===relation.to.key);
        const name=ax?.properties.find(p=>p.field==='accessibility_name');
        report.geometry.bounds.push({id,accessible_name:name?.state.availability==='known'?name.state.value.value:null,rect:rectangle(node(id)),units:'css_px'});
      }
      assert(component.snapshot.relations.some(r=>r.kind==='anchored_to'&&r.from.key===selectedKeys.get('portal').key&&r.to.key===selectedKeys.get('open-popup').key));
      assert(component.snapshot.relations.some(r=>r.kind==='controls'&&r.from.key===selectedKeys.get('draft').key&&r.to.key===selectedKeys.get('suggestions').key));
      const space=property(node('f01'),'layout_bounds').value.coordinate_space;
      const anchor=(id,fraction=0,axis='x',frame_kind='layout_bounds')=>({element:selectedKeys.get(id),frame_kind,coordinate_space:space,fraction,axis});
      const queries=[
        ['wrapper-width','width',[anchor('f01')]],['wrapper-height','height',[anchor('f01')]],
        ['popup-width','width',[anchor('portal')]],['popup-height','height',[anchor('portal')]],
        ['trigger-popup-gap','gap',[anchor('open-popup',1,'y'),anchor('portal',0,'y')]],
        ['popup-left-offset','gap',[anchor('open-popup'),anchor('portal')]],
        ['input-insets','inside',[anchor('draft'),anchor('f01')]],
        ['input-list-gap','gap',[anchor('draft',1,'y'),anchor('suggestions',0,'y')]],
        ['input-list-left-offset','gap',[anchor('draft'),anchor('suggestions')]],
        ['list-option-left-offset','gap',[anchor('suggestions'),anchor('option-london')]],
        ['left-edge-spread','aligned',[anchor('draft'),anchor('suggestions'),anchor('option-london')]],
        ['visible-popup-width','width',[anchor('portal',0,'x','visible_region')]]
      ];
      for(const [id,operation,anchors] of queries){
        const query={schema_version:'0.2.0',artifact:{kind:'geometry_query',data:{id,scope_id:component.snapshot.context.scope_id,
          targets:anchors.map(a=>a.element),operation,anchors,quantity_kind:'length',units:'css_px',applies_when:{platform:null,input_mode:null,text_scale:null}}}};
        const q=await save(`geometry-${id}-query.json`,query);
        const unknown=id==='visible-popup-width'||id==='left-edge-spread';
        const measured=await callCli(`geometry-${id}`,['measure','--snapshot',component.file,'--query',q,'--space',space.id,'--max-input-bytes','131072','--max-output-bytes','65536','--json'],unknown?4:0);
        assert.equal(measured.document.schema_version,'0.2.0');assert.equal(measured.document.artifact.kind,'measurement');
        const result=measured.document.artifact.data;assert.deepEqual(result.snapshot,component.snapshot);
        assert.equal(result.result.status,unknown?'unknown':'known');
        if(id==='left-edge-spread')assert.equal(result.result.reason,'incomplete_scope');
        if(!unknown){assert.deepEqual(result.result.measurement.space,space);assert(result.result.measurement.evidence.length>0);}
        if(id==='popup-width')assert.equal(result.result.measurement.value.value.amount,200);
        if(id==='popup-height')assert.equal(result.result.measurement.value.value.amount,60);
        report.geometry.measurements.push({id,result:result.result});
      }
      // Existing responsive F01 context marker shifts on real viewport resize.
      await page.setViewportSize({width:640,height:600});
      const after=await observeGeometry('geometry-resized',selection,'f01-640x600-geometry');
      const diff=await callCli('geometry-diff',['diff','--before',component.file,'--after',after.file,'--max-input-bytes','131072','--max-output-bytes','200000','--max-entries','128','--json'],0,200000);
      assert.equal(diff.document.kind,'recorded_difference');assert.equal(diff.document.omitted_entries,0);
      assert.deepEqual(diff.document.before,component.snapshot);assert.deepEqual(diff.document.after,after.snapshot);
      const changed=diff.document.entries.filter(e=>e.kind==='property'&&e.field==='layout_bounds'&&e.content_changed&&e.key.namespace==='web.dom');
      assert(changed.some(e=>e.key.key===selectedKeys.get('responsive').key));
      report.geometry.resize={from:[800,600],to:[640,600],changed_dom_bounds:changed.map(e=>e.key),responsive_after:rectangle(after.snapshot.nodes.find(n=>n.key.namespace==='web.dom'&&n.key.key===selectedKeys.get('responsive').key))};
      assert((await fs.readFile(component.file)).equals(component.bytes));assert((await fs.readFile(rooted.file)).equals(rooted.bytes));
      assert.deepEqual(ownWorkerPids(worker),[]);await uiState(page);
      report.cli_cleanup_confirmed=true;report.checks.push({case:'geometry-pipeline',engine_only_measurements:true,original_channel_bytes_preserved:true,browser_alive_after_workers:true});
      return;
    }
    if(report.mode==='cli_actions'){
      report.phase='public_cli_consumer';
      report.binaries.cli=report.binaries.test;delete report.binaries.test;
      // In this mode UIB_WEB_LIVE_TEST is the pinned PUBLIC CLI executable.
      // Each call owns its production worker; no test-host API substitutes for it.
      const page=pages.a, actual=await binding(page);
      assert.deepEqual(ownWorkerPids(worker),[]);
      const inputPins=[];report.cli_inputs=inputPins;
      async function invoke(stage,command,connection,request,source,expectedExit){
        report.last_stage=stage;
        const files=[];
        for(const [kind,bytes] of [['connection',Buffer.from(JSON.stringify(connection))],['request',Buffer.from(JSON.stringify(request))],...(source?[['source',source]]:[])]){
          const name=`${stage}-${kind}.json`;await writeExclusive(path.join(evidence,name),bytes);files.push([kind,name,bytes.length]);
          inputPins.push({file:name,bytes:bytes.length,sha256:crypto.createHash('sha256').update(bytes).digest('hex')});
        }
        assert(files.reduce((sum,f)=>sum+f[2],0)<=131072);
        const file=kind=>path.join(evidence,files.find(f=>f[0]===kind)[1]);
        const args=command==='observe'?['observe']:['action',command,command==='prepare'?'--snapshot':'--plan',file('source'),'--json'];
        args.push('--connection',file('connection'),'--request',file('request'),'--worker',worker,'--max-input-bytes','131072','--max-output-bytes','65536');
        return callCli(stage,args,expectedExit);
      }
      function observation(connection,selected,stage){
        return cliRequest({schema_version:'0.1.0',session_id:connection.session.session_id,target:actual.target,surfaces:[actual.surface],
          scope_id:`f01-${selected}`,projection:'interaction',fields:['enabled','checked','input_kind'],plugin:connection.session.plugin,
          environment_revision:'actions-800x600-dpr1-cli'}, {operation:'observe',channels:['external_semantics']},stage);
      }
      const connection=cliConnection(actual,'action-target');
      let prior=await uiState(page);assert.equal(prior.state.target,false);assert.equal(prior.state.duplicate,true);
      const observed=await invoke('cli-observe','observe',connection,observation(connection,'action-target','cli-observe'),null,4);
      assert.deepEqual(await uiState(page),prior);
      const seed=cliPrepareRequest(observed.document,'cli-prepare');
      const observedSnapshot=observed.document.artifact.data.result.data;
      assert.equal(observedSnapshot.coverage.status,'partial');
      const observedNode=observedSnapshot.nodes.find(n=>n.key.namespace==='web.dom');
      assert.equal(property(observedNode,'enabled').value,true);assert.equal(property(observedNode,'checked').value,false);
      assert.equal(property(observedNode,'input_kind').value,'checkbox');
      const prepared=await invoke('cli-prepare','prepare',connection,seed,observed.bytes,0);
      assert.deepEqual(await uiState(page),prior);assert.equal(prepared.document.artifact.kind,'action');
      const plan=prepared.document.artifact.data;
      assert.equal(plan.action.unique_match,true);assert.equal(plan.action.modality,'setter');
      for(const name of ['writable','value_allowed'])assert.deepEqual(plan.action.resolution[name],{availability:'known',value:{type:'flag',value:true}});
      assert(plan.action.resolution.available_intents.includes('set_checked'));
      assert.deepEqual(plan.action.backend_ref.key,seed.artifact.data.operation.action.backend_ref.key);
      assert.equal(plan.action.backend_ref.snapshot_id,plan.snapshot.id);
      assert.deepEqual(plan.snapshot.context,observedSnapshot.context);
      assert.equal(property(plan.snapshot.nodes.find(n=>n.key.namespace==='web.dom'),'checked').value,false);
      const act=stage=>cliRequest(plan.snapshot.context,{operation:'act',action:plan.action},stage);
      const executed=await invoke('cli-execute','execute',connection,act('cli-execute'),prepared.bytes,0);
      assert.equal(executed.document.artifact.kind,'transition_context');
      const transition=executed.document.artifact.data;assert.equal(transition.transition.steps.length,1);
      const step=transition.transition.steps[0];assert.equal(step.delivery,'confirmed');assert.equal(step.outcome,'succeeded');
      assert(transition.after&&step.verification_observation);assert.notEqual(transition.before.id,transition.after.id);
      assert.equal(property(transition.after.nodes.find(n=>n.key.namespace==='web.dom'),'checked').value,true);
      const after=await uiState(page);assert.equal(after.state.target,true);
      assert.deepEqual({...after,state:{...after.state,target:false}},prior);
      report.checks.push({case:'cli-positive',observe_exit:4,prepare_exit:0,execute_exit:0,source_state_verified:true,non_target_state_unchanged:true});
      prior=after;
      const disabledConnection=cliConnection(actual,'action-disabled');
      const disabled=await invoke('cli-disabled-observe','observe',disabledConnection,observation(disabledConnection,'action-disabled','cli-disabled-observe'),null,4);
      const disabledSeed=cliPrepareRequest(disabled.document,'cli-disabled-prepare');
      assert.equal(property(disabled.document.artifact.data.result.data.nodes.find(n=>n.key.namespace==='web.dom'),'enabled').value,false);
      const refusal=await invoke('cli-disabled-prepare','prepare',disabledConnection,disabledSeed,disabled.bytes,4);
      assert.equal(refusal.document.artifact.kind,'error');assert.equal(refusal.document.artifact.data.code,'unsupported');
      assert.deepEqual(await uiState(page),prior);
      report.checks.push({case:'cli-disabled-prepare',fresh_capability_refused:true,state_unchanged:true,not_readonly_authority_test:true});
      await page.locator('#remount-target').click(); // explicit fixture setup only
      prior=await uiState(page);
      const stale=await invoke('cli-remount-refused','execute',connection,act('cli-remount-refused'),prepared.bytes,4);
      assert.equal(stale.document.artifact.kind,'transition_context');
      const refusalStep=stale.document.artifact.data.transition.steps[0];
      assert.equal(refusalStep.delivery,'not_dispatched');assert.equal(refusalStep.outcome,'failed');
      assert.equal(stale.document.artifact.data.after,null);assert.deepEqual(await uiState(page),prior);
      report.checks.push({case:'cli-remount-refused',old_ref_refused:true,no_retry:true,state_unchanged:true});
      // All source files remain byte-identical, including original newline/evidence.
      for(const pin of inputPins){const data=await fs.readFile(path.join(evidence,pin.file));assert.equal(crypto.createHash('sha256').update(data).digest('hex'),pin.sha256);}
      assert.deepEqual(ownWorkerPids(worker),[]);await uiState(page);
      report.checks.push({case:'cli-cleanup',browser_alive_after_cli_workers:true,no_owned_worker_processes:true,private_session_counters:'not_exposed'});
      report.cli_cleanup_confirmed=true;assert.equal(report.outcomes.length,6);return;
    }
    let rootMembers, b05Previous;
    async function command(message) {
      const payload = message.payload; const page = pages[payload.page]; assert(page, 'owned page only');
      switch (message.command) {
        case 'binding': return binding(page);
        case 'root': {
          const session=await context.newCDPSession(page);
          try {
            const {root}=await session.send('DOM.getDocument',{depth:0,pierce:false});
            const {nodeIds}=await session.send('DOM.querySelectorAll',{nodeId:root.nodeId,selector:'#f01'});assert.equal(nodeIds.length,1);
            const described=await session.send('DOM.describeNode',{nodeId:nodeIds[0],depth:0,pierce:false});
            const children=await session.send('DOM.querySelectorAll',{nodeId:nodeIds[0],selector:'*'});assert.equal(children.nodeIds.length,8);
            rootMembers=new Map();
            for(const nodeId of [nodeIds[0],...children.nodeIds]){
              const {node}=await session.send('DOM.describeNode',{nodeId,depth:0,pierce:false});
              const attrs=node.attributes||[];const at=attrs.indexOf('id');rootMembers.set(String(node.backendNodeId),at<0?'label':attrs[at+1]);
            }
            assert.deepEqual([...rootMembers.values()],['f01','label','draft','suggestions','validation','commit','applied','open-popup','surprise']);
            return {binding:await binding(page),document_backend_id:root.backendNodeId,backend_node_id:described.node.backendNodeId};
          } finally {await session.detach();}
        }
        case 'before': {
          assert(CASES.has(payload.case)); report.last_stage=payload.case;
          const state=await uiState(page, ['form_reads','form_actions'].includes(report.mode));
          if(report.mode==='form_reads'){
            assert(Object.hasOwn(FORM_CASES,payload.case));
            assert.equal(report.form_setup?.at(-1),payload.case);
            assertFormState(state,FORM_CASES[payload.case]);
          }
          before.set(payload.case,state);return {};
        }
        case 'outcome': {
          assert(CASES.has(payload.stage)&&TERMINALS.has(payload.terminal));
          assert(Number.isSafeInteger(payload.operation)&&payload.operation>0);
          assert([payload.committed,payload.missing].every(n=>Number.isInteger(n)&&n>=0&&n<=7));
          const d=payload.diagnostic;
          if(d!==null){assert(d&&Object.keys(d).sort().join(',')==='cause,code,count,remote_cleanup,send_progress,stage');assert(Number.isInteger(d.stage)&&d.stage>=1&&d.stage<=9);assert(Number.isInteger(d.cause)&&d.cause>=1&&d.cause<=44);assert(Number.isInteger(d.remote_cleanup)&&d.remote_cleanup>=0&&d.remote_cleanup<=2);assert(Number.isInteger(d.send_progress)&&d.send_progress>=0&&d.send_progress<=3);assert(Number.isInteger(d.code)&&d.code>=-2147483648&&d.code<=2147483647);assert(Number.isInteger(d.count)&&d.count>=0&&d.count<=4294967295);}
          assert(['not_dispatched','possible','confirmed'].includes(payload.effect));
          assert(report.outcomes.length<16);report.outcomes.push({stage:payload.stage,terminal:payload.terminal,committed:payload.committed,missing:payload.missing,operation:payload.operation,diagnostic:d,effect:payload.effect});return {};
        }
        case 'worker_cleanup': {
          assert(typeof payload.confirmed==='boolean'&&typeof payload.abandoned==='boolean');
          assert(Number.isInteger(payload.reserved_sessions)&&payload.reserved_sessions>=0&&payload.reserved_sessions<=4);
          assert(Number.isInteger(payload.completion_groups)&&payload.completion_groups>=0&&payload.completion_groups<=8);
          report.worker_cleanup={confirmed:payload.confirmed,reserved_sessions:payload.reserved_sessions,completion_groups:payload.completion_groups,abandoned:payload.abandoned};return {};
        }
        case 'action_check': {
          assert.equal(report.mode,'actions');assert(CASES.has(payload.case));assert(before.has(payload.case));
          const prior=before.get(payload.case),after=await uiState(page);before.delete(payload.case);
          if(payload.kind==='success'){
            assert.equal(prior.state.target,false);assert.equal(after.state.target,true);
            assert.deepEqual({...after,state:{...after.state,target:prior.state.target}},prior);
          }else assert.deepEqual(after,prior);
          const doc=payload.document;
          if(doc){
            assert.equal(doc.schema_version,'0.1.0');
            if(payload.kind==='observe'){
              assert.equal(doc.artifact.kind,'channel_response');assert.equal(doc.artifact.data.result.status,'observed');
              const snapshot=doc.artifact.data.result.data;const dom=snapshot.nodes.find(n=>n.key.namespace==='web.dom');
              assert.equal(property(dom,'checked').value,prior.state.target);assert.equal(property(dom,'enabled').value,true);assert.equal(property(dom,'input_kind').value,'checkbox');
            }else if(payload.kind==='prepare'){
              assert.equal(doc.artifact.kind,'action');const c=doc.artifact.data;
              assert.equal(c.action.resolution.writable.availability,'known');assert.equal(c.action.resolution.writable.value.value,true);
              assert.equal(c.action.resolution.value_allowed.value.value,true);assert(c.action.resolution.available_intents.includes('set_checked'));
              assert.equal(c.action.backend_ref.snapshot_id,c.snapshot.id);assert.equal(c.action.modality,'setter');
              assert.equal(property(c.snapshot.nodes.find(n=>n.key.namespace==='web.dom'),'checked').value,prior.state.target);
            }else if(payload.kind==='success'||payload.kind==='refused'){
              assert.equal(doc.artifact.kind,'transition_context');const c=doc.artifact.data;assert.equal(c.transition.steps.length,1);const step=c.transition.steps[0];
              if(payload.kind==='success'){
                assert.equal(step.delivery,'confirmed');assert.equal(step.outcome,'succeeded');assert(c.after);assert.notEqual(c.before.id,c.after.id);assert(step.verification_observation);
                assert.equal(property(c.after.nodes.find(n=>n.key.namespace==='web.dom'),'checked').value,true);
              }else{assert.equal(step.delivery,'not_dispatched');assert.equal(step.outcome,'failed');assert.equal(c.after,null);}
            }else throw Error('unexpected_action_document');
          }else assert(['readonly_refused','possible_before_delivery'].includes(payload.kind));
          if(Object.hasOwn(FRAME_FILES,payload.case)){
            assert(doc&&typeof payload.canonical==='string');const bytes=Buffer.from(payload.canonical);assert(bytes.length<=65536);assert.deepEqual(JSON.parse(payload.canonical),doc);
            const file=FRAME_FILES[payload.case];await writeExclusive(path.join(evidence,file),bytes);report.frames.push({case:payload.case,file,bytes:bytes.length,sha256:crypto.createHash('sha256').update(bytes).digest('hex'),usage:'historical_analysis_only',live_ref_reuse:false});
          }
          checks.push({case:payload.case,readonly:payload.kind!=='success',oracle:payload.kind,non_target_state_unchanged:true});return {};
        }
        case 'form_action_check': {
          assert.equal(report.mode,'form_actions');assert(FORM_ACTION_CASES.includes(payload.case));assert(before.has(payload.case));
          const prior=before.get(payload.case),after=await uiState(page,true);before.delete(payload.case);
          assert.equal(after.form.documentFocused,true);assert.equal(after.form.output,'');
          assert.deepEqual([after.state.selected,after.state.applied,after.state.valid,after.state.delivered],['','',false,0]);
          assert.deepEqual(after.state.rows,prior.state.rows);assert.deepEqual(after.scroll,prior.scroll);
          if(payload.kind==='focus'){
            assert.equal(prior.active,'left');assert.equal(after.active,'draft');assert.equal(after.form.value,'L');assert.deepEqual(after.state,prior.state);
          }else if(payload.kind==='type'){
            assert.equal(prior.active,'draft');assert.equal(prior.form.value,'L');assert.deepEqual([prior.form.start,prior.form.end],[1,1]);
            assert.equal(after.active,'draft');assert.equal(after.form.value,'Lon');assert.deepEqual([after.form.start,after.form.end],[3,3]);
            assert(['draft-pending','suggestion-ready'].includes(after.state.source_state));
          }else assert.deepEqual(after,prior);
          const doc=payload.document;assert.equal(doc.schema_version,'0.1.0');let snapshot;
          if(payload.kind==='observe'){
            assert.equal(doc.artifact.kind,'channel_response');assert.equal(doc.artifact.data.result.status,'observed');snapshot=doc.artifact.data.result.data;
          }else if(payload.kind==='prepare'){
            assert.equal(doc.artifact.kind,'action');const c=doc.artifact.data;snapshot=c.snapshot;
            const intent=payload.case==='form-focus-prepare'?'focus':'type';
            assert.equal(c.action.intent.intent,intent);assert.equal(c.action.modality,intent==='focus'?'semantic':'keyboard');
            if(intent==='type')assert.equal(c.action.intent.text,payload.case==='form-type-prepare'?'on':'!');
            assert.equal(c.action.resolution.evidence.method,intent==='focus'?'cdp.DOM.focus-native-text-control':'cdp.Input.insertText-native-ImeCommitText');
            assert.equal(c.action.resolution.writable.value.value,true);assert.equal(c.action.resolution.value_allowed.value.value,true);
            assert(c.action.resolution.available_intents.includes(intent));assert.equal(c.action.backend_ref.snapshot_id,snapshot.id);
          }else{
            assert.equal(doc.artifact.kind,'transition_context');const c=doc.artifact.data;assert.equal(c.transition.steps.length,1);const step=c.transition.steps[0];
            if(payload.kind==='refused'){
              assert.equal(step.delivery,'not_dispatched');assert.equal(step.outcome,'failed');assert.equal(c.after,null);
              assert.equal(after.active,'left');assert.equal(after.form.value,'Lon');
            }else{
              assert(['focus','type'].includes(payload.kind));assert.equal(step.delivery,'confirmed');assert.equal(step.outcome,'succeeded');
              assert(c.after);assert.notEqual(c.before.id,c.after.id);assert(step.verification_observation);snapshot=c.after;
            }
          }
          if(snapshot){
            const input=snapshot.nodes.find(n=>n.key.namespace==='web.dom');assert(input);
            assert.equal(property(input,'value').value,after.form.value);assert.equal(property(input,'focused').value,after.active==='draft');
            if(after.active==='draft'){assert.equal(snapshot.focus.keyboard.status,'known');assert.deepEqual(snapshot.focus.keyboard.target,input.key);}
            assert.equal(snapshot.focus.composition_state.selection,'not_requested');
          }
          if(['form-focus','form-type','form-typed'].includes(payload.case)){
            assert.equal(typeof payload.canonical,'string');const bytes=Buffer.from(payload.canonical);assert(bytes.length<=65536);
            assert.deepEqual(JSON.parse(payload.canonical),doc);assert(!payload.canonical.includes(CANARY));
            const file=`${payload.case}.json`;assert((await fs.readFile(path.join(evidence,file))).equals(bytes));
          }
          checks.push({case:payload.case,oracle:payload.kind,unrelated_state_unchanged:true,draft:after.form.value,applied:after.state.applied});return {};
        }
        case 'check': {
          assert(CASES.has(payload.case));
          assert(before.has(payload.case)); assert.deepEqual(await uiState(page, report.mode==='form_reads'),before.get(payload.case)); before.delete(payload.case);
          if (payload.document !== null) {
            const doc = payload.document; assert.equal(doc.schema_version,'0.1.0');
            assert.equal(doc.artifact.kind,'channel_response'); const response = doc.artifact.data;
            assert.equal(response.result.status,'observed'); const s = response.result.data;
            assert.equal(s.coverage.status,'partial'); assert.equal(s.source_state,null);
            assert(s.nodes.length <= 32); const dom = s.nodes.filter(n=>n.key.namespace==='web.dom');
            assert.equal(dom.length,payload.kind==='form_reads'?2:payload.kind==='rooted'?9:payload.kind==='popup'?5:1); assert(s.observations.every(o=>o.freshness==='current'&&o.consistency==='unknown'));
            if (payload.kind === 'form_reads') {
              assert.equal(report.mode,'form_reads');assert(Object.hasOwn(FORM_CASES,payload.case));
              const expected=FORM_CASES[payload.case], [input,output]=dom;
              assert.deepEqual([...s.context.fields].sort(),['focused','value']);
              assert.equal(property(input,'focused').value,true);assert.equal(property(output,'focused').value,false);
              assert.deepEqual(property(output,'value'),{type:'text',value:expected.output});
              assert.equal(s.focus.active_descendant.status,'unknown');
              assert.equal(s.focus.composition_state.selection,'not_requested');
              if(expected.private){
                const value=input.properties.find(p=>p.field==='value');
                assert.equal(value.state.availability,'redacted');assert.equal(value.sensitivity,'sensitive');
                assert.equal(s.focus.text_selection,null);assert.equal(s.focus.keyboard.status,'unknown');
                assert(!JSON.stringify(doc).includes(CANARY));
              }else{
                assert.deepEqual(property(input,'value'),{type:'text',value:'A💡B'});
                assert.equal(s.focus.keyboard.status,'known');assert.deepEqual(s.focus.keyboard.target,input.key);
                const selection=s.focus.text_selection;assert(selection);
                assert.deepEqual([selection.anchor,selection.focus,selection.units],[expected.anchor,expected.focus,'utf16_code_units']);
                for(const e of [selection.evidence,s.focus.keyboard.evidence]){
                  assert.equal(e.source_namespace,'web.dom');assert.equal(e.provenance,'reported');
                  assert(s.observations.some(o=>o.id===e.observation_id&&o.source_namespace==='web.dom'));
                }
                assert.equal(selection.evidence.observation_id,s.focus.keyboard.evidence.observation_id);
              }
            } else if (payload.kind.startsWith('b05_')) {
              const dimensions=payload.kind==='b05_initial'?f01.B05.child_before:payload.kind==='b05_parent'?f01.B05.child_after_parent:f01.B05.child_after_font;
              compareRect(rectangle(dom[0]),[40,550,...dimensions]);
              const current={key:dom[0].key,space:property(dom[0],'layout_bounds').value.coordinate_space,revision:s.revision,observation:s.observations[0].id,environment:s.context.environment_revision};
              assert.equal(current.environment,`f01-800x600-dpr1-${payload.case}`);
              if(b05Previous){assert.deepEqual(current.key,b05Previous.key);assert.deepEqual(current.space,b05Previous.space);assert(current.revision>b05Previous.revision);assert.notEqual(current.observation,b05Previous.observation);assert.notEqual(current.environment,b05Previous.environment);}
              b05Previous=current;
            } else if (payload.kind === 'rooted') {
              assert(rootMembers);assert.deepEqual(new Set(dom.map(n=>n.key.key)),new Set(rootMembers.keys()));
              const named=id=>dom.find(n=>rootMembers.get(n.key.key)===id);
              const root=rectangle(named('f01'));[380,20,360].forEach((expected,i)=>assert(Math.abs(root[i]-expected)<=f01.geometry_tolerance_css_px));assert(root[3]>0);
              const input=named('draft'), trigger=named('open-popup'), suggestions=named('suggestions');
              assert.equal(property(trigger,'expanded').value,true);assert.equal(property(input,'focused').value,false);
              const same=(a,b)=>a.namespace===b.namespace&&a.key===b.key;
              assert(s.relations.some(r=>r.kind==='controls'&&same(r.from,input.key)&&same(r.to,suggestions.key)));
              assert(!s.relations.some(r=>r.kind==='anchored_to'||(r.kind==='controls'&&same(r.from,trigger.key))));
              const relation=s.relations.find(r=>r.kind==='corresponds_to'&&same(r.from,input.key));assert(relation);
              const ax=s.nodes.find(n=>same(n.key,relation.to));assert.equal(property(ax,'role').value,'combobox');assert.equal(property(ax,'accessibility_name').value,'City');
              assert.equal(property(input,'layout_bounds').value.coordinate_space.id,s.context.surfaces[0].id);
              assert.equal(s.focus.keyboard.status,'unknown');assert.equal(s.focus.active_descendant.status,'unknown');
            } else if (payload.kind === 'popup') {
              const [trigger,popup,close,input,suggestions]=dom;
              const same=(a,b)=>a.namespace===b.namespace&&a.key===b.key;
              const relation=(kind,from,to,method)=>assert(s.relations.some(r=>r.kind===kind&&same(r.from,from.key)&&same(r.to,to.key)&&r.evidence.method===method&&r.evidence.provenance==='reported'&&r.evidence.source_namespace==='web.dom'));
              relation('controls',trigger,popup,'dom-aria-controls');
              relation('controls',input,suggestions,'dom-aria-controls');
              relation('anchored_to',popup,trigger,'fixture-data-anchor-attribute');
              assert.equal(s.relations.filter(r=>r.kind!=='corresponds_to').length,3);
              // Literal CSS rectangle from frozen fixtures/web/extension.html; no runtime-derived oracle.
              compareRect(rectangle(popup),[400,290,200,60]);
              for(const node of dom){rectangle(node);assert.equal(property(node,'layout_bounds').value.coordinate_space.id,s.context.surfaces[0].id);assert.equal(node.properties.find(p=>p.field==='hit_region').state.availability,'unknown');}
              assert.equal(property(trigger,'expanded').value,true);
              assert.equal(property(close,'focused').value,true);
              assert.equal(property(input,'focused').value,false);
              assert.equal(property(input,'value').value,f01.initial.draft);
              const axFor=node=>{const r=s.relations.find(r=>r.kind==='corresponds_to'&&same(r.from,node.key));assert(r);return s.nodes.find(n=>same(n.key,r.to));};
              assert.equal(property(axFor(popup),'role').value,'dialog');
              assert.equal(property(axFor(popup),'accessibility_name').value,'Options');
              assert.equal(property(axFor(input),'role').value,'combobox');
              assert.equal(property(axFor(input),'accessibility_name').value,'City');
              assert.equal(s.focus.keyboard.status,'unknown');assert.equal(s.focus.active_descendant.status,'unknown');
            } else if (payload.kind === 'left') {
              compareRect(rectangle(dom[0]),r01.initial.left.bounds);
              const ax = s.nodes.filter(n=>n.key.namespace==='web.ax'); assert.equal(ax.length,1);
              assert.equal(property(ax[0],'role').value,f01.B06.ax_role);
              assert.equal(property(ax[0],'accessibility_name').value,f01.B06.ax_name);
              assert.equal(ax[0].properties.find(p=>p.field==='layout_bounds').state.availability,'unknown');
              assert.notDeepEqual(dom[0].key,ax[0].key);
            } else if (payload.kind === 'sized_before' || payload.kind === 'sized_after') {
              compareRect(rectangle(dom[0]),payload.kind==='sized_before'?f01.B04.text_before:f01.B04.text_after);
            } else if (payload.kind === 'private') {
              const value = dom[0].properties.find(p=>p.field==='value');
              assert.equal(value.state.availability,'redacted'); assert.equal(value.sensitivity,'sensitive');
              assert(!JSON.stringify(doc).includes(CANARY));
            } else throw new Error('unknown_oracle');
          }
          if (Object.hasOwn(FRAME_FILES,payload.case)) {
            assert(payload.document!==null,'canonical success required');
            assert(typeof payload.canonical==='string'); const bytes=Buffer.from(payload.canonical,'utf8');
            assert(bytes.length>0&&bytes.length<=65536); assert(!payload.canonical.includes(CANARY));
            assert.deepEqual(JSON.parse(payload.canonical),payload.document,'exact ACKed frame agrees with validated data');
            const file=FRAME_FILES[payload.case];
            await writeExclusive(path.join(evidence,file),bytes);
            report.frames.push({case:payload.case,file,bytes:bytes.length,sha256:crypto.createHash('sha256').update(bytes).digest('hex'),usage:'historical_analysis_only',live_ref_reuse:false});
          } else assert(!Object.hasOwn(payload,'canonical'),'only declared positive frames may be persisted');
          checks.push({case:payload.case,readonly:true,oracle:payload.kind}); return {};
        }
        case 'stimulus': {
          if(['form-start','form-settle','form-lose-focus'].includes(payload.action)){
            assert.equal(report.mode,'form_actions');assert.equal(before.size,0);
            if(payload.action==='form-start'){
              await page.bringToFront();await page.locator('#draft').fill('L');
              await page.waitForFunction(()=>document.getElementById('validation').textContent==='Invalid city');
              await page.locator('#left').focus();
            }else if(payload.action==='form-settle')await page.locator('#option-london').waitFor();
            else await page.locator('#left').focus();
            report.form_action_setup??=[];report.form_action_setup.push(payload.action);
          }else if(Object.hasOwn(FORM_CASES,payload.action)){
            assert.equal(report.mode,'form_reads');assert.equal(before.size,0);
            report.form_setup??=[];
            assert.equal(Object.keys(FORM_CASES)[report.form_setup.length],payload.action);
            const expected=FORM_CASES[payload.action];
            await page.bringToFront(); // addressed own headless page, no desktop input lane
            await page.evaluate(({state,canary})=>{
              const input=document.getElementById('draft'),output=document.getElementById('applied');
              input.setAttribute('autocomplete',state.private?'one-time-code':'');
              Object.getOwnPropertyDescriptor(HTMLInputElement.prototype,'value').set.call(input,state.private?canary:'A💡B');
              Object.getOwnPropertyDescriptor(HTMLOutputElement.prototype,'value').set.call(output,state.output);
              HTMLElement.prototype.focus.call(input,{preventScroll:true});
              HTMLInputElement.prototype.setSelectionRange.call(input,state.start,state.end,state.direction);
            },{state:expected,canary:CANARY}); // setup only: no events, delivery or app commit
            assertFormState(await uiState(page,true),expected);
            report.form_setup.push(payload.action);
          } else if (payload.action==='textLarge' || payload.action==='remount' || payload.action==='parentWide' || payload.action==='fontLarge')
            await page.evaluate(name=>window.f01.operate(name),payload.action);
          else if (payload.action==='action-remount'){assert.equal(report.mode,'actions');await page.locator('#remount-target').click();}
          else if (payload.action==='root-remount') {await page.evaluate(()=>{const root=document.getElementById('f01');root.replaceWith(root.cloneNode(true));});}
          else if (payload.action==='popup') {
            await page.locator('#open-popup').click();
            const declared=await page.evaluate(()=>({parent:document.getElementById('portal').parentElement.tagName,anchor:document.getElementById('portal').dataset.anchor,inputInside:document.getElementById('portal').contains(document.getElementById('draft'))}));
            assert.equal(declared.parent,f01.B03.portal_parent);assert.equal(declared.anchor,f01.B03.anchor);assert.equal(declared.inputInside,false);
          } else if (payload.action==='navigate') {
            await page.goto(`${fixture.url}/?generation=2`); await page.waitForFunction(()=>!!window.f01);
          } else if (payload.action==='private') {
            await page.evaluate(value=>{const input=document.getElementById('draft');input.type='password';
              input.autocomplete='current-password';input.value=value;},CANARY); // setup only; no events/input delivery
          } else throw new Error('unsupported_fixture_stimulus');
          return {};
        }
        case 'browser_alive': await uiState(page); checks.push({case:payload.case,browser_alive_after_worker_reap:true}); return {};
        default: throw new Error('unknown_fixture_command');
      }
    }
    report.phase='guarded_consumer';
    child = spawn(executable,['--ignored','--exact','guarded_live_f01','--nocapture','--test-threads=1'],
      {env:{...process.env,UIB_WEB_LIVE_ALLOW:'1'},stdio:['pipe','pipe','pipe']});
    exited = new Promise((resolve,reject)=>{child.once('error',reject);child.once('exit',(code,signal)=>resolve({code,signal}));});
    let output = ''; let diagnosticBytes = 0; let chain = Promise.resolve();
    child.stdout.setEncoding('utf8');
    child.stdin.on('error',()=>{failed=true;child.kill('SIGKILL');});
    child.stderr.on('data', chunk => { diagnosticBytes += chunk.length; if(diagnosticBytes>MAX_LINE){failed=true;child.kill('SIGKILL');} });
    child.stdout.on('data', chunk => {
      output += chunk; if(Buffer.byteLength(output)>MAX_LINE){failed=true;child.kill('SIGKILL');return;}
      for (;;) {
        const end=output.indexOf('\n'); if(end<0)break;const line=output.slice(0,end);output=output.slice(end+1);
        const marker=line.indexOf(PREFIX); if(marker<0)continue;
        chain=chain.then(async()=>{
          let message;
          try {message=JSON.parse(line.slice(marker+PREFIX.length));const result=await bounded(command(message),8000,'fixture_phase_timeout');
            child.stdin.write(JSON.stringify({sequence:message.sequence,ok:true,result})+'\n');
          } catch (_) {failed=true;report.failure={code:'fixture_phase_failed',sequence:Number.isSafeInteger(message?.sequence)?message.sequence:0};child.stdin.write(JSON.stringify({sequence:message?.sequence??0,ok:false})+'\n');}
        });
      }
    });
    const exit = await bounded(exited,90000,'rust_consumer_timeout'); await chain; report.test_exit={code:exit.code,signal:exit.signal};
    assert.equal(exit.code,0); assert.equal(exit.signal,null); assert(!failed,'live_case_failed');
    assert.equal(before.size,0);
    if(report.mode==='form_reads'){
      assert.deepEqual(report.form_setup,Object.keys(FORM_CASES));
      assert.deepEqual(report.outcomes.map(o=>o.stage),Object.keys(FORM_CASES));
      assert(report.outcomes.every(o=>o.terminal==='completed'&&o.effect==='not_dispatched'&&o.committed===1&&o.missing===0));
      assert.deepEqual(checks.filter(c=>c.oracle==='form_reads').map(c=>c.case),Object.keys(FORM_CASES));
      assert.equal(report.frames.length,3);
    }else if(report.mode==='actions'){assert.equal(report.outcomes.length,10);assert(checks.some(c=>c.case==='action-unknown'));assert.equal(report.frames.length,3);}else if(report.mode==='b05'){assert.equal(report.outcomes.length,6);assert(checks.some(c=>c.case==='b05-history-font'));assert.equal(report.frames.length,3);}else if(report.mode==='rooted'){assert.equal(report.outcomes.length,4);assert(checks.some(c=>c.case==='rooted-stale'));assert.equal(report.frames.length,1);}else if(report.mode==='popup_relations'){assert(checks.some(c=>c.case==='popup-context'));assert.equal(report.frames.length,1);}else if(report.mode==='first_observe_diagnostic'){assert(checks.some(c=>c.case==='left-initial'));assert.equal(report.frames.length,1);}else{assert(checks.length>=10,'all finite cases completed');assert.equal(report.frames.length,3,'all declared positive evidence captured');}
  } catch (_) {
    report.failure??={code:'live_run_failed',phase:report.phase}; throw new Error('live_run_failed');
  } finally {
    clearTimeout(whole); report.phase='cleanup';
    const errors=[];
    async function clean(name,exists,action){
      if(!exists){report.cleanup[name]='not_created';return;}
      try {await action();report.cleanup[name]='confirmed';}catch(_){report.cleanup[name]='unconfirmed';errors.push(name);}
    }
    await clean((directorCase||['cli_actions','geometry','first_use','form_actions'].includes(report.mode))?'cli_process':'test_process',!!child,async()=>{
      if(child.exitCode===null&&child.signalCode===null)child.kill('SIGKILL');
      if(exited)await bounded(exited,3000,'owned_test_reap_timeout');
    });
    await clean('context',!!context,()=>bounded(context.close(),3000,'context_cleanup_timeout'));
    await clean('driver_connection',!!browser,()=>bounded(browser.close(),3000,'driver_cleanup_timeout'));
    await clean('owned_browser',!!server,async()=>{try{await bounded(server.close(),4000,'browser_cleanup_timeout');}catch(_){await bounded(server.kill(),3000,'owned_browser_kill_timeout');}});
    await clean('fixture_server',!!fixture,()=>bounded(fixture.close(),3000,'server_cleanup_timeout'));
    await clean('owned_profile',!!profile,async()=>{try{await fs.stat(profile);}catch(error){if(error.code==='ENOENT')return;throw error;}throw new Error('profile_not_removed');});
    report.cleanup.worker_sessions=(directorCase||['cli_actions','geometry','first_use','form_actions'].includes(report.mode))
      ?(report.cli_cleanup_confirmed===true&&ownWorkerPids(worker).length===0?'confirmed_closed':'unconfirmed')
      :report.worker_cleanup?.confirmed===true&&report.worker_cleanup.reserved_sessions===0?'confirmed_closed':child?'unconfirmed':'not_created';
    report.pending_case_count=before.size;
    if(errors.length){report.failure={code:'owned_cleanup_unconfirmed',count:errors.length};throw new Error('cleanup_failed');}
  }
}
async function main(){
  if(!process.argv.includes('--run-authorized')||process.env.UIB_WEB_LIVE_ALLOW!=='1')throw new Error('explicit_live_activation_required');
  const mode=process.env.UIB_WEB_LIVE_CASE||'full';assert(['full','first_observe_diagnostic','popup_relations','rooted','b05','actions','form_reads','form_actions','cli_actions','geometry','director','director_semantics','first_use'].includes(mode));
  const evidence=process.env.UIB_WEB_LIVE_EVIDENCE;
  assert(evidence&&path.isAbsolute(evidence)&&evidence===path.join(EVIDENCE_ROOT,path.basename(evidence)));
  assert(/^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/.test(path.basename(evidence)),'fresh UUID directory required');
  assert.equal(await fs.realpath(EVIDENCE_ROOT),EVIDENCE_ROOT,'evidence parent must not redirect');
  await fs.mkdir(evidence,{mode:0o700}); // exclusive: EEXIST refuses before any launch
  const report={status:'failed',mode,outcomes:[],kind:'guarded-real-chromium-finite-scope',phase:'preflight',started_utc:new Date().toISOString(),
    retention:{owner:'Web-current-operation',consumers:[['geometry','director','director_semantics','first_use'].includes(mode)?'Web-component-geometry':mode==='form_actions'?'A03-public-Focus-Type':mode==='cli_actions'?'L01-public-CLI-action-qualification':mode==='form_reads'?'W02-form-read-qualification':'B03-result-and-immediate-measurement'],until:'current operation result accepted and consumed; remove owned directory and verify removal'},
    limits:{nodes:32,depth:8,output_bytes:65536,request_ms:250,traversal_nodes:256},checks:[],frames:[],cleanup:{test_process:'not_created',context:'not_created',driver_connection:'not_created',owned_browser:'not_created',fixture_server:'not_created',owned_profile:'not_created',worker_sessions:'not_created'}};
  try {await run(evidence,report);report.status=mode==='first_observe_diagnostic'?'diagnostic_passed':'passed';report.phase='complete';}
  catch(_){report.failure??={code:'live_run_failed'};process.exitCode=1;}
  report.finished_utc=new Date().toISOString();
  report.record_use={historical:true,live_ref_reuse:false,closed_sessions:report.cleanup.worker_sessions==='confirmed_closed'};
  const summary=JSON.stringify(report)+'\n';assert(Buffer.byteLength(summary)<=65536);
  try {await writeExclusive(path.join(evidence,'report.json'),summary);}
  catch(_){process.stdout.write(JSON.stringify({...report,status:'failed',evidence_write:'failed'})+'\n');throw new Error('evidence_write_failed');}
  process.stdout.write(JSON.stringify({status:report.status,evidence,report:'report.json'})+'\n');
}

if (require.main===module) main().catch(()=>{console.error('W01 live driver failed; raw diagnostics suppressed.');process.exitCode=1;});
