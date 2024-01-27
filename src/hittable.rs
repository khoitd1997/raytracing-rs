use super::ray::Ray;
use super::vec3::{Point3, Vec3};
use super::interval::Interval;
use super::material::{Material, Lambertian};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Rc<RefCell<dyn Material>>,
    pub t: f64,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::default(),
            normal: Point3::default(),
            mat: Rc::new(RefCell::new(Lambertian::default())),
            t: 0.0,
            front_face: false,
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&(r.direction()), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.to_owned()
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    ///
    /// Check if object hits a ray
    /// * `r` - Ray
    /// * `ray_t` - Ray Interval
    /// * `rec` - Hit record to modify
    /// # Returns
    /// Return true if it's hit
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
