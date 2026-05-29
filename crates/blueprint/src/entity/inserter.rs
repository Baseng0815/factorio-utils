use prototypes::ItemId;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Inserter {
    pub filters: Vec<ItemId>,
    pub override_stack_size: Option<u32>,
}

impl Inserter {
    pub fn with_filter(mut self, item: impl Into<ItemId>) -> Self {
        self.filters.push(item.into());
        self
    }

    pub fn with_override_stack_size(mut self, size: u32) -> Self {
        self.override_stack_size = Some(size);
        self
    }
}
