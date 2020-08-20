use crate::color::Color;
use crate::light::PointLight;
use crate::tuple::Tuple;
use crate::tuple::dot;

#[derive(PartialEq, Debug, Clone)]
pub struct Material {
    pub color: Color<f32>,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Material {
    pub fn new() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

pub fn lightning(material: &Material, light: &PointLight<f32>, pos: &Tuple<f32>, eyev: &Tuple<f32>, normalv: &Tuple<f32>, in_shadow: bool) -> Color<f32> {
    let black = Color::new(0.0, 0.0, 0.0);
    let effective_color = material.color * light.intensity;
    let lightv = (light.pos - *pos).normalize();

    let ambient = effective_color * material.ambient;

    if in_shadow { return ambient }

    let light_dot_normal = dot(lightv, *normalv);
    let (diffuse, specular) = if light_dot_normal < 0.0 {
        (black, black)
    } else {
        let diffuse = effective_color * material.diffuse * light_dot_normal;

        let reflectv = (-lightv).reflect(normalv);
        let reflect_dot_eye = dot(reflectv, *eyev);
        let specular = if reflect_dot_eye < 0.0 {
            black
        } else {
            let f = reflect_dot_eye.powf(material.shininess);
            light.intensity * material.specular * f
        };

        (diffuse, specular)
    };

    ambient + diffuse + specular
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lightning() {
        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Tuple::new_point(0.0, 0.0, -10.0));
        let in_shadow = true;
        
        let result = lightning(&Material::new(), &light, &Tuple::new_point(0.0, 0.0, 0.0), &eyev, &normalv, in_shadow);
        assert!(result == Color::new(0.1, 0.1, 0.1));
    }
}
