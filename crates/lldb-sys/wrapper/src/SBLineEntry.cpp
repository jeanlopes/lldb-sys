#include "lldb/API/SBFrame.h"
#include "lldb/API/SBLineEntry.h"
#include "lldb/API/SBFileSpec.h"
#include "lldb/API/SBValue.h"
#include "lldb/API/SBValueList.h"
#include "../include/lldb_c.h"
#include <cstring>

extern "C" {

// Fill `buf` with the full path of the source file for the frame's line entry.
// Returns an empty string if no debug information is available.
void LLDB_SBFrame_GetLineEntryFile(SBFrameRef frame, char* buf, size_t buf_len) {
    if (!frame || !buf || buf_len == 0) return;
    buf[0] = '\0';
    auto* f = reinterpret_cast<lldb::SBFrame*>(frame);
    lldb::SBLineEntry entry = f->GetLineEntry();
    if (!entry.IsValid()) return;
    lldb::SBFileSpec spec = entry.GetFileSpec();
    if (!spec.IsValid()) return;
    spec.GetPath(buf, buf_len);
}

// Return the source line number for the frame's line entry (0 if unavailable).
uint32_t LLDB_SBFrame_GetLineEntryLine(SBFrameRef frame) {
    if (!frame) return 0;
    auto* f = reinterpret_cast<lldb::SBFrame*>(frame);
    lldb::SBLineEntry entry = f->GetLineEntry();
    if (!entry.IsValid()) return 0;
    return entry.GetLine();
}

// Fill `out_refs` with up to `max_count` in-scope variables from the frame.
// Returns the number of SBValueRef handles written (caller must Destroy each).
uint32_t LLDB_SBFrame_GetVariables(SBFrameRef frame,
                                   bool arguments,
                                   bool locals,
                                   bool statics,
                                   bool in_scope_only,
                                   SBValueRef* out_refs,
                                   uint32_t max_count) {
    if (!frame || !out_refs || max_count == 0) return 0;
    auto* f = reinterpret_cast<lldb::SBFrame*>(frame);
    lldb::SBValueList list = f->GetVariables(arguments, locals, statics, in_scope_only);
    uint32_t n = list.GetSize();
    if (n > max_count) n = max_count;
    for (uint32_t i = 0; i < n; ++i) {
        lldb::SBValue v = list.GetValueAtIndex(i);
        out_refs[i] = reinterpret_cast<SBValueRef>(new lldb::SBValue(v));
    }
    return n;
}

} // extern "C"
