use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AssetError {
    NotFound(String),
    InvalidAssetPack(String),
    IoError(std::io::Error),
    InvalidPath(String),
}

impl fmt::Display for AssetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssetError::NotFound(msg) => write!(f, "Asset not found: {}", msg),
            AssetError::InvalidAssetPack(msg) => write!(f, "Invalid asset pack: {}", msg),
            AssetError::IoError(err) => write!(f, "IO error: {}", err),
            AssetError::InvalidPath(msg) => write!(f, "Invalid path: {}", msg),
        }
    }
}

impl From<std::io::Error> for AssetError {
    fn from(err: std::io::Error) -> Self {
        AssetError::IoError(err)
    }
}

impl ResponseError for AssetError {
    fn error_response(&self) -> HttpResponse {
        let status = match self {
            AssetError::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        HttpResponse::build(status).json(json!({
            "error": self.to_string(),
        }))
    }
}
