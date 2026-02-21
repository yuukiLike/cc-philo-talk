use crate::error::AppError;
use crate::storage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: String,
    pub role: String,
    pub content: String,
    #[serde(default)]
    pub saved_as_insight: bool,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dialogue {
    pub id: String,
    pub philosopher_id: String,
    pub messages: Vec<Message>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DialogueSummary {
    pub id: String,
    pub philosopher_id: String,
    pub message_count: usize,
    pub last_message: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

fn dialogues_dir() -> Result<std::path::PathBuf, AppError> {
    let dir = storage::data_dir()?.join("dialogues");
    storage::ensure_dir(&dir)?;
    Ok(dir)
}

#[tauri::command]
pub fn list_dialogues(philosopher_id: Option<String>) -> Result<Vec<DialogueSummary>, AppError> {
    let dir = dialogues_dir()?;
    let files = storage::list_json_files(&dir)?;
    let mut summaries = Vec::new();
    for file in files {
        if let Some(d) = storage::read_json::<Dialogue>(&file)? {
            if let Some(ref pid) = philosopher_id {
                if &d.philosopher_id != pid {
                    continue;
                }
            }
            summaries.push(DialogueSummary {
                id: d.id,
                philosopher_id: d.philosopher_id,
                message_count: d.messages.len(),
                last_message: d.messages.last().map(|m| {
                    if m.content.len() > 100 {
                        format!("{}...", &m.content[..100])
                    } else {
                        m.content.clone()
                    }
                }),
                created_at: d.created_at,
                updated_at: d.updated_at,
            });
        }
    }
    summaries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(summaries)
}

#[tauri::command]
pub fn get_dialogue(id: String) -> Result<Option<Dialogue>, AppError> {
    let path = dialogues_dir()?.join(format!("{}.json", id));
    storage::read_json(&path)
}

#[tauri::command]
pub fn save_dialogue(dialogue: Dialogue) -> Result<(), AppError> {
    let path = dialogues_dir()?.join(format!("{}.json", dialogue.id));
    storage::write_json(&path, &dialogue)
}
