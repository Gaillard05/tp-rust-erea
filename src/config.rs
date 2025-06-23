pub struct Config {
    pub width: usize,
    pub height: usize,
    pub seed: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 50,
            height: 15,
            seed: 42,
        }
    }
}