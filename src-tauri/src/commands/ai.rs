use crate::error::AppError;
use crate::storage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Config {
    api_key: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self { api_key: None }
    }
}

#[derive(Debug, Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    system: String,
    messages: Vec<ClaudeMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClaudeMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeResponse {
    content: Vec<ClaudeContent>,
}

#[derive(Debug, Deserialize)]
struct ClaudeContent {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

fn config_path() -> Result<std::path::PathBuf, AppError> {
    let dir = storage::data_dir()?;
    storage::ensure_dir(&dir)?;
    Ok(dir.join("config.json"))
}

#[tauri::command]
pub fn get_api_key() -> Result<String, AppError> {
    let path = config_path()?;
    let config: Config = storage::read_json(&path)?.unwrap_or_default();
    Ok(config.api_key.unwrap_or_default())
}

#[tauri::command]
pub fn save_api_key(api_key: String) -> Result<(), AppError> {
    let path = config_path()?;
    let config = Config {
        api_key: Some(api_key),
    };
    storage::write_json(&path, &config)
}

#[tauri::command]
pub async fn chat(
    system_prompt: String,
    messages: Vec<ChatMessage>,
) -> Result<String, AppError> {
    let path = config_path()?;
    let config: Config = storage::read_json(&path)?.unwrap_or_default();
    let api_key = config
        .api_key
        .filter(|k| !k.is_empty())
        .ok_or_else(|| AppError::Custom("API Key not configured. Please set your Claude API key in settings.".into()))?;

    let claude_messages: Vec<ClaudeMessage> = messages
        .into_iter()
        .map(|m| ClaudeMessage {
            role: if m.role == "philosopher" {
                "assistant".into()
            } else {
                m.role
            },
            content: m.content,
        })
        .collect();

    let request = ClaudeRequest {
        model: "claude-sonnet-4-20250514".into(),
        max_tokens: 4096,
        system: system_prompt,
        messages: claude_messages,
    };

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(AppError::Custom(format!(
            "Claude API error ({}): {}",
            status, body
        )));
    }

    let claude_response: ClaudeResponse = response.json().await?;
    let text = claude_response
        .content
        .first()
        .map(|c| c.text.clone())
        .unwrap_or_default();

    Ok(text)
}
