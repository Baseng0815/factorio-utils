use thiserror::Error;

use recipes::{CraftingCategory, MachineId, RecipeId, ResourceId};

#[derive(Debug, Error)]
pub enum Error {
    #[error("no recipe produces `{0}` and it was not marked as a raw input")]
    NoRecipe(ResourceId),

    #[error("multiple recipes produce `{resource}`: {candidates:?}; specify one in PlanConfig::recipe_for")]
    AmbiguousRecipe {
        resource: ResourceId,
        candidates: Vec<RecipeId>,
    },

    #[error("recipe `{0}` selected for `{1}` does not actually produce it")]
    RecipeDoesNotProduce(RecipeId, ResourceId),

    #[error("recipe `{0}` not found in database")]
    UnknownRecipe(RecipeId),

    #[error("machine `{0}` not found in database")]
    UnknownMachine(MachineId),

    #[error("no machine in database supports crafting category `{0}`")]
    NoMachineForCategory(CraftingCategory),

    #[error("machine `{machine}` does not support category `{category}` required by recipe `{recipe}`")]
    MachineCategoryMismatch {
        machine: MachineId,
        recipe: RecipeId,
        category: CraftingCategory,
    },

    #[error("recipe `{0}` has zero expected yield for `{1}`; cannot use it as a producer")]
    NoYield(RecipeId, ResourceId),

    #[error("cycle detected involving recipe `{0}`; cyclic production not yet supported")]
    Cycle(RecipeId),
}

pub type Result<T> = std::result::Result<T, Error>;
