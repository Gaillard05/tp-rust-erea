use crate::GameState;
use crate::game::game_automation::automate_all_robots;
use crate::utils::display::{print_commands_and_indicators, print_map_stats};
use crate::utils::keyboard::handle_keyboard_events;
use clearscreen;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

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

    if handle_keyboard_events(&mut state, &mut automation_enabled)? {
      break; // Quitte la boucle principale si Esc pressé
    }

    automate_all_robots(&mut state, automation_enabled);

    std::thread::sleep(std::time::Duration::from_millis(state.robot_speed_ms));
  }

  Ok(())
}
