mod assets;
mod models;
mod api;
mod errors;

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use tracing_subscriber;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Initialize asset manager
    let asset_manager = Arc::new(assets::AssetManager::new("../assets").await);

    println!("Starting Saturday at Three server on http://127.0.0.1:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(asset_manager.clone()))
            .configure(api::routes::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
