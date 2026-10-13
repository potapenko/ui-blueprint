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

async function offline(){
  const parsed=helper.parseFirstUse(args());assert.equal(parsed.selector,'#f01');assert.equal(parsed.maxVisited,2048);
  for(const selector of ['div.card','[role="dialog"][aria-label="Режиссёр"]','*','#f01','.card[data-testid="x"]'])assert.equal(helper.parseFirstUse(args(selector)).selector,selector);
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
  class Document{};class Element{constructor(key){this.key=key;this.localName='div';this.isConnected=true;}matches(selector){return selector===this.key||selector==='*';}}
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
      else if(q.method==='DOM.describeNode')result={node:{nodeType:1,backendNodeId:42}};
      else if(q.method==='Runtime.releaseObjectGroup')result={};
      else throw new Error('unexpected_method');
      return {id:q.id,result};
    };
    for(const [which,expected] of [['missing','target_unresolved'],['ambiguous','ambiguous_target'],['stale-loader','stale_target'],['malformed-status','cdp_invalid_reply']]){
      mode=which;frames=0;await rejects(()=>helper.openBootstrap(parsed),expected);assert(FakeSocket.sockets.every(s=>s.didClose));
    }
    mode='ok';const prepared=await helper.openBootstrap(parsed);assert.equal(prepared.args.root,42);assert.equal(prepared.binding.document,1);
    mode='remount';await rejects(()=>prepared.revalidate(),'stale_target');await prepared.close();assert(FakeSocket.sockets.every(s=>s.didClose));
  }finally{global.WebSocket=old.WebSocket;}
  // The overall preparation deadline also bounds a stalled Observe child.
  const temp=await fs.mkdtemp(path.join(os.tmpdir(),'uib-bootstrap-deadline-'));
  const delayed=path.join(temp,'delayed-cli.cjs');
  try{
    await fs.writeFile(delayed,'#!/usr/bin/env node\nsetTimeout(()=>{},2000);\n',{mode:0o700});
    const ownedDirs=[],make=fs.mkdtemp;fs.mkdtemp=async(...args)=>{const dir=await make(...args);ownedDirs.push(dir);return dir;};
    try{
      const owned={cli:delayed,worker:'/unused',target:'target',endpoint:'ws://127.0.0.1:9222/devtools/page/target',root:42};
      await rejects(()=>helper.example(owned,{binding:{surface:{id:'frame',generation:'loader'},document:1},remaining:()=>20,revalidate:()=>{throw new Error('must not publish');}}),'bootstrap_deadline');
      await fs.writeFile(delayed,'#!/usr/bin/env node\nprocess.exit(0);\n');
      await assert.rejects(()=>helper.example(owned,{binding:{surface:{id:'frame',generation:'loader'},document:1},remaining:()=>1000}),e=>e.code==='observe_refused_exit_0'&&e.exit===1);
    }finally{fs.mkdtemp=make;}
    assert.equal(ownedDirs.length,2);for(const dir of ownedDirs)await assert.rejects(fs.stat(dir),e=>e.code==='ENOENT');
  }finally{await fs.unlink(delayed);await fs.rmdir(temp);}
  console.log('PASS offline: input/URL/selector, unique bounded walk, remount, CDP bytes/calls/events/deadline/cleanup, source-derived binding');
}

async function live(){
  assert(process.env.UIB_WEB_LIVE_ALLOW==='1','explicit own-fixture activation required');
  const bundle=process.env.UIB_WEB_FIRST_USE_BUNDLE;assert(bundle&&path.isAbsolute(bundle));
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
    results.push({component,exit:code,stdout_bytes:Buffer.byteLength(stdout),stderr:stderr.trim()});return {stdout,stderr};
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
    const state=()=>page.evaluate(()=>({focus:document.activeElement?.id,scroll:[scrollX,scrollY],state:window.f01.checkpoint()}));
    const before=await state();const good=await invoke(ws,page.url(),'#f01',4);assert.equal(good.stderr,'');
    for(const [name,amount] of [['width',360],['height',123]]){const line=good.stdout.split('\n').find(x=>x.startsWith(name+': '));assert(line);const result=JSON.parse(line.slice(name.length+2));assert.equal(result.status,'known');assert.equal(result.measurement.value.value.amount,amount);assert.equal(result.measurement.space.units,'css_px');}
    assert.deepEqual(await state(),before);
    for(const [pageUrl,selector,code,extra] of [[page.url().replace('#first','#different'),'#f01','target_unresolved',[]],[page.url(),'#absent','component_unresolved',[]],[page.url(),'button','ambiguous_component',[]],[page.url(),'#child','unsupported_scope',[]],[page.url(),'#f01','selection_limit',['--max-visited-nodes','1']]]){
      const r=await invoke(ws,pageUrl,selector,code==='selection_limit'?2:4,extra);assert(!r.stdout);assert.equal(r.stderr.trim(),code);assert.deepEqual(await state(),before);
    }
    const duplicate=await context.newPage();await duplicate.goto(page.url());const ambiguous=await invoke(ws,page.url(),'#f01',4);assert(!ambiguous.stdout);assert.equal(ambiguous.stderr.trim(),'ambiguous_target');await duplicate.close();
    await page.evaluate(()=>{const input=document.getElementById('draft');input.type='password';input.value='PRIVATE_BOOTSTRAP_CANARY';});
    const privateBefore=await page.inputValue('#draft'),privateState=await state();
    const privateRead=await invoke(ws,page.url(),'#draft',4);assert.equal(privateRead.stderr,'');
    assert.equal(await page.inputValue('#draft'),privateBefore);assert.deepEqual(await state(),privateState);
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
  if(process.argv.includes('--check'))await offline();
  else if(process.argv.includes('--run-authorized'))await live();
  else throw new Error('choose --check or explicitly authorized --run-authorized');
})().catch(error=>{console.error(error);process.exitCode=1;});
