use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Analysis mode selection
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AnalysisMode {
    /// Use Claude API for analysis
    CloudAPI,
    /// Use local Qwen2-VL model
    Offline,
    /// Automatically choose based on availability
    Auto,
}

/// Available Qwen2-VL model variants
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ModelVariant {
    /// 2B parameter model (fastest)
    Qwen2VL2B,
    /// 7B parameter model (balanced)
    Qwen2VL7B,
    /// 72B parameter model (highest quality)
    Qwen2VL72B,
}

/// Application settings for analysis modes
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    /// Current analysis mode
    pub analysis_mode: AnalysisMode,
    /// Selected offline model variant
    pub offline_model_variant: ModelVariant,
    /// Optional custom directory for model cache
    pub model_cache_dir: Option<PathBuf>,
    /// Whether to fallback to cloud API if offline fails
    pub auto_fallback: bool,
    /// Whether to keep model loaded in memory between analyses
    pub keep_model_loaded: bool,
}

fn get_config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Failed to get config directory")?
        .join("rzem-mj-lora");

    fs::create_dir_all(&config_dir)
        .context("Failed to create config directory")?;

    Ok(config_dir)
}

fn get_settings_path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("settings.json"))
}

pub fn load_settings() -> Result<AppSettings> {
    let path = get_settings_path()?;

    if !path.exists() {
        return Ok(AppSettings::default());
    }

    let content = fs::read_to_string(&path)
        .context("Failed to read settings file")?;

    let settings: AppSettings = serde_json::from_str(&content)
        .context("Failed to parse settings JSON")?;

    Ok(settings)
}

pub fn save_settings(settings: &AppSettings) -> Result<()> {
    let path = get_settings_path()?;

    let json = serde_json::to_string_pretty(settings)
        .context("Failed to serialize settings")?;

    fs::write(&path, json)
        .context("Failed to write settings file")?;

    Ok(())
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            analysis_mode: AnalysisMode::Auto,
            offline_model_variant: ModelVariant::Qwen2VL2B,
            model_cache_dir: None,
            auto_fallback: true,
            keep_model_loaded: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = AppSettings::default();

        assert_eq!(settings.analysis_mode, AnalysisMode::Auto);
        assert_eq!(settings.offline_model_variant, ModelVariant::Qwen2VL2B);
        assert_eq!(settings.model_cache_dir, None);
        assert_eq!(settings.auto_fallback, true);
        assert_eq!(settings.keep_model_loaded, true);
    }

    #[test]
    fn test_save_and_load_settings() {
        // Test serialization round-trip
        let mut settings = AppSettings::default();
        settings.analysis_mode = AnalysisMode::Offline;

        let json = serde_json::to_string(&settings).unwrap();
        let loaded: AppSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(loaded.analysis_mode, AnalysisMode::Offline);
    }
}
