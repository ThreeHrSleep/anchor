//! The routes for the HTTP API

use api_types::{GenericResponse, VersionData};
use axum::{routing::get, Json, Router};
use version::version_with_platform;
use system_health::SystemHealth;
use processor::Senders;
use eth2::lighthouse::Health;
use health_metrics::observe::Observe;
/// Creates all the routes for HTTP API
pub fn new() -> Router {
    // Default route
    Router::new()
        .route("/", get(root))
        .route("/anchor/version", get(get_version))
        .route("/anchor/health", get(get_health))
}

// Temporary return value.
async fn root() -> &'static str {
    "Anchor client"
}

async fn get_version() -> Json<GenericResponse<VersionData>> {
    Json(GenericResponse::from(VersionData {
        version: version_with_platform(),
    }))
}

async fn get_health() -> Json<GenericResponse<Result<Health, String>>> {
    Json(GenericResponse::from(
        eth2::lighthouse::Health::observe()
    ))
                    
}
