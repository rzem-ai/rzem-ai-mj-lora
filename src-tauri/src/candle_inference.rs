use crate::settings::ModelVariant;
use anyhow::Result;
use image::DynamicImage;
use std::path::Path;

pub struct Qwen2VLInference {
    variant: ModelVariant,
    model_path: std::path::PathBuf,
    mmproj_path: std::path::PathBuf,
}

impl Qwen2VLInference {
    pub async fn new(model_path: &Path, variant: ModelVariant) -> Result<Self> {
        log::info!("Loading Qwen3-VL model from {:?}", model_path);

        // Validate paths exist
        if !model_path.exists() {
            anyhow::bail!("Model path does not exist: {:?}", model_path);
        }

        // Determine model and mmproj file names based on variant
        let (model_file, mmproj_file) = match variant {
            ModelVariant::Qwen3VL2B => (
                "Qwen3-VL-2B-Instruct-Q8_0.gguf",
                "mmproj-Qwen3-VL-2B-Instruct-F16.gguf",
            ),
            ModelVariant::Qwen3VL4B => (
                "Qwen3-VL-4B-Instruct-Q8_0.gguf",
                "mmproj-Qwen3-VL-4B-Instruct-F16.gguf",
            ),
            ModelVariant::Qwen3VL8B => (
                "Qwen3-VL-8B-Instruct-Q8_0.gguf",
                "mmproj-Qwen3-VL-8B-Instruct-F16.gguf",
            ),
        };

        let model_file_path = model_path.join(model_file);
        let mmproj_path = model_path.join(mmproj_file);

        // Verify both files exist
        if !model_file_path.exists() {
            anyhow::bail!("Model file not found: {:?}", model_file_path);
        }
        if !mmproj_path.exists() {
            anyhow::bail!("Vision projection file not found: {:?}", mmproj_path);
        }

        log::info!("Model files validated: {:?}", model_file_path);

        Ok(Self {
            variant,
            model_path: model_file_path,
            mmproj_path,
        })
    }

    pub fn analyze_images(
        &mut self,
        images: Vec<DynamicImage>,
        prompt: &str,
    ) -> Result<String> {
        log::info!("Analyzing {} images with Qwen3-VL", images.len());

        // TODO: Implement actual llama.cpp inference
        // This requires:
        // 1. Initialize llama-cpp-2 context with model and mmproj
        // 2. Preprocess images (resize, convert to RGB)
        // 3. Encode images using vision projection
        // 4. Tokenize prompt
        // 5. Run inference with vision embeddings
        // 6. Decode response tokens to text
        // 7. Parse JSON from response

        // For now, return stub JSON to maintain existing interface
        log::warn!("llama.cpp inference not yet implemented - returning stub data");

        let stub_json = r###"{
            "sref_code": "stub-qwen3vl",
            "style_analysis": {
                "primary_style": "stub implementation",
                "era_influence": "pending llama.cpp integration",
                "color_palette": ["#000000"],
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
        }"###;

        Ok(stub_json.to_string())
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
