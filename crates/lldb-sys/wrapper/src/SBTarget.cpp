#include "lldb/API/SBTarget.h"
#include "lldb/API/SBProcess.h"
#include "lldb/API/SBBreakpoint.h"
#include "lldb/API/SBListener.h"
#include "lldb/API/SBError.h"
#include "../include/lldb_c.h"

extern "C" {

void LLDB_SBTarget_Destroy(SBTargetRef ref) {
    delete reinterpret_cast<lldb::SBTarget*>(ref);
}

bool LLDB_SBTarget_IsValid(SBTargetRef ref) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBTarget*>(ref)->IsValid();
}

SBProcessRef LLDB_SBTarget_LaunchSimple(SBTargetRef  ref,
                                         const char** argv,
                                         const char** envp,
                                         const char*  working_dir) {
    if (!ref) return nullptr;
    auto* target = reinterpret_cast<lldb::SBTarget*>(ref);
    lldb::SBProcess proc = target->LaunchSimple(argv, envp, working_dir);
    if (!proc.IsValid()) return nullptr;
    return reinterpret_cast<SBProcessRef>(new lldb::SBProcess(proc));
}

SBProcessRef LLDB_SBTarget_Launch(SBTargetRef   ref,
                                    SBListenerRef listener_ref,
                                    const char**  argv,
                                    const char**  envp,
                                    const char*   stdin_path,
                                    const char*   stdout_path,
                                    const char*   stderr_path,
                                    const char*   working_dir,
                                    uint32_t      launch_flags,
                                    bool          stop_at_entry,
                                    SBErrorRef    error_ref) {
    if (!ref) return nullptr;
    auto* target = reinterpret_cast<lldb::SBTarget*>(ref);

    lldb::SBListener default_listener;
    lldb::SBListener* listener = listener_ref
        ? reinterpret_cast<lldb::SBListener*>(listener_ref)
        : &default_listener;

    lldb::SBError local_err;
    lldb::SBError* err = error_ref
        ? reinterpret_cast<lldb::SBError*>(error_ref)
        : &local_err;

    lldb::SBProcess proc = target->Launch(*listener, argv, envp,
                                           stdin_path, stdout_path, stderr_path,
                                           working_dir, launch_flags,
                                           stop_at_entry, *err);
    if (!proc.IsValid()) return nullptr;
    return reinterpret_cast<SBProcessRef>(new lldb::SBProcess(proc));
}

SBProcessRef LLDB_SBTarget_AttachToProcessWithID(SBTargetRef   ref,
                                                   SBListenerRef listener_ref,
                                                   uint64_t      pid,
                                                   SBErrorRef    error_ref) {
    if (!ref) return nullptr;
    auto* target = reinterpret_cast<lldb::SBTarget*>(ref);

    lldb::SBListener default_listener;
    lldb::SBListener* listener = listener_ref
        ? reinterpret_cast<lldb::SBListener*>(listener_ref)
        : &default_listener;

    lldb::SBError local_err;
    lldb::SBError* err = error_ref
        ? reinterpret_cast<lldb::SBError*>(error_ref)
        : &local_err;

    lldb::SBProcess proc = target->AttachToProcessWithID(*listener,
                                                           static_cast<lldb::pid_t>(pid),
                                                           *err);
    if (!proc.IsValid()) return nullptr;
    return reinterpret_cast<SBProcessRef>(new lldb::SBProcess(proc));
}

SBProcessRef LLDB_SBTarget_GetProcess(SBTargetRef ref) {
    if (!ref) return nullptr;
    lldb::SBProcess proc = reinterpret_cast<lldb::SBTarget*>(ref)->GetProcess();
    if (!proc.IsValid()) return nullptr;
    return reinterpret_cast<SBProcessRef>(new lldb::SBProcess(proc));
}

SBBreakpointRef LLDB_SBTarget_BreakpointCreateByName(SBTargetRef ref,
                                                       const char* name,
                                                       const char* module_name) {
    if (!ref) return nullptr;
    auto* target = reinterpret_cast<lldb::SBTarget*>(ref);
    lldb::SBBreakpoint bp = target->BreakpointCreateByName(name, module_name);
    if (!bp.IsValid()) return nullptr;
    return reinterpret_cast<SBBreakpointRef>(new lldb::SBBreakpoint(bp));
}

SBBreakpointRef LLDB_SBTarget_BreakpointCreateByLocation(SBTargetRef ref,
                                                           const char* file,
                                                           uint32_t    line) {
    if (!ref) return nullptr;
    auto* target = reinterpret_cast<lldb::SBTarget*>(ref);
    lldb::SBBreakpoint bp = target->BreakpointCreateByLocation(file, line);
    if (!bp.IsValid()) return nullptr;
    return reinterpret_cast<SBBreakpointRef>(new lldb::SBBreakpoint(bp));
}

SBBreakpointRef LLDB_SBTarget_BreakpointCreateByAddress(SBTargetRef ref, uint64_t address) {
    if (!ref) return nullptr;
    auto* target = reinterpret_cast<lldb::SBTarget*>(ref);
    lldb::SBBreakpoint bp = target->BreakpointCreateByAddress(
        static_cast<lldb::addr_t>(address));
    if (!bp.IsValid()) return nullptr;
    return reinterpret_cast<SBBreakpointRef>(new lldb::SBBreakpoint(bp));
}

bool LLDB_SBTarget_DeleteBreakpoint(SBTargetRef ref, uint32_t break_id) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBTarget*>(ref)->BreakpointDelete(
        static_cast<lldb::break_id_t>(break_id));
}

uint32_t LLDB_SBTarget_GetNumBreakpoints(SBTargetRef ref) {
    if (!ref) return 0;
    return reinterpret_cast<lldb::SBTarget*>(ref)->GetNumBreakpoints();
}

SBBreakpointRef LLDB_SBTarget_GetBreakpointAtIndex(SBTargetRef ref, uint32_t idx) {
    if (!ref) return nullptr;
    lldb::SBBreakpoint bp = reinterpret_cast<lldb::SBTarget*>(ref)->GetBreakpointAtIndex(idx);
    if (!bp.IsValid()) return nullptr;
    return reinterpret_cast<SBBreakpointRef>(new lldb::SBBreakpoint(bp));
}

const char* LLDB_SBTarget_GetTriple(SBTargetRef ref) {
    if (!ref) return nullptr;
    return reinterpret_cast<lldb::SBTarget*>(ref)->GetTriple();
}

} // extern "C"
