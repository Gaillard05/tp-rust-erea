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
use robot::robot::{ResourceType, Robot};
use std::collections::HashMap;
use std::io::Write;
use utils::display::{print_commands_and_indicators, print_inventories};

fn main() -> std::io::Result<()> {
  let config = Config::default();
  let mut robot = Robot {
    x: 8,
    y: 4,
    inventory: HashMap::new(),
    inventory_capacity: 5,
  };
  let mut map = Map::new(config.width, config.height, config.seed);
  let mut station = Station {
    x: 9,
    y: 4,
    inventory: HashMap::new(),
  };
  let mut last_collect_message: Option<String> = None;

  loop {
    disable_raw_mode().ok(); // Désactive pour éviter de décaler sur mac

    clearscreen::clear().unwrap();
    map.print(&robot, &station);

    print_commands_and_indicators();
    print_inventories(&station, &robot);

    if let Some(msg) = &last_collect_message {
      println!("{msg}");
    }
    for (res, qty) in &robot.inventory {
      let icon = match res {
        ResourceType::Mineral => "💎",
        ResourceType::Energy => "⚡",
        ResourceType::Science => "🧪",
      };
      println!("{} : {}", icon, qty);
    }

    enable_raw_mode()?; // Active pour permettre utiliser les touches sans entrée

    if let Event::Key(key_event) = event::read()? {
      if key_event.kind == KeyEventKind::Press {
        match key_event.code {
          KeyCode::Up => robot.try_move(0, -1, &map),
          KeyCode::Down => robot.try_move(0, 1, &map),
          KeyCode::Left => robot.try_move(-1, 0, &map),
          KeyCode::Right => robot.try_move(1, 0, &map),
          KeyCode::Char('u' | 'U') if robot.x == station.x && robot.y == station.y => {
            println!("Inventaire robot: {:?}", robot.inventory);
            robot.unload_resources(&mut station);
            println!("Inventaire station: {:?}", station.inventory);
            std::io::stdout().flush()?;
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

    last_collect_message = robot.collect_resource(&mut map);
	
  }
}
