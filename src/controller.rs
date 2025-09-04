use crate::context::Ctx;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resource {
    pub name: String,
    pub service_name: String,
    pub service_type: String,
    pub org_id: String,
    pub properties: Value,
}

#[derive(Debug, Deserialize)]
pub struct ResourceParams {
    pub service_name: String,
    pub service_type: String,
    pub org_id: String,
    pub name: String,
}

// Create a new resource
pub async fn create_resource(
    State(_ctx): State<Arc<Ctx>>,
    Path(params): Path<ResourceParams>,
    Json(_payload): Json<Value>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    tracing::info!(
        "Creating resource: {}/{}/{}/{}",
        params.service_name,
        params.service_type,
        params.org_id,
        params.name
    );

    let resource_key = format!(
        "{}/{}/{}/{}",
        params.service_name, params.service_type, params.org_id, params.name
    );

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "message": "Resource created successfully",
            "resource_id": resource_key
        })),
    ))
}

// Update an existing resource
pub async fn update_resource(
    State(_ctx): State<Arc<Ctx>>,
    Path(params): Path<ResourceParams>,
    Json(_payload): Json<Value>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    tracing::info!(
        "Updating resource: {}/{}/{}/{}",
        params.service_name,
        params.service_type,
        params.org_id,
        params.name
    );

    let resource_key = format!(
        "{}/{}/{}/{}",
        params.service_name, params.service_type, params.org_id, params.name
    );

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Resource updated successfully",
            "resource_id": resource_key
        })),
    ))
}

// Get a resource
pub async fn get_resource(
    State(_ctx): State<Arc<Ctx>>,
    Path(params): Path<ResourceParams>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    tracing::info!(
        "Getting resource: {}/{}/{}/{}",
        params.service_name,
        params.service_type,
        params.org_id,
        params.name
    );

    let resource_key = format!(
        "{}/{}/{}/{}",
        params.service_name, params.service_type, params.org_id, params.name
    );

    Ok((
        StatusCode::OK,
        Json(json!({
            "resource_id": resource_key,
            "name": params.name,
            "service_name": params.service_name,
            "service_type": params.service_type,
            "org_id": params.org_id
        })),
    ))
}

// Delete a resource
pub async fn delete_resource(
    State(_ctx): State<Arc<Ctx>>,
    Path(params): Path<ResourceParams>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    tracing::info!(
        "Deleting resource: {}/{}/{}/{}",
        params.service_name,
        params.service_type,
        params.org_id,
        params.name
    );

    let resource_key = format!(
        "{}/{}/{}/{}",
        params.service_name, params.service_type, params.org_id, params.name
    );

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Resource deleted successfully",
            "resource_id": resource_key
        })),
    ))
}
