use serde::{Deserialize, Serialize};

pub const DEFAULT_ICON_SIZE: u32 = 64;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IconRef {
    pub path: String,
    pub size: u32,
}

impl IconRef {
    pub fn new(path: impl Into<String>, size: u32) -> Self {
        Self {
            path: path.into(),
            size,
        }
    }
}

impl std::fmt::Display for IconRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}px)", self.path, self.size)
    }
}
