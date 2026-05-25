#include "lldb/API/SBDebugger.h"
#include "lldb/API/SBTarget.h"
#include "lldb/API/SBError.h"
#include "../include/lldb_c.h"

extern "C" {

void LLDB_SBDebugger_Initialize(void) {
    lldb::SBDebugger::Initialize();
}

void LLDB_SBDebugger_Terminate(void) {
    lldb::SBDebugger::Terminate();
}

SBDebuggerRef LLDB_SBDebugger_Create(bool async) {
    lldb::SBDebugger dbg = lldb::SBDebugger::Create(async);
    return reinterpret_cast<SBDebuggerRef>(new lldb::SBDebugger(dbg));
}

void LLDB_SBDebugger_Destroy(SBDebuggerRef ref) {
    if (!ref) return;
    auto* dbg = reinterpret_cast<lldb::SBDebugger*>(ref);
    lldb::SBDebugger::Destroy(*dbg);
    delete dbg;
}

bool LLDB_SBDebugger_IsValid(SBDebuggerRef ref) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBDebugger*>(ref)->IsValid();
}

void LLDB_SBDebugger_SetAsync(SBDebuggerRef ref, bool async) {
    if (!ref) return;
    reinterpret_cast<lldb::SBDebugger*>(ref)->SetAsync(async);
}

bool LLDB_SBDebugger_GetAsync(SBDebuggerRef ref) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBDebugger*>(ref)->GetAsync();
}

SBTargetRef LLDB_SBDebugger_CreateTarget(SBDebuggerRef ref,
                                          const char*   filename,
                                          const char*   triple,
                                          const char*   platform,
                                          bool          add_deps,
                                          SBErrorRef    error_ref) {
    if (!ref) return nullptr;
    auto* dbg = reinterpret_cast<lldb::SBDebugger*>(ref);
    lldb::SBError local_err;
    lldb::SBError* err = error_ref
        ? reinterpret_cast<lldb::SBError*>(error_ref)
        : &local_err;

    lldb::SBTarget target = dbg->CreateTarget(filename, triple, platform, add_deps, *err);
    if (!target.IsValid()) return nullptr;
    return reinterpret_cast<SBTargetRef>(new lldb::SBTarget(target));
}

SBTargetRef LLDB_SBDebugger_CreateTargetSimple(SBDebuggerRef ref, const char* filename) {
    if (!ref) return nullptr;
    auto* dbg = reinterpret_cast<lldb::SBDebugger*>(ref);
    lldb::SBTarget target = dbg->CreateTarget(filename);
    if (!target.IsValid()) return nullptr;
    return reinterpret_cast<SBTargetRef>(new lldb::SBTarget(target));
}

uint32_t LLDB_SBDebugger_GetNumTargets(SBDebuggerRef ref) {
    if (!ref) return 0;
    return reinterpret_cast<lldb::SBDebugger*>(ref)->GetNumTargets();
}

SBTargetRef LLDB_SBDebugger_GetTargetAtIndex(SBDebuggerRef ref, uint32_t idx) {
    if (!ref) return nullptr;
    lldb::SBTarget t = reinterpret_cast<lldb::SBDebugger*>(ref)->GetTargetAtIndex(idx);
    if (!t.IsValid()) return nullptr;
    return reinterpret_cast<SBTargetRef>(new lldb::SBTarget(t));
}

SBTargetRef LLDB_SBDebugger_GetSelectedTarget(SBDebuggerRef ref) {
    if (!ref) return nullptr;
    lldb::SBTarget t = reinterpret_cast<lldb::SBDebugger*>(ref)->GetSelectedTarget();
    if (!t.IsValid()) return nullptr;
    return reinterpret_cast<SBTargetRef>(new lldb::SBTarget(t));
}

const char* LLDB_SBDebugger_GetVersionString(void) {
    return lldb::SBDebugger::GetVersionString();
}

} // extern "C"
