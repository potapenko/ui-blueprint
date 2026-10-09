#!/usr/bin/env python3
"""Finite diagnostic replay of the actual Native Rust admission/receive owners."""
import argparse, difflib, hashlib, json, math, os
from pathlib import Path
import statistics, subprocess, tempfile

parser = argparse.ArgumentParser()
parser.add_argument('action', choices=['prepare', 'build', 'run'])
parser.add_argument('--root', type=Path, required=True)
parser.add_argument('--sample', type=Path)
parser.add_argument('--config', type=Path)
a = parser.parse_args()
r = a.root.resolve()
assert r.is_relative_to(Path(tempfile.gettempdir()).resolve())
# Root is the already-created Q02 archive/build owner, not another project.
repo = Path(__file__).resolve().parents[3]
pin = '770906798e1326fed3dd0edec40dc59b13772b3a'
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
if a.action == 'prepare':
    inputs = r/'native-stage-inputs'
    inputs.mkdir(mode=0o700)
    config = json.loads(a.config.read_text())
    sample = json.loads(a.sample.read_text())
    assert sample['terminal'] == 'Completed' and sample['committed'] == 3
    (inputs/'descriptor.json').write_text(json.dumps(config['descriptor']))
    for frame in sample['frames']:
        name = 'ax' if frame['slot'] == 0 else 'capture'
        (inputs/f'{name}.json').write_text(frame['canonical'])
    response = json.loads((inputs/'ax.json').read_text())['artifact']['data']
    request = {'schema_version':'0.1.0','artifact':{'kind':'request','data':{
        'clock_domain':'q02-native-saved','request_id':response['request_id'],
        'context':response['result']['data']['context'],
        'limits':{'max_elements':160,'max_depth':9,'max_output_bytes':524288,'deadline_ms':3000},
        'freshness_policy':'current_required','operation':{'operation':'observe','channels':['external_semantics','rendered_capture']}}}}
    (inputs/'request.json').write_text(json.dumps(request))
    rel = 'crates/host/src/worker_observation.rs'
    file = r/'source'/rel
    original = subprocess.check_output(['git','show',f'{pin}:{rel}'],cwd=repo).decode()
    assert file.read_text() == original
    amended = original + '\n' + (repo/'tests/bridges/native/rust-stages.rs').read_text()
    file.write_text(amended)
    delta = ''.join(difflib.unified_diff(original.splitlines(True),amended.splitlines(True),fromfile=rel,tofile=rel))
    (r/'native-stage-delta.diff').write_text(delta)
    manifest = {'source_pin':pin,'delta_sha256':digest(r/'native-stage-delta.diff'),
                'input_hashes':{p.name:digest(p) for p in inputs.iterdir()},
                'method':'test-only appendix; original production bodies unchanged; no platform calls'}
    (r/'native-stage-prepared.json').write_text(json.dumps(manifest,indent=2))
    print(json.dumps(manifest))
elif a.action == 'build':
    env = dict(os.environ,CARGO_TARGET_DIR=str(r/'target'))
    command = ['cargo','+1.96.0','test','--locked','--offline','--release','--no-run','--message-format=json',
               '-p','uiblueprint-host','--features','web','--bin','session-worker']
    result = subprocess.run(command,cwd=r/'source',env=env,capture_output=True,timeout=180)
    if result.returncode:
        print(result.stderr.decode()[-5000:]);raise SystemExit(result.returncode)
    products = [json.loads(line) for line in result.stdout.splitlines() if line.startswith(b'{')]
    binary = next(x['executable'] for x in products if x.get('reason')=='compiler-artifact' and x.get('executable') and x.get('profile',{}).get('test'))
    (r/'native-stage-binary.json').write_text(json.dumps({'path':binary,'sha256':digest(Path(binary))}))
    print(binary)
else:
    binary = json.loads((r/'native-stage-binary.json').read_text());assert digest(Path(binary['path'])) == binary['sha256']
    report = {'source_pin':pin,'binary':binary,'kind':'offline diagnostic, not a latency cohort','runs':[]}
    file = r/'native-stage-report.json'
    for cohort,n in [('process_fresh',1)]*20+[('reused_process',100)]:
        env = dict(os.environ,UIB_Q02_NATIVE_INPUTS=str(r/'native-stage-inputs'),UIB_Q02_NATIVE_REPEATS=str(n))
        result = subprocess.run([binary['path'],'--ignored','--exact','worker_ops::observation::q02_native_stages::saved_receive','--nocapture','--test-threads=1'],env=env,capture_output=True,timeout=30)
        rows = [json.loads(line.split('@Q02_NATIVE_STAGES ',1)[1]) for line in result.stdout.decode().splitlines() if '@Q02_NATIVE_STAGES ' in line]
        report['runs'].append({'cohort':cohort,'exit':result.returncode,'data':rows})
        file.write_text(json.dumps(report,indent=2))
        if result.returncode or len(rows)!=1 or len(rows[0]['samples'])!=n:
            print(result.stdout.decode()[-3000:],result.stderr.decode()[-3000:]);raise SystemExit(1)
    report['summary']=[]
    for cohort in ['process_fresh','reused_process']:
        samples=[s for run in report['runs'] if run['cohort']==cohort for s in run['data'][0]['samples']]
        for key in ['request_admission_ns','ax_receive_ns','capture_receive_ns','complete_ns']:
            values=sorted(s[key]/1e6 for s in samples)
            report['summary'].append({'cohort':cohort,'stage':key,'n':len(values),'p50_ms':statistics.median(values),'p95_ms':values[math.ceil(.95*len(values))-1]})
    file.write_text(json.dumps(report,indent=2));print(json.dumps(report['summary']))
