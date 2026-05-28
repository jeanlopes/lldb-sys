use lldb_sys::*;
use crate::event::Event;

/// Wraps `SBListenerRef`.
pub struct Listener(SBListenerRef);

impl Listener {
    pub fn create(name: &str) -> Option<Self> {
        use std::ffi::CString;
        let c_name = CString::new(name).ok()?;
        let raw = unsafe { LLDB_SBListener_Create(c_name.as_ptr()) };
        if raw.is_null() { None } else { Some(Self(raw)) }
    }

    pub(crate) fn from_raw(raw: SBListenerRef) -> Self {
        Self(raw)
    }

    pub(crate) fn as_raw(&self) -> SBListenerRef {
        self.0
    }

    pub fn is_valid(&self) -> bool {
        unsafe { LLDB_SBListener_IsValid(self.0) }
    }

    /// Block until an event arrives or `timeout_secs` elapses.
    /// Returns `Some(event)` on success, `None` on timeout.
    pub fn wait_for_event(&self, timeout_secs: u32) -> Option<Event> {
        let mut event_out: SBEventRef = std::ptr::null_mut();
        let got = unsafe {
            LLDB_SBListener_WaitForEvent(self.0, timeout_secs, &mut event_out)
        };
        if got && !event_out.is_null() {
            Some(Event::from_raw(event_out))
        } else {
            None
        }
    }
}

impl Drop for Listener {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { LLDB_SBListener_Destroy(self.0) };
        }
    }
}
