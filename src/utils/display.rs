use crate::robot::robot::ResourceType;
use crate::robot::robot::Robot;
use crate::station::station::Station;

pub fn print_inventories(station: &Station, robot: &Robot) {
  println!("╔══════════════════════════════╗ ╔══════════════════════════════╗");
  println!("║   INVENTAIRE DE LA STATION   ║ ║    INVENTAIRE DU ROBOT       ║");
  println!("╚══════════════════════════════╝ ╚══════════════════════════════╝");

  let station_items: Vec<String> = if station.inventory.is_empty() {
    vec!["       Aucune ressource       ".to_string()]
  } else {
    station
      .inventory
      .iter()
      .map(|(res, qty)| {
        let icon = match res {
          ResourceType::Mineral => "💎",
          ResourceType::Energy => "⚡",
          ResourceType::Science => "🧪",
        };
        format!("  {} {:?} : {} unités", icon, res, qty)
      })
      .collect()
  };

  let robot_items: Vec<String> = if robot.inventory.is_empty() {
    vec!["       Aucune ressource       ".to_string()]
  } else {
    robot
      .inventory
      .iter()
      .map(|(res, qty)| {
        let icon = match res {
          ResourceType::Mineral => "💎",
          ResourceType::Energy => "⚡",
          ResourceType::Science => "🧪",
        };
        format!("  {} {:?} : {} unités", icon, res, qty)
      })
      .collect()
  };

  let max_items = station_items.len().max(robot_items.len());
  let empty_line = "                              ";

  for i in 0..max_items {
    let station_line = station_items
      .get(i)
      .map(|s| s.as_str())
      .unwrap_or(empty_line);
    let robot_line = robot_items.get(i).map(|s| s.as_str()).unwrap_or(empty_line);
    println!("{:<30} {:<30}", station_line, robot_line);
  }

  println!();
}

pub fn print_commands_and_indicators() {
  println!("╔══════════════════════════════╗ ╔══════════════════════════════╗");
  println!("║          COMMANDES           ║ ║       INDICATEURS            ║");
  println!("╠══════════════════════════════╣ ╠══════════════════════════════╣");
  println!("║ ↑ ↓ ← →  : Déplacer robot    ║ ║ 💎 Mineral                   ║");
  println!("║ u        : Décharger         ║ ║ ⚡ Energy                    ║");
  println!("║ ESC      : Quitter           ║ ║ 🧪 Science                   ║");
  println!("╚══════════════════════════════╝ ║ 🤖 Robot                     ║");
  println!("                                 ║ 🏭 Station                   ║");
  println!("                                 ╚══════════════════════════════╝");
  println!();
}
