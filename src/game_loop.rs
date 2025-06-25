use clearscreen;

use crate::GameState;
use crate::utils::display::{print_commands_and_indicators, print_inventories};
use crossterm::{
  event::{self, Event, KeyCode, KeyEventKind},
  terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};

pub fn run_game_loop(mut state: GameState) -> Result<(), Box<dyn std::error::Error>> {
  enable_raw_mode()?;

  loop {
    disable_raw_mode().ok();
    clearscreen::clear()?;

    state
      .map
      .print(&state.robot, &state.station, state.resources_revealed);
    print_commands_and_indicators();
    print_inventories(&state.station, &state.robot);

    if let Some(msg) = &state.last_collect_message {
      println!("{msg}");
    }

    enable_raw_mode()?;

    if let Event::Key(key_event) = event::read()? {
      if key_event.kind == KeyEventKind::Press {
        match key_event.code {
          KeyCode::Up => state
            .robot
            .try_move(0, -1, &state.map, state.resources_revealed),
          KeyCode::Down => state
            .robot
            .try_move(0, 1, &state.map, state.resources_revealed),
          KeyCode::Left => state
            .robot
            .try_move(-1, 0, &state.map, state.resources_revealed),
          KeyCode::Right => state
            .robot
            .try_move(1, 0, &state.map, state.resources_revealed),
          KeyCode::Char('u' | 'U')
            if state.robot.x == state.station.x && state.robot.y == state.station.y =>
          {
            let _science_deposited = state
              .robot
              .unload_resources(&mut state.station, &mut state.map);

            io::stdout().flush()?;
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
    state.last_collect_message = state
      .robot
      .collect_resource(&mut state.map, state.resources_revealed);
  }
}
