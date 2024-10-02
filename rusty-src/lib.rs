mod geode;

use std::mem::transmute;

use geode::{gl, log};

#[no_mangle]
pub extern "C" fn bingus(this_ptr: isize, fn_ptr: isize) {
    log::debug(format!("Hello from rust!! >:) {:#06x} {:#06x}", this_ptr, fn_ptr));
    log::info(format!("example info"));
    log::warn(format!("example warning"));
    log::error(format!("example error"));
    unsafe { transmute::<_, fn(isize)>(fn_ptr)(this_ptr); }
}

#[no_mangle]
pub extern "C" fn swap_buffers_detour() {
    let thing = unsafe {
        gl::gl_get_proc_address("wglCopyContext")
    };
    log::debug(format!("{:?}", thing));
}