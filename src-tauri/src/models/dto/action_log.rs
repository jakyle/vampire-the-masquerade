use crate::models::db;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddActionLog {
    pub modifier: i32,
    pub difficulty: i32,
}

impl AddActionLog {
    pub fn new(modifier: i32, difficulty: i32) -> Self {
        Self {
            modifier,
            difficulty,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ActionLog {
    pub id: String,
    pub modifier: i32,
    pub difficulty: i32,
    pub created_at: NaiveDateTime,
}

impl From<db::action_log::ActionLog> for ActionLog {
    fn from(value: db::action_log::ActionLog) -> Self {
        Self {
            id: value.id,
            modifier: value.modifier,
            difficulty: value.difficulty,
            created_at: value.created_at,
        }
    }
}
