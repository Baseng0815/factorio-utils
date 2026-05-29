use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct RawRecipe {
    #[serde(default = "default_category")]
    pub category: String,
    #[serde(default = "default_energy")]
    pub energy_required: f64,
    #[serde(default)]
    pub ingredients: Vec<RawIngredient>,
    #[serde(default)]
    pub results: Vec<RawProduct>,
    #[serde(default)]
    pub allow_productivity: bool,
}

fn default_category() -> String {
    "crafting".into()
}

fn default_energy() -> f64 {
    0.5
}

#[derive(Debug, Deserialize)]
pub(super) struct RawIngredient {
    #[serde(default = "default_resource_type", rename = "type")]
    pub ty: String,
    pub name: String,
    #[serde(default = "default_amount")]
    pub amount: f64,
}

fn default_resource_type() -> String {
    "item".into()
}

fn default_amount() -> f64 {
    1.0
}

#[derive(Debug, Deserialize)]
pub(super) struct RawProduct {
    #[serde(default = "default_resource_type", rename = "type")]
    pub ty: String,
    pub name: String,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub amount_min: Option<f64>,
    #[serde(default)]
    pub amount_max: Option<f64>,
    #[serde(default = "default_probability")]
    pub probability: f64,
}

fn default_probability() -> f64 {
    1.0
}

#[derive(Debug, Deserialize)]
pub(super) struct RawItem {
    #[serde(default = "default_stack_size")]
    pub stack_size: u32,
}

fn default_stack_size() -> u32 {
    1
}

#[derive(Debug, Deserialize)]
pub(super) struct RawFluid {
    #[serde(default)]
    pub default_temperature: f64,
}

#[derive(Debug, Deserialize)]
pub(super) struct RawMachine {
    pub crafting_categories: Vec<String>,
    pub crafting_speed: f64,
    pub energy_usage: String,
    #[serde(default)]
    pub module_slots: Option<u32>,
    #[serde(default)]
    pub module_specification: Option<RawModuleSpec>,
}

#[derive(Debug, Deserialize)]
pub(super) struct RawModuleSpec {
    #[serde(default)]
    pub module_slots: u32,
}

#[derive(Debug, Deserialize)]
pub(super) struct RawMiningDrill {
    pub resource_categories: Vec<String>,
    pub mining_speed: f64,
    pub energy_usage: String,
    #[serde(default)]
    pub module_slots: Option<u32>,
    #[serde(default)]
    pub module_specification: Option<RawModuleSpec>,
}

#[derive(Debug, Deserialize)]
pub(super) struct RawResource {
    #[serde(default = "default_resource_category")]
    pub category: String,
    #[serde(default)]
    pub minable: Option<RawMinable>,
}

fn default_resource_category() -> String {
    "basic-solid".into()
}

#[derive(Debug, Deserialize)]
pub(super) struct RawMinable {
    #[serde(default = "default_mining_time")]
    pub mining_time: f64,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub count: Option<u32>,
    #[serde(default)]
    pub results: Vec<RawProduct>,
    #[serde(default)]
    pub required_fluid: Option<String>,
    #[serde(default)]
    pub fluid_amount: f64,
}

fn default_mining_time() -> f64 {
    1.0
}
