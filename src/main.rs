mod config;
mod map;
mod utils;

use crate::map::map::Map;

fn main() {
    let width = 20;
    let height = 20;
    let seed = 42;

    let map = Map::new(width, height, seed);

    map.print();
}
