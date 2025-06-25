use std::collections::HashMap;

use crate::{Robot, Map, Station};
use crate::Config;
use std::sync::mpsc::Sender;


pub struct GameState {
    pub robot: Robot,
    pub map: Map,
    pub station: Station,
    pub last_collect_message: Option<String>,
    pub resources_revealed: bool,
    pub sender_to_earth : Sender<String>,
}

impl GameState {
    pub fn new(config: &Config, sender_to_earth : Sender<String>) -> Self {
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