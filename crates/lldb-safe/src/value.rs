use lldb_sys::*;
use std::ffi::{CStr, CString};
use crate::Error;

/// Wraps `SBValueRef` — represents a variable, register, or expression result.
pub struct Value(SBValueRef);

impl Value {
    pub(crate) fn from_raw(raw: SBValueRef) -> Self {
        Self(raw)
    }

    pub fn is_valid(&self) -> bool {
        unsafe { LLDB_SBValue_IsValid(self.0) }
    }

    pub fn name(&self) -> Option<&str> {
        let ptr = unsafe { LLDB_SBValue_GetName(self.0) };
        if ptr.is_null() { None } else {
            Some(unsafe { CStr::from_ptr(ptr).to_str().unwrap_or("") })
        }
    }

    pub fn type_name(&self) -> Option<&str> {
        let ptr = unsafe { LLDB_SBValue_GetTypeName(self.0) };
        if ptr.is_null() { None } else {
            Some(unsafe { CStr::from_ptr(ptr).to_str().unwrap_or("") })
        }
    }

    pub fn display_type_name(&self) -> Option<&str> {
        let ptr = unsafe { LLDB_SBValue_GetDisplayTypeName(self.0) };
        if ptr.is_null() { None } else {
            Some(unsafe { CStr::from_ptr(ptr).to_str().unwrap_or("") })
        }
    }

    /// The value as a string (as LLDB would display it).
    pub fn value_string(&self) -> Option<&str> {
        let ptr = unsafe { LLDB_SBValue_GetValue(self.0) };
        if ptr.is_null() { None } else {
            Some(unsafe { CStr::from_ptr(ptr).to_str().unwrap_or("") })
        }
    }

    pub fn is_in_scope(&self) -> bool {
        unsafe { LLDB_SBValue_IsInScope(self.0) }
    }

    pub fn as_u64(&self) -> u64 {
        unsafe { LLDB_SBValue_GetValueAsUnsigned(self.0, u64::MAX) }
    }

    pub fn as_i64(&self) -> i64 {
        unsafe { LLDB_SBValue_GetValueAsSigned(self.0, i64::MIN) }
    }

    pub fn load_address(&self) -> u64 {
        unsafe { LLDB_SBValue_GetAddress(self.0) }
    }

    pub fn num_children(&self) -> u32 {
        unsafe { LLDB_SBValue_GetNumChildren(self.0) }
    }

    pub fn child_at(&self, idx: u32) -> Option<Value> {
        let raw = unsafe { LLDB_SBValue_GetChildAtIndex(self.0, idx) };
        if raw.is_null() { None } else { Some(Value::from_raw(raw)) }
    }

    pub fn child_named(&self, name: &str) -> Option<Value> {
        let c = CString::new(name).unwrap();
        let raw = unsafe { LLDB_SBValue_GetChildMemberWithName(self.0, c.as_ptr()) };
        if raw.is_null() { None } else { Some(Value::from_raw(raw)) }
    }

    pub fn dereference(&self) -> Option<Value> {
        let raw = unsafe { LLDB_SBValue_Dereference(self.0) };
        if raw.is_null() { None } else { Some(Value::from_raw(raw)) }
    }

    pub fn error(&self) -> Error {
        let raw = unsafe { LLDB_SBValue_GetError(self.0) };
        Error::from_raw(raw).unwrap_or_else(Error::new)
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { LLDB_SBValue_Destroy(self.0) };
        }
    }
}
