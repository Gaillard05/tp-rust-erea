use std::collections::HashMap;

use crate::Config;
use crate::{ Map, Robot, Station };

pub struct GameState {
    pub robot: Robot,
    pub map: Map,
    pub station: Station,
    pub last_collect_message: Option<String>,
    pub resources_revealed: bool,
    pub robot_speed_ms: u64,
}

impl GameState {
    pub fn new(config: &Config) -> Self {
        Self {
            robot: Robot {
                x: 8,
                y: 4,
                inventory: HashMap::new(),
                inventory_capacity: 5,
                collected_science_positions: Vec::new(),
            },
            map: Map::new(config.width, config.height, config.seed),
            station: Station {
                x: 9,
                y: 4,
                inventory: HashMap::new(),
            },
            last_collect_message: None,
            resources_revealed: false,
            robot_speed_ms: 250, // Vitesse initiale en ms
        }
    }
}
