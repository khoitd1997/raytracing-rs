use super::color::Color;
use super::hittable::HitRecord;
use super::ray::Ray;
use super::vec3::Vec3;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}


#[derive(Debug, Clone, Default)]
pub struct Lambertian {
    albedo: Color,
}
impl Lambertian {
    pub fn new(a: &Color) -> Self {
        Self {
            albedo: a.to_owned(),
        }
    }
}
impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(&(rec.p), &scatter_direction);
        *attenuation = self.albedo.to_owned();
        return true;
    }
}

#[derive(Debug, Clone, Default)]
pub struct Metal {
    albedo: Color,
}
impl Metal {
    pub fn new(a: &Color) -> Self {
        Self {
            albedo: a.to_owned(),
        }
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(&(Vec3::unit_vector(&(r_in.direction()))), &(rec.normal));
        *scattered = Ray::new(&(rec.p), &reflected);
        *attenuation = self.albedo;
        return true;
    }
}
