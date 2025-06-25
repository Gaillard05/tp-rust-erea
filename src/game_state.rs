use std::collections::HashMap;

use crate::robot::robot::RobotTypes;
use crate::Config;
use crate::{Map, Robot, Station};

pub struct GameState {
  pub robot: Robot,
  pub map: Map,
  pub station: Station,
  pub last_collect_message: Option<String>,
  pub resources_revealed: bool,
  pub status_message: Option<String>,
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
        robot_type: RobotTypes::Explorateur,
      },
      map: Map::new(config.width, config.height, config.seed),
      station: Station {
        x: 9,
        y: 4,
        inventory: HashMap::new(),
      },
      last_collect_message: None,
      resources_revealed: false,
      status_message: None
    }
  }
}
