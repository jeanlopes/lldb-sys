#include "lldb/API/SBFileSpec.h"
#include "../include/lldb_c.h"

extern "C" {

SBFileSpecRef LLDB_SBFileSpec_Create(const char* path, bool resolve) {
    return reinterpret_cast<SBFileSpecRef>(new lldb::SBFileSpec(path, resolve));
}

void LLDB_SBFileSpec_Destroy(SBFileSpecRef ref) {
    delete reinterpret_cast<lldb::SBFileSpec*>(ref);
}

bool LLDB_SBFileSpec_IsValid(SBFileSpecRef ref) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBFileSpec*>(ref)->IsValid();
}

const char* LLDB_SBFileSpec_GetFilename(SBFileSpecRef ref) {
    if (!ref) return nullptr;
    return reinterpret_cast<lldb::SBFileSpec*>(ref)->GetFilename();
}

const char* LLDB_SBFileSpec_GetDirectory(SBFileSpecRef ref) {
    if (!ref) return nullptr;
    return reinterpret_cast<lldb::SBFileSpec*>(ref)->GetDirectory();
}

bool LLDB_SBFileSpec_Exists(SBFileSpecRef ref) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBFileSpec*>(ref)->Exists();
}

} // extern "C"
