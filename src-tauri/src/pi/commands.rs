// Pi Tauri Commands

use crate::pi::PiManager;
use tauri::State;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct PiConfigDto {
    pub model: String,
    pub thinking: String,
    pub system_prompt: String,
    pub provider: String,
}

#[tauri::command]
pub async fn pi_list_models(manager: State<'_, PiManager>) -> Result<Vec<String>, String> {
    manager.list_models().await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn pi_get_config(manager: State<'_, PiManager>) -> Result<PiConfigDto, String> {
    let config = manager.get_config();
    Ok(PiConfigDto {
        model: config.model,
        thinking: config.thinking,
        system_prompt: config.system_prompt,
        provider: config.provider,
    })
}

#[tauri::command]
pub async fn pi_update_config(
    manager: State<'_, PiManager>,
    model: Option<&str>,
    thinking: Option<&str>,
    system_prompt: Option<&str>,
    provider: Option<&str>
) -> Result<bool, String> {
    let mut config = manager.get_config();
    if let Some(m) = model { config.model = m.to_string(); }
    if let Some(t) = thinking { config.thinking = t.to_string(); }
    if let Some(sp) = system_prompt { config.system_prompt = sp.to_string(); }
    if let Some(p) = provider { config.provider = p.to_string(); }
    
    manager.update_config(config);
    Ok(true)
}

#[tauri::command]
pub async fn pi_run_session(
    manager: State<'_, PiManager>,
    session_id: &str,
    prompt: &str,
    workdir: &str
) -> Result<bool, String> {
    manager.run(session_id, prompt, workdir).await
        .map_err(|e| e.to_string())?;
    Ok(true)
}

#[tauri::command]
pub async fn pi_wait_session(manager: State<'_, PiManager>, session_id: &str) -> Result<bool, String> {
    manager.wait(session_id).await
        .map_err(|e| e.to_string())?;
    Ok(true)
}

#[tauri::command]
pub async fn pi_kill_session(manager: State<'_, PiManager>, session_id: &str) -> Result<bool, String> {
    manager.kill(session_id).await;
    Ok(true)
}

#[tauri::command]
pub async fn pi_get_output(manager: State<'_, PiManager>, session_id: &str) -> Result<Vec<String>, String> {
    Ok(manager.output(session_id).await)
}
