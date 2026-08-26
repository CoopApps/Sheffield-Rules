use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;
use std::sync::Arc;
use std::path::PathBuf;
use crate::assets::AssetManager;
use crate::errors::AssetError;

/// Health check endpoint
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "service": "Saturday at Three"
    }))
}

/// List all available asset packs
pub async fn list_asset_packs(
    manager: web::Data<Arc<AssetManager>>,
) -> Result<HttpResponse, AssetError> {
    let packs = manager.list_packs();
    Ok(HttpResponse::Ok().json(json!({
        "packs": packs,
        "count": packs.len()
    })))
}

/// Get details of a specific asset pack
pub async fn get_asset_pack(
    manager: web::Data<Arc<AssetManager>>,
    pack_name: web::Path<String>,
) -> Result<HttpResponse, AssetError> {
    let pack = manager.get_pack(&pack_name)?;
    Ok(HttpResponse::Ok().json(pack))
}

/// Get theme metadata for a pack
pub async fn get_theme(
    manager: web::Data<Arc<AssetManager>>,
    pack_name: web::Path<String>,
) -> Result<HttpResponse, AssetError> {
    let theme = manager
        .get_theme(&pack_name)
        .ok_or_else(|| AssetError::InvalidAssetPack(format!("Pack not found: {}", pack_name)))?;
    Ok(HttpResponse::Ok().json(theme))
}

/// Get the path to an asset
pub async fn get_asset_path(
    manager: web::Data<Arc<AssetManager>>,
    path: web::Path<(String, String, String)>,
) -> Result<HttpResponse, AssetError> {
    let (pack_name, category, asset_id) = path.into_inner();
    let asset_ref = manager.get_asset_path(&pack_name, &category, &asset_id)?;
    Ok(HttpResponse::Ok().json(asset_ref))
}

/// Get an asset file (returns binary data)
pub async fn get_asset(
    manager: web::Data<Arc<AssetManager>>,
    path: web::Path<(String, String, String)>,
) -> Result<HttpResponse, AssetError> {
    let (pack_name, category, asset_id) = path.into_inner();

    // For now, return the path; we can extend this to serve actual files
    // In a real implementation, you'd serve files from disk or cache
    let asset_ref = manager.get_asset_path(&pack_name, &category, &asset_id)?;

    Ok(HttpResponse::Ok().json(json!({
        "asset": asset_ref,
        "url": format!("/assets/{}/{}/{}", pack_name, category, asset_id)
    })))
}

/// Serve a file from a pack directory with wildcard path support.
/// Route: /api/assets/{pack_name}/{tail:.*}
/// Resolves to: assets/packs/{pack_name}/{tail}
pub async fn serve_pack_file(
    req: HttpRequest,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (pack_name, tail) = path.into_inner();

    // Security: reject path traversal attempts
    if tail.contains("..") {
        return HttpResponse::BadRequest().body("Invalid path");
    }

    // Resolve relative to the assets directory (backend runs from backend/)
    let asset_path = PathBuf::from("../assets/packs")
        .join(&pack_name)
        .join(&tail);

    match tokio::fs::read(&asset_path).await {
        Ok(data) => {
            // Determine content type from extension
            let content_type = if tail.ends_with(".png") {
                "image/png"
            } else if tail.ends_with(".jpg") || tail.ends_with(".jpeg") {
                "image/jpeg"
            } else if tail.ends_with(".json") {
                "application/json"
            } else {
                "application/octet-stream"
            };
            HttpResponse::Ok()
                .content_type(content_type)
                .body(data)
        }
        Err(_) => HttpResponse::NotFound().body(format!("Asset not found: {}/{}", pack_name, tail)),
    }
}
