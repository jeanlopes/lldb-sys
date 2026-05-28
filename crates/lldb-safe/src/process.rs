use lldb_sys::*;
use std::ffi::CStr;
use crate::{Error, State, Thread};
use crate::event::Event;
use crate::broadcaster::Broadcaster;

/// Wraps `SBProcessRef`.
pub struct Process(SBProcessRef);

impl Process {
    pub(crate) fn from_raw(raw: SBProcessRef) -> Self {
        Self(raw)
    }

    pub fn is_valid(&self) -> bool {
        unsafe { LLDB_SBProcess_IsValid(self.0) }
    }

    pub fn state(&self) -> State {
        State::from_raw(unsafe { LLDB_SBProcess_GetState(self.0) } as u32)
    }

    pub fn pid(&self) -> u64 {
        unsafe { LLDB_SBProcess_GetProcessID(self.0) }
    }

    pub fn unique_id(&self) -> u32 {
        unsafe { LLDB_SBProcess_GetUniqueID(self.0) }
    }

    pub fn resume(&self) -> Result<(), Error> {
        let raw = unsafe { LLDB_SBProcess_Continue(self.0) };
        let err = Error::from_raw(raw).unwrap_or_else(Error::new);
        if err.fail() { Err(err) } else { Ok(()) }
    }

    pub fn stop(&self) -> Result<(), Error> {
        let raw = unsafe { LLDB_SBProcess_Stop(self.0) };
        let err = Error::from_raw(raw).unwrap_or_else(Error::new);
        if err.fail() { Err(err) } else { Ok(()) }
    }

    pub fn kill(&self) -> Result<(), Error> {
        let raw = unsafe { LLDB_SBProcess_Kill(self.0) };
        let err = Error::from_raw(raw).unwrap_or_else(Error::new);
        if err.fail() { Err(err) } else { Ok(()) }
    }

    pub fn detach(&self) -> Result<(), Error> {
        let raw = unsafe { LLDB_SBProcess_Detach(self.0) };
        let err = Error::from_raw(raw).unwrap_or_else(Error::new);
        if err.fail() { Err(err) } else { Ok(()) }
    }

    pub fn num_threads(&self) -> u32 {
        unsafe { LLDB_SBProcess_GetNumThreads(self.0) }
    }

    pub fn thread_at(&self, idx: usize) -> Option<Thread> {
        let raw = unsafe { LLDB_SBProcess_GetThreadAtIndex(self.0, idx) };
        if raw.is_null() { return None; }
        let t = Thread::from_raw(raw);
        if t.is_valid() { Some(t) } else { None }
    }

    pub fn thread_by_id(&self, tid: u64) -> Option<Thread> {
        let raw = unsafe { LLDB_SBProcess_GetThreadByID(self.0, tid) };
        if raw.is_null() { return None; }
        let t = Thread::from_raw(raw);
        if t.is_valid() { Some(t) } else { None }
    }

    pub fn selected_thread(&self) -> Option<Thread> {
        let raw = unsafe { LLDB_SBProcess_GetSelectedThread(self.0) };
        if raw.is_null() { return None; }
        let t = Thread::from_raw(raw);
        if t.is_valid() { Some(t) } else { None }
    }

    pub fn set_selected_thread(&self, tid: u64) -> bool {
        unsafe { LLDB_SBProcess_SetSelectedThreadByID(self.0, tid) }
    }

    pub fn exit_status(&self) -> i32 {
        unsafe { LLDB_SBProcess_GetExitStatus(self.0) }
    }

    pub fn exit_description(&self) -> Option<&str> {
        let ptr = unsafe { LLDB_SBProcess_GetExitDescription(self.0) };
        if ptr.is_null() { None } else {
            Some(unsafe { CStr::from_ptr(ptr).to_str().unwrap_or("") })
        }
    }

    /// Read bytes from process memory at `addr`.
    pub fn read_memory(&self, addr: u64, buf: &mut [u8]) -> Result<usize, Error> {
        let err = Error::new();
        let n = unsafe {
            LLDB_SBProcess_ReadMemory(
                self.0, addr, buf.as_mut_ptr() as *mut _, buf.len(), err.as_raw())
        };
        if err.fail() { Err(err) } else { Ok(n) }
    }

    /// Write bytes to process memory at `addr`.
    pub fn write_memory(&self, addr: u64, buf: &[u8]) -> Result<usize, Error> {
        let err = Error::new();
        let n = unsafe {
            LLDB_SBProcess_WriteMemory(
                self.0, addr, buf.as_ptr() as *const _, buf.len(), err.as_raw())
        };
        if err.fail() { Err(err) } else { Ok(n) }
    }

    /// Get the SBBroadcaster for this process (to subscribe a listener).
    pub fn get_broadcaster(&self) -> Option<Broadcaster> {
        let raw = unsafe { LLDB_SBProcess_GetBroadcaster(self.0) };
        if raw.is_null() { None } else { Some(Broadcaster::from_raw(raw)) }
    }

    /// Extract the process state from an event (static method).
    pub fn state_from_event(event: &Event) -> State {
        State::from_raw(unsafe { LLDB_SBProcess_GetStateFromEvent(event.as_raw()) } as u32)
    }

    /// Returns true if the stop was an internal auto-continue (not user-visible).
    pub fn restarted_from_event(event: &Event) -> bool {
        unsafe { LLDB_SBProcess_GetRestartedFromEvent(event.as_raw()) }
    }

    /// Returns true if the event is a process state-change event.
    pub fn is_process_event(event: &Event) -> bool {
        unsafe { LLDB_SBProcess_EventIsProcessEvent(event.as_raw()) }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { LLDB_SBProcess_Destroy(self.0) };
        }
    }
}
