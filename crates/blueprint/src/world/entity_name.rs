use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use prototypes::MachineId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EntityName(Cow<'static, str>);

impl EntityName {
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

impl From<String> for EntityName {
    fn from(s: String) -> Self {
        Self(Cow::Owned(s))
    }
}

impl From<&str> for EntityName {
    fn from(s: &str) -> Self {
        Self(Cow::Owned(s.to_owned()))
    }
}

impl From<MachineId> for EntityName {
    fn from(id: MachineId) -> Self {
        Self(Cow::Owned(id.as_str().to_owned()))
    }
}

impl From<EntityName> for MachineId {
    fn from(name: EntityName) -> Self {
        MachineId::from(name.as_str())
    }
}

impl std::fmt::Display for EntityName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn machine_id_round_trips() {
        let m = MachineId::from("assembling-machine-2");
        let n = EntityName::from(m.clone());
        assert_eq!(n.as_str(), "assembling-machine-2");
        let back: MachineId = n.into();
        assert_eq!(back.as_str(), m.as_str());
    }
}
