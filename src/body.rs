use ultraviolet::DVec2 as Vec2;

#[derive(Clone)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f64,
}

impl Body {
    pub fn new(pos: Vec2, vel: Vec2, mass: f64) -> Self {
        Self { pos, vel, acc: Vec2::zero(), mass }
    }

    pub fn update(&mut self, dt: f64) {
        self.pos += self.vel * dt;
        self.vel += self.acc * dt;
        self.acc = Vec2::zero();
    }
}
