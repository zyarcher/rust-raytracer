use ray_tracer::object::*;
use ray_tracer::tuple::Tuple;
use ray_tracer::ray::Ray;
use ray_tracer::color::Canvas;
use ray_tracer::color::Color;
use ray_tracer::matrix::Matrix;
use ray_tracer::matrix::TransformBuilder;
use ray_tracer::light::PointLight;
use ray_tracer::material::lightning;

use num_traits::Num;
use num_traits::real::Real;

fn map<T: Num + Real>(x: T, x1: T, x2: T, y1: T, y2: T) -> T {
    (y2 - y1) / (x2 - x1) * (x - x1) + y1
}

fn main() {
    let ray_origin = Tuple::new_point(0.0, 0.0, -5.0);
    let wall_z = 10.0;

    let w = 400;
    let h = 400;

    let scene_w = 5.0;
    let scene_h = scene_w * (h as f32) / (w as f32);

    // let sphere = Sphere { r: 1.0, pos: Tuple::new_point(0.0, 0.0, 3.0) };
    let mut sphere = Object::new(Sphere::new());
    sphere.material.color = Color::new(212.0/255.0, 129.0/255.0, 64.0/255.0);
    sphere.material.diffuse = 0.999;
    sphere.material.ambient = 0.0;

    let light_pos = Tuple::new_point(-10.0, 5.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_color, light_pos);

    sphere.apply_transform(
        TransformBuilder::identity()
            .translate(0.0, 0.0, 1.0)
            .scale(1.0, 1.3, 1.2)
            .build());

    let ppm = Canvas::write_ppm_fn_long(w, h, |x, y| {
        // canvas:
        // (-w, -h) ... (w, -h)
        // .              .
        // .    (0, 0)    .
        // .              .
        // (-w, h) ...  (-w, h)
        let ray_x = map(x as f32, 0.0, w as f32, -(scene_w as f32), scene_w as f32);
        let ray_y = map(y as f32, 0.0, h as f32, scene_h as f32, -(scene_h as f32));
        let ray_dir = Tuple::new_point(ray_x, ray_y, wall_z) - ray_origin;
        let r = Ray::new(ray_origin, ray_dir);
        
        if let Some(ht) = find_hit(sphere.hit(&r)) {
            let point = r.pos(ht.hit);
            let normal = ht.obj.normal_at(point);
            let eye = -r.dir;

            lightning(&sphere.material, &light, &point, &eye, &normal, false)
        } else {
            Color(0.0, 0.0, 0.0)
        }
    });
    println!("{}", ppm);
}
