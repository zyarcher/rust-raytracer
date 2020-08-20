use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::tuple::dot;
use crate::matrix::Matrix;
use crate::matrix::TransformBuilder;
use crate::material::Material;

#[derive(PartialEq, Debug)]
pub enum Shape {
    Sphere(Sphere),
}

#[derive(PartialEq, Debug)]
pub struct Object {
    shape: Shape,
    inv_transform: Matrix<f32>,
    pub material: Material
}

#[derive(PartialEq, Debug)]
pub struct Hitrecord<'a> {
    pub hit: f32,
    pub obj: &'a Object,
}

impl<'a> Hitrecord<'a> {
    pub fn new(hit: f32, obj: &'a Object) -> Self {
        Hitrecord { hit, obj }
    }

    pub fn new_vec(hits: Vec<f32>, obj: &'a Object) -> Vec<Self> {
        hits.iter().map(|&h| Self::new(h, obj)).collect()
    }
}

impl Object {
    pub fn new(shape: Shape) -> Self {
        Self { shape, inv_transform: Matrix::eye(4), material: Material::new() }
    }

    pub fn apply_transform(&mut self, transform: Matrix<f32>) {
        // TODO: do error handling
        self.inv_transform = transform.inverse().unwrap();
    }

    pub fn hit<'a>(&'a self, r: &Ray) -> Vec<Hitrecord<'a>> {
        let new_r = r.transform(&self.inv_transform);
        Hitrecord::new_vec(self.shape.hit(&new_r), &self)
    }

    pub fn normal_at(&self, pt: Tuple<f32>) -> Tuple<f32> {
        let obj_pt = &self.inv_transform * pt;
        let mut normal = &self.inv_transform.transpose() * self.shape.normal_at(obj_pt);
        normal.3 = 0.0; // set w to 0
        normal.normalize()
    }
}

pub trait Hittable {
    fn hit<'a>(&self, r: &Ray) -> Vec<f32>;

    fn normal_at(&self, pt: Tuple<f32>) -> Tuple<f32>;
}

#[derive(PartialEq, Debug)]
pub struct Sphere;

impl Sphere {
    pub fn new() -> Shape {
        Shape::Sphere(Sphere)
    }
}

impl Hittable for Shape {
    fn hit<'a>(&self, r: &Ray) -> Vec<f32> {
        match self {
            Shape::Sphere(sphere) => sphere.hit(&r),
        }
    }

    fn normal_at(&self, pt: Tuple<f32>) -> Tuple<f32> {
        match self {
            Shape::Sphere(sphere) => sphere.normal_at(pt),
        }
    }
}

impl Hittable for Sphere {
    fn hit<'a>(&self, r: &Ray) -> Vec<f32> {
        let sphere_to_ray = r.origin - Tuple::new_point(0.0, 0.0, 0.0);

        let a = dot(r.dir, r.dir);
        let b = 2.0 * dot(r.dir, sphere_to_ray);
        let c = dot(sphere_to_ray, sphere_to_ray) - 1.0;

        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 { 
            vec![]
        } else {
            let t1 = (-b - disc.sqrt()) / (2.0 * a);
            let t2 = (-b + disc.sqrt()) / (2.0 * a);
            vec![t1, t2]
        }
    }

    fn normal_at(&self, pt: Tuple<f32>) -> Tuple<f32> {
        (pt - Tuple::new_point(0.0, 0.0, 0.0)).normalize()
    }
}

// takes a vector of Hitrecord and returns the closest valid (nonnegative t) hit
pub fn find_hit<'a>(hits: Vec<Hitrecord<'a>>) -> Option<Hitrecord<'a>> {
    hits.into_iter()
        .filter(|h| h.hit >= 0.0)
        .min_by(|h1, h2| h1.hit.partial_cmp(&h2.hit).unwrap_or(std::cmp::Ordering::Equal))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere() {
        let s = Object::new(Sphere::new());
        let r = Ray::new(Tuple::new_point(0.0, 0.0, -5.0), Tuple::new_vector(0.0, 0.0, 1.0));
        let ht = s.hit(&r);
        assert!(ht.len() == 2);
        assert!(ht[0].hit == 4.0);
        assert!(ht[1].hit == 6.0);

        assert!(*ht[0].obj == s);
        assert!(*ht[1].obj == s);
    }

    #[test]
    fn test_hit() {
        let s = Object::new(Sphere::new());
        let hits = Hitrecord::new_vec(
            vec![5.0, 7.0, -3.0, 2.0],
            &s
        );

        assert!(find_hit(hits).unwrap() == Hitrecord::new(2.0, &s));
    }

    #[test]
    fn test_intersect() {
        let r = Ray::new(Tuple::new_point(0.0, 0.0, -5.0), Tuple::new_vector(0.0, 0.0, 1.0));
        let mut s = Object::new(Sphere::new());
        s.apply_transform(Matrix::scale(2.0, 2.0, 2.0));
        let xs = s.hit(&r);
        assert!(xs.len() == 2);
        println!("{:?}", xs.iter().map(|x| x.hit).collect::<Vec<f32>>());
        assert!(xs[0].hit == 3.0);
        assert!(xs[1].hit == 7.0);
    }

    #[test]
    fn test_normal() {
        let s = Sphere::new();
        assert!(s.normal_at(Tuple::new_point(1.0, 0.0, 0.0)) == Tuple::new_vector(1.0, 0.0, 0.0));

        let f = 3_f32.sqrt()/3.0;
        assert!(s.normal_at(Tuple::new_point(f, f, f)).eq_real(&Tuple::new_vector(f, f, f)));

        let mut sphere_obj = Object::new(s);
        sphere_obj.apply_transform(
            TransformBuilder::identity()
                .translate(0.0, 1.0, 0.0)
                .build()
        );
        let n = sphere_obj.normal_at(Tuple::new_point(0.0, 1.70711, -0.70711));
        println!("{:?}", n);
        assert!(n.eq_real(&Tuple::new_vector(0.0, 0.7071068, -0.70710677,)));// my results are off by a bit
    }
}
