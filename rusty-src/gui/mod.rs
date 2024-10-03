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

    // Code taken from https://github.com/spinningtoilet0/egui_glow_internal
    pub fn paint(&mut self, frame_size: (u32, u32)) -> Result<()> {
        let egui_ctx = self.egui_ctx.as_ref().context("Gui not initialized")?;
        let painter = self.painter.as_mut().context("Gui not initialized")?;

        let egui::FullOutput {
            platform_output: _,
            mut textures_delta,
            shapes,
            pixels_per_point,
            viewport_output: _,
        } = egui_ctx.run(Self::get_raw_input(frame_size), |ctx| {
            egui::Window::new("Freak bot 😝").collapsible(false).show(ctx, |ui| {
                ui.label("it works!");
                ui.label("it works!");
                ui.label("it works!");
                ui.label("it works!");
                ui.label("it works!");
                ui.checkbox(&mut self.checkbox_checked, "Freak mode");
            });
        });
        
        for (id, image_delta) in textures_delta.set {
            painter.set_texture(id, &image_delta);
        }
    
        let clipped_primitives = egui_ctx.tessellate(shapes, pixels_per_point);
    
        painter.paint_primitives(
            [frame_size.0, frame_size.1],
            egui_ctx.pixels_per_point(),
            &clipped_primitives,
        );
    
        for id in textures_delta.free.drain(..) {
            painter.free_texture(id);
        }

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
    
    fn get_raw_input(frame_size: (u32, u32)) -> egui::RawInput {
        egui::RawInput {
            screen_rect: Some(egui::Rect {
                min: egui::pos2(0.0, 0.0),
                max: egui::pos2(frame_size.0 as f32, frame_size.1 as f32),
            }),
            ..Default::default()
        }
    }
}