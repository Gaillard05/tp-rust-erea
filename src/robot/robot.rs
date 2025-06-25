use crate::map::cell::Cell;
use crate::map::map::Map;
use crate::station::station::Station;
use std::collections::{HashMap, VecDeque};

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
  pub robot_type: RobotTypes,
}

#[derive(Debug, Clone, Copy)]
pub enum RobotTypes {
  Explorateur, 
  Collecteurs
}

impl Robot {
  pub fn describe(&self) {
    println!("Ce robot est de type : {:?}", self.robot_type)
  }

  pub fn choice_type_robot() -> RobotTypes {
    use std::io::{self, Write};

    loop {
      println!("Choisissez le type pour le robot :");
      println!("1. Explorateur");
      println!("2. Collecteurs");
      print!("Votre choix (1-2) : ");

      io::stdout().flush().unwrap();

      let mut input = String::new();
      io::stdin().read_line(&mut input).unwrap();
      

      match input.trim() {
          "1" => return RobotTypes::Explorateur,
          "2" => return RobotTypes::Collecteurs,
          _=> println!("Choix invalide.Essayer encore."),
      }
    }
  }

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
        map.grid[self.y][self.x] = Cell::Empty;

        return Some("Lieu scientifique collecté ! Retourne au labo.".to_string());
      }
      Cell::Mineral if resources_revealed || is_accessible => {
        let count = self.inventory.entry(ResourceType::Mineral).or_insert(0);
        *count += 1;
        map.grid[self.y][self.x] = Cell::Empty;

        map.update_zone_resource_counts();

        Some("Minerai collecté !".to_string())
      }
      Cell::Energy if resources_revealed || is_accessible => {
        let count = self.inventory.entry(ResourceType::Energy).or_insert(0);
        *count += 1;
        map.grid[self.y][self.x] = Cell::Empty;
        map.update_zone_resource_counts();

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

  pub fn automate_robot(&mut self, map: &Map, station: &Station, resources_revealed: bool) {
    if self.inventory.contains_key(&ResourceType::Science) {
      if let Some((dx, dy)) = Self::next_step_towards(
        self.x,
        self.y,
        station.x,
        station.y,
        map,
        resources_revealed,
      ) {
        self.try_move(dx, dy, map, resources_revealed);
      }
      return;
    }

    let target = if self.inventory_count() >= self.inventory_capacity {
      Some((station.x, station.y))
    } else {
      if let Some(science_pos) =
        Self::find_nearest(self.x, self.y, map, Cell::Science, resources_revealed)
      {
        Some(science_pos)
      } else if let Some(resource_pos) =
        Self::find_nearest_accessible_resource(self.x, self.y, map, resources_revealed)
      {
        Some(resource_pos)
      } else {
        Self::find_exploration_target(self.x, self.y, map, resources_revealed)
      }
    };

    if let Some((tx, ty)) = target {
      if let Some((dx, dy)) =
        Self::next_step_towards(self.x, self.y, tx, ty, map, resources_revealed)
      {
        self.try_move(dx, dy, map, resources_revealed);
      }
    } else {
      println!("Aucune cible accessible - Robot en attente");
    }
  }

  pub fn find_exploration_target(
    start_x: usize,
    start_y: usize,
    map: &Map,
    _resources_revealed: bool,
  ) -> Option<(usize, usize)> {
    let width = map.width;
    let height = map.height;
    let mut best_distance = 0;
    let mut best_target = None;

    for y in 1..height - 1 {
      for x in 1..width - 1 {
        if matches!(map.grid[y][x], Cell::Empty) {
          let distance = (((x as i32) - (start_x as i32)).abs()
            + ((y as i32) - (start_y as i32)).abs()) as usize;
          if distance > best_distance {
            best_distance = distance;
            best_target = Some((x, y));
          }
        }
      }
    }

    best_target
  }

  pub fn find_nearest_accessible_resource(
    start_x: usize,
    start_y: usize,
    map: &Map,
    resources_revealed: bool,
  ) -> Option<(usize, usize)> {
    if let Some(pos) =
      Self::find_nearest_with_access_check(start_x, start_y, map, Cell::Mineral, resources_revealed)
    {
      return Some(pos);
    }
    Self::find_nearest_with_access_check(start_x, start_y, map, Cell::Energy, resources_revealed)
  }

  pub fn find_nearest_with_access_check(
    start_x: usize,
    start_y: usize,
    map: &Map,
    target: Cell,
    resources_revealed: bool,
  ) -> Option<(usize, usize)> {
    let width = map.width;
    let height = map.height;
    let mut visited = vec![vec![false; width]; height];
    let mut queue = VecDeque::new();

    queue.push_back((start_x, start_y));
    visited[start_y][start_x] = true;

    while let Some((x, y)) = queue.pop_front() {
      let cell = &map.grid[y][x];

      if *cell == target {
        let is_accessible = map.is_resource_accessible(x, y);
        if resources_revealed || is_accessible {
          return Some((x, y));
        }
      }

      let directions = [(0isize, 1isize), (1, 0), (0, -1), (-1, 0)];
      for (dx, dy) in directions.iter() {
        let nx = (x as isize) + dx;
        let ny = (y as isize) + dy;

        if nx >= 0
          && ny >= 0
          && (nx as usize) < width
          && (ny as usize) < height
          && !visited[ny as usize][nx as usize]
        {
          let next_cell = &map.grid[ny as usize][nx as usize];
          let is_accessible = map.is_resource_accessible(nx as usize, ny as usize);

          let blocked = matches!(next_cell, Cell::Wall | Cell::Obstacle)
            || (!resources_revealed
              && !is_accessible
              && matches!(next_cell, Cell::Mineral | Cell::Energy));

          if !blocked {
            visited[ny as usize][nx as usize] = true;
            queue.push_back((nx as usize, ny as usize));
          }
        }
      }
    }
    None
  }

  pub fn find_nearest(
    start_x: usize,
    start_y: usize,
    map: &Map,
    target: Cell,
    resources_revealed: bool,
  ) -> Option<(usize, usize)> {
    let width = map.width;
    let height = map.height;
    let mut visited = vec![vec![false; width]; height];
    let mut queue = VecDeque::new();

    queue.push_back((start_x, start_y));
    visited[start_y][start_x] = true;

    while let Some((x, y)) = queue.pop_front() {
      let cell = &map.grid[y][x];

      if *cell == target {
        if target == Cell::Mineral || target == Cell::Energy {
          if resources_revealed {
            return Some((x, y));
          }
        } else {
          return Some((x, y));
        }
      }

      let directions = [(0isize, 1isize), (1, 0), (0, -1), (-1, 0)];
      for (dx, dy) in directions.iter() {
        let nx = (x as isize) + dx;
        let ny = (y as isize) + dy;

        if nx >= 0
          && ny >= 0
          && (nx as usize) < width
          && (ny as usize) < height
          && !visited[ny as usize][nx as usize]
        {
          let next_cell = &map.grid[ny as usize][nx as usize];
          let blocked = matches!(next_cell, Cell::Wall | Cell::Obstacle)
            || (!resources_revealed && matches!(next_cell, Cell::Mineral | Cell::Energy));
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
    resources_revealed: bool,
  ) -> Option<(isize, isize)> {
    let width = map.width;
    let height = map.height;
    let mut visited = vec![vec![false; width]; height];
    let mut queue = VecDeque::new();
    let mut parent: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    queue.push_back((start_x, start_y));
    visited[start_y][start_x] = true;

    while let Some((x, y)) = queue.pop_front() {
      if x == target_x && y == target_y {
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
          return None;
        }
      }

      let directions = [(0isize, 1isize), (1, 0), (0, -1), (-1, 0)];
      for (dx, dy) in directions.iter() {
        let nx = (x as isize) + dx;
        let ny = (y as isize) + dy;
        if nx >= 0
          && ny >= 0
          && (nx as usize) < width
          && (ny as usize) < height
          && !visited[ny as usize][nx as usize]
        {
          let next_cell = &map.grid[ny as usize][nx as usize];

          let is_accessible = map.is_resource_accessible(nx as usize, ny as usize);
          let blocked = matches!(next_cell, Cell::Wall | Cell::Obstacle)
            || (!resources_revealed
              && !is_accessible
              && matches!(next_cell, Cell::Mineral | Cell::Energy));

          if !blocked {
            visited[ny as usize][nx as usize] = true;
            parent.insert((nx as usize, ny as usize), (x, y));
            queue.push_back((nx as usize, ny as usize));
          } else if (nx as usize) == target_x && (ny as usize) == target_y {
            if is_accessible || resources_revealed {
              visited[ny as usize][nx as usize] = true;
              parent.insert((nx as usize, ny as usize), (x, y));
              queue.push_back((nx as usize, ny as usize));
            }
          }
        }
      }
    }

    println!("Aucun chemin trouvé vers ({}, {})", target_x, target_y);
    None
  }
}
