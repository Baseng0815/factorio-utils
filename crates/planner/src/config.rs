use std::collections::{HashMap, HashSet};

use recipes::{CraftingCategory, MachineId, RecipeId, ResourceId};

use crate::rate::Rate;

#[derive(Debug, Default, Clone)]
pub struct PlanConfig {
    pub recipe_for: HashMap<ResourceId, RecipeId>,
    pub machine_for_category: HashMap<CraftingCategory, MachineId>,
    pub raw: HashSet<ResourceId>,
}

impl PlanConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_recipe(mut self, resource: ResourceId, recipe: RecipeId) -> Self {
        self.recipe_for.insert(resource, recipe);
        self
    }

    pub fn with_machine(mut self, category: CraftingCategory, machine: MachineId) -> Self {
        self.machine_for_category.insert(category, machine);
        self
    }

    pub fn with_raw(mut self, resource: ResourceId) -> Self {
        self.raw.insert(resource);
        self
    }
}

#[derive(Debug, Default, Clone)]
pub struct PlanRequest {
    pub targets: HashMap<ResourceId, Rate>,
    pub config: PlanConfig,
}

impl PlanRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn want(mut self, resource: ResourceId, rate: Rate) -> Self {
        *self.targets.entry(resource).or_insert(Rate::ZERO) += rate;
        self
    }

    pub fn with_config(mut self, config: PlanConfig) -> Self {
        self.config = config;
        self
    }
}
