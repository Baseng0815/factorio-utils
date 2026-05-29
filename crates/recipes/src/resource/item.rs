use serde::{Deserialize, Serialize};

use crate::icon::IconRef;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ItemId(String);

impl ItemId {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for ItemId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for ItemId {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl std::fmt::Display for ItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: ItemId,
    pub stack_size: u32,
    #[serde(default)]
    pub icon: Option<IconRef>,
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [stack {}]", self.id, self.stack_size)
    }
}
