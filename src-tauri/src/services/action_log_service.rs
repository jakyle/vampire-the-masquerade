use crate::schema::action_logs;
use crate::{
    db::establish_db_connection,
    models::{db, dto},
};
use diesel::prelude::*;

// pub fn select_dice_results() -> Vec<db::dice_result::DiceResult> {
//     let connection = &mut establish_db_connection();

//     dsl::dice_results
//         .order(dsl::created_at.asc())
//         .load::<db::dice_result::DiceResult>(connection)
//         .unwrap()
// }

// pub fn select_dice_result_by_id(id: &str) -> db::dice_result::DiceResult {
//     let connection = &mut establish_db_connection();

//     dsl::dice_results
//         .find(id)
//         .first::<db::dice_result::DiceResult>(connection)
//         .unwrap()
// }

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

// pub fn delete_dice_result(dice_result_id: &str) {
//     let connection = &mut establish_db_connection();

//     diesel::delete(dsl::action_logs.find(dice_result_id))
//         .execute(connection)
//         .unwrap();
// }
