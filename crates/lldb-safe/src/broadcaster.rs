use lldb_sys::*;
use crate::listener::Listener;

/// Wraps `SBBroadcasterRef`.
pub struct Broadcaster(SBBroadcasterRef);

impl Broadcaster {
    pub(crate) fn from_raw(raw: SBBroadcasterRef) -> Self {
        Self(raw)
    }

    pub fn is_valid(&self) -> bool {
        unsafe { LLDB_SBBroadcaster_IsValid(self.0) }
    }

    /// Subscribe `listener` to `event_mask` bits.
    /// Pass `0x01` for `eBroadcastBitStateChanged`.
    pub fn add_listener(&self, listener: &Listener, event_mask: u32) -> bool {
        unsafe { LLDB_SBBroadcaster_AddListener(self.0, listener.as_raw(), event_mask) }
    }
}

impl Drop for Broadcaster {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { LLDB_SBBroadcaster_Destroy(self.0) };
        }
    }
}
