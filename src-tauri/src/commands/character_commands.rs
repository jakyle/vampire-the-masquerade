use crate::{models, services::character_service::*};

#[tauri::command]
pub fn get_characters() -> Vec<models::dto::character::Character> {
    select_characters()
}

#[tauri::command]
pub fn get_character_by_id(id: &str) -> models::dto::character::Character {
    select_character_by_id(id)
}
