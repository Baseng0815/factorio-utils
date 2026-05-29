use std::collections::BTreeMap;

use itertools::Itertools;
use tracing::trace;

use super::{OpaqueJson, Position};
use crate::entity::{Entity, EntityKind, EntityNumber};

#[derive(Debug, Clone, Default)]
pub struct World {
    entities: BTreeMap<EntityNumber, Entity>,
    next_number: u64,
    pub tiles: OpaqueJson,
    pub schedules: OpaqueJson,
    pub icons: OpaqueJson,
    pub label: Option<String>,
    pub description: Option<String>,
    pub version: Option<u64>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, entity: impl Into<Entity>) -> EntityNumber {
        let mut entity = entity.into();
        let number = self.assign_number(entity.number);
        entity.number = number;
        trace!(?number, name = %entity.name, position = %entity.position, "inserting entity");
        self.entities.insert(number, entity);
        number
    }

    pub fn insert_with_number(&mut self, entity: Entity) -> crate::Result<EntityNumber> {
        let number = entity.number;
        if self.entities.contains_key(&number) {
            return Err(crate::Error::DuplicateEntityNumber(number.as_u64()));
        }
        self.bump_next_past(number);
        self.entities.insert(number, entity);
        Ok(number)
    }

    pub fn get(&self, number: EntityNumber) -> Option<&Entity> {
        self.entities.get(&number)
    }

    pub fn get_mut(&mut self, number: EntityNumber) -> Option<&mut Entity> {
        self.entities.get_mut(&number)
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    pub fn entities(&self) -> impl Iterator<Item = &Entity> {
        self.entities.values()
    }

    pub fn entities_sorted_by_number(&self) -> impl Iterator<Item = &Entity> {
        self.entities.values().sorted_by_key(|e| e.number)
    }

    pub fn at(&self, position: Position) -> impl Iterator<Item = &Entity> + '_ {
        self.entities
            .values()
            .filter(move |e| e.position == position)
    }

    pub fn kinds(&self) -> impl Iterator<Item = (&EntityKind, &Entity)> {
        self.entities.values().map(|e| (&e.kind, e))
    }

    fn assign_number(&mut self, requested: EntityNumber) -> EntityNumber {
        if requested.as_u64() != 0 && !self.entities.contains_key(&requested) {
            self.bump_next_past(requested);
            return requested;
        }
        let number = EntityNumber::new(self.next_number.max(1));
        self.next_number = number.as_u64() + 1;
        number
    }

    fn bump_next_past(&mut self, number: EntityNumber) {
        if number.as_u64() >= self.next_number {
            self.next_number = number.as_u64() + 1;
        }
    }
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "World {{ entities: {} }}", self.entities.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{Entity, TransportBelt};
    use crate::world::Direction;

    #[test]
    fn add_assigns_increasing_numbers() {
        let mut w = World::new();
        let a = w.add(Entity::new("transport-belt", (0.5, 0.5), TransportBelt));
        let b = w.add(Entity::new("transport-belt", (1.5, 0.5), TransportBelt));
        assert_eq!(a.as_u64(), 1);
        assert_eq!(b.as_u64(), 2);
        assert_eq!(w.len(), 2);
    }

    #[test]
    fn at_finds_entity_by_position() {
        let mut w = World::new();
        let pos = Position::new(3.5, 7.5);
        w.add(Entity::new("transport-belt", pos, TransportBelt).with_direction(Direction::East));
        assert_eq!(w.at(pos).count(), 1);
        assert_eq!(w.at(Position::new(0.0, 0.0)).count(), 0);
    }
}
