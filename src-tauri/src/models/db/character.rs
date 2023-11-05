use crate::schema::characters;
use crate::{models::dto, util::date_time::*};
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = characters)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Character {
    pub id: String,
    pub name: String,

    pub is_active: bool,

    pub strength: i32,
    pub dexterity: i32,
    pub stamina: i32,

    pub charisma: i32,
    pub manipulation: i32,
    pub composure: i32,

    pub intelligence: i32,
    pub wits: i32,
    pub resolve: i32,

    pub athletics: i32,
    pub brawl: i32,
    pub crafts: i32,
    pub drive: i32,
    pub firearms: i32,
    pub melee: i32,
    pub larceny: i32,
    pub stealth: i32,
    pub survival: i32,

    pub animal_ken: i32,
    pub etiquette: i32,
    pub insight: i32,
    pub intimidation: i32,
    pub leadership: i32,
    pub performance: i32,
    pub persuasion: i32,
    pub streetwise: i32,
    pub subterfuge: i32,

    pub academics: i32,
    pub awareness: i32,
    pub finance: i32,
    pub investigation: i32,
    pub medicine: i32,
    pub occult: i32,
    pub politics: i32,
    pub science: i32,
    pub technology: i32,

    pub hunger: i32,

    #[serde(serialize_with = "date_to_string", deserialize_with = "string_to_date")]
    pub created_at: NaiveDateTime,

    #[serde(serialize_with = "date_to_string", deserialize_with = "string_to_date")]
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, AsChangeset, Identifiable)]
#[diesel(table_name = characters)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateCharacter {
    pub id: String,
    pub name: Option<String>,

    pub strength: Option<i32>,
    pub dexterity: Option<i32>,
    pub stamina: Option<i32>,

    pub charisma: Option<i32>,
    pub manipulation: Option<i32>,
    pub composure: Option<i32>,

    pub intelligence: Option<i32>,
    pub wits: Option<i32>,
    pub resolve: Option<i32>,

    pub athletics: Option<i32>,
    pub brawl: Option<i32>,
    pub crafts: Option<i32>,
    pub drive: Option<i32>,
    pub firearms: Option<i32>,
    pub melee: Option<i32>,
    pub larceny: Option<i32>,
    pub stealth: Option<i32>,
    pub survival: Option<i32>,

    pub animal_ken: Option<i32>,
    pub etiquette: Option<i32>,
    pub insight: Option<i32>,
    pub intimidation: Option<i32>,
    pub leadership: Option<i32>,
    pub performance: Option<i32>,
    pub persuasion: Option<i32>,
    pub streetwise: Option<i32>,
    pub subterfuge: Option<i32>,

    pub academics: Option<i32>,
    pub awareness: Option<i32>,
    pub finance: Option<i32>,
    pub investigation: Option<i32>,
    pub medicine: Option<i32>,
    pub occult: Option<i32>,
    pub politics: Option<i32>,
    pub science: Option<i32>,
    pub technology: Option<i32>,

    pub hunger: Option<i32>,

    #[serde(serialize_with = "date_to_string", deserialize_with = "string_to_date")]
    pub updated_at: NaiveDateTime,
}

impl From<dto::character::AddCharacter> for Character {
    fn from(value: dto::character::AddCharacter) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: value.name,
            is_active: true,
            strength: value.strength,
            dexterity: value.dexterity,
            stamina: value.stamina,

            charisma: value.charisma,
            manipulation: value.manipulation,
            composure: value.composure,

            intelligence: value.intelligence,
            wits: value.wits,
            resolve: value.resolve,

            athletics: value.athletics,
            brawl: value.brawl,
            crafts: value.crafts,
            drive: value.drive,
            firearms: value.firearms,
            melee: value.melee,
            larceny: value.larceny,
            stealth: value.stealth,
            survival: value.survival,

            animal_ken: value.animal_ken,
            etiquette: value.etiquette,
            insight: value.insight,
            intimidation: value.intimidation,
            leadership: value.leadership,
            performance: value.performance,
            persuasion: value.persuasion,
            streetwise: value.streetwise,
            subterfuge: value.subterfuge,

            academics: value.academics,
            awareness: value.awareness,
            finance: value.finance,
            investigation: value.investigation,
            medicine: value.medicine,
            occult: value.occult,
            politics: value.politics,
            science: value.science,
            technology: value.technology,

            hunger: 1,

            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<dto::character::UpdateCharacter> for UpdateCharacter {
    fn from(value: dto::character::UpdateCharacter) -> Self {
        Self {
            id: value.id,
            name: value.name,

            strength: value.strength,
            dexterity: value.dexterity,
            stamina: value.stamina,

            charisma: value.charisma,
            manipulation: value.manipulation,
            composure: value.composure,

            intelligence: value.intelligence,
            wits: value.wits,
            resolve: value.resolve,

            athletics: value.athletics,
            brawl: value.brawl,
            crafts: value.crafts,
            drive: value.drive,
            firearms: value.firearms,
            melee: value.melee,
            larceny: value.larceny,
            stealth: value.stealth,
            survival: value.survival,

            animal_ken: value.animal_ken,
            etiquette: value.etiquette,
            insight: value.insight,
            intimidation: value.intimidation,
            leadership: value.leadership,
            performance: value.performance,
            persuasion: value.persuasion,
            streetwise: value.streetwise,
            subterfuge: value.subterfuge,

            academics: value.academics,
            awareness: value.awareness,
            finance: value.finance,
            investigation: value.investigation,
            medicine: value.medicine,
            occult: value.occult,
            politics: value.politics,
            science: value.science,
            technology: value.technology,

            hunger: value.hunger,

            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
