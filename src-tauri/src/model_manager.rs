use crate::settings::ModelVariant;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tauri::Emitter;
use thiserror::Error;

#[derive(Clone, Serialize)]
struct DownloadProgress {
    current_file: usize,
    total_files: usize,
    file_name: String,
    progress_percent: u8,
}

/// Errors that can occur during model operations
#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Model download failed: {0}")]
    DownloadFailed(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Error: {0}")]
    Other(#[from] anyhow::Error),
}

/// Status of a model on the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ModelStatus {
    NotDownloaded,
    Downloading { progress_percent: u8 },
    Ready,
    Error { message: String },
}

/// Configuration for a specific Qwen2-VL model variant
pub struct ModelConfig {
    pub variant: ModelVariant,
    pub hf_repo: String,
    pub files: Vec<String>,
    pub total_size_bytes: u64,
}

impl ModelConfig {
    /// Create a ModelConfig for the specified variant
    pub fn from_variant(variant: ModelVariant) -> Self {
        match variant {
            ModelVariant::Qwen3VL2B => Self {
                variant,
                hf_repo: "Qwen/Qwen3-VL-2B-Instruct-GGUF".to_string(),
                files: vec![
                    // Main model file (Q8_0 quantization for quality)
                    "Qwen3-VL-2B-Instruct-Q8_0.gguf".to_string(),
                    // Vision projection file (F16 for quality)
                    "mmproj-Qwen3-VL-2B-Instruct-F16.gguf".to_string(),
                ],
                total_size_bytes: 1_900_000_000, // ~1.9 GB
            },
            ModelVariant::Qwen3VL4B => Self {
                variant,
                hf_repo: "Qwen/Qwen3-VL-4B-Instruct-GGUF".to_string(),
                files: vec![
                    "Qwen3-VL-4B-Instruct-Q8_0.gguf".to_string(),
                    "mmproj-Qwen3-VL-4B-Instruct-F16.gguf".to_string(),
                ],
                total_size_bytes: 3_300_000_000, // ~3.3 GB
            },
            ModelVariant::Qwen3VL8B => Self {
                variant,
                hf_repo: "Qwen/Qwen3-VL-8B-Instruct-GGUF".to_string(),
                files: vec![
                    "Qwen3-VL-8B-Instruct-Q8_0.gguf".to_string(),
                    "mmproj-Qwen3-VL-8B-Instruct-F16.gguf".to_string(),
                ],
                total_size_bytes: 6_100_000_000, // ~6.1 GB
            },
        }
    }
}

/// Get the model cache directory, creating it if it doesn't exist
pub fn get_model_cache_dir(custom_dir: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(dir) = custom_dir {
        return Ok(dir);
    }

    let cache_dir = dirs::cache_dir()
        .context("Failed to get cache directory")?
        .join("rzem-mj-lora")
        .join("models");

    std::fs::create_dir_all(&cache_dir)
        .context("Failed to create model cache directory")?;

    Ok(cache_dir)
}

/// Get the full path for a specific model variant
pub fn get_model_path(variant: ModelVariant, custom_dir: Option<PathBuf>) -> Result<PathBuf> {
    let cache_dir = get_model_cache_dir(custom_dir)?;
    let variant_name = match variant {
        ModelVariant::Qwen3VL2B => "qwen3-vl-2b",
        ModelVariant::Qwen3VL4B => "qwen3-vl-4b",
        ModelVariant::Qwen3VL8B => "qwen3-vl-8b",
    };
    Ok(cache_dir.join(variant_name))
}

/// Check the status of a model variant on the system
pub fn check_model_status(variant: ModelVariant, custom_dir: Option<PathBuf>) -> ModelStatus {
    let model_path = match get_model_path(variant.clone(), custom_dir) {
        Ok(path) => path,
        Err(e) => {
            return ModelStatus::Error {
                message: format!("Failed to determine model path: {}", e),
            }
        }
    };

    // Check if model directory exists
    if !model_path.exists() {
        return ModelStatus::NotDownloaded;
    }

    // Check if all required files exist
    let config = ModelConfig::from_variant(variant);
    for file in &config.files {
        let file_path = model_path.join(file);
        if !file_path.exists() {
            return ModelStatus::Error {
                message: format!("Missing required file: {}", file),
            };
        }
    }

    ModelStatus::Ready
}

/// Download a model from Hugging Face
pub async fn download_model(
    variant: ModelVariant,
    custom_dir: Option<PathBuf>,
    app: tauri::AppHandle,
) -> std::result::Result<(), ModelError> {
    let model_path = get_model_path(variant.clone(), custom_dir.clone())?;
    let config = ModelConfig::from_variant(variant.clone());

    // Create model directory
    std::fs::create_dir_all(&model_path)?;

    log::info!(
        "Downloading model {:?} from {} to {:?}",
        variant,
        config.hf_repo,
        model_path
    );

    let total_files = config.files.len();

    // Run the synchronous download in a blocking task to avoid blocking the async runtime
    tokio::task::spawn_blocking(move || {
        // Initialize HF Hub API with proper configuration
        let api = hf_hub::api::sync::ApiBuilder::new()
            .with_progress(true)
            .build()
            .map_err(|e| ModelError::DownloadFailed(format!("Failed to initialize HF Hub API: {}", e)))?;

        let repo = api.model(config.hf_repo.clone());

        // Download each required file
        for (index, file) in config.files.iter().enumerate() {
            let current_file = index + 1;
            log::info!("Downloading file {}/{}: {}", current_file, total_files, file);

            // Emit progress event at start of file
            let _ = app.emit(
                "download-progress",
                DownloadProgress {
                    current_file,
                    total_files,
                    file_name: file.clone(),
                    progress_percent: ((current_file as f32 / total_files as f32) * 100.0) as u8,
                },
            );

            let downloaded_path = repo
                .get(file)
                .map_err(|e| ModelError::DownloadFailed(format!("Failed to download {}: {}", file, e)))?;

            // Copy the downloaded file to our model directory
            let target_path = model_path.join(file);
            std::fs::copy(&downloaded_path, &target_path)
                .map_err(|e| ModelError::DownloadFailed(format!("Failed to copy {} to model directory: {}", file, e)))?;

            log::info!("Successfully downloaded: {}", file);
        }

        // Emit 100% completion
        let _ = app.emit(
            "download-progress",
            DownloadProgress {
                current_file: total_files,
                total_files,
                file_name: "Complete".to_string(),
                progress_percent: 100,
            },
        );

        log::info!("Model download complete: {:?}", variant);
        Ok::<(), ModelError>(())
    })
    .await
    .map_err(|e| ModelError::DownloadFailed(format!("Download task failed: {}", e)))??;

    Ok(())
}

/// Helper function to calculate directory size recursively
fn dir_size(path: &Path) -> std::io::Result<u64> {
    let mut total = 0;

    if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let metadata = entry.metadata()?;

            if metadata.is_dir() {
                total += dir_size(&entry.path())?;
            } else {
                total += metadata.len();
            }
        }
    } else if path.is_file() {
        total = path.metadata()?.len();
    }

    Ok(total)
}

/// Clear the model cache and return the number of bytes freed
pub fn clear_model_cache(custom_dir: Option<PathBuf>) -> std::result::Result<u64, ModelError> {
    let cache_dir = get_model_cache_dir(custom_dir)?;

    if !cache_dir.exists() {
        return Ok(0);
    }

    // Calculate size before removal
    let bytes_freed = dir_size(&cache_dir)?;

    // Remove the entire cache directory
    std::fs::remove_dir_all(&cache_dir)?;

    log::info!(
        "Cleared model cache at {:?}, freed {} bytes",
        cache_dir,
        bytes_freed
    );

    Ok(bytes_freed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_config_2b() {
        let config = ModelConfig::from_variant(ModelVariant::Qwen3VL2B);
        assert_eq!(config.hf_repo, "Qwen/Qwen3-VL-2B-Instruct-GGUF");
        assert_eq!(config.files.len(), 2); // GGUF model + mmproj
        assert!(config.total_size_bytes > 0);
    }

    #[test]
    fn test_model_path_generation() {
        let path = get_model_path(ModelVariant::Qwen3VL2B, None).unwrap();
        assert!(path.to_string_lossy().contains("qwen3-vl-2b"));
    }

    #[test]
    fn test_model_status_not_downloaded() {
        use tempfile::TempDir;

        // Create a temporary directory that won't contain a model
        let temp_dir = TempDir::new().unwrap();
        let custom_dir = Some(temp_dir.path().to_path_buf());

        let status = check_model_status(ModelVariant::Qwen3VL2B, custom_dir);
        assert_eq!(status, ModelStatus::NotDownloaded);
    }
}
