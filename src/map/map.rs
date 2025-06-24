use super::cell::Cell;
use crate::robot::robot::Robot;
use crate::station::station::Station;
use crate::utils::noise::generate_noise;
use colored::*;

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

  pub fn print(&self, robot: &Robot, station: &Station) {
    println!("Map size: {}x{}", self.width, self.height);

    for (y, row) in self.grid.iter().enumerate() {
      for (x, cell) in row.iter().enumerate() {
        if x == robot.x && y == robot.y {
          print!("{}", "🤖".green().bold());
        } else if x == station.x && y == station.y {
          print!("{}", "🏭".yellow().bold()); // Station en jaune
        } else {
          let symbol = match cell {
            Cell::Wall | Cell::Obstacle => "██".bright_black(),
            Cell::Empty => "  ".white(),
            Cell::Mineral => "💎".blue().bold(),
            Cell::Energy => "⚡".yellow().bold(),
            Cell::Science => "🧪".purple().bold(),
          };
          print!("{}", symbol);
        }
      }
      println!();
    }
  }
}
