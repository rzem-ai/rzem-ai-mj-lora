# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

This is a **Claude Code skill package** for generating optimized LoRA (Low-Rank Adaptation) training datasets from Midjourney SREF (style reference) codes. The skill analyzes style reference images, identifies visual characteristics, and generates structured Midjourney permutation batches designed to create comprehensive training datasets.

## Core Functionality

The skill operates in these phases:

1. **Style Analysis**: Receive 3-10 style reference images + SREF code, then identify visual characteristics (color palette, composition patterns, texture, line quality, subject affinity)
2. **Batch Generation**: Create 8-10 permutation batches with EXACTLY 40 images each using Midjourney's `{option1, option2}` syntax
3. **Output**: Structured JSON or Markdown format with complete specifications

## Skill Package Structure

```
lora-training-generator.skill (ZIP archive containing):
├── SKILL.md                          # Main skill definition and workflow
├── LICENSE.txt                       # MIT License
└── references/
    ├── json-schema.md                # Complete JSON output specification
    ├── markdown-template.md          # Alternative Markdown format
    ├── style-examples.md             # Common SREF style categories
    ├── api-usage.md                  # Python implementation guide
    └── js-implementation.md          # JavaScript/TypeScript implementation
```

## Key Constraints and Rules

### Permutation Batch Requirements

**CRITICAL**: Each batch MUST generate exactly 40 images. Valid calculations:
- 8 subjects × 5 modifiers = 40 ✓
- 5 subjects × 8 modifiers = 40 ✓
- 10 subjects × 4 modifiers = 40 ✓
- 4 subjects × 10 modifiers = 40 ✓

**Prompt Format**: `{subjects} with {modifiers} --sref [CODE]`

### Style-Specific Best Practices

**Retro/Vintage (60s-80s)**:
- Focus on: nature, vehicles, nostalgic objects, geometric patterns
- Avoid: modern tech, contemporary fashion

**Minimalist**:
- Focus on: simple objects, geometric shapes, negative space
- Avoid: complex scenes, busy compositions

**Photorealistic**:
- Focus on: detailed subjects, texture variety, lighting conditions
- Avoid: abstract concepts, stylized subjects

**Anime/Illustration**:
- Focus on: characters, expressive poses, narrative scenes
- Avoid: photorealistic details, complex lighting

### Dataset Quality Controls

- NO text/typography in prompts (causes training issues)
- Minimum 50 images, optimal 80-150 images
- Distribute across 8-10 categories
- Keep prompts SHORT (3-8 words before modifiers)
- Avoid style descriptors already in SREF (e.g., "retro", "vintage", "watercolor")
- Include compositional variety (times of day, weather, lighting)

## Output Formats

### JSON (Default)
See `references/json-schema.md` for complete schema. Key sections:
- `sref_code`: The Midjourney SREF code
- `style_analysis`: Detailed style breakdown
- `training_recommendations`: Dataset size, distribution
- `permutation_batches[]`: Array of 40-image batches
- `prompt_guidelines`: Do's and don'ts

### Markdown (Alternative)
See `references/markdown-template.md` for structure with organized sections, code blocks, and priority indicators.

## Validation Checklist

Before outputting, verify:
1. All batch `image_count` values equal 40
2. Batch prompts use valid permutation syntax
3. Math is correct (subjects × modifiers = 40)
4. SREF code included in every prompt
5. No duplicate batch numbers
6. Distribution percentages sum to ~100%
7. At least 8 batches provided
8. Priority values are only "high", "medium", or "low"

## Common Pitfalls to Avoid

- Batches that don't equal 40 images
- Over-complicated prompts (SREF does the styling)
- Style keywords redundant with SREF
- Text/typography in any form
- Modern subjects in vintage styles
- Homogeneous datasets (too similar)

## Working with the Skill Archive

To extract and examine the skill:
```bash
unzip -q lora-training-generator.skill -d extracted/
```

To view the skill structure:
```bash
unzip -l lora-training-generator.skill
```

## Architecture Notes

This is a **declarative skill definition**, not executable code. The skill works through:

1. **SKILL.md**: Contains the main instructions that Claude Code reads when the skill is invoked
2. **Reference files**: Provide detailed specifications, examples, and API usage patterns
3. **No build/test commands**: This is a pure specification package, not a software project

When users invoke this skill, Claude Code receives the instructions from SKILL.md and uses the reference materials to generate properly formatted LoRA training dataset specifications.
