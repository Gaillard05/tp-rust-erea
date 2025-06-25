use crate::GameState;
use crate::Robot;
use crate::utils::display::{ print_commands_and_indicators, print_inventories, print_map_stats };
use clearscreen;
use crossterm::{
    event::{ self, Event, KeyCode, KeyEventKind },
    terminal::{ disable_raw_mode, enable_raw_mode },
};
use std::io::{ self, Write };
use std::time::Duration;

pub fn run_game_loop(mut state: GameState) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut automation_enabled = true;

    loop {
        disable_raw_mode().ok();
        clearscreen::clear()?;

        state.map.print(&state.robot, &state.station, state.resources_revealed);
        print_commands_and_indicators();
        print_inventories(&state.station, &state.robot);
        print_map_stats(&state.map);

        println!("Automation: {}", if automation_enabled { "ON" } else { "OFF" });

        if let Some(msg) = &state.last_collect_message {
            println!("{msg}");
        }

        enable_raw_mode()?;
        let mut manual_move = false;

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
                            let _science_deposited = state.robot.unload_resources(
                                &mut state.station,
                                &mut state.map
                            );
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

        if automation_enabled && !manual_move {
            Robot::automate_robot(
                &mut state.robot,
                &state.map,
                &state.station,
                state.resources_revealed
            );
            if
                state.robot.x == state.station.x &&
                state.robot.y == state.station.y &&
                !state.robot.inventory.is_empty()
            {
                let _science_deposited = state.robot.unload_resources(
                    &mut state.station,
                    &mut state.map
                );
                if state.robot_speed_ms > 30 {
                    state.robot_speed_ms -= 50; // Réduit de 10ms à chaque science
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(state.robot_speed_ms));
        state.last_collect_message = state.robot.collect_resource(
            &mut state.map,
            state.resources_revealed
        );
    }
}
