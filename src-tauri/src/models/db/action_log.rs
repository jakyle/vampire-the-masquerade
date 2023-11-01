use crate::models::dto::action_log::AddActionLog;
use crate::schema::action_logs;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Insertable, PartialEq)]
#[diesel(table_name = action_logs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ActionLog {
    pub id: String,
    pub modifier: i32,
    pub difficulty: i32,
    pub created_at: NaiveDateTime,
}

impl ActionLog {
    pub fn new(modifier: i32, difficulty: i32) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            modifier,
            difficulty,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<AddActionLog> for ActionLog {
    fn from(
        AddActionLog {
            modifier,
            difficulty,
        }: AddActionLog,
    ) -> Self {
        Self::new(modifier, difficulty)
    }
}
