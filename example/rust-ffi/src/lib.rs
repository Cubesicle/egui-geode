use std::{mem::transmute, sync::{Arc, Mutex}};
use egui_geode_rust::geode::log;

static mut INPUT_STRING: String = String::new();
static mut CHECKBOX_CHECKED: bool = false;

#[no_mangle]
pub extern "C" fn init_gui() {
    egui_geode_rust::init_gui(|ctx: &egui::Context| {
        egui::Window::new("Freak bot 😛").show(ctx, |ui| {
            ui.label("it works!");
            ui.label("it works!");
            ui.label("it works!");
            ui.label("it works!");
            ui.label("it works!");
            ui.checkbox(unsafe { &mut CHECKBOX_CHECKED }, "Freak mode");
            ui.separator();

            ui.label("freaky message:");
            ui.text_edit_singleline(unsafe { &mut INPUT_STRING });
        });
    });
}

#[no_mangle]
pub extern "C" fn bingus(this_ptr: isize, fn_ptr: isize) {
    log::debug(format!("Hello from rust!! >:) {:#06x} {:#06x}", this_ptr, fn_ptr));
    log::info(format!("example info"));
    log::warn(format!("example warning"));
    log::error(format!("example error"));
    
    unsafe { transmute::<_, fn(isize)>(fn_ptr)(this_ptr); }
}