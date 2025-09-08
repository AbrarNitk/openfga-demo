use crate::auth::AuthUser;
use crate::context::Ctx;
use axum::{
    Extension,
    extract::{Json, Path, State},
    http::StatusCode,
};
use openfga_client::client::{CheckRequest, CheckRequestTupleKey, TupleKeyWithoutCondition};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::Arc;
use tonic::Request;

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

/// Check if a user has the required permission for a resource
async fn check_permission(
    ctx: &Arc<Ctx>,
    user_id: &str,
    relation: &str,
    object_id: &str,
) -> Result<bool, Box<dyn std::error::Error>> {

    tracing::info!(
        "Checking if user {} has {} permission on resource {}",
        user_id,
        relation,
        object_id
    );

    // Get store ID from context
    let store_id = &ctx.fga_config.store_id;
    if store_id.is_empty() {
        return Err("OpenFGA store ID not configured".into());
    }

    // Get authorization model ID from context
    let authorization_model_id = match &ctx.fga_config.authorization_model_id {
        Some(id) => id,
        None => return Err("OpenFGA authorization model ID not configured".into()),
    };

    // Get the OpenFGA client
    let mut service_client = ctx.fga_client.clone();

    // Create the tuple key for checking
    let tuple_key = TupleKeyWithoutCondition {
        user: format!("user:{}", user_id),
        relation: relation.to_string(),
        object: object_id.to_string(),
    };

    // Create a check request using tonic::Request
    let check_request = Request::new(CheckRequest {
        store_id: store_id.clone(),
        tuple_key: Some(CheckRequestTupleKey {
            user: tuple_key.user,
            relation: tuple_key.relation,
            object: tuple_key.object,
        }),
        authorization_model_id: authorization_model_id.clone(),
        ..Default::default()
    });

    // Perform the check
    match service_client.check(check_request).await {
        Ok(response) => {
            let allowed = response.into_inner().allowed;
            tracing::info!(
                "Permission check result for user {} on resource {}: {}",
                user_id,
                object_id,
                allowed
            );
            Ok(allowed)
        }
        Err(e) => {
            tracing::error!("Error checking permission with OpenFGA: {}", e);

            // Check if it's a transport error (connection issue)
            let error_msg = e.to_string();
            if error_msg.contains("transport error") || error_msg.contains("Connection refused") {
                tracing::error!("OpenFGA server appears to be unavailable. Please check:");
                tracing::error!("1. OpenFGA server is running");
                tracing::error!(
                    "2. OPENFGA_CLIENT_URL is correct (default: http://localhost:8081)"
                );
                tracing::error!("3. Network connectivity to OpenFGA server");
                return Err("OpenFGA server is not available. Please check server status and configuration.".into());
            }

            Err(format!("OpenFGA permission check failed: {}", e).into())
        }
    }
}

// Create a new resource
pub async fn create_resource(
    State(ctx): State<Arc<Ctx>>,
    Extension(auth_user): Extension<AuthUser>,
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

    // let resource_key = format!(
    //     "{}/{}/{}/{}",
    //     params.service_name, params.service_type, params.org_id, params.name
    // );

    let org_key = format!(
        "organisation:{}",
        params.org_id
    );

    // Get user ID from authentication middleware
    let user_id = &auth_user.user_id;

    // To create a resource, user needs to be an admin of the organization
    // In a real app, we would check if the user is an admin of the organization
    // For this example, we'll check if the user has admin permission on the resource
    match check_permission(&ctx, user_id, "admin", &org_key).await {
        Ok(allowed) => {
            if !allowed {
                tracing::warn!(
                    "User {} does not have admin permission for resource {}",
                    user_id,
                    org_key
                );
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(json!({
                        "error": "Permission denied",
                        "message": "You do not have permission to create this resource"
                    })),
                ));
            }

            tracing::info!(
                "User {} has admin permission for organisation {}",
                user_id,
                org_key
            );

            // In a real app, we would create the resource in the database

            Ok((
                StatusCode::CREATED,
                Json(json!({
                    "message": "Resource created successfully",
                    "organisation": params.org_id
                })),
            ))
        }
        Err(e) => {
            tracing::error!("Error checking permission: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to check permission",
                    "message": e.to_string()
                })),
            ))
        }
    }
}

// Update an existing resource
pub async fn update_resource(
    State(ctx): State<Arc<Ctx>>,
    Extension(auth_user): Extension<AuthUser>,
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

    // Get user ID from authentication middleware
    let user_id = &auth_user.user_id;

    // To update a resource, user needs to be an editor of the resource
    match check_permission(&ctx, user_id, "editor", &resource_key).await {
        Ok(allowed) => {
            if !allowed {
                tracing::warn!(
                    "User {} does not have editor permission for resource {}",
                    user_id,
                    resource_key
                );
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(json!({
                        "error": "Permission denied",
                        "message": "You do not have permission to update this resource"
                    })),
                ));
            }

            tracing::info!(
                "User {} has editor permission for resource {}",
                user_id,
                resource_key
            );

            // In a real app, we would update the resource in the database

            Ok((
                StatusCode::OK,
                Json(json!({
                    "message": "Resource updated successfully",
                    "resource_id": resource_key
                })),
            ))
        }
        Err(e) => {
            tracing::error!("Error checking permission: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to check permission",
                    "message": e.to_string()
                })),
            ))
        }
    }
}

// Get a resource
pub async fn get_resource(
    State(ctx): State<Arc<Ctx>>,
    Extension(auth_user): Extension<AuthUser>,
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

    // Get user ID from authentication middleware
    let user_id = &auth_user.user_id;

    // Check if user has viewer permission on the resource
    match check_permission(&ctx, user_id, "viewer", &resource_key).await {
        Ok(allowed) => {
            if !allowed {
                tracing::warn!(
                    "User {} does not have viewer permission for resource {}",
                    user_id,
                    resource_key
                );
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(json!({
                        "error": "Permission denied",
                        "message": "You do not have permission to view this resource"
                    })),
                ));
            }

            tracing::info!(
                "User {} has viewer permission for resource {}",
                user_id,
                resource_key
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
        Err(e) => {
            tracing::error!("Error checking permission: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to check permission",
                    "message": e.to_string()
                })),
            ))
        }
    }
}

// Delete a resource
pub async fn delete_resource(
    State(ctx): State<Arc<Ctx>>,
    Extension(auth_user): Extension<AuthUser>,
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

    // Get user ID from authentication middleware
    let user_id = &auth_user.user_id;

    // To delete a resource, user needs to be an owner of the resource
    match check_permission(&ctx, user_id, "owner", &resource_key).await {
        Ok(allowed) => {
            if !allowed {
                tracing::warn!(
                    "User {} does not have owner permission for resource {}",
                    user_id,
                    resource_key
                );
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(json!({
                        "error": "Permission denied",
                        "message": "You do not have permission to delete this resource"
                    })),
                ));
            }

            tracing::info!(
                "User {} has owner permission for resource {}",
                user_id,
                resource_key
            );

            // In a real app, we would delete the resource from the database

            Ok((
                StatusCode::OK,
                Json(json!({
                    "message": "Resource deleted successfully",
                    "resource_id": resource_key
                })),
            ))
        }
        Err(e) => {
            tracing::error!("Error checking permission: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to check permission",
                    "message": e.to_string()
                })),
            ))
        }
    }
}
