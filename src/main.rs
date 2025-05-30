mod map;
mod utils;
mod config;

use map::map::Map;
use config::Config;

fn main() {
    let config = Config::default();
    let map = Map::new(config.width, config.heigth, config.seed);
    map.print();
}
