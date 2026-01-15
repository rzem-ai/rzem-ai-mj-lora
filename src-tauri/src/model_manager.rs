use crate::settings::ModelVariant;
use anyhow::{Context, Result};
use std::path::PathBuf;

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
            ModelVariant::Qwen2VL2B => Self {
                variant,
                hf_repo: "Qwen/Qwen2-VL-2B-Instruct".to_string(),
                // TODO: This is a simplified file list for stub implementation.
                // Real Qwen2-VL models require additional files (preprocessor_config.json,
                // merges.txt, vocab.json, etc.). Update this when implementing real model
                // loading in Task 7.
                files: vec![
                    "model.safetensors".to_string(),
                    "config.json".to_string(),
                    "tokenizer.json".to_string(),
                    "tokenizer_config.json".to_string(),
                ],
                total_size_bytes: 4_500_000_000, // ~4.5 GB
            },
            ModelVariant::Qwen2VL7B => Self {
                variant,
                hf_repo: "Qwen/Qwen2-VL-7B-Instruct".to_string(),
                // TODO: This is a simplified file list for stub implementation.
                // Real Qwen2-VL models require additional files (preprocessor_config.json,
                // merges.txt, vocab.json, etc.). Update this when implementing real model
                // loading in Task 7.
                files: vec![
                    "model.safetensors".to_string(),
                    "config.json".to_string(),
                    "tokenizer.json".to_string(),
                    "tokenizer_config.json".to_string(),
                ],
                total_size_bytes: 15_000_000_000, // ~15 GB
            },
            ModelVariant::Qwen2VL72B => Self {
                variant,
                hf_repo: "Qwen/Qwen2-VL-72B-Instruct".to_string(),
                // TODO: This is a simplified file list for stub implementation.
                // Real Qwen2-VL models require additional files (preprocessor_config.json,
                // merges.txt, vocab.json, etc.). Update this when implementing real model
                // loading in Task 7.
                files: vec![
                    "model.safetensors".to_string(),
                    "config.json".to_string(),
                    "tokenizer.json".to_string(),
                    "tokenizer_config.json".to_string(),
                ],
                total_size_bytes: 146_000_000_000, // ~146 GB
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
        ModelVariant::Qwen2VL2B => "qwen2-vl-2b",
        ModelVariant::Qwen2VL7B => "qwen2-vl-7b",
        ModelVariant::Qwen2VL72B => "qwen2-vl-72b",
    };
    Ok(cache_dir.join(variant_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_config_2b() {
        let config = ModelConfig::from_variant(ModelVariant::Qwen2VL2B);
        assert_eq!(config.hf_repo, "Qwen/Qwen2-VL-2B-Instruct");
        assert_eq!(config.files.len(), 4);
        assert!(config.total_size_bytes > 0);
    }

    #[test]
    fn test_model_path_generation() {
        let path = get_model_path(ModelVariant::Qwen2VL2B, None).unwrap();
        assert!(path.to_string_lossy().contains("qwen2-vl-2b"));
    }
}
