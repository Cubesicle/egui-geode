use std::sync::{Arc, Mutex};
use anyhow::{ensure, Context, Result};
use crate::error_messages::{GUI_ALREADY_INITIALIZED, GUI_NOT_INITIALIZED, PAINTER_INITIALIZE_FAIL};

pub static GLOBAL_GUI: Mutex<Gui> = Mutex::new(Gui::new());

pub struct Gui {
    initialized: bool,
    egui_ctx: Option<egui::Context>,
    painter: Option<egui_glow::Painter>,
    modifiers: egui::Modifiers,
    events: Vec<egui::Event>,
    checkbox_checked: bool,
}

impl Default for Gui {
    fn default() -> Self {
        Self::new()
    }
}

impl Gui {
    pub fn init(&mut self, gl_ctx: Arc<egui_glow::glow::Context>) -> Result<()> {
        ensure!(self.initialized == false, GUI_ALREADY_INITIALIZED);

        self.painter = Some(egui_glow::Painter::new(gl_ctx, "", None, true).context(PAINTER_INITIALIZE_FAIL)?);
        self.egui_ctx = Some(egui::Context::default());
        self.initialized = true;

        Ok(())
    }

    // Code taken from https://github.com/spinningtoilet0/egui_glow_internal
    pub fn paint(&mut self, frame_size: (u32, u32)) -> Result<()> {
        let egui_ctx = self.egui_ctx.as_ref().context(GUI_NOT_INITIALIZED)?;
        let painter = self.painter.as_mut().context(GUI_NOT_INITIALIZED)?;

        let egui::FullOutput {
            platform_output: _,
            mut textures_delta,
            shapes,
            pixels_per_point,
            viewport_output: _,
        } = egui_ctx.run(egui::RawInput {
            screen_rect: Some(egui::Rect {
                min: egui::pos2(0.0, 0.0),
                max: egui::pos2(frame_size.0 as f32, frame_size.1 as f32),
            }),
            modifiers: self.modifiers,
            events: std::mem::take(&mut self.events),
            ..Default::default()
        }, |ctx| {
            egui::Window::new("Freak bot 😝").show(ctx, |ui| {
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
    
    pub fn register_event(&mut self, event: egui::Event) -> Result<()> {
        ensure!(self.initialized == true, GUI_NOT_INITIALIZED);

        self.events.push(event);

        Ok(())
    }
    
    pub fn wants_pointer_input(&self) -> Result<bool> {
        Ok(self.egui_ctx.as_ref().context(GUI_NOT_INITIALIZED)?.wants_pointer_input())
    }

    pub fn wants_keyboard_input(&self) -> Result<bool> {
        Ok(self.egui_ctx.as_ref().context(GUI_NOT_INITIALIZED)?.wants_keyboard_input())
    }

    const fn new() -> Self {
        Self {
            initialized: false,
            egui_ctx: None,
            painter: None,
            modifiers: egui::Modifiers {
                alt: false,
                ctrl: false,
                shift: false,
                mac_cmd: false,
                command: false,
            },
            events: Vec::new(),
            checkbox_checked: false,
        }
    }
}