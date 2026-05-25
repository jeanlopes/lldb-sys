#include "lldb/API/SBValue.h"
#include "lldb/API/SBError.h"
#include "../include/lldb_c.h"

extern "C" {

void LLDB_SBValue_Destroy(SBValueRef ref) {
    delete reinterpret_cast<lldb::SBValue*>(ref);
}

bool LLDB_SBValue_IsValid(SBValueRef ref) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBValue*>(ref)->IsValid();
}

const char* LLDB_SBValue_GetName(SBValueRef ref) {
    if (!ref) return nullptr;
    return reinterpret_cast<lldb::SBValue*>(ref)->GetName();
}

const char* LLDB_SBValue_GetTypeName(SBValueRef ref) {
    if (!ref) return nullptr;
    return reinterpret_cast<lldb::SBValue*>(ref)->GetTypeName();
}

const char* LLDB_SBValue_GetDisplayTypeName(SBValueRef ref) {
    if (!ref) return nullptr;
    return reinterpret_cast<lldb::SBValue*>(ref)->GetDisplayTypeName();
}

const char* LLDB_SBValue_GetValue(SBValueRef ref) {
    if (!ref) return nullptr;
    return reinterpret_cast<lldb::SBValue*>(ref)->GetValue();
}

bool LLDB_SBValue_IsInScope(SBValueRef ref) {
    if (!ref) return false;
    return reinterpret_cast<lldb::SBValue*>(ref)->IsInScope();
}

uint64_t LLDB_SBValue_GetValueAsUnsigned(SBValueRef ref, uint64_t fail_value) {
    if (!ref) return fail_value;
    return reinterpret_cast<lldb::SBValue*>(ref)->GetValueAsUnsigned(fail_value);
}

int64_t LLDB_SBValue_GetValueAsSigned(SBValueRef ref, int64_t fail_value) {
    if (!ref) return fail_value;
    return reinterpret_cast<lldb::SBValue*>(ref)->GetValueAsSigned(fail_value);
}

uint64_t LLDB_SBValue_GetAddress(SBValueRef ref) {
    if (!ref) return 0;
    return static_cast<uint64_t>(
        reinterpret_cast<lldb::SBValue*>(ref)->GetLoadAddress());
}

uint32_t LLDB_SBValue_GetNumChildren(SBValueRef ref) {
    if (!ref) return 0;
    return reinterpret_cast<lldb::SBValue*>(ref)->GetNumChildren();
}

SBValueRef LLDB_SBValue_GetChildAtIndex(SBValueRef ref, uint32_t idx) {
    if (!ref) return nullptr;
    lldb::SBValue child = reinterpret_cast<lldb::SBValue*>(ref)->GetChildAtIndex(idx);
    if (!child.IsValid()) return nullptr;
    return reinterpret_cast<SBValueRef>(new lldb::SBValue(child));
}

SBValueRef LLDB_SBValue_GetChildMemberWithName(SBValueRef ref, const char* name) {
    if (!ref) return nullptr;
    lldb::SBValue child = reinterpret_cast<lldb::SBValue*>(ref)->GetChildMemberWithName(name);
    if (!child.IsValid()) return nullptr;
    return reinterpret_cast<SBValueRef>(new lldb::SBValue(child));
}

SBValueRef LLDB_SBValue_Dereference(SBValueRef ref) {
    if (!ref) return nullptr;
    lldb::SBValue deref = reinterpret_cast<lldb::SBValue*>(ref)->Dereference();
    if (!deref.IsValid()) return nullptr;
    return reinterpret_cast<SBValueRef>(new lldb::SBValue(deref));
}

SBErrorRef LLDB_SBValue_GetError(SBValueRef ref) {
    auto* err = new lldb::SBError();
    if (ref) {
        *err = reinterpret_cast<lldb::SBValue*>(ref)->GetError();
    }
    return reinterpret_cast<SBErrorRef>(err);
}

} // extern "C"
