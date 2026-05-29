use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(into = "u8", from = "u8")]
pub enum Direction {
    #[default]
    North,
    NorthNorthEast,
    NorthEast,
    EastNorthEast,
    East,
    EastSouthEast,
    SouthEast,
    SouthSouthEast,
    South,
    SouthSouthWest,
    SouthWest,
    WestSouthWest,
    West,
    WestNorthWest,
    NorthWest,
    NorthNorthWest,
}

impl Direction {
    pub const NORTH: Self = Self::North;
    pub const EAST: Self = Self::East;
    pub const SOUTH: Self = Self::South;
    pub const WEST: Self = Self::West;

    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn opposite(self) -> Self {
        Self::from((self.as_u8() + 8) % 16)
    }

    pub fn rotate_clockwise_quarter(self) -> Self {
        Self::from((self.as_u8() + 4) % 16)
    }
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value & 0x0F {
            0 => Self::North,
            1 => Self::NorthNorthEast,
            2 => Self::NorthEast,
            3 => Self::EastNorthEast,
            4 => Self::East,
            5 => Self::EastSouthEast,
            6 => Self::SouthEast,
            7 => Self::SouthSouthEast,
            8 => Self::South,
            9 => Self::SouthSouthWest,
            10 => Self::SouthWest,
            11 => Self::WestSouthWest,
            12 => Self::West,
            13 => Self::WestNorthWest,
            14 => Self::NorthWest,
            15 => Self::NorthNorthWest,
            other => {
                warn!(value = other, "direction value out of 0..16; clamping to North");
                Self::North
            }
        }
    }
}

impl From<Direction> for u8 {
    fn from(d: Direction) -> u8 {
        d.as_u8()
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::North => "N",
            Self::NorthNorthEast => "NNE",
            Self::NorthEast => "NE",
            Self::EastNorthEast => "ENE",
            Self::East => "E",
            Self::EastSouthEast => "ESE",
            Self::SouthEast => "SE",
            Self::SouthSouthEast => "SSE",
            Self::South => "S",
            Self::SouthSouthWest => "SSW",
            Self::SouthWest => "SW",
            Self::WestSouthWest => "WSW",
            Self::West => "W",
            Self::WestNorthWest => "WNW",
            Self::NorthWest => "NW",
            Self::NorthNorthWest => "NNW",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn east_roundtrips_through_u8() {
        assert_eq!(Direction::from(4u8), Direction::East);
        assert_eq!(u8::from(Direction::East), 4);
    }

    #[test]
    fn opposite_inverts() {
        assert_eq!(Direction::North.opposite(), Direction::South);
        assert_eq!(Direction::East.opposite(), Direction::West);
        assert_eq!(Direction::NorthEast.opposite(), Direction::SouthWest);
    }

    #[test]
    fn rotate_clockwise_quarter_steps_through_compass() {
        let mut d = Direction::North;
        d = d.rotate_clockwise_quarter();
        assert_eq!(d, Direction::East);
        d = d.rotate_clockwise_quarter();
        assert_eq!(d, Direction::South);
    }
}
