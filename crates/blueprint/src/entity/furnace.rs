use recipes::ItemId;

use super::ItemStack;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Furnace {
    pub modules: Vec<ItemStack>,
}

impl Furnace {
    pub fn with_module(mut self, module: impl Into<ItemId>, count: u32) -> Self {
        self.modules.push(ItemStack {
            item: module.into(),
            count,
        });
        self
    }
}
