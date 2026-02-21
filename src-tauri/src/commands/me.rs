use crate::error::AppError;
use crate::storage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Insight {
    pub id: String,
    pub content: String,
    pub source: String,
    pub dialogue_id: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyPhilosophy {
    pub name: String,
    pub manifesto: String,
    pub insights: Vec<Insight>,
    pub context: String,
    pub updated_at: String,
}

impl Default for MyPhilosophy {
    fn default() -> Self {
        Self {
            name: "我".into(),
            manifesto: String::new(),
            insights: Vec::new(),
            context: String::new(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

fn me_path() -> Result<std::path::PathBuf, AppError> {
    let dir = storage::data_dir()?;
    storage::ensure_dir(&dir)?;
    Ok(dir.join("me.json"))
}

#[tauri::command]
pub fn get_my_philosophy() -> Result<MyPhilosophy, AppError> {
    let path = me_path()?;
    Ok(storage::read_json(&path)?.unwrap_or_default())
}

#[tauri::command]
pub fn save_my_philosophy(data: MyPhilosophy) -> Result<(), AppError> {
    let path = me_path()?;
    storage::write_json(&path, &data)
}

#[tauri::command]
pub fn add_insight(insight: Insight) -> Result<(), AppError> {
    let path = me_path()?;
    let mut my_philosophy: MyPhilosophy = storage::read_json(&path)?.unwrap_or_default();
    my_philosophy.insights.push(insight);
    my_philosophy.updated_at = chrono::Utc::now().to_rfc3339();
    storage::write_json(&path, &my_philosophy)
}
