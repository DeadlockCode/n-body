use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use quarkstrom::egui::{self, mutex::Mutex};
use quarkstrom::{winit_input_helper::WinitInputHelper, winit::event::VirtualKeyCode};

use once_cell::sync::Lazy;
use ultraviolet::Vec2;

use crate::START_SEED;
use crate::body::Body;

pub static PAUSED: Lazy<AtomicBool> = Lazy::new(|| false.into());
pub static TICK: Lazy<AtomicI32> = Lazy::new(|| 0.into());
pub static SEED: Lazy<Mutex<Option<u64>>> = Lazy::new(|| Mutex::new(None));

pub static BODIES: Lazy<Mutex<Option<Vec<Body>>>> = Lazy::new(|| Mutex::new(None));

pub struct Renderer {
    pos: Vec2,
    scale: f32,
    seed: u64,

    info_window_open: bool,
}

impl quarkstrom::Renderer for Renderer {
    fn new() -> Self {
        Self {
            pos: Vec2::zero(),
            scale: 2.0,
            seed: START_SEED,

            info_window_open: true,
        }
    }

    fn input(&mut self, input: &WinitInputHelper, width: u16, height: u16) {
        self.info_window_open ^= input.key_pressed(VirtualKeyCode::T);

        if input.key_pressed(VirtualKeyCode::Space) {
            let val = PAUSED.load(Ordering::Relaxed);
            PAUSED.store(!val, Ordering::Relaxed)
        }

        if let Some((mx, my)) = input.mouse() {
            // Scroll steps to double/halve the scale
            let steps = 5.0;

            // Modify input
            let zoom = (-input.scroll_diff() / steps).exp2();

            // Screen space -> view space
            let target =
                Vec2::new(mx * 2.0 - width as f32, height as f32 - my * 2.0) / height as f32;

            // Move view position based on target
            self.pos += target * self.scale * (1.0 - zoom);

            // Zoom
            self.scale *= zoom;
        }

        // Grab
        if input.mouse_held(2) {
            let (mdx, mdy) = input.mouse_diff();
            self.pos.x -= mdx / height as f32 * self.scale * 2.0;
            self.pos.y += mdy / height as f32 * self.scale * 2.0;
        }
    }

    fn render(&mut self, ctx: &mut quarkstrom::RenderContext) {
        ctx.set_view_pos(self.pos);
        ctx.set_view_scale(self.scale);

        if let Some(bodies) = BODIES.lock().clone() {
            ctx.clear_circles();
            ctx.clear_lines();

            for i in 0..bodies.len() {
                let p = bodies[i].pos;
                ctx.draw_circle(Vec2::new(p.x as f32, p.y as f32), 0.05, 0xffffff);
            }
        };
    }

    fn gui(&mut self, ctx: &quarkstrom::egui::Context) {
        egui::Window::new("")
            .open(&mut self.info_window_open)
            .show(ctx, |ui| {
                ui.label(format!("Simulation Tick: {}", TICK.load(Ordering::Relaxed)));
                ui.horizontal(|ui| {
                    ui.label("Seed:");
                    if ui.add(egui::DragValue::new(&mut self.seed)).changed() {
                        TICK.store(0, Ordering::Relaxed);
                        SEED.lock().replace(self.seed);
                    }
                    if ui.button("Randomize").clicked() {
                        self.seed = fastrand::u64(..);
                        TICK.store(0, Ordering::Relaxed);
                        SEED.lock().replace(self.seed);
                    }
                })
            });
    }
}
