mod fluid;
mod item;

use derive_more::From;
pub use fluid::{Fluid, FluidId};
pub use item::{Item, ItemId};

use serde::{Deserialize, Serialize};

#[derive(Debug, From, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "lowercase")]
pub enum ResourceId {
    Item(ItemId),
    Fluid(FluidId),
}

impl ResourceId {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Item(id) => id.as_str(),
            Self::Fluid(id) => id.as_str(),
        }
    }

    pub fn is_item(&self) -> bool {
        matches!(self, Self::Item(_))
    }

    pub fn is_fluid(&self) -> bool {
        matches!(self, Self::Fluid(_))
    }
}

impl std::fmt::Display for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
