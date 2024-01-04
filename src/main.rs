use std::{
    sync::atomic::Ordering,
    time::Duration, thread,
};

use quarkstrom;

mod body;

mod simulation;
use simulation::Simulation;

mod loop_limiter;
use loop_limiter::{LoopLimiter, Checkable};

mod renderer;
use renderer::{Renderer, BODIES, TICK, PAUSED, SEED};

pub const START_SEED: u64 = 3; 

fn main() {
    let config = quarkstrom::Config {
        window_mode: quarkstrom::WindowMode::Windowed(900, 900),
    };
    quarkstrom::run::<Renderer>(config);

    let mut simulation = Simulation::new(START_SEED);

    let mut loop_limiter: Option<LoopLimiter> = Some(LoopLimiter::new(1000000.0));
    loop {
        simulation.update();
        *BODIES.lock() = Some(simulation.bodies.clone());
        TICK.fetch_add(1, Ordering::Relaxed);

        if let Some(seed) = SEED.lock().take() {
            simulation = Simulation::new(seed);
        }

        // Cap tps
        loop_limiter.check();

        while PAUSED.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(16));
        }
    }
}
