function selectIds(options) {
  'use strict';
  // Exact IDs are data. No selectors/eval, frame/shadow traversal or app mutation.
  const result = Object.create(null);
  let visited = 0;
  const finish = status => { result.status = status; result.visited = visited; return result; };
  const rooted = options.rooted === true;
  if (rooted ? !(this instanceof Element) || !this.isConnected || this.ownerDocument !== document : this !== document) return finish('stale');
  const originalParent = rooted ? this.parentNode : null;
  const end = performance.now() + options.remainingMs;
  const ids = new Map();
  const found = rooted ? [] : new Array(options.ids.length).fill(null);
  let maxIdChars = 0;
  for (let index = 0; index < options.ids.length; index++) {
    if (performance.now() >= end) return finish('timeout');
    const id = options.ids[index]; ids.set(id, index); maxIdChars = Math.max(maxIdChars, id.length);
  }
  let node = this, depth = 0;
  while (node) {
    if (visited >= options.maxVisited) return finish('incomplete');
    if (performance.now() >= end) return finish('timeout');
    visited++;
    if (node instanceof Element) {
      const id = rooted ? null : Element.prototype.getAttribute.call(node, 'id');
      const index = rooted ? found.length : typeof id === 'string' && id.length <= maxIdChars ? ids.get(id) : undefined;
      if (index !== undefined) {
        if (rooted && found.length >= options.maxSelected) return finish('incomplete');
        if (!rooted && found[index] !== null) return finish('ambiguous');
        // Do not turn a selected frame/shadow/template boundary into a wider read.
        if (node.shadowRoot || ['iframe', 'frame', 'template', 'slot'].includes(node.localName)) return finish('unsupported');
        found[index] = node;
      }
    }
    const child = node.firstChild;
    if (child) {
      if (depth >= options.maxDepth) return finish('incomplete');
      node = child; depth++; continue;
    }
    while (node !== this && !node.nextSibling) {
      if (performance.now() >= end) return finish('timeout');
      node = node.parentNode; depth--;
    }
    if (node === this) break;
    node = node.nextSibling;
  }
  if (performance.now() >= end) return finish('timeout');
  if (found.some(node => node === null)) return finish('missing');
  for (let index = 0; index < found.length; index++) result['node_' + index] = found[index];
  if (rooted) result.parent = originalParent;
  return finish('selected');
}
