pub mod gl {
    use std::{ffi::{c_void, CString}, mem::transmute};
    use super::ffi;

    pub fn get_proc_address(str: &str) -> *const c_void {
        let c_string = CString::new(str).unwrap_or_default();
        unsafe { transmute(ffi::gl_get_proc_address(c_string.as_ptr())) }
    }
}

pub mod log {
    use std::ffi::CString;
    use super::ffi;

    pub fn debug(string: String) {
        let s = CString::new(string).unwrap_or_default().into_raw();
        unsafe { ffi::log_debug(s); }
    }
    
    pub fn info(string: String) {
        let s = CString::new(string).unwrap_or_default().into_raw();
        unsafe { ffi::log_info(s); }
    }
    
    pub fn warn(string: String) {
        let s = CString::new(string).unwrap_or_default().into_raw();
        unsafe { ffi::log_warn(s); }
    }
    
    pub fn error(string: String) {
        let s = CString::new(string).unwrap_or_default().into_raw();
        unsafe { ffi::log_error(s); }
    }
}

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("../include/ffi.h");

        unsafe fn gl_get_proc_address(str: *const c_char) -> usize;
        unsafe fn log_debug(str: *const c_char);
        unsafe fn log_info(str: *const c_char);
        unsafe fn log_warn(str: *const c_char);
        unsafe fn log_error(str: *const c_char);
    }
}
