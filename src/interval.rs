#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Interval {
    pub lo: f64,
    pub hi: f64,
}

impl Interval {
    pub fn constant(c: f64) -> Self {
        Interval { lo: c, hi: c }
    }
    pub fn add(a: &Interval, b: &Interval) -> Self {
        Interval { lo: a.lo + b.lo, hi: a.hi + b.hi }
    }
    pub fn sub(a: &Interval, b: &Interval) -> Self {
        Interval { lo: a.lo - b.hi, hi: a.hi - b.lo }
    }
    pub fn mul(a: &Interval, b: &Interval) -> Self {
        let products = [
            a.lo * b.lo, a.lo * b.hi,
            a.hi * b.lo, a.hi * b.hi
        ];
        Interval {
            lo: products.iter().cloned().fold(f64::INFINITY, f64::min),
            hi: products.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        }
    }
    pub fn neg(a: &Interval) -> Self {
        Interval { lo: -a.hi, hi: -a.lo }
    }
    pub fn square(a: &Interval) -> Self {
        if a.lo >= 0.0 {
            Interval { lo: a.lo * a.lo, hi: a.hi * a.hi }
        } else if a.hi <= 0.0 {
            Interval { lo: a.hi * a.hi, hi: a.lo * a.lo }
        } else {
            Interval { lo: 0.0, hi: f64::max(a.lo.abs(), a.hi.abs()).powi(2) }
        }
    }
    pub fn sqrt(a: &Interval) -> Self {
        Interval {
            lo: a.lo.max(0.0).sqrt(),
            hi: a.hi.sqrt(),
        }
    }
    pub fn min(a: &Interval, b: &Interval) -> Self {
        Interval { lo: a.lo.min(b.lo), hi: a.hi.min(b.hi) }
    }
    pub fn max(a: &Interval, b: &Interval) -> Self {
        Interval { lo: a.lo.max(b.lo), hi: a.hi.max(b.hi) }
    }
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(intv {} {})", self.lo, self.hi)
    }
}