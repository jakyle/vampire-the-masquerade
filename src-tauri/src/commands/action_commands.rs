use crate::{
    models::{action_info::ActionInfo, action_result::ActionResult},
    services::action_service::*,
};

#[tauri::command]
pub fn execute_action(action_info: ActionInfo) -> Result<ActionResult, String> {
    get_action_result(&action_info)
}
