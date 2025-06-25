use colored::*;
use super::cell::Cell;
use crate::robot::robot::Robot;
use crate::station::station::Station;
use crate::utils::noise::generate_noise;

pub struct Map {
  pub width: usize,
  pub height: usize,
  pub grid: Vec<Vec<Cell>>,
}

impl Map {
  pub fn new(width: usize, height: usize, seed: u32) -> Self {
    let mut grid = generate_noise(width, height, seed);

    for x in 0..width {
      grid[0][x] = Cell::Obstacle;
      grid[height - 1][x] = Cell::Obstacle;
    }

    for y in 0..height {
      grid[y][0] = Cell::Obstacle;
      grid[y][width - 1] = Cell::Obstacle;
    }

    Self {
      width,
      height,
      grid,
    }
  }

  pub fn print(&self, robot: &Robot, station: &Station, resources_revealed: bool) {
   // println!("Map size: {}x{}", self.width, self.height);

    for (y, row) in self.grid.iter().enumerate() {
      for (x, cell) in row.iter().enumerate() {
        if x == robot.x && y == robot.y {
          print!("{}", "🤖".green().bold());
        } else if x == station.x && y == station.y {
          print!("{}", "🏭".yellow().bold());
        } else {
          let symbol = match cell {
            Cell::Wall | Cell::Obstacle => "██".bright_black(),
            Cell::Empty => "  ".white(),
            Cell::Science => "🧪".purple().bold(),
            Cell::Mineral => {
              if resources_revealed {
                "💎".blue().bold()
              } else {
                "❓".red().bold()
              }
            }
            Cell::Energy => {
              if resources_revealed {
                "⚡".yellow().bold()
              } else {
                "❓".red().bold()
              }
            }
          };
          print!("{}", symbol);
        }
      }
      println!();
    }
  }
}
