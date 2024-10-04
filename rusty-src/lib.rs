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
    let _ = (|| gui::GLOBAL_GUI.lock().ok().context(MUTEX_LOCK_FAIL)?.register_event(
        egui::Event::PointerMoved(egui::pos2(mouse_x, mouse_y))       
    ))().map_err(print_err);
}

#[no_mangle]
pub extern "C" fn gui_send_mouse_btn(mouse_x: f32, mouse_y: f32, right: bool, pressed: bool) {
    let _ = (|| gui::GLOBAL_GUI.lock().ok().context(MUTEX_LOCK_FAIL)?.register_event(
        egui::Event::PointerButton {
            pos: egui::pos2(mouse_x, mouse_y),
            button: if right {
                egui::PointerButton::Secondary
            } else {
                egui::PointerButton::Primary
            },
            pressed,
            modifiers: egui::Modifiers::default(), // TODO: add modifiers
        }
    ))().map_err(print_err);
}

#[no_mangle]
pub extern "C" fn gui_wants_pointer_input() -> bool {
    (||
        gui::GLOBAL_GUI.lock().ok().context(MUTEX_LOCK_FAIL)?.wants_pointer_input()
    )().map_err(print_err).unwrap_or_default()
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