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

    /// Return the source file path and line number for this frame via `SBLineEntry`.
    /// Returns `None` if no debug information is available.
    pub fn source_location(&self) -> Option<(String, u32)> {
        let mut buf = vec![0i8; 1024];
        // Safety: LLDB_SBFrame_GetLineEntryFile writes into a caller-supplied buffer
        // with an explicit length bound. No safe alternative exists because lldb-sys
        // (via bindgen) does not expose SBLineEntry's path methods directly.
        unsafe { LLDB_SBFrame_GetLineEntryFile(self.0, buf.as_mut_ptr(), buf.len()) };
        let path = unsafe { CStr::from_ptr(buf.as_ptr()) }
            .to_string_lossy()
            .into_owned();
        // Safety: same wrapper pattern; returns 0 when line entry is invalid.
        let line = unsafe { LLDB_SBFrame_GetLineEntryLine(self.0) };
        if path.is_empty() || line == 0 { None } else { Some((path, line)) }
    }

    /// Return all variables in scope for this frame.
    ///
    /// `arguments` — include function arguments
    /// `locals`    — include local variables
    /// `statics`   — include static variables
    pub fn variables(&self, arguments: bool, locals: bool, statics: bool) -> Vec<Value> {
        const MAX: u32 = 256;
        let mut ptrs: Vec<lldb_sys::SBValueRef> = vec![std::ptr::null_mut(); MAX as usize];
        // Safety: LLDB_SBFrame_GetVariables writes heap-allocated SBValue* into `ptrs`.
        // Each non-null pointer is wrapped in a `Value` which calls Destroy on drop.
        let n = unsafe {
            LLDB_SBFrame_GetVariables(
                self.0, arguments, locals, statics, true,
                ptrs.as_mut_ptr(), MAX,
            )
        } as usize;
        ptrs.into_iter()
            .take(n)
            .filter(|p| !p.is_null())
            .map(Value::from_raw)
            .collect()
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { LLDB_SBFrame_Destroy(self.0) };
        }
    }
}
