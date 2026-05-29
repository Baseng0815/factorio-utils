use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::machine::{CraftingCategory, Machine, MachineId};
use crate::recipe::{Recipe, RecipeId};
use crate::resource::{Fluid, FluidId, Item, ItemId};

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

    pub fn recipe(&self, id: &RecipeId) -> Option<&Recipe> {
        self.recipes.get(id)
    }

    pub fn machine(&self, id: &MachineId) -> Option<&Machine> {
        self.machines.get(id)
    }

    pub fn item(&self, id: &ItemId) -> Option<&Item> {
        self.items.get(id)
    }

    pub fn fluid(&self, id: &FluidId) -> Option<&Fluid> {
        self.fluids.get(id)
    }

    pub fn recipes_producing<'a>(
        &'a self,
        resource_name: &'a str,
    ) -> impl Iterator<Item = &'a Recipe> + 'a {
        self.recipes.values().filter(move |r| r.produces(resource_name))
    }

    pub fn recipes_in_category<'a>(
        &'a self,
        category: &'a CraftingCategory,
    ) -> impl Iterator<Item = &'a Recipe> + 'a {
        self.recipes.values().filter(move |r| &r.category == category)
    }

    pub fn machines_for_category<'a>(
        &'a self,
        category: &'a CraftingCategory,
    ) -> impl Iterator<Item = &'a Machine> + 'a {
        self.machines.values().filter(move |m| m.supports(category))
    }

    pub fn machines_for_recipe<'a>(
        &'a self,
        recipe: &'a Recipe,
    ) -> impl Iterator<Item = &'a Machine> + 'a {
        self.machines_for_category(&recipe.category)
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
