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
}

impl Robot {
  pub fn inventory_count(&self) -> usize {
    self.inventory.values().sum::<u32>() as usize
  }

  pub fn collect_resource(&mut self, map: &mut Map) -> Option<String> {
    let cell = &mut map.grid[self.y][self.x];
    let resource = match cell {
      Cell::Mineral => Some(ResourceType::Mineral),
      Cell::Energy => Some(ResourceType::Energy),
      Cell::Science => Some(ResourceType::Science),
      _ => None,
    };

    if let Some(res) = resource {
      if self.inventory_count() < self.inventory_capacity {
        let count = self.inventory.entry(res).or_insert(0);
        *count += 1;
        *cell = Cell::Empty;
        Some(format!(
          "Ressource collectée ! Inventaire : {:?}",
          self.inventory
        ))
      } else {
        Some("Inventaire plein ! Retourne à la station pour décharger.".to_string())
      }
    } else {
      None
    }
  }

  pub fn unload_resources(&mut self, station: &mut Station) {
    if self.x == station.x && self.y == station.y {
      if self.inventory.is_empty() {
        println!("Aucune ressource à décharger !");
      } else {
        for (res, qty) in self.inventory.drain() {
          *station.inventory.entry(res).or_insert(0) += qty;
        }
        println!("Ressources déposées à la station !");
      }
    } else {
      println!("Le robot doit être sur la station pour décharger.");
    }
  }

  // Fonction helper pour tenter un déplacement
  pub fn try_move(&mut self, dx: isize, dy: isize, map: &Map) {
    let new_x = self.x as isize + dx;
    let new_y = self.y as isize + dy;

    if new_x >= 0
      && new_y >= 0
      && (new_x as usize) < map.width
      && (new_y as usize) < map.height
      && map.grid[new_y as usize][new_x as usize] != Cell::Wall
      && map.grid[new_y as usize][new_x as usize] != Cell::Obstacle
    {
      self.x = new_x as usize;
      self.y = new_y as usize;
    } else {
      println!("Déplacement impossible !");
    }
  }
}
