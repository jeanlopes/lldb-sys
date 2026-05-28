pub mod debugger;
pub mod error;
pub mod target;
pub mod process;
pub mod thread;
pub mod frame;
pub mod breakpoint;
pub mod value;
pub mod event;
pub mod broadcaster;
pub mod listener;

pub use debugger::Debugger;
pub use error::Error;
pub use target::Target;
pub use process::Process;
pub use thread::Thread;
pub use frame::Frame;
pub use breakpoint::Breakpoint;
pub use value::Value;
pub use event::Event;
pub use broadcaster::Broadcaster;
pub use listener::Listener;

/// Process state, mirroring LLDB's `StateType` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum State {
    Invalid   = 0,
    Unloaded  = 1,
    Connected = 2,
    Attaching = 3,
    Launching = 4,
    Stopped   = 5,
    Running   = 6,
    Stepping  = 7,
    Crashed   = 8,
    Detached  = 9,
    Exited    = 10,
    Suspended = 11,
}

impl State {
    pub fn from_raw(v: u32) -> Self {
        match v {
            1  => State::Unloaded,
            2  => State::Connected,
            3  => State::Attaching,
            4  => State::Launching,
            5  => State::Stopped,
            6  => State::Running,
            7  => State::Stepping,
            8  => State::Crashed,
            9  => State::Detached,
            10 => State::Exited,
            11 => State::Suspended,
            _  => State::Invalid,
        }
    }

    pub fn is_stopped(self) -> bool {
        matches!(self, State::Stopped | State::Crashed | State::Suspended)
    }

    pub fn is_running(self) -> bool {
        matches!(self, State::Running | State::Stepping)
    }
}

/// Stop reason for a thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum StopReason {
    Invalid         = 0,
    None            = 1,
    Trace           = 2,
    Breakpoint      = 3,
    Watchpoint      = 4,
    Signal          = 5,
    Exception       = 6,
    Exec            = 7,
    PlanComplete    = 8,
    ThreadExiting   = 9,
}

impl StopReason {
    pub fn from_raw(v: u32) -> Self {
        match v {
            1  => StopReason::None,
            2  => StopReason::Trace,
            3  => StopReason::Breakpoint,
            4  => StopReason::Watchpoint,
            5  => StopReason::Signal,
            6  => StopReason::Exception,
            7  => StopReason::Exec,
            8  => StopReason::PlanComplete,
            9  => StopReason::ThreadExiting,
            _  => StopReason::Invalid,
        }
    }
}

/// On Windows: add `path` to the DLL search path so that `liblldb.dll` can be
/// found at runtime.  Call this *before* [`Debugger::initialize`].
///
/// The default path at `%LLDB_DLL_DIR%` (set by lldb-sys build.rs) is a good
/// starting point:
/// ```rust,no_run
/// let dll_dir = std::env::var("LLDB_DLL_DIR").expect("LLDB_DLL_DIR not set");
/// lldb_safe::set_lldb_dll_dir(dll_dir.as_ref());
/// ```
#[cfg(target_os = "windows")]
pub fn set_lldb_dll_dir(path: &std::path::Path) {
    use std::os::windows::ffi::OsStrExt;
    let mut wide: Vec<u16> = path.as_os_str().encode_wide().collect();
    wide.push(0); // null terminator
    unsafe {
        windows_sys::Win32::System::LibraryLoader::SetDllDirectoryW(wide.as_ptr());
    }
}

#[cfg(not(target_os = "windows"))]
pub fn set_lldb_dll_dir(_path: &std::path::Path) {
    // No-op on non-Windows: the dynamic linker handles rpath / LD_LIBRARY_PATH.
}
