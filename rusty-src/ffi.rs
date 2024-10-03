use std::sync::Arc;
use anyhow::{Context, Result};
use crate::geode::gl;
use crate::gui;

pub fn init_gui() -> Result<()> {
    gui::GLOBAL_GUI.lock().ok().context("wtf")?.init(
        Arc::new(unsafe { egui_glow::glow::Context::from_loader_function(|s| gl::get_proc_address(s)) })
    )?;
    
    Ok(())
}

pub fn swap_buffers_detour(frame_size: (u32, u32)) -> Result<()> {
    gui::GLOBAL_GUI.lock().ok().context("wtf")?.paint(frame_size)?;

    Ok(())
}