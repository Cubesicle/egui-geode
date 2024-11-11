use std::{sync::Arc, time::SystemTime};
use parking_lot::Mutex;

pub static GLOBAL_GUI: Mutex<Option<GuiBackend>> = Mutex::new(None);

pub struct GuiBackend {
    egui_ctx: egui::Context,
    painter: egui_glow::Painter,
    modifiers: egui::Modifiers,
    events: Vec<egui::Event>,
    run_fns: Vec<fn(&egui::Context)>,
}

impl GuiBackend {
    pub fn from(gl_ctx: Arc<egui_glow::glow::Context>) -> Result<Self, egui_glow::PainterError> {
        Ok(Self {
            egui_ctx: Default::default(),
            painter: egui_glow::Painter::new(gl_ctx, "", None, true)?,
            modifiers: Default::default(),
            events: Default::default(),
            run_fns: Default::default(),
        })
    }

    pub fn get_context(&self) -> &egui::Context {
        &self.egui_ctx
    }
    
    pub fn add_run_fn(&mut self, func: fn(&egui::Context)) {
        self.run_fns.push(func);
    }

    // Code taken from https://github.com/spinningtoilet0/egui_glow_internal
    pub fn paint(&mut self, frame_size: (f32, f32)) {
        let egui::FullOutput {
            platform_output: _,
            mut textures_delta,
            shapes,
            pixels_per_point,
            viewport_output: _,
        } = self.egui_ctx.run(egui::RawInput {
            screen_rect: Some(egui::Rect {
                min: egui::pos2(0.0, 0.0),
                max: egui::pos2(frame_size.0, frame_size.1),
            }),
            time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map(|d| d.as_secs_f64()).ok(),
            modifiers: self.modifiers,
            events: std::mem::take(&mut self.events),
            ..Default::default()
        }, |ctx| {
            for f in &self.run_fns { f(ctx); }
        });
        
        for (id, image_delta) in textures_delta.set {
            self.painter.set_texture(id, &image_delta);
        }
    
        if !shapes.is_empty() {
            let clipped_primitives = self.egui_ctx.tessellate(shapes, pixels_per_point);
    
            self.painter.paint_primitives(
                [frame_size.0 as u32, frame_size.1 as u32],
                self.egui_ctx.pixels_per_point(),
                &clipped_primitives,
            );
        }
    
        for id in textures_delta.free.drain(..) {
            self.painter.free_texture(id);
        }
    }
    
    pub fn wants_pointer_input(&self) -> bool {
        self.egui_ctx.wants_pointer_input()
    }

    pub fn wants_keyboard_input(&self) -> bool {
        self.egui_ctx.wants_keyboard_input()
    }
    
    pub fn is_pos_over_area(&self, pos: egui::Pos2) -> bool {
        if let Some(layer) = self.egui_ctx.layer_id_at(pos) {
            if layer.order == egui::Order::Background {
                !self.egui_ctx.viewport(|v| v.this_pass.unused_rect.contains(pos))
            } else {
                true
            }
        } else {
            false
        }
    }
    
    pub fn send_mouse_pos(&mut self, pos: egui::Pos2) {
        self.events.push(egui::Event::PointerMoved(pos));
    }
    
    pub fn send_mouse_button(&mut self, pos: egui::Pos2, button: egui::PointerButton, pressed: bool) {
        self.events.push(egui::Event::PointerButton {
            pos,
            button,
            pressed,
            modifiers: self.modifiers,
        });
    }
    
    pub fn send_scroll_event(&mut self, delta: egui::Vec2) {
        self.events.push(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta,
            modifiers: self.modifiers,
        });
    }
    
    pub fn send_touch(&mut self, id: egui::TouchId, phase: egui::TouchPhase, pos: egui::Pos2) {
        use egui::TouchPhase::{Start, Move, End};
        use egui::PointerButton::Primary;

        match phase {
            Start => {
                self.send_mouse_button(pos, Primary, true);
            },
            Move => {
                self.send_mouse_pos(pos);
            },
            End => {
                self.send_mouse_button(pos, Primary, false);
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
    }
    
    pub fn send_key_press(&mut self, key: egui::Key, pressed: bool, repeat: bool) {
        self.events.push(egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed,
            repeat,
            modifiers: self.modifiers,
        });
    }
    
    pub fn send_text_input(&mut self, text: &str) {
        self.events.push(egui::Event::Text(text.to_string()));
    }
    
    pub fn send_modifiers(&mut self, modifiers: egui::Modifiers) {
        self.modifiers = modifiers;
    }
}