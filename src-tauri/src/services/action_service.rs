use rand::Rng;

use crate::{
    models::{
        action_info::ActionInfo,
        action_result::{ActionResult, ActionResultType},
        action_type::ActionType,
        dto::{
            action_log::AddActionLog, character::Character, dice_result::AddDiceResult,
            passive_result::AddPassiveResult,
        },
    },
    services::{
        action_log_service::insert_new_action_log, character_service::select_characters_by_ids,
        passive_result_service::insert_new_passive_result,
        roll_result_service::insert_new_dice_result,
    },
};

pub fn get_action_result(roll_info: &ActionInfo) -> Result<ActionResult, String> {
    let characters = select_characters_by_ids(&roll_info.selected_characters);
    let action_log =
        insert_new_action_log(AddActionLog::new(roll_info.modifier, roll_info.difficulty));

    let action_result_type = match roll_info.action_type {
        ActionType::Roll => {
            let results = characters
                .iter()
                .map(|character| {
                    roll_dice(
                        &roll_info.skill,
                        &roll_info.attribute,
                        roll_info.difficulty,
                        roll_info.modifier,
                        character,
                        &action_log.id,
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;

            let roll_results = insert_new_dice_result(&results);
            ActionResultType::Rolls(roll_results)
        }
        ActionType::Passive => {
            let results = characters
                .iter()
                .map(|character| {
                    check_passive(
                        &roll_info.skill,
                        &roll_info.attribute,
                        roll_info.difficulty,
                        character,
                        &action_log.id,
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;

            let passive_results = insert_new_passive_result(&results);
            ActionResultType::Passives(passive_results)
        }
    };

    Ok(ActionResult::new(action_log, action_result_type))
}

fn roll_dice(
    selected_skill: &Option<&str>,
    selected_attribute: &Option<&str>,
    difficulty: i32,
    modifier: i32,
    character: &Character,
    log_id: &str,
) -> Result<AddDiceResult, String> {
    let skill = selected_skill
        .as_ref()
        .map(|skill| character.try_get_skill(skill))
        .transpose()?
        .unwrap_or(0);

    let attribute = selected_attribute
        .as_ref()
        .map(|skill| character.try_get_attribute(skill))
        .transpose()?
        .unwrap_or(0);

    let dice_pool_without_hunger = std::cmp::max(skill + attribute + modifier, 0);
    let hunger = character.hunger;

    let mut rng = rand::thread_rng();
    let rolls = (0..dice_pool_without_hunger - std::cmp::min(dice_pool_without_hunger, hunger))
        .map(|_| rng.gen_range(1..=10))
        .collect::<Vec<_>>();

    let hunger_rolls = (0..std::cmp::min(dice_pool_without_hunger, hunger))
        .map(|_| rng.gen_range(1..=10))
        .collect::<Vec<_>>();

    let successes = rolls.iter().filter(|&&roll| roll >= 6).count() as i32
        + hunger_rolls.iter().filter(|&&roll| roll >= 6).count() as i32;
    let half_messy_critical = hunger_rolls.iter().filter(|&&roll| roll == 10).count();
    let half_critical = rolls.iter().filter(|&&roll| roll == 10).count();
    let criticals = ((half_messy_critical + half_critical) / 2) as i32;
    let messy_critical = criticals > 0 && half_messy_critical > 0;
    let succeeded = successes >= difficulty;
    let bestial_failure = hunger_rolls.iter().filter(|&&roll| roll == 1).count() >= 1 && !succeeded;

    Ok(AddDiceResult::new(
        successes,
        criticals,
        bestial_failure,
        messy_critical,
        succeeded,
        rolls,
        hunger_rolls,
        Some(character.id.to_string()),
        Some(log_id.to_string()),
    ))
}

fn check_passive(
    selected_skill: &Option<&str>,
    selected_attribute: &Option<&str>,
    difficulty: i32,
    character: &Character,
    log_id: &str,
) -> Result<AddPassiveResult, String> {
    let skill = selected_skill
        .as_ref()
        .map(|skill| character.try_get_skill(skill))
        .transpose()?
        .unwrap_or(0);

    let attribute = selected_attribute
        .as_ref()
        .map(|skill| character.try_get_attribute(skill))
        .transpose()?
        .unwrap_or(0);

    Ok(AddPassiveResult::new(
        Some(character.id.to_string()),
        Some(log_id.to_string()),
        (skill + attribute) >= difficulty,
        character.hunger,
        skill + attribute,
    ))
}
