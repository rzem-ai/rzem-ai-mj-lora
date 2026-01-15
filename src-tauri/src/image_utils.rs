use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine as _};
use std::fs;
use std::path::Path;

/// Read an image file and encode it as base64
pub fn read_and_encode_image(path: &str) -> Result<String> {
    let path_obj = Path::new(path);

    // Validate file exists
    if !path_obj.exists() {
        anyhow::bail!("Image file does not exist: {}", path);
    }

    // Read file bytes
    let bytes = fs::read(path_obj)
        .with_context(|| format!("Failed to read image file: {}", path))?;

    // Encode to base64
    let encoded = general_purpose::STANDARD.encode(&bytes);

    Ok(encoded)
}

/// Determine MIME type from file extension
pub fn get_mime_type(path: &str) -> Result<String> {
    let path_obj = Path::new(path);
    let extension = path_obj
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "jpg" | "jpeg" => Ok("image/jpeg".to_string()),
        "png" => Ok("image/png".to_string()),
        "webp" => Ok("image/webp".to_string()),
        "gif" => Ok("image/gif".to_string()),
        _ => anyhow::bail!("Unsupported image format: {}", extension),
    }
}

/// Validate that a file is a supported image format
pub fn is_valid_image(path: &str) -> bool {
    get_mime_type(path).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mime_type() {
        assert_eq!(get_mime_type("test.jpg").unwrap(), "image/jpeg");
        assert_eq!(get_mime_type("test.png").unwrap(), "image/png");
        assert_eq!(get_mime_type("test.webp").unwrap(), "image/webp");
        assert!(get_mime_type("test.txt").is_err());
    }
}
