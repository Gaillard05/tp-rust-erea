#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Obstacle,
    Energy,
    Mineral,
    Science,
}