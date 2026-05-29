use derive_more::Display;

use super::Position;

#[derive(Debug, Clone, Copy, PartialEq, Display)]
#[display("[{min} .. {max}]")]
pub struct BoundingBox {
    pub min: Position,
    pub max: Position,
}

impl BoundingBox {
    pub fn from_center_and_size(center: Position, width: f64, height: f64) -> Self {
        let half_w = width / 2.0;
        let half_h = height / 2.0;
        Self {
            min: center.offset(-half_w, -half_h),
            max: center.offset(half_w, half_h),
        }
    }

    pub fn width(&self) -> f64 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f64 {
        self.max.y - self.min.y
    }

    pub fn contains(&self, p: Position) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_center_and_size_is_symmetric() {
        let b = BoundingBox::from_center_and_size(Position::new(0.0, 0.0), 2.0, 4.0);
        assert_eq!(b.min, Position::new(-1.0, -2.0));
        assert_eq!(b.max, Position::new(1.0, 2.0));
        assert_eq!(b.width(), 2.0);
        assert_eq!(b.height(), 4.0);
    }

    #[test]
    fn contains_includes_boundary() {
        let b = BoundingBox::from_center_and_size(Position::ORIGIN, 2.0, 2.0);
        assert!(b.contains(Position::new(1.0, 1.0)));
        assert!(!b.contains(Position::new(2.0, 0.0)));
    }

    #[test]
    fn intersects_when_overlapping() {
        let a = BoundingBox::from_center_and_size(Position::new(0.0, 0.0), 2.0, 2.0);
        let b = BoundingBox::from_center_and_size(Position::new(1.5, 0.0), 2.0, 2.0);
        assert!(a.intersects(&b));
        let c = BoundingBox::from_center_and_size(Position::new(5.0, 0.0), 2.0, 2.0);
        assert!(!a.intersects(&c));
    }
}
