use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use tracing::{error, instrument, warn};

use crate::icon::IconRef;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MachineId(Cow<'static, str>);

impl MachineId {
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

impl From<String> for MachineId {
    fn from(s: String) -> Self {
        Self(Cow::Owned(s))
    }
}

impl From<&str> for MachineId {
    fn from(s: &str) -> Self {
        Self(Cow::Owned(s.to_owned()))
    }
}

impl std::fmt::Display for MachineId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CraftingCategory(Cow<'static, str>);

impl CraftingCategory {
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

impl From<String> for CraftingCategory {
    fn from(s: String) -> Self {
        Self(Cow::Owned(s))
    }
}

impl From<&str> for CraftingCategory {
    fn from(s: &str) -> Self {
        Self(Cow::Owned(s.to_owned()))
    }
}

impl std::fmt::Display for CraftingCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MachineKind {
    AssemblingMachine,
    Furnace,
    RocketSilo,
    MiningDrill,
    Other,
}

impl std::fmt::Display for MachineKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::AssemblingMachine => "assembling-machine",
            Self::Furnace => "furnace",
            Self::RocketSilo => "rocket-silo",
            Self::MiningDrill => "mining-drill",
            Self::Other => "other",
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Machine {
    pub kind: MachineKind,
    pub crafting_categories: Vec<CraftingCategory>,
    pub crafting_speed: f64,
    pub module_slots: u32,
    pub energy_usage_watts: f64,
    #[serde(default)]
    pub icon: Option<IconRef>,
}

impl Machine {
    pub fn supports(&self, category: &CraftingCategory) -> bool {
        self.crafting_categories.contains(category)
    }

    #[instrument(level = "trace", skip(self))]
    pub fn crafts_per_second(&self, recipe_crafting_time: f64) -> f64 {
        if recipe_crafting_time < 0.0 {
            error!("negative crafting time; treating throughput as zero");
            return 0.0;
        }
        if recipe_crafting_time == 0.0 {
            warn!("zero crafting time; treating throughput as zero to avoid division by zero");
            return 0.0;
        }
        self.crafting_speed / recipe_crafting_time
    }
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}) speed={} slots={} energy={} categories=[{}]",
            self.kind,
            self.crafting_speed,
            self.module_slots,
            format_watts(self.energy_usage_watts),
            join_categories(&self.crafting_categories),
        )
    }
}

fn join_categories(cats: &[CraftingCategory]) -> String {
    cats.iter()
        .map(CraftingCategory::as_str)
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_watts(w: f64) -> String {
    if w.abs() >= 1e9 {
        format!("{:.2} GW", w / 1e9)
    } else if w.abs() >= 1e6 {
        format!("{:.2} MW", w / 1e6)
    } else if w.abs() >= 1e3 {
        format!("{:.2} kW", w / 1e3)
    } else {
        format!("{:.0} W", w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drill() -> Machine {
        Machine {
            kind: MachineKind::AssemblingMachine,
            crafting_categories: vec![CraftingCategory::from("crafting")],
            crafting_speed: 2.0,
            module_slots: 0,
            energy_usage_watts: 0.0,
            icon: None,
        }
    }

    fn init_tracing() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("off")),
            )
            .with_test_writer()
            .try_init();
    }

    #[test]
    fn crafts_per_second_returns_zero_for_negative_time() {
        init_tracing();
        assert_eq!(drill().crafts_per_second(-1.0), 0.0);
    }

    #[test]
    fn crafts_per_second_returns_zero_for_zero_time() {
        init_tracing();
        assert_eq!(drill().crafts_per_second(0.0), 0.0);
    }

    #[test]
    fn crafts_per_second_divides_speed_by_time() {
        assert_eq!(drill().crafts_per_second(4.0), 0.5);
    }
}
