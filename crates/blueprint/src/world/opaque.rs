use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OpaqueJson(pub Value);

impl OpaqueJson {
    pub fn null() -> Self {
        Self(Value::Null)
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

impl From<Value> for OpaqueJson {
    fn from(v: Value) -> Self {
        Self(v)
    }
}

impl std::fmt::Display for OpaqueJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
