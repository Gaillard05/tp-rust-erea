pub mod config;
pub mod map;
pub mod robot;
pub mod station;
pub mod utils;
pub mod game_state;

mod game_loop;

use config::Config;
//use map::map::Map;
use game_state::GameState;
use robot::robot::Robot;
use game_loop::run_game_loop;
//use station::station::Station;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();
    let game_state = GameState::new(&config);
    run_game_loop(game_state)
}
