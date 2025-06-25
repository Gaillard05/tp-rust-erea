use crate::GameState;
use crate::Robot;
use crate::utils::display::{print_commands_and_indicators, print_inventories, print_map_stats};
use clearscreen;
use crossterm::{
  event::{self, Event, KeyCode, KeyEventKind},
  terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};
use std::time::Duration;

pub fn run_game_loop(mut state: GameState) -> Result<(), Box<dyn std::error::Error>> {
  enable_raw_mode()?;
  let mut automation_enabled = true;

  loop {
    disable_raw_mode().ok();
    clearscreen::clear()?;

    state
      .map
      .print_map(&state.robots, &state.station, state.resources_revealed);
    print_commands_and_indicators();
    // print_inventories(&state.station, &state.robot);
    print_map_stats(&state.map, state.robot_speed_ms);

    println!(
      "Automation: {}",
      if automation_enabled { "ON" } else { "OFF" }
    );

    if let Some(msg) = &state.last_collect_message {
      println!("{msg}");
    }

    enable_raw_mode()?;
    // let mut manual_move = false;

    if event::poll(Duration::from_millis(200))? {
      if let Event::Key(key_event) = event::read()? {
        if key_event.kind == KeyEventKind::Press {
          match key_event.code {
            // KeyCode::Up => {
            //     state.robot.try_move(0, -1, &state.map, state.resources_revealed);
            //     manual_move = true;
            // }
            // KeyCode::Down => {
            //     state.robot.try_move(0, 1, &state.map, state.resources_revealed);
            //     manual_move = true;
            // }
            // KeyCode::Left => {
            //     state.robot.try_move(-1, 0, &state.map, state.resources_revealed);
            //     manual_move = true;
            // }
            // KeyCode::Right => {
            //     state.robot.try_move(1, 0, &state.map, state.resources_revealed);
            //     manual_move = true;
            // }
            // KeyCode::Char('u' | 'U') if
            //     state.robot.x == state.station.x &&
            //     state.robot.y == state.station.y
            // => {
            //     let science_deposited = state.robot.unload_resources(
            //         &mut state.station,
            //         &mut state.map
            //     );
            //     if science_deposited && state.robot_speed_ms > 30 {
            //         state.robot_speed_ms -= 50; // Réduit de 50ms à chaque science déposée
            //     }
            //     io::stdout().flush()?;
            // }
            KeyCode::Char('a' | 'A') => {
              automation_enabled = !automation_enabled;
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
    }

    // Capture d'abord les positions des robots de manière immuable
    let robot_positions: Vec<(usize, usize)> = state.robots.iter().map(|r| (r.x, r.y)).collect();

    for (i, robot) in state.robots.iter_mut().enumerate() {
      if automation_enabled {
        // Crée une liste des autres robots (exclut le robot courant)
        let other_robots: Vec<(usize, usize)> = robot_positions
          .iter()
          .enumerate()
          .filter(|(j, _)| *j != i)
          .map(|(_, pos)| *pos)
          .collect();

        Robot::automate_robot(
          robot,
          &state.map,
          &state.station,
          state.resources_revealed,
          &other_robots,
        );
      }

      // Déchargement automatique à la station
      if robot.x == state.station.x && robot.y == state.station.y {
        let science_deposited = robot.unload_resources(&mut state.station, &mut state.map);
        if science_deposited {
          if let Some(msg) = state.map.unlock_zone_with_science((robot.x, robot.y)) {
            println!("{msg}");
          }
          if state.robot_speed_ms > 30 {
            state.robot_speed_ms -= 10;
          }
        }
      }

      state.last_collect_message = robot.collect_resource(&mut state.map, state.resources_revealed);
    }

    std::thread::sleep(std::time::Duration::from_millis(state.robot_speed_ms));
  }
}
