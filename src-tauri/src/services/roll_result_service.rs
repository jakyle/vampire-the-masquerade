use crate::db::establish_db_connection;
use crate::models::{db, dto};
use crate::schema::dice_results;
use diesel::prelude::*;

pub fn insert_new_dice_result(
    new_dice_result: &[dto::dice_result::AddDiceResult],
) -> Vec<dto::dice_result::DiceResult> {
    let connection = &mut establish_db_connection();

    new_dice_result
        .iter()
        .map(|die| db::dice_result::DiceResult::from(die))
        .map(|die| {
            diesel::insert_into(dice_results::table)
                .values(&die)
                .get_result::<db::dice_result::DiceResult>(connection)
                .unwrap()
                .into()
        })
        .collect::<Vec<dto::dice_result::DiceResult>>()
}

// pub fn delete_dice_result(dice_result_id: &str) {
//     let connection = &mut establish_db_connection();

//     diesel::delete(dsl::dice_results.find(dice_result_id))
//         .execute(connection)
//         .unwrap();
// }
