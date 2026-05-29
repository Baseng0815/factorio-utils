use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Rate(f64);

impl Rate {
    pub const ZERO: Self = Self(0.0);

    pub fn per_second(items: f64) -> Self {
        Self(items)
    }

    pub fn per_minute(items: f64) -> Self {
        Self(items / 60.0)
    }

    pub fn per_hour(items: f64) -> Self {
        Self(items / 3600.0)
    }

    pub fn as_per_second(self) -> f64 {
        self.0
    }

    pub fn as_per_minute(self) -> f64 {
        self.0 * 60.0
    }

    pub fn as_per_hour(self) -> f64 {
        self.0 * 3600.0
    }

    pub fn is_zero(self) -> bool {
        self.0 == 0.0
    }

    pub fn is_negligible(self, eps: f64) -> bool {
        self.0.abs() < eps
    }
}

impl Add for Rate {
    type Output = Rate;
    fn add(self, other: Rate) -> Rate {
        Rate(self.0 + other.0)
    }
}

impl AddAssign for Rate {
    fn add_assign(&mut self, other: Rate) {
        self.0 += other.0;
    }
}

impl Sub for Rate {
    type Output = Rate;
    fn sub(self, other: Rate) -> Rate {
        Rate(self.0 - other.0)
    }
}

impl SubAssign for Rate {
    fn sub_assign(&mut self, other: Rate) {
        self.0 -= other.0;
    }
}

impl Mul<f64> for Rate {
    type Output = Rate;
    fn mul(self, scalar: f64) -> Rate {
        Rate(self.0 * scalar)
    }
}

impl std::fmt::Display for Rate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.3}/s", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn per_minute_converts_to_per_second() {
        let r = Rate::per_minute(60.0);
        assert_eq!(r.as_per_second(), 1.0);
    }

    #[test]
    fn per_hour_converts_to_per_second() {
        let r = Rate::per_hour(3600.0);
        assert_eq!(r.as_per_second(), 1.0);
    }

    #[test]
    fn arithmetic() {
        let a = Rate::per_second(2.0);
        let b = Rate::per_second(3.0);
        assert_eq!((a + b).as_per_second(), 5.0);
        assert_eq!((b - a).as_per_second(), 1.0);
        assert_eq!((a * 4.0).as_per_second(), 8.0);
    }
}
