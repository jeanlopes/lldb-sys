use lldb_sys::*;
use std::ffi::{CStr, CString};
use crate::{Error, Process, Breakpoint};

/// Wraps `SBTargetRef`.
pub struct Target(SBTargetRef);

impl Target {
    pub(crate) fn from_raw(raw: SBTargetRef) -> Self {
        Self(raw)
    }

    pub fn is_valid(&self) -> bool {
        unsafe { LLDB_SBTarget_IsValid(self.0) }
    }

    pub fn triple(&self) -> Option<&str> {
        let ptr = unsafe { LLDB_SBTarget_GetTriple(self.0) };
        if ptr.is_null() { None } else {
            Some(unsafe { CStr::from_ptr(ptr).to_str().unwrap_or("") })
        }
    }

    /// Launch the target as a new process with `argv`, stopping at entry.
    ///
    /// Returns the live `Process`.
    pub fn launch(
        &self,
        argv: &[&str],
        envp: &[&str],
        working_dir: Option<&str>,
        stop_at_entry: bool,
    ) -> Result<Process, Error> {
        let c_argv: Vec<CString> = argv.iter().map(|s| CString::new(*s).unwrap()).collect();
        let c_envp: Vec<CString> = envp.iter().map(|s| CString::new(*s).unwrap()).collect();
        let c_argv_ptrs: Vec<*const i8> = c_argv.iter().map(|s| s.as_ptr()).chain(std::iter::once(std::ptr::null())).collect();
        let c_envp_ptrs: Vec<*const i8> = c_envp.iter().map(|s| s.as_ptr()).chain(std::iter::once(std::ptr::null())).collect();
        let wd = working_dir.map(|d| CString::new(d).unwrap());

        let err = Error::new();
        let raw = unsafe {
            LLDB_SBTarget_Launch(
                self.0,
                std::ptr::null_mut(), // use default listener
                c_argv_ptrs.as_ptr() as *mut *const i8,
                if c_envp_ptrs.len() <= 1 { std::ptr::null_mut() }
                else { c_envp_ptrs.as_ptr() as *mut *const i8 },
                std::ptr::null(), // stdin
                std::ptr::null(), // stdout
                std::ptr::null(), // stderr
                wd.as_ref().map(|s| s.as_ptr()).unwrap_or(std::ptr::null()),
                0, // launch flags
                stop_at_entry,
                err.as_raw(),
            )
        };

        if raw.is_null() || err.fail() {
            Err(err)
        } else {
            Ok(Process::from_raw(raw))
        }
    }

    /// Attach to an existing process by PID.
    pub fn attach_to_pid(&self, pid: u64) -> Result<Process, Error> {
        let err = Error::new();
        let raw = unsafe {
            LLDB_SBTarget_AttachToProcessWithID(
                self.0,
                std::ptr::null_mut(), // default listener
                pid,
                err.as_raw(),
            )
        };
        if raw.is_null() || err.fail() {
            Err(err)
        } else {
            Ok(Process::from_raw(raw))
        }
    }

    /// Get the process associated with this target (if running).
    pub fn process(&self) -> Option<Process> {
        let raw = unsafe { LLDB_SBTarget_GetProcess(self.0) };
        if raw.is_null() { None } else { Some(Process::from_raw(raw)) }
    }

    /// Set a breakpoint by symbol name.
    pub fn breakpoint_by_name(&self, name: &str, module: Option<&str>) -> Option<Breakpoint> {
        let c_name = CString::new(name).unwrap();
        let c_mod  = module.map(|m| CString::new(m).unwrap());
        let raw = unsafe {
            LLDB_SBTarget_BreakpointCreateByName(
                self.0,
                c_name.as_ptr(),
                c_mod.as_ref().map(|s| s.as_ptr()).unwrap_or(std::ptr::null()),
            )
        };
        if raw.is_null() { None } else { Some(Breakpoint::from_raw(raw)) }
    }

    /// Set a breakpoint at a source location.
    pub fn breakpoint_by_location(&self, file: &str, line: u32) -> Option<Breakpoint> {
        let c_file = CString::new(file).unwrap();
        let raw = unsafe { LLDB_SBTarget_BreakpointCreateByLocation(self.0, c_file.as_ptr(), line) };
        if raw.is_null() { None } else { Some(Breakpoint::from_raw(raw)) }
    }

    /// Set a breakpoint at a raw memory address.
    pub fn breakpoint_by_address(&self, address: u64) -> Option<Breakpoint> {
        let raw = unsafe { LLDB_SBTarget_BreakpointCreateByAddress(self.0, address) };
        if raw.is_null() { None } else { Some(Breakpoint::from_raw(raw)) }
    }

    /// Remove a breakpoint by its ID.
    pub fn delete_breakpoint(&self, id: u32) -> bool {
        unsafe { LLDB_SBTarget_DeleteBreakpoint(self.0, id) }
    }

    pub fn num_breakpoints(&self) -> u32 {
        unsafe { LLDB_SBTarget_GetNumBreakpoints(self.0) }
    }

    pub fn breakpoint_at(&self, idx: u32) -> Option<Breakpoint> {
        let raw = unsafe { LLDB_SBTarget_GetBreakpointAtIndex(self.0, idx) };
        if raw.is_null() { None } else { Some(Breakpoint::from_raw(raw)) }
    }
}

impl Drop for Target {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { LLDB_SBTarget_Destroy(self.0) };
        }
    }
}
