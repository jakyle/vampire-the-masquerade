use crate::{
    models::{action_info::ActionInfo, action_result::ActionResult},
    services::{action_log_service::select_action_logs_with_results, action_service::*},
};

#[tauri::command(async)]
pub fn execute_action(payload: ActionInfo) -> Result<ActionResult, String> {
    get_action_result(&payload)
}

#[tauri::command(async)]
pub fn get_action_logs() -> Vec<ActionResult> {
    select_action_logs_with_results()
}
