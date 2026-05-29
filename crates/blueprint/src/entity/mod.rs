mod assembling_machine;
mod belt;
mod chest;
mod furnace;
mod inserter;
mod item_stack;
mod mining_drill;
mod number;
mod pipe;
mod pipe_to_ground;
mod power_pole;
mod pump;
mod splitter;
mod underground_belt;

pub use assembling_machine::AssemblingMachine;
pub use belt::TransportBelt;
pub use chest::Chest;
pub use furnace::Furnace;
pub use inserter::Inserter;
pub use item_stack::ItemStack;
pub use mining_drill::MiningDrill;
pub use number::EntityNumber;
pub use pipe::Pipe;
pub use pipe_to_ground::PipeToGround;
pub use power_pole::PowerPole;
pub use pump::Pump;
pub use splitter::{Splitter, SplitterPriority};
pub use underground_belt::{BeltIo, UndergroundBelt};

use derive_more::From;

use crate::world::{Direction, EntityName, Position};

#[derive(Debug, Clone, PartialEq, From)]
pub enum EntityKind {
    AssemblingMachine(AssemblingMachine),
    Furnace(Furnace),
    MiningDrill(MiningDrill),
    Inserter(Inserter),
    TransportBelt(TransportBelt),
    UndergroundBelt(UndergroundBelt),
    Splitter(Splitter),
    Pipe(Pipe),
    PipeToGround(PipeToGround),
    Pump(Pump),
    Chest(Chest),
    PowerPole(PowerPole),
}

impl EntityKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AssemblingMachine(_) => "assembling-machine",
            Self::Furnace(_) => "furnace",
            Self::MiningDrill(_) => "mining-drill",
            Self::Inserter(_) => "inserter",
            Self::TransportBelt(_) => "transport-belt",
            Self::UndergroundBelt(_) => "underground-belt",
            Self::Splitter(_) => "splitter",
            Self::Pipe(_) => "pipe",
            Self::PipeToGround(_) => "pipe-to-ground",
            Self::Pump(_) => "pump",
            Self::Chest(_) => "chest",
            Self::PowerPole(_) => "power-pole",
        }
    }
}

impl std::fmt::Display for EntityKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    pub number: EntityNumber,
    pub name: EntityName,
    pub position: Position,
    pub direction: Direction,
    pub kind: EntityKind,
}

impl Entity {
    pub fn new(
        name: impl Into<EntityName>,
        position: impl Into<Position>,
        kind: impl Into<EntityKind>,
    ) -> Self {
        Self {
            number: EntityNumber::new(0),
            name: name.into(),
            position: position.into(),
            direction: Direction::default(),
            kind: kind.into(),
        }
    }

    pub fn with_direction(mut self, direction: impl Into<Direction>) -> Self {
        self.direction = direction.into();
        self
    }

    pub fn with_position(mut self, position: impl Into<Position>) -> Self {
        self.position = position.into();
        self
    }

    pub fn with_number(mut self, number: impl Into<EntityNumber>) -> Self {
        self.number = number.into();
        self
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} @ {} ({}) [{}]",
            self.number, self.name, self.position, self.direction, self.kind,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use recipes::RecipeId;

    #[test]
    fn entity_new_builds_with_defaults() {
        let e = Entity::new(
            "assembling-machine-2",
            (3.5, 4.5),
            AssemblingMachine::default().with_recipe(RecipeId::from("iron-gear-wheel")),
        );
        assert_eq!(e.name.as_str(), "assembling-machine-2");
        assert_eq!(e.position, Position::new(3.5, 4.5));
        assert_eq!(e.direction, Direction::North);
        assert!(matches!(e.kind, EntityKind::AssemblingMachine(_)));
    }

    #[test]
    fn with_direction_sets_field() {
        let e = Entity::new("transport-belt", (0.5, 0.5), TransportBelt)
            .with_direction(Direction::East);
        assert_eq!(e.direction, Direction::East);
    }
}
