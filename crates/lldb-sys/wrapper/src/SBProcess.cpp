#include "lldb/API/SBProcess.h"
#include "lldb/API/SBThread.h"
#include "lldb/API/SBError.h"
#include "lldb/API/SBEvent.h"
#include "lldb/API/SBBroadcaster.h"
#include "../include/lldb_c.h"

extern "C" {

void LLDB_SBProcess_Destroy(SBProcessRef ref) {
    delete reinterpret_cast<lldb::SBProcess*>(ref);
}

bool LLDB_SBProcess_IsValid(SBProcessRef ref) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBProcess*>(ref)->IsValid();
}

LLDBStateType LLDB_SBProcess_GetState(SBProcessRef ref) {
    if (!ref) return LLDB_STATE_INVALID;
    return static_cast<LLDBStateType>(
        reinterpret_cast<lldb::SBProcess*>(ref)->GetState());
}

uint64_t LLDB_SBProcess_GetProcessID(SBProcessRef ref) {
    if (!ref) return 0;
    return static_cast<uint64_t>(
        reinterpret_cast<lldb::SBProcess*>(ref)->GetProcessID());
}

uint32_t LLDB_SBProcess_GetUniqueID(SBProcessRef ref) {
    if (!ref) return 0;
    return reinterpret_cast<lldb::SBProcess*>(ref)->GetUniqueID();
}

SBErrorRef LLDB_SBProcess_Continue(SBProcessRef ref) {
    auto* err = new lldb::SBError();
    if (ref) {
        *err = reinterpret_cast<lldb::SBProcess*>(ref)->Continue();
    } else {
        err->SetErrorString("invalid process");
    }
    return reinterpret_cast<SBErrorRef>(err);
}

SBErrorRef LLDB_SBProcess_Stop(SBProcessRef ref) {
    auto* err = new lldb::SBError();
    if (ref) {
        *err = reinterpret_cast<lldb::SBProcess*>(ref)->Stop();
    } else {
        err->SetErrorString("invalid process");
    }
    return reinterpret_cast<SBErrorRef>(err);
}

SBErrorRef LLDB_SBProcess_Kill(SBProcessRef ref) {
    auto* err = new lldb::SBError();
    if (ref) {
        *err = reinterpret_cast<lldb::SBProcess*>(ref)->Kill();
    } else {
        err->SetErrorString("invalid process");
    }
    return reinterpret_cast<SBErrorRef>(err);
}

SBErrorRef LLDB_SBProcess_Detach(SBProcessRef ref) {
    auto* err = new lldb::SBError();
    if (ref) {
        *err = reinterpret_cast<lldb::SBProcess*>(ref)->Detach();
    } else {
        err->SetErrorString("invalid process");
    }
    return reinterpret_cast<SBErrorRef>(err);
}

uint32_t LLDB_SBProcess_GetNumThreads(SBProcessRef ref) {
    if (!ref) return 0;
    return reinterpret_cast<lldb::SBProcess*>(ref)->GetNumThreads();
}

SBThreadRef LLDB_SBProcess_GetThreadAtIndex(SBProcessRef ref, size_t idx) {
    if (!ref) return nullptr;
    lldb::SBThread t = reinterpret_cast<lldb::SBProcess*>(ref)->GetThreadAtIndex(idx);
    if (!t.IsValid()) return nullptr;
    return reinterpret_cast<SBThreadRef>(new lldb::SBThread(t));
}

SBThreadRef LLDB_SBProcess_GetThreadByID(SBProcessRef ref, uint64_t tid) {
    if (!ref) return nullptr;
    lldb::SBThread t = reinterpret_cast<lldb::SBProcess*>(ref)->GetThreadByID(
        static_cast<lldb::tid_t>(tid));
    if (!t.IsValid()) return nullptr;
    return reinterpret_cast<SBThreadRef>(new lldb::SBThread(t));
}

SBThreadRef LLDB_SBProcess_GetThreadByIndexID(SBProcessRef ref, uint32_t idx_id) {
    if (!ref) return nullptr;
    lldb::SBThread t = reinterpret_cast<lldb::SBProcess*>(ref)->GetThreadByIndexID(idx_id);
    if (!t.IsValid()) return nullptr;
    return reinterpret_cast<SBThreadRef>(new lldb::SBThread(t));
}

SBThreadRef LLDB_SBProcess_GetSelectedThread(SBProcessRef ref) {
    if (!ref) return nullptr;
    lldb::SBThread t = reinterpret_cast<lldb::SBProcess*>(ref)->GetSelectedThread();
    if (!t.IsValid()) return nullptr;
    return reinterpret_cast<SBThreadRef>(new lldb::SBThread(t));
}

bool LLDB_SBProcess_SetSelectedThreadByID(SBProcessRef ref, uint64_t tid) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBProcess*>(ref)->SetSelectedThreadByID(
        static_cast<lldb::tid_t>(tid));
}

int32_t LLDB_SBProcess_GetExitStatus(SBProcessRef ref) {
    if (!ref) return -1;
    return reinterpret_cast<lldb::SBProcess*>(ref)->GetExitStatus();
}

const char* LLDB_SBProcess_GetExitDescription(SBProcessRef ref) {
    if (!ref) return nullptr;
    return reinterpret_cast<lldb::SBProcess*>(ref)->GetExitDescription();
}

size_t LLDB_SBProcess_ReadMemory(SBProcessRef ref,
                                  uint64_t     addr,
                                  void*        buf,
                                  size_t       size,
                                  SBErrorRef   error_ref) {
    if (!ref) return 0;
    auto* proc = reinterpret_cast<lldb::SBProcess*>(ref);
    lldb::SBError local_err;
    lldb::SBError* err = error_ref
        ? reinterpret_cast<lldb::SBError*>(error_ref)
        : &local_err;
    return proc->ReadMemory(static_cast<lldb::addr_t>(addr), buf, size, *err);
}

size_t LLDB_SBProcess_WriteMemory(SBProcessRef ref,
                                   uint64_t     addr,
                                   const void*  buf,
                                   size_t       size,
                                   SBErrorRef   error_ref) {
    if (!ref) return 0;
    auto* proc = reinterpret_cast<lldb::SBProcess*>(ref);
    lldb::SBError local_err;
    lldb::SBError* err = error_ref
        ? reinterpret_cast<lldb::SBError*>(error_ref)
        : &local_err;
    return proc->WriteMemory(static_cast<lldb::addr_t>(addr), buf, size, *err);
}

SBBroadcasterRef LLDB_SBProcess_GetBroadcaster(SBProcessRef ref) {
    if (!ref) return nullptr;
    lldb::SBBroadcaster b = reinterpret_cast<lldb::SBProcess*>(ref)->GetBroadcaster();
    if (!b.IsValid()) return nullptr;
    return reinterpret_cast<SBBroadcasterRef>(new lldb::SBBroadcaster(b));
}

LLDBStateType LLDB_SBProcess_GetStateFromEvent(SBEventRef event) {
    if (!event) return LLDB_STATE_INVALID;
    return static_cast<LLDBStateType>(
        lldb::SBProcess::GetStateFromEvent(
            *reinterpret_cast<lldb::SBEvent*>(event)));
}

bool LLDB_SBProcess_GetRestartedFromEvent(SBEventRef event) {
    if (!event) return false;
    return lldb::SBProcess::GetRestartedFromEvent(
        *reinterpret_cast<lldb::SBEvent*>(event));
}

bool LLDB_SBProcess_EventIsProcessEvent(SBEventRef event) {
    if (!event) return false;
    return lldb::SBProcess::EventIsProcessEvent(
        *reinterpret_cast<lldb::SBEvent*>(event));
}

} // extern "C"