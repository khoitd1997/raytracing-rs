use super::hittable::{HitRecord, Hittable};
use super::interval::Interval;
use super::ray::Ray;
use std::cell::RefCell;
use std::rc::Rc;

pub struct HittableList {
    pub objects: Vec<Rc<RefCell<dyn Hittable>>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    pub fn new_1(obj: Rc<RefCell<dyn Hittable>>) -> Self {
        HittableList { objects: vec![obj] }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, obj: Rc<RefCell<dyn Hittable>>) {
        self.objects.push(obj)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            ..Default::default()
        };
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if object.borrow().hit(
                r,
                Interval::new_val(ray_t.min, closest_so_far),
                &mut temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.to_owned();
            }
        }

        return hit_anything;
    }
}
