// Independent preparation check; no browser collection or proposed wire envelope.
const path = require('node:path');
const { prepare } = require('./fixture-host.cjs');
(async () => {
  const setup = prepare();
  const { start } = require(path.join(setup.root, 'fixtures/web/server.cjs'));
  const server = await start();
  try {
    const response = await fetch(`${server.url}/?generation=1`, { signal: AbortSignal.timeout(3000) });
    if (!response.ok) throw new Error('owned fixture server failed');
    const html = await response.text();
    if (!html.includes('id="left"') || !html.includes('src="/fixture.js"')) throw new Error('fixture composition missing');
    console.log(JSON.stringify({ status: 'prepared', fixture_commit: setup.fixtureCommit,
      node: process.version, playwright_core: setup.runtimeVersion,
      committed_fixture_files_checked: setup.fixtureFiles.length, localhost_setup: 'pass',
      browser_collection: 'not_run', rust_interface: 'waiting_for_stage_a_commit' }));
  } finally { await server.close(); }
})().catch(error => { console.error(`S01-Web preparation failed: ${error.message}`); process.exitCode = 1; });
