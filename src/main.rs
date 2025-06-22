mod map;
mod utils;
mod config;
mod robot;
mod station;

use map::map::Map;
use config::Config;
use crate::robot::robot::Robot;
use crate::map::cell::Cell; 
use crate::station::station::Station;
use text_io::read;

fn main() {
    let config = Config::default();
    let mut robot = Robot { x: 8, y: 4 };
    let map = Map::new(config.width, config.heigth, config.seed);
    let station = Station { x: 9, y: 4 };

    loop {
        map.print(&robot, &station); 
        move_robot(&map, &mut robot); 
    }
}

fn move_robot(map: &Map, robot: &mut Robot) {
    println!("Déplace le robot (z=haut, s=bas, q=gauche, d=droite, x=quitter) : ");
    let cmd: String = read!();

    let (dx, dy) = match cmd.as_str() {
        "z" => (0, -1),
        "s" => (0, 1),
        "q" => (-1, 0),
        "d" => (1, 0),
        "x" => {
            println!("Arrêt du programme.");
            std::process::exit(0);
        }
        _ => {
            println!("Commande inconnue.");
            return;
        }
    };

    let new_x = robot.x as isize + dx;
    let new_y = robot.y as isize + dy;

    if new_x >= 0
        && new_y >= 0
        && (new_x as usize) < map.width
        && (new_y as usize) < map.height
        && map.grid[new_y as usize][new_x as usize] != Cell::Wall
        && map.grid[new_y as usize][new_x as usize] != Cell::Obstacle
    {
        robot.x = new_x as usize;
        robot.y = new_y as usize;
    } else {
        println!("Déplacement impossible !");
    }
}