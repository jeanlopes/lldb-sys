use lldb_sys::*;
use std::ffi::{CStr, CString};
use crate::Value;

/// Wraps `SBFrameRef`.
pub struct Frame(SBFrameRef);

impl Frame {
    pub(crate) fn from_raw(raw: SBFrameRef) -> Self {
        Self(raw)
    }

    pub fn is_valid(&self) -> bool {
        unsafe { LLDB_SBFrame_IsValid(self.0) }
    }

    pub fn frame_id(&self) -> u32 {
        unsafe { LLDB_SBFrame_GetFrameID(self.0) }
    }

    pub fn pc(&self) -> u64 {
        unsafe { LLDB_SBFrame_GetPC(self.0) }
    }

    pub fn sp(&self) -> u64 {
        unsafe { LLDB_SBFrame_GetSP(self.0) }
    }

    pub fn fp(&self) -> u64 {
        unsafe { LLDB_SBFrame_GetFP(self.0) }
    }

    pub fn cfa(&self) -> u64 {
        unsafe { LLDB_SBFrame_GetCFA(self.0) }
    }

    pub fn function_name(&self) -> Option<&str> {
        let ptr = unsafe { LLDB_SBFrame_GetFunctionName(self.0) };
        if ptr.is_null() { None } else {
            Some(unsafe { CStr::from_ptr(ptr).to_str().unwrap_or("") })
        }
    }

    pub fn is_inlined(&self) -> bool {
        unsafe { LLDB_SBFrame_IsInlined(self.0) }
    }

    pub fn is_artificial(&self) -> bool {
        unsafe { LLDB_SBFrame_IsArtificial(self.0) }
    }

    /// Look up a local variable by name.
    pub fn find_variable(&self, name: &str) -> Option<Value> {
        let c_name = CString::new(name).unwrap();
        let raw = unsafe { LLDB_SBFrame_FindVariable(self.0, c_name.as_ptr()) };
        if raw.is_null() { None } else { Some(Value::from_raw(raw)) }
    }

    /// Evaluate an LLDB expression in the context of this frame.
    pub fn evaluate_expression(&self, expr: &str) -> Option<Value> {
        let c_expr = CString::new(expr).unwrap();
        let raw = unsafe { LLDB_SBFrame_EvaluateExpression(self.0, c_expr.as_ptr()) };
        if raw.is_null() { None } else { Some(Value::from_raw(raw)) }
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { LLDB_SBFrame_Destroy(self.0) };
        }
    }
}
