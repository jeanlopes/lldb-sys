use lldb_sys::*;
use std::ffi::CStr;
use crate::{Frame, StopReason};

/// Wraps `SBThreadRef`.
pub struct Thread(SBThreadRef);

impl Thread {
    pub(crate) fn from_raw(raw: SBThreadRef) -> Self {
        Self(raw)
    }

    pub fn is_valid(&self) -> bool {
        unsafe { LLDB_SBThread_IsValid(self.0) }
    }

    pub fn thread_id(&self) -> u64 {
        unsafe { LLDB_SBThread_GetThreadID(self.0) }
    }

    pub fn index_id(&self) -> u32 {
        unsafe { LLDB_SBThread_GetIndexID(self.0) }
    }

    pub fn name(&self) -> Option<&str> {
        let ptr = unsafe { LLDB_SBThread_GetName(self.0) };
        if ptr.is_null() { None } else {
            Some(unsafe { CStr::from_ptr(ptr).to_str().unwrap_or("") })
        }
    }

    pub fn queue_name(&self) -> Option<&str> {
        let ptr = unsafe { LLDB_SBThread_GetQueueName(self.0) };
        if ptr.is_null() { None } else {
            Some(unsafe { CStr::from_ptr(ptr).to_str().unwrap_or("") })
        }
    }

    pub fn stop_reason(&self) -> StopReason {
        StopReason::from_raw(unsafe { LLDB_SBThread_GetStopReason(self.0) } as u32)
    }

    pub fn stop_reason_data_count(&self) -> usize {
        unsafe { LLDB_SBThread_GetStopReasonDataCount(self.0) }
    }

    pub fn stop_reason_data_at(&self, idx: u32) -> u64 {
        unsafe { LLDB_SBThread_GetStopReasonDataAtIndex(self.0, idx) }
    }

    pub fn stop_description(&self) -> String {
        let mut buf = vec![0u8; 1024];
        unsafe {
            LLDB_SBThread_GetStopDescription(self.0, buf.as_mut_ptr() as *mut i8, buf.len());
        }
        let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        String::from_utf8_lossy(&buf[..end]).into_owned()
    }

    pub fn num_frames(&self) -> u32 {
        unsafe { LLDB_SBThread_GetNumFrames(self.0) }
    }

    pub fn frame_at(&self, idx: u32) -> Option<Frame> {
        let raw = unsafe { LLDB_SBThread_GetFrameAtIndex(self.0, idx) };
        if raw.is_null() { None } else { Some(Frame::from_raw(raw)) }
    }

    pub fn selected_frame(&self) -> Option<Frame> {
        let raw = unsafe { LLDB_SBThread_GetSelectedFrame(self.0) };
        if raw.is_null() { None } else { Some(Frame::from_raw(raw)) }
    }

    /// Step over the current source line (step over calls).
    pub fn step_over(&self) {
        unsafe { LLDB_SBThread_StepOver(self.0) };
    }

    /// Step into the current source line.
    pub fn step_into(&self) {
        unsafe { LLDB_SBThread_StepInto(self.0) };
    }

    /// Step out of the current function.
    pub fn step_out(&self) {
        unsafe { LLDB_SBThread_StepOut(self.0) };
    }

    /// Single instruction step.
    pub fn step_instruction(&self, step_over: bool) {
        unsafe { LLDB_SBThread_StepInstruction(self.0, step_over) };
    }

    /// Run until `addr` is reached.
    pub fn run_to_address(&self, addr: u64) {
        unsafe { LLDB_SBThread_RunToAddress(self.0, addr) };
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { LLDB_SBThread_Destroy(self.0) };
        }
    }
}
