use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::world::World;
use crate::color::Color;

pub struct Camera {
    hsize: f32,
    vsize: f32,
    fov: f32,
    pub inv_transform: Matrix<f32>,

    half_width: f32,
    half_height: f32,

    pixel_size: f32,
}

impl Camera {
    pub fn new(hsize: f32, vsize: f32, fov: f32) -> Self {
        let half_view = (fov / 2.0).tan();
        let aspect = hsize / vsize;

        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = (half_width * 2.0) / hsize;

        Self { hsize, vsize, fov, inv_transform: Matrix::eye(4), half_width, half_height, pixel_size }
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f32 + 0.5) * self.pixel_size;
        let yoffset = (py as f32 + 0.5) * self.pixel_size;

        let worldx = self.half_width - xoffset;
        let worldy = self.half_height - yoffset;

        let pixel = &self.inv_transform * Tuple::new_point(worldx, worldy, -1.0);
        let origin = &self.inv_transform * Tuple::new_point(0.0, 0.0, 0.0);

        Ray::new(origin, pixel - origin)
    }

    pub fn render_pixel(&self, world: &World, px: usize, py: usize) -> Color<f32>{
        let r = self.ray_for_pixel(px, py);
        world.color_at(&r)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ray() {
        let w = World::new_default();
        let mut c = Camera::new(11.0, 11.0, std::f32::consts::PI / 2.0);

        let from = Tuple::new_point(0.0, 0.0, -5.0);
        let to = Tuple::new_point(0.0, 0.0, 0.0);
        let up = Tuple::new_vector(0.0, 1.0, 0.0);

        c.inv_transform = Matrix::view_transform(from, to, up).inverse().unwrap();
        println!("{:?}", c.render_pixel(&w, 5, 5));
        assert!(c.render_pixel(&w, 5, 5) == Color::new(0.38066125, 0.4758265, 0.28549594));
    }
}
