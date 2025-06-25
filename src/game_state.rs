use std::collections::HashMap;

//use crate::{Robot, Map, Station};
//use crate::Config;
use crate::robot::robot::Robot;
use crate::map::map::Map;
use crate::station::station::Station;
use crate::config::Config;


pub struct GameState {
    pub robot: Robot,
    pub map: Map,
    pub station: Station,
    pub last_collect_message: Option<String>,
    pub resources_revealed: bool,
}

impl GameState {
    pub fn new(config: &Config) -> Self {
        Self {
            robot: Robot {
                x: 8,
                y: 4,
                inventory: HashMap::new(),
                inventory_capacity: 5,
            },
            map: Map::new(config.width, config.height, config.seed),
            station: Station {
                x: 9,
                y: 4,
                inventory: HashMap::new(),
            },
            last_collect_message: None,
            resources_revealed: false,
        }
    }
}