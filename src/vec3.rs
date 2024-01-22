use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;
use std::string::ToString;

pub type Point3 = Vec3;

#[derive(Debug, Clone, Copy)]
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

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self::Output {
        Vec3 {
            e: (self.x() * t, self.y() * t, self.z() * t),
        }
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

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
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

    pub fn unit_vector(&self) -> Self {
        self.to_owned() / self.length()
    }
}
