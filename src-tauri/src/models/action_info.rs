use super::action_type::ActionType;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ActionInfo<'a> {
    pub skill: Option<&'a str>,
    pub attribute: Option<&'a str>,
    pub difficulty: i32,
    pub modifier: i32,
    pub selected_characters: Vec<&'a str>,
    pub action_type: ActionType,
}
