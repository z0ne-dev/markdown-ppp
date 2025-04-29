pub struct Config {
    pub(crate) width: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self { width: 80 }
    }
}

impl Config {
    pub fn with_width(width: usize) -> Self {
        Self { width }
    }
}
