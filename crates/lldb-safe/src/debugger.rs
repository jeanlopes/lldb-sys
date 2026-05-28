use lldb_sys::*;
use std::ffi::CStr;
use crate::{Error, Target};
use crate::listener::Listener;

/// Owns an `SBDebugger` instance.
///
/// # Thread safety
///
/// `SBDebugger` is **not** thread-safe.  Do not share across threads without
/// external synchronisation.  `Debugger` is intentionally `!Send + !Sync`.
pub struct Debugger(SBDebuggerRef);

impl Debugger {
    /// One-time global initialisation.  Call before creating any `Debugger`.
    pub fn initialize() {
        unsafe { LLDB_SBDebugger_Initialize() };
    }

    /// One-time global teardown.  Call after all `Debugger` instances are dropped.
    pub fn terminate() {
        unsafe { LLDB_SBDebugger_Terminate() };
    }

    /// Create a new debugger session.
    ///
    /// `async_mode`: when `true`, commands return immediately and events are
    /// delivered via a listener; when `false` (the default), commands block
    /// until complete.
    pub fn create(async_mode: bool) -> Option<Self> {
        let raw = unsafe { LLDB_SBDebugger_Create(async_mode) };
        if raw.is_null() { None } else { Some(Self(raw)) }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { LLDB_SBDebugger_IsValid(self.0) }
    }

    pub fn set_async(&self, async_mode: bool) {
        unsafe { LLDB_SBDebugger_SetAsync(self.0, async_mode) };
    }

    pub fn get_async(&self) -> bool {
        unsafe { LLDB_SBDebugger_GetAsync(self.0) }
    }

    /// Create a debug target for `filename`.
    ///
    /// `triple`    – target triple, e.g. `"x86_64-pc-windows-msvc"` (pass `""` for auto-detect)
    /// `platform`  – platform name (pass `""` for host platform)
    /// `add_deps`  – add dependent modules automatically
    pub fn create_target(
        &self,
        filename: &str,
        triple: &str,
        platform: &str,
        add_deps: bool,
    ) -> Result<Target, Error> {
        use std::ffi::CString;
        let fname  = CString::new(filename).unwrap();
        let triple = CString::new(triple).unwrap();
        let plat   = CString::new(platform).unwrap();
        let err    = Error::new();

        let raw = unsafe {
            LLDB_SBDebugger_CreateTarget(
                self.0,
                fname.as_ptr(),
                if triple.as_bytes().is_empty() { std::ptr::null() } else { triple.as_ptr() },
                if plat.as_bytes().is_empty()   { std::ptr::null() } else { plat.as_ptr() },
                add_deps,
                err.as_raw(),
            )
        };

        if raw.is_null() || err.fail() {
            Err(err)
        } else {
            Ok(Target::from_raw(raw))
        }
    }

    /// Simplified target creation — auto-detects triple and platform.
    pub fn create_target_simple(&self, filename: &str) -> Option<Target> {
        use std::ffi::CString;
        let fname = CString::new(filename).unwrap();
        let raw = unsafe { LLDB_SBDebugger_CreateTargetSimple(self.0, fname.as_ptr()) };
        if raw.is_null() { None } else { Some(Target::from_raw(raw)) }
    }

    pub fn num_targets(&self) -> u32 {
        unsafe { LLDB_SBDebugger_GetNumTargets(self.0) }
    }

    pub fn target_at(&self, idx: u32) -> Option<Target> {
        let raw = unsafe { LLDB_SBDebugger_GetTargetAtIndex(self.0, idx) };
        if raw.is_null() { None } else { Some(Target::from_raw(raw)) }
    }

    pub fn selected_target(&self) -> Option<Target> {
        let raw = unsafe { LLDB_SBDebugger_GetSelectedTarget(self.0) };
        if raw.is_null() { None } else { Some(Target::from_raw(raw)) }
    }

    pub fn version_string() -> &'static str {
        let ptr = unsafe { LLDB_SBDebugger_GetVersionString() };
        if ptr.is_null() {
            "<unknown>"
        } else {
            unsafe { CStr::from_ptr(ptr).to_str().unwrap_or("<invalid utf8>") }
        }
    }

    /// Run a single LLDB CLI command and return the combined stdout+stderr output.
    pub fn handle_command(&self, command: &str) -> String {
        use std::ffi::CString;
        let c_cmd = CString::new(command).unwrap_or_default();
        let ptr = unsafe { LLDB_SBDebugger_HandleCommand(self.0, c_cmd.as_ptr()) };
        if ptr.is_null() {
            return String::new();
        }
        let s = unsafe { CStr::from_ptr(ptr).to_str().unwrap_or("").to_owned() };
        unsafe { LLDB_FreeString(ptr) };
        s
    }

    /// Get the debugger's default listener (used in async mode for event delivery).
    pub fn get_listener(&self) -> Option<Listener> {
        let raw = unsafe { LLDB_SBDebugger_GetListener(self.0) };
        if raw.is_null() { None } else { Some(Listener::from_raw(raw)) }
    }
}

impl Drop for Debugger {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { LLDB_SBDebugger_Destroy(self.0) };
        }
    }
}
