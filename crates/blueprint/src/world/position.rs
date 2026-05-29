use std::ops::{Add, Sub};

use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Display, Serialize, Deserialize)]
#[display("({x}, {y})")]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub const ORIGIN: Self = Self { x: 0.0, y: 0.0 };

    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn offset(self, dx: f64, dy: f64) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }

    pub fn distance_to(self, other: Position) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl From<(f64, f64)> for Position {
    fn from((x, y): (f64, f64)) -> Self {
        Self::new(x, y)
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x as f64, y as f64)
    }
}

impl Add for Position {
    type Output = Position;
    fn add(self, other: Position) -> Position {
        Position::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Position {
    type Output = Position;
    fn sub(self, other: Position) -> Position {
        Position::new(self.x - other.x, self.y - other.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_tuple_constructs_position() {
        let p: Position = (1.5, 2.5).into();
        assert_eq!(p, Position::new(1.5, 2.5));
    }

    #[test]
    fn offset_shifts_by_delta() {
        assert_eq!(Position::new(1.0, 2.0).offset(0.5, -1.0), Position::new(1.5, 1.0));
    }

    #[test]
    fn arithmetic_works() {
        let a = Position::new(1.0, 2.0);
        let b = Position::new(3.0, 4.0);
        assert_eq!(a + b, Position::new(4.0, 6.0));
        assert_eq!(b - a, Position::new(2.0, 2.0));
    }

    #[test]
    fn distance_to_is_euclidean() {
        assert_eq!(Position::new(0.0, 0.0).distance_to(Position::new(3.0, 4.0)), 5.0);
    }
}
