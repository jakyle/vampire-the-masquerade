// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod db;
mod models;
mod schema;
mod services;
mod window_menu;

use crate::commands::{
    action_commands::execute_action,
    character_commands::{get_character_by_id, get_characters},
};
use window_menu::{build_menu, menu_event_handler};

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    tauri::Builder::default()
        .setup(|_app| {
            tokio::spawn(async move {
                db::init();
            });
            Ok(())
        })
        .menu(build_menu())
        .on_menu_event(menu_event_handler)
        .invoke_handler(tauri::generate_handler![
            execute_action,
            get_characters,
            get_character_by_id
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
