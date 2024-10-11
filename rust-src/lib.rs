use std::sync::Arc;
use anyhow::Context;
use error_messages::MUTEX_LOCK_FAIL;
use geode::{gl, log};

pub mod error_messages;
pub mod geode;
pub mod gui;

pub fn init_gui(run_fn: fn(&egui::Context)) {
    let _ = (|| gui::GLOBAL_GUI.lock().ok().context(MUTEX_LOCK_FAIL)?.init(
        Arc::new(unsafe { egui_glow::glow::Context::from_loader_function(|s| gl::get_proc_address(s)) }),
        run_fn
    ))().map_err(|e| log::error(e.to_string()));
}