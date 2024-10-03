use std::{ffi::c_uint, mem::transmute, sync::Arc};
use anyhow::{Context, Result};
use geode::{gl, log};

mod geode;
mod gui;

#[no_mangle]
pub extern "C" fn init_gui() {
    let _ = _init_gui().map_err(|e| log::error(e.to_string()));
}

#[no_mangle]
pub extern "C" fn bingus(this_ptr: isize, fn_ptr: isize) {
    log::debug(format!("Hello from rust!! >:) {:#06x} {:#06x}", this_ptr, fn_ptr));
    log::info(format!("example info"));
    log::warn(format!("example warning"));
    log::error(format!("example error"));
    
    unsafe { transmute::<_, fn(isize)>(fn_ptr)(this_ptr); }
}

#[no_mangle]
pub extern "C" fn swap_buffers_detour(frame_width: c_uint, frame_height: c_uint) {
    let _ = _swap_buffers_detour((frame_width, frame_height)).map_err(|e| log::error(e.to_string()));
}

fn _init_gui() -> Result<()> {
    gui::GLOBAL_GUI.lock().ok().context("wtf")?.init(
        Arc::new(unsafe { egui_glow::glow::Context::from_loader_function(|s| gl::get_proc_address(s)) })
    )?;
    
    Ok(())
}

fn _swap_buffers_detour(frame_size: (u32, u32)) -> Result<()> {
    gui::GLOBAL_GUI.lock().ok().context("wtf")?.paint(frame_size)?;

    Ok(())
}