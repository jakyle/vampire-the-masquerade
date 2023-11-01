use serde::{Deserialize, Serialize};

use super::dto::{action_log::ActionLog, dice_result::DiceResult, passive_result::PassiveResult};

#[derive(Serialize, Deserialize)]
pub enum ActionResultType {
    Rolls(Vec<DiceResult>),
    Passives(Vec<PassiveResult>),
}

#[derive(Serialize, Deserialize)]
pub struct ActionResult {
    log: ActionLog,
    action_result: ActionResultType,
}

impl ActionResult {
    pub fn new(log: ActionLog, action_result: ActionResultType) -> Self {
        Self { log, action_result }
    }
}
