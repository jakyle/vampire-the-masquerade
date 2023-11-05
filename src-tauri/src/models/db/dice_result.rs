use super::{action_log::ActionLog, character::Character};
use crate::{models::dto::dice_result::AddDiceResult, schema::dice_results, util::date_time::*};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Insertable)]
#[diesel(table_name = dice_results)]
#[diesel(belongs_to(Character))]
#[diesel(belongs_to(ActionLog, foreign_key = log_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DiceResult {
    pub id: String,
    pub character_id: Option<String>,
    pub log_id: Option<String>,
    pub successes: i32,
    pub criticals: i32,
    pub beastial_failure: bool,
    pub messy_critical: bool,
    pub succeeded: bool,
    pub rolls: String,
    pub hunger_rolls: String,

    #[serde(serialize_with = "date_to_string", deserialize_with = "string_to_date")]
    pub created_at: NaiveDateTime,
}

impl From<&AddDiceResult> for DiceResult {
    fn from(dice_result: &AddDiceResult) -> Self {
        let rolls = serde_json::to_string(dice_result.rolls.as_slice()).unwrap();
        let hunger_rolls = serde_json::to_string(dice_result.hunger_rolls.as_slice()).unwrap();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            character_id: dice_result.character_id.clone(),
            log_id: dice_result.log_id.clone(),
            successes: dice_result.successes,
            criticals: dice_result.criticals,
            beastial_failure: dice_result.beastial_failure,
            messy_critical: dice_result.messy_critical,
            succeeded: dice_result.succeeded,
            rolls,
            hunger_rolls,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}
