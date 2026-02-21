use crate::error::AppError;
use std::path::PathBuf;

/// Get the app data directory: ~/.cc-philo-talk/
pub fn data_dir() -> Result<PathBuf, AppError> {
    let home = dirs::home_dir().ok_or_else(|| AppError::Custom("Cannot find home directory".into()))?;
    let dir = home.join(".cc-philo-talk");
    Ok(dir)
}

/// Ensure a directory exists, creating it if necessary
pub fn ensure_dir(path: &PathBuf) -> Result<(), AppError> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Read a JSON file, returning None if it doesn't exist
pub fn read_json<T: serde::de::DeserializeOwned>(path: &PathBuf) -> Result<Option<T>, AppError> {
    if !path.exists() {
        return Ok(None);
    }
    let content = std::fs::read_to_string(path)?;
    let data = serde_json::from_str(&content)?;
    Ok(Some(data))
}

/// Write data as JSON to a file
pub fn write_json<T: serde::Serialize>(path: &PathBuf, data: &T) -> Result<(), AppError> {
    if let Some(parent) = path.parent() {
        ensure_dir(&parent.to_path_buf())?;
    }
    let content = serde_json::to_string_pretty(data)?;
    std::fs::write(path, content)?;
    Ok(())
}

/// Delete a file if it exists
pub fn delete_file(path: &PathBuf) -> Result<(), AppError> {
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

/// List all JSON files in a directory
pub fn list_json_files(dir: &PathBuf) -> Result<Vec<PathBuf>, AppError> {
    ensure_dir(dir)?;
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "json") {
            files.push(path);
        }
    }
    Ok(files)
}
