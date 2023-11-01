use crate::models::{db, text_array::TextArray};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DiceResult {
    pub id: String,
    pub character_id: Option<String>,
    pub log_id: Option<String>,
    pub successes: i32,
    pub criticals: i32,
    pub beastial_failure: bool,
    pub messy_critical: bool,
    pub succeeded: bool,
    pub rolls: TextArray<i32>,
    pub hunger_rolls: TextArray<i32>,
    pub created_at: NaiveDateTime,
}

pub struct AddDiceResult {
    pub character_id: Option<String>,
    pub log_id: Option<String>,
    pub successes: i32,
    pub criticals: i32,
    pub beastial_failure: bool,
    pub messy_critical: bool,
    pub succeeded: bool,
    pub rolls: TextArray<i32>,
    pub hunger_rolls: TextArray<i32>,
}

impl AddDiceResult {
    pub fn new(
        successes: i32,
        criticals: i32,
        bestial_failure: bool,
        messy_critical: bool,
        succeeded: bool,
        rolls: Vec<i32>,
        hunger_rolls: Vec<i32>,
        character_id: Option<String>,
        log_id: Option<String>,
    ) -> Self {
        Self {
            character_id,
            log_id,
            successes,
            criticals,
            beastial_failure: bestial_failure,
            messy_critical,
            succeeded,
            rolls: rolls.into(),
            hunger_rolls: hunger_rolls.into(),
        }
    }
}

impl From<db::dice_result::DiceResult> for DiceResult {
    fn from(value: db::dice_result::DiceResult) -> Self {
        let rolls = serde_json::from_str(&value.rolls).unwrap();
        let hunger_rolls = serde_json::from_str(&value.hunger_rolls).unwrap();

        Self {
            id: value.id,
            character_id: value.character_id,
            log_id: value.log_id,
            successes: value.successes,
            criticals: value.criticals,
            beastial_failure: value.beastial_failure,
            messy_critical: value.messy_critical,
            succeeded: value.succeeded,
            rolls,
            hunger_rolls,
            created_at: value.created_at,
        }
    }
}
