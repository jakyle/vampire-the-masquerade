use crate::db::establish_db_connection;
use crate::models::{db, dto};
use crate::schema::passive_results;
use diesel::prelude::*;

pub fn insert_new_passive_result(
    new_dice_result: &[dto::passive_result::AddPassiveResult],
) -> Vec<dto::passive_result::PassiveResult> {
    let connection = &mut establish_db_connection();

    new_dice_result
        .iter()
        .map(|die| db::passive_result::PassiveResult::from(die))
        .map(|die| {
            diesel::insert_into(passive_results::table)
                .values(&die)
                .get_result::<db::passive_result::PassiveResult>(connection)
                .unwrap()
                .into()
        })
        .collect::<Vec<dto::passive_result::PassiveResult>>()
}

// pub fn delete_dice_result(dice_result_id: &str) {
//     let connection = &mut establish_db_connection();

//     diesel::delete(dsl::dice_results.find(dice_result_id))
//         .execute(connection)
//         .unwrap();
// }
