#include "lldb/API/SBBreakpoint.h"
#include "../include/lldb_c.h"

extern "C" {

void LLDB_SBBreakpoint_Destroy(SBBreakpointRef ref) {
    delete reinterpret_cast<lldb::SBBreakpoint*>(ref);
}

bool LLDB_SBBreakpoint_IsValid(SBBreakpointRef ref) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBBreakpoint*>(ref)->IsValid();
}

uint32_t LLDB_SBBreakpoint_GetID(SBBreakpointRef ref) {
    if (!ref) return 0;
    return static_cast<uint32_t>(
        reinterpret_cast<lldb::SBBreakpoint*>(ref)->GetID());
}

void LLDB_SBBreakpoint_SetEnabled(SBBreakpointRef ref, bool enable) {
    if (!ref) return;
    reinterpret_cast<lldb::SBBreakpoint*>(ref)->SetEnabled(enable);
}

bool LLDB_SBBreakpoint_IsEnabled(SBBreakpointRef ref) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBBreakpoint*>(ref)->IsEnabled();
}

bool LLDB_SBBreakpoint_IsOneShot(SBBreakpointRef ref) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBBreakpoint*>(ref)->IsOneShot();
}

void LLDB_SBBreakpoint_SetOneShot(SBBreakpointRef ref, bool one_shot) {
    if (!ref) return;
    reinterpret_cast<lldb::SBBreakpoint*>(ref)->SetOneShot(one_shot);
}

uint32_t LLDB_SBBreakpoint_GetHitCount(SBBreakpointRef ref) {
    if (!ref) return 0;
    return reinterpret_cast<lldb::SBBreakpoint*>(ref)->GetHitCount();
}

void LLDB_SBBreakpoint_ResetHitCount(SBBreakpointRef ref) {
    if (!ref) return;
    // SBBreakpoint doesn't expose ResetHitCount directly in the public API;
    // re-setting ignore count to 0 is the standard workaround.
    reinterpret_cast<lldb::SBBreakpoint*>(ref)->SetIgnoreCount(0);
}

void LLDB_SBBreakpoint_SetCondition(SBBreakpointRef ref, const char* condition) {
    if (!ref) return;
    reinterpret_cast<lldb::SBBreakpoint*>(ref)->SetCondition(condition);
}

const char* LLDB_SBBreakpoint_GetCondition(SBBreakpointRef ref) {
    if (!ref) return nullptr;
    return reinterpret_cast<lldb::SBBreakpoint*>(ref)->GetCondition();
}

void LLDB_SBBreakpoint_SetIgnoreCount(SBBreakpointRef ref, uint32_t count) {
    if (!ref) return;
    reinterpret_cast<lldb::SBBreakpoint*>(ref)->SetIgnoreCount(count);
}

uint32_t LLDB_SBBreakpoint_GetIgnoreCount(SBBreakpointRef ref) {
    if (!ref) return 0;
    return reinterpret_cast<lldb::SBBreakpoint*>(ref)->GetIgnoreCount();
}

uint32_t LLDB_SBBreakpoint_GetNumLocations(SBBreakpointRef ref) {
    if (!ref) return 0;
    return reinterpret_cast<lldb::SBBreakpoint*>(ref)->GetNumLocations();
}

} // extern "C"
