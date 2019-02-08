use super::ray;
use super::vec;
use super::vec::Vec3;
use super::drand48;
use std::f32::consts::PI;

#[allow(dead_code)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,

    u : Vec3,
    v : Vec3,
    w : Vec3,
    lens_radius : f32,
}

impl Camera {
    pub fn new(lookfrom : Vec3, lookat : Vec3, vup : Vec3, vfov : f32,
               aspect : f32, aperture : f32, focus_dist : f32) -> Camera {

        let lens_radius = aperture / 2.0;
        let theta = vfov * PI / 180.0;
        let half_height = f32::tan(theta/2.0);
        let half_width = aspect * half_height;

        let origin = lookfrom;
        let w = vec::unit_vector(lookfrom - lookat);
        let u = vec::unit_vector(vec::cross(vup, w));
        let v = vec::cross(w, u);

        let lower_left_corner = origin - half_width*focus_dist*u - half_height*focus_dist*v - focus_dist*w;
        let horizontal = 2.0*half_width*focus_dist*u;
        let vertical = 2.0*half_height*focus_dist*v;

        Camera{
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u, v, w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> ray::Ray {
        let rd = self.lens_radius*random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        ray::Ray::new(
            self.origin + offset,
            self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin - offset
        )
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut p : Vec3;

    loop {
        p = 2.0*Vec3::new(drand48(),drand48(),0.0) -
            Vec3::new(1.0, 1.0,0.0);

        if vec::dot(p,p) < 1.0 {
            return p;
        }
    }
}

