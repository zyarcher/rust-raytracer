use ray_tracer::color::Canvas;
use ray_tracer::color::Color;
use ray_tracer::tuple::Tuple;
use ray_tracer::matrix::Matrix;
use ray_tracer::matrix::TransformBuilder;
use std::f32::consts;

fn main() {
    let mut canvas = Canvas::new(100, 100);
    let clock_radius = 30.0;
    let n = 12;

    let middle_x = canvas.w as f32 / 2.0;
    let middle_y = canvas.h as f32 / 2.0;
    let middle = Tuple::new_point(middle_x, middle_y, 0.0);

    let east = &Matrix::translate(clock_radius, 0.0, 0.0) * middle;

    let pts: Vec<Tuple<f32>> = (0..n)
        .map(|i| (i as f32) * (2.0 * consts::PI) / (n as f32))
        .map(|angle| {
            let m = TransformBuilder::identity()
                .translate(-middle_x, -middle_y, 0.0)
                .rotation_z(angle)
                .translate(middle_x, middle_y, 0.0)
                .build();
            &m * east
        })
        .collect();

    // canvas.write_pixel(east.0 as usize, east.1 as usize, Color::new(255.0, 0.0, 0.0));

    for p in pts {
        let (x, y) = (p.0 as usize, p.1 as usize);
        canvas.write_pixel(x, y, Color::new(255.0, 0.0, 0.0));
    }

    println!("{}", canvas.write_ppm());
}
