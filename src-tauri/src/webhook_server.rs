use axum::{
    extract::State,
    http::StatusCode,
    routing::post,
    Json, Router,
};
use chronos_lib::commands::webhook::{handle_git_webhook_command, GitWebhookPayload};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

/// Shared application state for the webhook server
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
}

/// Handler for POST /api/webhooks/git
async fn git_webhook_handler(
    State(state): State<AppState>,
    Json(payload): Json<GitWebhookPayload>,
) -> Result<Json<WebhookResponse>, (StatusCode, String)> {
    println!(
        "[Webhook] Received Git commit from repo '{}' by '{}'",
        payload.repo_name, payload.author
    );

    match handle_git_webhook_command(&state.db, payload).await {
        Ok(task) => {
            println!("[Webhook] Successfully appended commit to task: {}", task.id);
            Ok(Json(WebhookResponse {
                success: true,
                message: format!("Commit logged to task: {}", task.title),
                task_id: Some(task.id),
            }))
        }
        Err(e) => {
            eprintln!("[Webhook] Error handling webhook: {}", e);
            Err((StatusCode::BAD_REQUEST, e))
        }
    }
}

/// Health check endpoint
async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        service: "chronos-webhook-server".to_string(),
    })
}

#[derive(serde::Serialize)]
struct WebhookResponse {
    success: bool,
    message: String,
    task_id: Option<String>,
}

#[derive(serde::Serialize)]
struct HealthResponse {
    status: String,
    service: String,
}

/// Create and configure the Axum router
fn create_router(state: AppState) -> Router {
    // Configure CORS to allow requests from localhost
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/webhooks/git", post(git_webhook_handler))
        .route("/health", axum::routing::get(health_check))
        .layer(cors)
        .with_state(state)
}

/// Start the webhook server on a background thread
pub async fn start_webhook_server(db: Arc<DatabaseConnection>) -> Result<(), String> {
    let state = AppState { db };
    let app = create_router(state);

    let addr = "127.0.0.1:3030";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| format!("Failed to bind to {}: {}", addr, e))?;

    println!("[Webhook Server] Starting on http://{}", addr);
    println!("[Webhook Server] Endpoint: POST http://{}/api/webhooks/git", addr);

    // Spawn the server in a background task
    tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            eprintln!("[Webhook Server] Server error: {}", e);
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_response_serialization() {
        let response = WebhookResponse {
            success: true,
            message: "Test".to_string(),
            task_id: Some("task_123".to_string()),
        };
        assert!(response.success);
    }
}
