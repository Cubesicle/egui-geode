use std::ffi::CString;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn debug(string: String) {
    if let Ok(s) = CString::new(string) {
        unsafe { _log_debug(CString::into_raw(s)); }
    }
}

pub fn info(string: String) {
    if let Ok(s) = CString::new(string) {
        unsafe { _log_info(CString::into_raw(s)); }
    }
}


pub fn warn(string: String) {
    if let Ok(s) = CString::new(string) {
        unsafe { _log_warn(CString::into_raw(s)); }
    }
}


pub fn error(string: String) {
    if let Ok(s) = CString::new(string) {
        unsafe { _log_error(CString::into_raw(s)); }
    }
}