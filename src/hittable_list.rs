use super::hittable::{HitRecord, Hittable};
use super::interval::Interval;
use super::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Sync + Send>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, obj: Box<dyn Hittable + Sync + Send>) {
        self.objects.push(obj)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            let mut temp_rec = HitRecord {
                ..Default::default()
            };
            if object.hit(
                r,
                Interval::new_val(ray_t.min, closest_so_far),
                &mut temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        return hit_anything;
    }
}
