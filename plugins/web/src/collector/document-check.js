function checkDocument(options) {
  'use strict';
  if (this !== document) return {status:'stale',count:0};
  if (options.sensitive) return {status:'private',count:0};
  const privateURL=value=>{
    if(value.length>options.maxChars) return true;
    try {
      const url=new URL(value,document.baseURI), query=url.search+url.hash;
      return !!(url.username||url.password) || /%|token|secret|password|authorization|api[_-]?key|signature/i.test(query);
    } catch { return true; }
  };
  const privateSrcset=value=>{
    const space=c=>/[\t\n\f\r ]/.test(c);
    let i=0;
    while(i<value.length) {
      while(i<value.length && (space(value[i]) || value[i]===',')) i++;
      const start=i;
      // URL tokens include internal commas (notably data URLs). Only trailing
      // commas end a candidate here; a descriptor's comma ends the next phase.
      while(i<value.length && !space(value[i])) i++;
      let end=i;
      while(end>start && value[end-1]===',') end--;
      if(end>start && privateURL(value.slice(start,end))) return true;
      if(end<i) continue;
      while(i<value.length && value[i]!==',') {
        // Do not guess boundaries inside unsupported descriptor groups.
        if(value[i]==='(' || value[i]===')') return true;
        i++;
      }
      if(i<value.length) i++;
    }
    return false;
  };
  if(privateURL(document.URL)||privateURL(document.baseURI)) return {status:'private',count:0};
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
        if(['href','src','action','formaction'].includes(name) && privateURL(attribute.value)) return {status:'private',count};
        if(name==='srcset' && privateSrcset(attribute.value)) return {status:'private',count};
      }
      if(typeof HTMLImageElement!=='undefined' && node instanceof HTMLImageElement) {
        const currentSrc=Object.getOwnPropertyDescriptor(HTMLImageElement.prototype,'currentSrc').get.call(node);
        if(privateURL(currentSrc)) return {status:'private',count};
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
