use std::collections::HashMap;
use std::ops::Index;

use serde::{Deserialize, Serialize};

use crate::machine::{CraftingCategory, Machine, MachineId};
use crate::recipe::{Recipe, RecipeId};
use crate::resource::{Fluid, FluidId, Item, ItemId};
use crate::ResourceId;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Database {
    pub items: HashMap<ItemId, Item>,
    pub fluids: HashMap<FluidId, Fluid>,
    pub recipes: HashMap<RecipeId, Recipe>,
    pub machines: HashMap<MachineId, Machine>,
}

impl Database {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn recipes_producing<'a>(
        &'a self,
        resource: impl Into<ResourceId>,
    ) -> impl Iterator<Item = &'a RecipeId> + 'a {
        let resource: ResourceId = resource.into();

        self.recipes
            .iter()
            .filter_map(move |(id, r)| r.produces(resource.clone()).then_some(id))
    }

    pub fn recipes_in_category<'a>(
        &'a self,
        category: &'a CraftingCategory,
    ) -> impl Iterator<Item = &'a RecipeId> + 'a {
        self.recipes
            .iter()
            .filter_map(move |(id, r)| (&r.category == category).then_some(id))
    }

    pub fn machines_for_category<'a>(
        &'a self,
        category: &'a CraftingCategory,
    ) -> impl Iterator<Item = &'a MachineId> + 'a {
        self.machines
            .iter()
            .filter_map(move |(id, m)| m.supports(category).then_some(id))
    }

    pub fn machines_for_recipe<'a>(
        &'a self,
        recipe: &'a Recipe,
    ) -> impl Iterator<Item = &'a MachineId> + 'a {
        self.machines_for_category(&recipe.category)
    }
}

impl Index<&ItemId> for Database {
    type Output = Item;

    fn index(&self, index: &ItemId) -> &Self::Output {
        &self.items[index]
    }
}

impl Index<&FluidId> for Database {
    type Output = Fluid;

    fn index(&self, index: &FluidId) -> &Self::Output {
        &self.fluids[index]
    }
}

impl Index<&RecipeId> for Database {
    type Output = Recipe;

    fn index(&self, index: &RecipeId) -> &Self::Output {
        &self.recipes[index]
    }
}

impl Index<&MachineId> for Database {
    type Output = Machine;

    fn index(&self, index: &MachineId) -> &Self::Output {
        &self.machines[index]
    }
}

impl std::fmt::Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Database {{ items: {}, fluids: {}, recipes: {}, machines: {} }}",
            self.items.len(),
            self.fluids.len(),
            self.recipes.len(),
            self.machines.len(),
        )
    }
}
