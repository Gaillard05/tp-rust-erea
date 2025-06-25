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
    println!("Map size: {}x{}", self.width, self.height);

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


#[cfg(test)]
mod tests {
  use super::*;
  use crate::map::cell::Cell;

  #[test]
  fn map_is_created_with_correct_dimensions() {
    let width = 10;
    let height = 8;
    let seed = 42;

    let map = Map::new(width, height, seed);

    assert_eq!(map.width, width);
    assert_eq!(map.height, height);
    assert_eq!(map.grid.len(), height);
    assert_eq!(map.grid[0].len(), width);
  }

  #[test]
  fn map_borders_are_obstacles() {
    let width = 6;
    let height = 6;
    let seed = 123;

    let map = Map::new(width, height, seed);

    // Vérifie les bords haut et bas
    for x in 0..width {
      assert_eq!(map.grid[0][x], Cell::Obstacle); // haut
      assert_eq!(map.grid[height - 1][x], Cell::Obstacle); // bas
    }

    // Vérifie les bords gauche et droite
    for y in 0..height {
      assert_eq!(map.grid[y][0], Cell::Obstacle); // gauche
      assert_eq!(map.grid[y][width - 1], Cell::Obstacle); // droite
    }
  }
}

