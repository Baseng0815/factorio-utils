use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BeltIo {
    #[default]
    Input,
    Output,
}

impl BeltIo {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Input => "input",
            Self::Output => "output",
        }
    }
}

impl From<&str> for BeltIo {
    fn from(s: &str) -> Self {
        match s {
            "input" => Self::Input,
            "output" => Self::Output,
            other => {
                warn!(value = other, "unknown belt I/O type; defaulting to Input");
                Self::Input
            }
        }
    }
}

impl std::fmt::Display for BeltIo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UndergroundBelt {
    pub io: BeltIo,
}

impl UndergroundBelt {
    pub fn input() -> Self {
        Self { io: BeltIo::Input }
    }

    pub fn output() -> Self {
        Self { io: BeltIo::Output }
    }

    pub fn with_io(mut self, io: impl Into<BeltIo>) -> Self {
        self.io = io.into();
        self
    }
}
