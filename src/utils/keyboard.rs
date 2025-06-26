use crate::GameState;
use crossterm::{
  event::{self, Event, KeyCode, KeyEventKind},
  terminal::{disable_raw_mode},
};
use std::time::Duration;

pub fn handle_keyboard_events(
  state: &mut GameState,
  automation_enabled: &mut bool,
) -> Result<bool, Box<dyn std::error::Error>> {
  if event::poll(Duration::from_millis(200))? {
    if let Event::Key(key_event) = event::read()? {
      if key_event.kind == KeyEventKind::Press {
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
            state.robots[0].try_move(
              0,
              -1,
              &state.map,
              state.resources_revealed,
              &other_robots,
              &state.station,
            );
          }
          KeyCode::Down => {
            state.robots[0].try_move(
              0,
              1,
              &state.map,
              state.resources_revealed,
              &other_robots,
              &state.station,
            );
          }
          KeyCode::Left => {
            state.robots[0].try_move(
              -1,
              0,
              &state.map,
              state.resources_revealed,
              &other_robots,
              &state.station,
            );
          }
          KeyCode::Right => {
            state.robots[0].try_move(
              1,
              0,
              &state.map,
              state.resources_revealed,
              &other_robots,
              &state.station,
            );
          }
          KeyCode::Char('a' | 'A') => {
            *automation_enabled = !*automation_enabled;
          }
          KeyCode::Esc => {
            disable_raw_mode()?;
            println!("Arrêt du programme.");
            return Ok(true);
          }
          _ => {}
        }
      }
    }
  }
  Ok(false)
}
