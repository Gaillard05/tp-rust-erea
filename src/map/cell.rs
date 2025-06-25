#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
  Wall,
  Empty,
  Obstacle,
  Energy,
  Mineral,
  Science,
}
