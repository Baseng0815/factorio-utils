use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::blueprint::RawBlueprint;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Envelope {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blueprint: Option<RawBlueprint>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blueprint_book: Option<Value>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upgrade_planner: Option<Value>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deconstruction_planner: Option<Value>,
}

impl Envelope {
    pub fn variant_name(&self) -> Option<&'static str> {
        if self.blueprint.is_some() {
            Some("blueprint")
        } else if self.blueprint_book.is_some() {
            Some("blueprint_book")
        } else if self.upgrade_planner.is_some() {
            Some("upgrade_planner")
        } else if self.deconstruction_planner.is_some() {
            Some("deconstruction_planner")
        } else {
            None
        }
    }
}
