use std::time::Duration;

#[derive(Debug, Clone)]
pub struct LayoutConfig {
    pub width: u32,
    pub height: u32,
    pub timeout: Option<Duration>,
}

impl LayoutConfig {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            timeout: None,
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}
