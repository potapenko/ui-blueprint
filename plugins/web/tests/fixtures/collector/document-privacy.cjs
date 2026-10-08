// Q01's inert-DOM preflight counterexample with repaired expectations.
const fs=require('node:fs'),assert=require('node:assert/strict'),vm=require('node:vm'),path=require('node:path');
class Element {
  constructor(attrs){this.attributes=Object.entries(attrs).map(([name,value])=>({name,value}));this.lastChild=null;this.previousSibling=null;this.shadowRoot=null;}
  getAttribute(name){return this.attributes.find(a=>a.name===name)?.value??null;}
}
const source=fs.readFileSync(path.join(__dirname,'../../../src/collector/document-check.js'),'utf8');
const cases=require('./srcset-cases.json');
for(const {name,value:srcset,private:blocked}of cases){
  const document={URL:'http://127.0.0.1/fixture',baseURI:'http://127.0.0.1/fixture',lastChild:new Element({src:'/public.png',srcset})};
  const check=vm.runInNewContext('('+source+')',{document,Element,HTMLTemplateElement:class extends Element{},URL,performance});
  assert.equal(check.call(document,{maxChars:3000,maxAttributes:32,maxNodes:128,maxDepth:8,remainingMs:1000,sensitive:false}).status,blocked?'private':'current',name);
}
console.log(`${cases.length} srcset candidate-boundary preflight cases passed`);
