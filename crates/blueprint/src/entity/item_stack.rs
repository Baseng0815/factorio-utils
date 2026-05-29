use recipes::ItemId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemStack {
    pub item: ItemId,
    pub count: u32,
}

impl ItemStack {
    pub fn new(item: impl Into<ItemId>, count: u32) -> Self {
        Self {
            item: item.into(),
            count,
        }
    }
}

impl std::fmt::Display for ItemStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} × {}", self.count, self.item)
    }
}
