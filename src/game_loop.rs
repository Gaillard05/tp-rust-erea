use clearscreen;
use crossterm::{
    event::{ self, Event, KeyCode, KeyEventKind },
    terminal::{ enable_raw_mode, disable_raw_mode },
};
use std::io::{ self, Write };
//use std::thread;
use std::time::Duration;
use crate::{ GameState };
use crate::Robot;
use crate::utils::display::{ print_commands_and_indicators, print_inventories };
//use crate::map::map::Map;



pub fn run_game_loop(mut state: GameState) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut automation_enabled = true;

    loop {
        disable_raw_mode().ok();
        clearscreen::clear()?;
        state.map.print(&state.robot, &state.station, state.resources_revealed);
        print_commands_and_indicators();
        print_inventories(&state.station, &state.robot);
        println!("Automation: {}", if automation_enabled { "ON" } else { "OFF" });
        println!("Press 'a' to toggle automation");

        if let Some(msg) = &state.last_collect_message {
            println!("{msg}");
        }

        enable_raw_mode()?;

        let mut manual_move = false;

        // Non bloquant : attend une touche pendant 200ms, sinon continue
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Up => {
                            state.robot.try_move(0, -1, &state.map, state.resources_revealed);
                            manual_move = true;
                        }
                        KeyCode::Down => {
                            state.robot.try_move(0, 1, &state.map, state.resources_revealed);
                            manual_move = true;
                        }
                        KeyCode::Left => {
                            state.robot.try_move(-1, 0, &state.map, state.resources_revealed);
                            manual_move = true;
                        }
                        KeyCode::Right => {
                            state.robot.try_move(1, 0, &state.map, state.resources_revealed);
                            manual_move = true;
                        }
                        KeyCode::Char('u' | 'U') if
                            state.robot.x == state.station.x &&
                            state.robot.y == state.station.y
                        => {
                            let science_deposited = state.robot.unload_resources(&mut state.station, &mut state.map);
                            if science_deposited {
                                state.resources_revealed = true;
                            }
                            io::stdout().flush()?;
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

        // Automatisation si activée et pas de move manuel ce tour-ci
        if automation_enabled && !manual_move {
            Robot::automate_robot(
                &mut state.robot,
                &state.map,
                &state.station,
                state.resources_revealed
            );
            if state.robot.x == state.station.x && state.robot.y == state.station.y {
                let science_deposited = state.robot.unload_resources(&mut state.station, &mut state.map);

                if science_deposited {
                    state.resources_revealed = true;
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
        state.last_collect_message = state.robot.collect_resource(
            &mut state.map,
            state.resources_revealed
        );
    }
}
