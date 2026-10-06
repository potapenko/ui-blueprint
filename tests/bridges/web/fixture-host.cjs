// S01-Web setup only. Common wire/lifecycle policy belongs to committed Stage A.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { execFileSync } = require('node:child_process');
const root = path.resolve(__dirname, '../../..');
const fixtureCommit = 'a8368076cdf0d4a917e7c3c5448ca6b6d919cf64';
const fixtureFiles = [
  'experiments/web/fixture.html', 'fixtures/web/extension.html',
  'fixtures/web/fixture.js', 'fixtures/web/server.cjs', 'fixtures/web/expected.json'
];
function prepare() {
  // Do not consume mutable shared Rust sources or silently adopt changed fixtures.
  for (const file of fixtureFiles) {
    const committed = execFileSync('git', ['show', `${fixtureCommit}:${file}`],
      { cwd: root, timeout: 5000, maxBuffer: 128 * 1024 });
    assert.ok(fs.readFileSync(path.join(root, file)).equals(committed), `F01 drift: ${file}`);
  }
  assert.equal(process.version, 'v24.15.0', 'approved fixture Node version');
  const runtime = process.env.S01_WEB_PLAYWRIGHT_CORE;
  assert.ok(runtime && path.isAbsolute(runtime), 'provide absolute S01_WEB_PLAYWRIGHT_CORE');
  const metadata = require(path.join(runtime, 'package.json'));
  assert.equal(metadata.version, '1.58.2', 'approved fixture Playwright Core version');
  const { chromium } = require(runtime);
  fs.accessSync(chromium.executablePath(), fs.constants.X_OK);
  return { root, fixtureCommit, fixtureFiles, chromium, runtimeVersion: metadata.version };
}
async function withOwnedFixture(callback) {
  const setup = prepare();
  const { start } = require(path.join(root, 'fixtures/web/server.cjs'));
  let server, browser;
  try {
    server = await start();
    browser = await setup.chromium.launch({ headless: true, timeout: 8000 });
    const context = await browser.newContext({ viewport: { width: 800, height: 600 }, deviceScaleFactor: 1 });
    await context.route('**/*', route => new URL(route.request().url()).origin === server.url ? route.continue() : route.abort());
    const page = await context.newPage();
    page.setDefaultTimeout(2000);
    page.setDefaultNavigationTimeout(3000);
    await page.goto(`${server.url}/?generation=1`);
    await page.waitForFunction(() => !!window.f01);
    assert.equal(browser.version(), '145.0.7632.6', 'approved fixture browser');
    return await callback({ context, page, url: server.url });
  } finally {
    try { if (browser) await browser.close(); }
    finally { if (server) await server.close(); }
  }
}
module.exports = { prepare, withOwnedFixture };
