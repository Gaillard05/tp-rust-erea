pub struct Config {
    pub width: usize,
    pub heigth: usize,
    pub seed: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 50,
            heigth: 15,
            seed: 42,
        }
    }
}