use std::{sync::{Arc, Mutex}, time::SystemTime};
use anyhow::{ensure, Context, Result};
use crate::error_messages::{GUI_ALREADY_INITIALIZED, GUI_NOT_INITIALIZED, PAINTER_INITIALIZE_FAIL};

pub static GLOBAL_GUI: Mutex<GuiBackend> = Mutex::new(GuiBackend::new());

pub struct GuiBackend {
    initialized: bool,
    egui_ctx: Option<egui::Context>,
    painter: Option<egui_glow::Painter>,
    modifiers: egui::Modifiers,
    events: Vec<egui::Event>,
    checkbox_checked: bool,
}

impl Default for GuiBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl GuiBackend {
    pub fn init(&mut self, gl_ctx: Arc<egui_glow::glow::Context>) -> Result<()> {
        ensure!(!self.initialized, GUI_ALREADY_INITIALIZED);

        self.painter = Some(egui_glow::Painter::new(gl_ctx, "", None, true).context(PAINTER_INITIALIZE_FAIL)?);
        self.egui_ctx = Some(egui::Context::default());
        self.initialized = true;

        Ok(())
    }

    // Code taken from https://github.com/spinningtoilet0/egui_glow_internal
    pub fn paint(&mut self, frame_size: (f32, f32)) -> Result<()> {
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
                max: egui::pos2(frame_size.0, frame_size.1),
            }),
            time: Some(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs_f64()),
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
            [frame_size.0 as u32, frame_size.1 as u32],
            egui_ctx.pixels_per_point(),
            &clipped_primitives,
        );
    
        for id in textures_delta.free.drain(..) {
            painter.free_texture(id);
        }

        Ok(())
    }
    
    pub fn send_mouse_pos(&mut self, pos: egui::Pos2) -> Result<()> {
        ensure!(self.initialized, GUI_NOT_INITIALIZED);

        self.events.push(egui::Event::PointerMoved(pos));
        
        Ok(())
    }
    
    pub fn send_mouse_button(&mut self, pos: egui::Pos2, button: egui::PointerButton, pressed: bool) -> Result<bool> {
        let egui_ctx = self.egui_ctx.as_ref().context(GUI_NOT_INITIALIZED)?;
        let should_block = pressed && egui_ctx.wants_pointer_input();
        
        self.events.push(egui::Event::PointerButton {
            pos,
            button,
            pressed,
            modifiers: self.modifiers,
        });

        Ok(should_block)
    }
    
    pub fn send_touch(&mut self, id: egui::TouchId, phase: egui::TouchPhase, pos: egui::Pos2) -> Result<bool> {
        ensure!(self.initialized, GUI_NOT_INITIALIZED);
        
        use egui::TouchPhase::{Start, End};
        use egui::PointerButton::Primary;
        let should_block = match phase {
            Start => {
                self.send_mouse_button(pos, Primary, true)?
            },
            End => {
                self.send_mouse_button(pos, Primary, false)?; false
            },
            _ => false,
        };
        
        self.events.push(egui::Event::Touch {
            device_id: egui::TouchDeviceId(0),
            id,
            phase,
            pos,
            force: None,
        });

        Ok(should_block)
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