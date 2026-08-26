use actix_web::web;
use super::handlers;

/// Configure all API routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        // Asset endpoints (specific routes first)
        .route("/api/assets/packs", web::get().to(handlers::list_asset_packs))
        .route("/api/assets/pack/{pack_name}", web::get().to(handlers::get_asset_pack))
        .route("/api/assets/theme/{pack_name}", web::get().to(handlers::get_theme))
        .route("/api/assets/path/{pack_name}/{category}/{asset_id}", web::get().to(handlers::get_asset_path))
        .route("/api/assets/{pack_name}/{category}/{asset_id}", web::get().to(handlers::get_asset))
        // Wildcard file server for pack assets (e.g. sprites with deep paths)
        .route("/api/assets/{pack_name}/{tail:.*}", web::get().to(handlers::serve_pack_file))

        // Health check
        .route("/health", web::get().to(handlers::health_check));
}
