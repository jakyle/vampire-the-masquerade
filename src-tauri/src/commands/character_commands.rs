use crate::services::character_service;
use crate::{models, services::character_service::*};

#[tauri::command(async)]
pub fn get_characters() -> Vec<models::dto::character::Character> {
    select_characters()
}

#[tauri::command(async)]
pub fn get_character_by_id(id: &str) -> models::dto::character::Character {
    select_character_by_id(id)
}

#[tauri::command(async)]
pub fn post_character(
    character: models::dto::character::AddCharacter,
) -> models::dto::character::Character {
    character_service::insert_new_character(character)
}

#[tauri::command(async)]
pub fn delete_character(id: &str) {
    remove_character(id)
}

#[tauri::command(async)]
pub fn patch_character(
    character: models::dto::character::UpdateCharacter,
) -> models::dto::character::Character {
    update_character(character)
}
