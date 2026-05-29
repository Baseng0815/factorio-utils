use serde::{Deserialize, Serialize};
use tracing::warn;

use recipes::ItemId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SplitterPriority {
    Left,
    Right,
}

impl SplitterPriority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

impl From<&str> for SplitterPriority {
    fn from(s: &str) -> Self {
        match s {
            "left" => Self::Left,
            "right" => Self::Right,
            other => {
                warn!(value = other, "unknown splitter priority; defaulting to Left");
                Self::Left
            }
        }
    }
}

impl std::fmt::Display for SplitterPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Splitter {
    pub input_priority: Option<SplitterPriority>,
    pub output_priority: Option<SplitterPriority>,
    pub filter: Option<ItemId>,
}

impl Splitter {
    pub fn with_input_priority(mut self, p: impl Into<SplitterPriority>) -> Self {
        self.input_priority = Some(p.into());
        self
    }

    pub fn with_output_priority(mut self, p: impl Into<SplitterPriority>) -> Self {
        self.output_priority = Some(p.into());
        self
    }

    pub fn with_filter(mut self, item: impl Into<ItemId>) -> Self {
        self.filter = Some(item.into());
        self
    }
}
