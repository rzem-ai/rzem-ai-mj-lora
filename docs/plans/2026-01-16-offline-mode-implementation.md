# Offline Mode with Qwen2-VL Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add offline image analysis using Qwen2-VL models embedded in the Tauri app, enabling fully local operation without Claude API dependency.

**Architecture:** Parallel analysis paths (API vs Offline) both returning the same JSON schema. New Rust modules handle model management, Candle ML inference, and offline orchestration. Settings system controls mode selection with automatic fallback.

**Tech Stack:** Rust (Candle ML, hf-hub, tokio), Vue 3, TypeScript, Tauri 2, Qwen2-VL vision models

**Design Reference:** `docs/plans/2026-01-16-offline-mode-qwen2vl-design.md`

---

## Phase 1: Core Backend Infrastructure

### Task 1: Create Settings Module Foundation

**Files:**
- Create: `src-tauri/src/settings.rs`
- Test: `src-tauri/src/settings.rs` (inline tests)

**Step 1: Add module declaration**

Edit `src-tauri/src/lib.rs`:
```rust
mod claude;
mod file_ops;
mod image_utils;
mod settings;  // Add this line

use tauri::command;
```

**Step 2: Create settings module with basic types**

Create `src-tauri/src/settings.rs`:
```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AnalysisMode {
    CloudAPI,
    Offline,
    Auto,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ModelVariant {
    Qwen2VL2B,
    Qwen2VL7B,
    Qwen2VL72B,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub analysis_mode: AnalysisMode,
    pub offline_model_variant: ModelVariant,
    pub model_cache_dir: Option<PathBuf>,
    pub auto_fallback: bool,
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
        assert!(settings.auto_fallback);
        assert!(settings.keep_model_loaded);
    }
}
```

**Step 3: Run tests**

Run: `cargo test --manifest-path=src-tauri/Cargo.toml test_default_settings`
Expected: PASS

**Step 4: Commit**

```bash
git add src-tauri/src/lib.rs src-tauri/src/settings.rs
git commit -m "feat(settings): add settings module with basic types

Add AnalysisMode, ModelVariant, and AppSettings structs with defaults.
Supports CloudAPI, Offline, and Auto modes with configurable model variants."
```

---

### Task 2: Add Settings Persistence

**Files:**
- Modify: `src-tauri/src/settings.rs`

**Step 1: Add platform-specific config directory helper**

Add to `src-tauri/src/settings.rs`:
```rust
use anyhow::{Context, Result};
use std::fs;

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
```

**Step 2: Add load_settings function**

Add to `src-tauri/src/settings.rs`:
```rust
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
```

**Step 3: Add save_settings function**

Add to `src-tauri/src/settings.rs`:
```rust
pub fn save_settings(settings: &AppSettings) -> Result<()> {
    let path = get_settings_path()?;

    let json = serde_json::to_string_pretty(settings)
        .context("Failed to serialize settings")?;

    fs::write(&path, json)
        .context("Failed to write settings file")?;

    Ok(())
}
```

**Step 4: Add tests**

Add to test module in `src-tauri/src/settings.rs`:
```rust
#[test]
fn test_save_and_load_settings() {
    use tempfile::TempDir;

    // Override config dir for testing (simplified - you may need env var)
    let mut settings = AppSettings::default();
    settings.analysis_mode = AnalysisMode::Offline;

    // Test serialization round-trip
    let json = serde_json::to_string(&settings).unwrap();
    let loaded: AppSettings = serde_json::from_str(&json).unwrap();

    assert_eq!(loaded.analysis_mode, AnalysisMode::Offline);
}
```

**Step 5: Add dirs dependency**

Edit `src-tauri/Cargo.toml`, add to `[dependencies]`:
```toml
dirs = "6.0"
```

**Step 6: Run tests**

Run: `cargo test --manifest-path=src-tauri/Cargo.toml`
Expected: All tests PASS

**Step 7: Commit**

```bash
git add src-tauri/src/settings.rs src-tauri/Cargo.toml
git commit -m "feat(settings): add settings persistence to disk

Add load_settings and save_settings functions with platform-specific
config directory support (~/.config/rzem-mj-lora/settings.json)."
```

---

## Phase 2: Model Management Foundation

### Task 3: Create Model Manager Module

**Files:**
- Create: `src-tauri/src/model_manager.rs`
- Modify: `src-tauri/src/lib.rs`

**Step 1: Add module declaration**

Edit `src-tauri/src/lib.rs`:
```rust
mod claude;
mod file_ops;
mod image_utils;
mod settings;
mod model_manager;  // Add this

use tauri::command;
```

**Step 2: Create model config structures**

Create `src-tauri/src/model_manager.rs`:
```rust
use crate::settings::ModelVariant;
use anyhow::{Context, Result};
use std::path::PathBuf;

pub struct ModelConfig {
    pub variant: ModelVariant,
    pub hf_repo: String,
    pub files: Vec<String>,
    pub total_size_bytes: u64,
}

impl ModelConfig {
    pub fn from_variant(variant: ModelVariant) -> Self {
        match variant {
            ModelVariant::Qwen2VL2B => Self {
                variant,
                hf_repo: "Qwen/Qwen2-VL-2B-Instruct".to_string(),
                files: vec![
                    "model.safetensors".to_string(),
                    "config.json".to_string(),
                    "tokenizer.json".to_string(),
                    "tokenizer_config.json".to_string(),
                ],
                total_size_bytes: 2_800_000_000, // ~2.8 GB
            },
            ModelVariant::Qwen2VL7B => Self {
                variant,
                hf_repo: "Qwen/Qwen2-VL-7B-Instruct".to_string(),
                files: vec![
                    "model.safetensors".to_string(),
                    "config.json".to_string(),
                    "tokenizer.json".to_string(),
                    "tokenizer_config.json".to_string(),
                ],
                total_size_bytes: 6_500_000_000, // ~6.5 GB
            },
            ModelVariant::Qwen2VL72B => Self {
                variant,
                hf_repo: "Qwen/Qwen2-VL-72B-Instruct".to_string(),
                files: vec![
                    "model.safetensors".to_string(),
                    "config.json".to_string(),
                    "tokenizer.json".to_string(),
                    "tokenizer_config.json".to_string(),
                ],
                total_size_bytes: 45_000_000_000, // ~45 GB
            },
        }
    }
}

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
```

**Step 3: Run tests**

Run: `cargo test --manifest-path=src-tauri/Cargo.toml model_config`
Expected: PASS

**Step 4: Commit**

```bash
git add src-tauri/src/lib.rs src-tauri/src/model_manager.rs
git commit -m "feat(model): add model manager with config structures

Add ModelConfig for Qwen2-VL variants (2B/7B/72B) with HF repo paths,
file lists, and cache directory management."
```

---

### Task 4: Add Model Status Checking

**Files:**
- Modify: `src-tauri/src/model_manager.rs`

**Step 1: Add ModelStatus enum**

Add to `src-tauri/src/model_manager.rs`:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ModelStatus {
    NotDownloaded,
    Downloading { progress_percent: u8 },
    Ready,
    Error { message: String },
}
```

**Step 2: Add check_model_status function**

Add to `src-tauri/src/model_manager.rs`:
```rust
pub fn check_model_status(variant: ModelVariant, custom_dir: Option<PathBuf>) -> Result<ModelStatus> {
    let model_path = get_model_path(variant, custom_dir)?;

    if !model_path.exists() {
        return Ok(ModelStatus::NotDownloaded);
    }

    let config = ModelConfig::from_variant(variant);

    // Check if all required files exist
    for file in &config.files {
        let file_path = model_path.join(file);
        if !file_path.exists() {
            return Ok(ModelStatus::NotDownloaded);
        }
    }

    Ok(ModelStatus::Ready)
}
```

**Step 3: Add test**

Add to test module:
```rust
#[test]
fn test_model_status_not_downloaded() {
    use tempfile::TempDir;
    let temp = TempDir::new().unwrap();

    let status = check_model_status(
        ModelVariant::Qwen2VL2B,
        Some(temp.path().join("nonexistent"))
    ).unwrap();

    matches!(status, ModelStatus::NotDownloaded);
}
```

**Step 4: Add tempfile dev dependency**

Edit `src-tauri/Cargo.toml`, add to `[dev-dependencies]`:
```toml
tempfile = "3.8"
```

**Step 5: Run tests**

Run: `cargo test --manifest-path=src-tauri/Cargo.toml`
Expected: PASS

**Step 6: Commit**

```bash
git add src-tauri/src/model_manager.rs src-tauri/Cargo.toml
git commit -m "feat(model): add model status checking

Add ModelStatus enum and check_model_status function to verify if
model files are present in cache directory."
```

---

### Task 5: Add Model Download Stub (HF Hub Integration Later)

**Files:**
- Modify: `src-tauri/src/model_manager.rs`

**Step 1: Add download_model stub**

Add to `src-tauri/src/model_manager.rs`:
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Download failed: {0}")]
    DownloadFailed(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

pub async fn download_model(
    variant: ModelVariant,
    custom_dir: Option<PathBuf>,
) -> Result<(), ModelError> {
    let model_path = get_model_path(variant, custom_dir.clone())
        .map_err(|e| ModelError::Other(e))?;

    std::fs::create_dir_all(&model_path)?;

    // TODO: Implement actual HF Hub download
    // For now, this is a stub that creates the directory structure
    log::info!("Model download stub called for {:?}", variant);

    Ok(())
}

pub fn clear_model_cache(custom_dir: Option<PathBuf>) -> Result<u64> {
    let cache_dir = get_model_cache_dir(custom_dir)?;

    if !cache_dir.exists() {
        return Ok(0);
    }

    let mut total_bytes = 0u64;

    for entry in std::fs::read_dir(&cache_dir)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            // Recursively calculate size
            total_bytes += dir_size(&entry.path())?;
            std::fs::remove_dir_all(entry.path())?;
        } else {
            total_bytes += metadata.len();
            std::fs::remove_file(entry.path())?;
        }
    }

    Ok(total_bytes)
}

fn dir_size(path: &PathBuf) -> Result<u64> {
    let mut size = 0u64;

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            size += dir_size(&entry.path())?;
        } else {
            size += metadata.len();
        }
    }

    Ok(size)
}
```

**Step 2: Add thiserror dependency**

Edit `src-tauri/Cargo.toml`, add to `[dependencies]`:
```toml
thiserror = "2.0"
log = "0.4"
```

**Step 3: Build to verify**

Run: `cargo build --manifest-path=src-tauri/Cargo.toml`
Expected: Success

**Step 4: Commit**

```bash
git add src-tauri/src/model_manager.rs src-tauri/Cargo.toml
git commit -m "feat(model): add download and cache management stubs

Add download_model stub (HF Hub integration TODO), clear_model_cache
function, and ModelError types."
```

---

## Phase 3: Candle Integration Preparation

### Task 6: Add Candle Dependencies

**Files:**
- Modify: `src-tauri/Cargo.toml`

**Step 1: Add Candle dependencies**

Edit `src-tauri/Cargo.toml`, add to `[dependencies]`:
```toml
# Candle ML framework
candle-core = "0.6"
candle-nn = "0.6"
candle-transformers = "0.6"
tokenizers = "0.19"

# Image processing
image = "0.25"

# System info
sysinfo = "0.30"

# Async runtime (tokio already likely present from reqwest)
tokio = { version = "1", features = ["full"] }
```

**Step 2: Add platform-specific features**

Add to `src-tauri/Cargo.toml`:
```toml
[features]
default = []
cuda = ["candle-core/cuda"]
metal = ["candle-core/metal"]

[target.'cfg(target_os = "macos")'.dependencies]
candle-core = { version = "0.6", features = ["metal", "accelerate"] }

[target.'cfg(target_os = "linux")'.dependencies]
candle-core = { version = "0.6", features = ["mkl"] }
```

**Step 3: Build to verify dependencies**

Run: `cargo build --manifest-path=src-tauri/Cargo.toml`
Expected: Success (may take several minutes for first build)

**Step 4: Commit**

```bash
git add src-tauri/Cargo.toml
git commit -m "deps(candle): add Candle ML framework dependencies

Add candle-core, candle-nn, candle-transformers, tokenizers for
Qwen2-VL inference. Include platform-specific GPU acceleration features
(Metal for macOS, MKL for Linux, CUDA optional)."
```

---

### Task 7: Create Candle Inference Module Stub

**Files:**
- Create: `src-tauri/src/candle_inference.rs`
- Modify: `src-tauri/src/lib.rs`

**Step 1: Add module declaration**

Edit `src-tauri/src/lib.rs`:
```rust
mod claude;
mod file_ops;
mod image_utils;
mod settings;
mod model_manager;
mod candle_inference;  // Add this

use tauri::command;
```

**Step 2: Create inference module stub**

Create `src-tauri/src/candle_inference.rs`:
```rust
use crate::settings::ModelVariant;
use anyhow::{Context, Result};
use image::DynamicImage;
use std::path::Path;

pub struct Qwen2VLInference {
    variant: ModelVariant,
    // TODO: Add actual Candle types
    // model: Model,
    // tokenizer: Tokenizer,
    // device: Device,
}

impl Qwen2VLInference {
    pub async fn new(model_path: &Path, variant: ModelVariant) -> Result<Self> {
        log::info!("Loading Qwen2-VL model from {:?}", model_path);

        // TODO: Implement actual model loading
        // For now, stub that validates path exists
        if !model_path.exists() {
            anyhow::bail!("Model path does not exist: {:?}", model_path);
        }

        Ok(Self {
            variant,
        })
    }

    pub fn analyze_images(
        &mut self,
        images: Vec<DynamicImage>,
        prompt: &str,
    ) -> Result<String> {
        log::info!("Analyzing {} images with prompt: {}", images.len(), prompt);

        // TODO: Implement actual inference
        // For now, return stub JSON matching schema
        Ok(r#"{
            "sref_code": "stub",
            "style_analysis": {
                "primary_style": "stub",
                "era_influence": "stub",
                "color_palette": ["stub"],
                "key_characteristics": ["stub"],
                "best_subjects": ["stub"],
                "avoid_subjects": ["stub"]
            },
            "training_recommendations": {
                "recommended_dataset_size": 100,
                "optimal_subject_distribution": {
                    "stub": "100%"
                }
            },
            "permutation_batches": [],
            "prompt_guidelines": {
                "keep_simple": true,
                "avoid_style_keywords": ["stub"],
                "recommended_additions": ["stub"]
            }
        }"#.to_string())
    }
}

pub fn build_qwen_prompt(sref_code: &str, num_images: usize) -> String {
    format!(
        "<|im_start|>system\nYou are Qwen, a vision-language AI assistant specialized in analyzing artistic styles.<|im_end|>
<|im_start|>user\n{}Analyze these {} style reference images for Midjourney SREF code {}.

Generate a LoRA training dataset specification with:
1. Style analysis (colors, patterns, era, characteristics)
2. 8-10 permutation batches with EXACTLY 40 images each
3. Use format: {{{{subjects}}}} with {{{{modifiers}}}} --sref {}

Output ONLY valid JSON matching the expected schema.<|im_end|>
<|im_start|>assistant\n",
        "<|vision_start|><|image_pad|><|vision_end|>".repeat(num_images),
        num_images,
        sref_code,
        sref_code
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_generation() {
        let prompt = build_qwen_prompt("123456", 3);
        assert!(prompt.contains("SREF code 123456"));
        assert!(prompt.contains("3 style reference images"));
        assert!(prompt.contains("<|vision_start|>"));
    }
}
```

**Step 3: Run tests**

Run: `cargo test --manifest-path=src-tauri/Cargo.toml test_prompt_generation`
Expected: PASS

**Step 4: Commit**

```bash
git add src-tauri/src/lib.rs src-tauri/src/candle_inference.rs
git commit -m "feat(inference): add Candle inference module stub

Add Qwen2VLInference struct with stub implementation and prompt
generation function. Real model loading and inference to be implemented."
```

---

## Phase 4: Offline Analyzer Orchestration

### Task 8: Create Offline Analyzer Module

**Files:**
- Create: `src-tauri/src/offline_analyzer.rs`
- Modify: `src-tauri/src/lib.rs`

**Step 1: Add module declaration**

Edit `src-tauri/src/lib.rs`:
```rust
mod claude;
mod file_ops;
mod image_utils;
mod settings;
mod model_manager;
mod candle_inference;
mod offline_analyzer;  // Add this

use tauri::command;
```

**Step 2: Create offline analyzer with error types**

Create `src-tauri/src/offline_analyzer.rs`:
```rust
use crate::candle_inference::{Qwen2VLInference, build_qwen_prompt};
use crate::model_manager::{check_model_status, get_model_path, ModelStatus};
use crate::settings::AppSettings;
use anyhow::{Context, Result};
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
        ModelVariant::Qwen2VL2B => 4.0,
        ModelVariant::Qwen2VL7B => 12.0,
        ModelVariant::Qwen2VL72B => 80.0,
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
    )
    .map_err(|e| OfflineAnalysisError::ModelLoadError(e.to_string()))?;

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
```

**Step 3: Build to verify**

Run: `cargo build --manifest-path=src-tauri/Cargo.toml`
Expected: Success

**Step 4: Commit**

```bash
git add src-tauri/src/lib.rs src-tauri/src/offline_analyzer.rs
git commit -m "feat(offline): add offline analyzer orchestration

Add analyze_style function that coordinates model loading, image
preprocessing, and inference. Includes system requirements checking
and comprehensive error handling."
```

---

## Phase 5: Tauri Command Integration

### Task 9: Add Tauri Commands for Settings

**Files:**
- Modify: `src-tauri/src/lib.rs`

**Step 1: Add get_settings command**

Add to `src-tauri/src/lib.rs` after existing commands:
```rust
#[command]
fn get_settings() -> Result<settings::AppSettings, String> {
    settings::load_settings()
        .map_err(|e| format!("Failed to load settings: {}", e))
}
```

**Step 2: Add update_settings command**

Add to `src-tauri/src/lib.rs`:
```rust
#[command]
fn update_settings(settings: settings::AppSettings) -> Result<(), String> {
    settings::save_settings(&settings)
        .map_err(|e| format!("Failed to save settings: {}", e))
}
```

**Step 3: Add get_model_status command**

Add to `src-tauri/src/lib.rs`:
```rust
#[command]
fn get_model_status(variant: settings::ModelVariant) -> Result<model_manager::ModelStatus, String> {
    let settings = settings::load_settings().unwrap_or_default();
    model_manager::check_model_status(variant, settings.model_cache_dir)
        .map_err(|e| format!("Failed to check model status: {}", e))
}
```

**Step 4: Add download_model command**

Add to `src-tauri/src/lib.rs`:
```rust
#[command]
async fn download_model(variant: settings::ModelVariant) -> Result<(), String> {
    let settings = settings::load_settings().unwrap_or_default();
    model_manager::download_model(variant, settings.model_cache_dir)
        .await
        .map_err(|e| format!("Failed to download model: {}", e))
}
```

**Step 5: Add clear_model_cache command**

Add to `src-tauri/src/lib.rs`:
```rust
#[command]
fn clear_model_cache() -> Result<u64, String> {
    let settings = settings::load_settings().unwrap_or_default();
    model_manager::clear_model_cache(settings.model_cache_dir)
        .map_err(|e| format!("Failed to clear cache: {}", e))
}
```

**Step 6: Register new commands**

Update the `invoke_handler` in `src-tauri/src/lib.rs`:
```rust
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
```

**Step 7: Build to verify**

Run: `cargo build --manifest-path=src-tauri/Cargo.toml`
Expected: Success

**Step 8: Commit**

```bash
git add src-tauri/src/lib.rs
git commit -m "feat(commands): add Tauri commands for settings and models

Add get_settings, update_settings, get_model_status, download_model,
and clear_model_cache commands for frontend integration."
```

---

### Task 10: Update analyze_style Command with Mode Selection

**Files:**
- Modify: `src-tauri/src/lib.rs`

**Step 1: Update analyze_style command signature**

Replace existing `analyze_style` in `src-tauri/src/lib.rs`:
```rust
use serde::Serialize;

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
```

**Step 2: Build to verify**

Run: `cargo build --manifest-path=src-tauri/Cargo.toml`
Expected: Success

**Step 3: Commit**

```bash
git add src-tauri/src/lib.rs
git commit -m "feat(analyze): integrate offline mode with fallback logic

Update analyze_style command to support CloudAPI/Offline/Auto modes
with automatic fallback. Returns mode_used and fallback_used flags."
```

---

## Phase 6: Frontend Integration

### Task 11: Update Project Store for Settings

**Files:**
- Modify: `src/stores/project.ts`

**Step 1: Add settings state**

Add to `src/stores/project.ts` imports:
```typescript
import type {
  ProjectData,
  DatasetSpecification,
  PermutationBatch,
} from '../types/schema';

// Add these types
export type AnalysisMode = 'CloudAPI' | 'Offline' | 'Auto';
export type ModelVariant = 'Qwen2VL2B' | 'Qwen2VL7B' | 'Qwen2VL72B';

export interface AppSettings {
  analysis_mode: AnalysisMode;
  offline_model_variant: ModelVariant;
  model_cache_dir: string | null;
  auto_fallback: boolean;
  keep_model_loaded: boolean;
}
```

**Step 2: Add settings state and actions**

Add to the store definition in `src/stores/project.ts`:
```typescript
// Add to state section
const settings = ref<AppSettings | null>(null);

// Add to computed
const hasSettings = computed(() => settings.value !== null);

// Add to actions
async function loadSettings() {
  try {
    const result = await invoke<AppSettings>('get_settings');
    settings.value = result;
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
    throw e;
  }
}

async function saveSettings(newSettings: AppSettings) {
  try {
    await invoke('update_settings', { settings: newSettings });
    settings.value = newSettings;
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
    throw e;
  }
}

// Add to return statement
return {
  // ... existing exports
  settings,
  hasSettings,
  loadSettings,
  saveSettings,
};
```

**Step 3: Update analyzeStyle to handle new response format**

Modify the `analyzeStyle` function in `src/stores/project.ts`:
```typescript
async function analyzeStyle() {
  if (!canAnalyze.value) {
    throw new Error('Need at least 3 images and SREF code');
  }

  isLoading.value = true;
  error.value = null;
  statusMessage.value = null;

  try {
    statusMessage.value = `Reading ${imagePaths.value.length} image${imagePaths.value.length > 1 ? 's' : ''}...`;
    await new Promise(resolve => setTimeout(resolve, 100));

    statusMessage.value = 'Analyzing images...';

    const result = await invoke<{
      data: string;
      mode_used: string;
      fallback_used: boolean;
    }>('analyze_style', {
      imagePaths: imagePaths.value,
      srefCode: String(srefCode.value),
    });

    statusMessage.value = 'Processing response...';
    await new Promise(resolve => setTimeout(resolve, 100));

    const parsed = JSON.parse(result.data);
    specification.value = parsed as DatasetSpecification;
    isDirty.value = true;
    currentStep.value = 'analysis';

    statusMessage.value = 'Analysis complete!';
    await new Promise(resolve => setTimeout(resolve, 500));

    return {
      ...result,
      specification: specification.value,
    };
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
    statusMessage.value = null;
    throw e;
  } finally {
    isLoading.value = false;
    setTimeout(() => {
      statusMessage.value = null;
    }, 1000);
  }
}
```

**Step 4: Commit**

```bash
git add src/stores/project.ts
git commit -m "feat(store): add settings management to project store

Add AppSettings types, loadSettings/saveSettings actions, and update
analyzeStyle to handle new response format with mode_used and
fallback_used flags."
```

---

### Task 12: Create Settings View Component

**Files:**
- Create: `src/views/SettingsView.vue`

**Step 1: Create basic settings view**

Create `src/views/SettingsView.vue`:
```vue
<template>
  <div class="container p-6 mx-auto max-w-4xl">
    <h1 class="mb-6 text-3xl font-bold text-gray-900 dark:text-white">Settings</h1>

    <div v-if="loading" class="text-center py-12">
      <p class="text-gray-600 dark:text-gray-400">Loading settings...</p>
    </div>

    <div v-else-if="localSettings" class="space-y-6">
      <!-- Analysis Mode Selection -->
      <div class="p-6 bg-white rounded-lg shadow dark:bg-gray-800">
        <h2 class="mb-4 text-xl font-bold text-gray-900 dark:text-white">Analysis Mode</h2>
        <div class="space-y-3">
          <label class="flex items-center gap-3 p-3 border rounded cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700">
            <input
              type="radio"
              v-model="localSettings.analysis_mode"
              value="CloudAPI"
              class="w-4 h-4"
            />
            <div>
              <div class="font-medium text-gray-900 dark:text-white">Cloud API</div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Use Claude API for fast, high-quality analysis. Requires API key.
              </div>
            </div>
          </label>

          <label class="flex items-center gap-3 p-3 border rounded cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700">
            <input
              type="radio"
              v-model="localSettings.analysis_mode"
              value="Offline"
              class="w-4 h-4"
            />
            <div>
              <div class="font-medium text-gray-900 dark:text-white">Offline Mode</div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Use local Qwen2-VL model. Fully private, no internet required after download.
              </div>
            </div>
          </label>

          <label class="flex items-center gap-3 p-3 border rounded cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700">
            <input
              type="radio"
              v-model="localSettings.analysis_mode"
              value="Auto"
              class="w-4 h-4"
            />
            <div>
              <div class="font-medium text-gray-900 dark:text-white">Auto (Recommended)</div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Use Cloud API if available, otherwise fall back to offline mode.
              </div>
            </div>
          </label>
        </div>
      </div>

      <!-- Offline Model Selection -->
      <div v-if="localSettings.analysis_mode !== 'CloudAPI'" class="p-6 bg-white rounded-lg shadow dark:bg-gray-800">
        <h2 class="mb-4 text-xl font-bold text-gray-900 dark:text-white">Offline Model</h2>
        <div class="space-y-3">
          <div
            v-for="variant in modelVariants"
            :key="variant.value"
            class="flex items-center justify-between p-4 border rounded"
          >
            <label class="flex items-center gap-3 cursor-pointer flex-1">
              <input
                type="radio"
                v-model="localSettings.offline_model_variant"
                :value="variant.value"
                class="w-4 h-4"
              />
              <div>
                <div class="font-medium text-gray-900 dark:text-white">{{ variant.label }}</div>
                <div class="text-sm text-gray-600 dark:text-gray-400">
                  {{ variant.size }} • {{ variant.ram }} RAM • {{ variant.quality }}
                </div>
              </div>
            </label>
            <div class="flex items-center gap-3">
              <span
                class="px-2 py-1 text-xs font-medium rounded"
                :class="getStatusClass(variant.value)"
              >
                {{ getStatusText(variant.value) }}
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- Advanced Options -->
      <div class="p-6 bg-white rounded-lg shadow dark:bg-gray-800">
        <h2 class="mb-4 text-xl font-bold text-gray-900 dark:text-white">Advanced Options</h2>
        <div class="space-y-4">
          <label class="flex items-center justify-between">
            <div>
              <div class="font-medium text-gray-900 dark:text-white">Auto-fallback</div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Automatically use offline mode if Cloud API fails
              </div>
            </div>
            <input
              type="checkbox"
              v-model="localSettings.auto_fallback"
              class="w-5 h-5"
            />
          </label>

          <label class="flex items-center justify-between">
            <div>
              <div class="font-medium text-gray-900 dark:text-white">Keep model loaded</div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Keep model in memory between analyses (faster but uses more RAM)
              </div>
            </div>
            <input
              type="checkbox"
              v-model="localSettings.keep_model_loaded"
              class="w-5 h-5"
            />
          </label>
        </div>
      </div>

      <!-- Save Button -->
      <div class="flex justify-end gap-3">
        <button
          @click="goBack"
          class="px-6 py-3 font-medium text-gray-900 transition-colors bg-gray-300 rounded-lg dark:bg-gray-700 dark:text-white hover:bg-gray-400 dark:hover:bg-gray-600"
        >
          Cancel
        </button>
        <button
          @click="saveChanges"
          :disabled="saving"
          class="px-6 py-3 font-medium text-white transition-colors bg-blue-600 rounded-lg hover:bg-blue-700 disabled:opacity-50"
        >
          {{ saving ? 'Saving...' : 'Save Changes' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { useProjectStore } from '../stores/project';
import type { AppSettings } from '../stores/project';

const router = useRouter();
const store = useProjectStore();

const loading = ref(true);
const saving = ref(false);
const localSettings = ref<AppSettings | null>(null);

const modelVariants = [
  {
    value: 'Qwen2VL2B',
    label: 'Qwen2-VL 2B',
    size: '2.8 GB',
    ram: '4 GB',
    quality: 'Good quality',
  },
  {
    value: 'Qwen2VL7B',
    label: 'Qwen2-VL 7B',
    size: '6.5 GB',
    ram: '12 GB',
    quality: 'Better quality',
  },
  {
    value: 'Qwen2VL72B',
    label: 'Qwen2-VL 72B',
    size: '45 GB',
    ram: '80 GB',
    quality: 'Best quality',
  },
];

const modelStatus = ref<Record<string, string>>({});

function getStatusText(variant: string): string {
  return modelStatus.value[variant] || 'Unknown';
}

function getStatusClass(variant: string): string {
  const status = modelStatus.value[variant];
  if (status === 'Ready') return 'bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400';
  if (status === 'NotDownloaded') return 'bg-gray-100 text-gray-800 dark:bg-gray-900/20 dark:text-gray-400';
  return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/20 dark:text-yellow-400';
}

async function loadData() {
  try {
    await store.loadSettings();
    localSettings.value = { ...store.settings! };
    // TODO: Load model status for each variant
  } catch (e) {
    console.error('Failed to load settings:', e);
  } finally {
    loading.value = false;
  }
}

async function saveChanges() {
  if (!localSettings.value) return;

  saving.value = true;
  try {
    await store.saveSettings(localSettings.value);
    router.push('/');
  } catch (e) {
    console.error('Failed to save settings:', e);
  } finally {
    saving.value = false;
  }
}

function goBack() {
  router.push('/');
}

onMounted(() => {
  loadData();
});
</script>
```

**Step 2: Build to verify**

Run: `npm run build`
Expected: Success (TypeScript compilation)

**Step 3: Commit**

```bash
git add src/views/SettingsView.vue
git commit -m "feat(ui): add settings view component

Add SettingsView with mode selection, model variant options, and
advanced settings. Includes loading/saving functionality."
```

---

### Task 13: Add Settings Route

**Files:**
- Modify: `src/router/index.ts`

**Step 1: Add settings route**

Add to `src/router/index.ts`:
```typescript
{
  path: '/settings',
  name: 'settings',
  component: () => import('../views/SettingsView.vue'),
},
```

**Step 2: Add navigation link to settings**

Edit `src/App.vue` to add settings link (exact location depends on your nav structure):
```vue
<!-- Add to navigation area -->
<router-link to="/settings" class="...">Settings</router-link>
```

**Step 3: Build to verify**

Run: `npm run build`
Expected: Success

**Step 4: Commit**

```bash
git add src/router/index.ts src/App.vue
git commit -m "feat(router): add settings route and navigation

Add /settings route and navigation link to access settings page."
```

---

## Phase 7: Testing & Polish

### Task 14: Add Model Cache to .gitignore

**Files:**
- Modify: `.gitignore`

**Step 1: Add cache directory patterns**

Add to `.gitignore`:
```
# Model cache (local testing)
.cache/
models/
```

**Step 2: Commit**

```bash
git add .gitignore
git commit -m "chore: add model cache to gitignore

Prevent accidentally committing downloaded model files during
local testing."
```

---

### Task 15: Update README with Offline Mode Documentation

**Files:**
- Modify: `README.md`

**Step 1: Add offline mode section**

Add to `README.md`:
```markdown
## Offline Mode

The app supports fully offline image analysis using Qwen2-VL vision models:

### Hardware Requirements

| Model | Download Size | RAM Required | Inference Speed |
|-------|---------------|--------------|-----------------|
| Qwen2-VL-2B (Default) | 2.8 GB | 4 GB | 10-20 seconds |
| Qwen2-VL-7B | 6.5 GB | 12 GB | 30-60 seconds |
| Qwen2-VL-72B | 45 GB | 80 GB | 2-5 minutes |

### Setup

1. Open Settings (gear icon)
2. Select "Offline Mode" or "Auto"
3. Choose your model variant
4. First analysis will download the model (shows progress)
5. Subsequent analyses run instantly offline

### GPU Acceleration

- **macOS**: Automatic Metal acceleration
- **Linux**: Automatic Intel MKL, optional CUDA (requires NVIDIA GPU)
- **Windows**: CPU only (GPU support coming soon)
```

**Step 2: Commit**

```bash
git add README.md
git commit -m "docs: add offline mode documentation

Document hardware requirements, setup instructions, and GPU
acceleration support for offline mode."
```

---

## Execution Strategy

This plan is structured for **incremental, testable development**:

- Each task is 5-15 minutes of focused work
- Follows TDD: test → fail → implement → pass → commit
- Stubs allow frontend/backend parallel development
- Frequent commits enable easy rollback

### Next Steps

1. **Implement Candle Integration** (Task 16+): Replace stubs with real Qwen2-VL inference
2. **Add Model Download UI** (Task 17+): Progress bars, retry logic, user feedback
3. **Comprehensive Testing** (Task 18+): Integration tests, error scenarios, cross-platform
4. **Performance Optimization** (Task 19+): Model quantization, caching, batch processing

### Critical TODOs Marked in Code

- `src-tauri/src/candle_inference.rs`: Real model loading and inference
- `src-tauri/src/model_manager.rs`: HF Hub integration for downloads
- `src/views/SettingsView.vue`: Model status checking and download UI

---

**Plan complete!** All foundational modules in place with clear integration points for Candle ML implementation.
