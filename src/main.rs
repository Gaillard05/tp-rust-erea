mod map;
mod utils;
mod config;
mod robot;
mod station;

use map::map::Map;
use std::collections::HashMap;
use std::io::{self, Write};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{enable_raw_mode, disable_raw_mode},
};
use config::Config;
use robot::robot::{Robot, ResourceType};
use utils::display::{print_station_inventory, print_commands_and_indicators};
use crate::station::station::Station;


fn main() -> std::io::Result<()> {
    let config = Config::default();
    let mut robot = Robot { x: 8, y: 4, inventory: HashMap::new()};
    let mut map = Map::new(config.width, config.height, config.seed);
    let mut station = Station { x: 9, y: 4, inventory: HashMap::new() };
    let mut last_collect_message: Option<String> = None;

    loop {
        disable_raw_mode().ok(); // Désactive pour éviter de décaler sur mac

        clearscreen::clear().unwrap();
        map.print(&robot, &station);

        print_commands_and_indicators();
        print_station_inventory(&station);

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

