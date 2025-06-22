use super::cell::Cell;
use crate::utils::noise::generate_noise;
use crate::robot::robot::Robot;
use crate::station::station::Station;
use colored::*; 

pub struct Map {
    pub width: usize,  
    pub height: usize,  
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

        Self {width, height, grid}
    }

    pub fn print(&self, robot: &Robot, station: &Station) {
        println!("Legend: @ = Station, R = Robot, ## = Wall, . = Empty, M = Mineral, E = Energy, S = Science, ## = Obstacle");
        println!("Map size: {}x{}", self.width, self.height);
        
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if x == robot.x && y == robot.y {
                    print!("{}", "R ".green().bold());
                } else if x == station.x && y == station.y {
                    print!("{}", "@ ".yellow().bold()); // Station en jaune
                } else {
                    let symbol = match cell {
                        Cell::Wall | Cell::Obstacle => "##",
                        Cell::Empty => ". ",
                        Cell::Mineral => "M ",
                        Cell::Energy => "E ",
                        Cell::Science => "S ",
                    };
                    print!("{}", symbol);
                }
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