#include "lldb/API/SBThread.h"
#include "lldb/API/SBFrame.h"
#include "../include/lldb_c.h"

extern "C" {

void LLDB_SBThread_Destroy(SBThreadRef ref) {
    delete reinterpret_cast<lldb::SBThread*>(ref);
}

bool LLDB_SBThread_IsValid(SBThreadRef ref) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBThread*>(ref)->IsValid();
}

uint64_t LLDB_SBThread_GetThreadID(SBThreadRef ref) {
    if (!ref) return 0;
    return static_cast<uint64_t>(
        reinterpret_cast<lldb::SBThread*>(ref)->GetThreadID());
}

uint32_t LLDB_SBThread_GetIndexID(SBThreadRef ref) {
    if (!ref) return 0;
    return reinterpret_cast<lldb::SBThread*>(ref)->GetIndexID();
}

const char* LLDB_SBThread_GetName(SBThreadRef ref) {
    if (!ref) return nullptr;
    return reinterpret_cast<lldb::SBThread*>(ref)->GetName();
}

const char* LLDB_SBThread_GetQueueName(SBThreadRef ref) {
    if (!ref) return nullptr;
    return reinterpret_cast<lldb::SBThread*>(ref)->GetQueueName();
}

LLDBStopReason LLDB_SBThread_GetStopReason(SBThreadRef ref) {
    if (!ref) return LLDB_STOP_REASON_INVALID;
    return static_cast<LLDBStopReason>(
        reinterpret_cast<lldb::SBThread*>(ref)->GetStopReason());
}

size_t LLDB_SBThread_GetStopReasonDataCount(SBThreadRef ref) {
    if (!ref) return 0;
    return reinterpret_cast<lldb::SBThread*>(ref)->GetStopReasonDataCount();
}

uint64_t LLDB_SBThread_GetStopReasonDataAtIndex(SBThreadRef ref, uint32_t idx) {
    if (!ref) return 0;
    return reinterpret_cast<lldb::SBThread*>(ref)->GetStopReasonDataAtIndex(idx);
}

const char* LLDB_SBThread_GetStopDescription(SBThreadRef ref, char* dst, size_t dst_len) {
    if (!ref) return nullptr;
    reinterpret_cast<lldb::SBThread*>(ref)->GetStopDescription(dst, dst_len);
    return dst;
}

uint32_t LLDB_SBThread_GetNumFrames(SBThreadRef ref) {
    if (!ref) return 0;
    return reinterpret_cast<lldb::SBThread*>(ref)->GetNumFrames();
}

SBFrameRef LLDB_SBThread_GetFrameAtIndex(SBThreadRef ref, uint32_t idx) {
    if (!ref) return nullptr;
    lldb::SBFrame f = reinterpret_cast<lldb::SBThread*>(ref)->GetFrameAtIndex(idx);
    if (!f.IsValid()) return nullptr;
    return reinterpret_cast<SBFrameRef>(new lldb::SBFrame(f));
}

SBFrameRef LLDB_SBThread_GetSelectedFrame(SBThreadRef ref) {
    if (!ref) return nullptr;
    lldb::SBFrame f = reinterpret_cast<lldb::SBThread*>(ref)->GetSelectedFrame();
    if (!f.IsValid()) return nullptr;
    return reinterpret_cast<SBFrameRef>(new lldb::SBFrame(f));
}

void LLDB_SBThread_StepOver(SBThreadRef ref) {
    if (!ref) return;
    reinterpret_cast<lldb::SBThread*>(ref)->StepOver();
}

void LLDB_SBThread_StepInto(SBThreadRef ref) {
    if (!ref) return;
    reinterpret_cast<lldb::SBThread*>(ref)->StepInto();
}

void LLDB_SBThread_StepOut(SBThreadRef ref) {
    if (!ref) return;
    reinterpret_cast<lldb::SBThread*>(ref)->StepOut();
}

void LLDB_SBThread_StepInstruction(SBThreadRef ref, bool step_over) {
    if (!ref) return;
    reinterpret_cast<lldb::SBThread*>(ref)->StepInstruction(step_over);
}

void LLDB_SBThread_RunToAddress(SBThreadRef ref, uint64_t addr) {
    if (!ref) return;
    reinterpret_cast<lldb::SBThread*>(ref)->RunToAddress(
        static_cast<lldb::addr_t>(addr));
}

} // extern "C"
