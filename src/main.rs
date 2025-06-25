mod config;
mod game_loop;
mod game_state;
mod map;
mod robot;
mod station;
mod utils;

use crate::station::station::Station;
use config::Config;
use game_loop::run_game_loop;
use game_state::GameState;
use map::map::Map;
use robot::robot::Robot;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let config = Config::default();
  let game_state = GameState::new(&config);
  run_game_loop(game_state)
}
