pub static PI: f64 = 3.1415926535897932385;
pub static INFINITY: f64 = f64::INFINITY;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}
