# OpenFGA Client Usage Examples

This document provides examples of how to use the OpenFGA client in the application.

## Basic Usage

```rust
// Get the OpenFGA client from the context
let fga_client = ctx.fga_client.clone();

// Create a store
let create_store_response = fga_client
    .create_store(openfga_client::api::CreateStoreRequest {
        name: "my-store".to_string(),
    })
    .await?;

let store_id = create_store_response.into_inner().id;
println!("Created store with ID: {}", store_id);

// Write an authorization model
let authorization_model = r#"{
  "type_definitions": [
    {
      "type": "user",
      "relations": {}
    },
    {
      "type": "document",
      "relations": {
        "reader": {
          "this": {}
        },
        "writer": {
          "this": {}
        },
        "owner": {
          "this": {}
        }
      }
    }
  ]
}"#;

let write_model_response = fga_client
    .write_authorization_model(openfga_client::api::WriteAuthorizationModelRequest {
        store_id: store_id.clone(),
        type_definitions: serde_json::from_str(authorization_model).unwrap(),
        schema_version: "1.1".to_string(),
    })
    .await?;

let model_id = write_model_response.into_inner().authorization_model_id;
println!("Created authorization model with ID: {}", model_id);

// Write tuples (relationships)
let write_response = fga_client
    .write(openfga_client::api::WriteRequest {
        store_id: store_id.clone(),
        writes: vec![openfga_client::api::TupleKey {
            user: "user:anne".to_string(),
            relation: "reader".to_string(),
            object: "document:doc1".to_string(),
        }],
        authorization_model_id: Some(model_id.clone()),
        ..Default::default()
    })
    .await?;

println!("Write response: {:?}", write_response);

// Check if a user has permission
let check_response = fga_client
    .check(openfga_client::api::CheckRequest {
        store_id: store_id.clone(),
        tuple_key: Some(openfga_client::api::TupleKey {
            user: "user:anne".to_string(),
            relation: "reader".to_string(),
            object: "document:doc1".to_string(),
        }),
        authorization_model_id: Some(model_id.clone()),
        ..Default::default()
    })
    .await?;

let allowed = check_response.into_inner().allowed;
println!("Is Anne allowed to read doc1? {}", allowed);
```

## Using the Client in Controllers

```rust
pub async fn check_permission(
    State(ctx): State<Arc<Ctx>>,
    Path(params): Path<ResourceParams>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let fga_client = ctx.fga_client.clone();
    
    // Get store ID and model ID from configuration or database
    let store_id = "YOUR_STORE_ID";
    let model_id = "YOUR_MODEL_ID";
    
    // Check if user has permission
    let check_result = fga_client
        .check(openfga_client::api::CheckRequest {
            store_id: store_id.to_string(),
            tuple_key: Some(openfga_client::api::TupleKey {
                user: format!("user:{}", params.name),
                relation: "reader".to_string(),
                object: format!("document:{}", params.service_name),
            }),
            authorization_model_id: Some(model_id.to_string()),
            ..Default::default()
        })
        .await;
    
    match check_result {
        Ok(response) => {
            let allowed = response.into_inner().allowed;
            Ok((
                StatusCode::OK,
                Json(json!({
                    "allowed": allowed,
                })),
            ))
        },
        Err(e) => {
            tracing::error!("Error checking permission: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to check permission",
                })),
            ))
        }
    }
}
```

## Environment Setup

Make sure to set the following environment variables:

```bash
# OpenFGA client URL
export OPENFGA_CLIENT_URL=http://localhost:8080

# Database URL
export DATABASE_URL=postgres://user:password@localhost:5432/openfga_demo

# Application profile
export PROFILE=dev
```

You can also create a `.env` file in the project root with these variables.
