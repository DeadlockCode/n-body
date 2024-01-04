use std::f64::consts::TAU;

use ultraviolet::DVec2 as Vec2;

use crate::body::Body;

fn rand_disc() -> Vec2 {
    let theta = fastrand::f64() * TAU;
    Vec2::new(theta.cos(), theta.sin()) * fastrand::f64()
}

fn rand_body() -> Body {
    let pos = rand_disc();
    let vel = rand_disc();

    Body::new(pos, vel, 1.0)
}

type Real = f64;

const DT: Real = 0.000001;
const MIN: Real = 0.0001;

pub struct Simulation {
    pub bodies: Vec<Body>,
}

impl Simulation {
    pub fn new(seed: u64) -> Self {
        fastrand::seed(seed);
        let mut bodies = Vec::new();

        let n = 3;
        for _ in 0..n {
            bodies.push(rand_body());
        }

        let vel = bodies.iter().map(|b| b.vel * b.mass).sum::<Vec2>() / n as Real;
        let pos = bodies.iter().map(|b| b.pos * b.mass).sum::<Vec2>() / n as Real;
        for b in &mut bodies {
            b.vel -= vel;
            b.pos -= pos;
        }
        let r = bodies.iter().map(|b| b.pos.mag()).max_by(Real::total_cmp).unwrap();
        for b in &mut bodies {
            b.pos /= r;
        }

        Self {
            bodies,
        }
    }

    pub fn update(&mut self) {
        for i in 0..self.bodies.len() {
            let p1 = self.bodies[i].pos;
            let m1 = self.bodies[i].mass;
            for j in (i+1)..self.bodies.len() {
                let p2 = self.bodies[j].pos;
                let m2 = self.bodies[j].mass;

                let r = p2 - p1;
                let mag_sq = r.x * r.x + r.y * r.y;
                let mag = mag_sq.sqrt();
                let tmp = r / (mag_sq.max(MIN) * mag);

                self.bodies[i].acc += m2 * tmp;
                self.bodies[j].acc -= m1 * tmp;
            }
        }

        for i in 0..self.bodies.len() {
            self.bodies[i].update(DT);
        }
    }
}
