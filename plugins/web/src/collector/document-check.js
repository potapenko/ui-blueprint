function checkDocument(options) {
  'use strict';
  if (this !== document) return {status:'stale',count:0};
  const end=performance.now()+options.remainingMs;
  const stack=[[this,0]];
  let count=0;
  while(stack.length) {
    if(performance.now()>=end) return {status:'timeout',count};
    const [node,depth]=stack.pop();
    if(depth>options.maxDepth || count>=options.maxNodes) return {status:'limit',count};
    count++;
    if(node instanceof Element && (node.shadowRoot || node instanceof HTMLTemplateElement))
      return {status:'unsupported',count};
    // Native child links only; frame documents are checked in their own realm.
    // Limit the pending stack before pushing, even for an extremely broad node.
    for(let child=node.lastChild;child;child=child.previousSibling) {
      if(stack.length>=options.maxNodes-count) return {status:'limit',count};
      stack.push([child,depth+1]);
    }
  }
  return {status:'current',count};
}
