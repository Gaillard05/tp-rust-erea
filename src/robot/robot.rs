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

    pub fn collect_resource(&mut self, map: &mut Map, resources_revealed: bool) -> Option<String> {
        // Vérifie si l'inventaire est plein
        if self.inventory_count() >= self.inventory_capacity {
            return Some("Inventaire plein ! Retourne à la station.".to_string());
        }

        let cell = &mut map.grid[self.y][self.x];
        match cell {
            Cell::Science => {
                let count = self.inventory.entry(ResourceType::Science).or_insert(0);
                *count += 1;
                *cell = Cell::Empty;
                Some("Lieu scientifique collecté !".to_string())
            }
            Cell::Mineral if resources_revealed => {
                let count = self.inventory.entry(ResourceType::Mineral).or_insert(0);
                *count += 1;
                *cell = Cell::Empty;
                Some("Minerai collecté !".to_string())
            }
            Cell::Energy if resources_revealed => {
                let count = self.inventory.entry(ResourceType::Energy).or_insert(0);
                *count += 1;
                *cell = Cell::Empty;
                Some("Énergie collectée !".to_string())
            }
            _ => None,
        }
    }

    pub fn unload_resources(&mut self, station: &mut Station) -> bool {
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
                println!("Ressources déposées à la station !");
                if science_deposited {
                    println!("Lieu scientifique rapporté ! Toute la carte est révélée !");
                }
                return science_deposited;
            }
        } else {
            println!("Le robot doit être sur la station pour décharger.");
            false
        }
    }

    // Fonction helper pour tenter un déplacement
    pub fn try_move(&mut self, dx: isize, dy: isize, map: &Map, resources_revealed: bool) {
        let new_x = (self.x as isize) + dx;
        let new_y = (self.y as isize) + dy;

        if
            new_x >= 0 &&
            new_y >= 0 &&
            (new_x as usize) < map.width &&
            (new_y as usize) < map.height
        {
            let cell = &map.grid[new_y as usize][new_x as usize];

            // Empêche de marcher sur une ressource cachée
            if !resources_revealed && (cell == &Cell::Mineral || cell == &Cell::Energy) {
                return;
            }

            // Empêche de marcher sur un mur ou un obstacle
            if cell != &Cell::Wall && cell != &Cell::Obstacle {
                self.x = new_x as usize;
                self.y = new_y as usize;
            } else {
                println!("Déplacement impossible !");
            }
        }
    }
}
