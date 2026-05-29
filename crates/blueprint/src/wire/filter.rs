use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawInserterFilter {
    pub index: u32,
    pub name: String,
}
