use std::sync::{Arc, Mutex};
use anyhow::{ensure, Context, Result};

pub static GLOBAL_GUI: Mutex<Gui> = Mutex::new(Gui::new());

pub struct Gui {
    initialized: bool,
    egui_ctx: Option<egui::Context>,
    painter: Option<egui_glow::Painter>,
    checkbox_checked: bool,
}

impl Default for Gui {
    fn default() -> Self {
        Self::new()
    }
}

impl Gui {
    pub fn init(&mut self, gl_ctx: Arc<egui_glow::glow::Context>) -> Result<()> {
        ensure!(self.initialized == false, "Gui already initialized");

        self.painter = Some(egui_glow::Painter::new(gl_ctx, "", None, true).context("Failed to initialize painter")?);
        self.egui_ctx = Some(egui::Context::default());
        self.initialized = true;

        Ok(())
    }

    pub fn paint(&mut self) -> Result<()> {
        let egui_ctx = self.egui_ctx.as_ref().context("Gui not initialized")?;

        let egui::FullOutput {
            platform_output: _,
            mut textures_delta,
            shapes,
            pixels_per_point,
            viewport_output: _,
        } = egui_ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Hello egui!");
                ui.checkbox(&mut self.checkbox_checked, "Checkbox");
            });
        });

        Ok(())
    }

    const fn new() -> Self {
        Self {
            initialized: false,
            egui_ctx: None,
            painter: None,
            checkbox_checked: false,
        }
    }
}