use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetPack {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub era: String, // e.g., "1888", "modern", "1970s"
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReference {
    pub pack: String,
    pub category: String,
    pub asset_id: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerPortrait {
    pub player_id: String,
    pub name: String,
    pub image_path: String,
    pub pack: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeMetadata {
    pub pack_name: String,
    pub colors: ColorPalette,
    pub fonts: FontConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub background: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontConfig {
    pub body: String,
    pub heading: String,
    pub mono: String,
}
