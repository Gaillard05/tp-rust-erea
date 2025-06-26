use crate::GameState;
use crate::Robot;
use crate::utils::display::{print_commands_and_indicators, print_map_stats};
use clearscreen;
use crossterm::{
  event::{self, Event, KeyCode, KeyEventKind},
  terminal::{disable_raw_mode, enable_raw_mode},
};
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
    print_map_stats(&state.map, state.robot_speed_ms, &state.station);

    println!(
      "Automation: {}",
      if automation_enabled { "ON" } else { "OFF" }
    );

    if let Some(msg) = &state.last_collect_message {
      println!("{msg}");
    }

    enable_raw_mode()?;

    if event::poll(Duration::from_millis(200))? {
      if let Event::Key(key_event) = event::read()? {
        if key_event.kind == KeyEventKind::Press {
          // Capture robot positions once for all movements
          let robot_positions: Vec<(usize, usize)> =
            state.robots.iter().map(|r| (r.x, r.y)).collect();
          let other_robots: Vec<(usize, usize)> = robot_positions
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != 0)
            .map(|(_, pos)| *pos)
            .collect();

          match key_event.code {
            KeyCode::Up => {
              state.robots[0].try_move(0, -1, &state.map, state.resources_revealed, &other_robots,&state.station);
            }
            KeyCode::Down => {
              state.robots[0].try_move(0, 1, &state.map, state.resources_revealed, &other_robots, &state.station);
            }
            KeyCode::Left => {
              state.robots[0].try_move(-1, 0, &state.map, state.resources_revealed, &other_robots, &state.station);
            }
            KeyCode::Right => {
              state.robots[0].try_move(1, 0, &state.map, state.resources_revealed, &other_robots, &state.station);
            }
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
      let other_robots: Vec<(usize, usize)> = robot_positions
        .iter()
        .enumerate()
        .filter(|(j, _)| *j != i)
        .map(|(_, pos)| *pos)
        .collect();

      if automation_enabled {
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
            state.robot_speed_ms -= 50;
          }
        }
      }

      state.last_collect_message = robot.collect_resource(&mut state.map, state.resources_revealed);
    }

    std::thread::sleep(std::time::Duration::from_millis(state.robot_speed_ms));
  }
}
