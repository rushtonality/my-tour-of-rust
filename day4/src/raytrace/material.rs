use super::ray;
use super::vec;
use super::vec::Vec3;
use super::drand48;
use super::HitRecord;

pub trait Material {
    fn scatter(&self, r_in : &ray::Ray, rec : &HitRecord,
               attenuation : &mut vec::Vec3, scattered : &mut ray::Ray) -> bool;
    fn clone(&self) -> Box<Material>;
}

#[derive(Debug)]
pub struct Lambertian {
    albedo : vec::Vec3,
}
impl Lambertian {
    pub fn new(a : vec::Vec3) -> Lambertian {
        Lambertian{
            albedo: a
        }
    }
}
impl Material for Lambertian {
    fn scatter(&self, _r_in : &ray::Ray, rec : &HitRecord,
               attenuation : &mut vec::Vec3, scattered : &mut ray::Ray) -> bool {

        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = ray::Ray::new(rec.p, target-rec.p);
        *attenuation = self.albedo;
        true
    }
    fn clone(&self) -> Box<Material> {
        Box::new(Lambertian::new(self.albedo))
    }
}


pub struct Metal {
    albedo : vec::Vec3,
    fuzz : f32,
}
impl Metal {
    pub fn new(a : vec::Vec3, f : f32 ) -> Metal {
        Metal{
            albedo: a,
            fuzz : f32::min(f, 1.0)
        }
    }
}
impl Material for Metal {
    fn scatter(&self, r_in : &ray::Ray, rec : &HitRecord,
               attenuation : &mut vec::Vec3, scattered : &mut ray::Ray) -> bool {
        let reflected = reflect(vec::unit_vector(r_in.direction()), rec.normal);
        *scattered = ray::Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());
        *attenuation = self.albedo;
        vec::dot(scattered.direction(), rec.normal) > 0.0
    }
    fn clone(&self) -> Box<Material> {
        Box::new(Metal::new(self.albedo, self.fuzz))
    }
}

pub struct Dielectric {
    ref_idx : f32,
}
impl Dielectric {
    pub fn new(ri : f32 ) -> Dielectric {
        Dielectric{
            ref_idx: ri
        }
    }
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &ray::Ray, rec: &HitRecord,
               attenuation: &mut vec::Vec3, scattered: &mut ray::Ray) -> bool {

        let outward_normal;
        let reflected = reflect(r_in.direction(), rec.normal);
        let ni_over_nt;
        *attenuation = Vec3::new(1.0, 1.0, 1.0);

        let mut refracted = Vec3::new(0.0, 0.0, 0.0);
        let reflect_prob : f32;
        let cosine : f32;

        if vec::dot(r_in.direction(), rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * vec::dot(r_in.direction(), rec.normal) / r_in.direction().length();
        }
        else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -vec::dot(r_in.direction(), rec.normal) / r_in.direction().length();
        }

        if refract(r_in.direction(), outward_normal, ni_over_nt, &mut refracted) {
            reflect_prob = schlick(cosine, self.ref_idx);
        }
        else {
            *scattered = ray::Ray::new(rec.p, reflected);
            reflect_prob = 1.0;
        }
        if drand48() < reflect_prob {
            *scattered = ray::Ray::new(rec.p, reflected);
        }
        else {
            *scattered = ray::Ray::new(rec.p, refracted);
        }
        return true;
    }
    fn clone(&self) -> Box<Material> {
        Box::new(Dielectric::new(self.ref_idx))
    }
}


fn reflect(v : Vec3, n : Vec3) -> Vec3 {
    v - 2.0*vec::dot(v,n)*n
}

fn refract(v : Vec3, n : Vec3, ni_over_nt : f32, refracted : &mut Vec3) -> bool {
    let uv = vec::unit_vector(v);
    let dt= vec::dot(uv, n);
    let discriminant = 1.0 - ni_over_nt*ni_over_nt*(1.0-dt*dt);
    if discriminant > 0.0 {
        *refracted = ni_over_nt * (uv - n*dt) - n*f32::sqrt(discriminant);
        return true;
    }
    return false;
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
    r0 = r0*r0;
    r0 + (1.0-r0)*f32::powi(1.0 - cosine, 5)
}


fn random_in_unit_sphere() -> Vec3 {
    let mut p : Vec3;

    loop {
        p = 2.0*Vec3::new(drand48(),drand48(),drand48()) -
            Vec3::new(1.0, 1.0,1.0);

        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

