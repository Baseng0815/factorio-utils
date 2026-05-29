mod ingredient;
mod product;

pub use ingredient::Ingredient;
pub use product::{Product, ProductAmount};

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{machine::CraftingCategory, ResourceId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RecipeId(Cow<'static, str>);

impl RecipeId {
    pub fn new(name: impl Into<String>) -> Self {
        Self(Cow::Owned(name.into()))
    }

    pub const fn from_static(name: &'static str) -> Self {
        Self(Cow::Borrowed(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for RecipeId {
    fn from(s: String) -> Self {
        Self(Cow::Owned(s))
    }
}

impl From<&str> for RecipeId {
    fn from(s: &str) -> Self {
        Self(Cow::Owned(s.to_owned()))
    }
}

impl std::fmt::Display for RecipeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub category: CraftingCategory,
    pub ingredients: Vec<Ingredient>,
    pub products: Vec<Product>,
    pub crafting_time: f64,
    pub allow_productivity: bool,
}

impl Recipe {
    pub fn produces(&self, resource: impl Into<ResourceId>) -> bool {
        let resource: ResourceId = resource.into();

        self.products.iter().any(|p| p.resource == resource)
    }

    pub fn consumes(&self, resource: impl Into<ResourceId>) -> bool {
        let resource: ResourceId = resource.into();

        self.ingredients.iter().any(|i| i.resource == resource)
    }

    pub fn expected_yield(&self, resource: impl Into<ResourceId>) -> f64 {
        let resource: ResourceId = resource.into();

        self.products
            .iter()
            .filter(|p| p.resource == resource)
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
                "→ {} ({}s, {})",
                products, self.crafting_time, self.category,
            )
        } else {
            write!(
                f,
                "{} → {} ({}s, {})",
                ingredients, products, self.crafting_time, self.category,
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
