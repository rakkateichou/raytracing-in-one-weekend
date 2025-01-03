use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Default for HittableList {
    fn default() -> Self {
        Self { objects: Vec::new() }
    }
}

impl HittableList {
    pub fn add<T: Hittable + 'static>(&mut self, object: T) { // assuming we never deallocate
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut result: Option<HitRecord> = None;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(r, &Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit.t;
                result = Some(hit);
            }
        }
        return result;
    }
}
