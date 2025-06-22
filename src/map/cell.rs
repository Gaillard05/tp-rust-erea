#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Wall,
    Empty,
    Obstacle,
    Energy,
    Mineral,
    Science,
}
