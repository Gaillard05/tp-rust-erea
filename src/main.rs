mod map;
mod utils;
mod config;
mod robot;
mod station;

use map::map::Map;
use std::collections::HashMap;
use config::Config;
use robot::robot::Robot;
use crate::station::station::Station;

fn main() -> std::io::Result<()> {
    let config = Config::default();
    let mut robot = Robot { x: 8, y: 4, inventory: HashMap::new()};
    let mut map = Map::new(config.width, config.heigth, config.seed);
    let mut last_collect_message: Option<String> = None;
    let station = Station { x: 9, y: 4 };

    loop {
        clearscreen::clear().unwrap();
        map.print(&robot, &station); 

        if let Some(msg) = &last_collect_message {
            println!("{}", msg);
        }

        for (res, qty) in &robot.inventory {
            println!("{:?} : {}", res, qty);
        }

        robot.move_robot(&map)?; 
        last_collect_message = robot.collect_resource(&mut map); 
    }
}

