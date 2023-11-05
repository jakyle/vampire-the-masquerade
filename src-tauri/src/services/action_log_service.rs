use std::collections::HashMap;
use std::rc::Rc;

use crate::{
    db::establish_db_connection,
    models::db::dice_result::DiceResult,
    models::{
        action_result::{ActionResult, ActionResultType},
        db::{self, passive_result::PassiveResult},
        dto,
    },
    schema::action_logs,
};

use diesel::prelude::*;

pub fn select_action_logs_with_results() -> Vec<ActionResult> {
    let connection = &mut establish_db_connection();

    let logs = action_logs::dsl::action_logs
        .order(action_logs::dsl::created_at.desc())
        .load::<db::action_log::ActionLog>(connection)
        .unwrap();

    let dice_results = DiceResult::belonging_to(&logs)
        .select(DiceResult::as_select())
        .load(connection)
        .unwrap()
        .into_iter()
        .map(ActionResultType::from);

    let passive_results = PassiveResult::belonging_to(&logs)
        .select(PassiveResult::as_select())
        .load(connection)
        .unwrap()
        .into_iter()
        .map(ActionResultType::from);

    let mut results_by_log: HashMap<Rc<str>, Vec<ActionResultType>> =
        HashMap::with_capacity(logs.len());
    for result in dice_results.chain(passive_results) {
        let log_id = Rc::<str>::from(result.get_log_id().unwrap());
        results_by_log
            .entry(log_id.clone())
            .or_default()
            .push(result);
    }

    logs.into_iter()
        .map(|log| {
            let log_id = Rc::from(log.id.as_str());
            let combined_results = results_by_log.remove(&log_id).unwrap_or_default();
            ActionResult::new(log.into(), combined_results)
        })
        .collect()
}

pub fn insert_new_action_log(
    new_action_log: dto::action_log::AddActionLog,
) -> dto::action_log::ActionLog {
    let connection = &mut establish_db_connection();

    let insert_action_log = db::action_log::ActionLog::from(new_action_log);

    diesel::insert_into(action_logs::table)
        .values(&insert_action_log)
        .get_result::<db::action_log::ActionLog>(connection)
        .unwrap()
        .into()
}
