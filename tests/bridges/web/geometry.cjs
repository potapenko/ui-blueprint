// Developer example only: explicit existing CDP target -> public Rust CLI.
// No browser launch/navigation, selector scan, application input or JS geometry.
const fs = require('node:fs/promises');
const path = require('node:path');
const os = require('node:os');
const crypto = require('node:crypto');
const {spawn} = require('node:child_process');
const assert = require('node:assert/strict');

const FRAME=65536;
function failure(code,exit=4){return Object.assign(new Error(code),{code,exit});}
function bounded(promise,ms,code){
  let timer;return Promise.race([promise,new Promise((_,reject)=>{timer=setTimeout(()=>reject(failure(code,1)),ms);})]).finally(()=>clearTimeout(timer));
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

function inputs(args,binding){
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
    limits:{max_elements:32,max_depth:8,max_output_bytes:FRAME,deadline_ms:250},freshness_policy:'current_required',operation:{operation:'observe',channels:['external_semantics']}}}};
  return {connection,request};
}

async function example(args){
  const binding=await metadata(args),input=inputs(args,binding);
  const temp=await fs.mkdtemp(path.join(os.tmpdir(),'uib-web-geometry-')),owned=[];let child=null,closed=null;
  const started=performance.now();
  async function save(name,bytes){const file=path.join(temp,name),handle=await fs.open(file,'wx',0o600);owned.push(file);try{await handle.writeFile(bytes);}finally{await handle.close();}return file;}
  async function json(name,value){return save(name,JSON.stringify(value));}
  async function call(argv){
    if(performance.now()-started>=120000)throw failure('example_deadline',1);
    child=spawn(args.cli,argv,{stdio:['ignore','pipe','pipe']});const chunks=[];let total=0,diagnostics=0,over=false;
    child.stdout.on('data',chunk=>{total+=chunk.length;if(total>FRAME){over=true;child.kill('SIGKILL');}else chunks.push(chunk);});
    child.stderr.on('data',chunk=>{diagnostics+=chunk.length;if(diagnostics>FRAME){over=true;child.kill('SIGKILL');}});
    closed=new Promise((resolve,reject)=>{child.once('error',()=>reject(failure('cli_spawn_failed',1)));child.once('close',(code,signal)=>resolve({code,signal}));});
    const result=await bounded(closed,5000,'cli_timeout');
    if(over||result.signal)throw failure('cli_output_or_cleanup_failure',1);
    return {code:result.code,bytes:Buffer.concat(chunks)};
  }
  try{
    const connection=await json('connection.json',input.connection),request=await json('request.json',input.request);
    const observed=await call(['observe','--connection',connection,'--request',request,'--worker',args.worker,'--max-input-bytes','131072','--max-output-bytes',String(FRAME)]);
    if(![0,4].includes(observed.code)||!observed.bytes.length)throw failure(`observe_refused_exit_${observed.code}`,[0,1,2,3,4,5].includes(observed.code)?observed.code:1);
    const source=JSON.parse(observed.bytes),snapshot=source.artifact?.data?.result?.data;
    if(source.artifact?.kind!=='channel_response'||source.artifact.data.result.status!=='observed'||!snapshot)throw failure('snapshot_unavailable');
    const file=await save('observe.json',observed.bytes),key={namespace:'web.dom',key:String(args.root)};
    const root=snapshot.nodes.find(node=>node.key.namespace===key.namespace&&node.key.key===key.key);
    if(!root)throw failure('root_not_observed');
    const inspected=await call(['inspect','--snapshot',file,'--ref',JSON.stringify(key),'--view','design','--max-input-bytes','131072','--max-output-bytes',String(FRAME)]);
    if(inspected.code!==0)throw failure(`inspect_refused_exit_${inspected.code}`,inspected.code);
    process.stdout.write(inspected.bytes);
    console.log('Component parts: reported layout_bounds; names/roles not requested.');
    for(const node of snapshot.nodes){
      const prop=node.properties.find(p=>p.field==='layout_bounds'),state=prop?.state;
      if(state?.availability!=='known'){console.log(`${node.key.namespace}:${node.key.key} ${state?.availability??'not_requested'} ${state?.reason??''} evidence=${JSON.stringify(prop?.evidence??null)}`);continue;}
      const geometry=state.value.value,space=geometry.coordinate_space;
      console.log(`${node.key.namespace}:${node.key.key} ${geometry.frame_kind} ${JSON.stringify(geometry.shape)} ${space.units} ${space.kind}/${space.origin} space=${space.id} transform=${JSON.stringify(geometry.transform)} evidence=${JSON.stringify(prop.evidence)}`);
    }
    const geometry=root.properties.find(p=>p.field==='layout_bounds')?.state;
    if(geometry?.availability!=='known'){console.log('Root dimensions unknown: layout_bounds unavailable.');return 4;}
    const space=geometry.value.value.coordinate_space;let exit=observed.code;
    for(const operation of ['width','height']){
      const query={schema_version:'0.2.0',artifact:{kind:'geometry_query',data:{id:operation,scope_id:snapshot.context.scope_id,targets:[key],operation,
        anchors:[{element:key,frame_kind:'layout_bounds',coordinate_space:space,fraction:0,axis:'x'}],quantity_kind:'length',units:space.units,
        applies_when:{platform:null,input_mode:null,text_scale:null}}}};
      const q=await json(`${operation}.json`,query);
      const measured=await call(['measure','--snapshot',file,'--query',q,'--space',space.id,'--max-input-bytes','131072','--max-output-bytes',String(FRAME),'--json']);
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
  try{process.exitCode=await example(await argumentsFor(process.argv.slice(2)));}
  catch(error){console.error(error.code&&Number.isInteger(error.exit)?error.code:'example_io_or_validation_failure');process.exitCode=error.exit??1;}
}
if(require.main===module)main();
