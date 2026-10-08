function checkDocument(options) {
  'use strict';
  if (this !== document) return {status:'stale',count:0};
  if (options.sensitive) return {status:'private',count:0};
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
    if(node instanceof Element) {
      if(node.attributes.length>options.maxAttributes) return {status:'limit',count};
      for(const attribute of node.attributes) {
        const name=attribute.name.toLowerCase();
        if(name.length>options.maxChars || attribute.value.length>options.maxChars) return {status:'limit',count};
        if(name==='data-private' || name==='data-sensitive' || name.includes('token') || name.includes('secret')) return {status:'private',count};
      }
      const type=node.getAttribute('type'), autocomplete=node.getAttribute('autocomplete');
      if(type?.toLowerCase()==='password' || (autocomplete && /(?:^|\s)(?:current-password|new-password|one-time-code|cc-number|cc-csc)(?:\s|$)/i.test(autocomplete)))
        return {status:'private',count};
    }
    // Native child links only; frame documents are checked in their own realm.
    // Limit the pending stack before pushing, even for an extremely broad node.
    for(let child=node.lastChild;child;child=child.previousSibling) {
      if(stack.length>=options.maxNodes-count) return {status:'limit',count};
      stack.push([child,depth+1]);
    }
  }
  return {status:'current',count};
}
