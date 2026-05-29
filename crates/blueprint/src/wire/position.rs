use serde::{Deserialize, Serialize};

use crate::world::Position;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RawPosition {
    pub x: f64,
    pub y: f64,
}

impl From<Position> for RawPosition {
    fn from(p: Position) -> Self {
        Self { x: p.x, y: p.y }
    }
}

impl From<RawPosition> for Position {
    fn from(r: RawPosition) -> Self {
        Position::new(r.x, r.y)
    }
}
