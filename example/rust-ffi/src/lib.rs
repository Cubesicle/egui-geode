use std::{ffi::c_void, mem::transmute};

static mut INPUT_STRING: String = String::new();
static mut CHECKBOX_CHECKED: bool = false;

#[no_mangle]
pub extern "C" fn run_fn(ctx: *const c_void) {
    let ctx = unsafe { transmute::<_, &egui::Context>(ctx) };
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
}

#[no_mangle]
pub extern "C" fn bingus(this_ptr: isize, fn_ptr: isize) {
    println!("hello from rust! >:)");
    
    unsafe { transmute::<_, fn(isize)>(fn_ptr)(this_ptr); }
}