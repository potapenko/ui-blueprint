// W05 finite qualification of shipping commands. Browser calls are fixture stimuli
// or independent oracles; all positive form input goes through the public CLI.
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const crypto = require('node:crypto');
const CANARY = 'W05_PRIVATE_VALUE_CANARY';

module.exports = async function formE2e(h) {
  const {pages, binding, cliConnection, cliRequest, callCli, uiState, property,
    writeExclusive, evidence, report, worker} = h;
  const page = pages.a, pins = [], keys = new Map();
  report.phase = 'public_form_e2e';
  report.binaries.cli = report.binaries.test; delete report.binaries.test;
  await page.bringToFront(); // Own headless tab activation only, no field input.
  const other = await uiState(pages.b, true);
  const connection = cliConnection(await binding(page), 'draft');
  connection.session.session_id = 'w05-form';
  connection.session.allowed_scopes = ['w05-form'];
  const context = {schema_version:'0.1.0',session_id:'w05-form',target:connection.target,
    surfaces:connection.session.surfaces,scope_id:'w05-form',projection:'interaction',
    fields:['enabled','focused','value','input_kind','readonly','invalid','expanded','layout_bounds'],
    plugin:connection.session.plugin,environment_revision:'w05-fixture'};
  const emptyConditions = {platform:null,input_mode:null,text_scale:null};
  const file = stage => path.join(evidence, `${stage}.json`);
  async function save(stage, value) {
    const bytes = Buffer.from(JSON.stringify(value)); assert(bytes.length <= 131072);
    await writeExclusive(file(stage), bytes); pins.push({file:file(stage), bytes}); return file(stage);
  }
  async function identify(ids) {
    const cdp = await page.context().newCDPSession(page);
    try {
      const {root} = await cdp.send('DOM.getDocument',{depth:0,pierce:false});
      for (const id of ids) {
        const found = await cdp.send('DOM.querySelectorAll',{nodeId:root.nodeId,selector:`#${id}`});
        assert.equal(found.nodeIds.length,1);
        const {node} = await cdp.send('DOM.describeNode',{nodeId:found.nodeIds[0],depth:0,pierce:false});
        keys.set(id,{namespace:'web.dom',key:String(node.backendNodeId)});
      }
    } finally { await cdp.detach(); }
  }
  function node(snapshot,id) {
    const matches = snapshot.nodes.filter(n => JSON.stringify(n.key) === JSON.stringify(keys.get(id)));
    assert.equal(matches.length,1); return matches[0];
  }
  function ax(snapshot,id) {
    const source=node(snapshot,id);
    const link=snapshot.relations.find(r=>r.kind==='corresponds_to'&&JSON.stringify(r.from)===JSON.stringify(source.key));
    assert(link); const result=snapshot.nodes.find(n=>JSON.stringify(n.key)===JSON.stringify(link.to));assert(result);return result;
  }
  async function invoke(stage,kind,request,source,expectation,exit=0,ids=['draft']) {
    const config = await save(`${stage}-connection`, {...connection,provider:{...connection.provider,
      selection:{selection:'initial',ids:ids.map(id=>({id,sensitivity:'public'})),max_visited_nodes:256}}});
    const input = await save(`${stage}-request`,request);
    const args = kind === 'observe' ? ['observe'] : ['action',kind,
      kind === 'prepare' ? '--snapshot' : '--plan',file(source.stage),
      '--expectation',await save(`${stage}-expectation`,expectation),'--json'];
    args.push('--connection',config,'--request',input,'--worker',worker,
      '--max-input-bytes','131072','--max-output-bytes','65536');
    const prior = await uiState(page,true), result = await callCli(stage,args,exit), after = await uiState(page,true);
    if (kind !== 'execute' || exit !== 0) assert.deepEqual(after,prior);
    assert.deepEqual(after.scroll,prior.scroll); assert.deepEqual(after.state.rows,prior.state.rows);
    assert.deepEqual(await uiState(pages.b,true),other,'other exact target unchanged');
    report.checks.push({case:stage,exit,readonly:kind!=='execute',other_target_unchanged:true});
    return {...result,stage,prior,after};
  }
  async function observe(stage,ids=['draft','applied','commit'],fields=context.fields) {
    await identify(ids);
    const result = await invoke(stage,'observe',cliRequest({...context,fields},
      {operation:'observe',channels:['external_semantics']},stage),null,null,4,ids);
    assert.equal(result.document.artifact.kind,'channel_response');
    assert.equal(result.document.artifact.data.result.status,'observed');
    return {...result,snapshot:result.document.artifact.data.result.data};
  }
  async function prepare(stage,source,actorId,resultId,intent,field,expected) {
    const s=source.snapshot,actor=node(s,actorId),result=node(s,resultId);
    const ev=actor.properties.find(p=>p.field==='enabled').evidence;
    const unknown={availability:'unknown',reason:'not-prepared'};
    const action={id:stage,context:s.context,backend_ref:{session_id:s.context.session_id,
      target:s.context.target,surface:actor.surface,key:actor.key,snapshot_id:s.id,observation_id:ev.observation_id},
      intent,modality:intent.intent==='type'?'keyboard':'semantic',input_space:null,required_enabled:true,
      authorized_scope:s.context.scope_id,unique_match:false,
      resolution:{evidence:ev,writable:unknown,value_allowed:unknown,available_intents:[]}};
    const expectation={schema_version:'0.1.0',artifact:{kind:'expectation',data:{id:`${stage}-expected`,
      scope_id:s.context.scope_id,targets:[result.key],rule:{relation:'property_equals',field,expected},
      applies_when:emptyConditions,expected_from:'independently-authored-F01-B02-W05'}}};
    const out=await invoke(stage,'prepare',cliRequest(s.context,{operation:'prepare',action},stage),source,expectation);
    assert.equal(out.document.artifact.kind,'action');
    const prepared=out.document.artifact.data.action;
    const methods={focus:'cdp.DOM.focus-native-text-control',type:'cdp.Input.insertText-native-ImeCommitText',
      activate:'dom.HTMLElement.click-native-button-untrusted'};
    assert.equal(prepared.resolution.evidence.method,methods[intent.intent]);
    assert.equal(prepared.modality,action.modality);
    return {...out,expectation,resultId,field};
  }
  async function execute(stage,prepared,exit=0) {
    const c=prepared.document.artifact.data;
    const result=await invoke(stage,'execute',cliRequest(c.snapshot.context,{operation:'act',action:c.action},stage),prepared,prepared.expectation,exit);
    const t=result.document.artifact.data;
    assert.equal(result.document.artifact.kind,'transition_context'); assert.equal(t.transition.steps.length,1);
    const step=t.transition.steps[0];
    if(exit===0) {
      assert.equal(step.delivery,'confirmed');assert.equal(step.outcome,'succeeded');
      assert(t.after);assert(step.verification_observation);assert.notEqual(t.before.id,t.after.id);
      assert.deepEqual(property(node(t.after,prepared.resultId),prepared.field),prepared.expectation.artifact.data.rule.expected);
    } else {
      assert.equal(step.delivery,'not_dispatched');assert.equal(step.outcome,'failed');assert.equal(t.after,null);
      assert.deepEqual(t.transition.completed_steps,[]);
      assert.equal(t.transition.stop_on_error,true);assert.equal(t.transition.stopped_at,step.id);
    }
    return result;
  }
  const text=value=>({type:'text',value});
  function state(actual,draft,selected,applied,valid,delivered) {
    assert.deepEqual([actual.draft,actual.selected,actual.applied,actual.valid,actual.delivered],
      [draft,selected,applied,valid,delivered]);
  }
  const initial=await observe('e2e-initial');
  state(initial.after.state,'','','',false,0);
  for(const id of ['draft','applied'])assert.equal(property(node(initial.snapshot,id),'value').value,'');
  await execute('e2e-focus',await prepare('e2e-focus-plan',initial,'draft','draft',{intent:'focus'},'focused',{type:'flag',value:true}));
  const focused=await observe('e2e-focused');
  assert.deepEqual(focused.snapshot.focus.keyboard.target,keys.get('draft'));
  await execute('e2e-partial',await prepare('e2e-partial-plan',focused,'draft','draft',{intent:'type',text:'Lo'},'value',text('Lo')));
  // Named authored readiness, never network-idle or an arbitrary stability sleep.
  await page.waitForFunction(()=>window.f01.checkpoint().source_state==='draft-invalid');
  const partial=await observe('e2e-invalid');
  state(partial.after.state,'Lo','','',false,0);assert.equal(property(ax(partial.snapshot,'draft'),'invalid').value,true);
  await execute('e2e-complete-draft',await prepare('e2e-complete-plan',partial,'draft','draft',{intent:'type',text:'n'},'value',text('Lon')));
  await page.waitForFunction(()=>window.f01.checkpoint().source_state==='suggestion-ready');
  const ready=await observe('e2e-ready',['draft','option-london','applied']);
  state(ready.after.state,'Lon','','',false,0);assert.equal(property(node(ready.snapshot,'draft'),'expanded').value,true);
  const selected=await execute('e2e-select',await prepare('e2e-select-plan',ready,'option-london','draft',{intent:'activate'},'value',text('London')));
  state(selected.after.state,'London','London','',true,0);assert.equal(await page.locator('#option-london').count(),0);
  const commitSource=await observe('e2e-selected');
  assert.equal(property(ax(commitSource.snapshot,'draft'),'invalid').value,false);
  assert.equal(property(node(commitSource.snapshot,'applied'),'value').value,'');
  const applied=await execute('e2e-apply',await prepare('e2e-apply-plan',commitSource,'commit','applied',{intent:'activate'},'value',text('London')));
  state(applied.after.state,'London','London','London',true,1);
  const final=await observe('e2e-applied');
  assert.equal(property(node(final.snapshot,'applied'),'value').value,'London');
  assert.deepEqual(await page.evaluate(()=>window.f01.events.map(e=>e.source_state)),
    ['draft-pending','draft-invalid','draft-pending','suggestion-ready','city-selected','city-applied']);

  const diff=await callCli('e2e-diff',['diff','--graph','--before',file(initial.stage),'--after',file(final.stage),
    '--max-input-bytes','131072','--max-output-bytes','262144','--max-entries','256','--json'],0,262144);
  assert.deepEqual(diff.document.before,initial.snapshot);assert.deepEqual(diff.document.after,final.snapshot);
  assert.equal(diff.document.omitted_entries,0);
  for(const id of ['draft','applied']) {
    const index=final.snapshot.nodes.indexOf(node(final.snapshot,id));
    assert(diff.document.entries.some(e=>e.kind==='property'&&e.field==='value'&&e.after_index===index&&e.content_changed));
  }
  // Authored CSS gives Commit height32. This is a geometry rule, not applied success.
  const actor=node(final.snapshot,'commit'),space=property(actor,'layout_bounds').value.coordinate_space.id;
  const expectation={schema_version:'0.1.0',artifact:{kind:'expectation',data:{id:'w05-commit-height',scope_id:context.scope_id,
    targets:[actor.key],rule:{relation:'geometry',operation:'height',anchors:[{element:actor.key,frame_kind:'layout_bounds',coordinate_space:property(actor,'layout_bounds').value.coordinate_space,fraction:0.5,axis:'y'}],
      comparison:'equal',expected:32,quantity_kind:'length',units:'css_px',tolerance:0.01},
    applies_when:emptyConditions,expected_from:'F01-extension-css-button-height32'}}};
  const checkInput=await save('e2e-height-expectation',expectation);
  const checked=await callCli('e2e-check',['check','--snapshot',file(final.stage),'--expectation',checkInput,'--space',space,
    '--max-input-bytes','131072','--max-output-bytes','131072','--json','--result-version','0.2.0'],0,131072);
  assert.equal(checked.document.artifact.data.finding.status,'pass');
  assert.equal(checked.document.artifact.data.finding.measured.value.amount,32);
  assert.deepEqual(checked.document.artifact.data.snapshot,final.snapshot);
  const metadata={document_id:'W05',revision:'1',title:'Fixture form states',audience:'Developer',language:'en',date:'2026-10-08',
    owner:'W05 caller',retention:'Until this finite qualification is consumed',specification_refs:['UIB.DRAWING@1.1'],
    approval:{status:'draft',named_record:null},page_format:'A3 proportions',output_size:'3840 x 2160 output pixels'};
  const side=state=>({title:'Addressed form',state,scope:'draft, applied output and Commit control',environment:'Own headless Chromium145 800x600 DPR1',
    safe_source_reference:'W05 explicit saved observation',not_depicted:['Other fixture controls and pixels'],public_text_fields:[]});
  const meta=await save('e2e-export-metadata',{metadata,before:side('Empty form'),after:side('Fixture applied London'),different_basis:null,geometry_space:space});
  const destination=path.join(evidence,'compare');
  const exported=await callCli('e2e-export',['imagegen-prompt','--before',file(initial.stage),'--after',file(final.stage),'--metadata',meta,
    '--out',destination,'--purpose','compare','--max-input-bytes','2000000','--max-output-bytes','4000000',
    '--max-components','256','--max-views','8','--components-per-detail','12','--json'],0);
  assert.equal(exported.document.generated_image,false);assert.equal(exported.document.validation_status,'unverified');
  assert.equal(exported.document.comparison_attribution,'engine_recorded_graph');
  assert.deepEqual((await fs.readdir(destination)).sort(),['dimensions.json','drawing-brief.md','manifest.json','prompt.txt','scene.json','sheets.json']);
  const scene=JSON.parse(await fs.readFile(path.join(destination,'scene.json'),'utf8'));
  assert.equal(scene.views.length,2);assert.equal(scene.comparison_results.length,1);
  assert.equal(scene.comparison_results[0].status,'compared');
  assert.equal(scene.comparison_results[0].omitted_entries,0);
  assert(scene.comparison_results[0].entries.some(e=>e.field==='value'&&e.content_changed));
  for(const [index,source] of [initial.snapshot,final.snapshot].entries()) {
    const view=scene.views[index];assert.equal(view.components.length,source.nodes.length);
    assert.equal(view.source_kind,'observed');assert.equal(view.coverage.status,source.coverage.status);
    assert.equal(view.observations.length,source.observations.length);
    for(const [i,o] of source.observations.entries()) {
      for(const field of ['start','end','last_verified','time_unit','consistency','freshness','answer_source'])
        assert.deepEqual(view.observations[i][field],o[field]);
    }
  }

  // Explicit adversarial fixture stimuli after the positive chain. No product
  // input claim is made for opening a portal, remounting, disabling or marking private.
  const pending=await prepare('e2e-stale-plan',final,'commit','applied',{intent:'activate'},'value',text('London'));
  await page.locator('#open-popup').click();
  const portal=await observe('e2e-portal',['open-popup','portal','close-popup'],['enabled','role','layout_bounds']);
  assert.equal(await page.locator('#portal').evaluate(n=>n.parentElement.tagName),'BODY');
  assert(portal.snapshot.relations.some(r=>r.kind==='anchored_to'&&JSON.stringify(r.from)===JSON.stringify(keys.get('portal'))));
  const old=keys.get('commit');
  await page.evaluate(()=>{const n=document.getElementById('commit');n.replaceWith(n.cloneNode(true));});
  await execute('e2e-stale',pending,4);
  const remounted=await observe('e2e-remounted');assert.notDeepEqual(keys.get('commit'),old);
  state(remounted.after.state,'London','London','London',true,1);
  const disabled=await prepare('e2e-disabled-plan',remounted,'commit','applied',{intent:'activate'},'value',text('London'));
  await page.locator('#surprise').click();
  const stopped=await observe('e2e-stop',['commit','unexpected'],['enabled','role']);
  assert.equal(property(node(stopped.snapshot,'commit'),'enabled').value,false);
  assert.equal(property(ax(stopped.snapshot,'unexpected'),'role').value,'dialog');
  const start=report.outcomes.length;
  // Refusal probe is an independent adversarial case, not a dependent workflow step.
  await execute('e2e-disabled',disabled,4);
  assert.equal(report.outcomes.length-start,1);
  const publicDraft=await observe('e2e-before-private',['draft']);
  const privatePlan=await prepare('e2e-private-plan',publicDraft,'draft','draft',{intent:'focus'},'focused',{type:'flag',value:true});
  await page.evaluate(canary=>{const draft=document.getElementById('draft');
    draft.setAttribute('autocomplete','one-time-code');draft.value=canary;},CANARY);
  await execute('e2e-private',privatePlan,4);
  const secret=await observe('e2e-private-observe',['draft']);
  assert.equal(node(secret.snapshot,'draft').properties.find(p=>p.field==='value').state.availability,'redacted');
  // A separate permitted probe makes the public result unreadable AFTER delivery.
  // The unknown effect must remain terminal; no automatic retry of the click.
  await page.evaluate(()=>{
    const button=document.getElementById('commit');button.disabled=false;
    window.w05UnknownDispatches=0;
    button.onclick=()=>{window.w05UnknownDispatches++;
      document.getElementById('applied').setAttribute('autocomplete','one-time-code');};
  });
  const uncertainSource=await observe('e2e-before-unknown',['commit','applied']);
  const uncertain=await prepare('e2e-unknown-plan',uncertainSource,'commit','applied',{intent:'activate'},'value',text('London'));
  const c=uncertain.document.artifact.data;
  const unknown=await invoke('e2e-unknown','execute',cliRequest(c.snapshot.context,{operation:'act',action:c.action},'e2e-unknown'),uncertain,uncertain.expectation,4);
  assert.equal(unknown.document.artifact.kind,'transition_context');
  const transition=unknown.document.artifact.data;
  assert.equal(transition.transition.steps[0].delivery,'confirmed');
  assert.equal(transition.transition.steps[0].outcome,'action_outcome_unknown');assert.equal(transition.after,null);
  assert.equal(transition.transition.stop_on_error,true);
  assert.equal(transition.transition.stopped_at,transition.transition.steps[0].id);
  assert.equal(await page.evaluate(()=>window.w05UnknownDispatches),1);
  const reconciled=await observe('e2e-unknown-reconcile',['applied']);
  assert.equal(node(reconciled.snapshot,'applied').properties.find(p=>p.field==='value').state.availability,'redacted');
  assert.equal(await page.evaluate(()=>window.w05UnknownDispatches),1);
  for(const pin of pins)assert((await fs.readFile(pin.file)).equals(pin.bytes));
  for(const frame of report.frames) {
    const bytes=await fs.readFile(path.join(evidence,frame.file));assert(!bytes.includes(CANARY));
    assert.equal(crypto.createHash('sha256').update(bytes).digest('hex'),frame.sha256);
  }
  for(const name of await fs.readdir(destination))assert(!(await fs.readFile(path.join(destination,name))).includes(CANARY));
  report.form_e2e={draft_input:'Lo then n via public Type',selected:'London',applied:'London',fixture_deliveries:1,
    positive_execute_calls:5,dependent_execute_after_unexpected:0,negative_refusals:['stale','disabled','private'],
    unknown_effect_dispatches:1,unknown_effect_retried:false,private_canary_absent:true,
    compare_files:6,source_inputs_unchanged:true,server_business_success:false};
};
