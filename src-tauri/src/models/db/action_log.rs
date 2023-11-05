use crate::schema::action_logs;
use crate::{models::dto::action_log::AddActionLog, util::date_time::*};
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
    #[serde(serialize_with = "date_to_string", deserialize_with = "string_to_date")]
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

// 2023-11-03T20:05:48.671851700Z
