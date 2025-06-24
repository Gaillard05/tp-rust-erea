use crate::map::cell::Cell;
use noise::{NoiseFn, Perlin};

pub fn generate_noise(width: usize, height: usize, send: u32) -> Vec<Vec<Cell>> {
  let perlin = Perlin::default();
  let mut grid = vec![vec![Cell::Empty; width]; height];

  for y in 0..height {
    for x in 0..width {
      if y == 0 || y == height - 1 || x == 0 || x == width - 1 {
        grid[y][x] = Cell::Wall;
        continue;
      }
      let noise_val = perlin.get([x as f64 / 10.0, y as f64 / 10.0, send as f64]);
      grid[y][x] = match noise_val {
        n if n < -0.3 => Cell::Obstacle,
        n if n < 0.0 => Cell::Energy,
        n if n < 0.2 => Cell::Mineral,
        n if n < 0.21 => Cell::Science,
        _ => Cell::Empty,
      };
    }
  }
  grid
}
