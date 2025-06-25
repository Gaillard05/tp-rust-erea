use colored::*;
use super::cell::Cell;
use super::zone::Zone;
use crate::robot::robot::Robot;
use crate::station::station::Station;
use crate::utils::noise::generate_noise;

pub struct Map {
  pub width: usize,
  pub height: usize,
  pub grid: Vec<Vec<Cell>>,
  pub zones: Vec<Zone>,
  pub current_turn: u32,
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

    let zones = Self::create_zones(width, height);

    let mut map = Self {
      width,
      height,
      grid,
      zones,
      current_turn: 0,
    };

    map.count_resources_in_zones();
    map
  }

  fn create_zones(width: usize, height: usize) -> Vec<Zone> {
    let mut zones = Vec::new();

    let zone_width = width / 2;
    let zone_height = height / 2;

    let zone_names = vec!["Nord-Ouest", "Nord-Est", "Sud-Ouest", "Sud-Est"];

    for (i, name) in zone_names.iter().enumerate() {
      let row = i / 2;
      let col = i % 2;

      let min_x = col * zone_width;
      let max_x = if col == 1 {
        width - 1
      } else {
        (col + 1) * zone_width - 1
      };
      let min_y = row * zone_height;
      let max_y = if row == 1 {
        height - 1
      } else {
        (row + 1) * zone_height - 1
      };

      zones.push(Zone::new(i, name.to_string(), min_x, max_x, min_y, max_y));
    }

    zones
  }

  fn count_resources_in_zones(&mut self) {
    for zone in &mut self.zones {
      let mut minerals = 0;
      let mut energies = 0;

      for y in zone.min_y..=zone.max_y {
        for x in zone.min_x..=zone.max_x {
          if y < self.height && x < self.width {
            match self.grid[y][x] {
              Cell::Mineral => minerals += 1,
              Cell::Energy => energies += 1,
              _ => {}
            }
          }
        }
      }

      zone.resource_count = (minerals, energies);
    }
  }

  pub fn get_zone_at(&self, x: usize, y: usize) -> Option<&Zone> {
    self.zones.iter().find(|zone| zone.contains_point(x, y))
  }

  pub fn unlock_zone_with_science(&mut self, science_pos: (usize, usize)) -> Option<String> {
    let (x, y) = science_pos;

    for zone in &mut self.zones {
      if zone.contains_point(x, y) && !zone.is_unlocked {
        zone.unlock();
        return Some(format!("Zone {} débloquée", zone.name));
      }
    }

    None
  }

  pub fn is_resource_accessible(&self, x: usize, y: usize) -> bool {
    if let Some(zone) = self.get_zone_at(x, y) {
      zone.is_unlocked
    } else {
      false
    }
  }

  pub fn zone_stats(&self) -> (usize, usize, f32) {
    let unlocked_count = self.zones.iter().filter(|z| z.is_unlocked).count();
    let total_zones = self.zones.len();
    let percentage = (unlocked_count as f32 / total_zones as f32) * 100.0;

    (unlocked_count, total_zones, percentage)
  }

  pub fn accessible_resources(&self) -> (u32, u32) {
    let mut minerals = 0;
    let mut energies = 0;

    for zone in &self.zones {
      if zone.is_unlocked {
        minerals += zone.resource_count.0;
        energies += zone.resource_count.1;
      }
    }

    (minerals, energies)
  }

  pub fn next_turn(&mut self) {
    self.current_turn += 1;
  }

  pub fn print(&self, robot: &Robot, station: &Station, resources_revealed: bool) {
    for (y, row) in self.grid.iter().enumerate() {
      for (x, cell) in row.iter().enumerate() {
        if x == robot.x && y == robot.y {
          print!("{}", "🤖".green().bold());
        } else if x == station.x && y == station.y {
          print!("{}", "🏭".yellow().bold());
        } else {
          let is_accessible = self.is_resource_accessible(x, y);

          let symbol = match cell {
            Cell::Wall | Cell::Obstacle => "██".bright_black(),
            Cell::Empty => "  ".white(),
            Cell::Science => "🧪".purple().bold(),
            Cell::Mineral => {
              if resources_revealed || is_accessible {
                "💎".blue().bold()
              } else {
                "❓".red().bold()
              }
            }
            Cell::Energy => {
              if resources_revealed || is_accessible {
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
