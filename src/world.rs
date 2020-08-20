use crate::object::Hittable;
use crate::object::find_hit;
use crate::object::Hitrecord;
use crate::object::Object;
use crate::light::PointLight;
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::tuple::dot;
use crate::color::Color;
use crate::object::Sphere;
use crate::matrix::Matrix;
use crate::material::lightning;

pub struct World {
    lights: Vec<PointLight<f32>>,
    objects: Vec<Object>,
}

pub struct Hitinfo<'a> {
    pub hit: f32,
    pub obj: &'a Object,
    pub point: Tuple<f32>,
    pub over_point: Tuple<f32>,
    pub eyev: Tuple<f32>,
    pub normalv: Tuple<f32>,
    pub inside: bool,
}

impl World {
    pub fn new() -> Self {
        Self {
            lights: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn new_default() -> Self {
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Tuple::new_point(-10.0, 10.0, -10.0));

        let mut s1 = Object::new(Sphere::new());
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Object::new(Sphere::new());
        s2.apply_transform(Matrix::scale(0.5, 0.5, 0.5));

        Self {
            lights: vec![light],
            objects: vec![s1, s2],
        }
    }

    pub fn add_light(&mut self, l: PointLight<f32>) {
        self.lights.push(l);
    }

    pub fn add_object(&mut self, obj: Object) {
        self.objects.push(obj);
    }

    pub fn intersect_world<'a>(&'a self, ray: &Ray) -> Vec<Hitrecord<'a>> {
        let mut v = self.objects.iter()
            .flat_map(|obj| {
                obj.hit(ray)
            })
            .collect::<Vec<Hitrecord<'a>>>();
        v.sort_by(|obj1, obj2| obj1.hit.partial_cmp(&obj2.hit).unwrap_or(std::cmp::Ordering::Equal));
        v
    }

    pub fn is_shadowed(&self, point: &Tuple<f32>, light: &PointLight<f32>) -> bool {
        let v = light.pos - *point;
        // must do square root because of t
        let dist = v.magnitude();
        let dir = v.normalize();

        let r = Ray::new(*point, dir);
        let i = self.intersect_world(&r);
        if let Some(h) = find_hit(i) {
            h.hit < dist
        } else { false }
    }

    pub fn prepare_computations<'a>(hr: &Hitrecord<'a>, ray: &Ray) -> Hitinfo<'a> {
        let pt = ray.pos(hr.hit);

        let normalv = hr.obj.normal_at(pt);
        let eyev = -ray.dir;
        let inside = dot(normalv, eyev) < 0.0;
        Hitinfo {
            hit: hr.hit,
            obj: hr.obj,
            point: pt,
            over_point: pt + normalv * (0.01),
            eyev,
            normalv: if inside { -normalv } else { normalv },
            inside,
        }
    }

    pub fn shade_hit<'a>(&self, comps: &Hitinfo<'a>) -> Color<f32> {
        self.lights.iter()
            .map(|l| {
                let in_shadow = self.is_shadowed(&comps.over_point, &l);
                lightning(&comps.obj.material, &l, &comps.over_point, &comps.eyev, &comps.normalv, in_shadow)
            })
            .fold(Color::new(0.0, 0.0, 0.0), |a, b| a + b)
    }

    pub fn color_at(&self, ray: &Ray) -> Color<f32> {
        let hrs = self.intersect_world(ray); // get closest one
        if hrs.len() == 0 {
            Color::new(0.0, 0.0, 0.0)
        } else {
            let hi = Self::prepare_computations(&hrs[0], ray);
            self.shade_hit(&hi)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect_world() {
        let w = World::new_default();
        let r = Ray::new(Tuple::new_point(0.0, 0.0, -5.0), Tuple::new_vector(0.0, 0.0, 1.0));
        let xs = w.intersect_world(&r);

        let hits: Vec<f32> = xs.iter().map(|x| x.hit).collect();
        println!("{:?}", hits);

        assert!(xs.len() == 4);
        assert!(xs[0].hit == 4.0);
        assert!(xs[1].hit == 4.5);
        assert!(xs[2].hit == 5.5);
        assert!(xs[3].hit == 6.0);
    }

    #[test]
    fn test_prepare_computations() {
        let r = Ray::new(Tuple::new_point(0.0, 0.0, -5.0), Tuple::new_vector(0.0, 0.0, 1.0));
        let s = Object::new(Sphere::new());
        let i = Hitrecord { hit: 4.0, obj: &s };
        let comps = World::prepare_computations(&i, &r);
        assert!(comps.inside == false);

        let r = Ray::new(Tuple::new_point(0.0, 0.0, 0.0), Tuple::new_vector(0.0, 0.0, 1.0));
        let i = Hitrecord { hit: 1.0, obj: &s };
        let comps = World::prepare_computations(&i, &r);

        assert!(comps.point == Tuple::new_point(0.0, 0.0, 1.0));
        assert!(comps.eyev == Tuple::new_vector(0.0, 0.0, -1.0));
        assert!(comps.inside == true);
        assert!(comps.normalv == Tuple::new_vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_color_at() {
        let mut w = World::new_default();
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;
        let r = Ray::new(Tuple::new_point(0.0, 0.0, 0.75), Tuple::new_vector(0.0, 0.0, -1.0));
        let c = w.color_at(&r);
        assert!(c == w.objects[0].material.color);
    }

    #[test]
    fn test_shadow() {
        let w = World::new_default();
        let p = Tuple::new_point(0.0, 10.0, 0.0);
        assert!(w.is_shadowed(&p) == false);

        let p = Tuple::new_point(10.0, -10.0, 10.0);
        assert!(w.is_shadowed(&p) == true);
    }
}
