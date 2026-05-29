mod ingredient;
mod product;

pub use ingredient::Ingredient;
pub use product::{Product, ProductAmount};

use serde::{Deserialize, Serialize};

use crate::machine::CraftingCategory;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RecipeId(String);

impl RecipeId {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for RecipeId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for RecipeId {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl std::fmt::Display for RecipeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: RecipeId,
    pub category: CraftingCategory,
    pub ingredients: Vec<Ingredient>,
    pub products: Vec<Product>,
    pub crafting_time: f64,
    pub allow_productivity: bool,
}

impl Recipe {
    pub fn produces(&self, resource_name: &str) -> bool {
        self.products
            .iter()
            .any(|p| p.resource.name() == resource_name)
    }

    pub fn consumes(&self, resource_name: &str) -> bool {
        self.ingredients
            .iter()
            .any(|i| i.resource.name() == resource_name)
    }

    pub fn expected_yield(&self, resource_name: &str) -> f64 {
        self.products
            .iter()
            .filter(|p| p.resource.name() == resource_name)
            .map(Product::expected_amount)
            .sum()
    }
}

impl std::fmt::Display for Recipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ingredients = join_display(&self.ingredients, " + ");
        let products = join_display(&self.products, " + ");
        if ingredients.is_empty() {
            write!(
                f,
                "{} → {} ({}s, {})",
                self.id, products, self.crafting_time, self.category,
            )
        } else {
            write!(
                f,
                "{}: {} → {} ({}s, {})",
                self.id, ingredients, products, self.crafting_time, self.category,
            )
        }
    }
}

fn join_display<T: std::fmt::Display>(items: &[T], sep: &str) -> String {
    items
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(sep)
}
