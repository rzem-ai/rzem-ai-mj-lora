# LoRA Training Dataset Generator

A desktop application for generating optimized Midjourney permutation batches for LoRA (Low-Rank Adaptation) training datasets from SREF (style reference) codes.

Built with **Tauri** + **Vue 3** + **Rust**.

## Features

- **Image Upload & Preview**: Drag-and-drop interface for 3-10 SREF reference images
- **AI-Powered Style Analysis**: Automatic analysis using Claude API (Sonnet 4.5) or offline Qwen2-VL models
- **Offline Mode**: Fully local analysis with Qwen2-VL vision-language models (no internet required after model download)
- **Flexible Analysis Modes**: Choose between Cloud API (fast), Offline (private), or Auto (fallback)
- **Batch Generation**: Create 8-10 permutation batches with exactly 40 images each
- **Real-time Validation**: Validates permutation syntax, image counts, and SREF codes
- **Manual Editing**: Edit, duplicate, and remove batches with live validation
- **Export Options**: Export to JSON or Markdown format
- **Project Management**: Save and load projects for later editing

## Prerequisites

- **Node.js** (v18 or later)
- **Rust** (latest stable version)
- **Claude API Key** (optional - only required for Cloud API mode)

### Hardware Requirements for Offline Mode

| Model Variant | Disk Space | RAM Required | Inference Time | Quality |
|---------------|------------|--------------|----------------|---------|
| Qwen2-VL-2B (Default) | 4.5 GB | 4 GB | 10-20s | Good |
| Qwen2-VL-7B | 15 GB | 12 GB | 30-60s | Better |
| Qwen2-VL-72B | 146 GB | 80 GB | 2-5 min | Best |

## Installation

1. **Clone or navigate to the repository:**
   ```bash
   cd rzem-ai-mj-lora
   ```

2. **Install dependencies:**
   ```bash
   npm install
   ```

3. **Set up Claude API Key:**

   Create a `.env` file in the project root:
   ```bash
   CLAUDE_API_KEY=your_api_key_here
   ```

   Or set it as an environment variable:
   ```bash
   export CLAUDE_API_KEY=your_api_key_here
   ```

## Offline Mode Setup

The application supports fully offline analysis using local Qwen2-VL models. No internet connection is required after the initial model download.

### Configuration

1. **Access Settings**: Click the "Settings" button in the top-right corner
2. **Choose Analysis Mode**:
   - **Cloud API**: Fast analysis using Claude API (requires API key)
   - **Offline**: Private local analysis using Qwen2-VL models
   - **Auto (Recommended)**: Uses API if available, falls back to offline mode
3. **Select Model Variant**: Choose based on your hardware (2B recommended for most users)
4. **Download Model**: Click "Download Model" for first-time setup
5. **Configure Options**:
   - **Auto Fallback**: Automatically use offline if API fails
   - **Keep Model Loaded**: Faster subsequent analyses (uses more RAM)

### Model Storage

Models are cached in platform-specific directories:
- **Linux/macOS**: `~/.cache/rzem-mj-lora/models/`
- **Windows**: `%LOCALAPPDATA%\rzem-mj-lora\models\`

Settings are stored in:
- **Linux/macOS**: `~/.config/rzem-mj-lora/settings.json`
- **Windows**: `%APPDATA%\rzem-mj-lora\settings.json`

### Performance Tips

- **Qwen2-VL-2B**: Best for most users - fast and good quality
- **Qwen2-VL-7B**: Use if you have 16+ GB RAM and want better quality
- **Qwen2-VL-72B**: For high-end workstations only (research/enthusiast)
- **Keep Model Loaded**: Enable for multiple analyses in one session
- **GPU Acceleration**: Automatically uses Metal (macOS) or CUDA (if available)

## Development

Run the application in development mode:

```bash
npm run tauri dev
```

This will:
- Start the Vite development server for the frontend
- Build and launch the Tauri app with hot-reload

## Building for Production

Build the application:

```bash
npm run tauri build
```

The built application will be in `src-tauri/target/release/bundle/`.

## Usage

### 1. Upload Images
- Upload 3-10 style reference images (SREF images from Midjourney)
- Enter the 10-digit SREF code
- Click "Analyze Style"

### 2. Review Analysis
- Review the AI-generated style analysis
- Check color palette, characteristics, and subject recommendations
- Proceed to edit batches

### 3. Edit Batches
- View and edit generated permutation batches
- Each batch must generate exactly 40 images
- Use the format: `{subject1, subject2, ...} with {modifier1, modifier2, ...} --sref CODE`
- Add, duplicate, or remove batches as needed
- Real-time validation ensures correctness

### 4. Export
- View dataset summary and validation status
- Export to JSON or Markdown format
- Save project for later editing

## Architecture

### Frontend (Vue 3)
- **Views**: UploadView, AnalysisView, BatchEditorView, ExportView, SettingsView
- **Components**: ImageUploader, StyleAnalysisCard, BatchCard, BatchValidator, ExportPanel
- **State Management**: Pinia store for project state and settings
- **Routing**: Vue Router for navigation
- **Styling**: Tailwind CSS v4

### Backend (Rust/Tauri)
- **image_utils.rs**: Image file handling and base64 encoding
- **claude.rs**: Claude API integration for cloud-based style analysis
- **offline_analyzer.rs**: Offline analysis orchestration with Qwen2-VL
- **model_manager.rs**: Model download, caching, and status management
- **candle_inference.rs**: Qwen2-VL inference using Candle ML framework
- **settings.rs**: Application settings persistence
- **file_ops.rs**: Project save/load and export operations
- **lib.rs**: Tauri command handlers

## Project Structure

```
rzem-ai-mj-lora/
├── src/                          # Vue3 frontend
│   ├── components/               # Reusable Vue components
│   ├── views/                    # Page-level components
│   │   └── SettingsView.vue      # Offline mode settings UI
│   ├── stores/                   # Pinia state management
│   │   └── project.ts            # Project store with settings
│   ├── router/                   # Vue Router configuration
│   ├── types/                    # TypeScript type definitions
│   ├── utils/                    # Validation and export utilities
│   ├── App.vue                   # Root component
│   └── main.ts                   # Application entry point
├── src-tauri/                    # Rust backend
│   └── src/
│       ├── claude.rs             # Claude API client
│       ├── offline_analyzer.rs   # Offline analysis orchestration
│       ├── model_manager.rs      # Model download & caching
│       ├── candle_inference.rs   # Qwen2-VL inference
│       ├── settings.rs           # Settings persistence
│       ├── file_ops.rs           # File operations
│       ├── image_utils.rs        # Image handling
│       └── lib.rs                # Main Tauri app
├── docs/plans/                   # Design documents
│   ├── 2026-01-16-offline-mode-qwen2vl-design.md
│   └── 2026-01-16-offline-mode-implementation.md
├── CLAUDE.md                     # Claude Code guidance
├── lora-training-generator.skill # Original skill specification
└── README.md                     # This file
```

## Key Validation Rules

1. Each batch MUST generate exactly 40 images
2. Valid calculations: 8×5=40, 5×8=40, 10×4=40, 4×10=40
3. All prompts must include `--sref CODE`
4. Minimum 8 batches required
5. No text/typography in prompts
6. Keep prompts simple (3-8 words before modifiers)

## Troubleshooting

### Claude API Errors
- Ensure your API key is set correctly
- Check your API key has sufficient credits
- Verify network connectivity
- Try switching to Offline mode in Settings if API is unavailable

### Offline Mode Issues

**Model Download Fails:**
- Check available disk space (4.5 GB for 2B model)
- Verify internet connection (required for initial download)
- Try clearing cache and re-downloading: Settings → "Clear Cache"
- Check Hugging Face Hub is accessible

**Insufficient Memory Error:**
- Use smaller model variant (switch from 7B to 2B in Settings)
- Close other applications to free RAM
- Check system meets minimum requirements (4 GB RAM for 2B)

**Slow Inference:**
- Ensure "Keep Model Loaded" is enabled for multiple analyses
- Check GPU acceleration is working (Metal on macOS, CUDA on Linux)
- Consider using 2B model for faster results
- First analysis loads model into memory (10-15s delay)

**Model Not Found:**
- Download model from Settings page before first use
- Verify cache directory permissions
- Check model download completed successfully

### Image Upload Issues
- Only JPG, PNG, and WEBP formats are supported
- Ensure images are accessible on the filesystem
- Maximum 10 images recommended

### Build Errors
- Run `npm install` to ensure dependencies are up to date
- Run `cargo build` in `src-tauri/` to check Rust compilation
- Check that Rust and Node.js versions meet requirements
- On macOS, ensure Xcode Command Line Tools are installed (for Metal support)
- On Linux, install Intel MKL if available for better performance

## License

See LICENSE.txt in the lora-training-generator.skill archive.

## Contributing

This project implements the LoRA Training Dataset Generator skill specification. Refer to `CLAUDE.md` for development guidance when working with Claude Code.

## Skill Reference

This application is based on the LoRA Training Dataset Generator skill. For detailed information about the skill specification, prompt guidelines, and batch requirements, extract and review:

```bash
unzip -q lora-training-generator.skill -d skill-docs/
```

Key reference files:
- `skill-docs/lora-training-generator/SKILL.md` - Main skill definition
- `skill-docs/lora-training-generator/references/json-schema.md` - Output schema
- `skill-docs/lora-training-generator/references/style-examples.md` - Style categories
