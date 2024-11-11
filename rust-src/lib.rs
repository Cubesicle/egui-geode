use std::{ffi::{c_char, c_void, CStr}, mem::transmute, sync::Arc};
use gd::{geode::log, gl};

mod gd;
mod gui;

#[no_mangle]
pub extern "C" fn init_gui() {
    let gl_ctx = Arc::new(unsafe {
        egui_glow::glow::Context::from_loader_function(gl::get_proc_address)
    });

    match gui::GuiBackend::from(gl_ctx) {
        Ok(gui) => *gui::GLOBAL_GUI.lock() = Some(gui),
        Err(e) => print_err(e),
    }
}

#[no_mangle]
pub extern "C" fn gui_context(reader: extern "C" fn(*const c_void)) {
    read_gui(|gui| reader(unsafe {
        transmute::<_, *const c_void>(gui.get_context())
    }));
}

#[no_mangle]
pub extern "C" fn gui_add_run_fn(run_fn: extern "C" fn(*const c_void)) {
    mut_gui(|gui| gui.add_run_fn(unsafe {
        transmute::<_, fn(&egui::Context)>(run_fn)
    }));
}

#[no_mangle]
pub extern "C" fn swap_buffers_detour(frame_w: f32, frame_h: f32) {
    mut_gui(|gui| gui.paint((frame_w, frame_h)));
}

#[no_mangle]
pub extern "C" fn gui_wants_pointer_input() -> bool {
    read_gui(|gui| gui.wants_pointer_input())
}

#[no_mangle]
pub extern "C" fn gui_wants_keyboard_input() -> bool {
    read_gui(|gui| gui.wants_keyboard_input())
}

#[no_mangle]
pub extern "C" fn is_pos_over_gui_area(x: f32, y: f32) -> bool {
    read_gui(|gui| gui.is_pos_over_area(egui::pos2(x, y)))
}

#[no_mangle]
pub extern "C" fn gui_send_mouse_pos(mouse_x: f32, mouse_y: f32) {
    mut_gui(|gui| gui.send_mouse_pos(
        egui::pos2(mouse_x, mouse_y)
    ));
}

#[no_mangle]
pub extern "C" fn gui_send_mouse_button(mouse_x: f32, mouse_y: f32, button: u32, pressed: bool) {
    mut_gui(|gui| gui.send_mouse_button(
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
    ));
}

#[no_mangle]
pub extern "C" fn gui_send_scroll_event(delta_x: f32, delta_y: f32) {
    mut_gui(|gui| gui.send_scroll_event(
        egui::vec2(delta_x, delta_y)
    ));
}

#[no_mangle]
pub extern "C" fn gui_send_touch(id: u64, phase: u32, touch_x: f32, touch_y: f32) {
    mut_gui(|gui| gui.send_touch(
        egui::TouchId(id),
        match phase {
            0 => egui::TouchPhase::Start,
            1 => egui::TouchPhase::Move,
            2 => egui::TouchPhase::End,
            _ => egui::TouchPhase::Cancel,
        },
        egui::pos2(touch_x, touch_y)
    ));
}

#[no_mangle]
pub extern "C" fn gui_send_key_press(key_name: *const c_char, pressed: bool, repeat: bool) {
    match unsafe { CStr::from_ptr(key_name) }.to_str() {
        Ok(key_name) => if let Some(key) = egui::Key::from_name(key_name) {
            mut_gui(|gui| gui.send_key_press(key, pressed, repeat));
        } else {
            log::error(format!("Could not convert {} to egui key.", key_name));
        },
        Err(e) => print_err(e),
    }
}

#[no_mangle]
pub extern "C" fn gui_send_text_input(text: *const c_char) {
    match unsafe { CStr::from_ptr(text) }.to_str() {
        Ok(text) => mut_gui(|gui| gui.send_text_input(text)),
        Err(e) => print_err(e),
    }
}

#[no_mangle]
pub extern "C" fn gui_send_modifiers(shift: bool, ctrl: bool, alt: bool, mac_cmd: bool, command: bool) {
    mut_gui(|gui| gui.send_modifiers(egui::Modifiers {
        alt,
        ctrl,
        shift,
        mac_cmd,
        command,
    }));
}

fn read_gui<R>(reader: impl FnOnce(&gui::GuiBackend) -> R) -> R {
    let gui = gui::GLOBAL_GUI.lock();
    let gui = gui.as_ref().unwrap();
    
    reader(gui)
}

fn mut_gui<R>(writer: impl FnOnce(&mut gui::GuiBackend) -> R) -> R {
    let mut gui = gui::GLOBAL_GUI.lock();
    let gui = gui.as_mut().unwrap();
    
    writer(gui)
}

fn print_err(e: impl std::error::Error) {
    log::error(e.to_string());
}