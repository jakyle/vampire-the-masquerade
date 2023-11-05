use crate::schema::characters::dsl;
use crate::schema::characters::dsl::characters;
use crate::schema::dice_results;
use crate::schema::passive_results;
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

pub fn insert_new_character(character: dto::character::AddCharacter) -> dto::character::Character {
    let connection = &mut establish_db_connection();

    let new_character = db::character::Character::from(character);

    diesel::insert_into(characters)
        .values(&new_character)
        .get_result::<db::character::Character>(connection)
        .unwrap()
        .into()
}

pub fn update_character(
    update_character: dto::character::UpdateCharacter,
) -> dto::character::Character {
    let connection = &mut establish_db_connection();
    let character = db::character::UpdateCharacter::from(update_character);

    diesel::update(dsl::characters.find(&character.id))
        .set(&character)
        .get_result::<db::character::Character>(connection)
        .unwrap()
        .into()
}

pub fn remove_character(character_id: &str) {
    let connection = &mut establish_db_connection();

    diesel::update(dice_results::table.filter(dice_results::character_id.eq(character_id)))
        .set(dice_results::character_id.eq::<Option<String>>(None))
        .execute(connection)
        .unwrap();

    diesel::update(passive_results::table.filter(passive_results::character_id.eq(character_id)))
        .set(passive_results::character_id.eq::<Option<String>>(None))
        .execute(connection)
        .unwrap();

    diesel::delete(dsl::characters.find(character_id))
        .execute(connection)
        .unwrap();
}
