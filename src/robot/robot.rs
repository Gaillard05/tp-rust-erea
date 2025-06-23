use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{enable_raw_mode, disable_raw_mode},
};
use std::collections::HashMap;
use crate::map::map::Map; 
use crate::map::cell::Cell;



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
}

impl Robot {
    pub fn collect_resource(&mut self, map: &mut Map) -> Option<String> {
        let cell = &mut map.grid[self.y][self.x]; 
        let resource = match cell {
            Cell::Mineral => Some(ResourceType::Mineral),
            Cell::Energy => Some(ResourceType::Energy),
            Cell::Science => Some(ResourceType::Science),
            _ => None,
        };
    
        if let Some(res) = resource {
            let res_clone = res.clone();
            let count = self.inventory.entry(res_clone).or_insert(0);
            *count += 1;
            *cell = Cell::Empty;
            Some(format!("Ressource collectée : {:?} !", res))
        } else {
            None
        }
    }
    
    pub fn move_robot(&mut self, map: &Map) -> std::io::Result<()> {
        enable_raw_mode()?;
    
        println!("Déplace le robot (flèches, Échap pour quitter) : ");
    
        loop {
            if let Event::Key(key_event) = event::read()? {
                // On agit uniquement sur KeyEventKind::Press pour éviter les doubles déplacements
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Up => {
                            self.try_move(0, -1, map);
                            break;
                        }
                        KeyCode::Down => {
                            self.try_move(0, 1, map);
                            break;
                        }
                        KeyCode::Left => {
                            self.try_move(-1, 0, map);
                            break;
                        }
                        KeyCode::Right => {
                            self.try_move(1, 0, map);
                            break;
                        }
                        KeyCode::Esc => {
                            println!("Arrêt du programme.");
                            disable_raw_mode()?;
                            std::process::exit(0);
                        }
                        _ => {
                            println!("Touche non reconnue. Utilise les flèches ou Échap.");
                            break;
                        }
                    }
                }
            }
        }
    
        disable_raw_mode()?; 
        Ok(())
    }
    
    // Fonction helper pour tenter un déplacement
    fn try_move(&mut self, dx: isize, dy: isize, map: &Map) {
        let new_x = self.x as isize + dx; 
        let new_y = self.y as isize + dy; 
    
        if new_x >= 0
            && new_y >= 0
            && (new_x as usize) < map.width
            && (new_y as usize) < map.height
            && map.grid[new_y as usize][new_x as usize] != Cell::Wall
            && map.grid[new_y as usize][new_x as usize] != Cell::Obstacle
        {
            self.x = new_x as usize;
            self.y = new_y as usize;
        } else {
            println!("Déplacement impossible !");
        }
    }
}