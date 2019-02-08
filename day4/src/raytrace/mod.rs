pub mod camera;
pub mod material;
pub mod ray;
pub mod vec;

extern crate rand;
use rand::Rng;

use self::material::Material;

pub struct HitRecord {
    pub t : f32,
    pub p : vec::Vec3,
    pub normal : vec::Vec3,
    pub material : Box<Material>,
}

pub trait Hitable {
    fn hit(&self, r : ray::Ray, t_min : f32, t_max : f32) -> Option<HitRecord>;
}

pub struct Sphere {
    center : vec::Vec3,
    radius : f32,
    material : Box<Material>,
}

impl Sphere {
    pub fn new(center : vec::Vec3, radius : f32, material : Box<Material>) -> Self {
        Sphere { center, radius, material }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r : ray::Ray, t_min : f32, t_max : f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = vec::dot(r.direction(), r.direction());
        let b = vec::dot(oc, r.direction());
        let c = vec::dot(oc, oc) - self.radius*self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let temp = (-b - f32::sqrt(b*b-a*c))/a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord{t, p, normal, material: self.material.clone()});
            }
            let temp = (-b + f32::sqrt(b*b-a*c))/a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord{t, p, normal, material: self.material.clone()});
            }
        }
        None
    }
}

pub struct HitableList {
    list : Vec<Box<Hitable>>,
}

impl Hitable for HitableList {
    fn hit(&self, r: ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut result : Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for a in &self.list {
            let temp_result = a.hit(r, t_min, closest_so_far);
            match temp_result {
                Some(rec) => {
                    closest_so_far = rec.t;
                    result = Some(rec);
                }
                None => {}
            }
        }
        result
    }
}

impl HitableList {
    pub fn new(capacity: usize) -> HitableList {
        HitableList{list: Vec::with_capacity(capacity)}
    }

    pub fn add(&mut self, h : Box<Hitable>) {
        self.list.push(h);
    }
}

pub fn drand48() -> f32 {
    let rand_float : f32 = rand::thread_rng().gen();
    rand_float
}
