mod config;
mod map;
mod robot;
mod station;
mod utils;

use crate::map::map::Map;
use crate::{map::cell::Cell, robot::robot::Robot};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use std::{io, time::Duration};
use text_io::read;

fn main() -> Result<(), io::Error> {
    let width = 20;
    let height = 20;
    let seed = 42;

    let map = Map::new(width, height, seed);

    // robot position
    let robot_x = 10;
    let robot_y = 10;

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // main loop
    loop {
        terminal.draw(|f| {
            let size = f.size();

            let block = Block::default()
                .title("Map (Q to quit)")
                .borders(Borders::ALL);
            f.render_widget(block, size);

            let lines: Vec<Line> = map
                .grid
                .iter()
                .enumerate()
                .map(|(j, row)| {
                    let row_str = row
                        .iter()
                        .enumerate()
                        .map(|(i, cell)| {
                            if i == robot_x && j == robot_y {
                                "R " // Robot here !
                            } else {
                                match cell {
                                    Cell::Wall => "##",
                                    Cell::Obstacle => "##",
                                    Cell::Empty => ". ",
                                    Cell::Energy => "E ",
                                    Cell::Mineral => "M ",
                                    Cell::Science => "S ",
                                }
                            }
                        })
                        .collect::<String>();

                    Line::from(Span::raw(row_str))
                })
                .collect();

            let paragraph = Paragraph::new(lines).block(Block::default().borders(Borders::NONE));
            f.render_widget(paragraph, size);
        })?;

        if crossterm::event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn move_robot(map: &Map, robot: &mut Robot) {
    println!("Déplace le robot (z=haut, s=bas, q=gauche, d=droite, x=quitter) : ");
    let cmd: String = read!();

    let (dx, dy) = match cmd.as_str() {
        "z" => (0, -1),
        "s" => (0, 1),
        "q" => (-1, 0),
        "d" => (1, 0),
        "x" => {
            println!("Arrêt du programme.");
            std::process::exit(0);
        }
        _ => {
            println!("Commande inconnue.");
            return;
        }
    };

    let new_x = robot.x as isize + dx;
    let new_y = robot.y as isize + dy;

    if new_x >= 0
        && new_y >= 0
        && (new_x as usize) < map.width
        && (new_y as usize) < map.height
        && map.grid[new_y as usize][new_x as usize] != Cell::Wall
        && map.grid[new_y as usize][new_x as usize] != Cell::Obstacle
    {
        robot.x = new_x as usize;
        robot.y = new_y as usize;
    } else {
        println!("Déplacement impossible !");
    }
}
