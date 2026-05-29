use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawItemStack {
    pub item: String,
    pub count: u32,
}
