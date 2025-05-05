pub struct Config {
    pub(crate) width: usize,
    pub(crate) anchor_prefix: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 80,
            anchor_prefix: String::new(),
        }
    }
}

impl Config {
    pub fn with_width(self, width: usize) -> Self {
        Self { width, ..self }
    }

    pub fn with_anchor_prefix(self, anchor_prefix: String) -> Self {
        Self {
            anchor_prefix,
            ..self
        }
    }
}
