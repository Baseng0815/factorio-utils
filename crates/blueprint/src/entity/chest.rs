#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Chest {
    pub bar: Option<u32>,
}

impl Chest {
    pub fn with_bar(mut self, bar: u32) -> Self {
        self.bar = Some(bar);
        self
    }
}
