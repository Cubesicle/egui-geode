pub mod geode {
    pub mod log {
        #![allow(dead_code)]
    
        use std::ffi::CString;
        use super::super::bindings::{log_debug, log_info, log_warn, log_error};
        
        macro_rules! define_logging_fn {
            ($wrapper_fn_name:ident, $original_fn_name:ident) => {
                pub fn $wrapper_fn_name<S: AsRef<str>>(string: S) {
                    let s = CString::new(string.as_ref()).unwrap_or_default().into_raw();
                    unsafe { $original_fn_name(s) };
                }
            };
        }
        
        define_logging_fn!(debug, log_debug);
        define_logging_fn!(info, log_info);
        define_logging_fn!(warn, log_warn);
        define_logging_fn!(error, log_error);
    }
}

pub mod gl {
    use std::ffi::{c_void, CString};
    use super::bindings::gl_get_proc_address;
    
    pub fn get_proc_address(str: &str) -> *const c_void {
        let c_string = CString::new(str).unwrap_or_default();
        unsafe { gl_get_proc_address(c_string.as_ptr()) }
    }
}

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}