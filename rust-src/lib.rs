use std::sync::{Arc, Mutex};
use anyhow::{Context, Error};
use error_messages::MUTEX_LOCK_FAIL;
use geode::{gl, log};

pub mod error_messages;
pub mod geode;
pub mod gui;

pub fn init_gui(run_fn: Arc<Mutex<dyn FnMut(&egui::Context) + Send>>) {
    let _ = (|| gui::GLOBAL_GUI.lock().ok().context(MUTEX_LOCK_FAIL)?.init(
        Arc::new(unsafe { egui_glow::glow::Context::from_loader_function(|s| gl::get_proc_address(s)) }),
        run_fn
    ))().map_err(print_err);
}

fn print_err(e: Error) {
    log::error(e.to_string());
}