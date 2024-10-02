use std::ffi::{c_void, CString};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn gl_get_proc_address(str: &str) -> *const c_void {
    let c_string = CString::new(str).unwrap();
    unsafe { _gl_get_proc_address(c_string.as_ptr()) }
}