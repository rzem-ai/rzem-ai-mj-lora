use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Save project data to a file
pub fn save_project(path: &str, data: &str) -> Result<()> {
    let path_obj = Path::new(path);

    // Create parent directories if they don't exist
    if let Some(parent) = path_obj.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    fs::write(path_obj, data)
        .with_context(|| format!("Failed to write project file: {}", path))?;

    Ok(())
}

/// Load project data from a file
pub fn load_project(path: &str) -> Result<String> {
    let path_obj = Path::new(path);

    if !path_obj.exists() {
        anyhow::bail!("Project file does not exist: {}", path);
    }

    let data = fs::read_to_string(path_obj)
        .with_context(|| format!("Failed to read project file: {}", path))?;

    // Validate it's valid JSON
    serde_json::from_str::<serde_json::Value>(&data)
        .context("Project file is not valid JSON")?;

    Ok(data)
}

/// Export data to a JSON file
pub fn export_json(path: &str, data: &str) -> Result<()> {
    save_project(path, data)
}

/// Export data to a Markdown file
pub fn export_markdown(path: &str, content: &str) -> Result<()> {
    let path_obj = Path::new(path);

    // Create parent directories if they don't exist
    if let Some(parent) = path_obj.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    fs::write(path_obj, content)
        .with_context(|| format!("Failed to write markdown file: {}", path))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_save_and_load_project() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");
        let file_path_str = file_path.to_str().unwrap();

        let test_data = r#"{"test": "data"}"#;

        // Save
        save_project(file_path_str, test_data).unwrap();

        // Load
        let loaded = load_project(file_path_str).unwrap();
        assert_eq!(loaded, test_data);
    }
}
