use crate::map::cell::Cell;
use crate::map::map::Map;
use crate::station::station::Station;
use std::collections::{HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RobotType {
  Explorator,
  Collector,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum ResourceType {
  Mineral,
  Energy,
  Science,
}

#[derive(Debug)]
pub struct Robot {
  pub robots: Vec<Robot>,
  pub x: usize,
  pub y: usize,
  pub inventory: HashMap<ResourceType, u32>,
  pub inventory_capacity: usize,
  pub collected_science_positions: Vec<(usize, usize)>,
  pub robot_type: RobotType,
}

impl Robot {
  pub fn inventory_count(&self) -> usize {
    self.inventory.values().sum::<u32>() as usize
  }

  pub fn collect_resource(&mut self, map: &mut Map, resources_revealed: bool) -> Option<String> {
    let current_cell = map.grid[self.y][self.x];
    let is_accessible = map.is_resource_accessible(self.x, self.y);

    match current_cell {
      Cell::Science => {
        let count = self.inventory.entry(ResourceType::Science).or_insert(0);
        *count += 1;

        self.collected_science_positions.push((self.x, self.y));
        map.grid[self.y][self.x] = Cell::Empty;

        return Some("Lieu scientifique collecté ! Retourne au labo.".to_string());
      }
      Cell::Mineral if resources_revealed || is_accessible => {
        let count = self.inventory.entry(ResourceType::Mineral).or_insert(0);
        *count += 1;
        map.grid[self.y][self.x] = Cell::Empty;

        map.update_zone_resource_counts();

        None
      }
      Cell::Energy if resources_revealed || is_accessible => {
        let count = self.inventory.entry(ResourceType::Energy).or_insert(0);
        *count += 1;
        map.grid[self.y][self.x] = Cell::Empty;
        map.update_zone_resource_counts();

        None
      }
      _ => None,
    }
  }

  pub fn unload_resources(&mut self, station: &mut Station, map: &mut Map) -> bool {
    if self.x == station.x && self.y == station.y {
      if self.inventory.is_empty() {
        println!("Aucune ressource à décharger !");
        return false;
      } else {
        let mut science_deposited = false;

        for (res, qty) in self.inventory.drain() {
          if let ResourceType::Science = res {
            science_deposited = true;
          }
          *station.inventory.entry(res).or_insert(0) += qty;
        }

        if science_deposited && !self.collected_science_positions.is_empty() {
          for &science_pos in &self.collected_science_positions {
            map.unlock_zone_with_science(science_pos);
          }

          self.collected_science_positions.clear();
        }
        map.next_turn();
        return science_deposited;
      }
    } else {
      false
    }
  }

  pub fn try_move(
    &mut self,
    dx: isize,
    dy: isize,
    map: &Map,
    resources_revealed: bool,
    other_robots: &[(usize, usize)],
    station: &Station, // Ajoute ce paramètre
  ) {
    let new_x = (self.x as isize) + dx;
    let new_y = (self.y as isize) + dy;

    if new_x >= 0 && new_y >= 0 && (new_x as usize) < map.width && (new_y as usize) < map.height {
      // Vérifie la collision avec les autres robots
      let is_station = new_x as usize == station.x && new_y as usize == station.y;
      if !is_station
        && other_robots
          .iter()
          .any(|(x, y)| *x == new_x as usize && *y == new_y as usize)
      {
        // On n'autorise le stacking que sur la station
        println!("Déplacement impossible : un autre robot occupe déjà cette case !");
        return;
      }

      let target_cell = map.grid[new_y as usize][new_x as usize];
      let is_accessible = map.is_resource_accessible(new_x as usize, new_y as usize);

      if !resources_revealed
        && !is_accessible
        && (target_cell == Cell::Mineral || target_cell == Cell::Energy)
      {
        return;
      }

      if target_cell != Cell::Wall && target_cell != Cell::Obstacle {
        self.x = new_x as usize;
        self.y = new_y as usize;
      } else {
        println!("Déplacement impossible !");
      }
    }
  }


}
