use std::{mem::transmute, sync::Arc};
use anyhow::{Context, Error};
use error_messages::MUTEX_LOCK_FAIL;
use geode::{gl, log};

mod error_messages;
mod geode;
mod gui;

#[no_mangle]
pub extern "C" fn init_gui() {
    let _ = (|| gui::GLOBAL_GUI.lock().ok().context(MUTEX_LOCK_FAIL)?.init(
        Arc::new(unsafe { egui_glow::glow::Context::from_loader_function(|s| gl::get_proc_address(s)) })
    ))().map_err(print_err);
}

#[no_mangle]
pub extern "C" fn swap_buffers_detour(frame_w: f32, frame_h: f32) {
    let _ = (||
        gui::GLOBAL_GUI.lock().ok().context(MUTEX_LOCK_FAIL)?.paint((frame_w, frame_h))
    )().map_err(print_err);
}

#[no_mangle]
pub extern "C" fn gui_send_mouse_pos(mouse_x: f32, mouse_y: f32) {
    let _ = (|| gui::GLOBAL_GUI.lock().ok().context(MUTEX_LOCK_FAIL)?.send_mouse_pos(
        egui::pos2(mouse_x, mouse_y)
    ))().map_err(print_err);
}

//#[no_mangle]
//pub extern "C" fn gui_send_mouse_button(mouse_x: f32, mouse_y: f32, button: u32, pressed: bool) {
//    let _ = (|| gui::GLOBAL_GUI.lock().ok().context(MUTEX_LOCK_FAIL)?.send_mouse_button(
//        egui::pos2(mouse_x, mouse_y)
//    ))().map_err(print_err);
//}

#[no_mangle]
pub extern "C" fn gui_send_touch(id: u64, phase: u32, touch_x: f32, touch_y: f32) -> bool {
    (|| gui::GLOBAL_GUI.lock().ok().context(MUTEX_LOCK_FAIL)?.send_touch(
        egui::TouchId(id),
        match phase {
            0 => egui::TouchPhase::Start,
            1 => egui::TouchPhase::Move,
            2 => egui::TouchPhase::End,
            _ => egui::TouchPhase::Cancel,
        },
        egui::pos2(touch_x, touch_y)
    ))().map_err(print_err).unwrap_or_default()
}

fn print_err(e: Error) {
    log::error(e.to_string());
}