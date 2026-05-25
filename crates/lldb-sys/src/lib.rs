#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::all)]

//! Raw FFI bindings to LLDB 19.
//!
//! All types are opaque handles (`*mut c_void` aliases).  Every handle must be
//! freed by the matching `LLDB_SB*_Destroy` function.
//!
//! Prefer using the `lldb-safe` crate which wraps these in safe Rust types.

// Pull in the bindgen-generated bindings (produced by build.rs at compile time).
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
