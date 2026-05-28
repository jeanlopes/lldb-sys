use lldb_sys::*;

/// Wraps `SBEventRef`. Owns the heap-allocated event object.
pub struct Event(SBEventRef);

impl Event {
    pub(crate) fn from_raw(raw: SBEventRef) -> Self {
        Self(raw)
    }

    pub(crate) fn as_raw(&self) -> SBEventRef {
        self.0
    }

    pub fn is_valid(&self) -> bool {
        unsafe { LLDB_SBEvent_IsValid(self.0) }
    }

    pub fn event_type(&self) -> u32 {
        unsafe { LLDB_SBEvent_GetType(self.0) }
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { LLDB_SBEvent_Destroy(self.0) };
        }
    }
}
