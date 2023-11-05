use super::{action_log::ActionLog, character::Character};
use crate::schema::passive_results;
use crate::{models::dto, util::date_time::*};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Insertable)]
#[diesel(table_name = passive_results)]
#[diesel(belongs_to(Character))]
#[diesel(belongs_to(ActionLog, foreign_key = log_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PassiveResult {
    pub id: String,
    pub character_id: Option<String>,
    pub log_id: Option<String>,
    pub succeeded: bool,
    pub hunger: i32,
    pub total: i32,

    #[serde(serialize_with = "date_to_string", deserialize_with = "string_to_date")]
    pub created_at: NaiveDateTime,
}

impl PassiveResult {
    pub fn new(character_id: String, succeeded: bool, hunger: i32, total: i32) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            character_id: Some(character_id),
            log_id: None,
            succeeded,
            hunger,
            total,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<&dto::passive_result::AddPassiveResult> for PassiveResult {
    fn from(value: &dto::passive_result::AddPassiveResult) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            character_id: value.character_id.clone(),
            log_id: value.log_id.clone(),
            succeeded: value.succeeded,
            hunger: value.hunger,
            total: value.total,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}
