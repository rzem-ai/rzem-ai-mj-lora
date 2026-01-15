use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_VERSION: &str = "2023-06-01";
const MODEL: &str = "claude-sonnet-4-5-20250929";

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub source: ImageSource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub media_type: String,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
    Image(ImageContent),
    Text(TextContent),
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: Vec<Content>,
}

#[derive(Debug, Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
struct ClaudeResponse {
    content: Vec<ResponseContent>,
}

#[derive(Debug, Deserialize)]
struct ResponseContent {
    #[serde(rename = "type")]
    _content_type: String,
    text: Option<String>,
}

/// Get the Claude API key from environment variable
fn get_api_key() -> Result<String> {
    std::env::var("CLAUDE_API_KEY")
        .or_else(|_| std::env::var("ANTHROPIC_API_KEY"))
        .context("CLAUDE_API_KEY or ANTHROPIC_API_KEY environment variable not set")
}

/// Build the skill prompt for analyzing SREF style
fn build_skill_prompt(sref_code: &str) -> String {
    format!(r#"You are an expert LoRA (Low-Rank Adaptation) training dataset generator for Midjourney SREF codes.

Analyze the provided style reference images for SREF code: {}

Based on these images, generate a complete LoRA training dataset specification. Follow these requirements:

1. **Style Analysis**: Identify visual characteristics, color palette, composition patterns, texture, line quality, and subject affinity

2. **Permutation Batches**: Create 8-10 batches where EACH batch generates EXACTLY 40 images using Midjourney's permutation syntax {{option1, option2, ...}}

3. **Batch Requirements**:
   - Format: {{subjects}} with {{modifiers}} --sref {}
   - Valid calculations: 8×5=40, 5×8=40, 10×4=40, 4×10=40
   - Keep prompts simple (3-8 words before modifiers)
   - Let SREF handle styling - avoid style descriptors

4. **Output Format**: Return ONLY valid JSON matching this schema (no markdown, no code blocks):

{{
  "sref_code": "{}",
  "style_analysis": {{
    "primary_style": "string",
    "era_influence": "string",
    "color_palette": ["color1", "color2"],
    "key_characteristics": ["trait1", "trait2"],
    "best_subjects": ["subject1", "subject2"],
    "avoid_subjects": ["subject1", "subject2"]
  }},
  "training_recommendations": {{
    "recommended_dataset_size": 100,
    "optimal_subject_distribution": {{
      "category": "percentage"
    }}
  }},
  "permutation_batches": [
    {{
      "batch_number": 1,
      "batch_name": "string",
      "category": "string",
      "image_count": 40,
      "prompt": "{{subject1, subject2, ...}} with {{modifier1, modifier2, ...}} --sref {}",
      "priority": "high|medium|low",
      "notes": "optional guidance"
    }}
  ],
  "prompt_guidelines": {{
    "keep_simple": true,
    "avoid_style_keywords": ["keyword1"],
    "recommended_additions": ["element1"]
  }}
}}

CRITICAL:
- Each batch MUST generate exactly 40 images
- Include SREF code in every prompt
- Return ONLY JSON, no additional text or markdown
- Ensure all batches have valid permutation syntax"#,
        sref_code, sref_code, sref_code, sref_code
    )
}

/// Call Claude API to analyze style and generate dataset specification
pub async fn analyze_style(
    image_data: Vec<(String, String)>, // (base64_data, mime_type)
    sref_code: &str,
) -> Result<String> {
    let api_key = get_api_key()?;
    let client = Client::new();

    // Build content array with images first, then text
    let mut content: Vec<Content> = Vec::new();

    // Add all images
    for (data, mime_type) in image_data {
        content.push(Content::Image(ImageContent {
            content_type: "image".to_string(),
            source: ImageSource {
                source_type: "base64".to_string(),
                media_type: mime_type,
                data,
            },
        }));
    }

    // Add text prompt
    content.push(Content::Text(TextContent {
        content_type: "text".to_string(),
        text: build_skill_prompt(sref_code),
    }));

    let request = ClaudeRequest {
        model: MODEL.to_string(),
        max_tokens: 8192,
        messages: vec![Message {
            role: "user".to_string(),
            content,
        }],
    };

    // Make API request
    let response = client
        .post(ANTHROPIC_API_URL)
        .header("x-api-key", api_key)
        .header("anthropic-version", ANTHROPIC_VERSION)
        .header("content-type", "application/json")
        .json(&request)
        .send()
        .await
        .context("Failed to send request to Claude API")?;

    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        anyhow::bail!("Claude API error ({}): {}", status, response_text);
    }

    // Parse response
    let claude_response: ClaudeResponse =
        serde_json::from_str(&response_text).context("Failed to parse Claude response")?;

    // Extract text from response
    let text = claude_response
        .content
        .iter()
        .find_map(|c| c.text.as_ref())
        .context("No text content in Claude response")?;

    // Try to extract JSON if it's wrapped in markdown code blocks
    let json_text = if text.contains("```json") {
        text.split("```json")
            .nth(1)
            .and_then(|s| s.split("```").next())
            .unwrap_or(text)
            .trim()
    } else if text.contains("```") {
        text.split("```")
            .nth(1)
            .and_then(|s| s.split("```").next())
            .unwrap_or(text)
            .trim()
    } else {
        text.trim()
    };

    // Validate that it's valid JSON
    serde_json::from_str::<serde_json::Value>(json_text)
        .context("Claude response is not valid JSON")?;

    Ok(json_text.to_string())
}
