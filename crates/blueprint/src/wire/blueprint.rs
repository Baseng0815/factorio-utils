use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::entity::RawEntity;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RawBlueprint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icons: Option<Value>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<RawEntity>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tiles: Option<Value>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedules: Option<Value>,
}
