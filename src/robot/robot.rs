use crate::map::cell::Cell;
use crate::map::map::Map;
use crate::station::station::Station;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum ResourceType {
  Mineral,
  Energy,
  Science,
}

#[derive(Debug)]
pub struct Robot {
  pub x: usize,
  pub y: usize,
  pub inventory: HashMap<ResourceType, u32>,
  pub inventory_capacity: usize,
  pub collected_science_positions: Vec<(usize, usize)>,
}

impl Robot {
  pub fn inventory_count(&self) -> usize {
    self.inventory.values().sum::<u32>() as usize
  }

  pub fn collect_resource(&mut self, map: &mut Map, resources_revealed: bool) -> Option<String> {
    if self.inventory_count() >= self.inventory_capacity {
      return Some("Inventaire plein ! Retourne à la station.".to_string());
    }

    let current_cell = map.grid[self.y][self.x];
    let is_accessible = map.is_resource_accessible(self.x, self.y);

    match current_cell {
      Cell::Science => {
        let count = self.inventory.entry(ResourceType::Science).or_insert(0);
        *count += 1;

        self.collected_science_positions.push((self.x, self.y));

        let zone_name = map.get_zone_at(self.x, self.y).map(|z| z.name.clone());

        map.grid[self.y][self.x] = Cell::Empty;

        if let Some(name) = zone_name {
          return Some(format!(
            "Échantillon de la région {} collecté ! Retourne au labo.",
            name
          ));
        } else {
          return Some("Lieu scientifique collecté ! Retourne au labo.".to_string());
        }
      }
      Cell::Mineral if resources_revealed || is_accessible => {
        let count = self.inventory.entry(ResourceType::Mineral).or_insert(0);
        *count += 1;
        map.grid[self.y][self.x] = Cell::Empty;
        Some("Minerai collecté !".to_string())
      }
      Cell::Energy if resources_revealed || is_accessible => {
        let count = self.inventory.entry(ResourceType::Energy).or_insert(0);
        *count += 1;
        map.grid[self.y][self.x] = Cell::Empty;
        Some("Énergie collectée !".to_string())
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
          println!("🧪 Analyse des échantillons en cours...");

          for &science_pos in &self.collected_science_positions {
            map.unlock_zone_with_science(science_pos);
          }

          self.collected_science_positions.clear();
        }

        println!("Ressources déposées à la station !");
        map.next_turn();
        return science_deposited;
      }
    } else {
      println!("Le robot doit être sur la station pour décharger.");
      false
    }
  }

  pub fn try_move(&mut self, dx: isize, dy: isize, map: &Map, resources_revealed: bool) {
    let new_x = (self.x as isize) + dx;
    let new_y = (self.y as isize) + dy;

    if new_x >= 0 && new_y >= 0 && (new_x as usize) < map.width && (new_y as usize) < map.height {
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
