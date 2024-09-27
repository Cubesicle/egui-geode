use std::ffi::CString;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn log_info(string: String) {
    let c_string = CString::new(string);
    if let Ok(s) = c_string {
        unsafe { _log_info(CString::into_raw(s)); }
    }
}