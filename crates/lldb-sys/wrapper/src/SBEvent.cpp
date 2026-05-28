#include "lldb/API/SBEvent.h"
#include "../include/lldb_c.h"

extern "C" {

void LLDB_SBEvent_Destroy(SBEventRef ref) {
    delete reinterpret_cast<lldb::SBEvent*>(ref);
}

bool LLDB_SBEvent_IsValid(SBEventRef ref) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBEvent*>(ref)->IsValid();
}

uint32_t LLDB_SBEvent_GetType(SBEventRef ref) {
    if (!ref) return 0;
    return reinterpret_cast<lldb::SBEvent*>(ref)->GetType();
}

} // extern "C"
