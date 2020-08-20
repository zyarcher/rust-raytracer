use ray_tracer::object::*;
use ray_tracer::tuple::Tuple;
use ray_tracer::ray::Ray;
use ray_tracer::color::Canvas;
use ray_tracer::color::Color;
use ray_tracer::matrix::Matrix;
use ray_tracer::matrix::TransformBuilder;

use num_traits::Num;
use num_traits::real::Real;

fn map<T: Num + Real>(x: T, x1: T, x2: T, y1: T, y2: T) -> T {
    (y2 - y1) / (x2 - x1) * (x - x1) + y1
}

fn main() {
    let ray_origin = Tuple::new_point(0.0, 0.0, -5.0);
    let wall_z = 10.0;

    let w = 200;
    let h = 100;

    let scene_w = 5.0;
    let scene_h = scene_w * (h as f32) / (w as f32);

    // let sphere = Sphere { r: 1.0, pos: Tuple::new_point(0.0, 0.0, 3.0) };
    let mut sphere = Object::new(Sphere::new());
    sphere.apply_transform(
        TransformBuilder::identity()
            .scale(0.5, 1.0, 1.0)
            .build());

    let ppm = Canvas::write_ppm_fn(w, h, |x, y| {
        // canvas:
        // (-w, -h) ... (w, -h)
        // .              .
        // .              .
        // (-w, h) ...  (-w, h)
        let ray_x = map(x as f32, 0.0, w as f32, -(scene_w as f32), scene_w as f32);
        let ray_y = map(y as f32, 0.0, h as f32, -(scene_h as f32), scene_h as f32);
        let ray_dir = Tuple::new_point(ray_x, ray_y, wall_z) - ray_origin;
        let r = Ray::new(ray_origin, ray_dir);
        
        if let Some(_) = find_hit(sphere.hit(&r)) {
            Color(1.0, 0.0, 0.0)
        } else {
            Color(0.0, 0.0, 0.0)
        }
    });
    println!("{}", ppm);
}
