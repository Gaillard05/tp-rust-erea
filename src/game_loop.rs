use clearscreen;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{enable_raw_mode, disable_raw_mode}
};
use std::{io::{self, Write}, thread, time::Duration};

use crate::GameState;
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

        // 🔁 Si le mode est automatique : agir sans clavier
        if state.autonomous_mode {
            state.robot.act_random(&mut state.map, &state.station, state.resources_revealed);
            state.last_collect_message = None;

            // Déchargement automatique
            if state.robot.x == state.station.x && state.robot.y == state.station.y {
                let deposited = state.robot.unload_resources(&mut state.station);
                if deposited {
                    state.resources_revealed = true;
                }
            }

            thread::sleep(Duration::from_millis(200));
            continue;
        }

        // 🎮 Sinon : mode manuel (attente touche)
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Up => state.robot.try_move(0, -1, &state.map),
                        KeyCode::Down => state.robot.try_move(0, 1, &state.map),
                        KeyCode::Left => state.robot.try_move(-1, 0, &state.map),
                        KeyCode::Right => state.robot.try_move(1, 0, &state.map),
                        KeyCode::Char('u') | KeyCode::Char('U')
                        if state.robot.x == state.station.x && state.robot.y == state.station.y =>
                            {
                                let deposited = state.robot.unload_resources(&mut state.station);
                                if deposited {
                                    state.resources_revealed = true;
                                }
                                io::stdout().flush()?;
                            }
                        KeyCode::Char('a') => {
                            state.autonomous_mode = true;
                            println!("Mode autonome activé.");
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
    }
}
