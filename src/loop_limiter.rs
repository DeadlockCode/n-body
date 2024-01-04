use std::time::{Instant, Duration};

pub struct LoopLimiter {
    last: Instant,
    min_loop_time: Duration,
}

impl LoopLimiter {
    #[allow(dead_code)]
    pub fn new(max_loops_per_second: f32) -> Self {
        let min_loop_time = Duration::from_secs_f64(1.0 / max_loops_per_second as f64);
        Self {
            last: Instant::now(),
            min_loop_time,
        }
    }
}

pub trait Checkable {
    fn check(&mut self);
}

impl Checkable for LoopLimiter {
    fn check(&mut self) {
        while self.last.elapsed() < self.min_loop_time {}
        self.last = Instant::now();
    }
}

impl<T: Checkable> Checkable for Option<T> {
    fn check(&mut self) {
        if let Some(inner) = self {
            inner.check();
        }
    }
}
