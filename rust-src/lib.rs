use std::{ffi::{c_char, c_void, CStr}, mem::transmute, sync::Arc};
use anyhow::{Context, Error};
use gd::{gl, log};

mod errors;
mod gd;
mod gui;

#[no_mangle]
pub extern "C" fn init_gui() {
    let _ = gui::GLOBAL_GUI.lock().init(Arc::new(unsafe {
        egui_glow::glow::Context::from_loader_function(gl::get_proc_address)
    })).map_err(print_err);
}

#[no_mangle]
pub extern "C" fn gui_context(reader: extern "C" fn(*const c_void)) {
    let _ = (|| {
        let gui = gui::GLOBAL_GUI.lock();
        let ctx = gui.get_context()?;
        Ok(unsafe { transmute::<_, *const c_void>(ctx) })
    })().and_then(|ctx| Ok(reader(ctx))).map_err(print_err);
}

#[no_mangle]
pub extern "C" fn gui_add_run_fn(run_fn: extern "C" fn(*const c_void)) {
    let _ = gui::GLOBAL_GUI.lock().add_run_fn(unsafe {
        transmute::<_, fn(&egui::Context)>(run_fn)
    }).map_err(print_err);
}

#[no_mangle]
pub extern "C" fn swap_buffers_detour(frame_w: f32, frame_h: f32) {
    let _ = gui::GLOBAL_GUI.lock().paint(
        (frame_w, frame_h)
    ).map_err(print_err);
}

#[no_mangle]
pub extern "C" fn gui_wants_pointer_input() -> bool {
    gui::GLOBAL_GUI.lock().wants_pointer_input()
        .map_err(print_err).unwrap_or_default()
}

#[no_mangle]
pub extern "C" fn gui_wants_keyboard_input() -> bool {
    gui::GLOBAL_GUI.lock().wants_keyboard_input()
        .map_err(print_err).unwrap_or_default()
}

#[no_mangle]
pub extern "C" fn is_pos_over_gui_area(x: f32, y: f32) -> bool {
    gui::GLOBAL_GUI.lock().is_pos_over_area(
        egui::pos2(x, y)
    ).map_err(print_err).unwrap_or_default()
}

#[no_mangle]
pub extern "C" fn gui_send_mouse_pos(mouse_x: f32, mouse_y: f32) {
    let _ = gui::GLOBAL_GUI.lock().send_mouse_pos(
        egui::pos2(mouse_x, mouse_y)
    ).map_err(print_err);
}

#[no_mangle]
pub extern "C" fn gui_send_mouse_button(mouse_x: f32, mouse_y: f32, button: u32, pressed: bool) {
    let _ = gui::GLOBAL_GUI.lock().send_mouse_button(
        egui::pos2(mouse_x, mouse_y),
        match button {
            0 => egui::PointerButton::Primary,
            1 => egui::PointerButton::Secondary,
            2 => egui::PointerButton::Middle,
            3 => egui::PointerButton::Extra1,
            4 => egui::PointerButton::Extra2,
            _ => egui::PointerButton::Primary,
        },
        pressed
    ).map_err(print_err);
}

#[no_mangle]
pub extern "C" fn gui_send_scroll_event(delta_x: f32, delta_y: f32) {
    let _ = gui::GLOBAL_GUI.lock().send_scroll_event(
        egui::vec2(delta_x, delta_y)
    ).map_err(print_err);
}

#[no_mangle]
pub extern "C" fn gui_send_touch(id: u64, phase: u32, touch_x: f32, touch_y: f32) {
    let _ = gui::GLOBAL_GUI.lock().send_touch(
        egui::TouchId(id),
        match phase {
            0 => egui::TouchPhase::Start,
            1 => egui::TouchPhase::Move,
            2 => egui::TouchPhase::End,
            _ => egui::TouchPhase::Cancel,
        },
        egui::pos2(touch_x, touch_y)
    ).map_err(print_err);
}

#[no_mangle]
pub extern "C" fn gui_send_key_press(key_name: *const c_char, pressed: bool, repeat: bool) {
    let _ = (|| {
        let key_name = unsafe { CStr::from_ptr(key_name) }.to_str()?;
        let key = egui::Key::from_name(key_name).context(format!(
            "Could not convert {} to egui key.", key_name
        ))?;

        gui::GLOBAL_GUI.lock().send_key_press(
            key,
            pressed,
            repeat
        )
    })().map_err(print_err);
}

#[no_mangle]
pub extern "C" fn gui_send_text_input(text: *const c_char) {
    let _ = (||
        gui::GLOBAL_GUI.lock().send_text_input(unsafe {
            CStr::from_ptr(text)
        }.to_str()?)
    )().map_err(print_err);
}

#[no_mangle]
pub extern "C" fn gui_send_modifiers(shift: bool, ctrl: bool, alt: bool, mac_cmd: bool, command: bool) {
    let _ = gui::GLOBAL_GUI.lock().send_modifiers(egui::Modifiers {
        alt,
        ctrl,
        shift,
        mac_cmd,
        command,
    }) .map_err(print_err);
}

fn print_err(e: Error) {
    log::error(e.to_string());
}