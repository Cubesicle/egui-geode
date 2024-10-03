use std::{ffi::{c_float, c_uint}, mem::transmute};
use anyhow::Result;
use geode::log;

mod ffi;
mod geode;
mod gui;

#[no_mangle]
pub extern "C" fn init_gui() {
    print_err(ffi::init_gui());
}

#[no_mangle]
pub extern "C" fn swap_buffers_detour(frame_width: c_uint, frame_height: c_uint, mouse_x: c_float, mouse_y: c_float) {
    print_err(ffi::swap_buffers_detour((frame_width, frame_height), (mouse_x, mouse_y)));
}

#[no_mangle]
pub extern "C" fn bingus(this_ptr: isize, fn_ptr: isize) {
    log::debug(format!("Hello from rust!! >:) {:#06x} {:#06x}", this_ptr, fn_ptr));
    log::info(format!("example info"));
    log::warn(format!("example warning"));
    log::error(format!("example error"));
    
    unsafe { transmute::<_, fn(isize)>(fn_ptr)(this_ptr); }
}

fn print_err(result: Result<()>) {
    let _ = result.map_err(|e| log::error(e.to_string()));
}