use crate::map::cell::Cell;
use crate::map::map::Map;
use crate::station::station::Station;
// use std::collections::HashMap;
use std::collections::{ VecDeque, HashMap }; // Déplacer cette ligne au début du fichier, pas dans un impl

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

    pub fn automate_robot(&mut self, map: &Map, station: &Station, resources_revealed: bool) {
        // Si le robot a la science, retourne à la station
        if self.inventory.contains_key(&ResourceType::Science) {
            if
                let Some((dx, dy)) = Self::next_step_towards(
                    self.x,
                    self.y,
                    station.x,
                    station.y,
                    map,
                    resources_revealed
                )
            {
                self.try_move(dx, dy, map, resources_revealed);
            }
            return;
        }

        // Sinon, cherche la science ou une autre ressource selon l’état de révélation
        let target = if !resources_revealed {
            Self::find_nearest(self.x, self.y, map, Cell::Science, resources_revealed)
        } else if self.inventory_count() >= self.inventory_capacity {
            Some((station.x, station.y))
        } else {
            Self::find_nearest(self.x, self.y, map, Cell::Mineral, resources_revealed).or_else(||
                Self::find_nearest(self.x, self.y, map, Cell::Energy, resources_revealed)
            )
        };

        if let Some((tx, ty)) = target {
            if
                let Some((dx, dy)) = Self::next_step_towards(
                    self.x,
                    self.y,
                    tx,
                    ty,
                    map,
                    resources_revealed
                )
            {
                self.try_move(dx, dy, map, resources_revealed);
            }
        }
    }

    pub fn find_nearest(
        start_x: usize,
        start_y: usize,
        map: &Map,
        target: Cell,
        resources_revealed: bool
    ) -> Option<(usize, usize)> {
        let width = map.width;
        let height = map.height;
        let mut visited = vec![vec![false; width]; height];
        let mut queue = VecDeque::new();

        queue.push_back((start_x, start_y));
        visited[start_y][start_x] = true;

        while let Some((x, y)) = queue.pop_front() {
            let cell = &map.grid[y][x];

            // Condition de recherche
            if *cell == target {
                // Si la ressource est cachée, on ne la cible qu'après révélation
                if target == Cell::Mineral || target == Cell::Energy {
                    if resources_revealed {
                        return Some((x, y));
                    }
                } else {
                    return Some((x, y));
                }
            }

            // Explore les 4 directions
            let directions = [
                (0isize, 1isize),
                (1, 0),
                (0, -1),
                (-1, 0),
            ];
            for (dx, dy) in directions.iter() {
                let nx = (x as isize) + dx;
                let ny = (y as isize) + dy;

                if
                    nx >= 0 &&
                    ny >= 0 &&
                    (nx as usize) < width &&
                    (ny as usize) < height &&
                    !visited[ny as usize][nx as usize]
                {
                    let next_cell = &map.grid[ny as usize][nx as usize];
                    // On n'explore pas les murs, obstacles, ou ressources cachées avant révélation
                    let blocked =
                        matches!(next_cell, Cell::Wall | Cell::Obstacle) ||
                        (!resources_revealed && matches!(next_cell, Cell::Mineral | Cell::Energy));
                    if !blocked {
                        visited[ny as usize][nx as usize] = true;
                        queue.push_back((nx as usize, ny as usize));
                    }
                }
            }
        }
        None
    }

    pub fn next_step_towards(
        start_x: usize,
        start_y: usize,
        target_x: usize,
        target_y: usize,
        map: &Map,
        resources_revealed: bool
    ) -> Option<(isize, isize)> {
        let width = map.width;
        let height = map.height;
        let mut visited = vec![vec![false; width]; height];
        let mut queue = VecDeque::new();
        let mut parent: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

        queue.push_back((start_x, start_y));
        visited[start_y][start_x] = true;

        // BFS pour trouver le chemin
        while let Some((x, y)) = queue.pop_front() {
            if x == target_x && y == target_y {
                // Remonte le chemin jusqu'à la première étape
                let mut cur = (x, y);
                let mut path = vec![cur];
                while let Some(&p) = parent.get(&cur) {
                    path.push(p);
                    cur = p;
                }
                path.reverse();
                if path.len() >= 2 {
                    let (nx, ny) = path[1];
                    let dx = (nx as isize) - (start_x as isize);
                    let dy = (ny as isize) - (start_y as isize);
                    return Some((dx, dy));
                } else {
                    return None; // Déjà sur la cible
                }
            }

            let directions = [
                (0isize, 1isize),
                (1, 0),
                (0, -1),
                (-1, 0),
            ];
            for (dx, dy) in directions.iter() {
                let nx = (x as isize) + dx;
                let ny = (y as isize) + dy;
                if
                    nx >= 0 &&
                    ny >= 0 &&
                    (nx as usize) < width &&
                    (ny as usize) < height &&
                    !visited[ny as usize][nx as usize]
                {
                    let next_cell = &map.grid[ny as usize][nx as usize];
                    let blocked =
                        matches!(next_cell, Cell::Wall | Cell::Obstacle) ||
                        (!resources_revealed && matches!(next_cell, Cell::Mineral | Cell::Energy));
                    if !blocked {
                        visited[ny as usize][nx as usize] = true;
                        parent.insert((nx as usize, ny as usize), (x, y));
                        queue.push_back((nx as usize, ny as usize));
                    }
                }
            }
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::cell::Cell;
    use crate::map::map::Map;

    #[test]
    fn robot_inventory_starts() {
        let robot = Robot {
            x: 0,
            y: 0,
            inventory: HashMap::new(),
            inventory_capacity: 5,
        };
        assert_eq!(robot.inventory_count(), 0);
    }

    #[test]
    fn robot_collects_science() {
        let mut robot = Robot {
            x: 0,
            y: 0,
            inventory: HashMap::new(),
            inventory_capacity: 5,
        };
        let mut map = Map {
            width: 1,
            height: 1,
            grid: vec![vec![Cell::Science]],
        };
        let msg = robot.collect_resource(&mut map, true);
        assert_eq!(msg, Some("Lieu scientifique collecté !".to_string()));
        assert_eq!(robot.inventory.get(&ResourceType::Science), Some(&1));
    }
}
