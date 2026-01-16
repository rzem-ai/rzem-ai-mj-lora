use crate::candle_inference::{Qwen2VLInference, build_qwen_prompt};
use crate::model_manager::{check_model_status, get_model_path, ModelStatus};
use crate::settings::AppSettings;
use anyhow::Result;
use image::DynamicImage;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OfflineAnalysisError {
    #[error("Model not found. Please download the model first.")]
    ModelNotFound,

    #[error("Insufficient memory. Requires {required}GB, available {available}GB")]
    InsufficientMemory { required: f32, available: f32 },

    #[error("Model loading failed: {0}")]
    ModelLoadError(String),

    #[error("Inference failed: {0}")]
    InferenceFailed(String),

    #[error("Image processing failed: {0}")]
    ImageProcessingError(String),
}

fn get_available_memory_gb() -> f32 {
    use sysinfo::System;
    let mut sys = System::new_all();
    sys.refresh_memory();
    sys.available_memory() as f32 / 1_073_741_824.0 // bytes to GB
}

pub fn check_system_requirements(settings: &AppSettings) -> Result<(), OfflineAnalysisError> {
    use crate::settings::ModelVariant;

    let required_gb = match settings.offline_model_variant {
        ModelVariant::Qwen3VL2B => 3.0,   // ~1.9GB model + overhead
        ModelVariant::Qwen3VL4B => 5.0,   // ~3.3GB model + overhead
        ModelVariant::Qwen3VL8B => 10.0,  // ~6.1GB model + overhead
    };

    let available = get_available_memory_gb();
    if available < required_gb {
        return Err(OfflineAnalysisError::InsufficientMemory {
            required: required_gb,
            available,
        });
    }

    Ok(())
}

fn load_images(image_paths: &[String]) -> Result<Vec<DynamicImage>, OfflineAnalysisError> {
    let mut images = Vec::new();

    for path in image_paths {
        let img = image::open(path)
            .map_err(|e| OfflineAnalysisError::ImageProcessingError(e.to_string()))?;
        images.push(img);
    }

    Ok(images)
}

pub async fn analyze_style(
    image_paths: Vec<String>,
    sref_code: &str,
    settings: &AppSettings,
) -> Result<String, OfflineAnalysisError> {
    // 1. Check system requirements
    check_system_requirements(settings)?;

    // 2. Verify model is available
    let model_status = check_model_status(
        settings.offline_model_variant.clone(),
        settings.model_cache_dir.clone(),
    );

    if !matches!(model_status, ModelStatus::Ready) {
        return Err(OfflineAnalysisError::ModelNotFound);
    }

    // 3. Get model path
    let model_path = get_model_path(
        settings.offline_model_variant.clone(),
        settings.model_cache_dir.clone(),
    )
    .map_err(|e| OfflineAnalysisError::ModelLoadError(e.to_string()))?;

    // 4. Load images
    let images = load_images(&image_paths)?;

    // 5. Build prompt
    let prompt = build_qwen_prompt(sref_code, images.len());

    // 6. Load model and run inference
    let mut inference = Qwen2VLInference::new(&model_path, settings.offline_model_variant.clone())
        .await
        .map_err(|e| OfflineAnalysisError::ModelLoadError(e.to_string()))?;

    let response = inference
        .analyze_images(images, &prompt)
        .map_err(|e| OfflineAnalysisError::InferenceFailed(e.to_string()))?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_system_requirements() {
        let settings = AppSettings::default();
        // Should pass on most systems (requires 4GB for 2B model)
        let result = check_system_requirements(&settings);
        // Can't assert pass/fail as depends on system
        println!("System check result: {:?}", result);
    }
}
