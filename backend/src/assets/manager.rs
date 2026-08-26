use crate::errors::AssetError;
use crate::models::{AssetPack, AssetReference, ThemeMetadata};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{debug, info, warn};

/// Core asset manager for loading and serving game assets
/// Supports multiple asset packs and provides a generic interface for asset retrieval
pub struct AssetManager {
    assets_dir: PathBuf,
    loaded_packs: HashMap<String, AssetPack>,
    asset_cache: HashMap<String, Vec<u8>>,
}

impl AssetManager {
    /// Create a new AssetManager pointing to the assets directory
    pub async fn new(assets_dir: impl AsRef<Path>) -> Self {
        let assets_dir = PathBuf::from(assets_dir.as_ref());
        let mut manager = AssetManager {
            assets_dir,
            loaded_packs: HashMap::new(),
            asset_cache: HashMap::new(),
        };

        // Auto-discover and load asset packs
        if let Err(e) = manager.discover_packs().await {
            warn!("Failed to discover asset packs: {}", e);
        }

        manager
    }

    /// Discover all available asset packs in the assets directory
    async fn discover_packs(&mut self) -> Result<(), AssetError> {
        let packs_dir = self.assets_dir.join("packs");

        if !packs_dir.exists() {
            debug!("Packs directory does not exist: {:?}", packs_dir);
            return Ok(());
        }

        let mut entries = fs::read_dir(&packs_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                if let Some(pack_name) = path.file_name() {
                    if let Some(pack_name_str) = pack_name.to_str() {
                        match self.load_pack(pack_name_str).await {
                            Ok(pack) => {
                                info!("Loaded asset pack: {}", pack_name_str);
                                self.loaded_packs.insert(pack_name_str.to_string(), pack);
                            }
                            Err(e) => {
                                warn!("Failed to load pack {}: {}", pack_name_str, e);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Load a specific asset pack from disk
    async fn load_pack(&self, pack_name: &str) -> Result<AssetPack, AssetError> {
        let pack_dir = self.assets_dir.join("packs").join(pack_name);
        let metadata_path = pack_dir.join("metadata.json");

        if metadata_path.exists() {
            let metadata = fs::read_to_string(&metadata_path).await?;
            let pack: AssetPack = serde_json::from_str(&metadata)
                .map_err(|e| AssetError::InvalidAssetPack(e.to_string()))?;
            Ok(pack)
        } else {
            // Create a default pack metadata
            Ok(AssetPack {
                name: pack_name.to_string(),
                version: "1.0.0".to_string(),
                description: format!("{} asset pack", pack_name),
                author: "Unknown".to_string(),
                era: pack_name.to_string(),
                metadata: HashMap::new(),
            })
        }
    }

    /// Get all available asset packs
    pub fn list_packs(&self) -> Vec<AssetPack> {
        self.loaded_packs.values().cloned().collect()
    }

    /// Get a specific asset pack by name
    pub fn get_pack(&self, pack_name: &str) -> Result<AssetPack, AssetError> {
        self.loaded_packs
            .get(pack_name)
            .cloned()
            .ok_or_else(|| AssetError::InvalidAssetPack(format!("Pack not found: {}", pack_name)))
    }

    /// Load an asset file into memory
    pub async fn load_asset(
        &mut self,
        pack_name: &str,
        category: &str,
        asset_id: &str,
    ) -> Result<Vec<u8>, AssetError> {
        let cache_key = format!("{}:{}:{}", pack_name, category, asset_id);

        // Check cache first
        if let Some(cached) = self.asset_cache.get(&cache_key) {
            return Ok(cached.clone());
        }

        // Build path to asset
        let asset_path = self
            .assets_dir
            .join("packs")
            .join(pack_name)
            .join(category)
            .join(asset_id);

        if !asset_path.exists() {
            return Err(AssetError::NotFound(format!(
                "{}/{}/{}",
                pack_name, category, asset_id
            )));
        }

        // Load from disk
        let data = fs::read(&asset_path).await?;

        // Cache it
        self.asset_cache.insert(cache_key, data.clone());

        Ok(data)
    }

    /// Get the path to an asset without loading it
    pub fn get_asset_path(
        &self,
        pack_name: &str,
        category: &str,
        asset_id: &str,
    ) -> Result<AssetReference, AssetError> {
        let base_path = self.assets_dir.join("packs").join(pack_name);

        if !base_path.exists() {
            return Err(AssetError::InvalidAssetPack(pack_name.to_string()));
        }

        Ok(AssetReference {
            pack: pack_name.to_string(),
            category: category.to_string(),
            asset_id: asset_id.to_string(),
            path: format!("/{}/{}/{}", pack_name, category, asset_id),
        })
    }

    /// Get theme metadata for a pack
    pub fn get_theme(&self, pack_name: &str) -> Option<ThemeMetadata> {
        self.loaded_packs.get(pack_name).map(|pack| ThemeMetadata {
            pack_name: pack.name.clone(),
            colors: crate::models::ColorPalette {
                primary: "#1a1a2e".to_string(),
                secondary: "#16213e".to_string(),
                accent: "#0f3460".to_string(),
                background: "#0f1820".to_string(),
                text: "#eaeaea".to_string(),
            },
            fonts: crate::models::FontConfig {
                body: "Segoe UI, Arial, sans-serif".to_string(),
                heading: "Georgia, serif".to_string(),
                mono: "Courier New, monospace".to_string(),
            },
        })
    }

    /// Clear the asset cache
    pub fn clear_cache(&mut self) {
        self.asset_cache.clear();
    }
}
