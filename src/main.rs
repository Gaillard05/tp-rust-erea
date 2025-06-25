mod config;
mod map;
mod robot;
mod station;
mod utils;

use crate::station::station::Station;
use config::Config;
use crossterm::{
  event::{self, Event, KeyCode, KeyEventKind},
  terminal::{disable_raw_mode, enable_raw_mode},
};
use map::map::Map;
use robot::robot::{Robot};
use std::collections::HashMap;
use std::io::Write;
use utils::display::{print_commands_and_indicators, print_inventories};

fn main() -> std::io::Result<()> {
  let config = Config::default();

  let mut robots = Vec::new();
  let mut active_robot_index = 0;

 
    let robot_type = Robot::choice_type_robot(0);

    let first_robot = Robot {
      x: 8,
      y: 4,
      inventory: HashMap::new(),
      inventory_capacity: 5,
      robot_type,
    };
  
    first_robot.describe();
    robots.push(first_robot);
  

  let mut map = Map::new(config.width, config.height, config.seed);
  let mut station = Station {
    x: 9,
    y: 4,
    inventory: HashMap::new(),
  };
  let mut _last_collect_message: Option<String> = None;
  let mut resources_revealed = false;
  let mut _status_message: Option<String> = None;

  loop {
    disable_raw_mode().ok(); // Désactive pour éviter de décaler sur mac

    clearscreen::clear().unwrap();
    let active_robot = &robots[active_robot_index];
    map.print(&robots, &station, resources_revealed);

    print_commands_and_indicators();
    print_inventories(&station, active_robot);

    if let Some(msg) = &_status_message {
      println!("{msg}");
    }

    enable_raw_mode()?; // Active pour permettre utiliser les touches sans entrée

    if let Event::Key(key_event) = event::read()? {
      if key_event.kind == KeyEventKind::Press {
        let robot = &mut robots[active_robot_index];
        match key_event.code {
          KeyCode::Up => robot.try_move(0, -1, &map),
          KeyCode::Down => robot.try_move(0, 1, &map),
          KeyCode::Left => robot.try_move(-1, 0, &map),
          KeyCode::Right => robot.try_move(1, 0, &map),
          KeyCode::Char('u' | 'U') if robot.x == station.x && robot.y == station.y => {
            println!("Inventaire robot: {:?}", robot.inventory);
            let science_deposited = robot.unload_resources(&mut station);
              if science_deposited {
                resources_revealed = true;
              }
              println!("Inventaire station: {:?}", station.inventory);
              
              disable_raw_mode()?;
              let new_type = Robot::choice_type_robot(active_robot_index);
              enable_raw_mode()?;

              robot.robot_type = new_type;
              robot.describe();
              _status_message = Some("Type du robot mis à jour après déchargement.".to_string());


            std::io::stdout().flush()?;
          }

          KeyCode::Tab => {
            active_robot_index = (active_robot_index + 1) % robots.len();
            _status_message = Some(format!("Changement vers robot #{}", active_robot_index + 1));
          }

          KeyCode::Esc => {
            disable_raw_mode()?;
            println!("Arrêt du programme.");
            return Ok(());
          }
          _ => {}
        }
      }
    }

    let robot = &mut robots[active_robot_index];
    _last_collect_message = robot.collect_resource(&mut map, resources_revealed);
  }
}
