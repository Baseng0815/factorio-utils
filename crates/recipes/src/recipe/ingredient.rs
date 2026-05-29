use serde::{Deserialize, Serialize};

use crate::resource::ResourceId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub resource: ResourceId,
    pub amount: f64,
}

impl Ingredient {
    pub fn new(resource: ResourceId, amount: f64) -> Self {
        Self { resource, amount }
    }
}

impl std::fmt::Display for Ingredient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} × {}", self.amount, self.resource)
    }
}
