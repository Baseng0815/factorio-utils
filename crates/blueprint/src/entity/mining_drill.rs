use recipes::ItemId;

use super::ItemStack;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MiningDrill {
    pub modules: Vec<ItemStack>,
    pub resource_filter: Option<ItemId>,
}

impl MiningDrill {
    pub fn with_module(mut self, module: impl Into<ItemId>, count: u32) -> Self {
        self.modules.push(ItemStack {
            item: module.into(),
            count,
        });
        self
    }

    pub fn with_resource_filter(mut self, resource: impl Into<ItemId>) -> Self {
        self.resource_filter = Some(resource.into());
        self
    }
}
