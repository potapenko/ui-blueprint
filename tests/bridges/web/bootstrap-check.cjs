// Focused optional-caller proof. --check is offline; live mode owns only F01.
const assert=require('node:assert/strict');
const fs=require('node:fs/promises');
const path=require('node:path');
const os=require('node:os');
const crypto=require('node:crypto');
const {spawn,execFileSync}=require('node:child_process');
const helper=require('./geometry.cjs');
const connection='ws://127.0.0.1:9222/devtools/browser/owned';
const url='http://127.0.0.1:3000/#first';
const args=(selector='#f01')=>['--connection',connection,'--url',url,'--component',selector];
async function rejects(work,code){await assert.rejects(work,e=>e.code===code);}
function diagnostic(overrides={}){
  return {output_version:'1.0.0',kind:'observe_diagnostic',exit_code:0,caller_phase:'complete',caller_error:null,
    operation:'observe',terminal:'completed',terminal_error:null,requested_channels:1,committed_channels:1,
    missing_channels:0,incomplete_channels:0,worker_cleanup:'confirmed',host_diagnostic:null,...overrides};
}

async function offline(){
  const encode=d=>Buffer.from(JSON.stringify(d)+'\n');
  const good=diagnostic(),timeout=diagnostic({exit_code:4,caller_phase:'wait_terminal',caller_error:'observe_incomplete',terminal:'timed_out',committed_channels:0,missing_channels:1});
  assert.equal(helper.parseObserveDiagnostic(encode(good),0).host_diagnostic,null);
  assert(helper.formatObserveDiagnostic(helper.parseObserveDiagnostic(encode(timeout),4)).includes('terminal=timed_out'));
  const limit=diagnostic({exit_code:2,caller_error:'observe_invalid_or_limit',terminal:'failed',terminal_error:'resource_limit',host_diagnostic:{stage:'collect',cause:'collector_limit',code:0,selection_visited_nodes:null,remote_cleanup:'released',send_progress:'flushed'}});
  const message=helper.formatObserveDiagnostic(helper.parseObserveDiagnostic(encode(limit),2));
  assert(message.includes('cause=collector_limit code=0'));assert(!message.includes('output_bytes')&&!message.includes('deadline_ms'));
  for(const bad of [diagnostic({caller_error:'PRIVATE_CANARY'}),diagnostic({selector:'PRIVATE_CANARY'}),diagnostic({worker_cleanup:'unconfirmed'}),diagnostic({requested_channels:8}),diagnostic({host_diagnostic:{...limit.host_diagnostic,cause:'PRIVATE_CANARY'}})]){
    assert.throws(()=>helper.parseObserveDiagnostic(encode(bad),bad.exit_code),e=>e.code==='observe_diagnostic_invalid'&&!e.message.includes('PRIVATE_CANARY'));
  }
  for(const bad of [Buffer.from('observe_incomplete\n'),Buffer.from('x'.repeat(4097)),Buffer.from(JSON.stringify(good).replace('"exit_code":0','"exit_code":0,"exit_code":0')+'\n')])assert.throws(()=>helper.parseObserveDiagnostic(bad,0),e=>e.code==='observe_diagnostic_invalid');
  assert.throws(()=>helper.parseObserveDiagnostic(encode(good),4),e=>e.code==='observe_diagnostic_invalid');
  const parsed=helper.parseFirstUse(args());assert.equal(parsed.selector,'#f01');assert.equal(parsed.maxVisited,2048);assert.equal(parsed.json,false);
  assert.equal(helper.parseFirstUse([...args(),'--json']).json,true);
  assert.equal(helper.parseFirstUse(['--json',...args()]).json,true);
  for(const extra of [['--json','--json'],['--json','true'],['--endpoint','ws://127.0.0.1:9222/devtools/page/x','--json']])
    assert.throws(()=>helper.parseFirstUse([...args(),...extra]),e=>e.code==='invalid_arguments');
  for(const selector of ['div.card','[role="dialog"][aria-label="Режиссёр"]','*','#f01','.card[data-testid="x"]'])assert.equal(helper.parseFirstUse(args(selector)).selector,selector);
  for(const selector of ['ul:nth-of-type(1)','UL:nth-of-type(2048)','*.list:nth-of-type(6)','[data-x="слово:nth-of-type(3)"]:nth-of-type(2)',"[x='a:b']"]){
    const p=helper.parseFirstUse(args(selector));assert.equal(p.selector,selector);
    assert(!p.compoundSelector.endsWith(':nth-of-type(2)'));assert(p.nthOfType===null||p.nthOfType>=1);
  }
  for(const selector of ['ul:nth-of-type(0)','ul:nth-of-type(01)','ul:nth-of-type(+1)','ul:nth-of-type(-1)','ul:nth-of-type(1.0)','ul:nth-of-type(odd)','ul:nth-of-type(2n+1)','ul:nth-of-type(2049)','ul:nth-of-type(99999)','ul:nth-of-type( 1)','ul:nth-of-type(1 )',':nth-of-type(1)','ul:nth-of-type(1).x','ul:nth-of-type(1):nth-of-type(2)','ul:nth-child(1)','ul:NTH-OF-TYPE(1)','ul:nth-of-type(1) > li','ul:nth-of-type(1),li','ul:nth-of-type(1)garbage'])assert.throws(()=>helper.parseFirstUse(args(selector)),e=>e.code==='unsupported_selector');
  const exact512='[x="'+'я'.repeat(253)+'"]';assert.equal(Buffer.byteLength(exact512),512);assert.equal(helper.parseFirstUse(args(exact512)).compoundSelector,exact512);
  assert.throws(()=>helper.parseFirstUse(args(exact512.replace('я','яя'))),e=>e.code==='unsupported_selector');
  for(const selector of ['div > input','div input','.a,.b',':has(input)',':first-child','[foo:bar]','#a\\:b','[data-x="oops]',''])assert.throws(()=>helper.parseFirstUse(args(selector)),e=>['unsupported_selector','invalid_arguments'].includes(e.code));
  for(const input of [[],['--url',url,'--component','#f01']])assert.throws(()=>helper.parseFirstUse(input),e=>e.code==='connection_required');
  for(const ws of ['wss://127.0.0.1:9222/devtools/browser/a','ws://localhost:9222/devtools/browser/a','ws://127.0.0.1:9222/devtools/page/a','ws://u:p@127.0.0.1:9222/devtools/browser/a','ws://127.0.0.1:9222/devtools/browser/a?q=x'])assert.throws(()=>helper.parseFirstUse(['--connection',ws,'--url',url,'--component','#f01']),e=>e.code==='invalid_connection');
  for(const cap of ['0','2049','-1','NaN'])assert.throws(()=>helper.parseFirstUse([...args(),'--max-visited-nodes',cap]),e=>e.code==='invalid_selection_limit');
  assert.throws(()=>helper.parseFirstUse([...args(),'--url',url]),e=>e.code==='invalid_arguments');
  assert.throws(()=>helper.parseFirstUse([...args(),'--cli','/only-one']),e=>e.code==='matching_executables_required');
  assert.throws(()=>helper.normalizePageUrl('http://u:p@localhost/'),e=>e.code==='invalid_page_url');
  assert.throws(()=>helper.normalizePageUrl('http://localhost/?token=CANARY'),e=>e.code==='private_page_url');
  assert.throws(()=>helper.normalizePageUrl('http://localhost/#/view?access_token=CANARY'),e=>e.code==='private_page_url');
  assert.equal(helper.normalizePageUrl('HTTP://LOCALHOST:80/path#one'),'http://localhost/path#one');
  assert.notEqual(helper.normalizePageUrl(url),helper.normalizePageUrl(url.replace('#first','#second')));

  const old={Document:global.Document,Element:global.Element,document:global.document,WebSocket:global.WebSocket};
  class Document{};class Element{constructor(key){this.key=key;this.localName='div';this.namespaceURI='http://www.w3.org/1999/xhtml';this.isConnected=true;}matches(selector){assert(!selector.includes(':nth-of-type('),'no opaque native ordinal scan');return selector===this.key||selector===this.localName||selector==='*';}get previousSibling(){throw new Error('uncharged sibling scan');}}
  global.Document=Document;global.Element=Element;const document=new Document();global.document=document;
  function children(parent,...nodes){parent.firstChild=nodes[0]??null;nodes.forEach((n,i)=>{n.parentNode=parent;n.nextSibling=nodes[i+1]??null;n.ownerDocument=document;});}
  const a=new Element('#one'),b=new Element('#two');children(document,a,b);
  const options={selector:'#one',maxVisited:20,remainingMs:1000};
  try{
    const selected=helper.selectComponent.call(document,options);assert.equal(selected.status,'selected');assert.equal(selected.root,a);
    assert.equal(helper.selectComponent.call(document,{...options,maxVisited:1}).status,'incomplete');
    assert.equal(helper.selectComponent.call(document,{...options,selector:'*'}).status,'ambiguous');
    assert.equal(helper.selectComponent.call(document,{...options,selector:'#absent'}).status,'missing');
    assert.equal(helper.selectComponent.call(document,{...options,remainingMs:-1}).status,'timeout');
    a.shadowRoot={};assert.equal(helper.selectComponent.call(document,options).status,'unsupported');delete a.shadowRoot;
    const parent=new Element('#parent'),other=new Element('#other');
    const list=key=>{const e=new Element(key);e.localName='ul';return e;};
    const first=list('#first'),second=list('#second'),svg=list('#svg');svg.namespaceURI='http://www.w3.org/2000/svg';
    const upper=list('#upper');upper.localName='UL';const text={},comment={};
    children(document,parent);children(parent,first,text,svg,upper,comment,second);
    const nth={selector:'ul',nthOfType:2,maxVisited:20,remainingMs:1000};
    let held=helper.selectComponent.call(document,nth);assert.equal(held.root,second);
    assert.equal(helper.selectComponent.call(document,{...nth,selector:'#second'}).root,second,'count nonmatching same-type siblings');
    assert.equal(helper.selectComponent.call(document,{...nth,maxVisited:7}).status,'incomplete');
    const tail={};children(parent,first,text,svg,upper,comment,second,tail);
    assert.equal(helper.selectComponent.call(document,{...nth,maxVisited:8}).status,'incomplete','match before unvisited tail is not unique');
    children(document,parent,other);children(other,list('#third'),list('#fourth'));
    assert.equal(helper.selectComponent.call(document,nth).status,'ambiguous');
    children(document,parent);children(parent,first,second);
    for(const mutation of ['insert','remove','reorder','remount','reparent']){
      children(document,parent,other);children(parent,first,second);children(other);
      held=helper.selectComponent.call(document,nth);assert.equal(held.root,second);
      if(mutation==='insert')children(parent,list('#new'),first,second);
      if(mutation==='remove')children(parent,second);
      if(mutation==='reorder')children(parent,second,first);
      if(mutation==='remount'){children(parent,first,list('#replacement'));second.isConnected=false;}
      if(mutation==='reparent'){children(parent,first);children(other,list('#new'),second);}
      assert.notEqual(helper.selectComponent.call(held,nth).status,'selected',mutation);
      second.isConnected=true;
    }
    children(document,a,b);
    const replacement=new Element('#one');children(document,replacement,b);a.isConnected=false;
    assert.equal(helper.selectComponent.call(selected,options).status,'stale');assert.equal(helper.heldCurrent.call(selected).current,false);
  }finally{global.Document=old.Document;global.Element=old.Element;global.document=old.document;}

  class FakeSocket extends EventTarget{
    static sockets=[];static autoOpen=true;static autoClose=true;static reply=q=>({id:q.id,result:{ok:true}});
    constructor(endpoint){super();this.endpoint=endpoint;this.sent=[];this.didClose=false;FakeSocket.sockets.push(this);if(FakeSocket.autoOpen)queueMicrotask(()=>this.dispatchEvent(new Event('open')));}
    send(wire){const request=JSON.parse(wire);this.sent.push(request);const reply=FakeSocket.reply(request,this);if(reply!==undefined)queueMicrotask(()=>this.dispatchEvent(new MessageEvent('message',{data:typeof reply==='string'?reply:JSON.stringify(reply)})));}
    close(){this.didClose=true;if(FakeSocket.autoClose)queueMicrotask(()=>this.dispatchEvent(new Event('close')));}
  }
  global.WebSocket=FakeSocket;
  try{
    let c=await helper.BootstrapCdp.open(connection,performance.now()+5000);
    FakeSocket.reply=q=>({id:q.id+1,result:{}});await rejects(()=>c.send('test'),'cdp_uncorrelated_reply');await c.close();
    FakeSocket.reply=()=> 'x'.repeat(65537);c=await helper.BootstrapCdp.open(connection,performance.now()+5000);await rejects(()=>c.send('test'),'bootstrap_limit');await c.close();
    FakeSocket.reply=q=>({id:q.id,result:'x'.repeat(33000)});c=await helper.BootstrapCdp.open(connection,performance.now()+5000);
    for(let i=0;i<7;i++)await c.send('test');await rejects(()=>c.send('test'),'bootstrap_limit');await c.close();
    FakeSocket.reply=q=>({id:q.id,result:{}});c=await helper.BootstrapCdp.open(connection,performance.now()+5000);
    for(let i=0;i<32;i++)await c.send('test');await rejects(()=>c.send('test'),'bootstrap_limit');await c.close();
    c=await helper.BootstrapCdp.open(connection,performance.now()+5000);for(let i=0;i<17;i++)c.socket.dispatchEvent(new MessageEvent('message',{data:'{"method":"event"}'}));await rejects(()=>c.send('test'),'bootstrap_limit');await c.close();
    FakeSocket.autoOpen=false;await rejects(()=>helper.BootstrapCdp.open(connection,performance.now()+20),'bootstrap_deadline');FakeSocket.autoOpen=true;
    c=await helper.BootstrapCdp.open(connection,performance.now()+5000);FakeSocket.autoClose=false;await rejects(()=>c.close(),'cdp_cleanup_unconfirmed');FakeSocket.autoClose=true;

    let mode='ok',frames=0;
    FakeSocket.reply=q=>{
      let result;
      if(q.method==='Target.getTargets')result={targetInfos:mode==='missing'?[]:Array.from({length:mode==='ambiguous'?2:1},(_,i)=>({targetId:'target'+i,type:'page',url}))};
      else if(q.method==='Target.getTargetInfo')result={targetInfo:{targetId:'target0',type:'page',url}};
      else if(q.method==='Page.getFrameTree')result={frameTree:{frame:{id:'frame',loaderId:mode==='stale-loader'&&++frames>1?'new':'loader'}}};
      else if(q.method==='DOM.getDocument')result={root:{nodeType:9,backendNodeId:1}};
      else if(q.method==='Page.createIsolatedWorld')result={executionContextId:1};
      else if(q.method==='Runtime.evaluate')result={result:{objectId:'document'}};
      else if(q.method==='Runtime.callFunctionOn')result={result:q.params.returnByValue?{value:q.params.functionDeclaration.includes('heldCurrent')?{current:true}:{status:mode==='remount'?'stale':'selected'}}:{objectId:'container'}};
      else if(q.method==='Runtime.getProperties')result={result:[{name:'status',value:{value:mode==='malformed-status'?'constructor':'selected'}},{name:'root',value:{objectId:'element'}}]};
      else if(q.method==='DOM.describeNode')result={node:{nodeType:1,backendNodeId:42,...(['closed-shadow','ua-shadow'].includes(mode)?{shadowRoots:[{shadowRootType:mode==='closed-shadow'?'closed':'user-agent'}]}:{})}};
      else if(q.method==='Runtime.releaseObjectGroup')result={};
      else throw new Error('unexpected_method');
      return {id:q.id,result};
    };
    for(const [which,expected] of [['missing','target_unresolved'],['ambiguous','ambiguous_target'],['stale-loader','stale_target'],['malformed-status','cdp_invalid_reply'],['closed-shadow','unsupported_scope']]){
      mode=which;frames=0;await rejects(()=>helper.openBootstrap(parsed),expected);assert(FakeSocket.sockets.every(s=>s.didClose));
    }
    mode='ok';const prepared=await helper.openBootstrap(parsed);assert.equal(prepared.args.root,42);assert.equal(prepared.binding.document,1);
    mode='remount';await rejects(()=>prepared.revalidate(),'stale_target');await prepared.close();assert(FakeSocket.sockets.every(s=>s.didClose));
    mode='ok';const boundary=await helper.openBootstrap(parsed);
    mode='closed-shadow';await rejects(()=>boundary.revalidate(),'unsupported_scope');await boundary.close();assert(FakeSocket.sockets.every(s=>s.didClose));
    mode='ua-shadow';const control=await helper.openBootstrap(parsed);await control.revalidate();await control.close();
  }finally{global.WebSocket=old.WebSocket;}
  // The overall preparation deadline also bounds a stalled Observe child.
  const temp=await fs.mkdtemp(path.join(os.tmpdir(),'uib-bootstrap-deadline-'));
  const delayed=path.join(temp,'delayed-cli.cjs'),guard=path.join(temp,'no-effects.cjs');
  try{
    await fs.writeFile(delayed,'#!/usr/bin/env node\nsetTimeout(()=>{},2000);\n',{mode:0o700});
    await fs.writeFile(guard,"const no=()=>{throw new Error('UNEXPECTED_EFFECT_CANARY')};global.WebSocket=class{constructor(){no()}};require('node:child_process').spawn=no;for(const n of ['stat','access','mkdtemp','open'])require('node:fs/promises')[n]=no;\n",{mode:0o600});
    for(const selector of ['ul:nth-of-type(0)','ul:nth-of-type(2049)','ul:nth-of-type(2n+1)','ul:nth-of-type(2) > li']){
      let refused=false;
      try{execFileSync(process.execPath,['--require',guard,path.join(__dirname,'geometry.cjs'),...args(selector)],{timeout:2000,stdio:['ignore','pipe','pipe']});}
      catch(e){assert.equal(e.status,2);assert.equal(e.stdout.length,0);assert.equal(e.stderr.toString(),'unsupported_selector\n');refused=true;}
      assert(refused);
    }
    const ownedDirs=[],make=fs.mkdtemp;fs.mkdtemp=async(...args)=>{const dir=await make(...args);ownedDirs.push(dir);return dir;};
    try{
      const owned={cli:delayed,worker:'/unused',target:'target',endpoint:'ws://127.0.0.1:9222/devtools/page/target',root:42};
      await rejects(()=>helper.example(owned,{binding:{surface:{id:'frame',generation:'loader'},document:1},remaining:()=>20,revalidate:()=>{throw new Error('must not publish');}}),'bootstrap_deadline');
      await fs.writeFile(delayed,'#!/usr/bin/env node\nprocess.stderr.write('+JSON.stringify(JSON.stringify(diagnostic())+'\n')+');\nprocess.exit(0);\n');
      await assert.rejects(()=>helper.example(owned,{binding:{surface:{id:'frame',generation:'loader'},document:1},remaining:()=>1000}),e=>e.code==='observe_refused_exit_0'&&e.exit===1);
    }finally{fs.mkdtemp=make;}
    assert.equal(ownedDirs.length,2);for(const dir of ownedDirs)await assert.rejects(fs.stat(dir),e=>e.code==='ENOENT');
  }finally{await fs.unlink(delayed);await fs.unlink(guard);await fs.rmdir(temp);}
  await machineOffline();
  console.log('PASS offline: input/URL/selector, unique bounded walk, remount, CDP bytes/calls/events/deadline/cleanup, source-derived binding');
}

async function machineOffline(){
  // Synthetic child protocol only; live proof below checks actual canonical data.
  const temp=await fs.mkdtemp(path.join(os.tmpdir(),'uib-json-offline-')),cli=path.join(temp,'cli.cjs');
  const dirs=[],make=fs.mkdtemp,unlink=fs.unlink,write=process.stdout.write,report=console.error;
  const output=[],diagnostics=[];let revalidated=false,remoteClosed=false;
  const bytes=Buffer.from('{ "artifact": {"kind":"channel_response","data":{"result":{"status":"observed","data":{"nodes":[{"key":{"namespace":"web.dom","key":"42"}}]}}}}}\n');
  const owned={cli,worker:'/unused',target:'target',endpoint:'ws://127.0.0.1:9222/devtools/page/target',root:42,json:true};
  const bootstrap=()=>({binding:{surface:{id:'frame',generation:'loader'},document:1},remaining:()=>1000,
    revalidate:async()=>{revalidated=true;},close:async()=>{remoteClosed=true;}});
  async function child(data,code=0,d=diagnostic({exit_code:code,incomplete_channels:code===4?1:0})){
    await fs.writeFile(cli,'#!/usr/bin/env node\nif(process.argv[2]!=="observe")process.exit(99);\nprocess.stdout.write(Buffer.from('+JSON.stringify(data.toString('base64'))+',"base64"));\nprocess.stderr.write('+JSON.stringify(JSON.stringify(d)+'\n')+');process.exitCode='+code+';\n',{mode:0o700});
  }
  try{
    fs.mkdtemp=async(...a)=>{const d=await make(...a);dirs.push(d);return d;};
    process.stdout.write=function(data,callback){output.push(Buffer.from(data));callback?.();return true;};
    console.error=(...a)=>diagnostics.push(a.join(' '));
    for(const code of [0,4]){
      await child(bytes,code);revalidated=false;remoteClosed=false;
      const result=await helper.example(owned,bootstrap());assert.equal(result.code,code);assert(result.bytes.equals(bytes));
      assert(revalidated&&remoteClosed);assert.equal(output.length,0);
      for(const d of dirs)await assert.rejects(fs.stat(d),e=>e.code==='ENOENT');
      await helper.publishCanonical(result.bytes);assert(Buffer.concat(output).equals(bytes));output.length=0;
    }
    assert(diagnostics.some(x=>x.startsWith('observe_diagnostic exit=4 ')));
    await child(Buffer.alloc(0),4);await rejects(()=>helper.example(owned,bootstrap()),'observe_refused_exit_4');
    await child(bytes,2,diagnostic({exit_code:2,terminal:'failed'}));await rejects(()=>helper.example(owned,bootstrap()),'observe_refused_exit_2');
    await child(bytes,4,diagnostic({exit_code:4,terminal:'failed'}));await rejects(()=>helper.example(owned,bootstrap()),'observe_not_completed');
    await child(bytes,0,diagnostic({worker_cleanup:'not_started'}));await rejects(()=>helper.example(owned,bootstrap()),'observe_not_completed');
    await child(Buffer.from('{"artifact":{"kind":"channel_response","data":{"result":{"status":"failed"}}}}'));
    await rejects(()=>helper.example(owned,bootstrap()),'snapshot_unavailable');
    await child(bytes);
    for(const [field,code] of [['revalidate','stale_target'],['close','bootstrap_cleanup_unconfirmed']]){
      const b=bootstrap();b[field]=async()=>{throw Object.assign(new Error(code),{code});};
      await rejects(()=>helper.example(owned,b),code);
    }
    fs.unlink=async file=>{if(file.endsWith('request.json'))throw new Error('PRIVATE_CLEANUP_CANARY');return unlink(file);};
    await rejects(()=>helper.example(owned,bootstrap()),'example_cleanup_unconfirmed');fs.unlink=unlink;
    assert.equal(output.length,0,'no publication before cleanup or after refusal');
    const destroy=process.stdout.destroy;process.stdout.destroy=()=>{};
    try{
      process.stdout.write=function(data,callback){const e=new Error('PRIVATE_WRITE_CANARY');callback(e);setImmediate(()=>process.stdout.emit('error',e));return false;};
      await rejects(()=>helper.publishCanonical(bytes),'stdout_write_failed');
      let destroyed=false;process.stdout.destroy=()=>{destroyed=true;};
      process.stdout.write=()=>false;
      await rejects(()=>helper.publishCanonical(bytes),'stdout_write_failed');assert(destroyed,'timed-out pipe retired');
    }finally{process.stdout.destroy=destroy;}
  }finally{
    fs.mkdtemp=make;fs.unlink=unlink;process.stdout.write=write;console.error=report;
    for(const d of dirs){try{for(const f of await fs.readdir(d))await fs.unlink(path.join(d,f));await fs.rmdir(d);}catch(e){if(e.code!=='ENOENT')throw e;}}
    await fs.unlink(cli);await fs.rmdir(temp);
  }
  console.log('PASS machine: exact bytes/partial, no human path, failed/empty/stale/cleanup suppress output, safe write failure');
}

async function scopeOffline(){
  const vm=require('node:vm'),crypto=require('node:crypto');
  const previous={exports:{}};
  vm.runInNewContext(execFileSync('git',['show','fad3dbe3:tests/bridges/web/geometry.cjs'],{cwd:path.resolve(__dirname,'../../..'),encoding:'utf8',timeout:5000}),{require,module:previous,__dirname,process,Buffer,URL,URLSearchParams,setTimeout,clearTimeout,setImmediate,performance,console});
  const old=previous.exports,temp=await fs.mkdtemp(path.join(os.tmpdir(),'uib-scope-profile-')),cli=path.join(temp,'cli.cjs'),capture=path.join(temp,'input.json');
  const uuid=crypto.randomUUID,report=console.error;
  const owned={cli,worker:'/unused',target:'target',endpoint:'ws://127.0.0.1:9222/devtools/page/target',root:42,json:true};
  const binding={surface:{id:'frame',generation:'loader'},document:1};
  const raw=JSON.stringify({artifact:{kind:'channel_response',data:{result:{status:'observed',data:{nodes:Array.from({length:16},(_,i)=>({key:{namespace:'web.dom',key:String(42+i)}}))}}}}})+'\n';
  try{
    crypto.randomUUID=()=> '11111111-1111-4111-8111-111111111111';console.error=()=>{};
    assert.equal(JSON.stringify(helper.inputs(owned,binding)),JSON.stringify(old.inputs(owned,binding)),'legacy exact-ID fixed input bytes');
    await fs.writeFile(cli,'#!'+process.execPath+'\nconst fs=require("node:fs"),a=process.argv.slice(2);if(a[0]!=="observe")process.exit(99);fs.writeFileSync('+JSON.stringify(capture)+',JSON.stringify([JSON.parse(fs.readFileSync(a[a.indexOf("--connection")+1])),JSON.parse(fs.readFileSync(a[a.indexOf("--request")+1]))]),{mode:384});process.stdout.write('+JSON.stringify(raw)+');process.stderr.write('+JSON.stringify(JSON.stringify(diagnostic({exit_code:4,incomplete_channels:1}))+'\n')+');process.exitCode=4;\n',{mode:0o700});
    const b=()=>({binding,remaining:()=>1000,revalidate:async()=>{},close:async()=>{}});
    const before=await old.example(owned,b());const original=JSON.parse(await fs.readFile(capture,'utf8'));
    const after=await helper.example(owned,b());const updated=JSON.parse(await fs.readFile(capture,'utf8'));
    assert.equal(original[0].provider.setup.collector.max_nodes,16);assert.equal(updated[0].provider.setup.collector.max_nodes,32);
    assert.equal(original[0].provider.setup.collector.max_methods,100);assert.equal(updated[0].provider.setup.collector.max_methods,109);
    updated[0].provider.setup.collector.max_nodes=16;updated[0].provider.setup.collector.max_methods=100;assert.deepEqual(updated,original,'only URL nodes/methods differ');
    assert.equal(before.code,4);assert.equal(after.code,4);assert(before.bytes.equals(after.bytes));assert(after.bytes.equals(Buffer.from(raw)));
  }finally{
    crypto.randomUUID=uuid;console.error=report;
    for(const f of [cli,capture]){try{await fs.unlink(f);}catch(e){if(e.code!=='ENOENT')throw e;}}
    await fs.rmdir(temp);await assert.rejects(fs.stat(temp),e=>e.code==='ENOENT');
  }
  console.log('PASS scope profile: only URL nodes16→32/methods100→109; exact legacy inputs and fixed16-node response bytes preserved');
}

async function live(){
  assert(process.env.UIB_WEB_LIVE_ALLOW==='1','explicit own-fixture activation required');
  const bundle=process.env.UIB_WEB_FIRST_USE_BUNDLE;assert(bundle&&path.isAbsolute(bundle));
  const addressing=process.argv.includes('--run-address-authorized'),whole=process.argv.includes('--run-scope-authorized');
  const selected=whole?'#scope-fixture':addressing?'ul:nth-of-type(6)':'#f01',width=whole?384:addressing?240:360,height=whole?420:addressing?160:123;
  const executable=path.join(bundle,'web-geometry.cjs');
  assert((await fs.readFile(executable)).equals(await fs.readFile(path.join(__dirname,'geometry.cjs'))),'installed helper drift');
  const manifest=JSON.parse(await fs.readFile(path.join(bundle,'distribution-manifest.json'),'utf8'));
  for(const name of ['uiblueprint','session-worker','web-geometry.cjs'])assert.equal(crypto.createHash('sha256').update(await fs.readFile(path.join(bundle,name))).digest('hex'),manifest.files[name].sha256);
  const workers=[path.join(bundle,'session-worker'),await fs.realpath(path.join(bundle,'session-worker'))];
  function noWorkers(){
    const escaped=workers.map(x=>x.replace(/[.*+?^${}()|[\]\\]/g,'\\$&'));
    try{execFileSync('/usr/bin/pgrep',['-f',`^(${escaped.join('|')})( |$)`],{timeout:1000,stdio:['ignore','pipe','pipe']});}
    catch(error){if(error.status===1)return;throw error;}
    throw new Error('owned_worker_still_running');
  }
  const {prepare}=require('./fixture-host.cjs'),setup=prepare();
  const {start}=require('../../../fixtures/web/server.cjs');
  let fixture,server,browser,context,profile,child;const results=[];
  const wait=(p,ms)=>{let timer;return Promise.race([p,new Promise((_,reject)=>{timer=setTimeout(()=>reject(new Error('test_timeout')),ms);})]).finally(()=>clearTimeout(timer));};
  async function invoke(ws,pageUrl,component,expected,extra=[]){
    child=spawn(process.execPath,[executable,'--connection',ws,'--url',pageUrl,'--component',component,...extra],{cwd:'/',stdio:['ignore','pipe','pipe']});
    const out=[],err=[];let bytes=0;
    for(const [stream,dest] of [[child.stdout,out],[child.stderr,err]])stream.on('data',chunk=>{bytes+=chunk.length;if(bytes>262144)child.kill('SIGKILL');else dest.push(chunk);});
    const code=await wait(new Promise((resolve,reject)=>{child.once('error',reject);child.once('close',(code,signal)=>signal?reject(new Error('child_signal')):resolve(code));}),15000);
    const stdout=Buffer.concat(out).toString(),stderr=Buffer.concat(err).toString();assert.equal(code,expected,stderr);assert(!stdout.includes('PRIVATE_BOOTSTRAP_CANARY')&&!stderr.includes('PRIVATE_BOOTSTRAP_CANARY'));
    noWorkers();
    results.push({component,exit:code,stdout_bytes:Buffer.byteLength(stdout),stderr:stderr.trim()});return {stdout,stderr,bytes:Buffer.concat(out)};
  }
  try{
    fixture=await start();server=await setup.chromium.launchServer({headless:true,timeout:8000,args:['--remote-debugging-port=0','--remote-debugging-address=127.0.0.1']});
    profile=server.process().spawnargs.find(x=>x.startsWith('--user-data-dir=')).slice('--user-data-dir='.length);
    assert(path.basename(profile).startsWith('playwright_chromiumdev_profile-'));
    assert((await fs.realpath(profile)).startsWith((await fs.realpath(os.tmpdir()))+path.sep));
    let ws;for(let i=0;i<100;i++){try{const lines=(await fs.readFile(path.join(profile,'DevToolsActivePort'),'utf8')).split('\n');ws=`ws://127.0.0.1:${lines[0]}${lines[1]}`;break;}catch(e){if(e.code!=='ENOENT')throw e;await new Promise(r=>setTimeout(r,20));}}
    assert(ws);browser=await setup.chromium.connect(server.wsEndpoint(),{timeout:3000});assert.equal(browser.version(),'145.0.7632.6');
    context=await browser.newContext({viewport:{width:800,height:600},deviceScaleFactor:1});
    await context.route('**/*',r=>new URL(r.request().url()).origin===fixture.url?r.continue():r.abort());
    const page=await context.newPage();page.setDefaultTimeout(2000);page.setDefaultNavigationTimeout(3000);
    await page.goto(fixture.url+'/?generation=1#first');await page.waitForFunction(()=>!!window.f01);
    if(whole)await page.evaluate(()=>{
      const root=document.createElement('div');root.id='scope-fixture';
      root.innerHTML='<style>#scope-fixture{position:fixed;left:20px;top:100px;width:384px;height:420px;box-sizing:border-box;background:white}#scope-close{position:absolute;right:8px;top:8px;width:24px;height:24px}#scope-copy{position:absolute;left:24px;top:48px;width:336px;height:80px}#scope-list{position:absolute;left:24px;top:144px;width:336px;height:208px}#scope-list>button{display:block;box-sizing:border-box;width:336px;height:52px}#scope-list img{width:16px;height:16px}#scope-guest{position:absolute;left:24px;top:376px;width:336px;height:24px}</style><button id="scope-close"><span>×</span></button><div id="scope-copy"><span>Account</span><h2>Fixture chooser</h2><p>Choose a fixture provider</p></div><div id="scope-list">'+[0,1,2,3].map(i=>'<button><span>Provider '+i+'</span><span>Detail</span>'+([1,2].includes(i)?'<img>':'')+'</button>').join('')+'</div><button id="scope-guest">Continue</button>';
      document.body.append(root);
      if(1+root.querySelectorAll('*').length!==24)throw new Error('fixture count');
    });
    if(addressing)await page.evaluate(()=>{
      const style=document.createElement('style');style.textContent='#address-fixture{position:fixed;top:10px;left:10px}#address-fixture>ul{box-sizing:border-box;width:240px;height:160px;margin:0;padding:0;list-style:none;position:absolute;top:0;left:0}#address-fixture>ul>li{height:40px}';document.head.append(style);
      const parent=document.createElement('section');parent.id='address-fixture';
      for(let i=0;i<6;i++){
        parent.append(document.createTextNode(' '),document.createComment('ordinal ignores this'),document.createElement('p'));
        const ul=document.createElement('ul');
        if(i===5)for(let n=0;n<4;n++){const li=document.createElement('li'),strong=document.createElement('strong');strong.textContent='Fixture item '+n;li.append(strong);ul.append(li);}
        parent.append(ul);
      }document.body.append(parent);
    });
    const state=()=>page.evaluate(()=>({focus:document.activeElement?.id,scroll:[scrollX,scrollY],state:window.f01.checkpoint()}));
    const before=await state();const good=await invoke(ws,page.url(),whole?'#f01':selected,4);assert(good.stderr.startsWith('observe_diagnostic exit=4 '));assert(good.stderr.includes('terminal=completed'));
    for(const [name,amount] of [['width',whole?360:width],['height',whole?123:height]]){const line=good.stdout.split('\n').find(x=>x.startsWith(name+': '));assert(line);const result=JSON.parse(line.slice(name.length+2));assert.equal(result.status,'known');assert.equal(result.measurement.value.value.amount,amount);assert.equal(result.measurement.space.units,'css_px');}
    assert.deepEqual(await state(),before);
    if(process.argv.includes('--run-export-authorized')||addressing||whole){
      const temp=await fs.mkdtemp(path.join(os.tmpdir(),'uib-json-f01-')),proxy=path.join(temp,'proxy.cjs'),original=path.join(temp,'original.json');
      const saved=path.join(temp,'observation.json'),metadata=path.join(temp,'metadata.json'),query=path.join(temp,'query.json'),destination=path.join(temp,'package');
      try{
        // Capture and forward actual CLI bytes verbatim; never construct source facts.
        await fs.writeFile(proxy,'#!'+process.execPath+'\nconst fs=require("node:fs"),{spawnSync}=require("node:child_process");\nconst r=spawnSync('+JSON.stringify(path.join(bundle,'uiblueprint'))+',process.argv.slice(2),{timeout:7000,maxBuffer:1048576});\nif(process.argv[2]==="observe"&&r.stdout)fs.writeFileSync('+JSON.stringify(original)+',r.stdout,{flag:"wx",mode:0o600});\nif(r.stdout)process.stdout.write(r.stdout);if(r.stderr)process.stderr.write(r.stderr);process.exitCode=r.status??1;\n',{mode:0o700});
        const machine=await invoke(ws,page.url(),selected,4,['--json','--cli',proxy,'--worker',workers[0]]);
        const raw=await fs.readFile(original);assert(machine.bytes.equals(raw));assert.equal(machine.stdout.trim().split('\n').length,1);
        assert(machine.stderr.startsWith('observe_diagnostic exit=4 '));assert(machine.stderr.includes('terminal=completed'));
        await fs.writeFile(saved,machine.bytes,{mode:0o600,flag:'wx'});
        const response=JSON.parse(raw),snapshot=response.artifact.data.result.data;
        assert.equal(response.artifact.kind,'channel_response');assert.equal(response.artifact.data.result.status,'observed');
        assert.deepEqual(snapshot.context.fields,['layout_bounds','hit_region','visible_region']);
        const root=snapshot.nodes[0],bounds=root.properties.find(p=>p.field==='layout_bounds').state.value.value;
        assert.equal(bounds.coordinate_space.units,'css_px');
        const annotations={metadata:{document_id:'F01-EXACT-HANDOFF',revision:'1',title:'Owned F01 fixture',audience:'Developer',language:'en',date:'2026-10-13',owner:'USE-WEB-EXPORT1',retention:'Transient verification; removed after assertions',specification_refs:['UIB.DRAWING@1.2'],approval:{status:'draft',named_record:null},page_format:'A3 proportions',output_size:'3840 x 2160 output pixels'},state:'Owned fixture initial state',scope:'Selected '+selected+' component',environment:'Headless Chromium 145; viewport 800 x 600 CSS px; DPR 1',safe_source_reference:'Owned F01 fixture',not_depicted:['Outside selected component; unrequested semantic fields'],public_text_fields:[]};
        await fs.writeFile(metadata,JSON.stringify(annotations),{mode:0o600,flag:'wx'});
        const run=(exe,a)=>execFileSync(exe,a,{cwd:'/',timeout:10000,maxBuffer:4194304,stdio:['ignore','pipe','pipe']});
        run(path.join(bundle,'uiblueprint-validate'),['--max-bytes','131072',saved]);
        if(addressing||whole)for(const [operation,amount] of [['width',width],['height',height]]){
          const space=bounds.coordinate_space,key=root.key;
          await fs.writeFile(query,JSON.stringify({schema_version:'0.2.0',artifact:{kind:'geometry_query',data:{id:operation,scope_id:snapshot.context.scope_id,targets:[key],operation,anchors:[{element:key,frame_kind:'layout_bounds',coordinate_space:space,fraction:0,axis:'x'}],quantity_kind:'length',units:space.units,applies_when:{platform:null,input_mode:null,text_scale:null}}}}),{mode:0o600});
          const measured=JSON.parse(run(path.join(bundle,'uiblueprint'),['measure','--snapshot',saved,'--query',query,'--space',space.id,'--max-input-bytes','131072','--max-output-bytes','131072','--json']));
          assert.deepEqual(measured.artifact.data.snapshot,snapshot);assert.equal(measured.artifact.data.result.measurement.value.value.amount,amount);
        }
        const receipt=JSON.parse(run(path.join(bundle,'uiblueprint'),['imagegen-prompt','--snapshot',saved,'--metadata',metadata,'--out',destination,'--max-input-bytes','2000000','--max-output-bytes','4000000','--max-components','32','--max-views','1','--components-per-detail','32','--json']));
        assert((await fs.readFile(saved)).equals(raw));
        const files=(await fs.readdir(destination)).sort();assert.deepEqual(files,['dimensions.json','drawing-brief.md','manifest.json','prompt.txt','scene.json','sheets.json']);
        const scene=JSON.parse(await fs.readFile(path.join(destination,'scene.json'),'utf8'));
        assert.equal(scene.views[0].components.length,snapshot.nodes.length);
        const serialized=JSON.stringify(scene);assert(serialized.includes('css_px'));assert(!serialized.includes('PRIVATE_BOOTSTRAP_CANARY'));
        assert.deepEqual(await state(),before);
        const refused=await invoke(ws,page.url(),'#absent',4,['--json']);assert(!refused.stdout);assert.equal(refused.stderr.trim(),'component_unresolved');
        if(whole){
          assert.equal(snapshot.nodes.length,24);assert.equal(scene.views[0].components.length,24);
          const roles=snapshot.nodes.map(n=>n.native_role.value?.value);
          assert.deepEqual(roles.reduce((a,v)=>(a[v]=(a[v]??0)+1,a),{}),{DIV:3,STYLE:1,BUTTON:6,SPAN:10,H2:1,P:1,IMG:2});
          for(const n of snapshot.nodes)assert.deepEqual(n.properties.map(p=>p.field),['layout_bounds','hit_region','visible_region']);
          assert.equal(snapshot.nodes[1].properties[0].state.availability,'unknown','STYLE preserved with unknown layout');
          assert.equal(snapshot.coverage.status,'partial');
          for(const [left,right,expected] of [[4,8,16],[8,23,24]]){
            const space=bounds.coordinate_space,keys=[snapshot.nodes[left].key,snapshot.nodes[right].key];
            await fs.writeFile(query,JSON.stringify({schema_version:'0.2.0',artifact:{kind:'geometry_query',data:{id:'gap',scope_id:snapshot.context.scope_id,targets:keys,operation:'gap',anchors:keys.map((element,i)=>({element,frame_kind:'layout_bounds',coordinate_space:space,fraction:i===0?1:0,axis:'y'})),quantity_kind:'length',units:space.units,applies_when:{platform:null,input_mode:null,text_scale:null}}}}));
            const gap=JSON.parse(run(path.join(bundle,'uiblueprint'),['measure','--snapshot',saved,'--query',query,'--space',space.id,'--max-input-bytes','131072','--max-output-bytes','131072','--json']));
            assert.deepEqual(gap.artifact.data.snapshot,snapshot);assert.equal(gap.artifact.data.result.measurement.value.value.amount,expected);
          }
          // Selection-boundary fixture deliberately reports unavailable layout;
          // this tests32 admitted Elements, not a promise32 visible boxes fit bytes/time.
          await page.evaluate(()=>{const root=document.createElement('div');root.id='scope-boundary';root.style.display='none';for(let i=0;i<31;i++)root.append(document.createElement('span'));document.body.append(root);});
          const edge=await invoke(ws,page.url(),'#scope-boundary',4,['--json']);const edgeSnapshot=JSON.parse(edge.stdout).artifact.data.result.data;assert.equal(edgeSnapshot.nodes.length,32);
          await page.evaluate(()=>document.getElementById('scope-boundary').append(document.createElement('span')));
          const beyond=await invoke(ws,page.url(),'#scope-boundary',2,['--json']);assert(!beyond.stdout);assert(beyond.stderr.includes('cause=selection_incomplete'));assert(beyond.stderr.includes('committed=0'));assert(beyond.stderr.includes('worker_cleanup=confirmed'));
          console.log('PASS scope:24 nodes including STYLE, same-snapshot gaps16/24 CSSpx;32 admitted,33 selection refusal with no publication');
        }
        if(addressing){
          assert.equal(snapshot.nodes.length,9);assert.equal(scene.views[0].components.length,9);
          assert(snapshot.nodes.every(n=>['UL','LI','STRONG'].includes(n.native_role.value?.value)));
          const both=await page.evaluate(()=>{const extra=document.createElement('section');extra.id='address-other';for(let i=0;i<6;i++)extra.append(document.createElement('ul'));document.body.append(extra);return true;});assert(both);
          const ambiguous=await invoke(ws,page.url(),selected,4,['--json']);assert(!ambiguous.stdout);assert.equal(ambiguous.stderr.trim(),'ambiguous_component');
          await page.evaluate(()=>document.getElementById('address-other').remove());
          for(const mutation of ['insert','remove','reorder','remount','reparent']){
            // Restore only our synthetic list fixture before each fresh binding.
            await page.evaluate(()=>{const p=document.getElementById('address-fixture');p.replaceChildren();for(let i=0;i<6;i++)p.append(document.createElement('ul'));});
            const held=await helper.openBootstrap(helper.parseFirstUse(['--connection',ws,'--url',page.url(),'--component',selected]));
            try{
              await page.evaluate(m=>{const p=document.getElementById('address-fixture'),target=p.lastElementChild;
                if(m==='insert')p.prepend(document.createElement('ul'));
                if(m==='remove')p.firstElementChild.remove();
                if(m==='reorder')p.prepend(target);
                if(m==='remount')target.replaceWith(target.cloneNode(true));
                if(m==='reparent'){const div=document.createElement('div');p.append(div);div.append(target);}
              },mutation);
              await assert.rejects(()=>held.revalidate(),e=>['stale_target','component_unresolved'].includes(e.code),mutation);
            }finally{await held.close();}
          }
          // Compare expanded-name bookkeeping with native CSS only as a fixture oracle.
          const parity=await page.evaluate(({source})=>{
            const parent=document.createElement('div');document.body.append(parent);
            const html='http://www.w3.org/1999/xhtml',svg='http://www.w3.org/2000/svg';
            for(const [ns,name] of [[html,'ul'],[svg,'ul'],[html,'UL'],[html,'ul']])parent.append(document.createElementNS(ns,name),document.createTextNode('x'),document.createComment('x'));
            const fn=(0,eval)('('+source+')'),items=Array.from(parent.children),results=[];
            items.forEach((node,i)=>node.id='ordinal-parity-'+i);
            for(const node of items)for(const n of [1,2]){
              const selector='#'+node.id,expected=node.matches(selector+':nth-of-type('+n+')');
              const got=fn.call(document,{selector,nthOfType:n,maxVisited:2048,remainingMs:1000});results.push(expected?(got.root===node):(got.status==='missing'));
            }
            parent.remove();return results;
          },{source:helper.selectComponent.toString()});assert(parity.every(Boolean));
        }
        console.log(JSON.stringify({status:'passed',kind:'owned-F01-exact-observe-to-export',source_revision:manifest.source_revision,bytes:raw.length,sha256:crypto.createHash('sha256').update(raw).digest('hex'),nodes:snapshot.nodes.length,export_receipt:receipt,files,default_geometry:`${width}x${height} css_px`,partial_exit:4,original_bytes_equal:true,state_unchanged:true}));
      }finally{
        try{for(const f of await fs.readdir(destination))await fs.unlink(path.join(destination,f));await fs.rmdir(destination);}catch(e){if(e.code!=='ENOENT')throw e;}
        for(const f of [proxy,original,saved,metadata,query]){try{await fs.unlink(f);}catch(e){if(e.code!=='ENOENT')throw e;}}
        await fs.rmdir(temp);await assert.rejects(fs.stat(temp),e=>e.code==='ENOENT');
      }
      return;
    }
    for(const [pageUrl,selector,code,extra] of [[page.url().replace('#first','#different'),'#f01','target_unresolved',[]],[page.url(),'#absent','component_unresolved',[]],[page.url(),'button','ambiguous_component',[]],[page.url(),'#child','unsupported_scope',[]],[page.url(),'#f01','selection_limit',['--max-visited-nodes','1']]]){
      const r=await invoke(ws,pageUrl,selector,code==='selection_limit'?2:4,extra);assert(!r.stdout);assert.equal(r.stderr.trim(),code);assert.deepEqual(await state(),before);
    }
    const duplicate=await context.newPage();await duplicate.goto(page.url());const ambiguous=await invoke(ws,page.url(),'#f01',4);assert(!ambiguous.stdout);assert.equal(ambiguous.stderr.trim(),'ambiguous_target');await duplicate.close();
    await page.evaluate(()=>{const input=document.getElementById('draft');input.type='password';input.value='PRIVATE_BOOTSTRAP_CANARY';});
    const privateBefore=await page.inputValue('#draft'),privateState=await state();
    const privateRead=await invoke(ws,page.url(),'#draft',4);assert(privateRead.stderr.startsWith('observe_diagnostic exit=4 '));assert(privateRead.stderr.includes('terminal=completed'));
    assert.equal(await page.inputValue('#draft'),privateBefore);assert.deepEqual(await state(),privateState);
    await page.evaluate(()=>{const host=document.createElement('div');host.id='closed-host';document.body.append(host);host.attachShadow({mode:'closed'});});
    const closed=await invoke(ws,page.url(),'#closed-host',4);assert(!closed.stdout);assert.equal(closed.stderr.trim(),'unsupported_scope');
    await page.evaluate(()=>{const host=document.createElement('div');host.id='new-boundary';document.body.append(host);});
    const boundary=await helper.openBootstrap(helper.parseFirstUse(['--connection',ws,'--url',page.url(),'--component','#new-boundary']));
    await page.evaluate(()=>document.getElementById('new-boundary').attachShadow({mode:'closed'}));
    await rejects(()=>boundary.revalidate(),'unsupported_scope');await boundary.close();
    await page.evaluate(()=>{const host=document.createElement('div');host.id='remount-boundary';document.body.append(host);});
    const retired=await helper.openBootstrap(helper.parseFirstUse(['--connection',ws,'--url',page.url(),'--component','#remount-boundary']));
    await page.evaluate(()=>{const old=document.getElementById('remount-boundary'),host=document.createElement('div');host.id=old.id;old.replaceWith(host);host.attachShadow({mode:'closed'});});
    await rejects(()=>retired.revalidate(),'stale_target');await retired.close();
    const reparent=await helper.openBootstrap(helper.parseFirstUse(['--connection',ws,'--url',page.url(),'--component','#f01']));
    await page.evaluate(()=>{const root=document.getElementById('f01'),wrapper=document.createElement('section');root.parentNode.insertBefore(wrapper,root);wrapper.append(root);});
    await rejects(()=>reparent.revalidate(),'stale_target');await reparent.close();
    // Real objects: remount after preparation must never reacquire by selector.
    const prepared=await helper.openBootstrap(helper.parseFirstUse(['--connection',ws,'--url',page.url(),'--component','#f01']));
    await page.evaluate(()=>{const old=document.getElementById('f01');old.replaceWith(old.cloneNode(true));});
    await rejects(()=>prepared.revalidate(),'stale_target');await prepared.close();
    const navigation=await helper.openBootstrap(helper.parseFirstUse(['--connection',ws,'--url',page.url(),'--component','#f01']));
    await page.reload();await rejects(()=>navigation.revalidate(),'stale_target');await navigation.close();
    console.log(JSON.stringify({status:'passed',kind:'owned-F01-only',source_revision:manifest.source_revision,results,geometry:'360x123 css_px; partial',remount_loader_parent_refused:true,state_unchanged_during_reads:true}));
  }finally{
    let cleanupFailed=false;
    if(child&&child.exitCode===null&&child.signalCode===null)child.kill('SIGKILL');
    for(const close of [()=>context?.close(),()=>browser?.close(),async()=>{if(server){try{await wait(server.close(),4000);}catch{await wait(server.kill(),3000);}}},()=>fixture?.close()]){
      try{await wait(Promise.resolve().then(close),7000);}catch{cleanupFailed=true;}
    }
    if(profile){try{await assert.rejects(fs.stat(profile),e=>e.code==='ENOENT');}catch{cleanupFailed=true;}}
    assert(!cleanupFailed,'owned cleanup unconfirmed');
    console.log('PASS owned browser/context/profile/server cleanup; no source application operated');
  }
}
(async()=>{
  if(process.argv.includes('--check-scope-only'))await scopeOffline();
  else if(process.argv.includes('--check'))await offline();
  else if(process.argv.includes('--run-authorized')||process.argv.includes('--run-export-authorized')||process.argv.includes('--run-address-authorized')||process.argv.includes('--run-scope-authorized'))await live();
  else throw new Error('choose --check or explicitly authorized --run-authorized');
})().catch(error=>{console.error(error);process.exitCode=1;});
