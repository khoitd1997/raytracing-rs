use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;
use std::string::ToString;

use super::rtweekend::{random_double, random_double_range};

pub type Point3 = Vec3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub e: (f64, f64, f64),
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: (-self.x(), -self.y(), -self.z()),
        }
    }
}
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -(&self)
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Self::Output {
        Vec3 {
            e: (
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ),
        }
    }
}
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self::Output {
        self + (&other)
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: &Self) -> Self::Output {
        Vec3 {
            e: (
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ),
        }
    }
}
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self - (&other)
    }
}
impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        self.to_owned() - (&other)
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Self::Output {
        Vec3 {
            e: (self.x() * t, self.y() * t, self.z() * t),
        }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Self::Output {
        (&self) * t
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Vec3 {
            e: (
                self.x() * other.x(),
                self.y() * other.y(),
                self.z() * other.z(),
            ),
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self::Output {
        self * (1.0 / t)
    }
}

impl ToString for Vec3 {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.x(), self.y(), self.z())
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { e: (x, y, z) }
    }
    pub fn new_random() -> Self {
        Vec3::new(random_double(), random_double(), random_double())
    }
    pub fn new_random_range(min: f64, max: f64) -> Self {
        Vec3::new(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }

    pub fn x(&self) -> f64 {
        self.e.0
    }
    pub fn y(&self) -> f64 {
        self.e.1
    }
    pub fn z(&self) -> f64 {
        self.e.2
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return (self.e.0.abs() < s) && (self.e.1.abs() < s) && (self.e.2.abs() < s);
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(lhs: &Self, rhs: &Self) -> f64 {
        lhs.x() * rhs.x() + lhs.y() * rhs.y() + lhs.z() * rhs.z()
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3 {
            e: (
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
            ),
        }
    }

    #[inline(always)]
    pub fn unit_vector(v: &Vec3) -> Self {
        v.to_owned() / v.length()
    }

    #[inline(always)]
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::new_random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    #[inline(always)]
    pub fn random_unit_vector() -> Self {
        Self::unit_vector(&(Self::random_in_unit_sphere()))
    }

    #[inline(always)]
    pub fn random_on_hemisphere(normal: &Self) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if Self::dot(&on_unit_sphere, normal) > 0.0 {
            return on_unit_sphere;
        } else {
            return -on_unit_sphere;
        }
    }

    #[inline(always)]
    pub fn reflect(v: &Self, n: &Self) -> Self {
        return v - (n * Self::dot(v, n));
    }
}
