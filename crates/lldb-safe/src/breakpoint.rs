use lldb_sys::*;
use std::ffi::{CStr, CString};

/// Wraps `SBBreakpointRef`.
pub struct Breakpoint(SBBreakpointRef);

impl Breakpoint {
    pub(crate) fn from_raw(raw: SBBreakpointRef) -> Self {
        Self(raw)
    }

    pub fn is_valid(&self) -> bool {
        unsafe { LLDB_SBBreakpoint_IsValid(self.0) }
    }

    pub fn id(&self) -> u32 {
        unsafe { LLDB_SBBreakpoint_GetID(self.0) }
    }

    pub fn enable(&self) {
        unsafe { LLDB_SBBreakpoint_SetEnabled(self.0, true) };
    }

    pub fn disable(&self) {
        unsafe { LLDB_SBBreakpoint_SetEnabled(self.0, false) };
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { LLDB_SBBreakpoint_IsEnabled(self.0) }
    }

    pub fn set_one_shot(&self, one_shot: bool) {
        unsafe { LLDB_SBBreakpoint_SetOneShot(self.0, one_shot) };
    }

    pub fn is_one_shot(&self) -> bool {
        unsafe { LLDB_SBBreakpoint_IsOneShot(self.0) }
    }

    pub fn hit_count(&self) -> u32 {
        unsafe { LLDB_SBBreakpoint_GetHitCount(self.0) }
    }

    pub fn set_condition(&self, condition: &str) {
        let c = CString::new(condition).unwrap();
        unsafe { LLDB_SBBreakpoint_SetCondition(self.0, c.as_ptr()) };
    }

    pub fn condition(&self) -> Option<&str> {
        let ptr = unsafe { LLDB_SBBreakpoint_GetCondition(self.0) };
        if ptr.is_null() { None } else {
            Some(unsafe { CStr::from_ptr(ptr).to_str().unwrap_or("") })
        }
    }

    pub fn set_ignore_count(&self, count: u32) {
        unsafe { LLDB_SBBreakpoint_SetIgnoreCount(self.0, count) };
    }

    pub fn ignore_count(&self) -> u32 {
        unsafe { LLDB_SBBreakpoint_GetIgnoreCount(self.0) }
    }

    pub fn num_locations(&self) -> u32 {
        unsafe { LLDB_SBBreakpoint_GetNumLocations(self.0) }
    }
}

impl Drop for Breakpoint {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { LLDB_SBBreakpoint_Destroy(self.0) };
        }
    }
}
