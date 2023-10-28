// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod models;
mod window_menu;

use models::*;
use window_menu::{build_menu, menu_event_handler};

fn main() {
    tauri::Builder::default()
        .menu(build_menu())
        .on_menu_event(menu_event_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn say_hello() -> String {
    "Hello, World!".to_string()
}
