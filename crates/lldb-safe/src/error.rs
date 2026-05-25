use lldb_sys::*;
use std::ffi::CStr;
use std::fmt;

/// Wraps `SBErrorRef`.  Owns the allocation; freed on drop.
pub struct Error(SBErrorRef);

impl Error {
    pub fn new() -> Self {
        Self(unsafe { LLDB_SBError_Create() })
    }

    pub(crate) fn from_raw(raw: SBErrorRef) -> Option<Self> {
        if raw.is_null() { None } else { Some(Self(raw)) }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { LLDB_SBError_IsValid(self.0) }
    }

    pub fn success(&self) -> bool {
        unsafe { LLDB_SBError_Success(self.0) }
    }

    pub fn fail(&self) -> bool {
        unsafe { LLDB_SBError_Fail(self.0) }
    }

    pub fn code(&self) -> u32 {
        unsafe { LLDB_SBError_GetError(self.0) }
    }

    pub fn message(&self) -> Option<&str> {
        let ptr = unsafe { LLDB_SBError_GetCString(self.0) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr).to_str().unwrap_or("<invalid utf8>") })
        }
    }

    pub(crate) fn as_raw(&self) -> SBErrorRef {
        self.0
    }
}

impl Default for Error {
    fn default() -> Self { Self::new() }
}

impl Drop for Error {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { LLDB_SBError_Destroy(self.0) };
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.message() {
            Some(msg) => write!(f, "{}", msg),
            None      => write!(f, "LLDB error (code {})", self.code()),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "lldb::Error({})", self)
    }
}

impl std::error::Error for Error {}
