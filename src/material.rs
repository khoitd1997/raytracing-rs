use super::color::Color;
use super::hittable::HitRecord;
use super::ray::Ray;
use super::rtweekend::{partial_min, random_double};
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
    fuzz: f64,
}
impl Metal {
    pub fn new(a: &Color, f: f64) -> Self {
        Self {
            albedo: a.to_owned(),
            fuzz: if f < 1.0 { f } else { 1.0 },
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
        *scattered = Ray::new(
            &(rec.p),
            &(reflected + (Vec3::random_unit_vector() * self.fuzz)),
        );
        *attenuation = self.albedo;
        return Vec3::dot(&(scattered.direction()), &(rec.normal)) > 0.0;
    }
}

#[derive(Debug, Clone, Default)]
pub struct Dielectric {
    ir: f64,
}
impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * ((1.0 - cosine).powi(5));
    }
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = Vec3::unit_vector(&(r_in.direction()));
        let cos_theta = partial_min(Vec3::dot(&(-unit_direction), &(rec.normal)), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
        let direction;

        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double()
        {
            direction = Vec3::reflect(&unit_direction, &(rec.normal));
        } else {
            direction = Vec3::refract(&unit_direction, &(rec.normal), refraction_ratio);
        }

        *scattered = Ray::new(&(rec.p), &direction);
        return true;
    }
}
