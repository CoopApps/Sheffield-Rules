use crate::errors::AssetError;
use std::path::Path;

/// Generic asset loader trait for different asset types
pub trait AssetLoader {
    type Output;

    fn load(&self, path: &Path) -> Result<Self::Output, AssetError>;
}

/// Image asset loader
pub struct ImageLoader;

impl AssetLoader for ImageLoader {
    type Output = Vec<u8>;

    fn load(&self, path: &Path) -> Result<Vec<u8>, AssetError> {
        std::fs::read(path).map_err(|e| AssetError::IoError(e))
    }
}

/// JSON asset loader for metadata and configuration
pub struct JsonLoader;

impl AssetLoader for JsonLoader {
    type Output = serde_json::Value;

    fn load(&self, path: &Path) -> Result<serde_json::Value, AssetError> {
        let content = std::fs::read_to_string(path).map_err(|e| AssetError::IoError(e))?;
        serde_json::from_str(&content)
            .map_err(|e| AssetError::InvalidAssetPack(e.to_string()))
    }
}
