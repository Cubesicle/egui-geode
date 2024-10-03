use std::ffi::{c_void, CString};
use super::bindings::gl_get_proc_address;

pub fn get_proc_address(str: &str) -> *const c_void {
    let c_string = CString::new(str).unwrap_or_default();
    unsafe { gl_get_proc_address(c_string.as_ptr()) }
}