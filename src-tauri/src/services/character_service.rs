use crate::schema::characters::dsl;
use crate::{
    db::establish_db_connection,
    models::{db, dto},
};
use diesel::prelude::*;

pub fn select_characters() -> Vec<dto::character::Character> {
    let connection = &mut establish_db_connection();

    dsl::characters
        .order(dsl::name.asc())
        .load::<db::character::Character>(connection)
        .unwrap()
        .into_iter()
        .map(Into::into)
        .collect()
}

pub fn select_characters_by_ids(ids: &[&str]) -> Vec<dto::character::Character> {
    let connection = &mut establish_db_connection();

    dsl::characters
        .order(dsl::name.asc())
        .filter(dsl::id.eq_any(ids))
        .load::<db::character::Character>(connection)
        .unwrap()
        .into_iter()
        .map(Into::into)
        .collect()
}

pub fn select_character_by_id(id: &str) -> dto::character::Character {
    let connection = &mut establish_db_connection();

    dsl::characters
        .find(id)
        .first::<db::character::Character>(connection)
        .unwrap()
        .into()
}

// pub fn insert_new_characters(
//     characters: &[dto::character::AddCharacter],
// ) -> dto::character::Character {
//     let connection = &mut establish_db_connection();

//     let new_character = db::character::Character::default();

//     diesel::insert_into(characters::table)
//         .values(&new_character)
//         .get_result::<db::character::Character>(connection)
//         .unwrap()
//         .into()
// }

// pub fn insert_new_character(new_character: dto::character::AddCharacter) {
//     let connection = &mut establish_db_connection();

//     let character = db::character::Character::from(new_character);

//     diesel::insert_into(characters::table)
//         .values(character)
//         .execute(connection)
//         .unwrap();
// }

// pub fn update_character(update_character: dto::character::UpdateCharacter) {
//     let connection = &mut establish_db_connection();

//     let character = db::character::UpdateCharacter::from(update_character);

//     diesel::update(dsl::characters.find(&character.id))
//         .set(&character)
//         .execute(connection)
//         .unwrap();
// }

// pub fn delete_character(character_id: &str) {
//     let connection = &mut establish_db_connection();

//     diesel::delete(dsl::characters.find(character_id))
//         .execute(connection)
//         .unwrap();
// }
