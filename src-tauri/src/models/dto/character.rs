use crate::{models::db, util::date_time::*};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddCharacter {
    pub name: String,

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
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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
}

impl From<db::character::Character> for Character {
    fn from(value: db::character::Character) -> Self {
        Self {
            id: value.id,
            name: value.name,
            is_active: value.is_active,
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
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl Character {
    pub fn try_get_skill(&self, skill: &str) -> Result<i32, String> {
        match skill {
            "athletics" => Ok(self.athletics),
            "brawl" => Ok(self.brawl),
            "crafts" => Ok(self.crafts),
            "drive" => Ok(self.drive),
            "firearms" => Ok(self.firearms),
            "melee" => Ok(self.melee),
            "larceny" => Ok(self.larceny),
            "stealth" => Ok(self.stealth),
            "survival" => Ok(self.survival),
            "animal_ken" => Ok(self.animal_ken),
            "etiquette" => Ok(self.etiquette),
            "insight" => Ok(self.insight),
            "intimidation" => Ok(self.intimidation),
            "leadership" => Ok(self.leadership),
            "performance" => Ok(self.performance),
            "persuasion" => Ok(self.persuasion),
            "streetwise" => Ok(self.streetwise),
            "subterfuge" => Ok(self.subterfuge),
            "academics" => Ok(self.academics),
            "awareness" => Ok(self.awareness),
            "finance" => Ok(self.finance),
            "investigation" => Ok(self.investigation),
            "medicine" => Ok(self.medicine),
            "occult" => Ok(self.occult),
            "politics" => Ok(self.politics),
            "science" => Ok(self.science),
            "technology" => Ok(self.technology),
            _ => Err(format!("Invalid skill: {skill}")),
        }
    }

    pub fn try_get_attribute(&self, attribute: &str) -> Result<i32, String> {
        match attribute {
            "strength" => Ok(self.strength),
            "dexterity" => Ok(self.dexterity),
            "stamina" => Ok(self.stamina),
            "charisma" => Ok(self.charisma),
            "manipulation" => Ok(self.manipulation),
            "composure" => Ok(self.composure),
            "intelligence" => Ok(self.intelligence),
            "wits" => Ok(self.wits),
            "resolve" => Ok(self.resolve),
            _ => Err(format!("Invalid attribute: {attribute}")),
        }
    }
}
