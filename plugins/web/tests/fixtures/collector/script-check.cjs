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
