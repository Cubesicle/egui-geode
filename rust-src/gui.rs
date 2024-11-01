use std::{sync::Arc, time::SystemTime};
use anyhow::{ensure, Context, Result};
use parking_lot::Mutex;
use crate::errors::{GUI_ALREADY_INITIALIZED, GUI_NOT_INITIALIZED, PAINTER_INITIALIZE_FAIL};

pub static GLOBAL_GUI: Mutex<GuiBackend> = Mutex::new(GuiBackend::new());

pub struct GuiBackend {
    initialized: bool,
    egui_ctx: Option<egui::Context>,
    painter: Option<egui_glow::Painter>,
    modifiers: egui::Modifiers,
    events: Vec<egui::Event>,
    run_fns: Vec<fn(&egui::Context)>,
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
    
    pub fn get_context(&self) -> Result<&egui::Context> {
        self.egui_ctx.as_ref().context(GUI_NOT_INITIALIZED)
    }
    
    pub fn add_run_fn(&mut self, func: fn(&egui::Context)) -> Result<()> {
        ensure!(self.initialized, GUI_NOT_INITIALIZED);
        
        self.run_fns.push(func);

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
            for f in &self.run_fns { f(ctx); }
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
    
    pub fn wants_pointer_input(&self) -> Result<bool> {
        Ok(self.egui_ctx.as_ref().context(GUI_NOT_INITIALIZED)?.wants_pointer_input())
    }

    pub fn wants_keyboard_input(&self) -> Result<bool> {
        Ok(self.egui_ctx.as_ref().context(GUI_NOT_INITIALIZED)?.wants_keyboard_input())
    }
    
    pub fn is_pos_over_area(&self, pos: egui::Pos2) -> Result<bool> {
        let egui_ctx = self.egui_ctx.as_ref().context(GUI_NOT_INITIALIZED)?;

        if let Some(layer) = egui_ctx.layer_id_at(pos) {
            if layer.order == egui::Order::Background {
                Ok(!egui_ctx.viewport(|v| v.this_pass.unused_rect.contains(pos)))
            } else {
                Ok(true)
            }
        } else {
            Ok(false)
        }
    }
    
    pub fn send_mouse_pos(&mut self, pos: egui::Pos2) -> Result<()> {
        ensure!(self.initialized, GUI_NOT_INITIALIZED);

        self.events.push(egui::Event::PointerMoved(pos));
        
        Ok(())
    }
    
    pub fn send_mouse_button(&mut self, pos: egui::Pos2, button: egui::PointerButton, pressed: bool) -> Result<()> {
        ensure!(self.initialized, GUI_NOT_INITIALIZED);
        
        self.events.push(egui::Event::PointerButton {
            pos,
            button,
            pressed,
            modifiers: self.modifiers,
        });
        
        Ok(())
    }
    
    pub fn send_scroll_event(&mut self, delta: egui::Vec2) -> Result<()> {
        ensure!(self.initialized, GUI_NOT_INITIALIZED);
        
        self.events.push(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta,
            modifiers: self.modifiers,
        });

        Ok(())
    }
    
    pub fn send_touch(&mut self, id: egui::TouchId, phase: egui::TouchPhase, pos: egui::Pos2) -> Result<()> {
        ensure!(self.initialized, GUI_NOT_INITIALIZED);
        
        use egui::TouchPhase::{Start, Move, End};
        use egui::PointerButton::Primary;
        match phase {
            Start => {
                self.send_mouse_button(pos, Primary, true)?;
            },
            Move => {
                self.send_mouse_pos(pos)?;
            },
            End => {
                self.send_mouse_button(pos, Primary, false)?;
                self.events.push(egui::Event::PointerGone);
            },
            _ => (),
        };
        
        self.events.push(egui::Event::Touch {
            device_id: egui::TouchDeviceId(0),
            id,
            phase,
            pos,
            force: None,
        });
        
        Ok(())
    }
    
    pub fn send_key_press(&mut self, key: egui::Key, pressed: bool, repeat: bool) -> Result<()> {
        ensure!(self.initialized, GUI_NOT_INITIALIZED);

        self.events.push(egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed,
            repeat,
            modifiers: self.modifiers,
        });

        Ok(())
    }
    
    pub fn send_text_input(&mut self, text: &str) -> Result<()> {
        ensure!(self.initialized, GUI_NOT_INITIALIZED);

        self.events.push(egui::Event::Text(text.to_string()));

        Ok(())
    }
    
    pub fn send_modifiers(&mut self, modifiers: egui::Modifiers) -> Result<()> {
        ensure!(self.initialized, GUI_NOT_INITIALIZED);

        self.modifiers = modifiers;

        Ok(())
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
            run_fns: Vec::new(),
        }
    }
}