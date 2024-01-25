use super::rtweekend::INFINITY;


#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new() -> Self {
        Interval {
            min: -INFINITY,
            max: INFINITY,
        }
    }

    pub const fn new_val(min: f64, max: f64) -> Self {
        Interval {
            min,
            max
        }
    }

    pub fn contains(&self, x: f64) -> bool {
        return self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        return self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min { return self.min; }
        if x > self.max { return self.max; }
        return x;
    }
}

pub static EMPTY: Interval = Interval::new_val(INFINITY, -INFINITY);
pub static UNIVERSE: Interval = Interval::new_val(-INFINITY, INFINITY);

