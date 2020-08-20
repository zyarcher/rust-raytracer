use ray_tracer::tuple::Tuple;
use ray_tracer::color::Canvas;
use ray_tracer::color::Color;

#[derive(Debug)]
struct Projectile {
    pos: Tuple<f32>,
    vel: Tuple<f32>,
}

#[derive(Debug)]
struct Env {
    gravity: Tuple<f32>,
    wind: Tuple<f32>,
}

impl Env {
    fn new(gravity: Tuple<f32>, wind: Tuple<f32>) -> Self {
        Self { gravity, wind }
    }
}

impl Projectile {
    fn new(pos: Tuple<f32>, vel: Tuple<f32>) -> Self {
        Self { pos, vel }
    }

    fn tick(&mut self, env: &Env) {
        self.pos = self.pos + self.vel;
        self.vel = self.vel + env.wind + env.gravity;
    }
}

pub fn main() {
    let mut canvas = Canvas::new(100, 100);
    let mut p = Projectile::new(Tuple::new_point(0.0, 1.0, 0.0), Tuple::new_vector(1.0, 1.0, 0.0));
    let env = Env::new(Tuple::new_vector(0.0, -0.1, 0.0), Tuple::new_vector(-0.01, 0.0, 0.0));
    let mut ticks = 0;
    while p.pos.1 >= 0.0 {
        p.tick(&env);
        canvas.write_pixel(p.pos.0 as usize, p.pos.1 as usize, Color(1.0, 0.0, 0.0));
        ticks += 1;
        // println!("{:?}", p.pos);
    }
    //println!("{:?} ticks: {}", p, ticks);
    println!("{}", canvas.write_ppm());
}
