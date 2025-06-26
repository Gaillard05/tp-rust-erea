use crate::Map;
use crate::robot::robot::ResourceType;
use crate::station::station::Station;

pub fn print_commands_and_indicators() {
  println!("╔══════════════════════════════╗ ╔══════════════════════════════╗");
  println!("║          COMMANDES           ║ ║       INDICATEURS            ║");
  println!("╠══════════════════════════════╣ ╠══════════════════════════════╣");
  println!("║ ↑ ↓ ← →  : Déplacer robot    ║ ║ 💎 Mineral                   ║");
  println!("║ u        : Décharger         ║ ║ ⚡ Energy                    ║");
  println!("║ a        : Automatiser       ║ ║ 🧪 Science                   ║");
  println!("║ ESC      : Quitter           ║ ║ 🤖 Collecteur                ║");
  println!("╚══════════════════════════════╝ ║ 👽 Explorateur               ║");
  println!("                                 ║ 🏭 Station                   ║");
  println!("                                 ╚══════════════════════════════╝");
  println!();
}

pub fn print_map_stats(map: &Map, robot_speed_ms: u64, station: &Station) {
  let speed_sec = robot_speed_ms as f64 / 1000.0;
  let speed_kmh = 3.6 / speed_sec; // Conversion en km/h (assumant 1 case = 1m)
  let unlocked_zones: Vec<&str> = map
    .zones
    .iter()
    .filter(|z| z.is_unlocked)
    .map(|z| z.name.as_str())
    .collect();

  
  println!("╔══════════════════════════════╗");
  println!("║       INFO DE LA MAP         ║");
  println!("╚══════════════════════════════╝");
  
  println!(
    "🗾 Map size: {}x{} | Tour: {}",
    map.width, map.height, map.current_turn
  );
  
  let (unlocked, total, percentage) = map.zone_stats();
  let (minerals, energies) = map.accessible_resources();
  
  println!(
    "🌍 Zones explorées: {}/{} ({:.0}%)",
    unlocked, total, percentage
  );
  println!("💎 Minerais accessibles: {} ", minerals);
  println!("⚡ Énergies accessibles: {}", energies);
  
  
  println!();
  
  println!("╔══════════════════════════════╗");
  println!("║   INVENTAIRE DE LA STATION   ║");
  println!("╚══════════════════════════════╝");
  
  if station.inventory.is_empty() {
    println!("       Aucune ressource       ");
  } else {
    for (res, qty) in &station.inventory {
      let icon = match res {
        ResourceType::Mineral => "💎",
        ResourceType::Energy => "⚡",
        ResourceType::Science => "🧪",
      };
      println!("  {} {:?} : {} unités", icon, res, qty);
    }
  }
  println!("");

  println!("🤖 Vitesse du robot: {:.2} km/h", speed_kmh);
  println!("📍 Régions cartographiées: {}", unlocked_zones.join(", "));
  println!("");
}
