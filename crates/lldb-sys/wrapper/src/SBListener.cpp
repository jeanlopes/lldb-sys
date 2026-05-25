#include "lldb/API/SBListener.h"
#include "../include/lldb_c.h"

extern "C" {

SBListenerRef LLDB_SBListener_Create(const char* name) {
    return reinterpret_cast<SBListenerRef>(new lldb::SBListener(name));
}

void LLDB_SBListener_Destroy(SBListenerRef ref) {
    delete reinterpret_cast<lldb::SBListener*>(ref);
}

bool LLDB_SBListener_IsValid(SBListenerRef ref) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBListener*>(ref)->IsValid();
}

} // extern "C"
