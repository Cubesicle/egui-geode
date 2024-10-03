use std::sync::Arc;
use anyhow::{Context, Result};
use crate::geode::{gl, log};
use crate::gui;

pub fn init_gui() -> Result<()> {
    gui::GLOBAL_GUI.lock().ok().context("wtf")?.init(
        Arc::new(unsafe { egui_glow::glow::Context::from_loader_function(|s| gl::get_proc_address(s)) })
    )?;
    
    Ok(())
}

pub fn swap_buffers_detour(frame_size: (u32, u32), mouse_pos: (f32, f32)) -> Result<()> {
    let mut gui = gui::GLOBAL_GUI.lock().ok().context("wtf")?;
    gui.register_event(egui::Event::PointerMoved(egui::Pos2::from(mouse_pos)));
    gui.paint(frame_size)?;

    Ok(())
}