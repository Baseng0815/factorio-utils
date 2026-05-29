pub mod database;
pub mod dump;
pub mod error;
pub mod icon;
pub mod machine;
pub mod recipe;
pub mod resource;

pub use database::Database;
pub use error::{Error, Result};
pub use icon::IconRef;
pub use machine::{CraftingCategory, Machine, MachineId, MachineKind};
pub use recipe::{Ingredient, Product, ProductAmount, Recipe, RecipeId};
pub use resource::{Fluid, FluidId, Item, ItemId, ResourceId};

include!(concat!(env!("OUT_DIR"), "/vanilla_ids.rs"));
