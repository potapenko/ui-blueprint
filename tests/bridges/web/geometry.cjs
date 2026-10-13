// Optional shipped first-use caller: authorized CDP + URL/component -> Rust CLI.
// Bounded selection only; no browser launch/navigation, app input or JS geometry.
const fs = require('node:fs/promises');
const path = require('node:path');
const os = require('node:os');
const crypto = require('node:crypto');
const {spawn} = require('node:child_process');
const assert = require('node:assert/strict');

const FRAME=65536;
const HELP=`UI Blueprint Web geometry helper (optional Node.js 24+ caller)

URL + selected component, no manual JSON or runtime IDs:
  node "$BUNDLE/web-geometry.cjs" --connection "$CDP_BROWSER_WS" \\
    --url 'http://127.0.0.1:PORT/page#route' --component '#selected-component'

--connection  Already authorized ws://127.0.0.1:PORT/devtools/browser/ID
              (numeric IPv6 loopback also supported). No default/guessed port.
--url         Exact existing http(s) URL; query and SPA fragment participate.
              Multiple matching pages refuse; this never navigates a page.
--component   One compound selector: optional tag/*, #id, .class, [attr],
              [attr="value"]. No combinators, lists, escapes or pseudo selectors.
              Unique root-document light-DOM match only; no frame/shadow descent.
--max-visited-nodes  Optional lower selection bound, 1..2048 (default 2048).
--cli / --worker     Optional matching absolute binaries (defaults: siblings).

Core and low-level guarded Web CLI remain Node-free. This caller does not discover
browsers, scan profiles, launch/reconfigure, navigate/focus, or change applications.
An already authorized connection is required; URL alone is not authority.
Selection is bounded metadata, not JS geometry or a manual snapshot adapter.
Real Rust Observe -> Inspect -> Measure(width,height) supplies facts and coverage.
Preparation uses a 10s deadline; URL mode collects 32 nodes/depth8/128KiB/1000ms
(collector cap16 nodes). Legacy mode keeps 64KiB/250ms. No automatic quota retry.
A matching diagnostics-capable CLI is required; safe enum/count reasons go to stderr.

Legacy exact-ID mode is unchanged (cannot mix modes):
  --cli ABSOLUTE_CLI --worker ABSOLUTE_WORKER
  --endpoint ws://127.0.0.1:PORT/devtools/page/TARGET --target-id TARGET
  --frame-id FRAME --loader-id LOADER --document-backend-id DOCUMENT
  --root-backend-id ELEMENT

Exit 0 complete; 4 partial/unresolved/ambiguous/stale/deadline; 2 invalid or bound;
1 IO/cleanup failure; underlying Rust exits preserved. Inspect partial results.
No automatic retry or guessed identity. --help/-h alone performs no connection,
executable check or temporary-file creation. See WEB-FIRST-USE.md.
`;
function failure(code,exit=4){return Object.assign(new Error(code),{code,exit});}
function bounded(promise,ms,code,exit=1){
  let timer;return Promise.race([promise,new Promise((_,reject)=>{timer=setTimeout(()=>reject(failure(code,exit)),ms);})]).finally(()=>clearTimeout(timer));
}
async function argumentsFor(argv){
  const allowed=new Set(['--cli','--worker','--endpoint','--target-id','--frame-id','--loader-id','--document-backend-id','--root-backend-id']);
  const args={};
  for(let i=0;i<argv.length;i+=2){
    const key=argv[i];if(!allowed.has(key)||key in args||!argv[i+1])throw failure('invalid_arguments',2);
    args[key]=argv[i+1];
  }
  if(Object.keys(args).length!==allowed.size)throw failure('invalid_arguments',2);
  for(const key of ['--cli','--worker']){
    if(!path.isAbsolute(args[key])||!(await fs.stat(args[key])).isFile())throw failure('invalid_executable',2);
    await fs.access(args[key],require('node:fs').constants.X_OK);
  }
  let url;try{url=new URL(args['--endpoint']);}catch{throw failure('invalid_endpoint',2);}
  const target=args['--target-id'];
  if(!/^[A-Za-z0-9_-]{1,256}$/.test(target)||url.protocol!=='ws:'||!['127.0.0.1','[::1]'].includes(url.hostname)
    ||url.href.length>1024||!url.port||url.username||url.password||url.search||url.hash||url.pathname!==`/devtools/page/${target}`)
    throw failure('invalid_target_endpoint',2);
  for(const key of ['--root-backend-id','--document-backend-id']){
    if(!/^[1-9][0-9]{0,9}$/.test(args[key])||Number(args[key])>2147483647)throw failure('invalid_backend_identity',2);
  }
  for(const key of ['--frame-id','--loader-id'])if(!/^[A-Za-z0-9_-]{1,256}$/.test(args[key]))throw failure('invalid_document_identity',2);
  return {cli:args['--cli'],worker:args['--worker'],endpoint:url.href,target,root:Number(args['--root-backend-id']),
    document:Number(args['--document-backend-id']),surface:{id:args['--frame-id'],generation:args['--loader-id']}};
}

// Fixed, sequential metadata reads only. Node's WebSocket implementation is
// tooling; the product's guarded transport/collection remains inside the worker.
async function metadata(args){
  const socket=new WebSocket(args.endpoint);let sequence=0,pending=null,wireBytes=0,events=0;
  const closed=new Promise(resolve=>socket.addEventListener('close',resolve,{once:true}));
  socket.addEventListener('error',()=>pending?.reject(failure('cdp_connection_failed')));
  socket.addEventListener('close',()=>pending?.reject(failure('cdp_connection_closed')));
  socket.addEventListener('message',event=>{
    if(typeof event.data!=='string'){pending?.reject(failure('cdp_nontext_reply'));socket.close();return;}
    const size=Buffer.byteLength(event.data);wireBytes+=size;
    if(size>8192||wireBytes>FRAME){pending?.reject(failure('cdp_metadata_limit',2));socket.close();return;}
    let message;try{message=JSON.parse(event.data);}catch{pending?.reject(failure('cdp_invalid_reply'));socket.close();return;}
    if(!('id' in message)){if(++events>4){pending?.reject(failure('cdp_event_limit',2));socket.close();}return;}
    if(!pending||message.id!==pending.id){pending?.reject(failure('cdp_uncorrelated_reply'));socket.close();return;}
    if(message.error)pending.reject(failure('target_unresolved'));
    else pending.resolve(message.result);
  });
  async function send(method,params={}){
    if(++sequence>5)throw failure('cdp_method_limit',2);
    const id=sequence;
    try{return await bounded(new Promise((resolve,reject)=>{pending={id,resolve,reject};socket.send(JSON.stringify({id,method,params}));}),2000,'cdp_metadata_timeout');}
    finally{pending=null;}
  }
  try{
    await bounded(new Promise((resolve,reject)=>{socket.addEventListener('open',resolve,{once:true});socket.addEventListener('error',()=>reject(failure('cdp_connection_failed')),{once:true});}),2000,'cdp_connect_timeout');
    const {targetInfo}=await send('Target.getTargetInfo');
    if(targetInfo?.targetId!==args.target||targetInfo?.type!=='page')throw failure('target_unresolved');
    const {frameTree}=await send('Page.getFrameTree'),frame=frameTree?.frame;
    if(frame?.id!==args.surface.id||frame?.loaderId!==args.surface.generation)throw failure('stale_document');
    const {root}=await send('DOM.getDocument',{depth:0,pierce:false});
    if(root?.nodeType!==9||root.backendNodeId!==args.document)throw failure('stale_document');
    const {node}=await send('DOM.describeNode',{backendNodeId:args.root,depth:0,pierce:false});
    if(node?.nodeType!==1||node.backendNodeId!==args.root)throw failure('root_unresolved');
    // Recheck navigation before handing the actual IDs to the existing collector.
    const again=await send('Page.getFrameTree');
    if(again.frameTree?.frame.id!==frame.id||again.frameTree?.frame.loaderId!==frame.loaderId)throw failure('stale_target');
    return {surface:{id:frame.id,generation:frame.loaderId},document:root.backendNodeId};
  }finally{
    socket.close();await bounded(closed,1000,'cdp_cleanup_unconfirmed');
  }
}

function normalizePageUrl(value){
  if(typeof value!=='string'||Buffer.byteLength(value)>4096||/[\x00-\x1f\x7f]/.test(value))throw failure('invalid_page_url',2);
  let url;try{url=new URL(value);}catch{throw failure('invalid_page_url',2);}
  if(!['http:','https:'].includes(url.protocol)||url.username||url.password)throw failure('invalid_page_url',2);
  const fragment=url.hash.slice(1),fragmentQuery=fragment.includes('?')?fragment.slice(fragment.indexOf('?')+1):fragment;
  for(const query of [url.searchParams,new URLSearchParams(fragmentQuery.includes('=')?fragmentQuery:'')])
    for(const key of query.keys())if(/token|secret|password|authorization|api[_-]?key|signature/i.test(key))throw failure('private_page_url',2);
  return url.href;
}
function parseFirstUse(argv){
  const allowed=new Set(['--connection','--url','--component','--max-visited-nodes','--cli','--worker']),args={};
  for(let i=0;i<argv.length;i+=2){
    if(!allowed.has(argv[i])||argv[i] in args||!argv[i+1])throw failure('invalid_arguments',2);
    args[argv[i]]=argv[i+1];
  }
  if(!args['--connection'])throw failure('connection_required',2);
  if(('--cli' in args)!==('--worker' in args))throw failure('matching_executables_required',2);
  let connection;try{connection=new URL(args['--connection']);}catch{throw failure('invalid_connection',2);}
  if(Buffer.byteLength(args['--connection'])>1024||connection.protocol!=='ws:'||!['127.0.0.1','[::1]'].includes(connection.hostname)
    ||!connection.port||connection.username||connection.password||connection.search||connection.hash
    ||!/^\/devtools\/browser\/[A-Za-z0-9_-]{1,256}$/.test(connection.pathname))throw failure('invalid_connection',2);
  const url=normalizePageUrl(args['--url']),selector=args['--component'];
  // Deliberately narrow compound grammar: native matches never scans relatives/:has.
  const compound=/^(?:[A-Za-z][A-Za-z0-9_-]*|\*)?(?:(?:[#.][A-Za-z_][A-Za-z0-9_-]*)|(?:\[[A-Za-z_][A-Za-z0-9_-]*(?:=(?:"[^"\\\]\r\n]*"|'[^'\\\]\r\n]*'|[A-Za-z0-9_-]+))?\]))*$/;
  if(typeof selector!=='string'||!selector||Buffer.byteLength(selector)>512||/[\x00-\x1f\x7f]/.test(selector)||!compound.test(selector))throw failure('unsupported_selector',2);
  const cap=args['--max-visited-nodes']??'2048';
  if(!/^[1-9][0-9]*$/.test(cap)||Number(cap)>2048)throw failure('invalid_selection_limit',2);
  return {connection:connection.href,url,selector,maxVisited:Number(cap),
    cli:args['--cli']??path.join(__dirname,'uiblueprint'),worker:args['--worker']??path.join(__dirname,'session-worker')};
}

// Bounded control-plane metadata, not a graph parser or browser launcher.
class BootstrapCdp{
  constructor(endpoint,deadline,budget={calls:0,events:0}){
    this.budget=budget;
    this.deadline=deadline;this.sequence=0;this.bytes=0;this.events=0;this.pending=null;this.fatal=null;
    this.socket=new WebSocket(endpoint);
    this.closed=new Promise(resolve=>this.socket.addEventListener('close',resolve,{once:true}));
    this.ready=new Promise((resolve,reject)=>{
      this.socket.addEventListener('open',resolve,{once:true});
      this.socket.addEventListener('error',()=>reject(failure('cdp_connection_failed',1)),{once:true});
      this.socket.addEventListener('close',()=>reject(failure('cdp_connection_closed',1)),{once:true});
    });
    const fail=error=>{this.fatal??=error;this.pending?.reject(error);};
    this.socket.addEventListener('error',()=>fail(failure('cdp_connection_failed',1)));
    this.socket.addEventListener('close',()=>fail(failure('cdp_connection_closed',1)));
    this.socket.addEventListener('message',event=>{
      if(typeof event.data!=='string')return fail(failure('cdp_invalid_reply'));
      const bytes=Buffer.byteLength(event.data);this.bytes+=bytes;
      if(bytes>65536||this.bytes>262144)return fail(failure('bootstrap_limit',2));
      let reply;try{reply=JSON.parse(event.data);}catch{return fail(failure('cdp_invalid_reply'));}
      if(!reply||typeof reply!=='object'||Array.isArray(reply))return fail(failure('cdp_invalid_reply'));
      if(!('id' in reply)){if(++this.budget.events>16)fail(failure('bootstrap_limit',2));return;}
      if(!this.pending||reply.id!==this.pending.id)return fail(failure('cdp_uncorrelated_reply'));
      if(reply.error)this.pending.reject(failure('target_unresolved'));
      else this.pending.resolve(reply.result);
    });
  }
  remaining(){const left=this.deadline-performance.now();if(left<=0)throw failure('bootstrap_deadline');return Math.min(2000,left);}
  static async open(endpoint,deadline,budget){
    const owner=new BootstrapCdp(endpoint,deadline,budget);
    try{await bounded(owner.ready,owner.remaining(),'bootstrap_deadline',4);return owner;}
    catch(error){await owner.close();throw error;}
  }
  async send(method,params={}){
    if(this.fatal)throw this.fatal;
    if(++this.budget.calls>32)throw failure('bootstrap_limit',2);
    const id=++this.sequence,wire=JSON.stringify({id,method,params});
    if(Buffer.byteLength(wire)>16384)throw failure('bootstrap_limit',2);
    // Teardown gets its own bounded allowance; it performs no collection.
    const timeout=method==='Runtime.releaseObjectGroup'?1000:this.remaining();
    try{return await bounded(new Promise((resolve,reject)=>{
      this.pending={id,resolve,reject};this.socket.send(wire);
    }),timeout,'bootstrap_deadline',4);}finally{this.pending=null;}
  }
  async close(){
    if(!this.closeStarted){this.closeStarted=true;this.socket.close();}
    await bounded(this.closed,1000,'cdp_cleanup_unconfirmed');
  }
}

// Fixed source code, invoked in an isolated world. Selector/options are JSON data.
function selectComponent(options){
  'use strict';
  const expected=this instanceof Document?this:this.document;
  if(expected!==document)return {status:'stale'};
  const end=performance.now()+options.remainingMs;
  let node=expected,depth=0,visited=0,found=null;
  while(node){
    if(performance.now()>=end)return {status:'timeout'};
    if(visited>=options.maxVisited)return {status:'incomplete'};
    visited++;
    if(node instanceof Element&&Element.prototype.matches.call(node,options.selector)){
      if(found)return {status:'ambiguous'};
      if(node.shadowRoot||['iframe','frame','template','slot'].includes(node.localName))return {status:'unsupported'};
      found=node;
    }
    const child=node.firstChild;
    if(child){if(depth>=64)return {status:'incomplete'};node=child;depth++;continue;}
    while(node!==expected&&!node.nextSibling){
      if(performance.now()>=end)return {status:'timeout'};
      node=node.parentNode;depth--;
    }
    if(node===expected)break;
    node=node.nextSibling;
  }
  if(!found)return {status:'missing'};
  if(!(this instanceof Document))return {status:found===this.root&&found.isConnected&&found.ownerDocument===expected&&found.parentNode===this.parent?'selected':'stale'};
  return {status:'selected',root:found,document:expected,parent:found.parentNode};
}
function heldCurrent(){
  'use strict';
  return {current:this.document===document&&this.root instanceof Element&&this.root.isConnected
    &&this.root.ownerDocument===document&&this.root.parentNode===this.parent};
}

async function openBootstrap(options){
  const deadline=performance.now()+10000,group=`uib-first-use-${crypto.randomUUID()}`,budget={calls:0,events:0};
  let browser=null,page=null,groupOwned=false,closed=false;
  const code={ambiguous:'ambiguous_component',missing:'component_unresolved',incomplete:'selection_limit',timeout:'selection_deadline',unsupported:'unsupported_scope',stale:'stale_target'};
  function selected(status){if(status!=='selected')throw failure(typeof status==='string'&&Object.hasOwn(code,status)?code[status]:'cdp_invalid_reply',status==='incomplete'?2:4);}
  function identity(value){if(typeof value!=='string'||!(/^[A-Za-z0-9_-]{1,256}$/).test(value))throw failure('cdp_invalid_reply');return value;}
  async function target(){
    const result=await browser.send('Target.getTargets');
    if(!Array.isArray(result?.targetInfos))throw failure('cdp_invalid_reply');
    if(result.targetInfos.length>128)throw failure('bootstrap_limit',2);
    const matches=result.targetInfos.filter(t=>{
      if(t?.type!=='page'||typeof t.url!=='string')return false;
      try{return normalizePageUrl(t.url)===options.url;}catch{return false;}
    });
    if(matches.length!==1)throw failure(matches.length?'ambiguous_target':'target_unresolved');
    return identity(matches[0].targetId);
  }
  async function metadata(){
    const {targetInfo}=await page.send('Target.getTargetInfo');
    let sameUrl=false;try{sameUrl=normalizePageUrl(targetInfo?.url)===options.url;}catch{}
    if(targetInfo?.targetId!==args.target||targetInfo?.type!=='page'||!sameUrl)throw failure('stale_target');
    const {frameTree}=await page.send('Page.getFrameTree');
    const frame=frameTree?.frame;
    const {root}=await page.send('DOM.getDocument',{depth:0,pierce:false});
    if(root?.nodeType!==9||!Number.isInteger(root.backendNodeId)||root.backendNodeId<=0||root.backendNodeId>2147483647)throw failure('cdp_invalid_reply');
    return {surface:{id:identity(frame?.id),generation:identity(frame?.loaderId)},document:root.backendNodeId};
  }
  async function invoke(objectId,fn,byValue){
    const result=await page.send('Runtime.callFunctionOn',{objectId,functionDeclaration:fn.toString(),
      arguments:[{value:{selector:options.selector,maxVisited:options.maxVisited,remainingMs:Math.min(1000,page.remaining())}}],
      // Chromium145 rejects these fixed readers in debug side-effect mode (F01).
      // They only read native DOM metadata; input is arguments, never source code.
      returnByValue:byValue,silent:true,userGesture:false,awaitPromise:false,throwOnSideEffect:false,objectGroup:group});
    if(result?.exceptionDetails||!result?.result)throw failure('selection_unavailable');
    return result.result;
  }
  async function describeRoot(objectId){
    const {node}=await page.send('DOM.describeNode',{objectId,depth:0,pierce:false});
    if(node?.nodeType!==1||!Number.isInteger(node.backendNodeId)||node.backendNodeId<=0||node.backendNodeId>2147483647)throw failure('cdp_invalid_reply');
    // Closed roots are absent from Element.shadowRoot but reported by CDP here.
    // Inspect only boundary presence; never resolve/traverse a shadow root.
    if(node.shadowRoots!==undefined){
      if(!Array.isArray(node.shadowRoots))throw failure('cdp_invalid_reply');
      for(const boundary of node.shadowRoots){
        if(boundary?.shadowRootType==='open'||boundary?.shadowRootType==='closed')throw failure('unsupported_scope');
        if(boundary?.shadowRootType!=='user-agent')throw failure('cdp_invalid_reply');
      }
    }
    return node.backendNodeId;
  }
  async function close(){
    if(closed)return;closed=true;let bad=false;
    if(page&&groupOwned){try{await page.send('Runtime.releaseObjectGroup',{objectGroup:group});}catch{bad=true;}}
    for(const owner of [page,browser])if(owner){try{await owner.close();}catch{bad=true;}}
    if(bad)throw failure('bootstrap_cleanup_unconfirmed',1);
  }
  let args,binding,container;
  try{
    browser=await BootstrapCdp.open(options.connection,deadline,budget);
    const id=await target(),endpoint=new URL(options.connection);endpoint.pathname=`/devtools/page/${id}`;
    args={cli:options.cli,worker:options.worker,target:id,endpoint:endpoint.href};
    page=await BootstrapCdp.open(args.endpoint,deadline,budget);binding=await metadata();
    const {executionContextId}=await page.send('Page.createIsolatedWorld',{frameId:binding.surface.id,worldName:'ui-blueprint-first-use',grantUniveralAccess:false});
    if(!Number.isInteger(executionContextId)||executionContextId<=0)throw failure('cdp_invalid_reply');
    groupOwned=true;
    const document=await page.send('Runtime.evaluate',{expression:'document',contextId:executionContextId,returnByValue:false,silent:true,objectGroup:group});
    if(document?.exceptionDetails||typeof document?.result?.objectId!=='string')throw failure('cdp_invalid_reply');
    const result=await invoke(document.result.objectId,selectComponent,false);
    if(typeof result.objectId!=='string')throw failure('cdp_invalid_reply');container=result.objectId;
    const properties=await page.send('Runtime.getProperties',{objectId:container,ownProperties:true,accessorPropertiesOnly:false,generatePreview:false});
    if(!Array.isArray(properties?.result)||properties.result.length>4)throw failure('cdp_invalid_reply');
    const status=properties.result.find(p=>p.name==='status')?.value?.value;selected(status);
    const root=properties.result.find(p=>p.name==='root')?.value?.objectId;
    if(typeof root!=='string')throw failure('cdp_invalid_reply');
    const backend=await describeRoot(root);
    args={...args,surface:binding.surface,document:binding.document,root:backend};
    const current=await metadata();
    if(JSON.stringify(current)!==JSON.stringify(binding)||(await invoke(container,heldCurrent,true)).value?.current!==true)throw failure('stale_target');
    return {args,binding,close,remaining:()=>{
      const left=deadline-performance.now();if(left<=0)throw failure('bootstrap_deadline');return left;
    },revalidate:async()=>{
      if(await target()!==args.target)throw failure('stale_target');
      if(JSON.stringify(await metadata())!==JSON.stringify(binding))throw failure('stale_target');
      if(await describeRoot(root)!==args.root)throw failure('stale_target');
      selected((await invoke(container,selectComponent,true)).value?.status);
    }};
  }catch(error){await close();throw error;}
}

function parseObserveDiagnostic(bytes,exit){
  const invalid=()=>failure('observe_diagnostic_invalid',1);
  if(!Buffer.isBuffer(bytes)||bytes.length>4096||!bytes.length)throw invalid();
  const text=bytes.toString('utf8');
  if(!text.endsWith('\n'))throw invalid();
  let d;try{d=JSON.parse(text);}catch{throw invalid();}
  const keys=(value,names)=>value!==null&&typeof value==='object'&&!Array.isArray(value)
    &&Object.keys(value).length===names.length&&names.every(k=>Object.hasOwn(value,k));
  const member=(value,names,nullable=false)=>(nullable&&value===null)||names.includes(value);
  const integer=(value,min,max)=>Number.isInteger(value)&&value>=min&&value<=max;
  const outer='output_version kind exit_code caller_phase caller_error operation terminal terminal_error requested_channels committed_channels missing_channels incomplete_channels worker_cleanup host_diagnostic'.split(' ');
  const callers='io_error invalid_arguments invalid_input_file invalid_connection invalid_connection_version invalid_request connection_request_mismatch input_limit output_limit invalid_deadline observe_invalid_or_limit observe_unavailable observe_incomplete observe_worker_or_cleanup_failure unsupported_observe_backend unsupported_observe_platform_or_feature'.split(' ');
  const hostErrors='invalid_limits resource_limit allocation_failure overflow busy invalid_input invalid_state invalid_control stale_operation deadline_expired permission_denied io worker_failed system_allocation_failure cleanup_pending resync_required action_refused'.split(' ');
  if(!keys(d,outer)||JSON.stringify(d)+'\n'!==text||d.output_version!=='1.0.0'||d.kind!=='observe_diagnostic'
    ||!integer(d.exit_code,0,5)||d.exit_code!==exit
    ||!member(d.caller_phase,'preflight read_inputs validate prepare attach configure_helpers submit wait_terminal shutdown publish_stdout complete'.split(' '))
    ||!member(d.caller_error,callers,true)||!member(d.operation,['attach','observe'],true)
    ||!member(d.terminal,['completed','failed','cancelled','timed_out'],true)
    ||!member(d.terminal_error,hostErrors,true)||!member(d.worker_cleanup,['not_started','confirmed','unconfirmed']))throw invalid();
  for(const field of ['requested_channels','committed_channels','missing_channels','incomplete_channels'])
    if(d[field]!==null&&!integer(d[field],0,7))throw invalid();
  if(d.worker_cleanup==='unconfirmed'&&d.exit_code!==1)throw invalid();
  const h=d.host_diagnostic;
  if(h!==null){
    const causes=['host',...'invalid_input limit stale_target resync_required malformed protocol timeout publication_stopped cleanup_unconfirmed'.split(' ').map(x=>'collector_'+x),
      ...'missing ambiguous incomplete unsupported timed_out'.split(' ').map(x=>'selection_'+x),
      ...'invalid_limits invalid_binding encoding envelope budget detached busy counter_exhausted wrong_session unexpected_reply uncorrelated_error cancelled'.split(' ').map(x=>'cdp_'+x),
      ...'invalid_limits endpoint_rejected logging_boundary connect handshake extensions timeout cancelled byte_limit work_limit io protocol capacity binary closed pending_write no_pending_write'.split(' ').map(x=>'transport_'+x)];
    if(!keys(h,'stage cause code selection_visited_nodes remote_cleanup send_progress'.split(' '))
      ||!member(h.stage,'decode begin permit capacity collect encode receive publish finish'.split(' '))
      ||!member(h.cause,causes)||!integer(h.code,-2147483648,2147483647)
      ||(h.cause.startsWith('selection_')?!integer(h.selection_visited_nodes,0,4294967295):h.selection_visited_nodes!==null)
      ||!member(h.remote_cleanup,['not_required','released','unconfirmed'])
      ||!member(h.send_progress,['not_queued','queued','possibly_written','flushed']))throw invalid();
  }
  return d;
}
function formatObserveDiagnostic(d){
  const value=x=>x===null?'unavailable':String(x),h=d.host_diagnostic;
  return 'observe_diagnostic exit='+d.exit_code+' phase='+d.caller_phase+' caller_error='+value(d.caller_error)
    +' operation='+value(d.operation)+' terminal='+value(d.terminal)+' terminal_error='+value(d.terminal_error)
    +' requested='+value(d.requested_channels)+' committed='+value(d.committed_channels)
    +' missing='+value(d.missing_channels)+' incomplete='+value(d.incomplete_channels)
    +' worker_cleanup='+d.worker_cleanup+(h?' stage='+h.stage+' cause='+h.cause+' code='+h.code
      +' selection_visited='+value(h.selection_visited_nodes)+' remote_cleanup='+h.remote_cleanup
      +' send_progress='+h.send_progress:' host_detail=unavailable');
}

function inputs(args,binding,outputBytes=FRAME,requestMs=250){
  const nonce=crypto.randomUUID(),mib=1048576;
  const target={id:args.target,generation:nonce},sessionId=`web-geometry-${nonce}`,scope='selected-component';
  const plugin={id:'web',version:'0.1.0'};
  const context={schema_version:'0.1.0',session_id:sessionId,target,surfaces:[binding.surface],scope_id:scope,projection:'design',
    fields:['layout_bounds','hit_region','visible_region'],plugin,environment_revision:nonce};
  const connection={connection_version:'1.0.0',target,attach_deadline_ms:2000,
    session:{session_id:sessionId,plugin,supported_versions:['0.1.0'],target,surfaces:[binding.surface],allowed_scopes:[scope],
      capabilities:[{channel:'external_semantics',operation:'observe',status:'partial',reason:'explicit_read_only_component'}]},
    host_limits:{workers:2,worker_bytes:64*mib,publication_reserve:mib,bootstrap_bytes:mib,parent_bytes:32*mib,input_bytes:2*mib,
      ingress_bytes:512*1024,output_bytes:512*1024,request_output_bytes:2*mib,completion_groups:2,control_bytes:4096,
      cleanup_ms:1000,retained_domain_bytes:64*mib,retained_per_worker:15*mib,main_stack_bytes:8*mib,watchdog_stack_bytes:mib},
    provider:{backend:'web',selection:{selection:'rooted',max_visited_nodes:256,root:{session_id:sessionId,target,surface:binding.surface,
      document_backend_id:binding.document,backend_node_id:args.root,sensitivity:'public'}},
      setup:{endpoint:args.endpoint,cdp_session_id:null,surface:binding.surface,
        transport:{endpoint_bytes:1024,handshake_bytes:2048,read_buffer_bytes:64,write_buffer_bytes:64,write_buffer_max:32768,frame_bytes:8192,message_bytes:8192,outbound_bytes:16384},
        cdp:{max_request_bytes:16384,max_message_bytes:8192,max_metadata_bytes:256,max_results:1,result_bytes:8192,max_events:4,event_bytes:34000},
        collector:{max_nodes:16,max_methods:100,max_reply_bytes:8192,max_total_reply_bytes:65536,max_text_bytes:600,max_handle_bytes:256,max_ax_properties:32,io_read_bytes:16384,io_write_bytes:32768,io_work:2048}}}};
  const request={schema_version:'0.1.0',artifact:{kind:'request',data:{request_id:nonce,clock_domain:'rebound_after_attach',context,
    limits:{max_elements:32,max_depth:8,max_output_bytes:outputBytes,deadline_ms:requestMs},freshness_policy:'current_required',operation:{operation:'observe',channels:['external_semantics']}}}};
  return {connection,request};
}

async function example(args,bootstrap=null){
  const outputBytes=bootstrap?131072:FRAME,requestMs=bootstrap?1000:250;
  const binding=bootstrap?bootstrap.binding:await metadata(args),input=inputs(args,binding,outputBytes,requestMs);
  const temp=await fs.mkdtemp(path.join(os.tmpdir(),'uib-web-geometry-')),owned=[];let child=null,closed=null;
  const started=performance.now();
  async function save(name,bytes){const file=path.join(temp,name),handle=await fs.open(file,'wx',0o600);owned.push(file);try{await handle.writeFile(bytes);}finally{await handle.close();}return file;}
  async function json(name,value){return save(name,JSON.stringify(value));}
  async function call(argv){
    if(performance.now()-started>=120000)throw failure('example_deadline',1);
    const bracket=bootstrap&&argv[0]==='observe';
    const timeout=bracket?Math.min(5000,bootstrap.remaining()):5000;
    child=spawn(args.cli,argv,{stdio:['ignore','pipe','pipe']});const chunks=[],errorChunks=[];let total=0,diagnostics=0,over=false;
    child.stdout.on('data',chunk=>{total+=chunk.length;if(total>outputBytes){over=true;child.kill('SIGKILL');}else chunks.push(chunk);});
    child.stderr.on('data',chunk=>{diagnostics+=chunk.length;if(diagnostics<=4096)errorChunks.push(chunk);if(diagnostics>FRAME){over=true;child.kill('SIGKILL');}});
    closed=new Promise((resolve,reject)=>{child.once('error',()=>reject(failure('cli_spawn_failed',1)));child.once('close',(code,signal)=>resolve({code,signal}));});
    const result=await bounded(closed,timeout,bracket?'bootstrap_deadline':'cli_timeout',bracket?4:1);
    if(over||result.signal)throw failure('cli_output_or_cleanup_failure',1);
    return {code:result.code,bytes:Buffer.concat(chunks),stderr:diagnostics<=4096?Buffer.concat(errorChunks):Buffer.alloc(0)};
  }
  try{
    const connection=await json('connection.json',input.connection),request=await json('request.json',input.request);
    const observed=await call(['observe','--connection',connection,'--request',request,'--worker',args.worker,'--max-input-bytes','131072','--max-output-bytes',String(outputBytes),
      ...(bootstrap?['--diagnostics-json','--max-diagnostic-bytes','4096']:[])]);
    const diagnostic=bootstrap?parseObserveDiagnostic(observed.stderr,observed.code):null;
    if(![0,4].includes(observed.code)||!observed.bytes.length){
      const error=failure(`observe_refused_exit_${observed.code}`,[1,2,3,4,5].includes(observed.code)?observed.code:1);error.observation=diagnostic;throw error;
    }
    if(bootstrap){await bootstrap.revalidate();await bootstrap.close();}
    const source=JSON.parse(observed.bytes),snapshot=source.artifact?.data?.result?.data;
    if(source.artifact?.kind!=='channel_response'||source.artifact.data.result.status!=='observed'||!snapshot)throw failure('snapshot_unavailable');
    const file=await save('observe.json',observed.bytes),key={namespace:'web.dom',key:String(args.root)};
    const root=snapshot.nodes.find(node=>node.key.namespace===key.namespace&&node.key.key===key.key);
    if(!root)throw failure('root_not_observed');
    if(diagnostic&&observed.code!==0)console.error(formatObserveDiagnostic(diagnostic));
    const inspected=await call(['inspect','--snapshot',file,'--ref',JSON.stringify(key),'--view','design','--max-input-bytes','131072','--max-output-bytes',String(outputBytes)]);
    if(inspected.code!==0)throw failure(`inspect_refused_exit_${inspected.code}`,inspected.code);
    process.stdout.write(inspected.bytes);
    console.log('Component parts: reported layout_bounds, source-native roles and reported children; normalized names/roles not requested.');
    for(const node of snapshot.nodes){
      const prop=node.properties.find(p=>p.field==='layout_bounds'),state=prop?.state;
      if(state?.availability!=='known'){console.log(`${node.key.namespace}:${node.key.key} ${state?.availability??'not_requested'} ${state?.reason??''} evidence=${JSON.stringify(prop?.evidence??null)}`);continue;}
      const geometry=state.value.value,space=geometry.coordinate_space;
      console.log(`${node.key.namespace}:${node.key.key} ${geometry.frame_kind} ${JSON.stringify(geometry.shape)} ${space.units} ${space.kind}/${space.origin} space=${space.id} transform=${JSON.stringify(geometry.transform)} evidence=${JSON.stringify(prop.evidence)} native_role=${JSON.stringify(node.native_role)} reported_children=${JSON.stringify(node.children)}`);
    }
    const geometry=root.properties.find(p=>p.field==='layout_bounds')?.state;
    if(geometry?.availability!=='known'){console.log('Root dimensions unknown: layout_bounds unavailable.');return 4;}
    const space=geometry.value.value.coordinate_space;let exit=observed.code;
    for(const operation of ['width','height']){
      const query={schema_version:'0.2.0',artifact:{kind:'geometry_query',data:{id:operation,scope_id:snapshot.context.scope_id,targets:[key],operation,
        anchors:[{element:key,frame_kind:'layout_bounds',coordinate_space:space,fraction:0,axis:'x'}],quantity_kind:'length',units:space.units,
        applies_when:{platform:null,input_mode:null,text_scale:null}}}};
      const q=await json(`${operation}.json`,query);
      const measured=await call(['measure','--snapshot',file,'--query',q,'--space',space.id,'--max-input-bytes','131072','--max-output-bytes',String(outputBytes),'--json']);
      if(![0,4].includes(measured.code))throw failure(`measure_refused_exit_${measured.code}`,measured.code);
      const result=JSON.parse(measured.bytes);assert.equal(result.artifact.kind,'measurement');assert.deepStrictEqual(result.artifact.data.snapshot,snapshot);
      console.log(`${operation}: ${JSON.stringify(result.artifact.data.result)}`);if(measured.code===4)exit=4;
    }
    assert((await fs.readFile(file)).equals(observed.bytes));
    console.log(`coverage=${JSON.stringify(snapshot.coverage)} observations=${JSON.stringify(snapshot.observations)}`);
    return exit;
  }finally{
    let cleanupFailed=false;
    if(child&&child.exitCode===null&&child.signalCode===null)child.kill('SIGKILL');
    try{if(closed)await bounded(closed,2000,'cli_cleanup_unconfirmed');}catch{cleanupFailed=true;}
    for(const file of owned){try{await fs.unlink(file);}catch{cleanupFailed=true;}}
    try{await fs.rmdir(temp);}catch{cleanupFailed=true;}
    if(cleanupFailed)throw failure('example_cleanup_unconfirmed',1);
  }
}

async function main(){
  if(process.argv.length===3&&['--help','-h'].includes(process.argv[2])){
    process.stdout.write(HELP);return;
  }
  try{
    const argv=process.argv.slice(2);
    if(argv.includes('--endpoint'))process.exitCode=await example(await argumentsFor(argv));
    else{
      const options=parseFirstUse(argv);
      for(const key of ['cli','worker']){
        if(!path.isAbsolute(options[key])||!(await fs.stat(options[key])).isFile())throw failure('invalid_executable',2);
        await fs.access(options[key],require('node:fs').constants.X_OK);
      }
      const bootstrap=await openBootstrap(options);
      try{process.exitCode=await example(bootstrap.args,bootstrap);}
      finally{await bootstrap.close();}
    }
  }
  catch(error){const code=error.code&&Number.isInteger(error.exit)?error.code:'example_io_or_validation_failure';console.error(error.observation?code+' '+formatObserveDiagnostic(error.observation):code==='connection_required'?'connection_required: provide an already authorized loopback CDP browser WebSocket via --connection; see --help':code);process.exitCode=error.exit??1;}
}
if(require.main===module)main();
module.exports={parseFirstUse,normalizePageUrl,selectComponent,heldCurrent,openBootstrap,BootstrapCdp,inputs,example,parseObserveDiagnostic,formatObserveDiagnostic};
