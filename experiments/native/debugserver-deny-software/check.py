"""Offline regression. Run: python3 check.py /path/to/llvm-project.

The checkout must contain the pinned commit. Its working tree is never modified.
Build the experiment separately with the upstream debugserver.xcodeproj scheme
debugserver, configuration Release, macosx SDK and CODE_SIGNING_ALLOWED=NO.
Apply both patches before building. A plain clang fixture refused task access.
Ordinary Xcode Debug fixtures with get-task-allow allowed stock and restricted
attach. A mixed Swift/C fixture rejected a data-sentinel write, observed intact
app text with hardware slots exhausted, detached and exited normally. These
checks qualify the fixture only; they do not establish whole-process immutability.
"""
from pathlib import Path
import subprocess
import sys
from tempfile import TemporaryDirectory

PIN = "8d69c36c8a43d465d9cbee82469d5c5e94b92c22"
FILE = "lldb/tools/debugserver/source/MacOSX/MachProcess.mm"
VM_FILE = "lldb/tools/debugserver/source/MacOSX/MachVMMemory.cpp"
HERE = Path(__file__).resolve().parent

# Compile the actual patched functions against a process-free stub. No API
# for executable memory access exists on the mock, even on slot exhaustion.
STUB = r"""
#include <cassert>
#include <cstddef>
using task_t = unsigned int;
using nub_addr_t = unsigned long;
using nub_size_t = unsigned long;
#define DNBLogThreadedIf(...) ((void)0)
#define DNBLogWarning(...) ((void)0)
struct MachVMMemory {
  nub_size_t Write(task_t,nub_addr_t,const void*,nub_size_t);
  nub_size_t WriteRegion(task_t,nub_addr_t,const void*,nub_size_t);
};
struct DNBBreakpoint {
  bool enabled=false, preferred=false;
  int index=-1, refs=1;
  void Retain() {++refs;}
  int Release() {return --refs;}
  bool IsEnabled() const {return enabled;}
  bool HardwarePreferred() const {return preferred;}
  bool IsHardware() const {return index>=0;}
  void SetHardwareIndex(int v) {index=v;}
  void SetEnabled(bool v) {enabled=v;}
};
struct Registry {
  DNBBreakpoint item;
  bool present=false;
  DNBBreakpoint *FindByAddress(nub_addr_t) {return present ? &item : nullptr;}
  DNBBreakpoint *Add(nub_addr_t,nub_size_t,bool hardware) {
    item=DNBBreakpoint{};item.preferred=hardware;present=true;return &item;
  }
  void Remove(nub_addr_t) {present=false;}
};
struct MachProcess {
  Registry m_breakpoints;
  struct Slots {
    int next=-1, calls=0;
    int EnableHardwareBreakpoint(DNBBreakpoint*) {++calls;return next;}
  } m_thread_list;
  DNBBreakpoint *CreateBreakpoint(nub_addr_t,nub_size_t,bool);
  bool EnableBreakpoint(nub_addr_t);
};
"""
CASES = r"""
int main() {
  MachVMMemory vm;
  unsigned int sentinel=0x1357, replacement=0x2468;
  auto address=reinterpret_cast<nub_addr_t>(&sentinel);
  assert(vm.Write(1,address,&replacement,sizeof replacement)==0);
  assert(vm.WriteRegion(1,address,&replacement,sizeof replacement)==0);
  assert(sentinel==0x1357);
  MachProcess p;
  assert(!p.CreateBreakpoint(1,4,false));
  assert(!p.m_breakpoints.present && !p.m_thread_list.calls);
  assert(!p.CreateBreakpoint(1,4,true));
  assert(!p.m_breakpoints.present && p.m_thread_list.calls==1);
  p.m_thread_list.next=0;
  assert(p.CreateBreakpoint(1,4,true));
  assert(p.m_breakpoints.item.enabled && p.m_breakpoints.item.IsHardware());
  assert(!p.CreateBreakpoint(1,4,false));  // Even at an existing hardware site.
  assert(p.m_breakpoints.item.refs==1);
}
"""

def main(checkout):
    with TemporaryDirectory() as tmp:
        root = Path(tmp)
        functions = []
        inputs = [
            (FILE, "deny-software.patch", ["DNBBreakpoint *MachProcess::CreateBreakpoint(",
                                           "bool MachProcess::EnableBreakpoint("]),
            (VM_FILE, "deny-vm-writes.patch", ["nub_size_t MachVMMemory::Write(",
                                              "nub_size_t MachVMMemory::WriteRegion("])
        ]
        for name, patch, signatures in inputs:
            path = root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(subprocess.check_output(["git", "-C", checkout, "show", f"{PIN}:{name}"]))
            subprocess.run(["git", "apply", str(HERE / patch)], cwd=root, check=True)
            source = path.read_text()
            for signature in signatures:
                start = source.index(signature)
                functions.append(source[start:source.index("\n}", start) + 2])
        test = root / "test.cpp"
        test.write_text(STUB + "\n".join(functions) + CASES)
        executable = root / "check"
        subprocess.run(["c++", "-std=c++17", str(test), "-o", str(executable)], check=True)
        subprocess.run([str(executable)], check=True)
    print("Passed explicit denial, unavailable slot, available slot and denial at existing hardware site; both VM write functions denied sentinel mutation.")


if __name__ == "__main__":
    main(sys.argv[1])
