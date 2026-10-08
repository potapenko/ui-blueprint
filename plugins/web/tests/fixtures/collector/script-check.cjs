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
class HTMLOutputElement extends Element {}
for (const [key, sourceKey] of Object.entries({type:'kind',value:'data',placeholder:'placeholder',required:'required',readOnly:'readOnly',checked:'checked',indeterminate:'indeterminate',validity:'validity'})) {
  Object.defineProperty(HTMLInputElement.prototype, key, { configurable:true, get() { if (key === 'value') valueReads++; return this.state[sourceKey]; } });
}
class Document {
  getElementById(id) { return this.idNodes.find(n=>n.attrs.id===id)??null; }
  hasFocus() { return this.hasFocusFlag === true; }
}
const document = { activeElement: null };
const context = vm.createContext({Element,HTMLInputElement,HTMLTextAreaElement,HTMLSelectElement,HTMLOptionElement,HTMLButtonElement,HTMLOutputElement,Document,document});
vm.runInContext(source, context, {timeout:100});
function node() { const n = new HTMLInputElement(); Object.assign(n,{attrs:{},box:{x:40,y:60,width:120,height:40},disabled:false,isConnected:true,ownerDocument:document,tagName:'INPUT',state:{kind:'checkbox',data:'',checked:false,indeterminate:false,required:false,readOnly:false,validity:{valid:true}}});return n; }
function read(n,fields,sensitive=false,doc=document,maxChars=100) { context.node=n;context.options={fields,sensitive,maxChars};context.expected=doc;return vm.runInContext('readNode.call(node, options, expected)',context,{timeout:100}); }
const ordinary=node(), before=JSON.stringify(ordinary);
const result=read(ordinary,['layout_bounds','checked','value','enabled','focused','invalid']);
assert.equal(JSON.stringify(ordinary),before);assert.deepEqual(JSON.parse(JSON.stringify(result.rect)),{x:40,y:60,width:120,height:40,viewport:{before:null,after:null}});assert.equal(result.checked,false);assert.equal(result.value,'');assert.equal(result.invalid,false);assert.equal(result.enabled,true);assert.equal(result.focused,false);
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

context.Document=Document;
function relationRead(n, selected, sensitive=false) {
  context.node=n;context.selected=selected;context.options={fields:['focused','layout_bounds'],maxChars:100,sensitive};context.expected=document;
  return vm.runInContext('readNode.call(node, options, expected, ...selected)',context,{timeout:100});
}
const trigger=identified('trigger'), popup=identified('popup'), external=identified('external');
trigger.attrs['aria-controls']='popup external';popup.attrs['data-anchor']='trigger';
trigger.attrs['aria-activedescendant']='popup';document.activeElement=trigger;document.idNodes=[trigger,popup,external];
const state=JSON.stringify([trigger.attrs,popup.attrs,trigger.state,popup.state]);
let linked=relationRead(trigger,[trigger,popup]);assert.deepEqual(Array.from(linked.controls),[1]);assert.equal(linked.activeDescendant,1);
assert.equal(relationRead(popup,[trigger,popup]).declaredAnchor,0);
assert.equal(JSON.stringify([trigger.attrs,popup.attrs,trigger.state,popup.state]),state);
const duplicatePopup=identified('popup');assert.deepEqual(Array.from(relationRead(trigger,[trigger,popup,duplicatePopup]).controls),[]);
document.idNodes=[duplicatePopup,trigger,popup];assert.deepEqual(Array.from(relationRead(trigger,[trigger,popup]).controls),[],'unselected browser endpoint cannot be replaced by same ID');
document.idNodes=[trigger,popup,external];trigger.attrs['aria-controls']='missing';assert.deepEqual(Array.from(relationRead(trigger,[trigger,popup]).controls),[]);
trigger.attrs['aria-controls']='x'.repeat(101);assert.equal(relationRead(trigger,[trigger,popup]).controls,undefined);
const redacted=relationRead(trigger,[trigger,popup],true);assert.equal(redacted.controls,undefined);assert.equal(redacted.declaredAnchor,undefined);assert.equal(redacted.activeDescendant,undefined);
document.activeElement=popup;assert.equal(relationRead(trigger,[trigger,popup]).activeDescendant,undefined);
console.log('8 offline relation scenarios passed; actual browser proof remains separate.');

function rootedSearch(root,maxVisited=256,maxDepth=8,maxSelected=16) {
  context.searchRoot=root;context.selectionOptions={ids:[],rooted:true,maxVisited,maxDepth,maxSelected,remainingMs:1000};
  return vm.runInContext('selectIds.call(searchRoot, selectionOptions)',context,{timeout:100});
}
const rootNode=identified('wrapper'), rootChild=identified('duplicate-id'), rootSibling=identified('duplicate-id');
children(rootNode,[rootChild]);children(document,[rootNode,rootSibling]);
const rootResult=rootedSearch(rootNode);assert.equal(rootResult.status,'selected');assert.equal(rootResult.visited,2);assert.equal(rootResult.node_0,rootNode);assert.equal(rootResult.node_1,rootChild);assert.equal(rootResult.node_2,undefined);assert.equal(rootResult.parent,document);
assert.equal(rootedSearch(rootNode,1).status,'incomplete');assert.equal(rootedSearch(rootNode,256,0).status,'incomplete');assert.equal(rootedSearch(rootNode,256,8,1).status,'incomplete');
rootChild.localName='iframe';assert.equal(rootedSearch(rootNode).status,'unsupported');rootChild.localName='input';rootChild.shadowRoot={};assert.equal(rootedSearch(rootNode).status,'unsupported');rootChild.shadowRoot=null;
rootNode.isConnected=false;assert.equal(rootedSearch(rootNode).status,'stale');rootNode.isConnected=true;
const bootstrapSource=fs.readFileSync(require('node:path').join(__dirname,'../../../src/collector/bootstrap.rs'),'utf8');
const verifier=bootstrapSource.match(/const VERIFY: &str = r#"([\s\S]*?)"#;/)[1];context.rootedVerifier=vm.runInContext('('+verifier+')',context);context.rootResult=rootResult;
const verifyRoot=()=>vm.runInContext('rootedVerifier.call(rootResult, document, 2, 8)',context,{timeout:100}).current;
assert.equal(verifyRoot(),true);rootChild.parentNode=rootSibling;assert.equal(verifyRoot(),false);rootChild.parentNode=rootNode;
rootNode.parentNode=rootSibling;assert.equal(verifyRoot(),false);rootNode.parentNode=document;
rootChild.isConnected=false;assert.equal(verifyRoot(),false);rootChild.isConnected=true;
const replacementRoot=identified('wrapper');children(replacementRoot,[identified('duplicate-id')]);rootNode.isConnected=false;assert.equal(verifyRoot(),false,'no re-query of same-ID replacement');rootNode.isConnected=true;
console.log('11 offline rooted scenarios passed; subtree identity/limits/continuity, no browser qualification.');

const setterSource=fs.readFileSync(require('node:path').join(__dirname,'../../../src/collector/set-checked.js'),'utf8');
vm.runInContext(setterSource,context,{timeout:100});let setterCalls=0;
Object.defineProperty(HTMLInputElement.prototype,'checked',{configurable:true,get(){return this.state.checked;},set(value){setterCalls++;this.state.checked=value;}});
const setterTarget=node(),setterOther=node();setterOther.state.checked=false;
setterTarget.click=()=>{throw Error('pointer fallback forbidden');};setterTarget.dispatchEvent=()=>{throw Error('event synthesis forbidden');};
function setCheckedFixture(target,value,doc=document){context.setterTarget=target;context.setterValue=value;context.setterDoc=doc;return vm.runInContext('setChecked.call(setterTarget,setterDoc,setterValue)',context,{timeout:100});}
assert.equal(setCheckedFixture(setterTarget,true).status,'applied');assert.equal(setterTarget.state.checked,true);assert.equal(setterOther.state.checked,false);
assert.equal(setCheckedFixture(setterTarget,true).status,'applied');assert.equal(setterCalls,2,'already-equal invokes one setter rather than toggle');
assert.equal(setCheckedFixture(setterTarget,false).status,'applied');assert.equal(setterTarget.state.checked,false);
setterTarget.disabled=true;assert.equal(setCheckedFixture(setterTarget,true).status,'disabled');setterTarget.disabled=false;
setterTarget.state.indeterminate=true;assert.equal(setCheckedFixture(setterTarget,true).status,'indeterminate');setterTarget.state.indeterminate=false;
setterTarget.isConnected=false;assert.equal(setCheckedFixture(setterTarget,true).status,'stale');setterTarget.isConnected=true;
assert.equal(setCheckedFixture(setterTarget,true,{}).status,'stale');setterTarget.state.kind='radio';assert.equal(setCheckedFixture(setterTarget,true).status,'unsupported');setterTarget.state.kind='checkbox';
setterTarget.attrs.autocomplete='one-time-code';assert.equal(setCheckedFixture(setterTarget,true).status,'unsupported');delete setterTarget.attrs.autocomplete;
assert.equal(setterCalls,3,'negative cases never call native setter');
console.log('8 offline native-setter scenarios passed; mocks only, no real input/parent permit.');

const actionSource=fs.readFileSync(require('node:path').join(__dirname,'../../../src/collector/action.rs'),'utf8');
const checkboxStateSource=actionSource.match(/const STATE: &str = r#"([\s\S]*?)"#;/)[1];vm.runInContext(checkboxStateSource,context,{timeout:100});
function checkboxStateFixture(target,doc=document){context.stateTarget=target;context.stateDoc=doc;return vm.runInContext('checkboxState.call(stateTarget,stateDoc)',context,{timeout:100});}
const stateTarget=node();stateTarget.state.data='PRIVATE_STATE_CANARY';valueReads=0;
let currentState=checkboxStateFixture(stateTarget);assert.equal(currentState.nativeCheckbox,true);assert.equal(currentState.enabled,true);assert.equal(currentState.checked,false);assert.equal(currentState.indeterminate,false);assert.equal(currentState.writable,true);assert.equal(valueReads,0);assert(!JSON.stringify(currentState).includes('PRIVATE_STATE_CANARY'));
stateTarget.disabled=true;assert.equal(checkboxStateFixture(stateTarget).enabled,false);stateTarget.disabled=false;
stateTarget.state.indeterminate=true;assert.equal(checkboxStateFixture(stateTarget).indeterminate,true);stateTarget.state.indeterminate=false;
stateTarget.attrs.autocomplete='one-time-code';assert.equal(checkboxStateFixture(stateTarget).sensitive,true);delete stateTarget.attrs.autocomplete;
assert.equal(checkboxStateFixture(stateTarget,{}).connected,false);
stateTarget.state.kind='radio';assert.equal(checkboxStateFixture(stateTarget).nativeCheckbox,false);
console.log('6 offline checkbox-capability scenarios passed; no saved-state freshness or mutation authority inferred.');

let selectionReads=0;
for (const prototype of [HTMLInputElement.prototype,HTMLTextAreaElement.prototype]) {
  for (const key of ['selectionStart','selectionEnd','selectionDirection']) {
    Object.defineProperty(prototype,key,{get(){selectionReads++;if(this.failSelection)throw Error('PRIVATE_GETTER_ERROR');return this.state[key];}});
  }
}
Object.defineProperty(HTMLTextAreaElement.prototype,'value',{get(){valueReads++;return this.state.data;}});
const textInput=node();Object.assign(textInput.state,{kind:'text',data:'A💡B',selectionStart:1,selectionEnd:3,selectionDirection:'forward'});
document.activeElement=textInput;document.hasFocusFlag=true;
const formFields=['focused','value','invalid'];
const readSelection=()=>read(textInput,formFields).selection;
assert.deepEqual(JSON.parse(JSON.stringify(readSelection())),{start:1,end:3,direction:'forward',documentFocused:true});
textInput.state.selectionDirection='backward';assert.equal(readSelection().direction,'backward');
Object.assign(textInput.state,{selectionStart:4,selectionEnd:4,selectionDirection:'none'});assert.equal(readSelection().start,4);
for(const values of [[null,null,null],[-1,1,'forward'],[3,2,'forward'],[0,5,'forward'],[0.5,2,'forward'],[0,1,'invalid']]) {
  [textInput.state.selectionStart,textInput.state.selectionEnd,textInput.state.selectionDirection]=values;
  assert.equal(readSelection(),undefined);
}
Object.assign(textInput.state,{selectionStart:0,selectionEnd:1,selectionDirection:'none'});
assert.equal(readSelection().direction,'none','source direction retained for Rust to withhold ambiguous orientation');
textInput.failSelection=true;assert.equal(readSelection(),undefined);assert.equal(read(textInput,formFields).value,'A💡B');textInput.failSelection=false;
for(const privacy of ['password','autocomplete','caller']) {
  textInput.state.kind=privacy==='password'?'password':'text';textInput.attrs.autocomplete=privacy==='autocomplete'?'one-time-code':'';
  valueReads=selectionReads=0;const privateRead=read(textInput,formFields,privacy==='caller');
  assert.equal(privateRead.selection,undefined);assert.equal(privateRead.value,undefined);assert.deepEqual([valueReads,selectionReads],[0,0]);
}
textInput.state.kind='text';textInput.attrs={};
valueReads=selectionReads=0;assert.equal(read(textInput,['focused']).selection,undefined);assert.deepEqual([valueReads,selectionReads],[0,0]);
selectionReads=0;assert.equal(read(textInput,['value']).selection,undefined);assert.equal(selectionReads,0);
document.hasFocusFlag=false;assert.equal(readSelection(),undefined);assert.equal(selectionReads,0);document.hasFocusFlag=true;
document.activeElement=null;assert.equal(readSelection(),undefined);assert.equal(selectionReads,0);document.activeElement=textInput;
assert.equal(read(textInput,formFields,false,document,3).selection,undefined);assert.equal(selectionReads,0);
textInput.attrs['aria-invalid']='true';assert.equal(read(textInput,formFields).invalid,false,'native validity is independent of app aria-invalid');
const textarea=new HTMLTextAreaElement();Object.assign(textarea,{attrs:{},isConnected:true,ownerDocument:document,tagName:'TEXTAREA',state:{data:'',selectionStart:0,selectionEnd:0,selectionDirection:'none'}});
document.activeElement=textarea;const areaBefore=JSON.stringify([textarea.attrs,textarea.state]);assert.equal(read(textarea,['focused','value']).selection.start,0);assert.equal(JSON.stringify([textarea.attrs,textarea.state]),areaBefore);
console.log('Selection getter checks passed: UTF-16 ranges/collapsed/null/malformed/private/gates/textarea; offline mocks only.');

let outputReads=0,lengthReads=0,shapeReads=0;
class NativeNode {}
for(const key of ['firstChild','nextSibling','nodeType'])Object.defineProperty(NativeNode.prototype,key,{get(){shapeReads++;return this[key];}});
class CharacterData {}
Object.defineProperty(CharacterData.prototype,'length',{get(){lengthReads++;return this.data.length;}});
context.Node=NativeNode;context.CharacterData=CharacterData;
Object.defineProperty(HTMLOutputElement.prototype,'value',{get(){outputReads++;return this.outputValue;}});
const output=new HTMLOutputElement();Object.assign(output,{attrs:{},isConnected:true,ownerDocument:document,tagName:'OUTPUT',firstChild:null,outputValue:''});
assert.equal(read(output,['value']).value,'');assert.equal(outputReads,1);
output.firstChild={nodeType:3,nextSibling:null,data:'London'};output.outputValue='London';const outputBefore=JSON.stringify([output.attrs,output.firstChild,output.outputValue]);assert.equal(read(output,['value']).value,'London');assert.equal(JSON.stringify([output.attrs,output.firstChild,output.outputValue]),outputBefore);
for(const child of [{nodeType:1,nextSibling:null},{nodeType:3,nextSibling:{}},{nodeType:3,nextSibling:null,data:'x'.repeat(101)}]) {
  output.firstChild=child;outputReads=0;assert.equal(read(output,['value']).value,undefined);assert.equal(outputReads,0);
}
output.firstChild={nodeType:3,nextSibling:null,data:'x'.repeat(100)};output.outputValue='x'.repeat(100);assert.equal(read(output,['value']).value.length,100);
outputReads=lengthReads=shapeReads=0;assert.equal(read(output,['value'],true).value,undefined);assert.deepEqual([outputReads,lengthReads,shapeReads],[0,0,0]);
read(output,['focused']);assert.deepEqual([outputReads,lengthReads,shapeReads],[0,0,0]);
console.log('Output getter checks passed: empty/nonempty/exact bound/shape/oversize/private/fields; offline mocks only.');

selectionReads=valueReads=0;document.activeElement=textInput;document.hasFocusFlag=true;
const focusOnly=read(textInput,['focused']);assert.equal(focusOnly.focused,true);assert.equal(focusOnly.documentFocused,true);assert.equal(focusOnly.selection,undefined);assert.deepEqual([selectionReads,valueReads],[0,0]);
console.log('Independent native document-focus fact requires no value/selection getter.');

const activationSource=fs.readFileSync(require('node:path').join(__dirname,'../../../src/collector/activate.rs'),'utf8').match(/const ACTIVATE: &str = r#"([\s\S]*?)"#;/)[1];
let nativeClicks=0;
class HTMLElement extends Element { click(){nativeClicks++;this.isConnected=false;} }
context.HTMLElement=HTMLElement;
vm.runInContext(activationSource,context,{timeout:100});
const button=()=>Object.assign(new HTMLButtonElement(),{attrs:{},isConnected:true,ownerDocument:document,disabled:false,
  click(){throw Error('own click override forbidden');},onclick(){throw Error('direct handler forbidden');}});
const publicOutput=()=>Object.assign(new HTMLOutputElement(),{attrs:{},isConnected:true,ownerDocument:document});
const activate=(actor,result,doc=document)=>context.activateButton.call(actor,doc,result).invoked;
let actor=button();nativeClicks=0;assert.equal(activate(actor,publicOutput()),true);assert.equal(nativeClicks,1);assert.equal(actor.isConnected,false,'self-removal does not undo native invocation');
for(const change of [a=>{a.isConnected=false;},a=>{a.ownerDocument={};},a=>{a.disabled=true;},a=>{a.attrs.autocomplete='one-time-code';}]){
  actor=button();change(actor);nativeClicks=0;assert.equal(activate(actor,publicOutput()),false);assert.equal(nativeClicks,0);
}
for(const change of [r=>{r.isConnected=false;},r=>{r.ownerDocument={};},r=>{r.attrs.autocomplete='current-password';}]){
  const result=publicOutput();change(result);nativeClicks=0;assert.equal(activate(button(),result),false);assert.equal(nativeClicks,0);
}
nativeClicks=0;assert.equal(activate(Object.assign(new Element(),{attrs:{},isConnected:true,ownerDocument:document}),publicOutput()),false);
assert.equal(activate(button(),Object.assign(new Element(),{attrs:{},isConnected:true,ownerDocument:document})),false);assert.equal(nativeClicks,0);
const resultInput=node();resultInput.state.kind='text';resultInput.state.readOnly=true;resultInput.disabled=true;
nativeClicks=valueReads=0;assert.equal(activate(button(),resultInput),true);assert.equal(nativeClicks,1);assert.equal(valueReads,0,'activation never reads result values');
resultInput.state.kind='password';nativeClicks=valueReads=0;assert.equal(activate(button(),resultInput),false);assert.deepEqual([nativeClicks,valueReads],[0,0]);
nativeClicks=0;assert.equal(activate(button(),publicOutput(),{}),false);assert.equal(nativeClicks,0);
console.log('Native activation guard mocks passed: exact button/result/document, disabled/private/stale refusal, readonly result and no direct handler/focus/value/own-click fallback.');

class Window {}
class VisualViewport {}
const viewportWindow=new Window(),visual=new VisualViewport();viewportWindow.top=viewportWindow;
const viewportFacts={scrollX:-12.25,scrollY:100.5,innerWidth:1000,innerHeight:600,devicePixelRatio:2,scale:1,offsetLeft:0,offsetTop:0,pageLeft:-12.25,pageTop:100.5,width:985,height:600};
let viewportReads=0;
for(const name of ['scrollX','scrollY','innerWidth','innerHeight','devicePixelRatio'])Object.defineProperty(viewportWindow,name,{get(){viewportReads++;return viewportFacts[name];}});
Object.defineProperty(viewportWindow,'visualViewport',{get(){return visual;}});
for(const name of ['scale','offsetLeft','offsetTop','pageLeft','pageTop','width','height'])Object.defineProperty(VisualViewport.prototype,name,{get(){viewportReads++;return viewportFacts[name];}});
context.Window=Window;context.VisualViewport=VisualViewport;document.defaultView=viewportWindow;
const layoutNode=node();const mapped=read(layoutNode,['layout_bounds']).rect;
assert.deepEqual([mapped.x,mapped.y,mapped.width,mapped.height],[40,60,120,40],'raw rect never translated in JS');
assert.deepEqual(JSON.parse(JSON.stringify(mapped.viewport.before)),{scrollX:-12.25,scrollY:100.5,width:1000,height:600,dpr:2,scale:1,offsetLeft:0,offsetTop:0,pageLeft:-12.25,pageTop:100.5,visualWidth:985,visualHeight:600});
assert.deepEqual(mapped.viewport.before,mapped.viewport.after);viewportReads=0;read(layoutNode,['enabled']);assert.equal(viewportReads,0);
viewportFacts.scale=2;assert.equal(read(layoutNode,['layout_bounds']).rect.viewport.before.scale,2,'unsupported scale remains an actual fact for Rust');viewportFacts.scale=1;
viewportWindow.top={};assert.equal(read(layoutNode,['layout_bounds']).rect.viewport.before,null);viewportWindow.top=viewportWindow;
viewportFacts.scrollX=NaN;assert.equal(read(layoutNode,['layout_bounds']).rect.viewport.before,null);viewportFacts.scrollX=-12.25;
const oldRect=Element.prototype.getBoundingClientRect;
Element.prototype.getBoundingClientRect=function(){viewportFacts.scrollY++;viewportFacts.pageTop++;return oldRect.call(this);};
const changing=read(layoutNode,['layout_bounds']).rect.viewport;assert.notEqual(changing.before.scrollY,changing.after.scrollY);
Element.prototype.getBoundingClientRect=oldRect;delete document.defaultView;
console.log('Viewport source mocks passed: native getters, signed/fractional CSS facts, separate scrollbar widths, requested-only reads, unsupported/missing and changed context; no JS conversion.');
