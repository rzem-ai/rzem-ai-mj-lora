# Offline Mode with Qwen2-VL Design

**Date:** 2026-01-16
**Status:** Approved
**Author:** Design collaboration

## Overview

Add offline image analysis capability to the LoRA Training Dataset Generator using Qwen2-VL vision-language models embedded in the Tauri app. This enables fully local operation without requiring Claude API access while maintaining the same output format.

## Goals

- **Offline-first operation**: Users can analyze style reference images without internet (after initial model download)
- **User choice**: Toggle between Claude API (fast, high-quality) and offline mode (private, no API cost)
- **Configurable models**: Support multiple Qwen2-VL variants (2B, 7B, 72B) for quality/speed trade-offs
- **Seamless integration**: No changes to existing UI for analysis results
- **Automatic fallback**: Gracefully fallback from API to offline if API fails

## Architecture

### High-Level Flow

```
┌─────────────┐
│   Vue UI    │
│  (Frontend) │
└──────┬──────┘
       │ invoke('analyze_style')
       ▼
┌──────────────────────────────────┐
│   Tauri Command Handler (Rust)   │
│   src-tauri/src/lib.rs           │
└──────┬───────────────────────────┘
       │
       ├─→ Check Settings → Mode?
       │
       ├─→ [API Mode] ──→ src-tauri/src/claude.rs (existing)
       │                   HTTP → Claude API
       │
       └─→ [Offline Mode] → src-tauri/src/offline_analyzer.rs (NEW)
                            ├─→ Model Manager: Download/cache models
                            ├─→ Candle Inference: Load model, analyze images
                            └─→ Prompt Engineering: Format for Qwen2-VL
```

### New Modules

| Module | Purpose |
|--------|---------|
| `offline_analyzer.rs` | Main offline analysis orchestration |
| `model_manager.rs` | Download, cache, and load model files from Hugging Face |
| `candle_inference.rs` | Qwen2-VL inference using Candle framework |
| `settings.rs` | Persist user preferences (mode, model variant, cache path) |

### Shared Interface

Both API and offline modes return the same `DatasetSpecification` JSON schema, so frontend analysis/batch views require no changes - only settings UI additions needed.

## Model Management

### Supported Variants

| Model | Size | RAM Required | Speed | Quality |
|-------|------|--------------|-------|---------|
| Qwen2-VL-2B | 2-3 GB | 4 GB | Fast | Good |
| Qwen2-VL-7B | 5-7 GB | 12 GB | Moderate | Better |
| Qwen2-VL-72B | 40+ GB | 80 GB | Slow | Best |

### Model Registry

```rust
pub enum ModelVariant {
    Qwen2VL2B,     // Default - balances quality/speed
    Qwen2VL7B,     // Better quality for capable hardware
    Qwen2VL72B,    // Enthusiast/research (optional)
}

struct ModelConfig {
    variant: ModelVariant,
    hf_repo: String,           // e.g., "Qwen/Qwen2-VL-2B-Instruct"
    files: Vec<String>,        // model.safetensors, config.json, etc.
    total_size_bytes: u64,
    cache_dir: PathBuf,        // Platform-specific cache
}
```

### Download Strategy

**First-time analysis in offline mode:**
1. Check if selected model exists in cache
2. Show modal: "Downloading Qwen2-VL-2B model (2.8 GB)..."
3. Download from Hugging Face using `hf_hub` crate
4. Show progress bar (bytes downloaded / total size)
5. Validate checksums
6. Cache in user data directory

**Subsequent analyses:**
- Load from cache instantly
- Optional integrity verification

**Cache locations:**
- Linux: `~/.cache/rzem-mj-lora/models/`
- macOS: `~/Library/Caches/rzem-mj-lora/models/`
- Windows: `%LOCALAPPDATA%\rzem-mj-lora\models\`

### Download Features

- **Resumable downloads**: Detect partial files and continue
- **Retry logic**: 3 attempts with exponential backoff
- **Model switching**: Users can download multiple variants
- **Cache management**: "Clear Model Cache" button to free space

## Candle Inference Integration

### Setup

```rust
pub struct Qwen2VLInference {
    model: Model,
    tokenizer: Tokenizer,
    device: Device,  // CUDA > Metal > CPU
    config: Config,
}

impl Qwen2VLInference {
    pub async fn new(model_path: &Path, variant: ModelVariant) -> Result<Self> {
        // Auto-detect best device: CUDA > Metal > CPU
        let device = detect_optimal_device();

        // Load model weights (safetensors)
        let vb = unsafe {
            candle_nn::VarBuilder::from_mmaped_safetensors(
                &[model_path.join("model.safetensors")],
                DType::F16,
                &device
            )?
        };

        // Initialize model, tokenizer, config
        // ...
    }
}
```

### Image Preprocessing

Qwen2-VL requirements:
- Resize to model's expected resolution (448x448 or dynamic)
- Normalize RGB to [0, 1] range
- Convert to tensors: `[batch, channels, height, width]`
- Support multiple images in single inference

### Prompt Engineering

Qwen2-VL uses chat templates different from Claude:

```rust
fn build_qwen_prompt(sref_code: &str, num_images: usize) -> String {
    format!(
        "<|im_start|>system\nYou are Qwen, a vision-language AI assistant specialized in analyzing artistic styles.<|im_end|>
<|im_start|>user\n{}<|vision_pad|>Analyze these {} style reference images for Midjourney SREF code {}.

Generate a LoRA training dataset specification with:
1. Style analysis (colors, patterns, era, characteristics)
2. 8-10 permutation batches with EXACTLY 40 images each
3. Use format: {{subjects}} with {{modifiers}} --sref {}

Output ONLY valid JSON matching this schema: [schema]<|im_end|>
<|im_start|>assistant\n",
        "<|vision_start|><|image_pad|><|vision_end|>".repeat(num_images),
        num_images, sref_code, sref_code
    )
}
```

### Performance Characteristics

- **First load:** 5-15 seconds (load model into memory)
- **Per-analysis:** 10-60 seconds (depends on model size + hardware)
- **Model caching:** Keep loaded between analyses (configurable)
- **Memory usage:** 4-16GB RAM depending on variant
- **Progress updates:** Emit events during inference for UI feedback

## Settings & Configuration

### Settings Schema

```rust
pub struct AppSettings {
    pub analysis_mode: AnalysisMode,          // CloudAPI / Offline / Auto
    pub offline_model_variant: ModelVariant,  // Qwen2VL2B / 7B / 72B
    pub model_cache_dir: Option<PathBuf>,     // Custom cache path
    pub auto_fallback: bool,                  // Fallback to offline if API fails
    pub keep_model_loaded: bool,              // Memory vs reload trade-off
}

pub enum AnalysisMode {
    CloudAPI,      // Use Claude API
    Offline,       // Use local Qwen2-VL
    Auto,          // API if key available, else offline (default)
}
```

### Persistence

Settings stored in:
- Linux/macOS: `~/.config/rzem-mj-lora/settings.json`
- Windows: `%APPDATA%\rzem-mj-lora\settings.json`

### New Tauri Commands

```rust
#[command]
fn get_settings() -> Result<AppSettings, String>

#[command]
fn update_settings(settings: AppSettings) -> Result<(), String>

#[command]
fn get_model_status(variant: ModelVariant) -> Result<ModelStatus, String>
// Returns: NotDownloaded | Downloading(progress%) | Ready | Error

#[command]
async fn download_model(variant: ModelVariant) -> Result<(), String>

#[command]
fn clear_model_cache() -> Result<u64, String>  // Returns bytes freed
```

### Updated Analysis Flow

```rust
#[command]
async fn analyze_style(image_paths: Vec<String>, sref_code: String)
    -> Result<AnalysisResult, String>
{
    let settings = settings::load_settings().unwrap_or_default();

    // Determine mode
    let mode = match settings.analysis_mode {
        AnalysisMode::CloudAPI => {
            if has_api_key() { CloudAPI }
            else if settings.auto_fallback { Offline }
            else { return Err("API key not found") }
        },
        AnalysisMode::Offline => Offline,
        AnalysisMode::Auto => {
            if has_api_key() { CloudAPI } else { Offline }
        }
    };

    // Execute with fallback
    match mode {
        CloudAPI => match claude::analyze_style(...).await {
            Ok(result) => Ok(AnalysisResult { data: result, mode_used: "cloud", ... }),
            Err(e) if settings.auto_fallback => {
                // Try offline fallback
                offline_analyzer::analyze_style(...).await
            },
            Err(e) => Err(e)
        },
        Offline => offline_analyzer::analyze_style(...).await
    }
}

struct AnalysisResult {
    data: String,           // JSON output
    mode_used: String,      // "cloud" or "offline"
    fallback_used: bool,    // True if fallback occurred
}
```

## UI Changes

### New Settings View

**Route:** `/settings`

**Components:**
- **Mode Selection:** Radio buttons for CloudAPI / Offline / Auto
- **Model Variant Cards:** Shows each model with:
  - Name, size, quality rating
  - Download status badge
  - Download button (if not ready)
  - Radio selection for active model
- **Advanced Options:** Toggles for auto-fallback, keep-model-loaded
- **Cache Management:** Display cache size, "Clear Cache" button

### Navigation

Add settings icon/button to main app navigation (top-right corner or in step indicator).

### Status Messages

Enhance existing status display during analysis:
- "Loading Qwen2-VL model (first time: 10-15s)..." (offline)
- "Analyzing with offline model..." (offline)
- "Sending request to Claude API..." (API)

### Notifications

- Show toast if fallback occurs: "API failed, completed using offline model"
- Prompt to download model if attempting offline without cached model
- Show memory warning if insufficient RAM for selected model

## Error Handling

### Error Types

```rust
pub enum OfflineAnalysisError {
    ModelNotFound,
    DownloadFailed(String),
    InsufficientMemory { required: f32, available: f32 },
    ModelLoadError(String),
    InferenceFailed(String),
    InvalidOutput(String),
    ImageProcessingError(String),
    DeviceError(String),
}
```

### Pre-flight Checks

```rust
fn check_system_requirements(variant: ModelVariant) -> Result<()> {
    let required_gb = match variant {
        Qwen2VL2B => 4.0,
        Qwen2VL7B => 12.0,
        Qwen2VL72B => 80.0,
    };

    let available = get_available_memory_gb();
    if available < required_gb {
        return Err(InsufficientMemory { required, available });
    }
    Ok(())
}
```

### Automatic Fallback Logic

1. **API → Offline fallback:** If API request fails and `auto_fallback=true`, try offline mode
2. **No fallback from offline:** Offline mode has no fallback (it's the last resort)
3. **User notification:** Toast notification when fallback occurs
4. **Retry logic:** 3 attempts with exponential backoff for downloads

### User-Friendly Error Messages

- **Model not found:** Show modal with "Download Model" button
- **Insufficient memory:** Suggest using smaller model variant
- **API key missing:** Prompt to add key or switch to offline mode
- **Download failed:** Show retry button with error details

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_prompt_generation() { ... }

#[test]
fn test_json_extraction() { ... }

#[test]
fn test_memory_check() { ... }

#[tokio::test]
async fn test_model_download_resume() { ... }
```

### Integration Tests

```rust
#[cfg(feature = "integration_tests")]
#[tokio::test]
async fn test_full_offline_analysis() {
    // Requires actual model download
    // Validates end-to-end offline flow
}
```

### Manual Testing Checklist

- [ ] Download each model variant
- [ ] Verify analysis output matches schema
- [ ] Test fallback from API to offline
- [ ] Verify resumable downloads
- [ ] Test on low-memory system (should fail gracefully)
- [ ] Verify cache clearing works
- [ ] Test cross-platform (Linux, macOS, Windows)
- [ ] Validate GPU acceleration (CUDA, Metal)

## Dependencies

### New Rust Crates

```toml
[dependencies]
# ML inference
candle-core = "0.6"
candle-nn = "0.6"
candle-transformers = "0.6"
tokenizers = "0.19"

# Model downloads
hf-hub = "0.3"

# System info
sysinfo = "0.30"

[features]
cuda = ["candle-core/cuda"]
metal = ["candle-core/metal"]
```

### Platform-Specific

- **macOS:** Metal framework, Accelerate
- **Linux:** Optional CUDA support, Intel MKL
- **Windows:** Standard CPU support

## Deployment

### Package Sizes

**Without bundled models:**
- macOS: ~35-45 MB
- Linux: ~40-50 MB
- Windows: ~38-48 MB

**With bundled Qwen2-VL-2B:**
- All platforms: ~3.2-3.5 GB

**Recommendation:** Ship without models, auto-download on first use.

### Runtime Requirements

| Model | RAM | Disk Space | Inference Time |
|-------|-----|------------|----------------|
| 2B | 4 GB | 2.8 GB | 10-20s |
| 7B | 12 GB | 6.5 GB | 30-60s |
| 72B | 80 GB | 45 GB | 2-5 min |

### Build Matrix

- Linux: x86_64-unknown-linux-gnu (CPU + optional CUDA)
- macOS: x86_64-apple-darwin, aarch64-apple-darwin (Metal)
- Windows: x86_64-pc-windows-msvc

### CI/CD Updates

- Run unit tests on all platforms
- Optional integration tests (requires model download)
- Build distributable for each target
- Test model download resumption

## Performance Optimizations

1. **Model quantization:** Use Q8_0 or Q4_0 for smaller size/faster inference
2. **KV-cache:** Reuse computation across similar requests
3. **Batch processing:** Process multiple images efficiently in single forward pass
4. **Keep-alive:** Option to keep model loaded between analyses

## Privacy & Security

- **Offline = private:** No data leaves device in offline mode
- **Official models only:** Download from verified Hugging Face repos
- **Checksum verification:** Validate model integrity on download
- **Clear documentation:** Explain API vs offline data handling
- **User choice:** Settings make trade-offs transparent

## Documentation Needed

1. **README updates:**
   - Hardware requirements per model variant
   - Offline mode explanation
   - Performance expectations by hardware

2. **First-run guide:**
   - Setting up offline mode
   - Downloading models
   - Choosing optimal variant

3. **Troubleshooting:**
   - GPU detection issues
   - Memory problems
   - Download failures
   - Model switching

## Rollout Plan

### Phase 1: Core Implementation (Week 1)
- Implement model_manager.rs (download, cache)
- Implement candle_inference.rs (basic Qwen2-VL loading)
- Implement settings.rs (persistence)

### Phase 2: Integration (Week 2)
- Implement offline_analyzer.rs (main orchestration)
- Update analyze_style command with mode selection
- Add error handling and fallback logic

### Phase 3: UI (Week 3)
- Create SettingsView.vue
- Add model download UI with progress
- Update status messages
- Add notifications for fallback

### Phase 4: Testing & Polish (Week 4)
- Cross-platform testing
- Performance optimization
- Documentation
- Release candidate

## Success Criteria

- [ ] Users can analyze styles completely offline (after model download)
- [ ] Model download shows progress and is resumable
- [ ] Settings persist across app restarts
- [ ] Fallback from API to offline works seamlessly
- [ ] Analysis output format matches existing schema
- [ ] No UI changes needed for analysis results
- [ ] Works on Linux, macOS, Windows
- [ ] Memory requirements clearly communicated
- [ ] Performance acceptable (< 60s for 2B model)

## Future Enhancements

- Custom model support (user-provided GGUF/Safetensors)
- Multi-GPU support for larger models
- Model compression/quantization options
- Background model updates (check for new versions)
- Hybrid mode (use API for first analysis, cache embeddings locally)
- Fine-tuned Qwen2-VL specifically for SREF analysis

## References

- Qwen2-VL: https://huggingface.co/Qwen/Qwen2-VL-2B-Instruct
- Candle framework: https://github.com/huggingface/candle
- Tauri patterns: https://tauri.app/v1/guides/
