mod candle_inference;
mod claude;
mod file_ops;
mod image_utils;
mod model_manager;
mod offline_analyzer;
mod settings;

use serde::Serialize;
use tauri::command;

#[derive(Serialize)]
struct AnalysisResult {
    data: String,
    mode_used: String,
    fallback_used: bool,
}

#[command]
async fn analyze_style(image_paths: Vec<String>, sref_code: String) -> Result<AnalysisResult, String> {
    let settings = settings::load_settings().unwrap_or_default();

    // Determine which mode to use
    let use_api = match settings.analysis_mode {
        settings::AnalysisMode::CloudAPI => {
            std::env::var("CLAUDE_API_KEY").is_ok() || std::env::var("ANTHROPIC_API_KEY").is_ok()
        }
        settings::AnalysisMode::Offline => false,
        settings::AnalysisMode::Auto => {
            std::env::var("CLAUDE_API_KEY").is_ok() || std::env::var("ANTHROPIC_API_KEY").is_ok()
        }
    };

    // Try primary mode
    if use_api {
        // Try Claude API
        let image_data: Vec<(String, String)> = image_paths
            .iter()
            .map(|path| {
                let base64_data = image_utils::read_and_encode_image(path)
                    .map_err(|e| format!("Failed to read image {}: {}", path, e))?;
                let mime_type = image_utils::get_mime_type(path)
                    .map_err(|e| format!("Invalid image format {}: {}", path, e))?;
                Ok((base64_data, mime_type))
            })
            .collect::<Result<Vec<_>, String>>()?;

        match claude::analyze_style(image_data, &sref_code).await {
            Ok(result) => {
                return Ok(AnalysisResult {
                    data: result,
                    mode_used: "cloud".to_string(),
                    fallback_used: false,
                });
            }
            Err(e) if settings.auto_fallback => {
                log::warn!("API analysis failed: {}. Attempting offline fallback...", e);
                // Fall through to offline mode
            }
            Err(e) => {
                return Err(format!("Claude API error: {}", e));
            }
        }
    }

    // Use offline mode (either primary or fallback)
    match offline_analyzer::analyze_style(image_paths, &sref_code, &settings).await {
        Ok(result) => Ok(AnalysisResult {
            data: result,
            mode_used: "offline".to_string(),
            fallback_used: use_api, // true if we tried API first
        }),
        Err(e) => Err(format!("Offline analysis error: {}", e)),
    }
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

#[command]
fn get_settings() -> Result<settings::AppSettings, String> {
    settings::load_settings()
        .map_err(|e| format!("Failed to load settings: {}", e))
}

#[command]
fn update_settings(settings: settings::AppSettings) -> Result<(), String> {
    settings::save_settings(&settings)
        .map_err(|e| format!("Failed to save settings: {}", e))
}

#[command]
fn get_model_status(variant: settings::ModelVariant) -> model_manager::ModelStatus {
    let settings = settings::load_settings().unwrap_or_default();
    model_manager::check_model_status(variant, settings.model_cache_dir)
}

#[command]
async fn download_model(variant: settings::ModelVariant) -> Result<(), String> {
    let settings = settings::load_settings().unwrap_or_default();
    model_manager::download_model(variant, settings.model_cache_dir)
        .await
        .map_err(|e| format!("Failed to download model: {}", e))
}

#[command]
fn clear_model_cache() -> Result<u64, String> {
    let settings = settings::load_settings().unwrap_or_default();
    model_manager::clear_model_cache(settings.model_cache_dir)
        .map_err(|e| format!("Failed to clear cache: {}", e))
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
            validate_image,
            get_settings,
            update_settings,
            get_model_status,
            download_model,
            clear_model_cache
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
