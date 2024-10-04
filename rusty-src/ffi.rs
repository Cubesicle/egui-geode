use std::sync::Arc;
use anyhow::{Context, Result};
use crate::error_messages::MUTEX_LOCK_FAIL;
use crate::geode::{gl, log};
use crate::gui;

pub fn init_gui() -> Result<()> {
    gui::GLOBAL_GUI.lock().ok().context(MUTEX_LOCK_FAIL)?.init(
        Arc::new(unsafe { egui_glow::glow::Context::from_loader_function(|s| gl::get_proc_address(s)) })
    )?;
    
    Ok(())
}

pub fn swap_buffers_detour(frame_size: (u32, u32)) -> Result<()> {
    gui::GLOBAL_GUI.lock().ok().context(MUTEX_LOCK_FAIL)?.paint(frame_size)?;

    Ok(())
}

pub fn gui_send_mouse_pos(mouse_pos: (f32, f32)) -> Result<()> {
    gui::GLOBAL_GUI.lock().ok().context(MUTEX_LOCK_FAIL)?.register_event(
        egui::Event::PointerMoved(egui::Pos2::from(mouse_pos))       
    )?;

    Ok(())
}

pub fn gui_wants_pointer_input() -> Result<bool> {
    gui::GLOBAL_GUI.lock().ok().context(MUTEX_LOCK_FAIL)?.wants_pointer_input()
}

pub fn gui_send_mouse_btn(mouse_pos: (f32, f32), right: bool, pressed: bool) -> Result<()> {
    gui::GLOBAL_GUI.lock().ok().context(MUTEX_LOCK_FAIL)?.register_event(
        egui::Event::PointerButton {
            pos: egui::Pos2::from(mouse_pos),
            button: if right {
                egui::PointerButton::Secondary
            } else {
                egui::PointerButton::Primary
            },
            pressed,
            modifiers: egui::Modifiers::default(), // TODO: add modifiers
        }
    )?;
    
    Ok(())
}