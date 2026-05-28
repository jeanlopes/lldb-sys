#include "lldb/API/SBListener.h"
#include "lldb/API/SBEvent.h"
#include "../include/lldb_c.h"

extern "C"
{

    SBListenerRef LLDB_SBListener_Create(const char *name)
    {
        return reinterpret_cast<SBListenerRef>(new lldb::SBListener(name));
    }

    void LLDB_SBListener_Destroy(SBListenerRef ref)
    {
        delete reinterpret_cast<lldb::SBListener *>(ref);
    }

    bool LLDB_SBListener_IsValid(SBListenerRef ref)
    {
        if (!ref)
            return false;
        return reinterpret_cast<lldb::SBListener *>(ref)->IsValid();
    }

    bool LLDB_SBListener_WaitForEvent(SBListenerRef ref,
                                      uint32_t timeout_secs,
                                      SBEventRef *event_out)
    {
        if (!ref || !event_out)
            return false;
        auto *listener = reinterpret_cast<lldb::SBListener *>(ref);
        auto *ev = new lldb::SBEvent();
        bool got = listener->WaitForEvent(timeout_secs, *ev);
        if (got)
        {
            *event_out = reinterpret_cast<SBEventRef>(ev);
        }
        else
        {
            delete ev;
            *event_out = nullptr;
        }
        return got;
    }

} // extern "C"
