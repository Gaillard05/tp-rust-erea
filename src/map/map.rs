use super::cell::Cell;
use crate::utils::noise::generate_noise;

pub struct Map {
    width: usize,
    height: usize,
    pub grid: Vec<Vec<Cell>>,
}

impl Map {
    pub fn new(width: usize, height: usize, seed: u32) -> Self {
        let mut grid = generate_noise(width, height, seed);

        for x in 0..width {
            grid[0][x] = Cell::Obstacle;
            grid[height - 1][x] = Cell::Obstacle;
        }

        for y in 0..height {
            grid[y][0] = Cell::Obstacle;
            grid[y][width - 1] = Cell::Obstacle;
        }

        Self {
            width,
            height,
            grid,
        }
    }

    pub fn print(&self) {
        println!(
            "Legend: ## = Wall, . = Empty, M = Mineral, E = Energy, S = lieu intérêt scientifique, ## = Obstacle"
        );
        println!("Map size: {}x{}", self.width, self.height);
        for row in &self.grid {
            for cell in row {
                let symbol = match cell {
                    Cell::Wall => "##",
                    Cell::Empty => ". ",
                    Cell::Energy => "E ",
                    Cell::Mineral => "M ",
                    Cell::Science => "S ",
                    Cell::Obstacle => "##",
                };
                print!("{symbol} ");
            }

            println!();
        }
    }

    // pub fn collect(&mut self, x: usize, y: usize) -> Option<Cell> {
    //     if x >= self.width || y >= self.height {
    //         return None;
    //     }

    //     let cell = &mut self.grid[y][x];
    //     match cell {
    //         Cell::Energy | Cell::Mineral => {
    //             let ressource = cell.clone();
    //             *cell = Cell::Empty;
    //             Some(ressource)
    //         }
    //         _ => None,
    //     }
    // }
}
