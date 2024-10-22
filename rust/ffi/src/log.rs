#![allow(dead_code)]

use std::ffi::CString;
use super::bindings::{log_debug, log_info, log_warn, log_error};

pub fn debug(string: String) {
    let s = CString::new(string).unwrap_or_default().into_raw();
    unsafe { log_debug(s); }
}

pub fn info(string: String) {
    let s = CString::new(string).unwrap_or_default().into_raw();
    unsafe { log_info(s); }
}

pub fn warn(string: String) {
    let s = CString::new(string).unwrap_or_default().into_raw();
    unsafe { log_warn(s); }
}

pub fn error(string: String) {
    let s = CString::new(string).unwrap_or_default().into_raw();
    unsafe { log_error(s); }
}