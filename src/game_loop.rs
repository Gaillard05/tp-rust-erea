use clearscreen;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{enable_raw_mode, disable_raw_mode}
};
use std::io::{self, Write};
use crate::{GameState};
use crate::utils::display::{print_commands_and_indicators, print_inventories};

pub fn run_game_loop(mut state: GameState) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;

    loop {
        disable_raw_mode().ok();
        clearscreen::clear()?;
        
        // Affichage
        state.map.print(&state.robot, &state.station, state.resources_revealed);
        print_commands_and_indicators();
        print_inventories(&state.station, &state.robot);
        
        if let Some(msg) = &state.last_collect_message {
            println!("{msg}");
        }

        enable_raw_mode()?;

        // Gestion des entrées
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Up => state.robot.try_move(0, -1, &state.map),
                    KeyCode::Down => state.robot.try_move(0, 1, &state.map),
                    KeyCode::Left => state.robot.try_move(-1, 0, &state.map),
                    KeyCode::Right => state.robot.try_move(1, 0, &state.map),
                    KeyCode::Char('u' | 'U') if state.robot.x == state.station.x && state.robot.y == state.station.y => {
                        let science_deposited = state.robot.unload_resources(&mut state.station);
                        if science_deposited {
                            state.resources_revealed = true;
                        }
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

        // Collecte des ressources
        state.last_collect_message = state.robot.collect_resource(&mut state.map, state.resources_revealed);
    }
}
