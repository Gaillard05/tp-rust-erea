mod config;
mod map;
mod robot;
mod station;
mod utils;
mod game;

use crate::station::station::Station;
use config::Config;
use crate::game::game_loop::run_game_loop;
use crate::game::game_state::GameState;
use map::map::Map;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let config = Config::default();
  let game_state = GameState::new(&config);
  run_game_loop(game_state)
}
