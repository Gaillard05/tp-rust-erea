use crate::station::station::Station;

pub fn print_station_inventory(station: &Station) {
  println!("╔══════════════════════════════╗");
  println!("║    INVENTAIRE DE LA STATION  ║");
  println!("╚══════════════════════════════╝");
  
  if station.inventory.is_empty() {
    println!("       Aucune ressource       ");
  } else {
    for (res, qty) in &station.inventory {
      let icon = match res {
        crate::robot::robot::ResourceType::Mineral => "💎",
        crate::robot::robot::ResourceType::Energy => "⚡",
        crate::robot::robot::ResourceType::Science => "🧪",
      };
      println!("  {} {:?} : {} unités", icon, res, qty);
    }
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
  println!("╚══════════════════════════════╝ ╚══════════════════════════════╝");
  println!();
}