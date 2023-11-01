use crate::models::db;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PassiveResult {
    pub id: String,
    pub character_id: Option<String>,
    pub log_id: Option<String>,
    pub succeeded: bool,
    pub hunger: i32,
    pub total: i32,
    pub created_at: NaiveDateTime,
}

impl From<db::passive_result::PassiveResult> for PassiveResult {
    fn from(value: db::passive_result::PassiveResult) -> Self {
        Self {
            id: value.id,
            character_id: value.character_id,
            log_id: value.log_id,
            succeeded: value.succeeded,
            hunger: value.hunger,
            total: value.total,
            created_at: value.created_at,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AddPassiveResult {
    pub character_id: Option<String>,
    pub log_id: Option<String>,
    pub succeeded: bool,
    pub hunger: i32,
    pub total: i32,
}

impl AddPassiveResult {
    pub fn new(
        character_id: Option<String>,
        log_id: Option<String>,
        succeeded: bool,
        hunger: i32,
        total: i32,
    ) -> Self {
        Self {
            character_id,
            log_id,
            succeeded,
            hunger,
            total,
        }
    }
}
