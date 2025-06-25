use std::collections::HashMap;

use crate::Config;
use crate::robot::robot::RobotType;
use crate::{Map, Robot, Station};

pub struct GameState {
  pub robots: Vec<Robot>,
  pub map: Map,
  pub station: Station,
  pub last_collect_message: Option<String>,
  pub resources_revealed: bool,
  pub robot_speed_ms: u64,
}

impl GameState {
  pub fn new(config: &Config) -> Self {
    let num_collectors = 5;
    let num_explorators = 1;

    let mut robots = Vec::new();

    for i in 0..num_collectors {
      robots.push(Robot {
        x: if i == 0 { 8 } else { 7 },
        y: 4,
        inventory: HashMap::new(),
        inventory_capacity: 5,
        collected_science_positions: Vec::new(),
        robots: Vec::new(),
        robot_type: RobotType::Collector,
      });
    }

    for i in 0..num_explorators {
      robots.push(Robot {
        x: 7,
        y: 4,
        inventory: HashMap::new(),
        inventory_capacity: 5,
        collected_science_positions: Vec::new(),
        robots: Vec::new(),
        robot_type: RobotType::Explorator,
      });
    }

    Self {
      robots,
      map: Map::new(config.width, config.height, config.seed),
      station: Station {
        x: 9,
        y: 4,
        inventory: HashMap::new(),
      },
      last_collect_message: None,
      resources_revealed: false,
      robot_speed_ms: 250,
    }
  }
}
