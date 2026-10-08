// W03-R fixture fault only: forward one owned numeric-loopback TCP connection.
// No CDP messages are fabricated, decoded, cached or retried. Destroying the pair
// loses the real browser transport; production recovery still requires reattach.
const assert = require('node:assert/strict');
const net = require('node:net');
async function lossTransport(endpoint) {
  const target = new URL(endpoint);
  assert.equal(target.protocol, 'ws:');
  assert.equal(target.hostname, '127.0.0.1');
  let pair, admitted = false, bytes = 0;
  const server = net.createServer(client => {
    if (admitted) { client.destroy(); return; }
    admitted = true;
    const upstream = net.connect(Number(target.port), '127.0.0.1');
    pair = [client, upstream];
    for (const socket of pair) {
      socket.setTimeout(10000, () => socket.destroy());
      socket.on('error', () => { for (const own of pair) own.destroy(); });
      socket.on('data', chunk => {
        bytes += chunk.length;
        if (bytes > 2 * 1024 * 1024) for (const own of pair) own.destroy();
      });
    }
    client.pipe(upstream); upstream.pipe(client);
  });
  await new Promise((resolve, reject) => {
    server.once('error', reject); server.listen(0, '127.0.0.1', resolve);
  });
  const address = new URL(endpoint); address.port = String(server.address().port);
  async function cut() {
    assert(pair && admitted, 'one actual worker connection');
    await Promise.all(pair.map(socket => new Promise(resolve => {
      if (socket.closed) return resolve();
      socket.once('close', resolve); socket.destroy();
    })));
  }
  return {endpoint:address.href, cut, bytes:()=>bytes,
    close:async()=>{ if(pair) await cut(); await new Promise(resolve=>server.close(resolve)); }};
}
module.exports = {lossTransport};
