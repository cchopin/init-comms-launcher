#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::process::Command;

#[tauri::command]
fn open_link(link: &str) -> Result<(), String> {
  println!("open_link appelé avec le lien : {}", link);

  // Appel pour Windows
  #[cfg(target_os = "windows")]
  {
    let status = Command::new("cmd")
      .args(&["/C", "start", "", link])
      .status()
      .map_err(|e| e.to_string())?;

    println!("Statut de la commande start : {:?}", status);
    if !status.success() {
      return Err(format!("La commande start a retourné un code non nul: {:?}", status));
    }
  }

  // Appel pour macOS
  #[cfg(target_os = "macos")]
  {
    // Si le lien commence par "mumble://", on encode les crochets
    let fixed_link = if link.starts_with("mumble://") {
      link.replace("[", "%5B").replace("]", "%5D")
    } else {
      link.to_string()
    };

    let status = Command::new("/usr/bin/open")
      .args(&["-a", "Mumble", &fixed_link])
      .status()
      .map_err(|e| e.to_string())?;

    println!("Statut de la commande open : {:?}", status);
    if !status.success() {
      return Err(format!("La commande open a retourné un code non nul: {:?}", status));
    }
  }

  Ok(())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![open_link])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
