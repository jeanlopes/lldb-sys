#include "lldb/API/SBBroadcaster.h"
#include "lldb/API/SBListener.h"
#include "../include/lldb_c.h"

extern "C" {

void LLDB_SBBroadcaster_Destroy(SBBroadcasterRef ref) {
    delete reinterpret_cast<lldb::SBBroadcaster*>(ref);
}

bool LLDB_SBBroadcaster_IsValid(SBBroadcasterRef ref) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBBroadcaster*>(ref)->IsValid();
}

bool LLDB_SBBroadcaster_AddListener(SBBroadcasterRef ref,
                                     SBListenerRef    listener,
                                     uint32_t         event_mask) {
    if (!ref || !listener) return false;
    uint32_t bits = reinterpret_cast<lldb::SBBroadcaster*>(ref)->AddListener(
        *reinterpret_cast<lldb::SBListener*>(listener), event_mask);
    return bits != 0;
}

} // extern "C"
