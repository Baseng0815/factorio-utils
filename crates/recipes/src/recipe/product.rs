use serde::{Deserialize, Serialize};

use crate::resource::ResourceId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub resource: ResourceId,
    pub amount: ProductAmount,
    pub probability: f64,
}

impl Product {
    pub fn new(resource: ResourceId, amount: ProductAmount, probability: f64) -> Self {
        Self {
            resource,
            amount,
            probability,
        }
    }

    pub fn expected_amount(&self) -> f64 {
        self.amount.expected() * self.probability
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProductAmount {
    Fixed(f64),
    Range { min: f64, max: f64 },
}

impl ProductAmount {
    pub fn expected(&self) -> f64 {
        match *self {
            Self::Fixed(n) => n,
            Self::Range { min, max } => (min + max) / 2.0,
        }
    }
}

impl std::fmt::Display for ProductAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Fixed(n) => write!(f, "{}", n),
            Self::Range { min, max } => write!(f, "{}–{}", min, max),
        }
    }
}

impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} × {}", self.amount, self.resource)?;
        if self.probability < 1.0 {
            write!(f, " (p={})", self.probability)?;
        }
        Ok(())
    }
}
