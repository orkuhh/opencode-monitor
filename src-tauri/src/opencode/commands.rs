// OpenCode Tauri Commands

use crate::opencode::OpenCodeClient;
use tauri::State;

// Global OpenCode client instance
#[tauri::command]
pub async fn opencode_health(client: State<'_, OpenCodeClient>) -> Result<serde_json::Value, String> {
    let health = client.health().await
        .map_err(|e| e.to_string())?;
    Ok(serde_json::to_value(health).unwrap())
}

#[tauri::command]
pub async fn opencode_list_sessions(client: State<'_, OpenCodeClient>) -> Result<Vec<super::opencode::Session>, String> {
    client.list_sessions().await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn opencode_create_session(client: State<'_, OpenCodeClient>, title: Option<&str>) -> Result<super::opencode::Session, String> {
    client.create_session(title).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn opencode_send_message(
    client: State<'_, OpenCodeClient>,
    session_id: &str,
    message: &str,
    model: Option<&str>
) -> Result<Vec<super::opencode::Message>, String> {
    client.send_message(session_id, message, model).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn opencode_get_messages(
    client: State<'_, OpenCodeClient>,
    session_id: &str,
    limit: Option<i32>
) -> Result<Vec<super::opencode::Message>, String> {
    client.get_messages(session_id, limit).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn opencode_get_diffs(client: State<'_, OpenCodeClient>, session_id: &str) -> Result<Vec<super::opencode::FileDiff>, String> {
    client.get_diffs(session_id).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn opencode_abort_session(client: State<'_, OpenCodeClient>, session_id: &str) -> Result<bool, String> {
    client.abort_session(session_id).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn opencode_delete_session(client: State<'_, OpenCodeClient>, session_id: &str) -> Result<bool, String> {
    client.delete_session(session_id).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn opencode_search_files(client: State<'_, OpenCodeClient>, pattern: &str) -> Result<Vec<String>, String> {
    client.search_files(pattern).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn opencode_read_file(client: State<'_, OpenCodeClient>, path: &str) -> Result<String, String> {
    client.read_file(path).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn opencode_list_files(client: State<'_, OpenCodeClient>, path: &str) -> Result<serde_json::Value, String> {
    client.list_files(path).await
        .map_err(|e| e.to_string())
}
