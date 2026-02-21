use crate::error::AppError;
use crate::storage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Philosopher {
    pub id: String,
    pub name: String,
    pub era: String,
    pub core_ideas: Vec<String>,
    pub context: String,
    pub created_at: String,
    pub updated_at: String,
}

fn philosophers_dir() -> Result<std::path::PathBuf, AppError> {
    let dir = storage::data_dir()?.join("philosophers");
    storage::ensure_dir(&dir)?;
    Ok(dir)
}

#[tauri::command]
pub fn list_philosophers() -> Result<Vec<Philosopher>, AppError> {
    let dir = philosophers_dir()?;
    let files = storage::list_json_files(&dir)?;
    let mut philosophers = Vec::new();
    for file in files {
        if let Some(p) = storage::read_json::<Philosopher>(&file)? {
            philosophers.push(p);
        }
    }
    philosophers.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(philosophers)
}

#[tauri::command]
pub fn get_philosopher(id: String) -> Result<Option<Philosopher>, AppError> {
    let path = philosophers_dir()?.join(format!("{}.json", id));
    storage::read_json(&path)
}

#[tauri::command]
pub fn save_philosopher(philosopher: Philosopher) -> Result<(), AppError> {
    let path = philosophers_dir()?.join(format!("{}.json", philosopher.id));
    storage::write_json(&path, &philosopher)
}

#[tauri::command]
pub fn delete_philosopher(id: String) -> Result<(), AppError> {
    let path = philosophers_dir()?.join(format!("{}.json", id));
    storage::delete_file(&path)
}
