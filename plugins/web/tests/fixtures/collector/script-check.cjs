// Offline mock getters only. This is NOT Chromium/CDP/DOM runtime qualification.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const vm = require('node:vm');
const source = fs.readFileSync(require('node:path').join(__dirname, '../../../src/collector/read-node.js'), 'utf8');
let valueReads = 0, rectReads = 0;
class Element {
  getAttribute(key) { return this.attrs[key] ?? null; }
  getClientRects() { return this.box ? [this.box] : []; }
  getBoundingClientRect() { rectReads++; return this.box; }
  matches(key) { assert.equal(key, ':disabled'); return this.disabled; }
  focus() { throw Error('forbidden mutation'); }
  scrollIntoView() { throw Error('forbidden mutation'); }
}
class HTMLInputElement extends Element {}
class HTMLTextAreaElement extends Element {}
class HTMLSelectElement extends Element {}
class HTMLOptionElement extends Element {}
class HTMLButtonElement extends Element {}
for (const [key, sourceKey] of Object.entries({type:'kind',value:'data',placeholder:'placeholder',required:'required',readOnly:'readOnly',checked:'checked',indeterminate:'indeterminate',validity:'validity'})) {
  Object.defineProperty(HTMLInputElement.prototype, key, { get() { if (key === 'value') valueReads++; return this.state[sourceKey]; } });
}
const document = { activeElement: null };
const context = vm.createContext({Element,HTMLInputElement,HTMLTextAreaElement,HTMLSelectElement,HTMLOptionElement,HTMLButtonElement,document});
vm.runInContext(source, context, {timeout:100});
function node() { const n = new HTMLInputElement(); Object.assign(n,{attrs:{},box:{x:40,y:60,width:120,height:40},disabled:false,isConnected:true,ownerDocument:document,tagName:'INPUT',state:{kind:'checkbox',data:'',checked:false,indeterminate:false,required:false,readOnly:false,validity:{valid:true}}});return n; }
function read(n,fields,sensitive=false,doc=document,maxChars=100) { context.node=n;context.options={fields,sensitive,maxChars};context.expected=doc;return vm.runInContext('readNode.call(node, options, expected)',context,{timeout:100}); }
const ordinary=node(), before=JSON.stringify(ordinary);
const result=read(ordinary,['layout_bounds','checked','value','enabled','focused','invalid']);
assert.equal(JSON.stringify(ordinary),before);assert.deepEqual(JSON.parse(JSON.stringify(result.rect)),{x:40,y:60,width:120,height:40});assert.equal(result.checked,false);assert.equal(result.value,'');assert.equal(result.invalid,false);assert.equal(result.enabled,true);assert.equal(result.focused,false);
const secret=node();secret.state.kind='password';secret.state.data='PRIVATE_SCRIPT_CANARY';valueReads=0;
assert.equal(read(secret,['value']).sensitive,true);assert.equal(valueReads,0);assert(!JSON.stringify(read(secret,['value','placeholder'])).includes('PRIVATE_SCRIPT_CANARY'));
const marked=node();marked.state.data='PRIVATE_SCRIPT_CANARY';valueReads=0;read(marked,['value'],true);assert.equal(valueReads,0);
const otp=node();otp.attrs.autocomplete='section-test one-time-code';valueReads=0;assert.equal(read(otp,['value']).sensitive,true);assert.equal(valueReads,0);
const geometry=node();valueReads=0;read(geometry,['layout_bounds']);assert.equal(valueReads,0);
const hidden=node();hidden.box=null;assert.equal(read(hidden,['layout_bounds']).rect,undefined);
const foreign=node();rectReads=0;assert.equal(read(foreign,['layout_bounds'],false,{}).sameDocument,false);assert.equal(rectReads,0);
const detached=node();detached.isConnected=false;rectReads=0;assert.equal(read(detached,['layout_bounds']).connected,false);assert.equal(rectReads,0);
const long=node();long.state.data='abcd';assert.equal(read(long,['value'],false,document,3).value,null);
const mixed=node();mixed.state.indeterminate=true;assert.equal(read(mixed,['checked']).checked,undefined);
console.log('11 offline script scenarios passed; mock getters only, no browser qualification.');
const verifySource = fs.readFileSync(require('node:path').join(__dirname, '../../../src/collector/verify-nodes.js'), 'utf8');
vm.runInContext(verifySource, context, {timeout:100});
function verify(nodes, expected=document) { context.selected=nodes;context.expected=expected;return vm.runInContext('verifyNodes(expected, ...selected)',context,{timeout:100}); }
const earlier=node(), later=node();
read(earlier,['value','layout_bounds']);read(later,['value','layout_bounds']);
const beforeVerify=[valueReads,rectReads];assert.equal(verify([earlier,later]).current,true);assert.deepEqual([valueReads,rectReads],beforeVerify);
earlier.isConnected=false;const replacement=node();replacement.attrs.id='same-label-or-id';assert.equal(verify([earlier,later]).current,false);assert.equal(verify([replacement,later]).current,true);
assert.equal(verify([later],{}).current,false);const foreignNode=node();foreignNode.ownerDocument={};assert.equal(verify([foreignNode]).current,false);
console.log('3 offline continuity scenarios passed; original handles, no field recollection or browser qualification.');
const selectSource = fs.readFileSync(require('node:path').join(__dirname, '../../../src/collector/select-ids.js'), 'utf8');
context.performance={now:()=>0};vm.runInContext(selectSource,context,{timeout:100});
function children(parent,nodes){parent.firstChild=nodes[0]??null;nodes.forEach((n,i)=>{n.parentNode=parent;n.nextSibling=nodes[i+1]??null;n.firstChild=n.firstChild??null;});}
function identified(id){const n=node();n.attrs.id=id;n.localName='input';n.shadowRoot=null;return n;}
function search(ids,maxVisited=20,maxDepth=8,remainingMs=1000,root=document){context.selectionOptions={ids,maxVisited,maxDepth,remainingMs};context.searchRoot=root;return vm.runInContext('selectIds.call(searchRoot, selectionOptions)',context,{timeout:100});}
const left=identified('left'),right=identified('right'),textNode={firstChild:null};children(document,[left,textNode,right]);
const selected=search(['right','left']);assert.equal(selected.status,'selected');assert.equal(selected.visited,4);assert.equal(selected.node_0,right);assert.equal(selected.node_1,left);assert.equal(Object.getPrototypeOf(selected),null);
const duplicate=identified('left');children(document,[left,right,duplicate]);assert.equal(search(['left']).status,'ambiguous');
assert.equal(search(['left'],2).status,'incomplete','a found candidate is not unique when traversal truncates');
children(document,[left,right]);assert.equal(search(['missing']).status,'missing');assert.equal(search(['left'],3).status,'selected','exact cap can finish the full bounded search');
const nested=identified('deep');children(left,[nested]);assert.equal(search(['left'],20,1).status,'incomplete','depth cap cannot certify a convenient earlier match');children(left,[]);
for(const boundary of ['iframe','frame','template','slot']){left.localName=boundary;assert.equal(search(['left']).status,'unsupported');}left.localName='input';left.shadowRoot={};assert.equal(search(['left']).status,'unsupported');left.shadowRoot=null;
const hostile=identified("x'); throw new Error('PRIVATE_SELECTOR_CANARY'); //");children(document,[hostile]);assert.equal(search([hostile.attrs.id]).node_0,hostile,'identifier text is only data');
assert.equal(search(['left'],20,8,1000,{}).status,'stale');
children(document,[left,right]);let tick=0;context.performance={now:()=>tick++};assert.equal(search(['left'],20,8,2).status,'timeout');context.performance={now:()=>0};
valueReads=0;rectReads=0;assert.equal(search(['left']).status,'selected');assert.deepEqual([valueReads,rectReads],[0,0],'selection never reads values or layout');
console.log('10 offline bootstrap scenarios passed; bounded light-DOM selection only, no browser qualification.');
