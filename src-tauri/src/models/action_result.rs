use crate::models::db;
use serde::{Deserialize, Serialize};

use super::dto::{action_log::ActionLog, dice_result::DiceResult, passive_result::PassiveResult};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActionResultType {
    Rolls(DiceResult),
    Passives(PassiveResult),
}

impl ActionResultType {
    pub fn get_log_id(&self) -> Option<&str> {
        match self {
            Self::Rolls(dice_result) => dice_result.log_id.as_deref(),
            Self::Passives(passive_result) => passive_result.log_id.as_deref(),
        }
    }
}

impl From<db::dice_result::DiceResult> for ActionResultType {
    fn from(value: db::dice_result::DiceResult) -> Self {
        Self::Rolls(value.into())
    }
}

impl From<DiceResult> for ActionResultType {
    fn from(value: DiceResult) -> Self {
        Self::Rolls(value)
    }
}

impl From<db::passive_result::PassiveResult> for ActionResultType {
    fn from(value: db::passive_result::PassiveResult) -> Self {
        Self::Passives(value.into())
    }
}

impl From<PassiveResult> for ActionResultType {
    fn from(value: PassiveResult) -> Self {
        Self::Passives(value)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionResult {
    log: ActionLog,
    action_results: Vec<ActionResultType>,
}

impl ActionResult {
    pub fn new(log: ActionLog, action_result: Vec<ActionResultType>) -> Self {
        Self {
            log,
            action_results: action_result,
        }
    }
}
