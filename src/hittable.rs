use super::ray::Ray;
use super::vec3::{Point3, Vec3};
use super::interval::Interval;

#[derive(Debug, Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
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
