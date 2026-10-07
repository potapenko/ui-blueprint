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
const EVIDENCE_ROOT = '/Users/eugenepotapenko/Library/Application Support/UIBlueprint/development/P2/W01-guarded-live';
const CASES = new Set(['left-initial','sized-before','sized-after','private','cross-target','old-remount-ref','new-remount-binding','old-navigation-ref','new-document-binding']);
const TERMINALS = new Set(['completed','cancelled','timed_out','invalid_limits','resource_limit','allocation_failure','overflow','busy','invalid_input','invalid_state','invalid_control','stale_operation','deadline_expired','permission_denied','io','worker_failed','system_allocation_failure','cleanup_pending','resync_required']);
const FRAME_FILES = Object.freeze({'left-initial':'initial-left.json','sized-before':'sized-before.json','sized-after':'sized-after.json'});
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
async function uiState(page) {
  // Private canary is compared in memory only; never serialize this checkpoint to evidence.
  return page.evaluate(() => ({active: document.activeElement?.id || '',
    scroll: [scrollX, scrollY], state: window.f01.checkpoint()}));
}
async function run(evidence, report) {
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
  let fixture, server, browser, context, child, exited, profile;
  const checks = report.checks; const before = new Map(); let failed = false;
  Object.assign(report,{sourceHashes,binaries:{test:process.env.UIB_WEB_LIVE_TEST_SHA256,worker:process.env.UIB_WEB_LIVE_WORKER_SHA256},environment:{node:process.version,playwright:setup.runtimeVersion,platform:process.platform,arch:process.arch,os_release:require('node:os').release()},fixture:setup.fixtureCommit});
  report.phase='fixture_setup';
  const whole = setTimeout(() => { failed = true; child?.kill('SIGKILL'); if(server) void server.kill().catch(()=>{}); }, 120000);
  try {
    fixture = await start();
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
    context = await browser.newContext({viewport:{width:800,height:600},deviceScaleFactor:1});
    await context.route('**/*', route => new URL(route.request().url()).origin === fixture.url ? route.continue() : route.abort());
    const pages = {};
    for (const name of ['a','b']) {
      const page = await context.newPage(); page.setDefaultTimeout(2000); page.setDefaultNavigationTimeout(3000);
      await page.goto(`${fixture.url}/?generation=1`); await page.waitForFunction(() => !!window.f01);
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
    async function command(message) {
      const payload = message.payload; const page = pages[payload.page]; assert(page, 'owned page only');
      switch (message.command) {
        case 'binding': return binding(page);
        case 'before': assert(CASES.has(payload.case)); report.last_stage=payload.case; before.set(payload.case, await uiState(page)); return {};
        case 'outcome': {
          assert(CASES.has(payload.stage)&&TERMINALS.has(payload.terminal));
          assert(Number.isSafeInteger(payload.operation)&&payload.operation>0);
          assert([payload.committed,payload.missing].every(n=>Number.isInteger(n)&&n>=0&&n<=7));
          const d=payload.diagnostic;
          if(d!==null){assert(d&&Object.keys(d).sort().join(',')==='cause,code,count,remote_cleanup,send_progress,stage');assert(Number.isInteger(d.stage)&&d.stage>=1&&d.stage<=9);assert(Number.isInteger(d.cause)&&d.cause>=1&&d.cause<=44);assert(Number.isInteger(d.remote_cleanup)&&d.remote_cleanup>=0&&d.remote_cleanup<=2);assert(Number.isInteger(d.send_progress)&&d.send_progress>=0&&d.send_progress<=3);assert(Number.isInteger(d.code)&&d.code>=-2147483648&&d.code<=2147483647);assert(Number.isInteger(d.count)&&d.count>=0&&d.count<=4294967295);}
          assert(report.outcomes.length<16);report.outcomes.push({stage:payload.stage,terminal:payload.terminal,committed:payload.committed,missing:payload.missing,operation:payload.operation,diagnostic:d});return {};
        }
        case 'worker_cleanup': {
          assert(typeof payload.confirmed==='boolean'&&typeof payload.abandoned==='boolean');
          assert(Number.isInteger(payload.reserved_sessions)&&payload.reserved_sessions>=0&&payload.reserved_sessions<=4);
          assert(Number.isInteger(payload.completion_groups)&&payload.completion_groups>=0&&payload.completion_groups<=8);
          report.worker_cleanup={confirmed:payload.confirmed,reserved_sessions:payload.reserved_sessions,completion_groups:payload.completion_groups,abandoned:payload.abandoned};return {};
        }
        case 'check': {
          assert(CASES.has(payload.case));
          assert(before.has(payload.case)); assert.deepEqual(await uiState(page),before.get(payload.case)); before.delete(payload.case);
          if (payload.document !== null) {
            const doc = payload.document; assert.equal(doc.schema_version,'0.1.0');
            assert.equal(doc.artifact.kind,'channel_response'); const response = doc.artifact.data;
            assert.equal(response.result.status,'observed'); const s = response.result.data;
            assert.equal(s.coverage.status,'partial'); assert.equal(s.source_state,null);
            assert(s.nodes.length <= 32); const dom = s.nodes.filter(n=>n.key.namespace==='web.dom');
            assert.equal(dom.length,1); assert(s.observations.every(o=>o.freshness==='current'&&o.consistency==='unknown'));
            if (payload.kind === 'left') {
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
          } else assert(!Object.hasOwn(payload,'canonical'),'only three declared positive frames may be persisted');
          checks.push({case:payload.case,readonly:true,oracle:payload.kind}); return {};
        }
        case 'stimulus': {
          if (payload.action==='textLarge' || payload.action==='remount')
            await page.evaluate(name=>window.f01.operate(name),payload.action);
          else if (payload.action==='navigate') {
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
    assert.equal(before.size,0); if(report.mode==='first_observe_diagnostic'){assert(checks.some(c=>c.case==='left-initial'));assert.equal(report.frames.length,1);}else{assert(checks.length>=10,'all finite cases completed');assert.equal(report.frames.length,3,'all declared positive evidence captured');}
  } catch (_) {
    report.failure??={code:'live_run_failed',phase:report.phase}; throw new Error('live_run_failed');
  } finally {
    clearTimeout(whole); report.phase='cleanup';
    const errors=[];
    async function clean(name,exists,action){
      if(!exists){report.cleanup[name]='not_created';return;}
      try {await action();report.cleanup[name]='confirmed';}catch(_){report.cleanup[name]='unconfirmed';errors.push(name);}
    }
    await clean('test_process',!!child,async()=>{
      if(child.exitCode===null&&child.signalCode===null)child.kill('SIGKILL');
      if(exited)await bounded(exited,3000,'owned_test_reap_timeout');
    });
    await clean('context',!!context,()=>bounded(context.close(),3000,'context_cleanup_timeout'));
    await clean('driver_connection',!!browser,()=>bounded(browser.close(),3000,'driver_cleanup_timeout'));
    await clean('owned_browser',!!server,async()=>{try{await bounded(server.close(),4000,'browser_cleanup_timeout');}catch(_){await bounded(server.kill(),3000,'owned_browser_kill_timeout');}});
    await clean('fixture_server',!!fixture,()=>bounded(fixture.close(),3000,'server_cleanup_timeout'));
    await clean('owned_profile',!!profile,async()=>{try{await fs.stat(profile);}catch(error){if(error.code==='ENOENT')return;throw error;}throw new Error('profile_not_removed');});
    report.cleanup.worker_sessions=report.worker_cleanup?.confirmed===true&&report.worker_cleanup.reserved_sessions===0?'confirmed_closed':child?'unconfirmed':'not_created';
    report.pending_case_count=before.size;
    if(errors.length){report.failure={code:'owned_cleanup_unconfirmed',count:errors.length};throw new Error('cleanup_failed');}
  }
}
async function main(){
  if(!process.argv.includes('--run-authorized')||process.env.UIB_WEB_LIVE_ALLOW!=='1')throw new Error('explicit_live_activation_required');
  const mode=process.env.UIB_WEB_LIVE_CASE||'full';assert(['full','first_observe_diagnostic'].includes(mode));
  const evidence=process.env.UIB_WEB_LIVE_EVIDENCE;
  assert(evidence&&path.isAbsolute(evidence)&&evidence===path.join(EVIDENCE_ROOT,path.basename(evidence)));
  assert(/^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/.test(path.basename(evidence)),'fresh UUID directory required');
  await fs.mkdir(EVIDENCE_ROOT,{recursive:true,mode:0o700});
  assert.equal(await fs.realpath(EVIDENCE_ROOT),EVIDENCE_ROOT,'evidence parent must not redirect');
  await fs.mkdir(evidence,{mode:0o700}); // exclusive: EEXIST refuses before any launch
  const report={status:'failed',mode,outcomes:[],kind:'guarded-real-chromium-finite-scope',phase:'preflight',started_utc:new Date().toISOString(),
    retention:{owner:'root',consumers:['W01-review','G02','P7'],until:'P7 acceptance or explicit discard/replacement'},
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
