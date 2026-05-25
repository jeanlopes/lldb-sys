#include "lldb/API/SBError.h"
#include "../include/lldb_c.h"

extern "C" {

SBErrorRef LLDB_SBError_Create(void) {
    return reinterpret_cast<SBErrorRef>(new lldb::SBError());
}

void LLDB_SBError_Destroy(SBErrorRef ref) {
    delete reinterpret_cast<lldb::SBError*>(ref);
}

bool LLDB_SBError_IsValid(SBErrorRef ref) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBError*>(ref)->IsValid();
}

bool LLDB_SBError_Success(SBErrorRef ref) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBError*>(ref)->Success();
}

bool LLDB_SBError_Fail(SBErrorRef ref) {
    if (!ref) return true;
    return reinterpret_cast<lldb::SBError*>(ref)->Fail();
}

const char* LLDB_SBError_GetCString(SBErrorRef ref) {
    if (!ref) return nullptr;
    return reinterpret_cast<lldb::SBError*>(ref)->GetCString();
}

uint32_t LLDB_SBError_GetError(SBErrorRef ref) {
    if (!ref) return 0;
    return reinterpret_cast<lldb::SBError*>(ref)->GetError();
}

void LLDB_SBError_SetErrorString(SBErrorRef ref, const char* err_str) {
    if (!ref) return;
    reinterpret_cast<lldb::SBError*>(ref)->SetErrorString(err_str);
}

void LLDB_SBError_Clear(SBErrorRef ref) {
    if (!ref) return;
    reinterpret_cast<lldb::SBError*>(ref)->Clear();
}

} // extern "C"
