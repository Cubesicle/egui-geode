use std::{ffi::{c_float, c_uint}, mem::transmute};
use anyhow::Error;
use geode::log;

mod error_messages;
mod ffi;
mod geode;
mod gui;

#[no_mangle]
pub extern "C" fn init_gui() {
    let _ = ffi::init_gui().map_err(print_err);
}

#[no_mangle]
pub extern "C" fn swap_buffers_detour(frame_width: c_uint, frame_height: c_uint) {
    let _ = ffi::swap_buffers_detour((frame_width, frame_height)).map_err(print_err);
}

#[no_mangle]
pub extern "C" fn gui_send_mouse_pos(mouse_x: c_float, mouse_y: c_float) {
    let _ = ffi::gui_send_mouse_pos((mouse_x, mouse_y)).map_err(print_err);
}

#[no_mangle]
pub extern "C" fn gui_send_mouse_btn(mouse_x: c_float, mouse_y: c_float, right: bool, pressed: bool) {
    let _ = ffi::gui_send_mouse_btn((mouse_x, mouse_y), right, pressed).map_err(print_err);
}

#[no_mangle]
pub extern "C" fn gui_wants_pointer_input() -> bool {
    ffi::gui_wants_pointer_input().map_err(print_err).unwrap_or_default()
}

#[no_mangle]
pub extern "C" fn bingus(this_ptr: isize, fn_ptr: isize) {
    log::debug(format!("Hello from rust!! >:) {:#06x} {:#06x}", this_ptr, fn_ptr));
    log::info(format!("example info"));
    log::warn(format!("example warning"));
    log::error(format!("example error"));
    
    unsafe { transmute::<_, fn(isize)>(fn_ptr)(this_ptr); }
}

fn print_err(e: Error) {
    log::error(e.to_string());
}