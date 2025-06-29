use crate::map::cell::Cell;
use crate::robot::robot::{ResourceType, Robot, RobotType};
use crate::{GameState, Map, Station};
use std::collections::{HashMap, VecDeque};

pub fn automate_all_robots(state: &mut GameState, automation_enabled: bool) {
  let robot_positions: Vec<(usize, usize)> = state.robots.iter().map(|r| (r.x, r.y)).collect();

  for (i, robot) in state.robots.iter_mut().enumerate() {
    let other_robots: Vec<(usize, usize)> = robot_positions
      .iter()
      .enumerate()
      .filter(|(j, _)| *j != i)
      .map(|(_, pos)| *pos)
      .collect();

    if automation_enabled {
      automate_robot(
        robot,
        &state.map,
        &state.station,
        state.resources_revealed,
        &other_robots,
      );
    }

    // Déchargement automatique à la station
    if robot.x == state.station.x && robot.y == state.station.y {
      let science_deposited = robot.unload_resources(&mut state.station, &mut state.map);
      if science_deposited {
        if let Some(msg) = state.map.unlock_zone_with_science((robot.x, robot.y)) {
          println!("{msg}");
        }
        if state.robot_speed_ms > 30 {
          state.robot_speed_ms -= 50;
        }
      }
    }

    state.last_collect_message = robot.collect_resource(&mut state.map, state.resources_revealed);
  }
}

pub fn automate_robot(
  robot: &mut Robot,
  map: &Map,
  station: &Station,
  resources_revealed: bool,
  other_robots: &[(usize, usize)],
) {
  match robot.robot_type {
    RobotType::Explorator => {
      // Si le robot a déjà de la science, il retourne à la station
      if robot.inventory.contains_key(&ResourceType::Science) {
        if let Some((dx, dy)) = next_step_towards(
          robot.x,
          robot.y,
          station.x,
          station.y,
          map,
          resources_revealed,
        ) {
          robot.try_move(dx, dy, map, resources_revealed, other_robots, station);
        }
        return;
      }
      // Sinon, cherche la science la plus proche
      if let Some((tx, ty)) = find_nearest(robot.x, robot.y, map, Cell::Science, resources_revealed)
      {
        if let Some((dx, dy)) = next_step_towards(robot.x, robot.y, tx, ty, map, resources_revealed)
        {
          robot.try_move(dx, dy, map, resources_revealed, other_robots, station);
        }
      }
    }
    RobotType::Collector => {
      // Si l'inventaire est plein, retourne à la station
      if robot.inventory_count() >= robot.inventory_capacity {
        if let Some((dx, dy)) = next_step_towards(
          robot.x,
          robot.y,
          station.x,
          station.y,
          map,
          resources_revealed,
        ) {
          robot.try_move(dx, dy, map, resources_revealed, other_robots, station);
        }
        return;
      }
      // Sinon, cherche le minerai ou l'énergie la plus proche
      if let Some((tx, ty)) =
        find_nearest_with_access_check(robot.x, robot.y, map, Cell::Mineral, resources_revealed)
          .or_else(|| {
            find_nearest_with_access_check(robot.x, robot.y, map, Cell::Energy, resources_revealed)
          })
      {
        if let Some((dx, dy)) = next_step_towards(robot.x, robot.y, tx, ty, map, resources_revealed)
        {
          robot.try_move(dx, dy, map, resources_revealed, other_robots, station);
        }
      }
    }
  }
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
