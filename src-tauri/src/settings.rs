use serde::{Deserialize, Serialize};
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
}
