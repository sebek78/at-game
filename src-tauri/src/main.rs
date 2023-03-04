#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod file;
use file::{load_from_file, save_to_file};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_from_file, save_to_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
