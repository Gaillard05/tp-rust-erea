#[derive(Debug, Clone, PartialEq)]
pub struct Zone {
  pub id: usize,
  pub name: String,
  pub min_x: usize,
  pub max_x: usize,
  pub min_y: usize,
  pub max_y: usize,
  pub is_unlocked: bool,
  pub resource_count: (u32, u32),
}

impl Zone {
  pub fn new(
    id: usize,
    name: String,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
  ) -> Self {
    Zone {
      id,
      name,
      min_x,
      max_x,
      min_y,
      max_y,
      is_unlocked: false,
      resource_count: (0, 0),
    }
  }

  pub fn contains_point(&self, x: usize, y: usize) -> bool {
    x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
  }

  pub fn unlock(&mut self) {
    self.is_unlocked = true;
  }
}
