use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use crate::todo::Todo;

fn data_file() -> PathBuf {
    let dir = dirs::config_dir()
        .expect("Cannot determine config directory")
        .join("tasky");
    fs::create_dir_all(&dir).expect("Cannot create config directory");
    dir.join("todos.json")
}

pub fn load_todos() -> Result<Vec<Todo>> {
    let path = data_file();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(&path).context("Failed to read todos file")?;
    let todos: Vec<Todo> = serde_json::from_str(&content).unwrap_or_default();
    Ok(todos)
}

pub fn save_todos(todos: &[Todo]) -> Result<()> {
    let path = data_file();
    let json = serde_json::to_string_pretty(todos).context("Failed to serialize todos")?;
    fs::write(&path, json).context("Failed to write todos file")?;
    Ok(())
}
