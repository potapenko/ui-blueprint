"""Fixed G2 component qualification, loaded by its sealed interactive LLDB.
No target selection expressions or generic debugger commands are accepted.
"""
import ctypes, hashlib, json, math, os, re, socket, subprocess, time, uuid, tempfile, importlib.util
from pathlib import Path
import lldb
D=Path(__file__).resolve().parent
group_spec=importlib.util.spec_from_file_location('s14_fixed_group',D/'group_value.py')
group_module=importlib.util.module_from_spec(group_spec)
group_spec.loader.exec_module(group_module)
read_group=group_module.read_group
profile=json.loads((D/'profile.json').read_text())
EVIDENCE=Path(profile['evidence'])
RUN_KIND=RUN_KIND
fixtureMode=RUN_KIND.startswith('fixture')
groupMode=RUN_KIND.endswith('-group')
debug=Path('/Applications/Xcode.app/Contents/Developer/usr/bin/lldb')
peerbin=D/'debugserver'
executable=Path('/private/tmp/playphraseme-guest-tour-runtime-axlhvjj9/PlayPhraseMe.app/Contents/MacOS/PlayPhraseMe')
dylib=executable.with_name('PlayPhraseMe.debug.dylib')
fixture=D/('GroupControl' if groupMode else 'Control')
expected={Path(k):v for k,v in profile['pins'].items()}
model=fixture if fixtureMode else dylib
module_uuid=profile['group_fixture_uuid' if groupMode else 'fixture_uuid'] if fixtureMode else '3456EB33-6C0E-393F-8486-10961C55A994'
cards=set('language actor commonPhrases vocabulary favorites grammar statistics previous playPause next speed repeatPlayback focus context search grammarSearch clearSearch favoriteSearch randomPhrase randomWord randomFavorite practice fullscreen reels'.split())
lib=ctypes.CDLL('/usr/lib/libproc.dylib')
peer=proc=target=bp=child=None
target_pid=None
contacted=False
managed_id=None
detach_attempted=False
owned_ids=set()
def require(ok,reason):
    if not ok:raise RuntimeError(reason)
def pin(path):
    with path.open('rb') as f:
        require(hashlib.sha256(f.read()).hexdigest()==expected[path],'disk_hash_refused')
def image(pid):
    buf=ctypes.create_string_buffer(4096)
    require(lib.proc_pidpath(pid,buf,len(buf))>0,'live_image_unavailable')
    return Path(os.fsdecode(buf.value)).resolve()
def mapped(pid,path):
    pin(path)
    require(image(pid)==path.resolve(),'live_image_path_refused')
    rows=subprocess.check_output(['/usr/sbin/lsof','-a','-p',str(pid),'-d','txt','-F','pDint'],timeout=2,text=True).splitlines()
    files=[];record={}
    for row in rows:
        if row.startswith('p'):continue
        if row.startswith('f'):
            files.append(record);record={}
        else:record[row[:1]]=row[1:]
    files.append(record); st=path.stat()
    require(any(r.get('i')==str(st.st_ino) and int(r.get('D','0'),0)==st.st_dev and Path(r.get('n','/')).resolve()==path.resolve() for r in files),'mapped_file_identity_refused')
def peerOK():
    require(peer is not None and peer.poll() is None,'peer_exited');mapped(peer.pid,peerbin)
def gui_identity():
    pin(D/'identity')
    result=subprocess.run([D/'identity'],capture_output=True,timeout=2)
    require(result.returncode==0,'owner_tuple_refused')
    obj=json.loads(result.stdout)
    require(set(obj)=={'pid','birth','window_id','bundle','window_screen_pt'} and obj['pid']==32187
            and obj['birth']==1795448825.752163 and obj['window_id']==1060 and obj['bundle']=='me.playphraseme.reference.guesttour.01a20845.g2','owner_identity_report_refused')
    return obj
def own():
    peerOK()
    if fixtureMode: require(image(child.pid)==fixture,'fixture_identity_refused')
    else:
        gui_identity()
        require(image(32187)==executable,'gui_executable_identity_refused')
def cmd(command):
    r=lldb.SBCommandReturnObject();lldb.debugger.GetCommandInterpreter().HandleCommand(command,r)
    require(r.Succeeded(),'fixed_lldb_command_refused');return r.GetOutput() or ''
def waitstate(state,seconds,min_stop=-1):
    until=time.monotonic()+seconds
    while time.monotonic()<until:
        if proc.GetState()==state and (min_stop<0 or proc.GetStopID()>min_stop):return True
        time.sleep(.02)
    return False
def drop_owned():
    for bid in list(owned_ids):
        active=target.FindBreakpointByID(bid)
        if active.IsValid():
            require(active.GetID()==bid and active.IsHardware(),'manual_recovery_required_bkpt_ownership')
            require(target.BreakpointDelete(bid),'manual_recovery_required_bkpt_delete')
        owned_ids.remove(bid)
def detach_owned():
    global detach_attempted
    require(not detach_attempted,'prior_detach_attempt_no_automatic_retry')
    require(proc.GetState()==lldb.eStateStopped and proc.GetProcessID()==target_pid,'manual_recovery_required_not_stopped')
    own()
    drop_owned()
    detach_attempted=True
    require(proc.Detach().Success() and waitstate(lldb.eStateDetached,2),'detach_failed_or_not_confirmed')
    if fixtureMode:
        (fixtureCase/'finish').touch();require(child.wait(timeout=4)==0 and 'recovered' in (fixtureCase/'stdout').read_text(),'fixture_recovery_unconfirmed')
    else:
        post_deadline=time.monotonic()+2
        consecutive=0
        while time.monotonic()<post_deadline:
            current=gui_identity()
            observed=subprocess.run(['/bin/ps','-p',str(target_pid),'-o','stat='],text=True,capture_output=True,timeout=1)
            state=observed.stdout.strip()
            consecutive=consecutive+1 if observed.returncode==0 and state[:1] in ('R','S','I','U') else 0
            if consecutive>=2:
                globals()['after_gui']=current;break
            time.sleep(.10)
        require(consecutive>=2,'detached_but_runnable_state_unconfirmed')
    print('DETACHED_AND_OWNER_ALIVE',flush=True)

def owner_recover_stopped():
    require(contacted and os.isatty(0),'not_original_interactive_session')
    detach_owned()


def sections(raw):
    require(len(raw)<8192,'local_text_limit')
    parts=re.split(r'(?m)^(?=\()',raw.strip())
    require(parts and parts[0]=='','local_sections_refused')
    return [x.strip() for x in parts[1:]]


def scalar_shape(name,typ,text):
    leaf=r'native\s*=\s*\{\s*_value\s*=\s*([-+0-9.eE]+)\s*\}'
    def child(key,body):return re.escape(key)+r'\s*=\s*\{\s*'+body+r'\s*\}'
    point=child('x',leaf)+r'\s*'+child('y',leaf)
    size=child('width',leaf)+r'\s*'+child('height',leaf)
    body={'CGPoint':point,'CGRect':child('origin',point)+r'\s*'+child('size',size),'CGFloat':leaf}[typ]
    match=re.fullmatch(r'\((?:\w+\.)*'+typ+r'\)\s+'+child(name,body),text)
    require(match is not None,'scalar_shape_refused')
    values=[float(x) for x in match.groups()]
    require(all(math.isfinite(x) for x in values),'nonfinite_scalar')
    if typ=='CGRect':require(min(values[2:])>=0,'negative_size')
    return values


def read_card_id(frame):
    name=('source.card' if fixtureMode else 'self.card') if groupMode else 'card'
    if groupMode:
        value=frame.FindVariable('source' if fixtureMode else 'self',lldb.eNoDynamicValues).GetNonSyntheticValue().GetChildMemberWithName('card',lldb.eNoDynamicValues).GetNonSyntheticValue()
    else:value=frame.FindVariable('card',lldb.eNoDynamicValues)
    require(value.IsValid() and value.GetError().Success() and value.GetTypeName().split('.')[-1] in ('Card','PlayphrasemeGuestTourCard'),'card_type_refused')
    raw=cmd('frame variable -d no-dynamic-values -R -S false -- '+name)
    match=re.fullmatch(r'\([\w.]+\)\s+'+re.escape(name)+r'\s*=\s*([A-Za-z][A-Za-z0-9_]*)\s*\{\s*\}',raw.strip())
    require(match is not None and match[1] in cards,'card_value_refused')
    return match[1]


def read_fields(frame,fields):
    for name,typ in fields:
        value=frame.FindVariable(name,lldb.eNoDynamicValues)
        require(value.IsValid() and value.GetError().Success() and value.GetTypeName().split('.')[-1]==typ,'field_type_refused')
    raw=cmd('frame variable -d no-dynamic-values -R -S false -- '+' '.join(name for name,_ in fields))
    parts=sections(raw)
    require(len(parts)==len(fields),'field_count_refused')
    return {name:scalar_shape(name,typ,text) for (name,typ),text in zip(fields,parts)}


def site(kind):
    if kind=='group':
        return (Path(profile['group_fixture_source']),28,None,None) if fixtureMode else (Path(profile['source_directory'])/'PlayphrasemeGuestTourAnnotation.swift',104,9,0x1149998)
    source=profile['fixture_source'] if fixtureMode else profile['source_directory']+'/'+('PlayphrasemeGuestTourMap.swift' if kind=='map' else 'PlayphrasemeGuestTourConnections.swift')
    line,column,address=({'map':(10,None,None),'origin':(13,None,None),'card':(19,None,None)} if fixtureMode else
        {'map':(23,13,0x116de4c),'origin':(25,13,0x1169a0c),'card':(29,28,0x116a4d0)})[kind]
    return Path(source),line,column,address


def install(kind,live=True):
    global bp
    require(not owned_ids,'prior_owned_breakpoint_remains')
    source,line,column,address=site(kind)
    cmd('breakpoint set --hardware --shlib '+model.name+' --file '+source.name+' --line '+str(line)+
        ('' if column is None else ' --column '+str(column))+' --move-to-nearest-code false')
    bp=target.GetBreakpointAtIndex(target.GetNumBreakpoints()-1)
    owned_ids.add(bp.GetID())
    require(bp.IsValid() and bp.IsHardware() and bp.GetNumLocations()==1,'site_ambiguous')
    loc=bp.GetLocationAtIndex(0);ad=loc.GetAddress();le=ad.GetLineEntry()
    require(ad.GetModule().GetUUIDString().upper()==module_uuid and Path(le.GetFileSpec().fullpath).resolve()==source.resolve()
            and le.GetLine()==line and (column is None or le.GetColumn()==column) and (address is None or ad.GetFileAddress()==address),'site_pin_refused')
    if live:
        require(loc.IsResolved() and loc.IsEnabled(),'site_unresolved')
        detail=cmd('breakpoint list --verbose '+str(bp.GetID())).split(str(bp.GetID())+'.1:',1)
        require(len(detail)==2 and re.search(r'(?m)^\s*resolved = true\s*$',detail[1]) and re.search(r'(?m)^\s*hardware = true\s*$',detail[1]),'installed_hardware_unconfirmed')
    return ad.GetLoadAddress(target)


def interrupt_for_timeout():
    require(proc.GetProcessID()==target_pid,'changed_target')
    own()
    if proc.GetState()!=lldb.eStateStopped:
        proc.SendAsyncInterrupt()
        require(waitstate(lldb.eStateStopped,2),'timeout_interrupt_unconfirmed')
    reasons=[th for th in proc if th.GetStopReason() not in (lldb.eStopReasonNone,lldb.eStopReasonInvalid)]
    require(reasons and all(th.GetStopReason()==lldb.eStopReasonSignal and th.GetStopReasonDataAtIndex(0) in (2,17) for th in reasons),'timeout_unexpected_stop_retained')
    detach_owned()
    print('NO_SELECTED_COMPONENT; detached without geometry publication',flush=True)


def run():
    global peer,proc,target,child,target_pid,contacted,before_gui,fixtureCase
    require(RUN_KIND in ('offline','owner','fixture','fixture-nohit','fixture-missing','fixture-absent','owner-group','fixture-group'),'invalid_mode')
    require(os.isatty(0) and os.isatty(1),'interactive_owner_pty_required')
    require(profile['card'] in cards and type(profile['window_frame']) is list and len(profile['window_frame'])==4,'profile_refused')
    for path in expected:pin(path)
    mapped(os.getpid(),debug)
    dbg=lldb.debugger
    for setting in ('plugin.process.gdb-remote.packet-timeout 3','stop-disassembly-display never','frame-format stopped-frame',
                    'stop-line-count-before 0','stop-line-count-after 0','target.require-hardware-breakpoint true',
                    'target.load-script-from-symbol-file false','target.process.stop-on-sharedlibrary-events false'):
        cmd('settings set '+setting)
    dbg.SetAsync(False)
    target=dbg.CreateTarget(str(fixture if fixtureMode else (dylib if RUN_KIND=='offline' else executable)))
    require(target.IsValid(),'target_creation_refused')
    if RUN_KIND=='offline':
        for kind in ('map','origin','card','group'):
            install(kind,False);drop_owned()
        print('OFFLINE_THREE_SOURCE_SITES_VERIFIED; no process selected',flush=True)
        return
    left,right=socket.socketpair()
    peer=subprocess.Popen([peerbin,'--fd='+str(right.fileno())],pass_fds=(right.fileno(),),stdin=subprocess.DEVNULL,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
    right.close();peerOK()
    err=lldb.SBError();proc=target.ConnectRemote(dbg.GetListener(),'fd://'+str(left.detach()),'gdb-remote',err)
    require(err.Success() and waitstate(lldb.eStateConnected,3),'idle_peer_connection_refused');peerOK()
    if fixtureMode:
        fixtureCase=Path(tempfile.mkdtemp(prefix=RUN_KIND+'-',dir=D/'tmp'))
        child=subprocess.Popen([fixture],cwd=fixtureCase,stdout=(fixtureCase/'stdout').open('wb'),stderr=subprocess.DEVNULL)
        target_pid=child.pid
    else:
        target_pid=32187
        before_gui=gui_identity()
        require(before_gui['window_screen_pt']==profile['window_frame'],'stationary_baseline_changed')
    own()
    if not fixtureMode:
        with (D/'owner-attempt.consumed').open('x') as f:f.write('one approved component attempt\n')
    contacted=True
    require(proc.RemoteAttachToProcessWithID(target_pid,err) and err.Success(),'attach_refused')
    require(waitstate(lldb.eStateStopped,4) and proc.GetProcessID()==target_pid,'attach_state_unconfirmed')
    own()
    if not fixtureMode:require(target.GetModuleAtIndex(0).GetUUIDString().upper()=='5CB8B83E-88FB-39EB-A162-36486FCD81D3','launcher_uuid_changed')
    require('arm64' in target.GetTriple() and target.GetNumWatchpoints()==0 and target.GetNumBreakpoints()==0,'unexpected_debug_state')
    kind='group' if groupMode else 'map';pc=install(kind);records=[];selected='grammar' if RUN_KIND=='fixture-absent' else profile['card']
    dbg.SetAsync(True);prior=proc.GetStopID()
    require(proc.Continue().Success(),'continue_refused')
    if fixtureMode:
        if RUN_KIND!='fixture-nohit':(fixtureCase/'trigger').write_text('baseline')
    else:
        require(waitstate(lldb.eStateRunning,1),'running_not_confirmed')
        print('ARMED_RUNNING: open Tour once in this stationary window',flush=True)
    deadline=time.monotonic()+(0.6 if RUN_KIND in ('fixture-nohit','fixture-absent') else 20)
    for hit in range(64):
        remaining=deadline-time.monotonic()
        if remaining<=0 or not waitstate(lldb.eStateStopped,remaining,prior):
            interrupt_for_timeout();return
        own()
        if not fixtureMode:require(gui_identity()['window_screen_pt']==before_gui['window_screen_pt'],'window_moved_during_acquisition')
        th=next((t for t in proc if t.GetStopReason()==lldb.eStopReasonBreakpoint and t.GetStopReasonDataCount()>=2 and t.GetStopReasonDataAtIndex(0)==bp.GetID()),None)
        require(th is not None,'unexpected_stop_retained')
        frame=th.GetFrameAtIndex(0)
        require(frame.GetPC()==pc and frame.GetModule().GetUUIDString().upper()==module_uuid,'foreign_hit')
        proc.SetSelectedThread(th);th.SetSelectedFrame(0)
        started=time.monotonic_ns()
        fields=None
        if kind=='group':
            if read_card_id(frame)==selected:
                values=read_group(frame,proc,cmd,lldb,fixture=fixtureMode)
                record={'kind':'group','observation_id':uuid.uuid4().hex,'read_start':started,'read_end':time.monotonic_ns(),'values':values}
                detach_owned()
                if not fixtureMode:require(after_gui['window_screen_pt']==before_gui['window_screen_pt'],'window_changed_after_detach')
                envelope={'format':'playphraseme.guest-tour.group-1','card':selected,'module_uuid':module_uuid,
                    'source_revision':'synthetic-owned-fixture' if fixtureMode else 'e19b20c474afe6462786ca0dd3dd5ad9ae37dbce','records':[record],
                    'time_unit':'ns','clock_domain':'owner-lldb-'+uuid.uuid4().hex,'detached_verified':True,'stationary_window':not fixtureMode,
                    'before':None if fixtureMode else before_gui,'after':None if fixtureMode else after_gui,'fixture':fixtureMode}
                if not fixtureMode:
                    fd=os.open(EVIDENCE/'group.json',os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o600)
                    with os.fdopen(fd,'w') as f:json.dump(envelope,f,allow_nan=False)
                print('GROUP_MEMBERSHIP '+json.dumps(envelope,allow_nan=False),flush=True)
                return
        elif kind=='map':fields=[('origin','CGPoint'),('viewport','CGRect'),('scale','CGFloat')]
        elif kind=='origin':fields=[('origin','CGPoint')]
        elif kind=='card' and read_card_id(frame)==selected:fields=[('target','CGRect'),('rect','CGRect'),('start','CGPoint')]
        if fields:
            if RUN_KIND=='fixture-missing':fields=[('missing','CGRect')]
            values=read_fields(frame,fields)
            records.append({'kind':kind,'observation_id':uuid.uuid4().hex,'read_start':started,'read_end':time.monotonic_ns(),'values':values})
            if kind=='card':
                t=values['target'];require(values['start']==[t[0]+t[2]/2,t[1]+t[3]/2],'midpoint_mismatch')
                detach_owned()
                if not fixtureMode:require(after_gui['window_screen_pt']==before_gui['window_screen_pt'],'window_changed_after_detach')
                envelope={'format':'playphraseme.guest-tour.component-1','card':selected,'module_uuid':module_uuid,
                    'source_revision':'synthetic-owned-fixture' if fixtureMode else 'e19b20c474afe6462786ca0dd3dd5ad9ae37dbce','records':records,'units':'pt','time_unit':'ns',
                    'clock_domain':'owner-lldb-'+uuid.uuid4().hex,'detached_verified':True,'stationary_window':not fixtureMode,
                    'before':None if fixtureMode else before_gui,'after':None if fixtureMode else after_gui,
                    'fixture':fixtureMode,'consistency':'sequential-source-reads-not-presented-frame','groups':'not_collected'}
                if not fixtureMode:
                    fd=os.open(EVIDENCE/'component.json',os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o600)
                    with os.fdopen(fd,'w') as f:json.dump(envelope,f,allow_nan=False)
                print('COMPONENT_GEOMETRY '+json.dumps(envelope,allow_nan=False),flush=True)
                return
            drop_owned();kind='origin' if kind=='map' else 'card';pc=install(kind)
        prior=proc.GetStopID();require(proc.Continue().Success(),'continue_refused')
    raise RuntimeError('hit_limit_retained')

try:run()
except BaseException as error:
    print('REFUSED_OR_RECOVERY_REQUIRED',str(error) if isinstance(error,RuntimeError) else type(error).__name__,flush=True)
    if contacted:
        print('OPERATOR_CONTROL_RETAINED; no automatic retry, detach or debugger teardown; detach_attempted='+str(detach_attempted),flush=True)
    elif peer is not None and peer.poll() is None:
        peer.terminate()
        try:peer.wait(timeout=2)
        except subprocess.TimeoutExpired:pass
