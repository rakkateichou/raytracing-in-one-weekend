use dyn_clone::DynClone;

use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material: DynClone {
    // TODO: make it so the Ray have a certain probability to scatter, not to always scatter with some attenuation
    // i feel like this will look nicer
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

dyn_clone::clone_trait_object!(Material);

#[derive(Clone, Copy, Default)]
pub struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> { 
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

#[derive(Clone, Copy, Default)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(&ray.direction, &rec.normal).unit_vector() + Vec3::random_unit_vector() * self.fuzz;
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;
        if scattered.direction.dot(&rec.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
