use super::vec3::{Point3, Vec3};

#[derive(Clone, Debug, Default)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Ray {
            orig: origin.to_owned(),
            dir: direction.to_owned(),
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig.to_owned()
    }
    pub fn direction(&self) -> Vec3 {
        self.dir.to_owned()
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}
