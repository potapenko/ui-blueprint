// R01 diagnostic only. Own fixture; no production adapter or copied upstream code.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const os = require('node:os');
const crypto = require('node:crypto');
const { performance } = require('node:perf_hooks');
const pwPath = process.env.R01_PLAYWRIGHT_CORE || 'playwright-core';
const { chromium } = require(pwPath);
const expected = JSON.parse(fs.readFileSync(path.join(__dirname, 'expected.json')));
const fixture = fs.readFileSync(path.join(__dirname, 'fixture.html'), 'utf8');
const run = crypto.randomUUID();
const results = [];
const records = [];
let revision = 0;
let browser;
let protocol;
// Bound the whole diagnostic, including launch/protocol operations and teardown.
const watchdog = setTimeout(() => {
  console.error('R01 overall deadline exceeded');
  browser?.close().finally(() => process.exit(1));
  setTimeout(() => process.exit(1), 2000).unref();
}, 25000);
function check(name, actual, wanted) {
  assert.deepEqual(actual, wanted, name);
  results.push({ name, expected: wanted, observed: actual });
}
function boundsCheck(name, actual, wanted) {
  assert.ok(actual && actual.length === 4, name);
  assert.ok(actual.every((n, i) => Math.abs(n - wanted[i]) <= expected.tolerance_css_px), name);
  results.push({ name, expected: wanted, observed: actual });
}
async function withTimeout(promise) {
  let timer;
  try {
    return await Promise.race([promise, new Promise((_, reject) => {
      timer = setTimeout(() => reject(new Error('R01 protocol deadline')), 3000);
    })]);
  } finally { clearTimeout(timer); }
}
async function connect(context, page) {
  const cdp = await context.newCDPSession(page);
  const send = (method, params = {}) => withTimeout(cdp.send(method, params));
  await send('DOM.enable');
  await send('Accessibility.enable');
  await send('Page.enable');
  const { targetInfo } = await send('Target.getTargetInfo');
  return { page, cdp, send, target: targetInfo.targetId, session: crypto.randomUUID() };
}
async function collect(owner, sourceState, dpr) {
  const start = performance.now();
  const utc = new Date().toISOString();
  const before = await owner.send('Page.getFrameTree');
  // Whole-document API is authorized only because the entire tiny fixture is owned.
  // This is NOT evidence of bounded acquisition on an arbitrary large page.
  const raw = await owner.send('DOMSnapshot.captureSnapshot', { computedStyles: [], includeDOMRects: true });
  assert.ok(raw.documents.length <= 2 && raw.documents.every(d => d.nodes.nodeName.length <= 100), 'fixture size bound');
  const frames = new Map();
  function visit(tree) { frames.set(tree.frame.id, tree.frame.loaderId); (tree.childFrames || []).forEach(visit); }
  visit(before.frameTree);
  const elements = [];
  const observation = `r01:${run}:${++revision}`;
  for (const doc of raw.documents) {
    const frame = raw.strings[doc.frameId];
    const document = doc.nodes.backendNodeId[0];
    for (let i = 0; i < doc.nodes.nodeName.length; i++) {
      const attrs = doc.nodes.attributes[i] || [];
      let id;
      for (let a = 0; a < attrs.length; a += 2) if (raw.strings[attrs[a]] === 'id') id = raw.strings[attrs[a + 1]];
      if (!['left', 'right', 'hidden', 'child-button'].includes(id)) continue;
      const backend = doc.nodes.backendNodeId[i];
      const ref = { session: owner.session, target: owner.target, frame, loader: frames.get(frame), document, backend, observation };
      const ax = await owner.send('Accessibility.getPartialAXTree', { backendNodeId: backend, fetchRelatives: false });
      const node = ax.nodes.find(n => n.backendDOMNodeId === backend);
      const layoutIndex = doc.layout.nodeIndex.indexOf(i);
      const object = await owner.send('DOM.resolveNode', { backendNodeId: backend, objectGroup: 'r01' });
      const rect = await owner.send('Runtime.callFunctionOn', {
        objectId: object.object.objectId, returnByValue: true,
        functionDeclaration: 'function(){ const r=this.getBoundingClientRect(); return [r.x,r.y,r.width,r.height]; }'
      });
      elements.push({ id, ref,
        ax: { source: 'cdp.ax', provenance: 'reported', availability: node?.name ? 'known' : 'unknown',
          ...(node?.name ? { name: node.name.value, role: node.role?.value, source_id: node.nodeId } : {}) },
        dom: { source: 'cdp.dom', source_id: backend },
        layout_bounds: { source: 'cdp.domsnapshot', provenance: 'reported', units: 'css_px', coordinate_space: `document:${frame}`,
          availability: layoutIndex < 0 ? 'unknown' : 'known', ...(layoutIndex < 0 ? {} : { value: doc.layout.bounds[layoutIndex] }) },
        cssom_rect: { source: 'cssom.getBoundingClientRect', provenance: 'reported', units: 'css_px', coordinate_space: `viewport:${frame}`, value: rect.result.value },
        accessibility_bounds: { availability: 'unsupported', reason: 'selected CDP AX API does not expose bounds' },
        hit_region: { availability: 'unknown', reason: 'no hit-region measurement requested from backend' },
        visible_region: { availability: 'unknown', reason: 'bounding rect does not establish clipping/occlusion' }
      });
    }
  }
  await owner.send('Runtime.releaseObjectGroup', { objectGroup: 'r01' });
  const after = await owner.send('Page.getFrameTree');
  assert.deepEqual(after, before, 'frame tree unchanged across controlled capture');
  const record = { observation, source_state: sourceState, environment: { kind: 'owned_headless_fixture', dpr },
    target: owner.target, started_utc: utc, start_ms: start, end_ms: performance.now(), clock_domain: 'node.performance',
    response_source: 'live', freshness: 'current_at_observation', consistency: 'unknown',
    consistency_method: 'frame tree before/after equal; no cross-channel atomicity claim',
    coverage: { status: 'partial', within: 'four named fixture controls', reason: 'geometry channels unavailable; only same-process frame exercised' }, elements };
  records.push(record);
  return record;
}
async function validate(owner, ref) {
  if (ref.target !== owner.target || ref.session !== owner.session) return 'stale_target';
  const { frameTree } = await owner.send('Page.getFrameTree');
  const queue = [frameTree]; let frame;
  while (queue.length) { const t = queue.shift(); if (t.frame.id === ref.frame) frame = t.frame; queue.push(...(t.childFrames || [])); }
  if (!ref.loader || frame?.loaderId !== ref.loader) return 'stale_target';
  try {
    const object = await owner.send('DOM.resolveNode', { backendNodeId: ref.backend, objectGroup: 'r01-validate' });
    const response = await owner.send('Runtime.callFunctionOn', {
      objectId: object.object.objectId, returnByValue: true, functionDeclaration: 'function(){return this.isConnected;}'
    });
    if (response.result.value !== true) return 'stale_target';
    const docObject = await owner.send('Runtime.callFunctionOn', {
      objectId: object.object.objectId, functionDeclaration: 'function(){return this.ownerDocument;}', objectGroup: 'r01-validate'
    });
    const document = await owner.send('DOM.describeNode', { objectId: docObject.result.objectId });
    return document.node.backendNodeId === ref.document ? 'current' : 'stale_target';
  } catch (error) {
    if (/No node with given|Could not find node|Cannot find context|Cannot find object/.test(error.message)) return 'stale_target';
    throw error; // A protocol timeout/error is not positive evidence of staleness.
  }
  finally { await owner.send('Runtime.releaseObjectGroup', { objectGroup: 'r01-validate' }); }
}
(async () => {
  browser = await chromium.launch({ headless: true, timeout: 8000 });
  const version = browser.version();
  for (const dpr of [1, 2]) {
    const context = await browser.newContext({ viewport: { width: 800, height: 600 }, deviceScaleFactor: dpr });
    // No real origin, network, cookies or pre-existing profile.
    await context.route('**/*', route => route.abort());
    const page = await context.newPage();
    page.setDefaultTimeout(1500);
    await page.setContent(fixture);
    const owner = await connect(context, page);
    protocol ||= await owner.send('Browser.getVersion');
    const first = await collect(owner, 'initial', dpr);
    for (const [id, wanted] of Object.entries(expected.initial)) {
      const element = first.elements.find(e => e.id === id);
      check(`dpr${dpr}:${id}:AX name`, element.ax.name, wanted.name);
      boundsCheck(`dpr${dpr}:${id}:DOMSnapshot`, element.layout_bounds.value, wanted.bounds);
      boundsCheck(`dpr${dpr}:${id}:CSSOM`, element.cssom_rect.value, wanted.bounds);
    }
    check(`dpr${dpr}:hidden layout`, first.elements.find(e => e.id === 'hidden').layout_bounds.availability, expected.hidden_layout);
    check(`dpr${dpr}:duplicate names`, first.elements.filter(e => ['left', 'right'].includes(e.id) && e.ax.name === 'Apply').length, expected.main_apply_count);
    const metrics = await owner.send('Page.getLayoutMetrics');
    const viewportRatio = metrics.visualViewport.clientWidth / metrics.cssVisualViewport.clientWidth;
    records.push({ source_state: 'initial', dpr, viewport_ratio: viewportRatio, window_dpr: await page.evaluate(() => devicePixelRatio) });
    const left = first.elements.find(e => e.id === 'left');
    const child = first.elements.find(e => e.id === 'child-button');
    check(`dpr${dpr}:fresh ref`, await validate(owner, left.ref), 'current');
    check(`dpr${dpr}:wrong document rejected`, await validate(owner, { ...left.ref, document: -1 }), 'stale_target');
    const ambiguous = first.elements.filter(e => e.ref.frame === left.ref.frame && e.ax.name === 'Apply');
    check(`dpr${dpr}:name-only resolution`, ambiguous.length === 1 ? 'unique' : 'ambiguous_target', 'ambiguous_target');
    // An exact ElementHandle, not a locator that can silently re-resolve after remount.
    const handle = await page.$('#left');
    await handle.click();
    check(`dpr${dpr}:delivery observed`, Number(await page.locator('#outcome').getAttribute('data-delivered')), expected.delivery_count);
    await page.waitForFunction(() => document.querySelector('#outcome').dataset.applied === '1');
    check(`dpr${dpr}:result verified separately`, Number(await page.locator('#outcome').getAttribute('data-applied')), expected.applied_count);
    await page.evaluate(() => { const old = document.querySelector('#left'); old.replaceWith(old.cloneNode(true)); });
    check(`dpr${dpr}:old remounted ref`, await validate(owner, left.ref), expected.remount);
    let detachedRejected = false;
    try { await handle.click({ timeout: 250 }); } catch (error) { detachedRejected = /not attached/.test(error.message); }
    check(`dpr${dpr}:old exact handle rejected`, detachedRejected, true);
    check(`dpr${dpr}:no remount click delivered`, Number(await page.locator('#outcome').getAttribute('data-delivered')), 1);
    const remount = await collect(owner, 'left-remounted', dpr);
    check(`dpr${dpr}:new backend identity`, remount.elements.find(e => e.id === 'left').ref.backend !== left.ref.backend, true);
    check(`dpr${dpr}:unrelated node unchanged`, remount.elements.find(e => e.id === 'right').ref.backend, first.elements.find(e => e.id === 'right').ref.backend);
    const childFrame = page.frames().find(f => f.parentFrame());
    await Promise.all([childFrame.waitForNavigation({ timeout: 1500 }), page.evaluate(() => { document.querySelector('iframe').srcdoc += '<!-- new document -->'; })]);
    check(`dpr${dpr}:child old ref`, await validate(owner, child.ref), expected.child_navigation);
    const navigated = await collect(owner, 'child-document-replaced', dpr);
    check(`dpr${dpr}:child generation changed`, navigated.elements.find(e => e.id === 'child-button').ref.loader !== child.ref.loader, true);
    const other = await context.newPage();
    await other.setContent(fixture);
    const otherOwner = await connect(context, other);
    check(`dpr${dpr}:cross target rejection`, await validate(otherOwner, left.ref), expected.cross_target);
    await context.close();
  }
  const report = { packet: 'R01', status: 'pass', run, environment: { node: process.version, platform: os.platform(), release: os.release(), arch: os.arch(), playwright: require(path.join(pwPath, 'package.json')).version, chromium: version, protocol },
    probe_sha256: crypto.createHash('sha256').update(fs.readFileSync(__filename)).digest('hex'),
    oracle_sha256: crypto.createHash('sha256').update(fs.readFileSync(path.join(__dirname, 'expected.json'))).digest('hex'),
    fixture_sha256: crypto.createHash('sha256').update(fixture).digest('hex'), checks: results, observations: records,
    limitations: ['No production adapter', 'No independent reviewer acceptance', 'No OOPIF, Safari, Firefox, real desktop or full B01–B06 proof', 'Whole-fixture DOMSnapshot acquisition; selected output only', 'No secret/pixel/cache or cancellation canary'] };
  if (process.env.R01_OUTPUT) fs.writeFileSync(process.env.R01_OUTPUT, JSON.stringify(report, null, 2) + '\n');
  console.log(JSON.stringify({ status: report.status, checks: results.length, environment: report.environment, viewport_ratios: records.filter(r => 'viewport_ratio' in r) }));
})().catch(error => { console.error(`R01 failed: ${error.message}`); process.exitCode = 1; })
  .finally(async () => { if (browser) await browser.close(); clearTimeout(watchdog); });
