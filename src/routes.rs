use crate::context::Ctx;
use crate::controller;
use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde_json::{Value, json};
use std::sync::Arc;

/// Create all routes for the application
pub fn create_routes<S: Send + Sync>(ctx: Arc<Ctx>) -> Router<S> {
    Router::new()
        .route("/health", get(health_check))
        .route("/", get(root))
        .route(
            "/api/resource/{service_name}/{service_type}/{org_id}/{name}",
            post(controller::create_resource)
                .put(controller::update_resource)
                .get(controller::get_resource)
                .delete(controller::delete_resource),
        )
        .with_state(ctx)
}

/// Health check endpoint
async fn health_check() -> (StatusCode, Json<Value>) {
    tracing::info!("Health check endpoint called");
    (StatusCode::OK, Json(json!({ "status": "healthy" })))
}

/// Root endpoint
async fn root() -> (StatusCode, Json<Value>) {
    tracing::info!("Root endpoint called");
    (
        StatusCode::OK,
        Json(json!({ "message": "Welcome to OpenFGA Demo API" })),
    )
}
