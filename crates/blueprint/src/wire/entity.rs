use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::filter::RawInserterFilter;
use super::item_stack::RawItemStack;
use super::position::RawPosition;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawEntity {
    pub entity_number: u64,
    pub name: String,
    pub position: RawPosition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<u8>,
    #[serde(flatten)]
    pub extras: RawEntityExtras,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RawEntityExtras {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipe: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<RawItemStack>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<RawInserterFilter>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub override_stack_size: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub belt_io: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_priority: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_priority: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bar: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_filter: Option<String>,

    #[serde(flatten)]
    pub other: Map<String, Value>,
}
