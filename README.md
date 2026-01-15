# LoRA Training Dataset Generator

A desktop application for generating optimized Midjourney permutation batches for LoRA (Low-Rank Adaptation) training datasets from SREF (style reference) codes.

Built with **Tauri** + **Vue 3** + **Rust**.

## Features

- **Image Upload & Preview**: Drag-and-drop interface for 3-10 SREF reference images
- **AI-Powered Style Analysis**: Automatic analysis using Claude API (Sonnet 4.5)
- **Batch Generation**: Create 8-10 permutation batches with exactly 40 images each
- **Real-time Validation**: Validates permutation syntax, image counts, and SREF codes
- **Manual Editing**: Edit, duplicate, and remove batches with live validation
- **Export Options**: Export to JSON or Markdown format
- **Project Management**: Save and load projects for later editing

## Prerequisites

- **Node.js** (v18 or later)
- **Rust** (latest stable version)
- **Claude API Key** (from Anthropic)

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
- **Views**: UploadView, AnalysisView, BatchEditorView, ExportView
- **Components**: ImageUploader, StyleAnalysisCard, BatchCard, BatchValidator, ExportPanel
- **State Management**: Pinia store for project state
- **Routing**: Vue Router for navigation
- **Styling**: Tailwind CSS v4

### Backend (Rust/Tauri)
- **image_utils.rs**: Image file handling and base64 encoding
- **claude.rs**: Claude API integration for style analysis
- **file_ops.rs**: Project save/load and export operations
- **lib.rs**: Tauri command handlers

## Project Structure

```
rzem-ai-mj-lora/
├── src/                          # Vue3 frontend
│   ├── components/               # Reusable Vue components
│   ├── views/                    # Page-level components
│   ├── stores/                   # Pinia state management
│   ├── router/                   # Vue Router configuration
│   ├── types/                    # TypeScript type definitions
│   ├── utils/                    # Validation and export utilities
│   ├── App.vue                   # Root component
│   └── main.ts                   # Application entry point
├── src-tauri/                    # Rust backend
│   └── src/
│       ├── claude.rs             # Claude API client
│       ├── file_ops.rs           # File operations
│       ├── image_utils.rs        # Image handling
│       └── lib.rs                # Main Tauri app
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

### Image Upload Issues
- Only JPG, PNG, and WEBP formats are supported
- Ensure images are accessible on the filesystem
- Maximum 10 images recommended

### Build Errors
- Run `npm install` to ensure dependencies are up to date
- Run `cargo build` in `src-tauri/` to check Rust compilation
- Check that Rust and Node.js versions meet requirements

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
