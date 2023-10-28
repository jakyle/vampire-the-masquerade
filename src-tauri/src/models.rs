#[derive(serde::Serialize, serde::Deserialize)]
pub struct DiceResult {
    successes: i32,
    criticals: i32,
    bestial_failure: bool,
    messy_critical: bool,
    succeeded: bool,
    rolls: Vec<i32>,
    hunger_rolls: Vec<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RollInfo {
    character_id: String,
    selected_skill: String,
    selected_attribute: String,
    difficulty: i32,
    modifier: i32,
    selected_characters: Vec<Character>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Character {
    pub is_active: bool,

    pub id: String,
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

    pub hunger: i32,
}
