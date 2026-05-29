use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EntityNumber(u64);

impl EntityNumber {
    pub fn new(n: u64) -> Self {
        Self(n)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl From<u64> for EntityNumber {
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl From<EntityNumber> for u64 {
    fn from(n: EntityNumber) -> u64 {
        n.0
    }
}

impl std::fmt::Display for EntityNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0)
    }
}
