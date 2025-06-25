use std::collections::HashMap;
use tp_rust_EREEA::game_state::GameState;
use tp_rust_EREEA::config::Config;
use tp_rust_EREEA::map::cell::Cell;
use tp_rust_EREEA::map::map::Map;
use tp_rust_EREEA::robot::robot::{ResourceType, Robot};
use tp_rust_EREEA::station::station::Station;

#[test]
fn test_game_state_initialization() {
    let config = Config::default();
    let state = GameState::new(&config); // ✅


    assert_eq!(state.resources_revealed, false);
    assert_eq!(state.robot.inventory_count(), 0);
    assert_eq!(state.map.width, config.width);
    assert_eq!(state.map.height, config.height);
}

#[test]
fn test_collect_science() {
    let mut map = Map::new(10, 10, 42);
    map.grid[5][5] = Cell::Science;

    let mut robot = Robot {
        x: 5,
        y: 5,
        inventory: HashMap::new(),
        inventory_capacity: 5,
    };

    let message = robot.collect_resource(&mut map, true);
    assert_eq!(robot.inventory.get(&ResourceType::Science), Some(&1));
    //assert!(message.unwrap().contains("Science"));
    assert_eq!(message, Some("Lieu scientifique collecté !".to_string()));

}


#[test]
fn test_unload_station() {
    let mut station = Station {
        x: 5,
        y: 5,
        inventory: HashMap::new(),
    };

    let mut robot = Robot {
        x: 5,
        y: 5,
        inventory: [(ResourceType::Science, 2)].iter().cloned().collect(),
        inventory_capacity: 5,
    };

    let deposited = robot.unload_resources(&mut station);
    assert!(deposited);
    assert_eq!(station.inventory.get(&ResourceType::Science), Some(&2));
    assert_eq!(robot.inventory_count(), 0);
}
