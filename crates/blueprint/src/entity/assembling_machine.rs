use recipes::{ItemId, RecipeId};

use super::ItemStack;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AssemblingMachine {
    pub recipe: Option<RecipeId>,
    pub modules: Vec<ItemStack>,
}

impl AssemblingMachine {
    pub fn with_recipe(mut self, recipe: impl Into<RecipeId>) -> Self {
        self.recipe = Some(recipe.into());
        self
    }

    pub fn with_module(mut self, module: impl Into<ItemId>, count: u32) -> Self {
        self.modules.push(ItemStack {
            item: module.into(),
            count,
        });
        self
    }
}
