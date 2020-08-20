use crate::color::Color;
use crate::tuple::Tuple;
use crate::tuple::dot;
use crate::material::Material;
use crate::material::lightning;

pub struct PointLight<T> {
    pub(crate) intensity: Color<T>,
    pub(crate) pos: Tuple<T>,
}

impl<T> PointLight<T> {
    pub fn new(intensity: Color<T>, pos: Tuple<T>) -> Self {
        Self { intensity, pos }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lightning() {
        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Tuple::new_point(0.0, 0.0, -10.0));
        let pos = Tuple::new_point(0.0, 0.0, 0.0);
        let res = lightning(&Material::new(), &light, &pos, &eyev, &normalv, false);
        assert!(res == Color::new(1.9, 1.9, 1.9));
    }
}
