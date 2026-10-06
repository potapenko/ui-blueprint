const http = require('node:http');
const fs = require('node:fs');
const path = require('node:path');
// Reuse frozen R01 foundation verbatim; no historical result/path is changed.
const root = path.resolve(__dirname, '../..');
const html = fs.readFileSync(path.join(root, 'experiments/web/fixture.html'), 'utf8') + '\n' + fs.readFileSync(path.join(__dirname, 'extension.html'), 'utf8');
async function start() {
  const server = http.createServer((req, res) => {
    const url = new URL(req.url, 'http://127.0.0.1');
    res.setHeader('Cache-Control', 'no-store');
    if (url.pathname === '/') { res.setHeader('Content-Type', 'text/html; charset=utf-8'); res.end(html); }
    else if (url.pathname === '/fixture.js') { res.setHeader('Content-Type', 'text/javascript; charset=utf-8'); res.end(fs.readFileSync(path.join(__dirname, 'fixture.js'))); }
    else { res.statusCode = 404; res.end('Not found'); }
  });
  server.requestTimeout = 3000; server.headersTimeout = 3000;
  await new Promise((resolve, reject) => { server.once('error', reject); server.listen(0, '127.0.0.1', resolve); });
  return { url: `http://127.0.0.1:${server.address().port}`, close: () => new Promise(resolve => server.close(resolve)) };
}
module.exports = { start };
if (require.main === module) start().then(server => {
  console.log(server.url);
  const stop = async () => { await server.close(); process.exit(); };
  process.once('SIGINT', stop); process.once('SIGTERM', stop);
  setTimeout(stop, 30 * 60 * 1000).unref(); // Bound manual serving; restart explicitly.
});
