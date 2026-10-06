// Tooling harness: verifies its synthetic application, not a production adapter.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const os = require('node:os');
const crypto = require('node:crypto');
const { performance } = require('node:perf_hooks');
const { start } = require('./server.cjs');
const pwPath = process.env.F01_PLAYWRIGHT_CORE || 'playwright-core';
const { chromium } = require(pwPath);
const expected = JSON.parse(fs.readFileSync(path.join(__dirname, 'expected.json')));
const config = expected.baseline_sampling;
const checks = [], checkpoints = [], calls = [];
const browsers = new Set();
let server, environment;
let phase = 'setup', generation = 0;
const watchdog = setTimeout(() => {
  console.error('F01 overall 60s deadline exceeded');
  Promise.allSettled([...browsers].map(b => b.close())).finally(() => process.exit(1));
  setTimeout(() => process.exit(1), 2000).unref();
}, 60000);
function check(name, actual, wanted, evidence = 'fixture') {
  assert.deepEqual(actual, wanted, name);
  checks.push({ scenario: phase, name, expected: wanted, observed: actual, evidence });
}
function geometry(name, actual, wanted) {
  assert.equal(actual.length, wanted.length, name);
  assert.ok(actual.every((v, i) => Math.abs(v - wanted[i]) <= expected.geometry_tolerance_css_px), `${name}: ${actual} != ${wanted}`);
  checks.push({ scenario: phase, name, expected: wanted, observed: actual, evidence: 'CSSOM reported css_px' });
}
async function bounded(promise, ms = 3000) {
  let timer;
  try { return await Promise.race([promise, new Promise((_, reject) => { timer = setTimeout(() => reject(new Error('F01 external-call deadline')), ms); })]); }
  finally { clearTimeout(timer); }
}
async function closeBrowser(browser) { await bounded(browser.close()); browsers.delete(browser); }
async function launch() {
  const browser = await chromium.launch({ headless: true, timeout: 8000 }); browsers.add(browser);
  const context = await browser.newContext({ viewport: config.viewport, deviceScaleFactor: config.dpr });
  await context.route('**/*', route => new URL(route.request().url()).origin === server.url ? route.continue() : route.abort());
  const page = await context.newPage(); page.setDefaultTimeout(2000); page.setDefaultNavigationTimeout(3000);
  return { browser, context, page };
}
async function reset(page) {
  await page.setViewportSize(config.viewport);
  await page.goto(`${server.url}/?generation=${++generation}`);
  await page.waitForFunction(() => !!window.f01);
  return generation;
}
async function checkpoint(page, label) {
  const state = await page.evaluate(() => window.f01.checkpoint());
  checkpoints.push({ label, state }); return state;
}
async function operate(page, name) { await page.evaluate(name => window.f01.operate(name), name); }
async function rect(page, selector) { return page.locator(selector).evaluate(n => { const r = n.getBoundingClientRect(); return [r.x, r.y, r.width, r.height]; }); }
async function dimensions(page) { return (await rect(page, '#mutation-child')).slice(2); }
async function attach(context, page) {
  const cdp = await context.newCDPSession(page);
  async function send(method, params = {}) {
    const start = performance.now(); const result = await bounded(cdp.send(method, params));
    calls.push({ phase, method, roundtrip_ms: performance.now() - start, bytes: Buffer.byteLength(JSON.stringify(result)) });
    return result;
  }
  await send('DOM.enable'); await send('Accessibility.enable');
  const { targetInfo } = await send('Target.getTargetInfo');
  return { send, target: targetInfo.targetId };
}
async function node(owner, selector) {
  // Resolve in the current document; frontend node IDs can be invalidated after navigation.
  const { result } = await owner.send('Runtime.evaluate', {
    expression: `document.querySelector(${JSON.stringify(selector)})`, objectGroup: 'f01-node'
  });
  assert.equal(result.subtype, 'node', 'exact fixture selector resolves a DOM node');
  try { return (await owner.send('DOM.describeNode', { objectId: result.objectId })).node; }
  finally { await owner.send('Runtime.releaseObject', { objectId: result.objectId }); }
}
async function full(owner) {
  return owner.send('DOMSnapshot.captureSnapshot', { computedStyles: [], includeDOMRects: true });
}
async function semantic(owner, backend) {
  return owner.send('Accessibility.getPartialAXTree', { backendNodeId: backend, fetchRelatives: false });
}
async function geometryQuery(owner, objectId) {
  return owner.send('Runtime.callFunctionOn', { objectId, returnByValue: true,
    functionDeclaration: 'function(){const r=this.getBoundingClientRect();return [r.x,r.y,r.width,r.height];}' });
}
async function selectCity(page) {
  await page.locator('#draft').pressSequentially(expected.B02.valid_draft);
  await page.locator('#option-london').click(); await page.locator('#commit').click();
}
async function scenarios() {
  const { browser, context, page } = await launch();
  try {
    phase = 'B01';
    const g1 = await reset(page);
    const initial = await checkpoint(page, 'reset-initial');
    const { revision, generation: ignored, ...semanticState } = initial;
    check('reset source state', semanticState, expected.initial);
    const owner = await attach(context, page);
    const protocol = await owner.send('Browser.getVersion');
    environment = { node: process.version, playwright: require(path.join(pwPath, 'package.json')).version,
      chromium: browser.version(), protocol, platform: os.platform(), release: os.release(), arch: os.arch(),
      cpu: os.cpus()[0].model, logical_cpus: os.cpus().length, host_memory_bytes: os.totalmem(), viewport: config.viewport, dpr: config.dpr };
    const other = await context.newPage(); await reset(other);
    const otherOwner = await attach(context, other);
    check('two exact targets', owner.target !== otherOwner.target, true, 'CDP target identity');
    check('same title cannot identify target', await page.title(), await other.title());
    check('duplicate main labels', await page.getByRole('button', { name: 'Apply', exact: true }).count(), expected.B01.main_apply_count);
    const handle = await page.$('#left'); const beforeNode = await node(owner, '#left');
    await operate(page, 'remount');
    const afterNode = await node(owner, '#left');
    check('remount gets new backend ID', afterNode.backendNodeId !== beforeNode.backendNodeId, true, 'CDP identity');
    let rejected = false;
    try { await handle.click({ timeout: 250 }); } catch (error) { rejected = /not attached/.test(error.message); }
    check('old handle detached', rejected, true, 'actual pointer API rejects detached handle');
    check('other target unchanged', (await other.evaluate(() => window.f01.checkpoint())).source_state === 'initial', expected.B01.other_target_unchanged);
    const frameBefore = (await owner.send('Page.getFrameTree')).frameTree.frame.loaderId;
    const g2 = await reset(page);
    check('reset generation advances', g2 > g1, true);
    check('navigation changes loader', (await owner.send('Page.getFrameTree')).frameTree.frame.loaderId !== frameBefore, true, 'CDP identity');
    const restored = await checkpoint(page, 'reset-reproduced');
    check('reset semantic state reproducible', { ...restored, generation: 1 }, { ...initial, generation: 1 });
    await other.close();

    phase = 'B02'; await reset(page);
    await page.locator('#draft').pressSequentially(expected.B02.partial_draft);
    let state = await checkpoint(page, 'partial-draft');
    check('keyboard draft retained', state.draft, expected.B02.partial_draft);
    check('draft does not apply', state.applied, '');
    check('debounce pending explicit', state.source_state, 'draft-pending');
    await page.waitForFunction(() => window.f01.checkpoint().source_state === 'draft-invalid');
    await page.locator('#commit').click(); state = await checkpoint(page, 'validation-rejected');
    check('invalid commit rejected', state.applied, expected.B02.rejected_applied);
    check('rejection is delivered but not applied', [state.delivered, state.valid], [1, false]);
    await reset(page); await selectCity(page);
    state = await checkpoint(page, 'valid-city-applied');
    check('selected suggestion applied', [state.selected, state.applied, state.valid], ['London', expected.B02.applied, true]);
    await page.locator('#surprise').click();
    // Test-driver stop policy only: a fresh observed transition prevents the next step.
    let dependentSteps = 0;
    const observedUnexpected = await page.locator('#unexpected').count();
    if (!observedUnexpected) { await page.locator('#commit').click(); dependentSteps++; }
    check('unexpected transition visible', observedUnexpected, 1);
    check('dependent step suppressed by driver', dependentSteps, expected.B02.dependent_steps_after_surprise, 'harness policy; not production executor');
    check('fixture disables stale form path', await page.locator('#commit').isDisabled(), true);

    phase = 'B03'; await reset(page);
    await page.locator('#open-popup').click();
    check('portal outside form subtree', await page.locator('#portal').evaluate(n => n.parentElement.tagName), expected.B03.portal_parent);
    check('explicit anchor', await page.locator('#portal').getAttribute('data-anchor'), expected.B03.anchor);
    check('portal focused', await page.evaluate(() => document.activeElement.id), 'close-popup');
    await page.locator('#close-popup').click();
    check('portal dismissed and focus returned', [await page.locator('#portal').count(), await page.evaluate(() => document.activeElement.id)], [0, 'open-popup']);
    await operate(page, 'overlay');
    const hit = () => page.evaluate(() => { const r = document.querySelector('#left').getBoundingClientRect(); const e = document.elementFromPoint(r.x + r.width / 2, r.y + r.height / 2); return e?.closest('button')?.id || e?.id; });
    check('overlay owns hit', await hit(), expected.B03.blocked_hit, 'DOM hit-test');
    let intercepted = false;
    try { await page.locator('#left').click({ timeout: 250 }); } catch (error) { intercepted = /intercepts pointer events/.test(error.message); }
    check('overlay blocks delivery', intercepted, true, 'actual pointer API');
    check('no blocked click delivered', await page.locator('#outcome').getAttribute('data-delivered'), '0');
    await operate(page, 'removeOverlay'); check('uncovered target hit', await hit(), expected.B03.visible_hit, 'DOM hit-test');
    await page.locator('#left').click();
    check('uncovered pointer delivered', await page.locator('#outcome').getAttribute('data-delivered'), '1');
    // Pointer actionability may scroll. Restore the declared geometry precondition explicitly.
    await page.evaluate(() => scrollTo(0,0));
    geometry('clipped child retains layout bounds', await rect(page, '#clip-child'), expected.B03.clip_bounds);
    const clip = await page.evaluate(() => { const a=document.querySelector('#clip').getBoundingClientRect(), b=document.querySelector('#clip-child').getBoundingClientRect();return { intersection: Math.max(0, Math.min(a.right,b.right)-Math.max(a.left,b.left)), inside:document.elementFromPoint(130,410).id, outside:document.elementFromPoint(160,410).id }; });
    check('clip intersection width', clip.intersection, expected.B03.clip_visible_width, 'derived rect intersection; not arbitrary visible-region proof');
    check('clip hit samples', [clip.inside, clip.outside === 'clip-child'], ['clip-child', false], 'DOM two-point hit-test only');
    const child = page.frames().find(f => f.parentFrame());
    geometry('child frame local rect', await child.locator('#child-button').evaluate(n => {const r=n.getBoundingClientRect();return [r.x,r.y,r.width,r.height];}), expected.B03.child_bounds);
    checkpoints.push({label:'B03-coverage', coverage:'partial', unavailable:['OOPIF','arbitrary occlusion','cross-frame transforms'], source:'harness scope declaration'});

    phase = 'B04'; await reset(page); await selectCity(page);
    geometry('responsive at 800px', await rect(page, '#responsive'), expected.B04.responsive_before);
    await page.setViewportSize({width:1000,height:600});
    geometry('responsive at 1000px', await rect(page, '#responsive'), expected.B04.responsive_after_resize);
    await page.evaluate(() => scrollTo(0,100));
    geometry('viewport rect after scroll', await rect(page, '#responsive'), expected.B04.responsive_after_scroll);
    check('document transform scroll amount', await page.evaluate(() => scrollY), expected.B04.scroll_y);
    await page.evaluate(() => scrollTo(0,0));
    geometry('text-sized box before', await rect(page, '#sized'), expected.B04.text_before);
    await operate(page, 'textLarge'); geometry('text-sized box after', await rect(page, '#sized'), expected.B04.text_after);
    await operate(page, 'locale');
    check('locale and label', [await page.locator('html').getAttribute('lang'), await page.locator('#responsive').textContent()], [expected.B04.locale,expected.B04.localized_label]);
    check('locale preserves applied value', (await checkpoint(page, 'B04-locale')).applied, expected.B02.applied);

    phase = 'B05'; await reset(page);
    await page.evaluate(() => {
      window.f01Received = [];
      document.addEventListener('fixture-change', e => { if (e.detail.seq !== 2) window.f01Received.push(e.detail); });
    });
    await checkpoint(page, 'B05-source-0'); geometry('initial child size', await dimensions(page), expected.B05.child_before);
    await operate(page, 'parentWide'); geometry('parent dependency', await dimensions(page), expected.B05.child_after_parent); await checkpoint(page, 'B05-source-1');
    await operate(page, 'fontLarge'); geometry('font dependency', await dimensions(page), expected.B05.child_after_font); await checkpoint(page, 'B05-source-2');
    await operate(page, 'rowsChanged'); state = await checkpoint(page, 'B05-source-3');
    check('literal final row state', state.rows, expected.B05.rows_after);
    const deliveredEvents = await page.evaluate(() => window.f01Received.map(e=>e.seq));
    check('deliberately dropped event visible', deliveredEvents, expected.B05.delivered_event_sequences, 'synthetic fixture notification subscriber; not browser event reliability');
    check('source journal complete', await page.evaluate(() => window.f01.events.map(e=>e.seq)), expected.B05.full_event_sequences);
    check('gap stimulus detected', deliveredEvents[1] - deliveredEvents[0] > 1, true, 'harness gap detection only');
    const allRows = await page.locator('[data-row]').evaluateAll(ns => ns.map(n=>({id:n.dataset.row,label:n.textContent})));
    const limited = { rows: allRows.slice(0,expected.B05.limit), coverage:'partial', omitted_count:allRows.length-expected.B05.limit, source_state:state.source_state };
    check('limited output declares omission', [limited.rows.length,limited.omitted_count,limited.coverage], [2,expected.B05.omitted_count,'partial'], 'harness response shaping; full acquisition');
    check('limit does not delete source rows', await page.locator('[data-row]').count(), 6);
    checkpoints.push({label:'B05-limited-view', ...limited});
    // Repeating reads at the same source checkpoint is the future replay oracle input.
    check('frozen checkpoint repeatability', (await checkpoint(page,'B05-source-3-repeat')).rows, allRows);

    phase = 'B06'; await reset(page);
    const leftNode = await node(owner,'#left'); const ax = await semantic(owner,leftNode.backendNodeId);
    const button = ax.nodes.find(n=>n.backendDOMNodeId===leftNode.backendNodeId);
    check('AX interaction control', [button.role.value,button.name.value], [expected.B06.ax_role,expected.B06.ax_name], 'CDP AX reported');
    check('explicit component mapping', await page.locator('#left').getAttribute('data-component-key'), expected.B06.component, 'fixture source declaration');
    check('DOM design parts', await page.locator('#left > span').evaluateAll(ns=>ns.map(n=>n.id)), expected.B06.dom_parts, 'DOM source nodes');
    const parts = await Promise.all(expected.B06.dom_parts.map(id=>node(owner,`#${id}`)));
    check('separate DOM identities', new Set([leftNode.backendNodeId,...parts.map(n=>n.backendNodeId)]).size, 3, 'CDP DOM identity');
    check('decoration is not a button', await page.locator('#compound-icon').evaluate(n=>[n.tagName,n.getAttribute('aria-hidden')]), ['SPAN','true']);
    check('fixture action owner', await page.locator('#compound-label').evaluate(n=>n.closest('button').id), expected.B06.action_owner);
    await page.locator('#left').click(); check('compound pointer delivered once',await page.locator('#outcome').getAttribute('data-delivered'),'1');
  } finally { await closeBrowser(browser); }
}
function stats(samples) {
  if (!samples.length) return {availability:'unknown'};
  const sorted=[...samples].sort((a,b)=>a-b), percentile=p=>sorted[Math.ceil(p*sorted.length)-1];
  return {n:samples.length,p50:percentile(.5),p95:percentile(.95),min:sorted[0],max:sorted.at(-1)};
}
async function baseline() {
  const cold=[];
  for (let i=0;i<config.cold_samples;i++) {
    phase='baseline-cold'; const start=performance.now(); const run=await launch();
    try {
      const launched=performance.now(); await reset(run.page); const loaded=performance.now();
      const owner=await attach(run.context,run.page); const attached=performance.now();
      const raw=await full(owner); const captured=performance.now();
      cold.push({launch_ms:launched-start,fixture_load_ms:loaded-launched,attach_ms:attached-loaded,
        capture_ms:captured-attached,attach_capture_ms:captured-loaded,total_ms:captured-start,
        bytes:Buffer.byteLength(JSON.stringify(raw)),documents:raw.documents.length,nodes:raw.documents.reduce((n,d)=>n+d.nodes.nodeName.length,0)});
    } finally { await closeBrowser(run.browser); }
  }
  phase='baseline-warm'; const run=await launch();
  try {
    await reset(run.page); const owner=await attach(run.context,run.page);
    const {backendNodeId}=await node(owner,'#left');
    const remote=await owner.send('DOM.resolveNode',{backendNodeId,objectGroup:'f01-baseline'});
    await owner.send('Performance.enable');
    const before=await checkpoint(run.page,'baseline-fixed-before');
    const warm={semantic:[],geometry:[],full:[]};
    for (let i=0;i<config.warm_samples;i++) {
      for (const [name,fn] of [['semantic',()=>semantic(owner,backendNodeId)],['geometry',()=>geometryQuery(owner,remote.object.objectId)],['full',()=>full(owner)]]) {
        const start=performance.now();const value=await fn();warm[name].push({ms:performance.now()-start,bytes:Buffer.byteLength(JSON.stringify(value))});
      }
    }
    phase='baseline-overhead'; const overhead=[];
    for (let pair=0;pair<config.overhead_pairs;pair++) {
      for (const mode of (pair%2 ? ['on','off'] : ['off','on'])) {
        await run.page.evaluate(() => {
          const s=window.f01Raf={active:true,gaps:[],last:null};
          function tick(t){if(!s.active)return;if(s.last!==null)s.gaps.push(t-s.last);s.last=t;requestAnimationFrame(tick);}requestAnimationFrame(tick);
        });
        const metricsBefore=await owner.send('Performance.getMetrics');
        const start=performance.now();let count=0;
        if(mode==='on'){while(performance.now()-start<config.window_ms){await full(owner);count++;}}
        else await new Promise(resolve=>setTimeout(resolve,config.window_ms));
        const elapsed=performance.now()-start;
        const metricsAfter=await owner.send('Performance.getMetrics');
        const gaps=await run.page.evaluate(()=>{window.f01Raf.active=false;return window.f01Raf.gaps;});
        const metric=(m,key)=>m.metrics.find(x=>x.name===key)?.value;
        const taskBefore=metric(metricsBefore,'TaskDuration'),taskAfter=metric(metricsAfter,'TaskDuration');
        overhead.push({pair,mode,elapsed_ms:elapsed,full_captures:count,raf_gaps_ms:gaps,
          renderer_task_duration_ms:taskBefore===undefined||taskAfter===undefined?{availability:'unsupported'}:(taskAfter-taskBefore)*1000,
          renderer_js_heap_used_bytes:metric(metricsAfter,'JSHeapUsedSize')??{availability:'unsupported'}});
      }
    }
    const after=await checkpoint(run.page,'baseline-fixed-after');
    check('baseline source state unchanged',after,before,'fixture source-state checkpoint');
    await owner.send('Runtime.releaseObjectGroup',{objectGroup:'f01-baseline'});
    const callSummary=Object.fromEntries([...new Set(calls.map(c=>c.phase))].map(p=>[p,{count:calls.filter(c=>c.phase===p).length,bytes:calls.filter(c=>c.phase===p).reduce((s,c)=>s+c.bytes,0)}]));
    return {method:config,cold,warm,overhead,
      summary:{cold:Object.fromEntries(['launch_ms','fixture_load_ms','attach_ms','capture_ms','attach_capture_ms','total_ms','bytes'].map(k=>[k,stats(cold.map(s=>s[k]))])),
        warm:Object.fromEntries(Object.entries(warm).map(([k,v])=>[k,{ms:stats(v.map(s=>s.ms)),bytes:stats(v.map(s=>s.bytes))}])),
        overhead:Object.fromEntries(['off','on'].map(mode=>[mode,{raf_gap_ms:stats(overhead.filter(s=>s.mode===mode).flatMap(s=>s.raf_gaps_ms)),renderer_task_ms:stats(overhead.filter(s=>s.mode===mode).map(s=>s.renderer_task_duration_ms).filter(Number.isFinite))}])),calls:callSummary},
      node_rss_bytes:process.memoryUsage().rss,
      unavailable:['browser/OS total RSS','production cache bytes/resync count','separate transport vs browser API CPU time','Rust normalization/match/diff/format stages','model text/image tokens','real-product overhead','production quality comparison']};
  } finally {await closeBrowser(run.browser);}
}
// Supplement mode leaves the historical scenarios/baseline functions unchanged.
async function fixedOverhead() {
  phase='fixed-5hz-supplement';
  assert.ok(process.env.F01_BASELINE_REPORT && process.env.F01_OUTPUT, 'supplement requires baseline and separate output');
  const baselinePath=path.resolve(process.env.F01_BASELINE_REPORT);
  assert.notEqual(path.resolve(process.env.F01_OUTPUT),baselinePath,'never overwrite frozen baseline');
  const frozenBytes=fs.readFileSync(baselinePath), frozen=JSON.parse(frozenBytes);
  assert.equal(frozen.status,'pass');
  const hash=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
  const priorHarnessPath=path.join(path.dirname(process.env.F01_OUTPUT),'baseline-run.cjs');
  const priorHarness=fs.readFileSync(priorHarnessPath);
  assert.equal(hash(priorHarness),frozen.source_hashes['fixtures/web/run.cjs'],'frozen harness retained exactly');
  const sourceHashes={};
  for(const [file,wanted] of Object.entries(frozen.source_hashes)) {
    sourceHashes[file]=hash(fs.readFileSync(path.join(__dirname,'../..',file)));
    if(file!=='fixtures/web/run.cjs') assert.equal(sourceHashes[file],wanted,`unchanged fixture input ${file}`);
  }
  const method={pairs:4,window_ms:2000,hz:5,period_ms:200,captures_per_on_window:10,
    order:['off/on','on/off','off/on','on/off'],percentile:'nearest-rank',
    late_slot_policy:'skip if at least one full period late; fail schedule check; never burst to catch up',
    scope:'full-fixture DOMSnapshot at fixed demand, not production policy'};
  const run=await launch();
  try {
    await reset(run.page);const owner=await attach(run.context,run.page);
    const protocol=await owner.send('Browser.getVersion');
    const env={node:process.version,playwright:require(path.join(pwPath,'package.json')).version,
      chromium:run.browser.version(),protocol,platform:os.platform(),release:os.release(),arch:os.arch(),
      cpu:os.cpus()[0].model,logical_cpus:os.cpus().length,host_memory_bytes:os.totalmem(),viewport:config.viewport,dpr:config.dpr};
    check('supplement environment matches frozen baseline',env,frozen.environment,'runtime metadata');
    await owner.send('Performance.enable');
    const before=await checkpoint(run.page,'fixed-5hz-before');
    const windows=[];
    async function until(deadline) { while(performance.now()<deadline) await new Promise(resolve=>setTimeout(resolve,Math.max(1,deadline-performance.now()))); }
    const metric=(value,name)=>value.metrics.find(m=>m.name===name)?.value;
    for(let pair=0;pair<method.pairs;pair++) {
      for(const mode of (pair%2?['on','off']:['off','on'])) {
        await run.page.evaluate(()=>{
          const s=window.f01Raf={active:true,gaps:[],last:null};
          function tick(t){if(!s.active)return;if(s.last!==null)s.gaps.push(t-s.last);s.last=t;requestAnimationFrame(tick);}requestAnimationFrame(tick);
        });
        const metricStart=performance.now();
        const metricsBefore=await owner.send('Performance.getMetrics');
        const start=performance.now();const captures=[];
        if(mode==='on') {
          for(let slot=0;slot<method.captures_per_on_window;slot++) {
            const due=start+slot*method.period_ms;await until(due);
            const begin=performance.now(),lag=begin-due;
            if(lag>=method.period_ms){captures.push({slot,status:'skipped',lag_ms:lag});continue;}
            const raw=await full(owner);
            captures.push({slot,status:'captured',scheduled_ms:slot*method.period_ms,start_ms:begin-start,
              lag_ms:lag,roundtrip_ms:performance.now()-begin,bytes:Buffer.byteLength(JSON.stringify(raw)),
              documents:raw.documents.length,nodes:raw.documents.reduce((n,d)=>n+d.nodes.nodeName.length,0)});
          }
        }
        await until(start+method.window_ms);
        const elapsed=performance.now()-start;
        const metricsAfter=await owner.send('Performance.getMetrics');
        const metricInterval=performance.now()-metricStart;
        const gaps=await run.page.evaluate(()=>{window.f01Raf.active=false;return window.f01Raf.gaps;});
        const task0=metric(metricsBefore,'TaskDuration'),task1=metric(metricsAfter,'TaskDuration');
        const task=task0===undefined||task1===undefined?undefined:(task1-task0)*1000;
        const sample={pair,mode,elapsed_ms:elapsed,metrics_interval_wall_ms:metricInterval,captures,raf_gaps_ms:gaps,
          renderer_task_ms:task??{availability:'unsupported'},
          renderer_task_ms_per_500:task===undefined?{availability:'unsupported'}:task*500/metricInterval,
          js_heap_used_bytes:metric(metricsAfter,'JSHeapUsedSize')??{availability:'unsupported'}};
        windows.push(sample);
        check(`pair${pair}:${mode}:expected capture count`,captures.filter(c=>c.status==='captured').length,mode==='on'?10:0,'scheduler verification');
        check(`pair${pair}:${mode}:rAF samples available`,gaps.length>0,true,'sampler verification');
        if(mode==='on') for(const c of captures) assert.deepEqual([c.documents,c.nodes],[2,97],'same frozen fixture size');
      }
    }
    check('supplement source checkpoint unchanged',await checkpoint(run.page,'fixed-5hz-after'),before,'fixture source state');
    const summary=Object.fromEntries(['off','on'].map(mode=>[mode,{
      raf_gap_ms:stats(windows.filter(w=>w.mode===mode).flatMap(w=>w.raf_gaps_ms)),
      renderer_task_ms_per_500:stats(windows.filter(w=>w.mode===mode).map(w=>w.renderer_task_ms_per_500).filter(Number.isFinite)),
      capture_roundtrip_ms:stats(windows.filter(w=>w.mode===mode).flatMap(w=>w.captures.filter(c=>c.status==='captured').map(c=>c.roundtrip_ms)))
    }]));
    const pairDeltas=Array.from({length:method.pairs},(_,pair)=>{
      const off=windows.find(w=>w.pair===pair&&w.mode==='off'),on=windows.find(w=>w.pair===pair&&w.mode==='on');
      return {pair,raf_p95_delta_ms:stats(on.raf_gaps_ms).p95-stats(off.raf_gaps_ms).p95,
        task_delta_ms_per_500:Number.isFinite(on.renderer_task_ms_per_500)&&Number.isFinite(off.renderer_task_ms_per_500)?on.renderer_task_ms_per_500-off.renderer_task_ms_per_500:{availability:'unsupported'}};
    });
    return {packet:'F01',kind:'fixed-5hz-supplement',status:'pass',created_utc:new Date().toISOString(),environment:env,
      frozen_report:{path:baselinePath,sha256:hash(frozenBytes),created_utc:frozen.created_utc},
      frozen_harness:{path:priorHarnessPath,sha256:hash(priorHarness)},source_hashes:sourceHashes,method,checks,checkpoints,windows,summary,pair_deltas:pairDeltas,
      node_rss_bytes:process.memoryUsage().rss,
      telemetry:{cdp_calls:calls.length,serialized_response_bytes:calls.reduce((sum,c)=>sum+c.bytes,0),
        unavailable:['browser/OS total RSS','isolated API/transport CPU','Rust stages/cache/model tokens']},
      limitations:['4 paired windows are exploratory, not acceptance','5 Hz is a test hypothesis, not policy',
        'off retains same AX/Performance/rAF instrumentation','full fixture acquisition remains unbounded for production',
        'TaskDuration is timeTicks task duration, not CPU utilization','B01–B06 and original cold/warm/saturation were not rerun']};
  } finally {await closeBrowser(run.browser);}
}
(async()=>{
  server=await start();
  if(process.argv.includes('--fixed-overhead-only')) {
    const report=await fixedOverhead();
    fs.writeFileSync(process.env.F01_OUTPUT,JSON.stringify(report,null,2)+'\n',{flag:'wx'});
    console.log(JSON.stringify({status:report.status,kind:report.kind,checks:report.checks.length,summary:report.summary,pair_deltas:report.pair_deltas}));
    return;
  }
  await scenarios();const measurements=await baseline();
  const hashes={};for(const file of ['fixtures/web/extension.html','fixtures/web/fixture.js','fixtures/web/expected.json','fixtures/web/server.cjs','fixtures/web/run.cjs','experiments/web/fixture.html']) hashes[file]=crypto.createHash('sha256').update(fs.readFileSync(path.join(__dirname,'../..',file))).digest('hex');
  const report={packet:'F01',status:'pass',created_utc:new Date().toISOString(),environment,source_hashes:hashes,checks,checkpoints,baseline:measurements,
    scope:'Owned fixture tooling only; no product pilot acceptance; full snapshot acquisition is not bounded collector'};
  if(process.env.F01_OUTPUT)fs.writeFileSync(process.env.F01_OUTPUT,JSON.stringify(report,null,2)+'\n');
  console.log(JSON.stringify({status:'pass',checks:checks.length,environment,summary:measurements.summary}));
})().catch(error=>{console.error(`F01 failed in ${phase}: ${error.message}`);process.exitCode=1;})
.finally(async()=>{await Promise.allSettled([...browsers].map(closeBrowser));if(server)await server.close();clearTimeout(watchdog);});
