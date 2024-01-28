use rand::Rng;

pub static PI: f64 = 3.1415926535897932385;
pub static INFINITY: f64 = f64::INFINITY;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn partial_min(lhs: f64, rhs: f64) -> f64 {
    if lhs < rhs {
        return lhs;
    }
    return rhs;
}
