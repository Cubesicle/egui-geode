#![allow(static_mut_refs)]

use std::{ffi::c_void, mem::transmute};

static mut INPUT_STRING: String = String::new();
static mut CHECKBOX_CHECKED: bool = false;

#[no_mangle]
pub extern "C" fn setup(ctx: *const c_void) {
    let ctx = unsafe { transmute::<_, &egui::Context>(ctx) };
    ctx.set_theme(egui::Theme::Light);
}

#[no_mangle]
pub extern "C" fn run_fn(ctx: *const c_void) {
    let ctx = unsafe { transmute::<_, &egui::Context>(ctx) };
    egui::Window::new("Freak bot 😛").show(ctx, |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            ///////////////////////////////////////////////////////////////////////
            // IMPORTANT!!! Disabling selectable labels fixes a weird bug where  //
            // the selected text gets stuck on the cursor and it randomly causes //
            // frames to drop!!!!! Disable it!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!  //
            // It has something to do with inconsistent TypeIds across builds,   //
            // and I have no idea how to fix it. I literally tried everything    //
            // like dynamic linking and shit and I almost went insane. This is a //
            // temporary fix for now.                                            //
            ui.style_mut().interaction.selectable_labels = false;                //
            ///////////////////////////////////////////////////////////////////////

            for _ in 0..5 {
                ui.label("it works!");
            }
            ui.checkbox(unsafe { &mut CHECKBOX_CHECKED }, "Freak mode");
            ui.separator();

            ui.label("freaky message:");
            ui.text_edit_singleline(unsafe { &mut INPUT_STRING });
        });
    });
}

#[no_mangle]
pub extern "C" fn bingus(this_ptr: isize, fn_ptr: isize) {
    println!("hello from rust! >:)");
    
    unsafe { transmute::<_, fn(isize)>(fn_ptr)(this_ptr); }
}