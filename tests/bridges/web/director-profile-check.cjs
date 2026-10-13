// Own static three-option widget only. No source-app access or timing acceptance.
const assert=require('node:assert/strict'),fs=require('node:fs/promises'),path=require('node:path'),os=require('node:os');
const {spawn}=require('node:child_process'),crypto=require('node:crypto');
const helper=require('./geometry.cjs');
async function main(){
 assert(process.argv.includes('--run-authorized')&&process.env.UIB_WEB_LIVE_ALLOW==='1');
 const bundle=process.env.UIB_WEB_FIRST_USE_BUNDLE;assert(bundle&&path.isAbsolute(bundle));
 const manifest=JSON.parse(await fs.readFile(path.join(bundle,'distribution-manifest.json'),'utf8'));
 for(const name of ['uiblueprint','session-worker','web-geometry.cjs'])assert.equal(crypto.createHash('sha256').update(await fs.readFile(path.join(bundle,name))).digest('hex'),manifest.files[name].sha256);
 assert((await fs.readFile(path.join(bundle,'web-geometry.cjs'))).equals(await fs.readFile(path.join(__dirname,'geometry.cjs'))));
 const {prepare}=require('./fixture-host.cjs'),setup=prepare(),{start}=require('../../../fixtures/web/server.cjs');
 const temp=await fs.mkdtemp(path.join(os.tmpdir(),'uib-widget-profile-'));let fixture,server,browser,context,profile,child;const owners=[];const results=[];
 const bounded=(promise,ms)=>{let t;return Promise.race([promise,new Promise((_,reject)=>{t=setTimeout(()=>reject(new Error('test_timeout')),ms)})]).finally(()=>clearTimeout(t))};
 async function command(exe,args){
  child=spawn(exe,args,{cwd:'/',stdio:['ignore','pipe','pipe']});let bytes=0;const out=[],err=[];
  for(const [stream,list] of [[child.stdout,out],[child.stderr,err]])stream.on('data',data=>{bytes+=data.length;if(bytes>300000)child.kill('SIGKILL');else list.push(data)});
  const code=await bounded(new Promise((resolve,reject)=>{child.once('error',reject);child.once('close',(c,s)=>s?reject(new Error('child_signal')):resolve(c))}),15000);
  return {code,out:Buffer.concat(out),err:Buffer.concat(err)};
 }
 try{
  fixture=await start();server=await setup.chromium.launchServer({headless:true,timeout:8000,args:['--remote-debugging-port=0','--remote-debugging-address=127.0.0.1']});
  profile=server.process().spawnargs.find(x=>x.startsWith('--user-data-dir=')).slice('--user-data-dir='.length);assert(path.basename(profile).startsWith('playwright_chromiumdev_profile-'));
  let ws;for(let i=0;i<100;i++){try{const lines=(await fs.readFile(path.join(profile,'DevToolsActivePort'),'utf8')).split('\n');ws=`ws://127.0.0.1:${lines[0]}${lines[1]}`;break}catch(e){if(e.code!=='ENOENT')throw e;await new Promise(r=>setTimeout(r,20))}}
  assert(ws);browser=await setup.chromium.connect(server.wsEndpoint(),{timeout:3000});context=await browser.newContext({viewport:{width:1280,height:900},deviceScaleFactor:2});
  await context.route('**/*',r=>new URL(r.request().url()).origin===fixture.url?r.continue():r.abort());
  const page=await context.newPage();page.setDefaultTimeout(2000);page.setDefaultNavigationTimeout(3000);await page.goto(fixture.url+'/?generation=1');await page.waitForFunction(()=>!!window.f01);
  await page.evaluate(()=>{const p=document.createElement('div');p.id='widget-popup';p.style.cssText='position:fixed;left:50px;top:300px;width:320px;height:200px;background:white;z-index:100';
   p.innerHTML='<div>Public title</div><label>Director</label><input id="widget-input" value="spiel" style="position:absolute;left:20px;top:32px;box-sizing:border-box;width:180px;height:26px"><div id="widget-options" style="position:absolute;left:20px;top:70px;width:280px;height:96px">'+['Alpha','Beta','Gamma'].map(x=>'<button style="position:static;display:block;box-sizing:border-box;width:280px;height:32px;border:0;padding:0;margin:0"><span>'+x+'</span><span>12</span></button>').join('')+'</div>';document.body.append(p)});
  const state=()=>page.evaluate(()=>({html:document.getElementById('widget-popup').outerHTML,input:document.getElementById('widget-input').value,focus:document.activeElement?.id,scroll:[scrollX,scrollY]}));
  const before=await state();
  for(const [selector,nodes,width,height] of [['#widget-popup',14,320,200],['#widget-options',10,280,96],['#widget-input',1,180,26]]){
   const r=await command(process.execPath,[path.join(bundle,'web-geometry.cjs'),'--connection',ws,'--url',page.url(),'--component',selector]);assert.equal(r.code,4);const text=r.out.toString();assert(r.err.toString().includes('terminal=completed'));assert(r.err.toString().includes('worker_cleanup=confirmed'));
   const parts=text.split('\n').filter(l=>/^web\.dom:\d+ layout_bounds /.test(l));assert.equal(parts.length,nodes);
   for(const [name,amount] of [['width',width],['height',height]]){const line=text.split('\n').find(l=>l.startsWith(name+': '));const v=JSON.parse(line.slice(name.length+2));assert.equal(v.status,'known');assert.equal(v.measurement.value.value.amount,amount);assert.equal(v.measurement.space.units,'css_px')}
   if(nodes>=10){const rows=parts.filter(l=>l.includes('"value":"BUTTON"')).map(l=>JSON.parse(l.slice(l.indexOf('{'),l.indexOf(' css_px '))).value);assert.equal(rows.length,3);assert.deepEqual(rows.map(r=>[r.x,r.y,r.width,r.height]),[[70,370,280,32],[70,402,280,32],[70,434,280,32]])}
   assert(text.includes('HitRegion')&&text.includes('VisibleRegion'));assert.deepEqual(await state(),before);results.push({selector,nodes,width,height,exit:4,all_fields_retained:true});
  }
  // Negative old output ceiling, using the SAME input builder and real binding.
  const prep=await helper.openBootstrap(helper.parseFirstUse(['--connection',ws,'--url',page.url(),'--component','#widget-popup']));owners.push(prep);
  for(const [bytes,label] of [[65536,'old-output-cap'],[131072,'complete-output']]){
   const input=helper.inputs(prep.args,prep.binding,bytes,1000);const c=path.join(temp,label+'-connection.json'),r=path.join(temp,label+'-request.json');await fs.writeFile(c,JSON.stringify(input.connection),{mode:0o600});await fs.writeFile(r,JSON.stringify(input.request),{mode:0o600});
   const actual=await command(path.join(bundle,'uiblueprint'),['observe','--connection',c,'--request',r,'--worker',path.join(bundle,'session-worker'),'--max-input-bytes','131072','--max-output-bytes',String(bytes),'--diagnostics-json','--max-diagnostic-bytes','4096']);
   const d=helper.parseObserveDiagnostic(actual.err,actual.code);assert.equal(d.worker_cleanup,'confirmed');
   if(bytes===65536){assert.equal(actual.code,2);assert(!actual.out.length);assert.equal(d.terminal_error,'resource_limit');assert.equal(d.host_diagnostic?.cause,'collector_limit')}
   else{assert.equal(actual.code,4);assert(actual.out.length>65536&&actual.out.length<131072);const s=JSON.parse(actual.out).artifact.data.result.data;assert.equal(s.nodes.length,14);assert.deepEqual(s.context.fields,['layout_bounds','hit_region','visible_region']);results.push({canonical_bytes:actual.out.length,all_nodes:14,diagnostic:d})}
   await fs.unlink(c);await fs.unlink(r);
  }
  await prep.revalidate();await prep.close();
  // Node quota is still16: no prefix pruning to make a larger root succeed.
  await page.evaluate(()=>{for(let i=0;i<3;i++){const e=document.createElement('span');e.textContent='public';document.getElementById('widget-popup').append(e)}});
  const larger=await command(process.execPath,[path.join(bundle,'web-geometry.cjs'),'--connection',ws,'--url',page.url(),'--component','#widget-popup']);assert.equal(larger.code,2);assert(!larger.out.length);assert(larger.err.toString().includes('cause=selection_incomplete'));
  console.log(JSON.stringify({status:'passed',kind:'own-3-option-widget-not-real-Director',source_revision:manifest.source_revision,results,node17_refused:true,no_pruning_no_retry:true}));
 }finally{
  let bad=false;if(child&&child.exitCode===null&&child.signalCode===null)child.kill('SIGKILL');
  for(const owner of owners){try{await owner.close()}catch{bad=true}}
  for(const stop of [()=>context?.close(),()=>browser?.close(),async()=>{if(server){try{await bounded(server.close(),4000)}catch{await bounded(server.kill(),3000)}}},()=>fixture?.close()]){try{await bounded(Promise.resolve().then(stop),7000)}catch{bad=true}}
  if(profile){try{await assert.rejects(fs.stat(profile),e=>e.code==='ENOENT')}catch{bad=true}}
  for(const name of await fs.readdir(temp)){assert(/^(old-output-cap|complete-output)-(connection|request)\.json$/.test(name));await fs.unlink(path.join(temp,name))}
  await fs.rmdir(temp);assert(!bad,'owned cleanup unconfirmed');console.log('PASS own widget/context/browser/profile/server/temp cleanup');
 }
}
main().catch(error=>{console.error(error);process.exitCode=1});
