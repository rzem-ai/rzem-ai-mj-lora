mod claude;
mod file_ops;
mod image_utils;

use tauri::command;

#[command]
async fn analyze_style(image_paths: Vec<String>, sref_code: String) -> Result<String, String> {
    // Read and encode all images
    let mut image_data: Vec<(String, String)> = Vec::new();

    for path in &image_paths {
        let base64_data = image_utils::read_and_encode_image(path)
            .map_err(|e| format!("Failed to read image {}: {}", path, e))?;

        let mime_type = image_utils::get_mime_type(path)
            .map_err(|e| format!("Invalid image format {}: {}", path, e))?;

        image_data.push((base64_data, mime_type));
    }

    // Call Claude API
    let result = claude::analyze_style(image_data, &sref_code)
        .await
        .map_err(|e| format!("Claude API error: {}", e))?;

    Ok(result)
}

#[command]
fn save_project(path: String, data: String) -> Result<(), String> {
    file_ops::save_project(&path, &data)
        .map_err(|e| format!("Failed to save project: {}", e))
}

#[command]
fn load_project(path: String) -> Result<String, String> {
    file_ops::load_project(&path)
        .map_err(|e| format!("Failed to load project: {}", e))
}

#[command]
fn export_json(path: String, data: String) -> Result<(), String> {
    file_ops::export_json(&path, &data)
        .map_err(|e| format!("Failed to export JSON: {}", e))
}

#[command]
fn export_markdown(path: String, content: String) -> Result<(), String> {
    file_ops::export_markdown(&path, &content)
        .map_err(|e| format!("Failed to export Markdown: {}", e))
}

#[command]
fn validate_image(path: String) -> bool {
    image_utils::is_valid_image(&path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            analyze_style,
            save_project,
            load_project,
            export_json,
            export_markdown,
            validate_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
