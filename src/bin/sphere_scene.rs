use ray_tracer::world::World;
use ray_tracer::object::{Object, Sphere};
use ray_tracer::matrix::Matrix;
use ray_tracer::matrix::TransformBuilder;
use ray_tracer::color::Color;
use ray_tracer::color::Canvas;
use ray_tracer::light::PointLight;
use ray_tracer::camera::Camera;
use ray_tracer::tuple::Tuple;

const PI: f32 = std::f32::consts::PI;

fn main() {
    let floor = {
        let mut obj = Object::new(Sphere::new());
        obj.apply_transform(Matrix::scale(10.0, 0.01, 10.0));
        obj.material.color = Color::new(1.0, 0.9, 0.9);
        obj.material.specular = 0.0;
        obj
    };

    let left_wall = {
        let mut obj = Object::new(Sphere::new());
        obj.apply_transform(
            TransformBuilder::identity()
                .scale(10.0, 0.01, 10.0)
                .rotation_x(PI / 2.0)
                .rotation_y(-PI / 4.0)
                .translate(0.0, 0.0, 5.0)
                .build()
        );
        obj.material = floor.material.clone();
        obj
    };

    let right_wall = {
        let mut obj = Object::new(Sphere::new());
        obj.apply_transform(
            TransformBuilder::identity()
                .scale(10.0, 0.01, 10.0)
                .rotation_x(PI / 2.0)
                .rotation_y(PI / 4.0)
                .translate(0.0, 0.0, 5.0)
                .build()
        );
        obj.material = floor.material.clone();
        obj
    };

    let middle = {
        let mut obj = Object::new(Sphere::new());
        obj.apply_transform(Matrix::translate(-0.5, 1.0, 0.5));
        obj
    };

    let right = {
        let mut obj = Object::new(Sphere::new());
        obj.apply_transform(&Matrix::translate(1.5, 0.0, -0.5) * &Matrix::scale(0.5, 0.5, 0.5));
        obj.material.color = Color::new(0.5, 1.0, 0.1);
        obj.material.diffuse = 0.7;
        obj.material.specular = 0.3;
        obj
    };
    
    let left = {
        let mut obj = Object::new(Sphere::new());
        obj.apply_transform(&Matrix::translate(-1.5, 0.33, -0.75) * &Matrix::scale(0.33, 0.33, 0.33));
        obj.material.color = Color::new(1.0, 0.8, 0.1);
        obj.material.diffuse = 0.7;
        obj.material.specular = 0.3;
        obj
    };

    let world = {
        let mut w = World::new();
        w.add_light(PointLight::new(Color::new(1.0, 1.0, 1.0), Tuple::new_point(-10.0, 10.0, -10.0)));
        w.add_light(PointLight::new(Color::new(0.4, 0.7, 0.2), Tuple::new_point(-10.0, 3.0, -10.0)));

        w.add_object(floor);
        w.add_object(left_wall);
        w.add_object(right_wall);
        w.add_object(middle);
        w.add_object(right);
        w.add_object(left);

        w
    };

    let width = 2000.0;
    let height = 1000.0;

    let camera = {
        let mut c = Camera::new(width, height, std::f32::consts::PI / 3.0);
        c.inv_transform = Matrix::view_transform(
            Tuple::new_point(0.0, 1.5, -5.0),
            Tuple::new_point(0.0, 1.0, 0.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        ).inverse().unwrap();
        c
    };

    println!("{}", Canvas::write_ppm_fn_long(width as usize, height as usize, |px, py| camera.render_pixel(&world, px, py)));
}
