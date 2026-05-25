#include "lldb/API/SBFrame.h"
#include "lldb/API/SBValue.h"
#include "../include/lldb_c.h"

extern "C" {

void LLDB_SBFrame_Destroy(SBFrameRef ref) {
    delete reinterpret_cast<lldb::SBFrame*>(ref);
}

bool LLDB_SBFrame_IsValid(SBFrameRef ref) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBFrame*>(ref)->IsValid();
}

uint32_t LLDB_SBFrame_GetFrameID(SBFrameRef ref) {
    if (!ref) return 0;
    return reinterpret_cast<lldb::SBFrame*>(ref)->GetFrameID();
}

uint64_t LLDB_SBFrame_GetCFA(SBFrameRef ref) {
    if (!ref) return 0;
    return static_cast<uint64_t>(
        reinterpret_cast<lldb::SBFrame*>(ref)->GetCFA());
}

uint64_t LLDB_SBFrame_GetPC(SBFrameRef ref) {
    if (!ref) return 0;
    return static_cast<uint64_t>(
        reinterpret_cast<lldb::SBFrame*>(ref)->GetPC());
}

bool LLDB_SBFrame_SetPC(SBFrameRef ref, uint64_t new_pc) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBFrame*>(ref)->SetPC(
        static_cast<lldb::addr_t>(new_pc));
}

uint64_t LLDB_SBFrame_GetSP(SBFrameRef ref) {
    if (!ref) return 0;
    return static_cast<uint64_t>(
        reinterpret_cast<lldb::SBFrame*>(ref)->GetSP());
}

uint64_t LLDB_SBFrame_GetFP(SBFrameRef ref) {
    if (!ref) return 0;
    return static_cast<uint64_t>(
        reinterpret_cast<lldb::SBFrame*>(ref)->GetFP());
}

const char* LLDB_SBFrame_GetFunctionName(SBFrameRef ref) {
    if (!ref) return nullptr;
    return reinterpret_cast<lldb::SBFrame*>(ref)->GetFunctionName();
}

bool LLDB_SBFrame_IsInlined(SBFrameRef ref) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBFrame*>(ref)->IsInlined();
}

bool LLDB_SBFrame_IsArtificial(SBFrameRef ref) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBFrame*>(ref)->IsArtificial();
}

SBValueRef LLDB_SBFrame_FindVariable(SBFrameRef ref, const char* name) {
    if (!ref) return nullptr;
    lldb::SBValue v = reinterpret_cast<lldb::SBFrame*>(ref)->FindVariable(name);
    if (!v.IsValid()) return nullptr;
    return reinterpret_cast<SBValueRef>(new lldb::SBValue(v));
}

SBValueRef LLDB_SBFrame_EvaluateExpression(SBFrameRef ref, const char* expr) {
    if (!ref) return nullptr;
    lldb::SBValue v = reinterpret_cast<lldb::SBFrame*>(ref)->EvaluateExpression(expr);
    if (!v.IsValid()) return nullptr;
    return reinterpret_cast<SBValueRef>(new lldb::SBValue(v));
}

} // extern "C"
