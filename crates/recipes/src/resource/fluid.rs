use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::icon::IconRef;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FluidId(Cow<'static, str>);

impl FluidId {
    pub fn new(name: impl Into<String>) -> Self {
        Self(Cow::Owned(name.into()))
    }

    pub const fn from_static(name: &'static str) -> Self {
        Self(Cow::Borrowed(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for FluidId {
    fn from(s: String) -> Self {
        Self(Cow::Owned(s))
    }
}

impl From<&str> for FluidId {
    fn from(s: &str) -> Self {
        Self(Cow::Owned(s.to_owned()))
    }
}

impl std::fmt::Display for FluidId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fluid {
    pub default_temperature: f64,
    #[serde(default)]
    pub icon: Option<IconRef>,
}

impl std::fmt::Display for Fluid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}°C]", self.default_temperature)
    }
}
